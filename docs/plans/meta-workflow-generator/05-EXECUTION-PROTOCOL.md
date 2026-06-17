# 05 — Execution Protocol (Ralph Loop)

This document defines the strict iteration protocol for building each sub-workflow.
Follow it exactly. Do not skip phases.

---

## Core Principle

> **Build baselines first. Iterate each SW until it surpasses the baseline.**

The user's directive: *"Don't stop iterating on a sub-workflow until it surpasses
the quality and completeness and weakness and ability of [Sisyphus] in opencode on a
single-shot attempt to achieve the same objective given the same input prompt."*

Therefore: every SW must beat the Sisyphus manual single-shot baseline on at least
3 of 5 test prompts before advancing.

---

## Phase 0: Environment Preparation

**Goal:** Qwen3.5-9B loads and serves on Docker llama.cpp with the locked config.

### Steps
1. Download Qwen3.5-9B GGUF:
   ```bash
   hf download unsloth/Qwen3.5-9B-GGUF --local-dir /models/qwen3.5-9b --include "*UD-Q4_K_XL*"
   ```
2. Update `config.yml` (or `docker/config.yml`) with:
   - `context.size: 262144`
   - `hosting.gpu_layers: 0`
   - `hardware.threads: 5`
   - `cache.cache_type_k: q8_0`
   - `cache.cache_type_v: q8_0`
3. Bring up Docker:
   ```bash
   docker compose -f docker/docker-compose.yml up -d --build
   ```
4. Verify health:
   ```bash
   curl -s http://localhost:8080/props | jq .
   curl -s http://localhost:8080/v1/models | jq .
   ```
5. Smoke test:
   ```bash
   curl -s http://localhost:8080/v1/chat/completions \
     -H "Content-Type: application/json" \
     -d '{"model":"qwen35","messages":[{"role":"user","content":"hello"}],"max_tokens":50}' | jq .
   ```

### Exit Criteria
- [ ] `curl /props` returns 200 with model metadata
- [ ] Smoke test returns a coherent response
- [ ] `docker logs llama-server` shows no errors

---

## Phase 1: Test Dataset Construction

**Goal:** Curate 5+ high-complexity prompts from real opencode sessions.

### Steps
1. Read `06-TEST-DATASET.md` for the curated list (Sisyphus-authored).
2. Copy each prompt verbatim into:
   `docs/plans/meta-workflow-generator/artifacts/dataset/prompt-NN.txt`
3. Validate each prompt is:
   - 200+ words OR explicitly multi-phase
   - Contains engineering directives
   - References specific code/files/systems
   - NOT trivially simple

### Exit Criteria
- [ ] 5+ prompts saved in `artifacts/dataset/`
- [ ] Each prompt passes validation above

---

## Phase 2: Baseline Generation (Per SW, Per Prompt)

**Goal:** Sisyphus manually produces the output each SW should produce, for each prompt.
This is the **quality floor** the SW must beat.

### Steps
For each SW (SW1 → SW5), for each prompt in dataset:
1. Sisyphus manually produces the SW's output file (e.g., `tasks.md` for SW1).
2. Save to `artifacts/sw<N>-<name>/baseline-opencode/prompt-NN-<output>`.
3. The baseline is a SINGLE-SHOT attempt — Sisyphus does not iterate on it.
   - The whole point is to set a beatable target for the SW's iterative loop.

### Exit Criteria
- [ ] 5 prompts × 5 SWs = 25 baseline files total
- [ ] Each baseline conforms to the SW's output schema

### Shortcuts
- SW2 baseline can reuse SW1's output (since SW2 reads SW1 output).
- SW3 baseline can reuse SW1+SW2 outputs.
- SW4 baseline can reuse SW1+SW2+SW3.
- SW5 baseline can reuse SW4.
- → If Sisyphus runs prompts sequentially, the baselines chain together naturally.

---

## Phase 3: SW1 Build + Iterate (CRITICAL — Establishes Pattern)

**Goal:** Build `sw1-task-deconstruction.yml` and iterate until it beats baselines.

### Build Sub-Phases (per iteration)

