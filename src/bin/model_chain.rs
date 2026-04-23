use anyhow::Result;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;
use whitt_execution_engine::client::http_client::LlamaHttpClient;
use whitt_execution_engine::client::model_download::download_model_from_hf;
use whitt_execution_engine::client::types::{ChatCompletionRequest, ChatMessage};

const SERVER_URL: &str = "http://localhost:8080";
const MODELS_DIR: &str = "./models";

struct StepConfig {
    role: String,
    model_id: String,
    system_prompt: String,
    hf_repo: Option<String>,
    hf_filename: Option<String>,
    local_filename: Option<String>,
    max_tokens: usize,
    max_input_chars: usize,
}

struct StepResult {
    role: String,
    model_id: String,
    output: String,
    prompt_tokens: usize,
    completion_tokens: usize,
    total_tokens: usize,
    load_ms: u64,
    inference_ms: u64,
    unload_ms: u64,
    download_ms: Option<u64>,
}

async fn retry_async<F, Fut, T>(
    mut operation: F,
    max_attempts: usize,
    backoff: Duration,
    context: String,
) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: futures::Future<Output = Result<T>>,
{
    let mut attempt = 0;
    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                attempt += 1;
                if attempt >= max_attempts {
                    return Err(e.context(format!("Failed after {} attempts: {}", max_attempts, context)));
                }
                warn!("Retry {}/{} for {}: {}", attempt, max_attempts, context, e);
                sleep(backoff).await;
            }
        }
    }
}

async fn recreate_container() -> Result<()> {
    info!("[container] stopping container (docker compose down)");
    let down = Command::new("docker")
        .args(["compose", "down"])
        .output()?;
    if !down.status.success() {
        warn!("[container] down stderr: {}", String::from_utf8_lossy(&down.stderr));
    }

    info!("[container] starting container (docker compose up -d)");
    let up = Command::new("docker")
        .args(["compose", "up", "-d"])
        .output()?;
    if !up.status.success() {
        anyhow::bail!(
            "docker compose up failed: {}",
            String::from_utf8_lossy(&up.stderr)
        );
    }
    info!("[container] container recreated");
    Ok(())
}

