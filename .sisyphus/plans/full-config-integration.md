# Full Config Integration & CLI Completion Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Wire ConfigLoader into CLI, add all missing sampling CLI flags, create per-model configs for actual models, migrate remaining shell scripts to Rust, and run full QA verification.

**Architecture:** ConfigLoader already exists in `src/config/mod.rs` with merge logic. Need to wire it into `src/bin/whitt.rs` so chat commands read per-model overrides and expose all sampling parameters via CLI flags. Shell scripts (start/stop/detect-gpu/logs) become `whitt server start/stop/gpu/logs` subcommands.

**Tech Stack:** Rust, clap derive, serde_saphyr, tokio, reqwest

---

## Phase 1: Add Missing CLI Sampling Flags (whitt chat)

### Task 1.1: Add --top-p, --top-k, --repeat-penalty, --presence-penalty, --frequency-penalty, --stop, --seed to Chat subcommand

**Files:**
- Modify: `src/bin/whitt.rs:38-63` (Chat variant in Commands enum)

Add these flags to the `Chat` variant:
```rust
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
```

- [ ] **Step 1:** Add all 7 new fields to Chat variant in Commands enum
- [ ] **Step 2:** Update `chat_command` signature and match arm to pass new fields
- [ ] **Step 3:** Update `one_shot_chat` and `repl_chat` signatures
- [ ] **Step 4:** Build `ChatCompletionRequest` with all fields populated from CLI args
- [ ] **Step 5:** Run `cargo build --bin whitt --features client` — must compile
- [ ] **Step 6:** Run `cargo test --features client` — all tests pass
- [ ] **Step 7:** Commit: `feat(cli): add all sampling flags to chat command`

### Task 1.2: Wire ConfigLoader into chat commands

**Files:**
- Modify: `src/bin/whitt.rs` (add import, use in chat_command)

The ConfigLoader provides per-model defaults. When user specifies `--model X`, load merged config for model X, use its sampling defaults, then override with explicit CLI flags.

```rust
use whitt_execution_engine::config::ConfigLoader;
```

In `chat_command`, after determining model_id:
```rust
let model_config = ConfigLoader::load_merged(Some(&model_id))
    .unwrap_or_else(|e| {
        tracing::debug!("Config load failed, using defaults: {}", e);
        whitt_execution_engine::config::LlamaConfig::default()
    });
```

Then use `model_config.sampling.temperature` etc. as defaults, overridden by explicit CLI args.

- [ ] **Step 1:** Add `ConfigLoader` import to whitt.rs
- [ ] **Step 2:** Load merged config in `chat_command` after model_id determined
- [ ] **Step 3:** Use config defaults for temperature/max_tokens, override with CLI args
- [ ] **Step 4:** Thread config through to `one_shot_chat` and `repl_chat`
- [ ] **Step 5:** Build + test — must pass
- [ ] **Step 6:** Commit: `feat(cli): wire ConfigLoader into chat commands`

---

## Phase 2: Server Management Subcommands (Rust Migration)

### Task 2.1: Add `whitt server start/stop/gpu/logs` subcommands

**Files:**
- Modify: `src/bin/whitt.rs` (replace flat `Server` command with subcommands)

Replace the current `Server` variant with:
```rust
/// Server management
Server {
    #[command(subcommand)]
    action: ServerAction,
},
```

New enum:
```rust
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
    /// Show server logs
    Logs,
}
```

Implementation:
- `Status` → current `server_command` logic
- `Start` → detect GPU, run `docker compose -f docker-compose.yml [-f overlay] up -d`
- `Stop` → `docker compose down`
- `Gpu` → detect nvidia-smi, /dev/kfd, lspci → print GPU type
- `Logs` → `docker compose logs -f` (use tokio::process::Command with stdout inheritance)

