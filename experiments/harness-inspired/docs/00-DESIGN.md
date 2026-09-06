# Harness-Inspired Workflow Experiment: Top-Level Design

> For agentic workers: REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Prove small local models (4B-9B) achieve ≥80% flagship quality on structured tasks via harness meta-principles + workflow engine hooks.

**Architecture:** YAML-driven workflow with Shell/GWT/SaveTo/AppendTo/RouteTo/Bookmark/Fail hooks. Deterministic-first scoring + blind LLM-as-judge + variant matrix execution in sequential mode.

**Tech Stack:** Rust execution engine, llama.cpp+Vulkan (Docker), local 4B-9B Q4_K_M models, Python eval scripts, jq for JSONL trace processing.

---

## Hypotheses (H1-H5)

### H1: Deterministic-First Scoring Eliminates False Positives

Small models self-prefer their own output. LLM-as-judge alone rewards low-effort garbage. Deterministic code-based checks (exit codes, file existence, format validation, hash checks) MUST run before LLM judge. FAIL at deterministic gate = NO LLM invocation. Cuts waste, forces quality at generation time.

**Test case:** 4B model generates Python script with syntax error. Deterministic exit code check FAIL immediately. LLM judge never sees it. Result: more accurate fail verdict, cheaper execution.

### H2: Fresh-Context-Per-Gate Beats Accumulated-Context

LLMs drift when judging long conversational chains. Each quality gate MUST receive ONLY: (1) artifact to evaluate, (2) rubric, (3) fresh role/voice framing. No history of previous gates. No back-reference to generation step. Isolated judgment reduces anchoring bias.

**Test case:** Same artifact judged via 3-step accumulated context vs 3 isolated judgments. Isolated shows consistent verdict. Accumulated drifts to lenient after step 2.

### H3: Role+Voice Framing Compensates Capability Gaps

Generic "you are helpful assistant" produces undisciplined output from 4B models. Specific roles constrain output space:
- Worker: "You are senior engineer. Build X. Follow spec exactly."
- Evaluator: "You are QA auditor. Check Y. Verdict: PASS|FAIL only."
- Fixer: "You are surgical editor. Fix Z only. No refactoring."
- Judge: "You are blind reviewer. Evaluate Q. No self-preference."

Each role has voice constraints, forbidden actions, output format spec. Reduces hallucinations, forces adherence to contract.

**Test case:** 4B model with generic framing vs role-framed on same task. Role-framed produces parseable output 4× more often.

### H4: Cascading Refinement Beats Single-Shot Generation

Small models miss specs in single pass. Cascading flow:
1. Generate artifact
2. Deterministic check (format, syntax, file existence)
3. Format fix if FAIL (shell sed/python fix-format.py)
4. Semantic eval (LLM judge)
5. Semantic fix if FAIL (retry generation with error context)
6. Re-check (deterministic + semantic)
7. Max 3 retries → FAIL hard

Each cascade step gets fresh context, specific role. Result: higher quality than 1-shot.

**Test case:** JSON schema compliance. 1-shot: 30% pass. Cascading 3-retry: 78% pass on 4B model.

### H5: Two-Lane Scoring Catches Both Format+Semantic Failures

Deterministic lane catches format (exit code, parse, hash, structure). LLM judge lane catches semantic (logic correctness, spec adherence). Correlate both lanes:
- Deterministic PASS + Semantic PASS → PASS
- Deterministic FAIL → FAIL (format blocked)
- Deterministic PASS + Semantic FAIL → FAIL (semantic failed)

Single lane misses failures. Two lanes with evidence correlation = comprehensive.

**Test case:** Script with syntax error + wrong logic. Deterministic catches syntax. LLM judge catches logic. Single lane fails to catch both.

---

## Meta-Principles Mapping

