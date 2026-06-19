# META-v6 Master Plan

> **Status:** Planning Phase - Documentation Only
> **Phase:** Meta-Workflow Generator Development
> **Approach:** Plan-driven development with live validation

## Executive Overview

META-v6 is a meta-workflow generator that builds executable YAML workflows from natural language task specifications. The system chains 5 sub-workflows (SW1-SW5) together to transform unstructured task descriptions into validated workflow YAML files with hooks, GWT expressions, and quality scoring.

**Execution Engine:** Rust-based workflow runner (`src/benchmark/runner.rs`) with YAML-driven hook system
**Model Backend:** llama.cpp with Vulkan in Docker (`whitt-llama-server` container)
**Model:** Qwen3-5-9B-Q4_K_M (6.53 GB) at `/models/Qwen3-5-9B-Q4_K_M.gguf`
**Context:** 262144 tokens with Q8_0 KV cache (~11.96 GB total, fits 15.5 GB RAM)

## Objectives

1. **Generate production-ready workflow YAMLs** from natural language task specs
2. **Wire all 7 wired hook triggers** in generated YAMLs with appropriate actions
3. **Achieve quality > baseline** on live user prompt dataset
4. **Validate via iteration protocol** with documented improvements
5. **Build unit test suite** only after live validation passes

## Success Criteria

| Criterion | Pass Threshold | Evidence Required |
|-----------|----------------|-------------------|
| **Output Validity** | 100% of generated YAMLs pass schema validation (`schema_valid=true` in logs) | Log file analysis |
| **Input Coverage** | 100% of input tasks covered in output (no dropped tasks) | Manual verification vs tasks.md |
| **Task Granularity** | ≤5 story points per leaf task | Structs.md parsing |
| **Hook Coverage** | All 7 wired triggers present in generated YAMLs | YAML inspection |
| **GWT Expressions** | All WHEN clauses have valid GWT expressions | GWT evaluator parsing |
| **Quality Score** | ≥80% on manual quality rubric (see `05-QUALITY-BENCHMARK.md`) | Quality report |
| **Live Validation** | All 5 SWs run end-to-end without panics | Script exit codes |
| **No Regressions** | Existing unit tests pass (`cargo test --all-features`) | Test output |
| **Clippy Clean** | 0 warnings (`cargo clippy --all-features -- -W clippy::all`) | Clippy output |

## Architecture Overview

```
User Tasks (tasks.md)
    ↓ SW1: Task Analysis
Outputs Specification (outputs.md)
    ↓ SW2: Output Structure
Categories Design (categories.md)
    ↓ SW3: Category Mapping
Struct Definitions (structs.md)
    ↓ SW4: Struct Generation
Workflow YAML (workflow.yml)
    ↓ SW5: Workflow Assembly
Final Validated Workflow
```

**Data Flow:**
1. Each SW reads input markdown file from previous step
2. Applies templates, reasoning, and quality checks
3. Writes output markdown file to intermediate directory
4. Hooks capture execution state for quality tracking
5. SW5 validates final YAML against schema (`docs/schema/unified-workflow-schema.yml`)

**Hook System:**
- 10 triggers defined in `src/workflow/hooks/context.rs`
- 7 fully wired in runner (`src/benchmark/runner.rs`)
- 2 partially wired (GWT triggers, info logging only)
- 1 dead (during_step_streaming, requires SSE streaming path)

**Execution Engine:**
- CLI: `./target/release/whitt benchmark --workflow <file>`
- Hooks fired at: `src/benchmark/runner.rs:813-848` (execute_hooks_for_trigger)
- Trigger points: runner lines 1257, 1286, 1318, 1340, 1355, 1535, 1592

## Plan Documents

This master plan references the following detailed documents:

1. **01-OBJECTIVES-AND-SCOPE.md** - Detailed objectives from user spec, scope boundaries
2. **02-ARCHITECTURE.md** - System architecture, data flow, component interaction
3. **03-SUBWORKFLOW-SPECIFICATIONS.md** - I/O contracts for SW1-SW5, hook configs
4. **04-ITERATION-PROTOCOL.md** - How to iterate each SW until quality > baseline
5. **05-QUALITY-BENCHMARK.md** - Baseline dataset strategy, scoring rubric
6. **06-HOOKS-STRATEGY.md** - Hook coverage plan, triggers to wire
7. **07-CONFIG-AND-INFRASTRUCTURE.md** - Server config, KV cache, model setup
8. **08-TESTING-STRATEGY.md** - Live testing protocol, unit test strategy
9. **09-EXECUTION-CHECKLIST.md** - Step-by-step execution with gates