- [ ] **Step 1:** Add `ServerAction` enum with Status/Start/Stop/Gpu/Logs variants
- [ ] **Step 2:** Replace `Server` variant to use subcommand
- [ ] **Step 3:** Implement `server_status_command` (existing server_command logic)
- [ ] **Step 4:** Implement `server_start_command` (GPU detect + docker compose up)
- [ ] **Step 5:** Implement `server_stop_command` (docker compose down)
- [ ] **Step 6:** Implement `server_gpu_command` (detect GPU type)
- [ ] **Step 7:** Implement `server_logs_command` (docker compose logs -f)
- [ ] **Step 8:** Build + test — must pass
- [ ] **Step 9:** Commit: `feat(cli): add server start/stop/gpu/logs subcommands`

### Task 2.2: Remove deprecated shell scripts

**Files:**
- Delete: `scripts/start.sh`
- Delete: `scripts/stop.sh`
- Delete: `scripts/detect-gpu.sh`
- Delete: `scripts/logs.sh`
- Modify: `docs/QA.md` (update Section 9)

- [ ] **Step 1:** Remove all 4 shell scripts
- [ ] **Step 2:** Update QA.md Section 9 to show scripts fully removed, replaced by `whitt server *`
- [ ] **Step 3:** Commit: `chore: remove remaining shell scripts, superseded by whitt server subcommands`

---

## Phase 3: Per-Model Configs for Actual Models

### Task 3.1: Create configs for the 4 actual models

**Files:**
- Create: `configs/models/Qwen2.5-0.5B-Instruct-Q4_K_M.yml`
- Create: `configs/models/SmolLM3-Q4_K_M.yml`
- Create: `configs/models/qwen2.5-0.5b-instruct-q4_k_m.yml`
- Create: `configs/models/tinyllama-1.1b-chat-v1.0.Q4_K_M.yml`

Each config should set appropriate sampling defaults for that model. Remove the 3 reference configs for non-existent models.

Example `SmolLM3-Q4_K_M.yml`:
```yaml
model:
  path: /models/SmolLM3-Q4_K_M.gguf
sampling:
  temperature: 0.60
  max_tokens: 2048
context:
  size: 4096
```

- [ ] **Step 1:** Create 4 per-model configs with appropriate defaults
- [ ] **Step 2:** Remove reference configs (llama-3.1-8b.yml, llama-3.2-1b.yml, rx-580-polaris.yml)
- [ ] **Step 3:** Verify ConfigLoader picks them up with `cargo test --features client config::`
- [ ] **Step 4:** Commit: `feat(config): add per-model configs for all 4 loaded models`

---

## Phase 4: Full QA Verification

### Task 4.1: Verify all CLI flags work end-to-end

- [ ] **Step 1:** Build all binaries
- [ ] **Step 2:** `whitt server status` — shows health
- [ ] **Step 3:** `whitt server gpu` — detects GPU type
- [ ] **Step 4:** `whitt chat "Say hi" --top-p 0.9 --top-k 50 --repeat-penalty 1.1 --seed 42` — all flags accepted
- [ ] **Step 5:** `whitt chat "What is 2+2?" --no-stream --max-tokens 100` — respects max_tokens
- [ ] **Step 6:** `whitt chat "Test" --stop "hello" "world"` — stop sequences work
- [ ] **Step 7:** Verify ConfigLoader integration with `RUST_LOG=debug`
- [ ] **Step 8:** Run `cargo test --features client` — all pass

### Task 4.2: Update QA docs

- [ ] **Step 1:** Update QA.md with new subcommands and flags
- [ ] **Step 2:** Update test counts if changed
- [ ] **Step 3:** Commit: `docs: update QA with new CLI flags and server subcommands`

### Task 4.3: Final push

- [ ] **Step 1:** `git push origin poc`

---

## Scope Boundaries

**IN scope:**
- All sampling CLI flags (top_p, top_k, repeat_penalty, presence_penalty, frequency_penalty, stop, seed)
- ConfigLoader wiring into chat commands
- Server management subcommands (start/stop/gpu/logs/status)
- Per-model configs for actual models
- Shell script removal
- QA docs update

**OUT of scope:**
- Agent ReAct fixes (deferred — 0.5B model limitation)
- Hot-reload (llama.cpp doesn't support it)
- Docker entrypoint changes (config mounted read-only)
- amdgpu-container-cli installation (needs root, /dev/dri works)