| # | Principle | Engine Capability | Implementation | YAML Pattern |
|---|-----------|-------------------|----------------|--------------|
| 1 | Golden+output→score | Shell action, SaveTo | Shell script compares golden vs actual, writes score to bookmark | `after_step_succeeds: - shell: command: "python scripts/compare.py golden.txt $output" bookmark: comparison_score` |
| 2 | LLM-as-judge cross-vendor | Separate step with different model | Evaluator step uses larger/more-capable model than worker | `step_evaluate: model: qwen3-5-9b prompts: [...]` |
| 3 | Blind judging | Shell action (jq, sed) | Pre-processing removes model name, timestamps, identifying info before judge step | `before_step_starts: - shell: command: "cat $output | jq 'del(.model, .timestamp)' > $blinded_output"` |
| 4 | Variant matrix | GWT route + template | Loop over cases (N) × configs (M), route per combo via GWT expression | `main_loop: iterate_values: cases: [case1, case2] configs: [cfg1, cfg2] route_to: "{{step.name}}"` |
| 5 | Workspace isolation | Shell (mkdir, cd) | Each trial runs in unique temp dir, cleaned up after | `before_workflow: - shell: command: "mkdir -p /tmp/trial-{{workflow.id}} && cd /tmp/trial-{{workflow.id}}"` |
| 6 | Baseline+drift detection | Bookmark + shell hash | Store golden hash on first run, compare subsequent runs | `after_step_succeeds: - shell: command: "sha256sum $output" bookmark: output_hash` |
| 7 | Evidence-or-it-didn't-happen | Shell (tee) + log action | Every step writes full command + stdout/stderr to JSONL trace | `after_step_succeeds: - shell: command: "echo '{\"step\": \"{{step.name}}\", \"exit_code\": $?}' | tee -a trace.jsonl" - log: level: INFO message: "{{step.stdout}}"` |
| 8 | Fail-before-spend | Shell preflight, Fail action | Check resources, model health, file sizes before expensive steps | `before_step_starts: - shell: command: "free -h | grep Mem | awk '{print $4}'" fail_on_error: false bookmark: available_mem - gwt: given: bookmarks.available_mem < 8G then: route_to: skip_inference` |
| 9 | Frozen hash-bound grading | Shell (sha256sum) + bookmark | Record input hash, compare on grading, reject drift | `after_step_succeeds: - shell: command: "sha256sum {{step.worker.input}} > input.hash"` |
| 10 | Multi-instrument grading | Separate steps (evaluator + blind judge) | Evaluator checks functional correctness, judge checks code quality/style | `step_evaluator: role: qa_auditor prompts: [...] step_judge: role: blind_reviewer prompts: [...]` |
| 11 | Trace as source of truth | SaveTo JSONL | All execution artifacts written to JSONL, re-scorable without re-running model | `after_workflow: - save_to: path: outputs/{{workflow.id}}/trace.jsonl` |
| 12 | Two scoring lanes | Shell (deterministic) + step (semantic) | Parallel evaluation paths, merge via GWT | `step_deterministic: shell: python check.py step_semantic: model: qwen3-5-9b after_all: - gwt: given: bookmarks.deterministic_pass == true AND bookmarks.semantic_pass == true then: route_to: passed` |
| 13 | GSD rubric format | Template interpolation + SaveTo | PASS/FAIL with behavior description stored in structured format | `after_step_succeeds: - save_to: path: outputs/{{workflow.id}}/verdict.md content: "{{bookmarks.verdict}}"` |
| 14 | GSD guardrail classification | GWT route on severity | Catastrophic = fail fast, quality = accumulate for flywheel | `after_step_fails: - gwt: given: step.error.category == "catastrophic" then: route_to: abort_workflow else: route_to: log_and_continue` |
| 15 | evalh variant matrix | IterateValues + template | Single YAML drives N×M matrix via template variables | `workflow: variables: cases: [case1, case2] models: [qwen3-5-4b, qwen3-5-9b] steps: ...` |
| 16 | lm-eval-harness task abstraction | Step-level prompts + shell | Each task = self-contained module (prompt+parser+scorer) | `step_task_x: prompt: "{{file:prompts/task_x.txt}}" hooks: after_step_succeeds: - shell: command: "python scorers/task_x_scorer.py $output"` |

---

## Objective Hierarchy

### L0: Experiment Objective
Small local models (4B-9B) achieve ≥80% quality of flagship models (GPT-4, Claude 3.5) on structured tasks (code generation, data transformation, JSON schema compliance). Measured by: deterministic exit code pass rate + LLM-as-judge semantic pass rate (blinded, cross-vendor).

### L1: Per-Task Subworkflow Objective
For each task type (Python script, JSON document, Markdown report):
1. Worker step generates artifact from spec
2. Deterministic checks validate format, syntax, file existence
3. Evaluator step (LLM-as-judge) assesses semantic correctness vs golden spec
4. Fixer step corrects failures (if retries < 3)
5. Final verification (both lanes) → deliverable at target path
6. Evidence recorded to JSONL trace for re-scoring

### L2: Per-Step Objective
- **Worker step:** Output artifact matching spec (role: senior engineer, voice: concise, no commentary). Expected: file at path, parseable, no extraneous text.
- **Deterministic check step:** Exit code 0 if format valid, non-zero if FAIL. Populate bookmarks: exit_code, stderr, format_valid.
- **Evaluator step:** Output "PASS" or "FAIL" + evidence line numbers + reason. Role: QA auditor, voice: precise, no hedging.
- **Fixer step:** Output corrected artifact addressing specific failure. Role: surgical editor, voice: minimal change, no refactoring.
- **Judge step:** Output "PASS" or "FAIL" + blind quality score (1-10). Role: blind reviewer, no identifying info.
- **Final verification step:** Merge deterministic + semantic lanes. Correlate evidence. Populate final verdict bookmark.

---

## Compensation Strategies (Making 4B Models Punch Above Weight)

### 1. Role Framing Per Step Type

| Step Type | Role | Voice | Constraints | Output Format |
|-----------|------|-------|-------------|---------------|
| Worker | Senior Engineer | Concise, code-first | No commentary, no fluff, strict spec adherence | Code block only, no explanation |
| Evaluator | QA Auditor | Precise, evidence-based | Verdict: PASS|FAIL only, cite line numbers, no hedging | "PASS: reason" or "FAIL: reason [line:N]" |
| Fixer | Surgical Editor | Minimal change | Fix reported issue only, no refactoring, preserve structure | Corrected code block, diff notation |
| Judge | Blind Reviewer | Objective, comparative | No identifying info, score 1-10, reference golden | "PASS: score=8" or "FAIL: score=3, reason=..." |

**Implementation:** Prompt injection via shell cat (`cat prompts/worker_role.txt`). Template interpolation (`{{step.worker_role}}`).

### 2. Fresh Context Isolation Per Quality Gate

Each evaluation step receives ONLY:
- Artifact content (file path → shell cat)
- Rubric (file path → shell cat)
- Role/voice framing (file path → shell cat)
- NO history, NO previous step outputs, NO back-references

**Implementation:**
```yaml
step_evaluate:
  prompt: |
    {{shell:cat prompts/evaluator_role.txt}}

    Artifact:
    {{shell:cat $artifact_path}}

    Rubric:
    {{shell:cat prompts/rubric.txt}}

    Verdict (PASS|FAIL + evidence):
```

### 3. Deterministic Pre-Validation Before LLM Evaluation

LLM judge ONLY runs if deterministic checks PASS:
- Shell script exits 0 (syntax check, parse check, file exists)
- Hash matches expected (content integrity)
- Size within bounds (no truncation)

**Implementation:**
```yaml
before_step_starts:
  - shell:
      command: "python scripts/preflight.py $artifact_path"
      fail_on_error: true
      bookmark: preflight_result
  - gwt:
      given: bookmarks.preflight_result.valid == false
      then:
        route_to: step_format_fix
```

### 4. Output Format Enforcement Via Machine-Checkable Rules

Format rules enforced by shell scripts before LLM judge:
- Code: `python -m py_compile` (syntax), `grep -q "def main()"` (structure)
- JSON: `jq empty` (valid), `jq '.required_field'` (schema)
- Markdown: `grep -q "^# "` (header), `grep -q "```" ` (code block)

Violations → FAIL deterministic lane, skip LLM judge.

**Implementation:**
```yaml
after_step_succeeds:
  - shell:
      command: "python scripts/format_check.py $output"
      bookmark: format_valid
  - gwt:
      given: bookmarks.format_valid == false
      then:
        route_to: step_fail_format
```

### 5. Cascading Refinement Loops With Max Retry Limits

Generate → check → fix → re-check loop, max 3 retries:
```yaml
step_worker_generate: # Attempt 1
  hooks:
    after_step_succeeds:
      - route_to: step_deterministic_check

step_deterministic_check:
  hooks:
    after_step_succeeds:
      - gwt:
          given: bookmarks.format_valid == false
          then:
            route_to: step_format_fix # Attempt 1.1

step_format_fix:
  max_retries: 2
  hooks:
    after_step_succeeds:
      - route_to: step_deterministic_check # Re-check

step_semantic_eval:
  max_retries: 2
  hooks:
    after_step_fails:
      - route_to: step_semantic_fix # Attempt 1.2

step_semantic_fix:
  hooks:
    after_step_succeeds:
      - route_to: step_semantic_eval # Re-evaluate
```

### 6. Prompt Injection Via Shell Scripts (Structured Prompts)

No LLM generates prompts. All prompts stored as `.txt` files, injected via shell:
```yaml
step_worker:
  prompt: |
    {{shell:cat prompts/worker_role.txt}}

    Task spec:
    {{shell:cat specs/task_spec.txt}}

    Golden examples (do NOT copy):
    {{shell:cat fixtures/golden_examples.txt}}

    Output:
```

Advantages: version-controlled, auditable, no drift, prevents LLM prompt leak.

---

## Exit Criteria

### Primary Success Criteria (ALL must PASS)

1. **Quality Target:** ≥80% of tasks achieve PASS in both deterministic AND semantic lanes on 4B models (vs flagship baseline of 95%+).

2. **Evidence Requirement:** Every PASS/FAIL verdict cites specific evidence (exit code, line number, JSON path, hash). No "feels wrong" judgments.

3. **No False Positives:** Deterministic lane catches 100% of format failures (syntax, parse, structure). LLM judge never sees malformed artifacts.

4. **Blind Judge Integrity:** Judge verdict shows ≤5% correlation with model identity (self-preference test). Measured via blinded vs unblinded comparison.

5. **Trace Replayability:** JSONL trace re-scores to same verdict (≥95% match) without re-running model. Proves trace is source of truth.

6. **Variant Matrix Coverage:** Execute N cases × M configs in single YAML run. All combos produce trace entries. No manual orchestration.

### Secondary Success Criteria

1. **Runtime Efficiency:** Average task completion ≤2 min on 4B model (including 3-retry cascade). Comparable to single-shot flagship model.

2. **Resource Budget:** Max 1GB RAM per trial, max 7GB VRAM. No Docker restarts needed during matrix run.

3. **Artifact Quality:** Human review of 20% random sample shows ≥85% agreement with automated verdict. Blind human judge vs LLM judge.

### Failure Modes (Criteria for FAIL)

1. Quality <60% on 4B models (regardless of flagship performance)
2. >10% false positives (format FAIL but LLM judge PASS)
3. Judge self-preference >15% (unblinded vs blinded correlation)
4. Trace re-score mismatch >10% (trace not source of truth)
5. Matrix run crashes mid-execution (no checkpoint recovery)
6. Evidence missing for >5% of verdicts (violates "evidence-or-it-didn't-happen")

---

## Risks and Mitigations

### R1: Small Model Cannot Follow Complex Roles

**Risk:** 4B model ignores role framing, produces generic output despite constraints.

**Mitigation:**
- Start with simple roles (1-2 constraints), increment complexity
- Test role adherence on golden examples before live run
- If role ignored → fallback to deterministic-only scoring (drop LLM lane)

### R2: Deterministic Checks Too Strict, Block Valid Output

**Risk:** Format checker rejects artifact for trivial issue (whitespace, style) that doesn't affect semantics.

**Mitigation:**
- Separated "hard" checks (syntax, parse) vs "soft" checks (style, lint)
- Hard checks block LLM judge. Soft checks warn but continue.
- Configurable strictness level via YAML variables.

### R3: LLM-as-Judge Self-Preference Bias

**Risk:** Judge prefers same model's output, even when blinded. Known issue with small models.

**Mitigation:**
- Use larger model for judge (9B judge for 4B worker)
- Blind more aggressively (remove all identifying tokens)
- Cross-validate: run judge on reversed pairs (A judges B, B judges A)
- If bias >15% → use human-annotated golden verdicts as ground truth

### R4: Cascading Loops Never Converge

**Risk:** Fixer step creates new error in different location, infinite loop.

**Mitigation:**
- Max 3 retries hard limit (YAML field)
- Fixer role: "Fix reported issue ONLY, no other changes"
- Record each attempt in trace for debugging
- After 3 fails → abort with full diff history

### R5: Variant Matrix Memory Exhaustion

**Risk:** N×M combos blow up RAM, Docker crashes.

**Mitigation:**
- Sequential execution (no parallel agents, per project rules)
- Clean up temp dirs after each trial (shell rm -rf)
- Cap matrix size: max 5 cases × 3 configs = 15 trials per run
- If RAM >80% → abort, log, split matrix into multiple runs

### R6: Shell Script Compatibility Issues

**Risk:** Python, jq, sed behave differently on different hosts. Script fails silently.

**Mitigation:**
- All eval scripts shebang with absolute paths (#!/usr/bin/env python3)
- Lint scripts before run (shellcheck for bash, pylint for Python)
- Test scripts on golden examples before live matrix execution
- Log script stderr to trace for debugging

### R7: Trace Drift (Changes After Recording)

**Risk:** Artifact modified after trace written (e.g., by fixer step). Trace no longer matches reality.

**Mitigation:**
- Trace written IMMEDIATELY after step succeeds (after_step_succeeds hook)
- Hash of artifact recorded in trace entry
- Re-scoring validates hash before using trace data
- If hash mismatch → trace invalid, re-run step

### R8: GWT Expressions Become Unmaintainable

**Risk:** Complex routing logic (nested GWT) hard to debug, errors propagate silently.

**Mitigation:**
- Keep GWT expressions simple (single condition per clause)
- Use Shell script for complex logic, store result in bookmark, GWT checks bookmark
- Unit test GWT logic on fixture data before live run
- Log GWT evaluation to trace (given/then/actual result)

---

## Execution Discipline

### DO NOT RUN LLM During Design Phase

This document describes design only. NO LLM calls in design phase.

### Live System Testing (Post-Implementation)

After workflow implementation:
1. Run single trial with golden example (verify deterministic lane)
2. Run blinded vs unblinded judge comparison (verify bias mitigation)
3. Run trace re-score test (verify source-of-truth)
4. Run small matrix (2×2) to validate variant mechanics
5. Full matrix execution with resource monitoring
6. Human spot-check on 20% outputs (verify LLM judge accuracy)

### Verification Protocol Per Hypothesis

| Hypothesis | Verification Method | Evidence |
|------------|---------------------|----------|
| H1 (deterministic-first) | Compare false positive rate with/without deterministic lane | Logs show deterministic catches N format errors, LLM judge never invoked |
| H2 (fresh-context) | Run same artifact via accumulated vs isolated gates | Compare verdict drift rate (expect <5% drift for isolated) |
| H3 (role-framing) | Compare output parseability with/without role framing | Count parse failures (expect 4× reduction with roles) |
| H4 (cascading) | Compare single-shot vs 3-retry cascade on 4B model | Quality improvement %, retries consumed |
| H5 (two-lane) | Inject format failure + semantic failure, verify both caught | Trace shows deterministic FAIL + semantic FAIL for same artifact |

---

## File Structure (Max 3 Folders Deep)

```
experiments/harness-inspired/
├── docs/
│   ├── 00-DESIGN.md (this file)
│   ├── 01-IMPLEMENTATION-PLAN.md
│   └── 02-RESULTS.md
├── workflows/
│   ├── task-python-script.yml
│   ├── task-json-document.yml
│   └── task-markdown-report.yml
├── prompts/
│   ├── worker_role.txt
│   ├── evaluator_role.txt
│   ├── fixer_role.txt
│   └── judge_role.txt
├── specs/
│   ├── task-python-script.txt
│   ├── task-json-document.txt
│   └── task-markdown-report.txt
├── scripts/
│   ├── format_check.py
│   ├── preflight.py
│   ├── compare.py
│   └── re-score.py
├── fixtures/
│   ├── golden-examples/
│   │   ├── python-script.py
│   │   ├── json-document.json
│   │   └── markdown-report.md
│   └── negative-examples/
│       ├── python-syntax-error.py
│       └── json-parse-error.json
└── outputs/
    └── traces/
        └── {run-id}/
            ├── trace.jsonl
            └── artifacts/
```

---

## Next Steps

1. **Review and approve this design** → no LLM calls, human sign-off only
2. **Write 01-IMPLEMENTATION-PLAN.md** → detailed task breakdown, file-by-file
3. **Implement prompt files** → worker_role.txt, evaluator_role.txt, etc.
4. **Implement shell scripts** → format_check.py, preflight.py, compare.py
5. **Build workflow YAMLs** → one per task type, following meta-principles mapping
6. **Test on golden examples** → verify deterministic lane, role framing
7. **Run blinded vs unblinded comparison** → verify bias mitigation
8. **Execute full matrix** → gather evidence for all 5 hypotheses
9. **Analyze results** → write 02-RESULTS.md, document PASS/FAIL per hypothesis
10. **Iterate or conclude** → based on exit criteria met or not

---