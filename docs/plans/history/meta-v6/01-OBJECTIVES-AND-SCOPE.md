# META-v6 Objectives and Scope

> **Purpose:** Define detailed objectives, scope boundaries, and acceptance criteria for META-v6 meta-workflow generator

## User Spec Objectives

From user specification:
> "Building META-v6 meta-workflow generator in Rust + llama.cpp + Qwen3-5-9B at 262144 ctx. 5 sub-workflows (SW1-SW5) chain together: tasks.md → outputs.md → categories.md → structs.md → workflow.yml. Currently working but need extensive plan file suite documenting intent, strategy, and iteration protocol."

### Primary Objectives

**O1: Generate Executable Workflow YAMLs**
Transform natural language task specifications into production-ready YAML workflows that execute without modification.

**Acceptance Criteria:**
- Output YAMLs pass schema validation (`schema_valid=true` in logs)
- No manual edits required before execution
- All required fields present per `docs/schema/unified-workflow-schema.yml`
- Provider key = `llama_cpp_with_vulkan` (schema line 28)
- Host.type = `llama_cpp_with_vulkan` (schema line 71)

**O2: Cover 100% of Input Tasks**
Every task in input `tasks.md` must be represented in output YAML.

**Acceptance Criteria:**
- No dropped tasks (verified by comparing tasks.md vs workflow steps)
- Task names preserved or semantically equivalent
- Task dependencies preserved via `depends_on`
- Story points assigned to each leaf task (≤5 per leaf)

**O3: Insert Hooks for Control Flow**
Wire hook triggers and actions to control execution flow, error handling, and logging.

**Acceptance Criteria:**
- All 7 wired triggers present: `before_step_starts`, `after_step_starts`, `after_step_fails`, `after_step_succeeds`, `after_all_retries_exhausted`, `on_requires_failed`, `after_loop_iteration_fails`
- Each trigger has at least one action
- Actions use valid syntax per `src/workflow/hooks/actions.rs:174-200`
- GWT expressions use valid syntax per `src/workflow/hooks/gwt.rs`

**O4: Achieve Quality > Baseline**
Generated YAMLs must exceed baseline quality on real user prompt dataset.

**Acceptance Criteria:**
- Quality score ≥80/100 on rubric (see `05-QUALITY-BENCHMARK.md`)
- All quality dimensions pass minimum thresholds
- Iteration protocol shows improvement vs baseline

**O5: Document Iteration Protocol**
Define clear process for improving YAMLs until quality > baseline.

**Acceptance Criteria:**
- Iteration protocol documented in `04-ITERATION-PROTOCOL.md`
- Baseline dataset defined and collected
- Quality tracking metrics defined
- Failure modes and mitigations documented

## Scope Boundaries

### IN SCOPE

**File Paths:**
- Input: `tasks.md` (natural language task descriptions)
- Intermediates: `outputs.md`, `categories.md`, `structs.md`
- Output: `workflow.yml` (executable YAML)
- Scripts: `scripts/meta-v6/run-sw{1,2,3,4,5}.sh`
- Workflows: `docs/benchmarks/workflows/sw{1,2,3,4,5}-*.yml`

**Functionality:**
- Task decomposition into hierarchical steps
- Output structure specification
- Category mapping to workflow steps
- Struct generation for step definitions
- YAML assembly with hooks
- Schema validation
- Quality assessment

**Testing:**
- Live validation on real prompts
- End-to-end SW execution
- Schema validation checks
- Manual quality assessment
- Unit tests (after live validation passes)

**Documentation:**
- This objectives document
- Architecture spec
- Sub-workflow specifications
- Iteration protocol
- Quality benchmark
- Hooks strategy
- Config and infrastructure
- Testing strategy
- Execution checklist

### OUT OF SCOPE

**Source Code Modifications:**
- No changes to `src/workflow/hooks/`
- No changes to `src/benchmark/runner.rs`
- No changes to any Rust source files
- Bug fixes deferred to post-validation

**YAML Workflow Modifications:**
- No changes to `docs/benchmarks/workflows/sw{1,2,3,4,5}-*.yml`
- Existing workflows remain as-is
- Only new generated YAMLs from META-v6

**Test Execution:**
- No `cargo test` runs during planning
- No `cargo clippy` runs during planning
- No test execution in this session
- Validation deferred to execution phase

**LLM Server Invocation:**
- No direct calls to llama.cpp server
- No prompt generation during planning
- Model used only during execution phase

**Vague Content:**
- No aspirational statements without evidence
- No claims without file/code path references
- No "we will improve quality" without metrics
- Every claim must be testable

## Success Metrics

### Quantitative Metrics

| Metric | Target | Measurement Method |
|--------|--------|---------------------|
| **Schema Validity** | 100% | Log file grep for `schema_valid=true` |
| **Input Coverage** | 100% | Manual count tasks.md vs workflow steps |
| **Task Granularity** | ≤5 points/leaf | Parse structs.md, compute avg |
| **Hook Coverage** | ≥5 triggers/step | YAML inspection, count hook configs |
| **GWT Validity** | 100% | GWT evaluator parsing test |
| **Quality Score** | ≥80/100 | Manual rubric (see 05-QUALITY-BENCHMARK.md) |
| **Iteration Improvement** | +5 points/iteration | Compare iteration reports |
| **Max Iterations** | ≤5 | Stop condition |

### Qualitative Metrics

**Q1: Readability**
- Generated YAMLs are human-readable
- Comments explain non-obvious decisions
- Step names match task intent

