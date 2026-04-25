//! Whitt CLI — unified tool for chatting with local LLMs, managing models, and running agents.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use futures::StreamExt;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::io::Write;
use std::path::PathBuf;

use whitt_execution_engine::client::http_client::LlamaHttpClient;
use whitt_execution_engine::client::model_download::download_model_from_hf;
use whitt_execution_engine::client::types::{
    ChatCompletionRequest, ChatMessage,
};

#[derive(Parser, Debug)]
#[command(name = "whitt")]
#[command(about = "Whitt CLI — chat with local LLMs, manage models, and run agents", long_about = None)]
struct Cli {
    /// Server URL
    #[arg(short, long, default_value = "http://localhost:8080", global = true)]
    url: String,

    /// Model ID (default: first loaded model)
    #[arg(short, long, global = true)]
    model: Option<String>,

    /// Verbose output
    #[arg(short = 'v', long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Chat with the model
    Chat {
        /// Prompt (if provided, one-shot mode; otherwise, REPL mode)
        prompt: Option<String>,

        /// System prompt
        #[arg(long)]
        system: Option<String>,

        /// Temperature (0.0-2.0)
        #[arg(long, default_value = "0.7")]
        temperature: f32,

        /// Max tokens to generate
        #[arg(long, default_value = "512")]
        max_tokens: usize,

        /// Disable streaming (non-interactive)
        #[arg(long)]
        no_stream: bool,

        /// Save conversation to JSON file
        #[arg(long)]
        save: Option<PathBuf>,
    },

    /// Model management
    Model {
        #[command(subcommand)]
        action: ModelAction,
    },

    /// Server health and status
    Server,

    /// Run ReAct agent loop
    Agent {
        /// Task description
        task: String,

        /// Max steps before giving up
        #[arg(long, default_value = "10")]
        max_steps: usize,
    },

    /// Benchmark model performance
    Benchmark {
        /// Prompt text
        #[arg(long, default_value = "The quick brown fox jumps over the lazy dog.")]
        prompt: Option<String>,

        /// Max tokens to generate
        #[arg(long, default_value = "100")]
        max_tokens: usize,

        /// Number of concurrent requests
        #[arg(short = 'c', long, default_value = "1")]
        concurrent: usize,
    },

    /// Download model from HuggingFace
    Download {
        /// HuggingFace repo (e.g. "Qwen/Qwen2.5-0.5B-Instruct-GGUF")
        repo: String,

        /// Filename to download (e.g. "qwen2.5-0.5b-instruct-q4_k_m.gguf")
        #[arg(long)]
        file: Option<String>,

        /// Output directory
        #[arg(short = 'o', long, default_value = "models")]
        output: PathBuf,
    },
}

#[derive(Subcommand, Debug)]
enum ModelAction {
    /// List all models
    List,
    /// Load a model
    Load { model: String },
    /// Unload a model
    Unload { model: String },
    /// Swap to a different model
    Swap { model: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(if cli.verbose {
                    "whitt=debug".parse().unwrap()
                } else {
                    "whitt=info".parse().unwrap()
                }),
        )
        .init();

    match cli.command {
        Commands::Chat {
            prompt,
            system,
            temperature,
            max_tokens,
            no_stream,
            save,
        } => {
            chat_command(&cli.url, cli.model, prompt, system, temperature, max_tokens, no_stream, save).await
        }

        Commands::Model { action } => {
            model_command(&cli.url, action).await
        }

        Commands::Server => {
            server_command(&cli.url).await
        }

        Commands::Agent { task, max_steps } => {
            agent_command(&cli.url, cli.model, task, max_steps, cli.verbose).await
        }

        Commands::Benchmark { prompt, max_tokens, concurrent } => {
            benchmark_command(&cli.url, prompt, max_tokens, concurrent).await
        }

        Commands::Download { repo, file, output } => {
            download_command(&repo, file, &output).await
        }
    }
}

async fn chat_command(
    url: &str,
    model: Option<String>,
    prompt: Option<String>,
    system: Option<String>,
    temperature: f32,
    max_tokens: usize,
    no_stream: bool,
    save: Option<PathBuf>,
) -> Result<()> {
    let client = LlamaHttpClient::new(url)?;

    let model_id = if let Some(m) = model {
        m
    } else {
        let models = client.list_models().await?;
        let loaded = models.iter().find(|m| m.status.value == "loaded");
        match loaded {
            Some(m) => m.id.clone(),
            None => {
                eprintln!("No model loaded and no model specified. Use 'whitt model load <name>' first.");
                anyhow::bail!("No model loaded");
            }
        }
    };

    ensure_model_loaded(&client, &model_id).await?;

    if let Some(p) = prompt {
        one_shot_chat(&client, &model_id, p, system, temperature, max_tokens, no_stream, save).await
    } else {
        repl_chat(&client, &model_id, system, temperature, max_tokens).await
    }
}

