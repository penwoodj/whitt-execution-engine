# 06 — Test Dataset

Curated high-complexity prompts harvested from real opencode sessions + existing
test prompts. Used as input to SW1–SW5 for iteration validation.

---

## Selection Criteria

A prompt qualifies for the dataset if it meets ALL of:
1. **Multi-phase**: ≥ 2 distinct phases of work
2. **Engineering-directive**: contains verbs like build, implement, refactor, iterate, design
3. **Specific references**: names files, modules, systems, or tools
4. **Non-trivial scope**: would take a mid-level engineer ≥ 2 days to complete manually
5. **Agentic-flavored**: implies autonomous/iterative/evaluative work

Rejected: trivial prompts ("continue", "yes", "status", one-liner questions).

---

## Dataset

### Prompt 01 — Meta-Workflow Generator v6 (CURRENT — highest complexity)

**Source:** User's active session, 2026-06-13
**File:** `artifacts/dataset/prompt-01.txt`
**Word count:** ~650
**Characteristics:** 5-sub-workflow orchestration, multi-phase pipeline, hooks/iteration/QA

Full text: see user's current task description (the `/ralph-loop` invocation that
launched this plan suite). Save verbatim with minor spelling/grammar fixes only.

---

### Prompt 02 — Single-Model Iterative Benchmark Workflow

**Source:** `ses_18911fba3ffeCV0aETu2P7em8Q` (2026-05-30)
**File:** `artifacts/dataset/prompt-02.txt`
**Word count:** ~78 (lower bound, but multi-requirement)
**Characteristics:** iterative task decomposition, tool integration, sequential pipeline

```
I want you to make a 1 model benchmark that attempts to do the same thing as
benchmark 3 but trying to do it iteratively through a sequence of broken down
tasks iterating on one string output. It should use tools and pass the output
of the previous step into the input of the next step in sequence. Try to make
the steps iteratively achieve the objective vs trying to achieve it all in
one go. @docs/benchmarks/workflows put it here don't try to run it just yet.
Still waiting on massive benchmark to give results as to which model I want
to use for live testing.
```

---

### Prompt 03 — Exhaustive Execution Engine Feature Inventory

**Source:** `ses_17a245a9cffeWo9uVFAyuF6C9I` (2026-06-02)
**File:** `artifacts/dataset/prompt-03.txt`
**Word count:** ~78 (multi-requirement)
**Characteristics:** feature inventory, codebase analysis, multi-source synthesis

```
So I want you to look through the repository and example live system testing
workflows and tell me an exhaustive list of features we currently have working
and implemented in the execution engine. I'm wanting to build a workflow and
live system test and iterate on to iterate on it to do something relatively
complicated and I'm wondering what puzzle pieces we currently have to work
with. No changes just read and answer.
```

---

### Prompt 04 — Baseline Inspection for Workflow Reliability

**Source:** `ses_161125e9affeL2IOlPulAB1M1l` (2026-06-06)
**File:** `artifacts/dataset/prompt-04.txt`
**Word count:** ~165
**Characteristics:** multi-step inspection, file creation, documentation

```
Goal: I want this system to become reliable through real execution, logs, and
live workflow testing. Do not add new abstractions yet.

Please do the following:

1. Identify the project structure.
2. Identify the main workflow execution path.
3. Identify where YAML workflows are generated, loaded, parsed, executed, and logged.
4. Identify the current test commands, build commands, and run commands.
5. Run the existing tests/build if available.
6. Create or update a tracking file called WORKFLOW_RELIABILITY_TRACKING.md.

In that tracking file, include:
- Current project structure summary
- Main workflow-related files
- Current commands for build/test/run
- What currently passes
- What currently fails
- Unknowns that need investigation

Do not implement major changes yet.
Do not add new architecture.
Only inspect, run baseline checks, and document what is real right now.
```

---

### Prompt 05 — Full-Spectrum Data Pipeline with Quality Gates

**Source:** `docs/prompts/aggregate-test-prompts.md` (pre-existing, prompt A)
**File:** `artifacts/dataset/prompt-05.txt`
**Characteristics:** CSV ingestion, multi-phase pipeline, quality gates, report

See source file for full text. Multi-paragraph spec for a 4-phase data pipeline
(ingestion → cleaning → enrichment → reporting) with quality gates between phases.

---

### Prompt 06 — Configuration Drift Detector with Remediation

**Source:** `docs/prompts/aggregate-test-prompts.md` (pre-existing, prompt B)
**File:** `artifacts/dataset/prompt-06.txt`
**Characteristics:** 2-YAML diff, risk assessment, remediation planning

See source file. Multi-paragraph spec comparing baseline vs observed YAML
configurations and producing a remediation plan.

---

### Prompt 07 — Multi-Pass Code Quality Auditor with Trend Analysis

**Source:** `docs/prompts/aggregate-test-prompts.md` (pre-existing, prompt C)
**File:** `artifacts/dataset/prompt-07.txt`
**Characteristics:** 4-phase audit, trend tracking, multi-pass refinement

See source file. Multi-paragraph spec for a 4-phase code quality audit
(style → security → performance → trends) producing trend report.

---

## Dataset Storage Convention

```
artifacts/dataset/
├── prompt-01.txt   # Meta-workflow generator
├── prompt-02.txt   # Iterative benchmark workflow
├── prompt-03.txt   # Feature inventory
├── prompt-04.txt   # Baseline inspection
├── prompt-05.txt   # Data pipeline (CSV)
├── prompt-06.txt   # Config drift detector
├── prompt-07.txt   # Code quality auditor
└── README.md       # This list, one-line summaries
```

Each file contains the verbatim prompt with only spelling/grammar corrections.

---

## Usage

During SW iteration:

```bash
RUN_ID="sw1-iter-N-$(date -u +%Y%m%d-%H%M%S)"
./scripts/generate-workflow.sh \
    --template docs/benchmarks/workflows/sw1-task-deconstruction.yml \
    --run-id "$RUN_ID" \
    --prompt "$(cat docs/plans/meta-workflow-generator/artifacts/dataset/prompt-01.txt)" \
    --output docs/benchmarks/workflows/sw1-task-deconstruction.run.yml
```

The selected prompt becomes the `__TASK_PLACEHOLDER__` substituted into the YAML.

---

## Expansion

If 7 prompts prove insufficient for statistical confidence, expand by:
1. Reading more opencode sessions via `session_list --limit 30`
2. Filtering for messages containing "build", "implement", "iterate"
3. Saving qualifying prompts as `prompt-08.txt`, `prompt-09.txt`, etc.

Target ceiling: 12 prompts. Beyond that, marginal value drops.
