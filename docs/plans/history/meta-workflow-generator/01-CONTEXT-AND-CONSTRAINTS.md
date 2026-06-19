# 01 — Context & Constraints

This document fixes the immutable facts of the execution environment. Every
sub-workflow design must respect these.

---

## 1. Model: Qwen3.5-9B (LOCKED)

### 1.1 Source
- **HuggingFace repo:** `unsloth/Qwen3.5-9B-GGUF`
- **File:** `Qwen3.5-9B-UD-Q4_K_XL.gguf` (Unsloth dynamic 4-bit quant, ~6.5 GB)
- **Alternative:** `bartowski/Qwen_Qwen3.5-9B-GGUF` (also has Q8_0)
- **Download command:**
  ```bash
  hf download unsloth/Qwen3.5-9B-GGUF \
      --local-dir /models/qwen3.5-9b \
      --include "*UD-Q4_K_XL*"
  ```

### 1.2 Server CLI (canonical)
```bash
./llama-server \
    -hf unsloth/Qwen3.5-9B-GGUF:UD-Q4_K_XL \
    -c 262144 \
    -ngl 0 \
    -t 5 \
    -np 1 \
    -ctk q8_0 \
    -ctv q8_0 \
    --host 0.0.0.0 \
    --port 8080
```

### 1.2.1 ⚠️ CRITICAL ADDON: Disable Reasoning (REQUIRED)

Qwen3.5-9B is a **reasoning model** that emits `<think>` blocks by default.
Without disabling, ALL token budget is consumed by reasoning and `content`
returns empty. **MUST set** in `docker/docker-compose.yml` environment:

```yaml
environment:
  LLAMA_ARG_REASONING: "off"
```

This applies `--reasoning off` to every chat request globally — no engine
code change needed. Verified working 2026-06-13: clean T1/T2/T3 output,
3.89 tps, zero reasoning tokens.

Alternative per-request override (NOT recommended — requires engine change):
send `"chat_template_kwargs":{"enable_thinking":false}` in the OpenAI request body.

### 1.3 Mapping to Whitt Config

Whitt's `src/config/mod.rs` (lines 99–228) maps these to `LLAMA_ARG_*` env vars consumed
by `docker/entrypoint.sh`:

| llama.cpp flag | Whitt config path | Env var | Default |
|---|---|---|---|
| `-c 262144` | `context.size` | `LLAMA_ARG_CTX_SIZE` | (none) |
| `-ngl 0` | `hosting.gpu_layers` | `LLAMA_ARG_N_GPU_LAYERS` | (none) |
| `-t 5` | `hardware.threads` | `LLAMA_ARG_N_THREADS` | (none) |
| `-np 1` | `sampling.parallel` *(verify)* | `LLAMA_ARG_PARALLEL` | (none) |
| `-ctk q8_0` | `cache.cache_type_k` | `LLAMA_ARG_CACHE_TYPE_K` | `f16` |
| `-ctv q8_0` | `cache.cache_type_v` | `LLAMA_ARG_CACHE_TYPE_V` | `f16` |

⚠️ **NOTE**: The workflow YAML itself cannot override these server-level params (they're
set at server startup, not per-request). Per-request overrides via `model_overrides:`
support only: `max_tokens`, `temperature`, `top_p`, `top_k`, `max_turns` (see
`src/workflow/step.rs:83-97`).

**Therefore**: Qwen3.5-9B server params must be baked into `docker/config.yml` (or
`config.yml` at repo root) before `docker compose up`. The workflow YAMLs reference
the model by name only.

### 1.4 Memory Budget

- Model: ~6.5 GB
- KV cache (256K, Q8_0): ~8–10 GB per slot
- Overhead: ~2–4 GB
- **Total: ~17–22 GB** (require ≥ 32 GB RAM for safety margin)

Expected throughput: **0.5–2 tokens/sec** (CPU-only). Workflow designs must budget
time accordingly. Long steps (max_tokens > 2000) will take minutes — use `timeout:`
generously (≥ 8 minutes per LLM step).

---

## 2. Engine Capability Matrix (VERIFIED)

Findings from background agent research (2026-06-13) cross-referenced against
`src/workflow/step.rs`, `src/workflow/hooks/actions.rs`, `src/benchmark/runner.rs`,
and `src/workflow/execution.rs`.