async fn ensure_model_loaded(client: &LlamaHttpClient, model_id: &str) -> Result<()> {
    let models = client.list_models().await?;
    if let Some(model) = models.iter().find(|m| m.id == model_id) {
        if model.status.value == "loaded" {
            return Ok(());
        }
    }
    client.load_model(model_id).await?;
    Ok(())
}

async fn one_shot_chat(
    client: &LlamaHttpClient,
    model_id: &str,
    prompt: String,
    system: Option<String>,
    temperature: f32,
    max_tokens: usize,
    no_stream: bool,
    save: Option<PathBuf>,
) -> Result<()> {
    let mut messages = vec![];
    if let Some(sys) = system {
        messages.push(ChatMessage::system(sys));
    }
    messages.push(ChatMessage::user(prompt));

    let request = ChatCompletionRequest {
        model: model_id.to_string(),
        messages: messages.clone(),
        max_tokens: Some(max_tokens),
        temperature: Some(temperature),
        stream: !no_stream,
        ..Default::default()
    };

    tracing::debug!(
        max_tokens = request.max_tokens,
        temperature = request.temperature,
        stream = request.stream,
        model = %request.model,
        "[chat_request] sending request"
    );

    if !no_stream {
        let mut stream = client.chat_completion_stream(request).await?;
        let mut full_response = String::new();
        let mut reasoning_buffer = String::new();

        while let Some(item) = stream.next().await {
            let chunk = item?;
            for choice in &chunk.choices {
                if let Some(ref reasoning) = choice.delta.reasoning_content {
                    reasoning_buffer.push_str(reasoning);
                }
                if let Some(ref content) = choice.delta.content {
                    if !reasoning_buffer.is_empty() {
                        eprintln!("\x1b[2m[thinking]\x1b[0m");
                        eprintln!("\x1b[2m{}\x1b[0m", reasoning_buffer.trim());
                        reasoning_buffer.clear();
                    }
                    print!("{}", content);
                    std::io::stdout().flush().ok();
                    full_response.push_str(content);
                }
            }
        }
        if !reasoning_buffer.is_empty() {
            eprintln!("\x1b[2m[thinking]\x1b[0m");
            eprintln!("\x1b[2m{}\x1b[0m", reasoning_buffer.trim());
        }
        println!();

        messages.push(ChatMessage::assistant(full_response));

        if let Some(path) = save {
            let output = serde_json::to_string_pretty(&messages)?;
            std::fs::write(&path, output)?;
            eprintln!("Saved conversation to {}", path.display());
        }
    } else {
        let response = client.chat_completion(request).await?;
        let choice = response.choices.first();
        let content = choice.map(|c| c.message.content.clone()).unwrap_or_default();
        let reasoning = choice.map(|c| c.message.reasoning_content.clone()).unwrap_or_default();
        if !reasoning.is_empty() && content.is_empty() {
            println!("[reasoning] {}", reasoning);
        } else {
            println!("{}", content);
        }
        eprintln!("Usage: {} tokens", response.usage.total_tokens);

        messages.push(ChatMessage::assistant(content));

        if let Some(path) = save {
            let output = serde_json::to_string_pretty(&messages)?;
            std::fs::write(&path, output)?;
            eprintln!("Saved conversation to {}", path.display());
        }
    }

    Ok(())
}

