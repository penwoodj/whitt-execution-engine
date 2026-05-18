# POC Plan: Bugfixes + YAML Config System

## Objective

Fix user-reported issues, revamp QA docs, implement YAML config system with global defaults, per-model overrides, and machine-wide config. Incremental commits per phase.

## Issues to Fix

### BUG-1: Thinking UI broken (HIGH)
**Symptom**: `[think]` prints after each word token instead of grouped reasoning block.
**Location**: `src/bin/whitt.rs` lines 253-254 (one_shot_chat) and 387-388 (repl_chat).
**Root cause**: Streaming handler prints `[think] {}` per `reasoning_content` delta token instead of buffering.
**Fix**: Buffer reasoning tokens in a `String`. Print grouped block only when `content` delta arrives (transition from thinking to response) or stream ends. Use ANSI formatting for visual separation.
**Verify**: `./target/debug/whitt model swap SmolLM3-Q4_K_M && ./target/debug/whitt chat "Say hello in one word"` — expect grouped `[think]` block then response, NOT per-word `[think]` prefix.

### BUG-2: model_chain truncation warning (MEDIUM)
**Symptom**: `WARN Input truncated from 1708 to 500 chars for summarizer`
**Location**: `src/bin/model_chain.rs` line 350 — `max_input_chars: 500` hardcoded in summarizer StepConfig. Truncation logic at lines 199-210.
**Root cause**: Summarizer max_input_chars too small relative to typical chain output. Not really a bug — it's a config tuning issue. The warning is correct behavior.
**Fix**: Increase summarizer `max_input_chars` from 500 to 4096. Log at DEBUG instead of WARN for truncation (it's expected, not alarming).
**Verify**: `./target/debug/model_chain` — no WARN on truncation for normal inputs. Only warns if input actually exceeds 4096.

### DEFERRED: React agent mode broken
**Symptom**: Agent ReAct loop doesn't work reliably.
**Reason**: Known limitation — 0.5B models can't follow ReAct JSON format. Needs 7B+ model for reliable agent output.
**Action**: Document in QA. Add to future scope plan (see `.sisyphus/plans/cli-and-agentic-cot-poc.md` for later POC phase).

### DEFERRED: amdgpu-container-cli
**Finding**: AMD Container Toolkit provides convenience (CDI, GPU tracker) but NO performance improvement over `/dev/dri` passthrough. Both use same kernel driver. Performance bottleneck is bundled ROCm libs in container, not passthrough method.
**Action**: Document finding in QA. No install needed. Keep current `/dev/dri` approach.

---

## YAML Config System

### Current State
- `config.yml` → docker-compose mounts at `/config/config.yml:ro`
- `docker/entrypoint.sh` reads YAML via `yq`, translates to `LLAMA_ARG_*` env vars → llama.cpp CLI flags
- `src/config/mod.rs` — Rust `LlamaConfig` struct with `to_env_vars()`, serde_saphyr parsing
- Per-model configs exist in `configs/models/` (llama-3.2-1b.yml, llama-3.1-8b.yml, rx-580-polaris.yml) but are NOT wired into runtime
- No global config loader, no per-model override resolution, no machine-wide config, no live-reload

### Target Architecture

```
Priority (highest → lowest):
1. Explicit CLI flag (--temperature 0.5)
2. Per-model override (configs/models/<model-name>.yml)
3. Machine-wide config (~/.config/whitt/config.yml)
4. Docker-mounted config (/config/config.yml)
5. Rust struct defaults (src/config/mod.rs defaults)
```

### Available YAML Properties

All properties map to llama.cpp CLI flags via `LLAMA_ARG_*` env vars:

| Section | Property | Type | Default | llama.cpp Flag |
|---------|----------|------|---------|----------------|
| model | path | PathBuf | required | -m |
| model | huggingface.repo | String | None | (download) |
| model | huggingface.filename | String | None | (download) |
| model | huggingface.branch | String | "main" | (download) |
| model | huggingface.sha256 | String | None | (validation) |
| model | quantization | String | "Q4_K_M" | (validation) |
| model | parameter_count | u64 | None | (validation) |
| context | size | usize | 2048 | -c |
| context | batch_size | usize | 2048 | -b |
| context | ubatch_size | usize | 512 | -ub |
| context | max_context_per_slot | usize | None | --slot-max-context |
| hardware | threads | usize | 4 | -t |
| hardware | gpu_layers | usize | 999 | -ngl |
| hardware | use_mmap | bool | true | -mmap |
| hardware | lock_memory | bool | false | --lock |
| sampling | temperature | f32 | 0.80 | --temp |
| sampling | top_p | f32 | 0.95 | --top-p |
| sampling | top_k | usize | 40 | --top-k |
| sampling | min_p | f32 | None | --min-p |
| sampling | typical_p | f32 | 1.0 | --typical |
| sampling | repeat_penalty | f32 | 1.00 | --repeat-penalty |
| sampling | presence_penalty | f32 | None | --presence-penalty |
| sampling | frequency_penalty | f32 | None | --frequency-penalty |
| sampling | repeat_last_n | usize | 64 | --repeat-last-n |
| sampling | seed | u32 | 0 | --seed |
| sampling | max_tokens | usize | 512 | -n |
| server | host | String | "127.0.0.1" | --host |
| server | port | u16 | 8080 | --port |
| server | parallel | bool | false | --parallel |
| server | timeout | u64 | 600 | --timeout |
| server | max_slots | usize | 8 | --slots |
| server | metrics | bool | true | --metrics |
| server | slots_endpoint | bool | true | --slot-endpoint |
| server | cors_origins | String | "http://localhost:8080" | --cors |
| server | api_key | String | None | --api-key |
| cache | cache_type_k | enum | f16 | --cache-type-k |
| cache | cache_type_v | enum | f16 | --cache-type-v |
| cache | kv_cache_size | usize | None | --kv-cache-size |
| features | log_level | enum | info | --log-level |
| features | profiling | bool | false | --profiling |
| features | print_system_info | bool | true | --system-info |
| features | verbose | bool | false | -v |
| features | color | bool | true | --color |
| vulkan | visible_devices | String | "0" | GGML_VK_VISIBLE_DEVICES |
| vulkan | force_max_allocation | usize | None | GGML_VK_MAX_ALLOCATION |
| vulkan | disable_debug | bool | true | GGML_VK_DISABLE_DEBUG |
| vulkan | enable_validation | bool | false | (debug) |
| vulkan | flash_attention | bool | None | --flash-attn |
| retry | timeout_seconds | u64 | 300 | (client-side) |
| retry | max_retries | u32 | 3 | (client-side) |
| retry | retry_delay_seconds | u64 | 5 | (client-side) |
| docker | memory_limit | usize | None | deploy.resources.limits.memory |
| docker | shm_size | usize | 8 | shm_size |
| docker | cpu_count | usize | None | deploy.resources.limits.cpus |

### Config File Locations

| Priority | Path | Scope | When Loaded |
|----------|------|-------|-------------|
| 4 (lowest) | `/config/config.yml` | Docker-mounted | Container startup |
| 3 | `~/.config/whitt/config.yml` | Machine-wide | `whitt` CLI startup |
| 2 | `configs/models/<model-name>.yml` | Per-model | On model load/swap |
| 1 (highest) | CLI flags | Per-invocation | Every command |

### Live Config Update (Docker)
To pass a new config to running container:
```bash
# Mount new config and restart (docker-compose recreates container)
docker compose down && docker compose up -d

# OR: docker exec with new env vars (temporary, lost on restart)
docker exec whitt-llama-server sh -c 'echo "LLAMA_ARG_TEMP=0.5" >> /config/config.yml'
# Then trigger reload via API:
curl -X POST http://localhost:8080/props  # server-side reload not supported by llama.cpp — requires restart
```

**Note**: llama.cpp does NOT support hot-reload of config changes. Container restart is required. This is an upstream limitation, not fixable in the POC.

---

## Implementation Plan

### Phase 1: Fix Thinking UI (BUG-1)

**Commit**: `fix: buffer reasoning tokens in streaming chat display`

Steps:
1. Add `reasoning_buffer: String` to streaming handlers in whitt.rs (both one_shot_chat and repl_chat)
2. On `reasoning_content` delta: append to buffer (don't print)
3. On first `content` delta: flush reasoning buffer as grouped block with ANSI header, then print content
4. On stream end: flush remaining reasoning buffer if non-empty
5. Add debug logging for reasoning token count
6. Build and verify: `cargo build --bin whitt --features client`
7. QA: swap to SmolLM3, run chat, verify grouped output

### Phase 2: Fix model_chain Truncation (BUG-2)

**Commit**: `fix: increase summarizer input limit and reduce truncation log level`

Steps:
1. Change summarizer `max_input_chars` from 500 to 4096 in model_chain.rs line 350
2. Change truncation log from `warn!` to `debug!` in model_chain.rs line ~205
3. Build and verify: `cargo build --bin model_chain --features client`
4. QA: run model_chain, verify no WARN for normal inputs

### Phase 3: YAML Config — Global Loader + Merge Logic

**Commit**: `feat: add global config loader with per-model override resolution`

Steps:
1. Add `ConfigLoader` to `src/config/mod.rs`:
   - `load_merged(model_name: Option<&str>) -> LlamaConfig`
   - Priority: docker config → machine-wide → struct defaults
   - Per-model: if model_name given, merge `configs/models/<model>.yml` over base
2. Add merge logic: per-model overrides deep-merge into base config (model section replaced, other sections merged at field level)
3. Add `~/.config/whitt/config.yml` as machine-wide config path
4. Add debug logging for config resolution chain
5. Add unit tests: merge priority, per-model override, missing files handled gracefully
6. Build and verify: `cargo build --features client && cargo test --features client`

### Phase 4: QA Documentation Revamp

**Commit**: `docs: revamp QA with config system, bug fixes, and deferred items`

Steps:
1. Update `docs/QA.md`:
   - Add Section 10: YAML Configuration System
   - Add config file locations table
   - Add per-model config QA steps
   - Add machine-wide config QA steps
   - Update Section 8 (SmolLM3 Thinking) — reference fixed streaming UI
   - Add Section 11: Deferred Items (react agent, amdgpu)
   - Update Known Limitations with new findings
2. Verify all QA steps are executable against live system

### Phase 5: Integration Verification + Push

**Commit**: `chore: dead_code warning cleanup and final integration test`

Steps:
1. Fix dead_code warning for unused `Quit` variant in whitt.rs
2. Run full test suite: `cargo test --features client`
3. Run full QA from docs/QA.md against live container
4. Push all commits to `poc` branch

---

## QA Steps for YAML Config (Phase 3/4 Verification)

### QA-C1: Default Config Load
```bash
# Verify whitt loads docker-mounted config by default
RUST_LOG=debug ./target/debug/whitt server 2>&1 | grep -i "config"
# Expected: debug logs showing config loaded from /config/config.yml (if in container)
#          or fallback to defaults
```

### QA-C2: Machine-Wide Config
```bash
# Create machine-wide config
mkdir -p ~/.config/whitt
cat > ~/.config/whitt/config.yml << 'EOF'
model:
  path: /models/qwen2.5-0.5b-instruct-q4_k_m.gguf
sampling:
  temperature: 0.42
  max_tokens: 1024
EOF

# Verify it's loaded
RUST_LOG=debug ./target/debug/whitt server 2>&1 | grep "temperature"
# Expected: temperature=0.42 visible in debug output
```

### QA-C3: Per-Model Override
```bash
# Create per-model config for SmolLM3
cat > configs/models/SmolLM3-Q4_K_M.yml << 'EOF'
model:
  path: /models/SmolLM3-Q4_K_M.gguf
sampling:
  temperature: 0.60
  max_tokens: 2048
hardware:
  gpu_layers: 99
EOF

# Load SmolLM3 — should use per-model override
./target/debug/whitt model swap SmolLM3-Q4_K_M
# Expected: loads with temperature=0.60, max_tokens=2048, gpu_layers=99
```

### QA-C4: CLI Flag Override
```bash
# CLI flag should override all config sources
./target/debug/whitt chat "test" --temperature 0.99 --max-tokens 100
# Expected: uses temperature=0.99 and max_tokens=100 regardless of config files
```

### QA-C5: Missing Config Graceful Degradation
```bash
# Remove machine-wide config
rm -rf ~/.config/whitt/config.yml

# Remove per-model config
rm -f configs/models/SmolLM3-Q4_K_M.yml

# Should still work with docker config + defaults
./target/debug/whitt server
# Expected: works fine, logs showing fallback to defaults
```

### QA-C6: Docker Container Config Reload
```bash
# Edit config.yml
# docker compose down && docker compose up -d
# Expected: container starts with new config
# Note: llama.cpp does NOT support hot-reload — restart required
```

---

## Verification Commands

After each phase, run:
```bash
# Build
cargo build --bin whitt --bin model_chain --features client

# Unit tests
cargo test --features client

# LSP diagnostics on changed files
# (verify via lsp_diagnostics tool)
```

After Phase 5, run full QA from `docs/QA.md` against live container.

---

## Deferred Items (Out of POC Scope)

1. **React Agent Mode**: Needs 7B+ model for reliable ReAct format. Track in `cli-and-agentic-cot-poc.md`.
2. **amdgpu-container-cli**: No performance benefit. Keep `/dev/dri` passthrough. Revisit if multi-GPU or GPU partitioning needed.
3. **Hot Config Reload**: llama.cpp upstream limitation. Would require custom server wrapper. Track for post-POC.
4. **Config Validation Schema**: Full JSON Schema or serde validation. Track for post-POC.
