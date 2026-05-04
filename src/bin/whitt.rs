//! Whitt CLI — unified tool for chatting with local LLMs, managing models, and running agents.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use futures::StreamExt;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::io::Write;
use std::path::{Path, PathBuf};

use whitt_execution_engine::client::http_client::LlamaHttpClient;
use whitt_execution_engine::client::model_download::download_model_from_hf;
use whitt_execution_engine::client::types::{
    ChatCompletionRequest, ChatMessage,
};
use whitt_execution_engine::benchmark::runner::{BenchmarkRunner, BenchmarkConfig};

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

    /// List available models and exit
    #[arg(long, global = true)]
    list_models: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Chat with model
    Chat {
        /// Prompt (if provided, one-shot mode; otherwise, REPL mode)
        prompt: Option<String>,

        /// System prompt
        #[arg(long)]
        system: Option<String>,

        /// Temperature (0.0-2.0). Overrides per-model config.
        #[arg(long)]
        temperature: Option<f32>,

        /// Max tokens to generate. Overrides per-model config.
        #[arg(long)]
        max_tokens: Option<usize>,

        /// Top-p (nucleus) sampling (0.0-1.0)
        #[arg(long)]
        top_p: Option<f32>,

        /// Top-k sampling
        #[arg(long)]
        top_k: Option<usize>,

        /// Repeat penalty (1.0 = disabled)
        #[arg(long)]
        repeat_penalty: Option<f32>,

        /// Presence penalty
        #[arg(long)]
        presence_penalty: Option<f32>,

        /// Frequency penalty
        #[arg(long)]
        frequency_penalty: Option<f32>,

        /// Stop sequences
        #[arg(long)]
        stop: Option<Vec<String>>,

        /// Random seed (0 = random)
        #[arg(long)]
        seed: Option<u32>,

        /// Disable streaming (non-interactive)
        #[arg(long)]
        no_stream: bool,

        /// Read prompt from stdin
        #[arg(long)]
        pipe: bool,

        /// Save conversation to JSON file
        #[arg(long)]
        save: Option<PathBuf>,
    },

    /// Model management
    Model {
        #[command(subcommand)]
        action: ModelAction,
    },

    /// Server management
    Server {
        #[command(subcommand)]
        action: ServerAction,
    },

    /// Run ReAct agent loop
    Agent {
        /// Task description
        task: String,

        /// Max steps before giving up
        #[arg(long, default_value = "10")]
        max_steps: usize,

        /// Allowed tools (comma-separated). Default: all tools.
        #[arg(long)]
        allowed_tools: Option<String>,

        /// Forbidden tools (comma-separated)
        #[arg(long)]
        forbidden_tools: Option<String>,

        /// Allowed file paths (comma-separated). Default: current directory.
        #[arg(long)]
        allowed_paths: Option<String>,

        /// Forbidden file paths (comma-separated)
        #[arg(long)]
        forbidden_paths: Option<String>,
    },

    /// Benchmark model performance
    Benchmark {
        /// Prompt text (can be specified multiple times)
        #[arg(long, default_value = "The quick brown fox jumps over the lazy dog.")]
        prompt: Option<String>,

        /// Max tokens to generate
        #[arg(long, default_value = "100")]
        max_tokens: usize,

        /// Directory to scan for GGUF models (multi-model benchmark)
        #[arg(long)]
        models_dir: Option<String>,

        /// File containing model paths, one per line
        #[arg(long)]
        model_list: Option<String>,

        /// Number of prompts to run per model
        #[arg(long, default_value = "3")]
        prompts: usize,

        /// Output format: table, json, csv
        #[arg(long, default_value = "table")]
        output: String,

        /// Skip models larger than this (bytes)
        #[arg(long)]
        filter_size_max: Option<u64>,

        /// Regex filter on model name
        #[arg(long)]
        filter_name: Option<String>,

        /// Compare GPU vs CPU performance (each model runs twice)
        #[arg(long)]
        compare_gpu_cpu: bool,

        /// Output directory for benchmark files (default: ./workspace)
        #[arg(long)]
        output_dir: Option<String>,

        /// Path to YAML workflow file for context in output reports
        #[arg(long)]
        workflow: Option<String>,
    },

    /// Load and validate unified YAML workflow configuration
    Workflow {
        /// Path to unified YAML workflow file
        workflow_file: PathBuf,

        /// Show loaded configuration details
        #[arg(long)]
        show_config: bool,
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

#[derive(Subcommand, Debug)]
enum ServerAction {
    /// Show server health and loaded models
    Status,
    /// Start the llama.cpp server via docker compose
    Start,
    /// Stop the llama.cpp server via docker compose
    Stop,
    /// Detect GPU type (nvidia/amd/cpu)
    Gpu,
    /// Show server logs (follow mode)
    Logs,
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

    if cli.list_models {
        tracing::info!("[WHT-LM001] --list-models invoked, url={}", cli.url);
        let client = LlamaHttpClient::new(&cli.url)?;
        let models = client.list_models().await?;
        tracing::info!("[WHT-LM002] --list-models returned {} models", models.len());
        for model in &models {
            println!("{}\t{}", model.id, model.status.value);
        }
        return Ok(());
    }

    let command = cli.command.context("No subcommand provided. Use --help for usage.")?;

    match command {
        Commands::Chat {
            prompt,
            system,
            temperature,
            max_tokens,
            top_p,
            top_k,
            repeat_penalty,
            presence_penalty,
            frequency_penalty,
            stop,
            seed,
            no_stream,
            pipe,
            save,
        } => {
            tracing::info!("[WHT-CHAT001] chat command, pipe={}, one_shot={}, stream={}", pipe, prompt.is_some(), !no_stream);
            chat_command(&cli.url, cli.model, prompt, system, temperature, max_tokens, top_p, top_k, repeat_penalty, presence_penalty, frequency_penalty, stop, seed, no_stream, pipe, save).await
        }

        Commands::Model { action } => {
            tracing::info!("[WHT-MOD001] model command dispatched");
            model_command(&cli.url, action).await
        }

        Commands::Server { action } => {
            tracing::info!("[WHT-SRV001] server command");
            match action {
                ServerAction::Status => server_status_command(&cli.url).await,
                ServerAction::Start => server_start_command().await,
                ServerAction::Stop => server_stop_command().await,
                ServerAction::Gpu => server_gpu_command().await,
                ServerAction::Logs => server_logs_command().await,
            }
        }

        Commands::Agent { task, max_steps, allowed_tools, forbidden_tools, allowed_paths, forbidden_paths } => {
            tracing::info!("[WHT-AGT001] agent command, max_steps={}, task_len={}", max_steps, task.len());
            agent_command(&cli.url, cli.model, task, max_steps, cli.verbose, AgentOpts { allowed_tools, forbidden_tools, allowed_paths, forbidden_paths }).await
        }

        Commands::Benchmark { prompt, max_tokens, models_dir, model_list, prompts, output, filter_size_max, filter_name, compare_gpu_cpu, output_dir, workflow } => {
            tracing::info!("[WHT-BEN001] benchmark command, max_tokens={}, prompts={}", max_tokens, prompts);

            if models_dir.is_some() || model_list.is_some() {
                let prompts_vec = (0..prompts).map(|i| format!("{} (iteration {})", prompt.clone().unwrap_or_else(|| "The quick brown fox jumps over the lazy dog.".to_string()), i + 1)).collect();

                let config = BenchmarkConfig {
                    server_url: cli.url.clone(),
                    models_dir,
                    model_list_file: model_list,
                    prompts: prompts_vec,
                    max_tokens,
                    filter_size_max,
                    filter_name,
                    delay_between_swaps: std::time::Duration::from_secs(2),
                    compare_gpu_cpu,
                    output_dir,
                    workflow_file: workflow,
                };

                let runner = BenchmarkRunner::new(config);
                let result = runner.run().await.context("Benchmark run failed")?;

                match output.as_str() {
                    "json" => println!("{}", serde_json::to_string_pretty(&result)?),
                    "csv" => println!("{}", result.to_csv()),
                    _ => println!("{}", result.to_table()),
                }

                Ok(())
            } else {
                benchmark_command(&cli.url, prompt, max_tokens).await
            }
        }

        Commands::Download { repo, file, output } => {
            tracing::info!("[WHT-DL001] download command, repo={}", repo);
            download_command(&repo, file, &output).await
        }

        Commands::Workflow { workflow_file, show_config } => {
            tracing::info!("[WHT-WF001] workflow command, file={}", workflow_file.display());
            workflow_command(&workflow_file, show_config).await
        }
    }
}

#[allow(clippy::too_many_arguments)]
async fn chat_command(
    url: &str,
    model: Option<String>,
    prompt: Option<String>,
    system: Option<String>,
    temperature: Option<f32>,
    max_tokens: Option<usize>,
    top_p: Option<f32>,
    top_k: Option<usize>,
    repeat_penalty: Option<f32>,
    presence_penalty: Option<f32>,
    frequency_penalty: Option<f32>,
    stop: Option<Vec<String>>,
    seed: Option<u32>,
    no_stream: bool,
    pipe: bool,
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
                let first = models.first();
                match first {
                    Some(m) => {
                        tracing::info!("[WHT-EML000] no loaded model, trying first available: {}", m.id);
                        m.id.clone()
                    }
                    None => {
                        eprintln!("No models available. Use 'whitt model load <name>' first.");
                        anyhow::bail!("No models available");
                    }
                }
            }
        }
    };

    ensure_model_loaded(&client, &model_id).await?;

    use whitt_execution_engine::config::ConfigLoader;

    let model_config = ConfigLoader::load_merged(Some(&model_id))
        .unwrap_or_else(|e| {
            tracing::debug!("Config load failed, using defaults: {}", e);
            whitt_execution_engine::config::LlamaConfig::default()
        });

    model_config.validate_config()?;

    let temperature = temperature.unwrap_or(model_config.sampling.temperature);
    let max_tokens = max_tokens.unwrap_or(model_config.sampling.max_tokens);
    let top_p = top_p.or(Some(model_config.sampling.top_p));
    let top_k = top_k.or(Some(model_config.sampling.top_k));
    let repeat_penalty = repeat_penalty.or(Some(model_config.sampling.repeat_penalty));
    let presence_penalty = presence_penalty.or(model_config.sampling.presence_penalty);
    let frequency_penalty = frequency_penalty.or(model_config.sampling.frequency_penalty);
    let seed = seed.or(Some(model_config.sampling.seed));

    if !(0.0..=2.0).contains(&temperature) {
        anyhow::bail!("temperature must be between 0.0 and 2.0, got {}", temperature);
    }
    if max_tokens < 1 {
        anyhow::bail!("max_tokens must be >= 1, got {}", max_tokens);
    }

    let resolved_prompt = if pipe {
        tracing::info!("[WHT-PIPE001] reading prompt from stdin");
        use std::io::{self, Read};
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).context("Failed to read from stdin")?;
        if buf.trim().is_empty() {
            anyhow::bail!("--pipe: stdin is empty");
        }
        tracing::info!("[WHT-PIPE002] stdin read OK, {} bytes", buf.len());
        Some(buf)
    } else {
        prompt
    };

    if let Some(p) = resolved_prompt {
        one_shot_chat(&client, &model_id, p, system, temperature, max_tokens, top_p, top_k, repeat_penalty, presence_penalty, frequency_penalty, stop, seed, no_stream, save).await
    } else {
        repl_chat(&client, &model_id, system, temperature, max_tokens, top_p, top_k, repeat_penalty, presence_penalty, frequency_penalty, stop, seed).await
    }
}