async fn repl_chat(
    client: &LlamaHttpClient,
    default_model: &str,
    initial_system: Option<String>,
    temperature: f32,
    max_tokens: usize,
) -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    let mut model_id = default_model.to_string();
    let mut system_prompt = initial_system.unwrap_or_else(|| "You are a helpful assistant.".to_string());
    let mut messages = vec![ChatMessage::system(system_prompt.clone())];

    eprintln!("Whitt REPL (Ctrl-C or /exit to quit)");
    eprintln!("Commands: /exit, /quit, /model <name>, /system <prompt>, /clear, /help");

    loop {
        let line = match rl.readline("whitt> ") {
            Ok(l) => l,
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("\nExiting...");
                break;
            }
            Err(e) => {
                anyhow::bail!("Readline error: {}", e);
            }
        };

        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        if trimmed.starts_with('/') {
            match parse_command(trimmed) {
                Some(Cmd::Exit) => {
                    println!("Exiting...");
                    break;
                }
                Some(Cmd::Model(new_model)) => {
                    if new_model != model_id {
                        let _ = client.unload_model(&model_id).await;
                        client.load_model(&new_model).await?;
                        model_id = new_model;
                        messages = vec![ChatMessage::system(system_prompt.clone())];
                        eprintln!("Switched to model: {}", model_id);
                    }
                }
                Some(Cmd::System(new_system)) => {
                    system_prompt = new_system;
                    messages = vec![ChatMessage::system(system_prompt.clone())];
                    eprintln!("System prompt updated");
                }
                Some(Cmd::Clear) => {
                    messages = vec![ChatMessage::system(system_prompt.clone())];
                    eprintln!("Conversation cleared");
                }
                Some(Cmd::Help) => {
                    println!("/exit, /quit - Exit REPL");
                    println!("/model <name> - Switch model and clear history");
                    println!("/system <prompt> - Update system prompt and clear history");
                    println!("/clear - Clear conversation history");
                    println!("/help - Show this help");
                }
                None => {
                    eprintln!("Unknown command: {}. Type /help for commands.", trimmed);
                }
            }
            continue;
        }

        messages.push(ChatMessage::user(trimmed.to_string()));

        let request = ChatCompletionRequest {
            model: model_id.clone(),
            messages: messages.clone(),
            max_tokens: Some(max_tokens),
            temperature: Some(temperature),
            stream: true,
            ..Default::default()
        };

        let mut stream = client.chat_completion_stream(request).await?;
        let mut full_response = String::new();

        print!("Assistant: ");
        std::io::stdout().flush().ok();

        let mut reasoning_buffer = String::new();
        while let Some(item) = stream.next().await {
            let chunk = item?;
            for choice in &chunk.choices {
                if let Some(ref reasoning) = choice.delta.reasoning_content {
                    reasoning_buffer.push_str(reasoning);
                }
                if let Some(ref content) = choice.delta.content {
                    if !reasoning_buffer.is_empty() {
                        eprintln!("\n\x1b[2m[thinking]\x1b[0m");
                        eprintln!("\x1b[2m{}\x1b[0m", reasoning_buffer.trim());
                        reasoning_buffer.clear();
                    }
                    print!("{}", content);
                    std::io::stdout().flush().ok();
                    full_response.push_str(content);
                }
            }
        }
        if !reasoning_buffer.is_empty() {
            eprintln!("\n\x1b[2m[thinking]\x1b[0m");
            eprintln!("\x1b[2m{}\x1b[0m", reasoning_buffer.trim());
        }
        println!();

        messages.push(ChatMessage::assistant(full_response));
    }

    Ok(())
}

enum Cmd {
    Exit,
    Model(String),
    System(String),
    Clear,
    Help,
}

fn parse_command(line: &str) -> Option<Cmd> {
    let parts: Vec<&str> = line.splitn(2, ' ').collect();
    match parts.get(0) {
        Some(&"/exit") | Some(&"/quit") => Some(Cmd::Exit),
        Some(&"/model") => parts.get(1).map(|m| Cmd::Model(m.to_string())),
        Some(&"/system") => parts.get(1).map(|s| Cmd::System(s.to_string())),
        Some(&"/clear") => Some(Cmd::Clear),
        Some(&"/help") => Some(Cmd::Help),
        _ => None,
    }
}

async fn model_command(url: &str, action: ModelAction) -> Result<()> {
    let client = LlamaHttpClient::new(url)?;

    match action {
        ModelAction::List => {
            let models = client.list_models().await?;
            println!("Models:");
            println!("{:<50} {}", "ID", "STATUS");
            println!("{}", "-".repeat(60));
            for model in &models {
                println!("{:<50} {}", model.id, model.status.value);
            }
        }
        ModelAction::Load { model } => {
            client.load_model(&model).await?;
            println!("Model '{}' loaded successfully", model);
        }
        ModelAction::Unload { model } => {
            client.unload_model(&model).await?;
            println!("Model '{}' unloaded successfully", model);
        }
        ModelAction::Swap { model } => {
            let models = client.list_models().await?;
            let loaded = models.iter().find(|m| m.status.value == "loaded");
            if let Some(current) = loaded {
                client.unload_model(&current.id).await?;
            }
            client.load_model(&model).await?;
            println!("Swapped to model '{}'", model);
        }
    }

    Ok(())
}