async fn execute_step(
    client: &LlamaHttpClient,
    config: &StepConfig,
    user_input: &str,
) -> Result<StepResult> {
    info!(
        "[step_start] role={} model_id={}",
        config.role, config.model_id
    );

    let download_ms = if let (Some(repo), Some(filename), Some(local_filename)) =
        (&config.hf_repo, &config.hf_filename, &config.local_filename)
    {
        let dest_path = format!("{}/{}", MODELS_DIR, local_filename);

        if Path::new(&dest_path).exists() {
            info!("Model already exists: {}", dest_path);
            None
        } else {
            info!("[model_download_start] model_id={}", config.model_id);
            let download_start = Instant::now();

            retry_async(
                || download_model_from_hf(repo, filename, &dest_path),
                2,
                Duration::from_secs(5),
                "download model".to_string(),
            )
            .await?;

            let duration = download_start.elapsed().as_millis() as u64;
            info!("[model_download_complete] model_id={} duration_ms={}", config.model_id, duration);
            Some(duration)
        }
    } else {
        None
    };

    // Ensure router knows about this model (may need container recreation after download)
    let needs_discovery = config.hf_repo.is_some();
    if needs_discovery {
        info!("[model_discovery] verifying router visibility for {}", config.model_id);
        match client.list_models().await {
            Ok(models) if models.iter().any(|m| m.id == config.model_id) => {
                info!("[model_discovery] router already knows {}", config.model_id);
            }
            _ => {
                info!(
                    "[model_discovery] router does not see {}, recreating container",
                    config.model_id
                );
                recreate_container().await?;
                info!("[model_discovery] waiting for container HTTP-healthy after recreation");
                client.wait_for_healthy(Duration::from_secs(60)).await?;
                info!("[model_discovery] container HTTP-healthy, polling for {}", config.model_id);
                let post_start = Instant::now();
                loop {
                    match client.list_models().await {
                        Ok(models) if models.iter().any(|m| m.id == config.model_id) => {
                            info!(
                                "[model_discovery] router now sees {} after {}ms",
                                config.model_id,
                                post_start.elapsed().as_millis()
                            );
                            break;
                        }
                        Ok(_) => {}
                        Err(e) => warn!("[model_discovery] poll error: {}", e),
                    }
                    if post_start.elapsed() > Duration::from_secs(15) {
                        anyhow::bail!(
                            "Router did not detect model {} even after container recreation",
                            config.model_id
                        );
                    }
                    sleep(Duration::from_millis(500)).await;
                }
            }
        }
    }

    info!("[model_load_start] model_id={}", config.model_id);
    let load_start = Instant::now();

    let load_result = retry_async(
        || client.load_model(&config.model_id),
        3,
        Duration::from_secs(2),
        "load model".to_string(),
    )
    .await;

    match load_result {
        Ok(_) => {}
        Err(e) => {
            if e.to_string().contains("400") && !e.to_string().contains("already running") {
                return Err(e);
            }
            return Err(e);
        }
    }

    let load_ms = load_start.elapsed().as_millis() as u64;
    info!(
        "[model_load_complete] model_id={} duration_ms={}",
        config.model_id, load_ms
    );

    // Brief pause after load to let proxy connections stabilize
    sleep(Duration::from_millis(500)).await;

    let effective_input = if user_input.len() > config.max_input_chars {
        let truncated = &user_input[..config.max_input_chars];
        warn!(
            "Input truncated from {} to {} chars for {}",
            user_input.len(),
            config.max_input_chars,
            config.role
        );
        truncated
    } else {
        user_input
    };

    let messages = vec![
        ChatMessage::system(&config.system_prompt),
        ChatMessage::user(effective_input),
    ];

    let request = ChatCompletionRequest {
        model: config.model_id.clone(),
        messages,
        max_tokens: Some(config.max_tokens),
        temperature: Some(0.7),
        top_p: Some(0.9),
        ..Default::default()
    };

    info!(
        "[chat_start] model_id={} prompt_chars={}",
        config.model_id,
        user_input.len()
    );
    let inference_start = Instant::now();

    let response = retry_async(
        || client.chat_completion(request.clone()),
        3,
        Duration::from_secs(3),
        "chat completion".to_string(),
    )
    .await?;

    let inference_ms = inference_start.elapsed().as_millis() as u64;
    info!(
        "[chat_complete] model_id={} duration_ms={} prompt_tokens={} completion_tokens={}",
        config.model_id,
        inference_ms,
        response.usage.prompt_tokens,
        response.usage.completion_tokens
    );

    let output = response
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .unwrap_or_default();

    info!("[model_unload_start] model_id={}", config.model_id);
    let unload_start = Instant::now();

    let _ = retry_async(
        || client.unload_model(&config.model_id),
        3,
        Duration::from_secs(2),
        "unload model".to_string(),
    )
    .await;

    let unload_ms = unload_start.elapsed().as_millis() as u64;
    info!(
        "[model_unload_complete] model_id={} duration_ms={}",
        config.model_id, unload_ms
    );

    info!(
        "[step_complete] role={} duration_ms={} tokens={}",
        config.role,
        load_ms + inference_ms + unload_ms,
        response.usage.total_tokens
    );

    Ok(StepResult {
        role: config.role.clone(),
        model_id: config.model_id.clone(),
        output,
        prompt_tokens: response.usage.prompt_tokens,
        completion_tokens: response.usage.completion_tokens,
        total_tokens: response.usage.total_tokens,
        load_ms,
        inference_ms,
        unload_ms,
        download_ms,
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    let env_filter = EnvFilter::from_default_env();
    let filter = env_filter.add_directive("model_chain=info".parse().unwrap());
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(false)
        .init();

    let client = LlamaHttpClient::new(SERVER_URL)?;
    info!("Connecting to {}", SERVER_URL);
    client.wait_for_healthy(Duration::from_secs(30)).await?;
    info!("Server healthy");

    let chain_start = Instant::now();

    let planner_config = StepConfig {
        role: "planner".to_string(),
        model_id: "Qwen2.5-0.5B-Instruct-Q4_K_M".to_string(),
        system_prompt: "Create a numbered plan. Be brief.".to_string(),
        hf_repo: None,
        hf_filename: None,
        local_filename: None,
        max_tokens: 200,
        max_input_chars: 500,
    };

    let planner_result = execute_step(
        &client,
        &planner_config,
        "Create a plan for building a simple REST API in Rust using axum. Output a numbered list of steps.",
    )
    .await?;

    let implementer_config = StepConfig {
        role: "implementer".to_string(),
        model_id: "SmolLM3-Q4_K_M".to_string(),
        system_prompt: "Implement the plan briefly.".to_string(),
        hf_repo: None,
        hf_filename: None,
        local_filename: None,
        max_tokens: 512,
        max_input_chars: 200,
    };

    let implementer_result = execute_step(&client, &implementer_config, &planner_result.output).await?;

    let summarizer_config = StepConfig {
        role: "summarizer".to_string(),
        model_id: "tinyllama-1.1b-chat-v1.0.Q4_K_M".to_string(),
        system_prompt: "Summarize briefly.".to_string(),
        hf_repo: Some("TheBloke/TinyLlama-1.1B-Chat-v1.0-GGUF".to_string()),
        hf_filename: Some("tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf".to_string()),
        local_filename: Some("tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf".to_string()),
        max_tokens: 200,
        max_input_chars: 500,
    };

    let summarizer_result =
        execute_step(&client, &summarizer_config, &implementer_result.output).await?;

    let total_duration_ms = chain_start.elapsed().as_millis() as u64;
    let total_tokens = planner_result.total_tokens
        + implementer_result.total_tokens
        + summarizer_result.total_tokens;

    info!(
        "[chain_complete] total_duration_ms={} steps={}",
        total_duration_ms,
        3
    );

    println!("========================================");
    println!("  Model Chain Results");
    println!("========================================");
    println!("Step 1: {} ({})", planner_result.role, planner_result.model_id);
    println!("  Load:     {}ms", planner_result.load_ms);
    println!("  Infer:    {}ms", planner_result.inference_ms);
    println!("  Unload:   {}ms", planner_result.unload_ms);
    println!(
        "  Tokens:   {} (prompt={}, completion={})",
        planner_result.total_tokens,
        planner_result.prompt_tokens,
        planner_result.completion_tokens
    );
    println!();
    println!("Step 2: {} ({})", implementer_result.role, implementer_result.model_id);
    println!("  Load:     {}ms", implementer_result.load_ms);
    println!("  Infer:    {}ms", implementer_result.inference_ms);
    println!("  Unload:   {}ms", implementer_result.unload_ms);
    println!(
        "  Tokens:   {} (prompt={}, completion={})",
        implementer_result.total_tokens,
        implementer_result.prompt_tokens,
        implementer_result.completion_tokens
    );
    println!();
    println!(
        "Step 3: {} ({})",
        summarizer_result.role, summarizer_result.model_id
    );
    if let Some(download_ms) = summarizer_result.download_ms {
        println!("  Download: {}ms (TinyLlama-1.1B Q4_K_M)", download_ms);
    }
    println!("  Load:     {}ms", summarizer_result.load_ms);
    println!("  Infer:    {}ms", summarizer_result.inference_ms);
    println!("  Unload:   {}ms", summarizer_result.unload_ms);
    println!(
        "  Tokens:   {} (prompt={}, completion={})",
        summarizer_result.total_tokens,
        summarizer_result.prompt_tokens,
        summarizer_result.completion_tokens
    );
    println!();
    println!(
        "Total: {}s | Total tokens: {}",
        total_duration_ms / 1000,
        total_tokens
    );

    Ok(())
}