### 2.1 Hook Triggers (10 defined in `WorkflowHookContext`)

| # | Trigger | Wired in runner? | Notes |
|---|---------|------------------|-------|
| 1 | `before_step_starts` | ✅ `runner.rs:1257` | Pre-execution; fires shell hooks here |
| 2 | `during_step_streaming` | ❌ NOT WIRED | Requires SSE streaming path; stream:false is hardcoded |
| 3 | `after_step_starts` | ✅ `runner.rs:1286` | |
| 4 | `after_step_succeeds` | ✅ `runner.rs:1340` | Primary gate point |
| 5 | `after_step_fails` | ✅ `runner.rs:1318` | Error path |
| 6 | `after_all_retries_exhausted` | ✅ `runner.rs:1355` | Fatal error |
| 7 | `before_gwt_evaluates` | ⚠️ PARTIAL | Logging only |
| 8 | `after_gwt_evaluates` | ⚠️ PARTIAL | Logging only |
| 9 | `on_requires_failed` | ✅ `runner.rs:1535` | Dependency failures |
| 10 | `after_loop_iteration_fails` | ✅ `runner.rs:1592` | Loop iteration errors |

### 2.2 Hook Actions (12 defined in `HookAction`)

| # | Action | Status | Use for |
|---|--------|--------|---------|
| 1 | `Log` | ✅ WIRED | File/stdout logging |
| 2 | `AppendTo` | ✅ WIRED | Variable/file accumulation (CRITICAL for `.md` iteration) |
| 3 | `SaveTo` | ✅ WIRED | Variable/file write (final outputs) |
| 4 | `RouteTo` | ✅ WIRED | GWT-driven branching |
| 5 | `Bookmark` | ✅ WIRED | Checkpoints + state vars |
| 6 | `Notify` | ✅ WIRED | Cross-step coordination |
| 7 | `Fail` | ✅ WIRED | Explicit failure |
| 8 | `Shell` | ✅ WIRED | External command execution (`whitt benchmark` invocations, `cat`, `python3`) |
| 9 | `SkipStep` | ✅ WIRED | Skip current step |
| 10 | `SkipRemaining` | ✅ WIRED | Abort workflow |
| 11 | `Gwt` | ✅ WIRED | Full expression evaluator (lexer+parser) — quality gates |
| 12 | `IterateValues` | ⚠️ STUB at execute_action BUT wired at runner.rs:800-836 via `extract_iterate_values` | **Use as hook in `before_step_starts`** to fan out a step |

### 2.3 Step Types (inferred from keys present)

| Step has keys | Inferred type | Wired? |
|---|---|---|
| `generative_entity` + `prompt` | LLM/agent step | ✅ |
| `tool:` | Tool call step | ✅ (limited tools) |
| only `when:` | Control flow step | ✅ |
| `sub_workflow:` | Sub-workflow step | ❌ **STUB** — engine ignores |
| `loop:` | Loop step | ⚠️ **STUB for validation; works for count via `iterate_values`** |

### 2.4 Template Interpolation

| Syntax | Resolved by | Status |
|--------|-------------|--------|
| `${models.X}` | Parse time, `workflow.rs` | ✅ |
| `${workspace.X}` | Parse time | ✅ |
| `{{step.NAME.output}}` | `runner.rs:1236` `resolve_step_output_templates` | ✅ |
| `{{bookmarks.KEY}}` | `hooks/mod.rs:142` `HookEngine::resolve_templates` | ✅ |
| `{{bookmarks.KEY.stdout}}` | Same | ✅ (for shell outputs) |
| `{{inputs.X}}` | Runtime, `agentic_workflow.inputs` | ✅ |
| `{{loop.iteration_variable}}` | `runner.rs:1198` | ✅ (with `iterate_values`) |
| `{{loop.iteration}}` | `runner.rs:1205` | ✅ |
| `{{sub_workflow.NAME.output}}` | — | ❌ Not wired (sub-workflows are stubs) |

### 2.5 GWT Evaluator (FULLY WIRED)

`src/workflow/hooks/gwt.rs` provides a complete expression evaluator:

- **Operators**: `+`, `-`, `*`, `/`, `==`, `!=`, `>`, `<`, `>=`, `<=`, `&&`, `\|\|`, `!`
- **Literals**: `true`, `false`, `null`, numbers, quoted strings
- **Field paths**: `step.X.output`, `error.is_retryable`, `quality_score`, nested dot access
- **Semantics**: first-match wins across clauses in a `gwt:` block

**Available context fields** (from `src/workflow/hooks/context.rs`):
- `BeforeStepStartsContext`: `step_name`, `timestamp`, `iteration` (if loop)
- `AfterStepSucceedsContext`: `step_name`, `output`, `duration_ms`, `quality_score` (token ratio), `total_tokens`, `model_name`
- `AfterStepFailsContext`: `step_name`, `error.is_retryable`, `error.message`, `error.type`, `attempts`
- `AfterAllRetriesExhaustedContext`: `step_name`, `total_attempts`, `last_error`
- `OnRequiresFailedContext`: `failed_step`, `required_step`
- `AfterLoopIterationFailsContext`: `iteration`, `iteration_variable`, `error_message`

⚠️ **CRITICAL**: `quality_score` is **token-ratio** (output_tokens / max_tokens), NOT semantic quality. Do NOT use it as a proxy for content quality. Use Shell + Python script for semantic checks instead.

---

## 3. Schema Source-of-Truth Map

The schema `docs/schema/unified-workflow-schema.yml` (807 lines) defines these features.
Implementer MUST cite schema line numbers when adding new YAML keys.

| Feature | Schema Line(s) | Engine Support |
|---------|---------------|----------------|
| `providers:` (top-level) | 28-33 | ✅ |
| `models:` (top-level) | 60-80 | ✅ |
| `sub_workflows:` (top-level) | 167-192 | ❌ STUB |
| `agentic_workflow.inputs` (typed) | 214-230 | ✅ |
| `agentic_workflow.retry` (workflow+step) | 247-270 | ✅ |
| `agentic_workflow.when` (default hooks) | 275-281 | ✅ |
| `agentic_workflow.steps` (inferred types) | 283-498 | ✅ partial |
| Step `generative_entity` + `prompt` | 297-344 | ✅ |
| Step `tool:` | 345-365 | ✅ |
| Step `when:` + GWT routing | 367-393 | ✅ |
| Step `sub_workflow:` | 395-411 | ❌ STUB |
| Step `loop:` (count or validation) | 413-454 | ⚠️ partial |
| `workflow_execution_strategy.memory` | 505-528 | ✅ |
| `tool_permissions:` | 608-678 | ✅ |
| `memory:` (RAG) | 684-698 | ✅ |
| `workspace:` | 703-724 | ✅ |

### 3.1 Critical Schema Rules (per `AGENTS.md`)

1. **Provider key MUST be `llama_cpp_with_vulkan`** (schema line 28). Never `lmstudio`, `ollama`.
2. **Redundant config is FORBIDDEN**: if `providers` defines host/port, models must NOT duplicate.
3. **Every new key added to YAMLs MUST have a schema line reference comment** (`# schema: LXXX`).
4. **Strict validation**: `WorkflowFile::validate()` rejects unknown top-level keys.
5. **`non-schema extensions`** (`benchmark:`, `model_list:`, `logging:`, `execution:`) are FORBIDDEN at top level.

---

## 4. Existing Infrastructure

### 4.1 Scripts (deterministic post-processing)

| Script | Purpose | Called from |
|--------|---------|-------------|
| `scripts/validate-yaml.py` | Schema validation; prints `VALID:` or `ERROR:` lines | Shell hook in SW5 |
| `scripts/fix-generated-yaml.py` | Strip markdown fences, re-indent steps, remove placeholders, fix `workflow_id:` line 1 | Shell hook in SW5 |
| `scripts/analyze-run.sh` | Parse benchmark log + output for QA evidence | Iteration validation |
| `scripts/validate-iteration.sh` | 8-point evidence gate (per AGENTS.md) | Iteration validation |
| `scripts/generate-workflow.sh` | `__RUN_ID__` + `__TASK_PLACEHOLDER__` sed substitution | Meta-workflow invocation |

### 4.2 Docker