async fn ensure_model_loaded(client: &LlamaHttpClient, model_id: &str) -> Result<()> {
    tracing::info!("[WHT-EML001] ensure_model_loaded, model={}", model_id);
    let models = client.list_models().await?;
    if let Some(model) = models.iter().find(|m| m.id == model_id) {
        if model.status.value == "loaded" {
            return Ok(());
        }
    }
    match client.load_model(model_id).await {
        Ok(()) => Ok(()),
        Err(e) => {
            if models.iter().any(|m| m.id == model_id) {
                eprintln!("[MODEL] load API failed (server may auto-load on request): {}", e);
                tracing::warn!("[WHT-EML002] load API failed but model exists in list, continuing");
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
async fn one_shot_chat(
    client: &LlamaHttpClient,
    model_id: &str,
    prompt: String,
    system: Option<String>,
    temperature: f32,
    max_tokens: usize,
    top_p: Option<f32>,
    top_k: Option<usize>,
    repeat_penalty: Option<f32>,
    presence_penalty: Option<f32>,
    frequency_penalty: Option<f32>,
    stop: Option<Vec<String>>,
    seed: Option<u32>,
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
        top_p,
        top_k,
        repeat_penalty,
        presence_penalty,
        frequency_penalty,
        stop: stop.clone(),
        seed,
        stream: !no_stream,
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

#[allow(clippy::too_many_arguments)]
async fn repl_chat(
    client: &LlamaHttpClient,
    default_model: &str,
    initial_system: Option<String>,
    temperature: f32,
    max_tokens: usize,
    top_p: Option<f32>,
    top_k: Option<usize>,
    repeat_penalty: Option<f32>,
    presence_penalty: Option<f32>,
    frequency_penalty: Option<f32>,
    stop: Option<Vec<String>>,
    seed: Option<u32>,
) -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    let mut model_id = default_model.to_string();
    let mut system_prompt = initial_system.unwrap_or_else(|| "You are a helpful assistant.".to_string());
    let mut messages = vec![ChatMessage::system(system_prompt.clone())];

    eprintln!("Whitt REPL (Ctrl-C or /exit to quit)");
    eprintln!("Commands: /exit, /quit, /model <name>, /system <prompt>, /clear, /copy, /help");

    let mut last_response = String::new();

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
                    last_response.clear();
                    eprintln!("Conversation cleared");
                }
                Some(Cmd::Copy) => {
                    tracing::info!("[WHT-COPY001] /copy command invoked, has_response={}", !last_response.is_empty());
                    if last_response.is_empty() {
                        eprintln!("No response to copy yet");
                    } else {
                        copy_to_clipboard(&last_response);
                    }
                }
                Some(Cmd::Help) => {
                    println!("/exit, /quit - Exit REPL");
                    println!("/model <name> - Switch model and clear history");
                    println!("/system <prompt> - Update system prompt and clear history");
                    println!("/clear - Clear conversation history");
                    println!("/copy - Copy last response to clipboard");
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
            top_p,
            top_k,
            repeat_penalty,
            presence_penalty,
            frequency_penalty,
            stop: stop.clone(),
            seed,
            stream: true,
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

        messages.push(ChatMessage::assistant(full_response.clone()));
        last_response = full_response;
    }

    Ok(())
}

enum Cmd {
    Exit,
    Model(String),
    System(String),
    Clear,
    Copy,
    Help,
}

fn parse_command(line: &str) -> Option<Cmd> {
    let parts: Vec<&str> = line.splitn(2, ' ').collect();
    match parts.first() {
        Some(&"/exit") | Some(&"/quit") => Some(Cmd::Exit),
        Some(&"/model") => parts.get(1).map(|m| Cmd::Model(m.to_string())),
        Some(&"/system") => parts.get(1).map(|s| Cmd::System(s.to_string())),
        Some(&"/clear") => Some(Cmd::Clear),
        Some(&"/copy") => Some(Cmd::Copy),
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
            println!("{:<50} STATUS", "ID");
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

async fn server_status_command(url: &str) -> Result<()> {
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

fn detect_gpu_type() -> String {
    if std::path::Path::new("/usr/bin/nvidia-smi").exists() {
        let output = std::process::Command::new("nvidia-smi")
            .args(["--query-gpu=name,driver_version,memory.total", "--format=csv,noheader"])
            .output();
        if let Ok(out) = output {
            if out.status.success() {
                let info = String::from_utf8_lossy(&out.stdout);
                println!("NVIDIA GPU detected:");
                println!("  {}", info.trim());
                return "nvidia".to_string();
            }
        }
    }

    if std::path::Path::new("/dev/kfd").exists() {
        println!("AMD GPU detected (via /dev/kfd)");
        return "amd".to_string();
    }

    let lspci = std::process::Command::new("lspci").output();
    if let Ok(out) = lspci {
        let output_str = String::from_utf8_lossy(&out.stdout);
        if output_str.contains("AMD") && output_str.contains("VGA") {
            println!("AMD GPU detected (via lspci)");
            return "amd".to_string();
        }
    }

    println!("No GPU detected. Running in CPU-only mode.");
    "cpu".to_string()
}

async fn server_start_command() -> Result<()> {
    let gpu_type = detect_gpu_type();
    eprintln!("[SERVER] Starting LLM server with {} GPU...", gpu_type);

    let status = match gpu_type.as_str() {
        "nvidia" => {
            tokio::process::Command::new("docker")
                .args(["compose", "-f", "docker-compose.yml", "-f", "docker-compose.nvidia.yml", "up", "-d"])
                .status().await?
        }
        "amd" => {
            tokio::process::Command::new("docker")
                .args(["compose", "-f", "docker-compose.yml", "-f", "docker-compose.amd.yml", "up", "-d"])
                .status().await?
        }
        _ => {
            tokio::process::Command::new("docker")
                .args(["compose", "up", "-d"])
                .status().await?
        }
    };

    if !status.success() {
        anyhow::bail!("docker compose up failed");
    }
    eprintln!("[SERVER] Server starting. Check health with: whitt server status");
    Ok(())
}

async fn server_stop_command() -> Result<()> {
    eprintln!("[SERVER] Stopping LLM server...");
    let status = tokio::process::Command::new("docker")
        .args(["compose", "down"])
        .status()
        .await?;
    if !status.success() {
        anyhow::bail!("docker compose down failed");
    }
    eprintln!("[SERVER] Server stopped");
    Ok(())
}

async fn server_gpu_command() -> Result<()> {
    let gpu_type = detect_gpu_type();
    println!("\nGPU type: {}", gpu_type);
    match gpu_type.as_str() {
        "nvidia" => println!("Recommended: docker compose -f docker-compose.yml -f docker-compose.nvidia.yml up -d"),
        "amd" => println!("Recommended: docker compose -f docker-compose.yml -f docker-compose.amd.yml up -d"),
        _ => println!("Recommended: docker compose up -d"),
    }
    Ok(())
}

async fn server_logs_command() -> Result<()> {
    tokio::process::Command::new("docker")
        .args(["compose", "logs", "-f"])
        .status()
        .await?;
    Ok(())
}

struct AgentOpts {
    allowed_tools: Option<String>,
    forbidden_tools: Option<String>,
    allowed_paths: Option<String>,
    forbidden_paths: Option<String>,
}

async fn agent_command(
    url: &str,
    model: Option<String>,
    task: String,
    max_steps: usize,
    verbose: bool,
    opts: AgentOpts,
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
                let first = models.first();
                match first {
                    Some(m) => {
                        tracing::info!("[WHT-AGT000] no loaded model, trying first available: {}", m.id);
                        m.id.clone()
                    }
                    None => {
                        eprintln!("No models available. Use 'whitt model load <name>' first.");
                        anyhow::bail!("No models available");
                    }
                }
            }
        }
    };

    ensure_model_loaded(&client, &model_id).await?;

    let forbidden: Vec<String> = opts.forbidden_tools
        .as_deref()
        .unwrap_or("")
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let allowed: Option<Vec<String>> = opts.allowed_tools.as_deref().map(|s| {
        s.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    });

    let all_tools: &[(&str, &str)] = &[
        ("model_list", "List all models (no input)"),
        ("model_load", "Load a model (input: {\"model\": \"name\"})"),
        ("model_unload", "Unload a model (input: {\"model\": \"name\"})"),
        ("chat", "Send a chat message (input: {\"message\": \"text\"})"),
        ("file_read", "Read a file (input: {\"path\": \"/path/to/file\"})"),
        ("final_answer", "Return the final answer (input: {\"answer\": \"your answer\"})"),
    ];

    let available_tools: Vec<_> = all_tools
        .iter()
        .filter(|(name, _)| {
            if forbidden.iter().any(|f| f == name) { return false; }
            if let Some(ref a) = allowed { return a.iter().any(|x| x == name); }
            true
        })
        .collect();

    tracing::info!("[WHT-AGT002] tool filtering: {} available, forbidden={:?}, allowed={:?}", available_tools.len(), forbidden, allowed);

    let tool_list = available_tools
        .iter()
        .map(|(name, desc)| format!("- {}: {}", name, desc))
        .collect::<Vec<_>>()
        .join("\n");

    let system_prompt = format!(r#"You are a ReAct agent that uses tools to solve tasks.
Use the following format:



<act>
{{
  "tool": "tool_name",
  "input": {{...}}
}}
</act>

<result>
The output from executing the tool.
</result>

Available tools:
{}

Use final_answer when you have completed the task.
Always think before acting.
 Use JSON format for the <act> tag."#, tool_list);

    let sandbox: Option<whitt_execution_engine::agent::sandbox::SandboxConfig> = if opts.allowed_paths.is_some() || opts.forbidden_paths.is_some() {
        Some(whitt_execution_engine::agent::sandbox::SandboxConfig {
            allowed_paths: opts.allowed_paths.as_deref().unwrap_or("").split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect(),
            forbidden_paths: opts.forbidden_paths.as_deref().unwrap_or("").split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect(),
            allowed_patterns: vec![],
            max_file_size_mb: 10,
        })
    } else {
        None
    };
    tracing::info!("[WHT-AGT003] sandbox config: allowed_paths={:?}, forbidden_paths={:?}", 
        sandbox.as_ref().map(|s| &s.allowed_paths), sandbox.as_ref().map(|s| &s.forbidden_paths));

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

        if let Some(tool_result) = parse_and_execute_tool(&client, &content, verbose, &forbidden, &allowed, &sandbox).await? {
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
    forbidden: &[String],
    allowed: &Option<Vec<String>>,
    sandbox: &Option<whitt_execution_engine::agent::sandbox::SandboxConfig>,
) -> Result<Option<String>> {
    use regex::Regex;

    let think_re = Regex::new(r"<think>(.*?)</think>").unwrap();
    let act_re = Regex::new(r"<act>([\s\S]*?)</act>").unwrap();

    let _thought = think_re.captures(content).and_then(|c| c.get(1)).map(|m| m.as_str());

    // Take the LAST <act> block — earlier ones may be inside <result> from prior turns
    let act = act_re.captures_iter(content).last().and_then(|c| c.get(1)).map(|m| m.as_str());

    if let Some(act) = act {
        let tool_call: serde_json::Value = serde_json::from_str(act).context("Failed to parse tool call JSON")?;
        let tool = tool_call.get("tool").and_then(|t| t.as_str()).context("Missing tool name")?;
        let input = tool_call.get("input").context("Missing input")?;

        tracing::info!("[WHT-TOOL001] parsed tool call: tool={}", tool);

        if verbose {
            eprintln!("Executing tool: {}", tool);
            eprintln!("Input: {}", serde_json::to_string_pretty(input)?);
        }

        if forbidden.iter().any(|f| f == tool) {
            tracing::warn!("[WHT-TOOL002] tool '{}' blocked by forbidden list", tool);
            return Ok(Some(format!("Error: tool '{}' is forbidden", tool)));
        }
        if let Some(ref a) = allowed {
            if !a.iter().any(|x| x == tool) {
                tracing::warn!("[WHT-TOOL003] tool '{}' not in allowed list", tool);
                return Ok(Some(format!("Error: tool '{}' is not in allowed list", tool)));
            }
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
                tracing::info!("[WHT-TOOL004] file_read, path={}", path);
                if let Some(ref cfg) = sandbox {
                    let sbox = whitt_execution_engine::agent::sandbox::ToolSandbox::new(cfg.clone());
                    match sbox.is_path_allowed(std::path::Path::new(path)) {
                        Ok(false) => {
                            tracing::warn!("[WHT-TOOL005] path '{}' blocked by sandbox", path);
                            return Ok(Some(format!("Error: path '{}' is not allowed", path)));
                        }
                        Err(e) => return Ok(Some(format!("Error validating path: {}", e))),
                        Ok(true) => {}
                    }
                }
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

async fn benchmark_command(url: &str, prompt: Option<String>, max_tokens: usize) -> Result<()> {
    let client = LlamaHttpClient::new(url)?;

    let models = client.list_models().await?;
    let model_id = match models.iter().find(|m| m.status.value == "loaded") {
        Some(loaded) => {
            tracing::info!(model = %loaded.id, "[WHT-BEN002] using loaded model");
            loaded.id.clone()
        }
        None => {
            let first = models.first().context("No models available on server")?;
            tracing::info!(model = %first.id, "[WHT-BEN003] no loaded model, using first available");
            first.id.clone()
        }
    };

    let prompt_text = prompt.unwrap_or_else(|| "The quick brown fox jumps over the lazy dog.".to_string());
    println!("Model: {}", model_id);
    println!("Server: {}", url);
    println!("Prompt: {}", prompt_text);
    println!("Max tokens: {}", max_tokens);
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

async fn download_command(repo: &str, file: Option<String>, output: &Path) -> Result<()> {
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

async fn workflow_command(workflow_file: &Path, show_config: bool) -> Result<()> {
    use whitt_execution_engine::config::unified::UnifiedConfig;

    if !workflow_file.exists() {
        anyhow::bail!("Workflow file not found: {}", workflow_file.display());
    }

    println!("Loading unified workflow: {}", workflow_file.display());

    let config = UnifiedConfig::from_file(workflow_file)
        .context("Failed to load unified config")?;

    println!("\n=== Unified Configuration ===");
    println!("Schema Version: {}", config.schema_version);

    println!("\nProviders:");
    for (name, provider) in &config.providers.providers {
        println!("  - {}", name);
        if let Some(config) = &provider.config {
            println!("      Host: {}:{}", config.host, config.port);
        }
        if let Some(requests) = &provider.requests {
            println!("      Timeout: {}s, Max Retries: {}", requests.request_timeout_secs,
                requests.retry.as_ref().map(|r| r.max_retries).unwrap_or(3));
        }
    }

    println!("\nModels:");
    for (name, model) in &config.models.models {
        println!("  - {}", name);
        println!("      Name: {}", model.name);
        println!("      Host Type: {}", model.host.r#type);
    }

    if show_config {
        println!("\n=== Model Configuration Resolution ===");
        for model_name in config.models.models.keys() {
            match config.resolve_model_config(model_name, None) {
                Ok(resolved) => {
                    println!("  Model: {}", model_name);
                    println!("    Host: {}:{}", resolved.host, resolved.port);
                    println!("    Temperature: {:?}", resolved.temperature);
                    println!("    Max Tokens: {:?}", resolved.max_tokens);
                    println!("    Timeout: {}s", resolved.timeout_secs);
                    println!("    Max Retries: {}", resolved.max_retries);
                }
                Err(e) => {
                    println!("  Model: {}", model_name);
                    println!("    Error: {}", e);
                }
            }
        }
    }

    println!("\n✓ Workflow configuration loaded and validated successfully");
    Ok(())
}

fn copy_to_clipboard(text: &str) {
    #[cfg(feature = "clipboard")]
    {
        match arboard::Clipboard::new().and_then(|mut cb| cb.set_text(text.to_string())) {
            Ok(()) => eprintln!("Copied to clipboard"),
            Err(e) => eprintln!("Clipboard error: {} (SSH/headless?)", e),
        }
    }
    #[cfg(not(feature = "clipboard"))]
    {
        eprintln!("Clipboard not available (build with --features clipboard to enable)");
        let _ = text;
    }
}