#### Iteration N=1 (Initial)
1. Author `sw1-task-deconstruction.yml` per spec in `03-SUB-WORKFLOW-SPECIFICATIONS/01-task-deconstruction.md`.
2. Substitute `__RUN_ID__` and `__TASK_PLACEHOLDER__`:
   ```bash
   RUN_ID="sw1-iter1-$(date -u +%Y%m%d-%H%M%S)"
   ./scripts/generate-workflow.sh \
       --template docs/benchmarks/workflows/sw1-task-deconstruction.yml \
       --run-id "$RUN_ID" \
       --prompt "$(cat artifacts/dataset/prompt-01.txt)" \
       --output docs/benchmarks/workflows/sw1-task-deconstruction.run.yml
   ```
3. Invoke:
   ```bash
   whitt benchmark --workflow docs/benchmarks/workflows/sw1-task-deconstruction.run.yml \
       --output-dir ./docs/benchmarks/outputs/meta-workflow/$RUN_ID
   ```
4. Inspect logs + output:
   ```bash
   ./scripts/analyze-run.sh \
       ./docs/benchmarks/outputs/meta-workflow/$RUN_ID/logs/sw1.log \
       ./docs/benchmarks/outputs/meta-workflow/$RUN_ID/sw1/tasks.md
   ```
5. Validate iteration gate:
   ```bash
   ./scripts/validate-iteration.sh \
       ./docs/benchmarks/outputs/meta-workflow/$RUN_ID/logs/sw1.log \
       ./docs/benchmarks/outputs/meta-workflow/$RUN_ID/sw1/tasks.md \
       docs/benchmarks/workflows/sw1-task-deconstruction.yml \
       [previous-iteration-log]
   ```
6. Compare against baseline:
   - Use Sisyphus judgment: is SW1 output better/worse than baseline for prompt-01?
   - Capture comparison in `artifacts/sw1-task-deconstruction/iterations/iter-001-comparison.md`.
7. Document in `08-ITERATION-LOG.md` under "SW1 Iterations".

#### Iteration N=2..K (Refine)
- Address specific failures from N-1.
- Re-run on prompt-01.
- Optionally test on prompt-02, prompt-03 to check generalization.
- Continue until SW1 beats baseline on 3 of 5 prompts.

### Exit Criteria
- [ ] `sw1-task-deconstruction.yml` produces output for all 5 prompts without crashes
- [ ] Output passes SW1's 6 quality gates (G1-G6) for at least 3 of 5 prompts
- [ ] Sisyphus judges SW1 output ≥ baseline on 3 of 5 prompts
- [ ] `artifacts/sw1-task-deconstruction/golden/prompt-NN-tasks.md` copied for the 3+ winners

---

## Phase 4: SW2 Build + Iterate

Same protocol as Phase 3, applied to `sw2-desired-output-state.yml`.
Reads SW1's golden output as input.

### Exit Criteria
- [ ] Output passes SW2's 6 quality gates for ≥ 3 of 5 prompts
- [ ] Output judged ≥ baseline on ≥ 3 of 5 prompts
- [ ] `artifacts/sw2-desired-output-state/golden/` populated

---

## Phase 5: SW3 Build + Iterate

Same protocol. Reads SW1 + SW2 golden outputs.

### Exit Criteria
- [ ] Output passes SW3's 6 quality gates for ≥ 3 of 5 prompts
- [ ] Output judged ≥ baseline on ≥ 3 of 5 prompts
- [ ] `artifacts/sw3-agentic-categorization/golden/` populated

---

## Phase 6: SW4 Build + Iterate

Same protocol. Reads SW1 + SW2 + SW3 golden outputs.

### Exit Criteria
- [ ] Output passes SW4's 7 quality gates for ≥ 3 of 5 prompts
- [ ] Zero non-YAML code blocks (verified by `scripts/check-yaml-only-blocks.py`)
- [ ] `artifacts/sw4-yaml-substructure/golden/` populated

---

## Phase 7: SW5 Build + Iterate

Same protocol. Reads SW4 golden output + SW1 tasks (for exhaustive check).

### Exit Criteria
- [ ] Output passes SW5's 5 quality gates for ≥ 3 of 5 prompts
- [ ] `workflow.yml` validates via `scripts/validate-yaml.py`
- [ ] `workflow.yml` executes standalone via `whitt benchmark --workflow workflow.yml`
- [ ] `artifacts/sw5-final-assembly/golden/` populated