## Known Issues

### GWT Evaluator Bug
**Location:** `src/workflow/hooks/gwt.rs`
**Issue:** String equality comparison (`==`) may be broken in GWT evaluator
**Impact:** WHEN clauses with string comparison in GWT expressions may evaluate incorrectly
**Mitigation:** Use numeric comparison or boolean checks where possible
**Status:** Known limitation, documented for future fix

### Partial Hook Wiring
**Issue:** 2/10 triggers partially wired (before_gwt_evaluates, after_gwt_evaluates)
**Location:** `src/workflow/hooks/actions.rs:404, 414`
**Impact:** These triggers fire with info-level logging only, no full action dispatch
**Mitigation:** Plan document 06-HOOKS-STRATEGY.md details when full wire needed
**Status:** Partial support, acceptable for META-v6

### Dead Trigger
**Issue:** during_step_streaming not wired in runner
**Location:** Not fired in `src/benchmark/runner.rs`
**Impact:** Cannot test chunk-level hooks during streaming
**Mitigation:** META-v6 uses `stream:false` in benchmark mode, bypassing this limitation
**Status:** Architectural limitation, acceptable for current scope

## Baseline Strategy

**Baseline Dataset:** Real user prompts from opencode session DB
**Collection:** Export 50-100 natural language task descriptions
**Storage:** `docs/plans/meta-v6/baseline/prompts/` (one .md per prompt)
**Single-Shot Baselines:** `docs/plans/meta-v6/baseline/single-shot/` (baseline YAMLs per prompt)
**Comparison:** Each iteration's output compared to baseline YAMLs for same prompts
**Improvement Criteria:** Quality score > baseline score (see `05-QUALITY-BENCHMARK.md`)

## Quality Benchmark Strategy

**Scoring Dimensions:**
1. Schema validity (pass/fail)
2. Input coverage (percentage)
3. Task granularity (avg story points per leaf)
4. Hook presence (7 triggers × count)
5. GWT expression validity (percentage)
6. Action variety (12 action types used)
7. Template interpolation (nested depth, variables used)

**Pass Criteria:**
- Schema validity: 100%
- Input coverage: 100%
- Task granularity: ≤5
- Hook presence: ≥5 triggers per step
- Quality score: ≥80/100

**Failure Modes:**
- Schema validation errors → immediate failure, stop iteration
- <100% input coverage → iterate YAML generation
- >5 story points per leaf → iterate task decomposition
- <5 triggers per step → iterate hook insertion logic

## Iteration Protocol

**Loop:**
1. Run SW on prompt dataset
2. Collect outputs in `docs/benchmarks/outputs/meta-workflow/meta-<run_id>-sw<N>-<timestamp>/`
3. Analyze vs baseline (compare quality scores)
4. Identify gaps (missing hooks, invalid GWT, poor granularity)
5. Improve YAML (add hooks, fix templates, adjust prompts)
6. Rerun SW
7. Compare quality score vs previous iteration
8. Continue until quality > baseline or max 5 iterations

**Tracking:** `docs/plans/meta-v6/iterations/` (one report per iteration with metrics)

**Gates:**
- Must pass schema validation before quality assessment
- Must improve quality score vs previous iteration to continue
- Stop after 5 iterations with no improvement

## Testing Strategy

**Phase 1: Live Validation**
- Run all 5 SWs on real prompts
- Verify end-to-end execution
- Check log files for `[workflow:end]` events
- Validate outputs against schema

**Phase 2: Unit Tests** (ONLY after Phase 1 passes)
- Test hook execution paths
- Test GWT expression evaluation
- Test template interpolation
- Test schema validation
- Reference: `docs/qa/extended-poc/QA-AREAS-EXTENDED-POC.md`

**Why this order:** Live validation finds architectural issues faster. Unit tests only after behavior validated.

## Execution Checklist

**Prerequisites:**
- Docker container `whitt-llama-server` running on port 8080
- Model loaded: Qwen3-5-9B-Q4_K_M.gguf with Q8_0 KV
- Engine built: `cargo build --release`
- Run scripts executable: `chmod +x scripts/meta-v6/run-sw{1,2,3,4,5}.sh`

