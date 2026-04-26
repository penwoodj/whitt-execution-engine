# QA Areas - Whitt Execution Engine POC

Comprehensive QA coverage for all POC functionality.

---

## 1. Server Management

**Description**: Docker container lifecycle management and health monitoring.

**Sub-areas**:
- Container lifecycle (start, stop, restart)
- Health checks and status reporting
- GPU detection (AMD Vulkan, NVIDIA, CPU fallback)
- Server logs streaming
- Server process management

**Status**: CONFIRMED WORKING

**Tested and confirmed**:
- Container starts/stops cleanly via `whitt server start/stop`
- Health checks pass and report server status
- AMD GPU detection works (RX 570 tested)
- Server logs stream correctly with Ctrl+C exit
- GPU command provides correct recommendation (nvidia/amd/cpu)

**Known issues/limitations**:
- Config hot reload not supported by llama.cpp upstream
- `--no-cache-idle-slots` required to attempt Vulkan slot reuse (still crashes on 2nd request, see llama.cpp #20002)

---

## 2. Model Management

**Description**: LLM model listing, loading, unloading, and swapping.

**Sub-areas**:
- Model listing with loaded/unloaded status
- Model swapping (unload + load atomic operation)
- Explicit load/unload operations
- Multiple model support (5 available models)
- Per-model config override loading

**Status**: CONFIRMED WORKING

**Tested and confirmed**:
- All 5 models listed correctly with status
- Model swap works between any two models
- Explicit load/unload operations succeed
- Per-model config overrides loaded on model swap
- Models: Qwen2.5-0.5B, SmolLM3, TinyLlama, qwen2.5-0.5b-instruct, Qwen2.5-7B

**Known issues/limitations**:
- None

---

## 3. Chat

**Description**: LLM inference with streaming/non-streaming responses and sampling controls.

**Sub-areas**:
- Non-streaming one-shot responses
- Streaming real-time responses
- Sampling flag overrides (temperature, top-p, top-k, etc.)
- Conversation history saving to JSON
- REPL (interactive) mode
- System prompts
- Config-driven defaults vs CLI overrides

**Status**: CONFIRMED WORKING

**Tested and confirmed**:
- Non-streaming returns complete response with token usage
- Streaming delivers tokens in real-time
- All sampling flags tested and working (temperature, max-tokens, top-p, top-k, repeat-penalty, presence-penalty, frequency-penalty, stop, seed)
- REPL commands work: `/exit`, `/model <name>`, `/system <prompt>`, `/clear`, `/help`
- Config-driven defaults loaded from per-model configs
- CLI flags override config values

**Known issues/limitations**:
- SmolLM3 thinking tokens consume token budget (known, not a bug). Increase `--max-tokens` for longer responses.

---

## 4. Agent ReAct

**Description**: ReAct (Reasoning + Acting) loop execution for tool-using agents.

**Sub-areas**:
- ReAct loop execution with multi-step reasoning
- Tool calling (model_list, model_load, model_unload, chat, file_read, final_answer)
- Parser handling of `<think/>` and `<act>` blocks
- Multi-step agent workflows
- CPU vs Vulkan GPU backend

**Status**: CONFIRMED WORKING (CPU only)

**Tested and confirmed** (CPU backend):
- ReAct loop executes correctly with Qwen2.5-7B-Instruct
- Parser correctly takes last `<act>` block for tool calls
- 2-step agent loop produces correct answers
- All available tools callable: model_list, model_load, model_unload, chat, file_read, final_answer
- Multi-step reasoning chains work

**Known issues/limitations**:
- Vulkan GPU backend crashes on 2nd request (llama.cpp #20002). Use CPU fallback for agent work.
- 0.5B models (Qwen2.5-0.5B, TinyLlama) cannot reliably follow ReAct JSON format. Requires 7B+ models.
- ⚠️ DEPRECATED for POC on 0.5B models due to format compliance limitations.

---

## 5. YAML Configuration

**Description**: Hierarchical configuration system with multi-source merging and validation.

**Sub-areas**:
- Project config.yml loading (Docker-mounted at /config/config.yml)
- Per-model config overrides (configs/models/*.yml)
- Config merge behavior (CLI > per-model > machine-wide > Docker-mounted > defaults)
- Config validation (range checks via garde)
- Env var translation (YAML → LLAMA_ARG_*)

**Status**: CONFIRMED WORKING

**Tested and confirmed**:
- Config loads correctly from project root `config.yml`
- Per-model overrides apply when model swapped
- Merge behavior verified: CLI overrides per-model overrides machine-wide overrides project overrides defaults
- Non-overridden fields preserved from lower-priority sources
- Validation catches out-of-range values (temperature, context size, etc.)
- Env var translation produces correct LLAMA_ARG_* values

**Known issues/limitations**:
- Invalid YAML keys are silently ignored by serde defaults (no JSON Schema validation yet)
- Per-model configs not wired into Docker container entrypoint (only `whitt` CLI)

---

## 6. Model Chain

**Description**: 3-step model chain binary demonstrating multi-model orchestration.

**Sub-areas**:
- Planner → Implementer → Summarizer workflow
- Model loading/unloading per step
- Sequential execution with state passing
- Multiple model coordination

**Status**: CONFIRMED WORKING

**Tested and confirmed**:
- All 3 steps execute sequentially
- Models swap correctly between steps (Planner: Qwen2.5-0.5B, Implementer: SmolLM3, Summarizer: TinyLlama)
- Full logs printed for each step
- State passed correctly between steps

**Known issues/limitations**:
- None

---

## 7. Benchmark

**Description**: Single-request performance benchmarking with TPS measurement.

**Sub-areas**:
- Single request benchmarking
- Tokens-per-second (TPS) calculation
- Model comparison capabilities
- Concurrent request support (configurable)

**Status**: CONFIRMED WORKING

**Tested and confirmed**:
- Benchmark runs successfully
- TPS measured correctly (typical 50-70 TPS on Qwen2.5-0.5B with Vulkan GPU)
- Elapsed time and total tokens reported
- Response captured and truncated for display

**Known issues/limitations**:
- None

---

## 8. Download

**Description**: HuggingFace model downloading with progress tracking and duplicate protection.

**Sub-areas**:
- HuggingFace model download with progress bars
- Duplicate file protection
- Output directory handling
- SHA256 verification (optional)

**Status**: CONFIRMED WORKING

**Tested and confirmed**:
- Downloads complete successfully
- Progress bars show during download
- Duplicate detection works (error on existing file)
- Output directory created if needed

**Known issues/limitations**:
- None

---

## 9. SmolLM3 Thinking Model

**Description**: Special handling for SmolLM3's reasoning token format.

**Sub-areas**:
- Reasoning token handling
- Thinking block display (dim text)
- Token budget management
- Differentiation from regular response tokens

**Status**: CONFIRMED WORKING

**Tested and confirmed**:
- Reasoning tokens buffered and displayed as single `[thinking]` block
- Thinking block displayed in dim text
- No more per-word `[think]` prefix
- Response tokens displayed separately after thinking block

**Known issues/limitations**:
- SmolLM3 thinking tokens consume token budget (known behavior, not a bug). Increase `--max-tokens` for longer responses.

---

## 10. Docker Infrastructure

**Description**: Container build, GPU passthrough, and configuration mounting.

**Sub-areas**:
- Container build (Dockerfile + build.sh)
- Vulkan GPU passthrough (AMD)
- Config volume mounting
- Health checks
- Entrypoint config translation
- Memory and resource limits

**Status**: CONFIRMED WORKING

**Tested and confirmed**:
- Container builds successfully from Dockerfile
- Vulkan passthrough works (AMD RX 570 tested)
- Config mounted correctly at `/config/config.yml`
- Health checks configured and working
- Entrypoint translates config to LLAMA_ARG_* env vars
- `--no-cache-idle-slots` flag available for Vulkan slot reuse attempt

**Known issues/limitations**:
- `--no-cache-idle-slots` does not fix Vulkan slot reuse crash (still crashes on 2nd request, see llama.cpp #20002)
- amdgpu-container-cli not installed, needs root. Legacy `/dev/dri` passthrough works fine.
- Config hot reload not supported (container restart required)

---

## Summary

| QA Area | Status | Tested |
|---------|--------|--------|
| 1. Server Management | CONFIRMED WORKING | ✅ Extensive manual QA |
| 2. Model Management | CONFIRMED WORKING | ✅ All models tested |
| 3. Chat | CONFIRMED WORKING | ✅ All flags tested |
| 4. Agent ReAct | CONFIRMED WORKING (CPU) | ✅ 7B model verified |
| 5. YAML Configuration | CONFIRMED WORKING | ✅ Merge behavior verified |
| 6. Model Chain | CONFIRMED WORKING | ✅ 3-step chain verified |
| 7. Benchmark | CONFIRMED WORKING | ✅ TPS measured |
| 8. Download | CONFIRMED WORKING | ✅ Downloads verified |
| 9. SmolLM3 Thinking | CONFIRMED WORKING | ✅ Reasoning blocks verified |
| 10. Docker Infrastructure | CONFIRMED WORKING | ✅ AMD GPU tested |

**Overall POC Status**: CONFIRMED WORKING
