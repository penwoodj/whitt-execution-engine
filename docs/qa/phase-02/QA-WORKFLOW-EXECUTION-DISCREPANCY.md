# QA: Workflow Execution Discrepancy — YAML vs Actual Output

## QA Area
Workflow-driven benchmark execution fidelity

## Date
2026-05-04

## Severity
**HIGH** — YAML workflows define prompts, max_tokens, compare_modes, and 4 agentic steps. None execute. Output files claim workflow context but show completely different data.

---

## Finding 1: Prompts Never Read From YAML

### Expected
YAML `benchmark.prompts` contains 3 substantive prompts:
1. "Explain concept of recursion in programming, with a practical example."
2. "Write a Rust function that finds the longest increasing subsequence in a vector."
3. "Analyze tradeoffs between microservices and monolithic architectures."

### Actual
Every model receives: `"The quick brown fox jumps over the lazy dog. (iteration 1)"`

### Root Cause
`src/bin/whitt.rs` line 312:
```rust
let prompts_vec = (0..prompts).map(|i| format!("{} (iteration {})", 
    prompt.clone().unwrap_or_else(|| "The quick brown fox jumps over the lazy dog.".to_string()), 
    i + 1)).collect();
```
CLI `--prompt` flag default overrides everything. No code reads `benchmark.prompts` from YAML.

### Detection Method
1. Read per-model `.log` file → check `prompt:` field
2. Compare against `benchmark.prompts` in workflow YAML
3. If they don't match → FAIL

### Severity
HIGH — renders workflow prompts meaningless

---

## Finding 2: max_tokens Not Read From YAML

### Expected
YAML `benchmark.max_tokens: 128`

### Actual
CLI default `--max-tokens 64` used instead

### Root Cause
`BenchmarkConfig.max_tokens` set from CLI arg only. No code reads YAML `benchmark.max_tokens`.

### Detection Method
1. Check `.log` file `completion_tokens` count
2. If capped at 64 when YAML says 128 → FAIL

---

## Finding 3: compare_modes Not Executed

### Expected
YAML `benchmark.compare_modes: true` → each model runs twice (GPU + CPU)

### Actual
CLI `--compare-gpu-cpu` defaults to false. Only GPU mode runs.

### Root Cause
`BenchmarkConfig.compare_gpu_cpu` set from CLI arg. YAML `benchmark.compare_modes` never read.

### Detection Method
1. Check `.log` file `compare_gpu_cpu: false` vs YAML `compare_modes: true`
2. Count result entries per model — should be 2 (GPU + CPU), not 1

---

## Finding 4: Agentic Steps 2-4 Never Execute

### Expected
4 agentic workflow steps:
1. `benchmark_performance` (loop over models × modes)
2. `refine_document` (oscillate_abstraction per model)
3. `generate_speedup_report` (GPU vs CPU comparison)
4. `generate_report` (combined report)

### Actual
Only step 1 (simplified) runs. Steps 2-4 don't exist in code.

### Root Cause
No workflow execution engine. `BenchmarkRunner::run()` is a monolithic loop, not step-based.

### Detection Method
1. Check output for `refined_plan_*.md` files → absent → FAIL
2. Check output for `speedup_report.json` → absent → FAIL
3. Check if `oscillate_abstraction` step type implemented → no → FAIL

---

## Finding 5: YAML Schema Non-Compliance

### Issues
| YAML Feature | Schema Says | Actual |
|---|---|---|
| `agentic_workflow` format | `steps:` with named steps | Array with `- step:` prefix |
| `model_list` | Not a top-level key | Present at line 135-138 |
| `execution` | Under `workflow_execution_strategy` | Free-floating at line 11 |
| `logging` | Part of workspace/hooks | Free-floating at line 18 |
| `benchmark` | Not a schema key | Custom section at line 29 |
| `providers` | Required section | Absent |
| `workflow_execution_strategy` | Required section | Absent |
| `workspace` | Required section | Absent |

### Detection Method
1. Parse YAML with `serde_saphyr`
2. Validate against unified-workflow-schema.yml structure
3. Unknown keys → WARN, missing required keys → FAIL

