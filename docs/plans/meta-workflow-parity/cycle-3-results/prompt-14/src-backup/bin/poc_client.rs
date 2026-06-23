//! PoC client binary for local LLM with Docker.
//!
//! Usage:
//!   cargo run --bin poc_client --features client -- \
//!     --prompt "Hello, world!" --stream
//!
//! With Docker lifecycle:
//!   cargo run --bin poc_client --features client -- \
//!     --start --stop --stream --prompt "Explain Rust in 3 sentences"

use anyhow::{Context, Result};
use clap::Parser;
use futures::StreamExt;
use std::path::PathBuf;
use std::time::Duration;

use whitt_execution_engine::client::docker_manager::DockerManager;
use whitt_execution_engine::client::http_client::LlamaHttpClient;
use whitt_execution_engine::client::types::{ChatCompletionRequest, ChatMessage};

#[derive(Parser, Debug)]
#[command(name = "poc_client")]
#[command(about = "PoC client for local LLM with Docker", long_about = None)]
struct Cli {
    /// Server URL.
    #[arg(short, long, default_value = "http://localhost:8080")]
    url: String,

    /// Config file path (for future use).
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Prompt text.
    #[arg(long)]
    prompt: String,

    /// Stream output.
    #[arg(short, long)]
    stream: bool,

    /// Max tokens to generate.
    #[arg(short, long, default_value = "512")]
    max_tokens: usize,

    /// Temperature.
    #[arg(short = 't', long, default_value = "0.7")]
    temperature: f32,

    /// Top-P.
    #[arg(short = 'p', long, default_value = "0.95")]
    top_p: f32,

    /// Start container via docker compose.
    #[arg(long)]
    start: bool,

    /// Stop container after completion.
    #[arg(long)]
    stop: bool,

    /// Max wait for server healthy (seconds).
    #[arg(long, default_value = "120")]
    wait: u64,

    /// Docker compose file.
    #[arg(long)]
    compose_file: Option<PathBuf>,

    /// Verbose output.
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let client = LlamaHttpClient::new(&cli.url)?;

    // --start: spin up container
    if cli.start {
        let mut dm = DockerManager::new("whitt-llama-server");
        if let Some(ref f) = cli.compose_file {
            dm = dm.with_compose_file(f.display().to_string());
        }
        dm.start().await?;
        println!("Waiting for server healthy…");
        client
            .wait_for_healthy(Duration::from_secs(cli.wait))
            .await
            .context("Server did not become healthy")?;
    }

    // Health check
    match client.health().await {
        Ok(h) => {
            println!("Health: {} (idle={}, processing={})", h.status, h.slots_idle, h.slots_processing);
        }
        Err(e) => {
            eprintln!("Health check failed: {}", e);
            eprintln!("Is llama-server running at {}?", cli.url);
            std::process::exit(1);
        }
    }

    let models = client.list_models().await.context("Failed to list models")?;
    let loaded = models
        .iter()
        .find(|m| m.status.value == "loaded")
        .context("No model loaded. Load one first via whitt model load or scripts/switch-model.sh")?;
    let model_id = &loaded.id;
    println!("Model: {}", model_id);

    // Build request
    let request = ChatCompletionRequest {
        model: model_id.clone(),
        messages: vec![ChatMessage::user(&cli.prompt)],
        max_tokens: Some(cli.max_tokens),
        temperature: Some(cli.temperature),
        top_p: Some(cli.top_p),
        stream: cli.stream,
        ..Default::default()
    };

    if cli.stream {
        println!("\n--- Streaming response ---");
        let mut stream = client
            .chat_completion_stream(request)
            .await
            .context("Failed to start stream")?;

        let mut full = String::new();
        while let Some(item) = stream.next().await {
            match item {
                Ok(chunk) => {
                    for choice in chunk.choices {
                        if let Some(ref content) = choice.delta.content {
                            print!("{}", content);
                            use std::io::Write;
                            std::io::stdout().flush().ok();
                            full.push_str(content);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("\nStream error: {}", e);
                    break;
                }
            }
        }
        println!("\n--- End ({} words) ---", full.split_whitespace().count());
    } else {
        println!("\nSending request…");
        let resp = client.chat_completion(request).await.context("Completion failed")?;

        println!("--- Response ---");
        for choice in resp.choices {
            println!("{}", choice.message.content);
        }
        println!("--- Usage ---");
        println!(
            "  prompt={}, completion={}, total={}",
            resp.usage.prompt_tokens, resp.usage.completion_tokens, resp.usage.total_tokens
        );
    }

    // --stop: tear down container
    if cli.stop {
        let mut dm = DockerManager::new("whitt-llama-server");
        if let Some(ref f) = cli.compose_file {
            dm = dm.with_compose_file(f.display().to_string());
        }
        dm.stop().await?;
    }

    Ok(())
}