- Compose: `docker/docker-compose.yml` — service `llama-server`, port 8080, healthcheck `/props`
- Entrypoint: `docker/entrypoint.sh` — uses `yq` to parse `config.yml`, `hf download`, starts llama.cpp server
- Vulkan devices: `/dev/dri`, groups `RENDER_GID` + `VIDEO_GID`
- AGENTS.md Vulkan constraints (now relaxed by PR #20797):
  - `--no-cache-prompt` REQUIRED (still — Vulkan cannot serialize KV cache)
  - `--cont-batching` MUST NOT be used (still — same reason)
  - KV cache: previously required `f16`; **now `q8_0` works** via PR #20797 DP4A support (2026-04-13)
  - `--flash-attn on` safe and recommended

### 4.3 Live-Test Pattern (proven in v5 / iter5x)

```
1. Build workflow YAML with `__RUN_ID__` and `__TASK_PLACEHOLDER__` markers
2. Run `scripts/generate-workflow.sh` to substitute markers via sed
3. Run `whitt benchmark --workflow <yaml>` against running Docker llama-server
4. Inspect `docs/benchmarks/outputs/<run-id>/logs/*.log` + `output/*.txt`
5. Run `scripts/analyze-run.sh LOG OUTPUT` for QA evidence
6. Run `scripts/validate-iteration.sh LOG OUTPUT YAML [PREV_LOG]` for 8-point gate
7. Iterate on YAML; repeat
```

---

## 5. Hard Constraints (DO NOT VIOLATE)

1. **No engine modifications** unless explicitly approved by user. Document gaps instead.
2. **No `sub_workflow:` step key** in any YAML. Use shell-based orchestration.
3. **No `loop:` with `validation:` block**. Use `iterate_values` + GWT in `after_step_succeeds`.
4. **No type error suppression** (`as any`, `@ts-ignore`) — N/A here since Rust, but applies to Python scripts.
5. **No commits without explicit user request.**
6. **Live system testing BEFORE unit tests.** Confirm behavior, then codify.
7. **Only Qwen3.5-9B model.** Never fall back to SmolLM3, Phi-4, Granite in any sub-workflow.
8. **Single .md output per SW.** No fan-out of output files within a single SW.
9. **YAML code blocks ONLY in SW4 output.** SW4's `structs.md` must not contain Rust/Python/Bash blocks.
10. **Stop iterating on a SW only when it surpasses Sisyphus's manual single-shot baseline.**

---

## 6. Soft Guidelines

- Prefer `iterate_values` chunks of **3 tasks** initially, scaling to 5 as the SW proves reliable.
- Keep `max_tokens` per LLM call ≤ 4000 to fit Qwen3.5-9B's effective window under CPU slowdown.
- Use `temperature: 0.1–0.3` for analytical steps (SW1–SW4), `0.05` for final YAML assembly (SW5).
- Always pair `save_to` with `log` in `after_step_succeeds` for traceability.
- Always pair `log` (error level) in `after_step_fails` for diagnostics.
- Bookmark intermediate state whenever a step's output is needed by ≥ 2 downstream steps.

---

## 7. Open Questions (RESOLVED)

### Q1: Vulkan + Q8_0 KV cache — does it work?
**RESOLVED (2026-06-13)**: YES. llama.cpp PR #20797 (2026-04-13) added DP4A support for
Q8_0/Q4_0 KV cache under Vulkan. AGENTS.md constraint about `f16`-only is OUTDATED. We
proceed with Q8_0 as requested.

### Q2: CPU-only 256K context — realistic?
**RESOLVED**: Yes on systems with ≥ 32 GB RAM. Expected throughput 0.5–2 tps. Workflow
must use generous timeouts. Will be slow but functional.

### Q3: `qwen/qwen3.5-9b` exact HF path?
**RESOLVED**: Official GGUF at `unsloth/Qwen3.5-9B-GGUF`, file `Qwen3.5-9B-UD-Q4_K_XL.gguf`.
Alternative: `bartowski/Qwen_Qwen3.5-9B-GGUF`.

### Q4: How to invoke sub-workflows?
**RESOLVED**: Use shell-based orchestration. The `sub_workflow:` step key is an engine
STUB. Each sub-workflow is a standalone YAML; meta-workflow invokes via
`shell: whitt benchmark --workflow <sub.yml>` in `before_step_starts` hook.