async fn server_command(url: &str) -> Result<()> {
    let client = LlamaHttpClient::new(url)?;

    let health = client.health().await?;
    println!("Server: {}", url);
    println!("Health: {}", health.status);
    println!("Slots: idle={}, processing={}", health.slots_idle, health.slots_processing);

    let models = client.list_models().await?;
    println!("\nLoaded models:");
    for model in models.iter().filter(|m| m.status.value == "loaded") {
        println!("  - {}", model.id);
    }

    Ok(())
}

async fn agent_command(
    url: &str,
    model: Option<String>,
    task: String,
    max_steps: usize,
    verbose: bool,
) -> Result<()> {
    let client = LlamaHttpClient::new(url)?;

    let model_id = if let Some(m) = model {
        m
    } else {
        let models = client.list_models().await?;
        let loaded = models.iter().find(|m| m.status.value == "loaded");
        match loaded {
            Some(m) => m.id.clone(),
            None => {
                eprintln!("No model loaded. Use 'whitt model load <name>' first.");
                anyhow::bail!("No model loaded");
            }
        }
    };

    ensure_model_loaded(&client, &model_id).await?;

    let system_prompt = r#"You are a ReAct agent that uses tools to solve tasks.
Use the following format:



<act>
{
  "tool": "tool_name",
  "input": {...}
}
</act>

<result>
The output from executing the tool.
</result>

Available tools:
- model_list: List all models (no input)
- model_load: Load a model (input: {"model": "name"})
- model_unload: Unload a model (input: {"model": "name"})
- chat: Send a chat message (input: {"message": "text"})
- file_read: Read a file (input: {"path": "/path/to/file"})
- final_answer: Return the final answer (input: {"answer": "your answer"})

Use final_answer when you have completed the task.
Always think before acting.
Use JSON format for the <act> tag."#;

    let mut messages = vec![
        ChatMessage::system(system_prompt.to_string()),
        ChatMessage::user(format!("Task: {}", task)),
    ];

    let mut parse_failures = 0;

    for step in 0..max_steps {
        tracing::info!("[agent] Step {}", step + 1);

        let request = ChatCompletionRequest {
            model: model_id.clone(),
            messages: messages.clone(),
            max_tokens: Some(1024),
            temperature: Some(0.7),
            stream: false,
            ..Default::default()
        };

        let response = client.chat_completion(request).await?;
        let content = response.choices.first().map(|c| c.message.content.clone()).unwrap_or_default();

        if verbose {
            eprintln!("\n=== Step {} ===", step + 1);
            eprintln!("Thinking:\n{}", content);
        }

        messages.push(ChatMessage::assistant(content.clone()));

        if let Some(tool_result) = parse_and_execute_tool(&client, &content, verbose).await? {
            if verbose {
                eprintln!("Tool result:\n{}", tool_result);
            }
            messages.push(ChatMessage::user(format!("<result>{}</result>", tool_result)));

            if tool_result.contains("FINAL ANSWER:") || tool_result.contains("[FINAL]") {
                println!("{}", tool_result);
                return Ok(());
            }
        } else {
            parse_failures += 1;
            if parse_failures >= 3 {
                eprintln!("Too many parse failures. Stopping.");
                anyhow::bail!("Agent failed to parse response after 3 attempts");
            }
            messages.push(ChatMessage::user("Error: Could not parse your response. Please use the correct format with <think>, <act>, and <result> tags.".to_string()));
        }
    }

    eprintln!("Agent reached max steps without completing the task.");
    Ok(())
}