**Execution Order:**
1. Run SW1 on baseline prompts
2. Validate SW1 outputs (check markdown structure, no panics)
3. Run SW2 on SW1 outputs
4. Validate SW2 outputs (check categories present)
5. Run SW3 on SW2 outputs
6. Validate SW3 outputs (check struct definitions)
7. Run SW4 on SW3 outputs
8. Validate SW4 outputs (check YAML skeleton)
9. Run SW5 on SW4 outputs
10. Validate SW5 outputs (check schema_valid=true in logs)
11. Run quality assessment
12. Document in iteration report
13. If quality < baseline, iterate
14. Repeat until quality > baseline or max iterations reached

**Gates:**
- Gate 1: All SWs run without panics
- Gate 2: All outputs pass schema validation
- Gate 3: Quality score > baseline
- Gate 4: Unit tests pass

## Failure Modes and Mitigation

| Failure Mode | Detection | Mitigation |
|--------------|-----------|------------|
| **OOM in LLM server** | Server crash, port 8080 unresponsive | Reduce KV cache to Q4_0, reduce batch_size |
| **Prompt too long** | Truncation warning in logs | Chunk input, process in multiple passes |
| **Schema validation fails** | `schema_valid=false` in logs | Fix YAML structure, reference schema line numbers |
| **Hooks not firing** | No hook logs in output | Check runner firing points, verify YAML structure |
| **GWT expression invalid** | Parse error in logs | Simplify expression, use numeric/boolean only |
| **Output directory conflict** | File write error | Use timestamp-based run IDs |
| **Infinite loop in SW** | Script timeout (>1800s) | Add max steps constraint, check loop conditions |

## Configuration References

**Engine Config:** `/home/jon/code/whitt-execution-engine/config.yml`
- KV cache: `cache_type_k: "q8_0"`, `cache_type_v: "q8_0"`
- Context: `n_ctx: 262144`
- Parallel: `parallel: 1` (single pipeline)

**Server Config:** `docker/docker-compose.yml` (base, not AMD/NVIDIA variant)
- Image: llama.cpp with Vulkan
- Port: 8080
- Entrypoint: `['tini', '--', '/entrypoint.sh']`

**Model:** `/models/Qwen3-5-9B-Q4_K_M.gguf`
- Size: 6.53 GB
- Quantization: Q4_K_M
- KV cache: Q8_0 at 262144 tokens (~5.43 GB)

## Run Scripts

**Location:** `scripts/meta-v6/`
- `run-sw1.sh` - Task analysis (tasks.md → outputs.md)
- `run-sw2.sh` - Output structure (outputs.md → categories.md)
- `run-sw3.sh` - Category mapping (categories.md → structs.md)
- `run-sw4.sh` - Struct generation (structs.md → workflow.yml skeleton)
- `run-sw5.sh` - Workflow assembly (workflow.yml skeleton → final validated YAML)

**Common Flags:**
- `--load-timeout 1800` - 30-minute model load timeout
- `--no-cache-prompt` - Required for Vulkan backend (cannot serialize KV cache)
- `--cont-batching` NOT USED - Triggers KV cache serialization on slot release

## Output Locations

**Base Directory:** `docs/benchmarks/outputs/meta-workflow/`
**Pattern:** `meta-<run_id>-sw<N>-<timestamp>/`
- `<run_id>`: Tracked in `.current-meta-run`
- `<N>`: Sub-workflow number (1-5)
- `<timestamp>`: YYYYMMDD-HHMMSS

**Contents:**
- `run.log` - Full execution log
- `output.md` - Generated markdown (SW1-SW4)
- `workflow.yml` - Final YAML (SW5 only)
- `benchmark.log` - Benchmark-specific log

## QA Framework Alignment

**Reference:** `AGENTS.md` (Lines 17-313)
**Mode:** Engineering Mode (implementation) → QA Mode (validation) → Engineering Mode (fix) loop
**Evidence Required:** Unit test output, clippy output, build exit code, LSP diagnostics, manual verification

**QA Area Structure:**
```
docs/qa/phase-XX/
├── QA-CRITERIA.md
├── QA-TEST-CASES.md
├── QA-FINDINGS.md
└── CROSS-REF.md
```

**Failure Severity:**
- HIGH: Blocks functionality (immediate fix)
- MEDIUM: Partial functionality (fix in current cycle)
- LOW: Code quality (document, fix opportunistically)

**Max Cycles:** 3 fix attempts per issue, then escalate

## Next Steps

1. Review this master plan
2. Read detailed specs (01-09)
3. Execute checklist from 09-EXECUTION-CHECKLIST.md
4. Document findings in iteration reports
5. Iterate until quality > baseline

---

**Document Status:** Draft
**Last Updated:** 2026-06-14
**Author:** META-v6 Planning Session
**Review Status:** Pending