---

## Finding 6: Workflow Context Is Metadata-Only

### Expected
`--workflow` flag drives execution: reads prompts, max_tokens, compare_modes, model_list from YAML

### Actual
`--workflow` flag only adds context section to `.log` output. Doesn't affect execution.

### Root Cause
`parse_workflow_context()` extracts fields but doesn't use them to configure the run. Returns `WorkflowContext` struct that only feeds into report text.

### Detection Method
1. Run with `--workflow` but different `--prompts 3` and `--max-tokens 128`
2. If CLI flags override YAML values → FAIL (workflow doesn't drive execution)

---

## Reproduction Steps

1. Start llama.cpp server: `docker compose up -d`
2. Run benchmark:
   ```bash
   cargo run --release --bin whitt --features client -- \
     benchmark --url http://localhost:8081 \
     --model-list test-models-3.txt \
     --prompts 1 --max-tokens 64 \
     --output-dir ./benchmark-attempts/3-model-run \
     --workflow docs/workflows/benchmarks/benchmark-3-models.yml \
     --output table
   ```
3. Read output: `cat benchmark-attempts/3-model-run/Qwen3-4B-Instruct-2507-Q4_K_M.log`
4. Check prompt field → shows "The quick brown fox..." not YAML prompts
5. Check max_tokens → 64 not 128
6. Check compare_modes → false not true
7. Count steps → only benchmark runs, no refine/speedup/report

---

## QA Criteria for Fix Verification

### Criterion 1: Prompt Fidelity
- [ ] Per-model `.log` shows YAML `benchmark.prompts` text
- [ ] Chat-log.md shows YAML prompts
- [ ] Not "The quick brown fox..."

### Criterion 2: Config Fidelity
- [ ] `max_tokens` matches YAML `benchmark.max_tokens`
- [ ] `temperature` matches YAML `benchmark.temperature`
- [ ] `top_p` matches YAML `benchmark.top_p`

### Criterion 3: Mode Fidelity
- [ ] `compare_modes: true` in YAML → each model runs GPU AND CPU
- [ ] Output has 2 entries per model (GPU + CPU)
- [ ] Speedup factor computed

### Criterion 4: Step Execution
- [ ] Step 1 (benchmark_performance) executes
- [ ] Step 2 (refine_document) executes with oscillate_abstraction
- [ ] Step 3 (generate_speedup_report) produces speedup_report.json
- [ ] Step 4 (generate_report) produces final benchmark_report.json

### Criterion 5: Schema Compliance
- [ ] YAML passes validation against unified-workflow-schema.yml
- [ ] Required sections present: providers, models, agentic_workflow, workspace
- [ ] agentic_workflow uses `steps:` map, not `- step:` array

### Criterion 6: Output Completeness
- [ ] `benchmark_results.yaml` accumulates per-model results
- [ ] `benchmark_report.json` has final combined report
- [ ] `benchmark-errors.log` only present on errors
- [ ] `benchmark.log` has step-by-step progress
- [ ] Per-model `.log` files correlate with workflow step that produced them

---

## Severity Assessment

| Finding | Severity | Impact |
|---|---|---|
| Prompts not from YAML | HIGH | Workflow claims meaningful tests, runs trivia |
| max_tokens not from YAML | MEDIUM | Affects response quality |
| compare_modes ignored | HIGH | GPU vs CPU comparison is core feature |
| Steps 2-4 missing | HIGH | 75% of workflow never executes |
| Schema non-compliance | HIGH | YAML won't parse in future engine |
| Context-only workflow flag | MEDIUM | Misleading — looks like it works |

---

## Prevention Checklist

For future QA of workflow-driven features:
1. **Always compare YAML inputs vs actual outputs** — don't trust metadata sections
2. **Run with and without `--workflow`** — verify behavior differs
3. **Check prompt text in output files** — not just timing metrics
4. **Validate YAML against schema** — structural compliance first
5. **Count output artifacts** — compare expected vs actual file count
6. **Read the actual prompt sent to LLM** — found in `.log` inference section