---

## Phase 8: META Orchestrator Build + End-to-End Test

**Goal:** Build `meta-workflow-v6.yml` and verify it chains SW1→SW5 correctly.

### Steps
1. Author `meta-workflow-v6.yml` per spec in `04-META-WORKFLOW-SPECIFICATION.md`.
2. Run end-to-end on prompt-01:
   ```bash
   RUN_ID="meta-v6-$(date -u +%Y%m%d-%H%M%S)"
   ./scripts/generate-workflow.sh \
       --template docs/benchmarks/workflows/meta-workflow-v6.yml \
       --run-id "$RUN_ID" \
       --prompt "$(cat artifacts/dataset/prompt-01.txt)" \
       --output docs/benchmarks/workflows/meta-workflow-v6.run.yml
   whitt benchmark --workflow docs/benchmarks/workflows/meta-workflow-v6.run.yml \
       --output-dir ./docs/benchmarks/outputs/meta-workflow/$RUN_ID
   ```
3. Inspect every SW's output in the run dir.
4. Validate final workflow.yml.

### Exit Criteria
- [ ] META runs end-to-end on ≥ 2 prompts without manual intervention
- [ ] Each SW produces its expected output file
- [ ] Final `workflow.yml` is valid + executable
- [ ] Total runtime < 5 hours per prompt

---

## Phase 9: Logging Hardening

**Goal:** Make logs rich enough to diagnose any failure without re-running.

### Steps
1. Audit each step's `event_fields` list across all 6 YAMLs.
2. Add fields: `total_tokens`, `prompt_length`, `output_length`, `iteration_count`.
3. Create `scripts/analyze-meta-run.sh` that parses the run dir and emits a summary.
4. Test by intentionally breaking one SW and verifying logs surface the issue.

### Exit Criteria
- [ ] Every LLM step logs token counts
- [ ] Every loop step logs iteration count
- [ ] `scripts/analyze-meta-run.sh` produces a one-page summary

---

## Phase 10: Unit Tests

**Goal:** Codify confirmed engine behaviors as Rust tests so regressions are caught.

### Steps
1. ONLY after Phase 8 passes — behaviors must be confirmed via live system first.
2. For each SW, identify the engine features it depends on:
   - `iterate_values` parsing in `runner.rs:800-836`
   - GWT evaluation in `hooks/gwt.rs`
   - Template interpolation in `runner.rs:1236-1248`
   - Shell action in `hooks/actions.rs:436-549`
   - RouteTo handling in `runner.rs:2134-2196`
3. Add tests under `tests/meta_workflow/`:
   - `iterate_values_parsing_test.rs`
   - `gwt_quality_gate_test.rs`
   - `template_resolution_test.rs`
   - `shell_action_invoke_test.rs`
   - `route_to_after_gwt_test.rs`
4. Run: `cargo test --test meta_workflow`

### Exit Criteria
- [ ] 5+ test files in `tests/meta_workflow/`
- [ ] All tests pass: `cargo test --all-features`
- [ ] `cargo clippy --all-features -- -W clippy::all` reports 0 warnings
- [ ] `cargo build --release --all-features` exits 0

---

## Hard Rules

1. **NEVER skip phases.** Phase N+1 depends on Phase N's exit criteria being met.
2. **NEVER claim done without evidence.** Every ✅ must reference a log path or output file.
3. **NEVER advance an SW that hasn't beaten baseline.** Loop back; refine prompts.
4. **NEVER write unit tests before live validation.** Live-first; tests codify only.
5. **Maximum 5 iterations per SW before escalation.** If SW doesn't beat baseline in 5
   iterations, escalate to user (or Oracle) with full failure context.

## Escalation Triggers

| Trigger | Action |
|---------|--------|
| SW fails 5 iterations | STOP. Document failures in `08-ITERATION-LOG.md`. Escalate. |
| Engine crashes on valid YAML | Document bug. Decide: fix engine OR work around. |
| Qwen3.5-9B too slow to complete in 8 hours | Reduce max_tokens or chunk size. Re-test. |
| Output quality plateaus below baseline | Consult Oracle for prompt engineering guidance. |
| Schema validation rejects valid-looking YAML | Read schema line cited; adjust YAML. |