**Q2: Maintainability**
- Hooks follow DRY principle (common patterns extracted)
- Templates use consistent variable naming
- GWT expressions are clear, not overly nested

**Q3: Executability**
- YAMLs run without panics
- Hooks fire at expected points
- Error handling catches failures gracefully

## Failure Definitions

**Critical Failures (Block Execution):**
- F1: Schema validation fails → Must fix YAML structure immediately
- F2: Server OOM during inference → Must reduce KV cache or batch size
- F3: Hook execution panics → Must fix hook actions or disable failing hooks
- F4: GWT parse errors → Must simplify expressions or fix syntax
- F5: Infinite loop in SW → Must add max steps constraint

**Quality Failures (Require Iteration):**
- QF1: <100% input coverage → Iterate decomposition logic
- QF2: >5 story points per leaf → Iterate task splitting
- QF3: <5 triggers per step → Iterate hook insertion
- QF4: Quality score <80 → Iterate YAML generation prompts
- QF5: No improvement after 3 iterations → Reassess baseline

**Documentation Failures (Acceptable Defer):**
- D1: Missing plan document → Can create during iteration
- D2: Unclear success criteria → Can refine during execution
- D3: Missing baseline data → Can collect after planning

## Scope Constraints

**Model Constraints:**
- Context: 262144 tokens max (hard limit)
- KV cache: Q8_0 at 262144 (~5.43 GB)
- RAM: 15.5 GB total available (model + KV)
- Cannot use Q4_0 KV (mandated Q8_0)

**Infrastructure Constraints:**
- Docker container required (`whitt-llama-server`)
- Port 8080 fixed (no port conflicts)
- `--no-cache-prompt` required for Vulkan
- `--cont-batching` forbidden (triggers KV serialization)

**Execution Constraints:**
- Single model loaded at a time
- Parallel=1 (single pipeline)
- Load timeout: 1800s (30 minutes)
- No streaming support (during_step_streaming dead)

**Schema Constraints:**
- MUST use `docs/schema/unified-workflow-schema.yml` as source of truth
- Provider key MUST be `llama_cpp_with_vulkan`
- No non-schema keys allowed
- Redundant config forbidden

**Testing Constraints:**
- Unit tests ONLY after live validation passes
- Live validation first, unit tests second
- No test execution during planning phase
- Evidence required for all claims

## Dependencies

**Internal Dependencies:**
- `docs/schema/unified-workflow-schema.yml` - Schema source of truth
- `src/workflow/hooks/context.rs` - Hook context definitions
- `src/workflow/hooks/actions.rs` - Hook action implementations
- `src/workflow/hooks/gwt.rs` - GWT expression evaluator
- `src/benchmark/runner.rs` - Runner firing points
- `AGENTS.md` - QA framework and protocols

**External Dependencies:**
- llama.cpp server in Docker
- Qwen3-5-9B-Q4_K_M.gguf model
- Vulkan backend
- Linux host (AMD GPU)

**Data Dependencies:**
- Baseline prompt dataset (50-100 real user prompts)
- Single-shot baseline YAMLs
- Iteration reports for tracking

## Risk Assessment

**High Risks (Must Mitigate):**
- R1: Server OOM due to 262144 context + Q8_0 KV
  - Mitigation: Monitor RAM usage, reduce to Q4_0 if needed
- R2: Prompt truncation exceeds context limit
  - Mitigation: Chunk input, process in multiple passes
- R3: GWT evaluator bug breaks WHEN clauses
  - Mitigation: Use numeric/boolean comparisons, avoid string ==

**Medium Risks (Monitor):**
- R4: Hook actions misbehave in generated YAMLs
  - Mitigation: Start with Log actions only, add complexity gradually
- R5: Quality assessment subjective
  - Mitigation: Define clear rubric with examples
- R6: Baseline dataset not representative
  - Mitigation: Collect diverse real prompts, sample multiple domains

**Low Risks (Accept):**
- R7: YAML generation takes >30 minutes per SW
  - Mitigation: Accept as cost of quality, optimize prompts later
- R8: Iteration stalls at 3 cycles
  - Mitigation: Stop, document findings, escalate if needed

## Exit Criteria

**Planning Complete When:**
1. All 9 plan documents created
2. Each document references real file paths/code
3. Success criteria defined and testable
4. Failure modes documented with mitigations
5. Scope boundaries clear and agreed
6. Risk assessment complete with mitigations
7. Baseline dataset strategy defined
8. Iteration protocol documented with gates
9. Execution checklist complete

**Can Proceed to Execution When:**
1. Planning documents reviewed and approved
2. Baseline prompts collected
3. Single-shot baselines generated
4. Docker server running and tested
5. Engine built and tested
6. Run scripts executable

## Out of Scope Future Work

**Post-Validation (Not Planned):**
- Fix GWT evaluator string comparison bug
- Wire during_step_streaming trigger
- Full wire of GWT triggers (before/after)
- Add streaming support to generated YAMLs
- Optimize prompts for faster generation
- Add multi-model support
- Add provider abstraction layer
- Build comprehensive unit test suite
- Add automated quality scoring
- Add regression testing

**Future Enhancements (Not Planned):**
- Visual workflow editor
- Workflow diffing and merging
- Workflow version control
- Workflow sharing and templates
- Automated hook suggestion
- Hook testing framework
- GWT expression debugger
- Live workflow monitoring
- Workflow analytics

---

**Document Status:** Draft
**Last Updated:** 2026-06-14
**Author:** META-v6 Planning Session
**Review Status:** Pending