async fn parse_and_execute_tool(
    client: &LlamaHttpClient,
    content: &str,
    verbose: bool,
) -> Result<Option<String>> {
    use regex::Regex;

    let think_re = Regex::new(r"<think>(.*?)</think>").unwrap();
    let act_re = Regex::new(r"<act>(.*?)</act>").unwrap();

    let _thought = think_re.captures(content).and_then(|c| c.get(1)).map(|m| m.as_str());

    if let Some(act) = act_re.captures(content).and_then(|c| c.get(1)).map(|m| m.as_str()) {
        let tool_call: serde_json::Value = serde_json::from_str(act).context("Failed to parse tool call JSON")?;
        let tool = tool_call.get("tool").and_then(|t| t.as_str()).context("Missing tool name")?;
        let input = tool_call.get("input").context("Missing input")?;

        if verbose {
            eprintln!("Executing tool: {}", tool);
            eprintln!("Input: {}", serde_json::to_string_pretty(input)?);
        }

        let result = match tool {
            "model_list" => {
                let models = client.list_models().await?;
                let output: Vec<String> = models.iter().map(|m| m.id.clone()).collect();
                serde_json::to_string(&output)?
            }
            "model_load" => {
                let model = input.get("model").and_then(|m| m.as_str()).context("Missing 'model' in input")?;
                client.load_model(model).await?;
                format!("Loaded model: {}", model)
            }
            "model_unload" => {
                let model = input.get("model").and_then(|m| m.as_str()).context("Missing 'model' in input")?;
                client.unload_model(model).await?;
                format!("Unloaded model: {}", model)
            }
            "chat" => {
                let message = input.get("message").and_then(|m| m.as_str()).context("Missing 'message' in input")?;
                let models = client.list_models().await?;
                let loaded = models.iter().find(|m| m.status.value == "loaded")
                    .map(|m| m.id.clone())
                    .context("No model loaded")?;

                let req = ChatCompletionRequest {
                    model: loaded,
                    messages: vec![ChatMessage::user(message)],
                    max_tokens: Some(512),
                    temperature: Some(0.7),
                    stream: false,
                    ..Default::default()
                };
                let resp = client.chat_completion(req).await?;
                resp.choices.first().map(|c| c.message.content.clone()).unwrap_or_default()
            }
            "file_read" => {
                let path = input.get("path").and_then(|p| p.as_str()).context("Missing 'path' in input")?;
                std::fs::read_to_string(path).context("Failed to read file")?
            }
            "final_answer" => {
                let answer = input.get("answer").and_then(|a| a.as_str()).context("Missing 'answer' in input")?;
                format!("FINAL ANSWER: {}", answer)
            }
            _ => format!("Unknown tool: {}", tool),
        };

        return Ok(Some(result));
    }

    Ok(None)
}

async fn benchmark_command(url: &str, prompt: Option<String>, max_tokens: usize, concurrent: usize) -> Result<()> {
    let client = LlamaHttpClient::new(url)?;

    let models = client.list_models().await?;
    let loaded = models.iter().find(|m| m.status.value == "loaded")
        .context("No model loaded. Load one first.")?;
    let model_id = &loaded.id;

    let prompt_text = prompt.unwrap_or_else(|| "The quick brown fox jumps over the lazy dog.".to_string());
    println!("Model: {}", model_id);
    println!("Server: {}", url);
    println!("Prompt: {}", prompt_text);
    println!("Max tokens: {}", max_tokens);
    println!("Concurrent: {}", concurrent);
    println!();

    let request = ChatCompletionRequest {
        model: model_id.clone(),
        messages: vec![ChatMessage::user(&prompt_text)],
        max_tokens: Some(max_tokens),
        stream: false,
        ..Default::default()
    };

    println!("--- Single Request ---");
    let start = std::time::Instant::now();
    let resp = client.chat_completion(request).await.context("Benchmark request failed")?;
    let elapsed = start.elapsed();

    let total_tokens = resp.usage.total_tokens;
    let tps = if elapsed.as_secs_f64() > 0.0 {
        total_tokens as f64 / elapsed.as_secs_f64()
    } else {
        0.0
    };

    println!("Elapsed time: {}ms", elapsed.as_millis());
    println!("Total tokens: {}", total_tokens);
    println!("Tokens per second: {:.2}", tps);

    for choice in resp.choices {
        if !choice.message.content.is_empty() {
            println!("Response: {}...", &choice.message.content[..choice.message.content.len().min(80)]);
        }
    }

    println!();
    println!("--- Benchmark Complete ---");
    Ok(())
}

async fn download_command(repo: &str, file: Option<String>, output: &PathBuf) -> Result<()> {
    println!("Repo: {}", repo);

    let filename = match file {
        Some(f) => f,
        None => {
            anyhow::bail!("No filename specified. Use --file to select which GGUF to download.");
        }
    };

    let dest = output.join(&filename);
    if dest.exists() {
        anyhow::bail!("File already exists: {}", dest.display());
    }

    println!("File: {}", filename);
    println!("Output: {}", dest.display());
    println!("Downloading...");

    let bytes = download_model_from_hf(repo, &filename, dest.to_str()
        .context("Invalid output path")?)
        .await
        .context("Download failed")?;

    println!("Downloaded {} bytes ({:.1} MB)", bytes, bytes as f64 / 1_048_576.0);
    Ok(())
}

