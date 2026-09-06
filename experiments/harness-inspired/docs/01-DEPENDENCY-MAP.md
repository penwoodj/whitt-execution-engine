# Dependency Map: 3-Level Subworkflow Structure

**Goal:** Show orchestrator → per-case subworkflow → per-step dependencies.

**Architecture:** L0 meta-workflow drives N cases × M model configs. L1 runs 5-stage cascade. L2 executes each step with hooks.

**Tech Stack:** GWT routing, shell hooks, bookmarks, template interpolation.

---

## L0: Orchestrator Level

```
┌─────────────────────────────────────────────────────────┐
│ META-WORKFLOW (Orchestrator)                             │
│ Input: N cases × M model configs = NM total runs         │
├─────────────────────────────────────────────────────────┤
│                                                           │
│  IterateValues over cases                                │
│       │                                                  │
│       ▼                                                  │
│  IterateValues over model configs                        │
│       │                                                  │
│       ▼                                                  │
│  RouteTo: per-case-subworkflow                           │
│       │                                                  │
│       ├── case_1_model_4b ──► subworkflow_1              │
│       ├── case_1_model_9b ──► subworkflow_2              │
│       ├── case_2_model_4b ──► subworkflow_3              │
│       └── case_N_model_M ──► subworkflow_NM             │
│                                                           │
└─────────────────────────────────────────────────────────┘
```

**Mechanism:** IterateValues action + RouteTo action + template variables `{{case}} {{model_config}}`.

---

## L1: Per-Case Subworkflow (5-Stage Cascade)

```
┌─────────────────────────────────────────────────────────────┐
│ SUBWORKFLOW: case_X_model_Y                                 │
│                                                             │
│  ┌──────┐     ┌─────┐     ┌──────┐     ┌──────┐     ┌─────┐│
│  │SCREEN│────►│ GEN │────►│CHECK │────►│ JUDGE│     │ END ││
│  └──────┘     └─────┘     └──────┘     └──────┘     └─────┘│
│  shell        LLM        shell        LLM               │
│  (pre-flight) (worker)   (format)     (blind)            │
│                             │                              │
│                             ▼                              │
│                        ┌────────┐                         │
│                        │   FIX  │ (max 3 retries)         │
│                        └────────┘                         │
│                          LLM                               │
│                        (fixer)                             │
└─────────────────────────────────────────────────────────────┘

GWT Routing:
- CHECK pass (exit 0) → JUDGE
- CHECK fail (exit 1) → FIX → re-CHECK (max 3 total)
- FIX count > 3 → FAIL workflow
```

**Hook firing points:**
- SCREEN: `before_step_starts` (emit-prompt.py shell)
- GEN: `before_step_starts` (emit-prompt.py shell) → LLM call → `after_step_succeeds` (save artifact)
- CHECK: `after_step_succeeds` (check-deterministic.py shell)
- FIX: `before_step_starts` (emit-prompt.py shell with retry count) → LLM call → `after_step_succeeds` (save fixed artifact)
- JUDGE: `before_step_starts` (emit-prompt.py shell) → LLM call → `after_step_succeeds` (log verdict)

---

## L2: Per-Step Details

### SCREEN (Shell-only, pre-flight)
```
┌─────────────────────────────────────┐
│ Input: case YAML                     │
│ Hook: before_step_starts            │
│   - shell: emit-prompt.py --stage SCREEN │
│   - shell: validate YAML structure   │
│   - gwt: check deterministic_checks parseable │
│ Output: bookmarks.screen_valid (bool)│
│ RouteTo: GEN if PASS, SkipRemaining if FAIL │
└─────────────────────────────────────┘
```

### GEN (LLM, worker role)
```
┌─────────────────────────────────────┐
│ Input: case.prompt, auxiliary       │
│ Hook: before_step_starts            │
│   - shell: emit-prompt.py --role worker --stage GEN │
│ LLM step: model=4B/9B, max_tokens≥1024 │
│ Hook: after_step_succeeds           │
│   - save_to: artifact_path          │
│   - shell: baseline.py hash --artifact artifact_path │
│   - bookmark: content_hash          │
│ Output: artifact file               │
│ RouteTo: CHECK                      │
└─────────────────────────────────────┘
```

### CHECK (Shell-only, deterministic)
```
┌─────────────────────────────────────┐
│ Input: artifact_path, criteria.yaml │
│ Hook: after_step_succeeds (from GEN)│
│   - shell: check-deterministic.py --artifact FILE --criteria YAML │
│   - bookmark: {pass, checks, evidence, score, hash} │
│ Output: exit 0=PASS, 1=FAIL         │
│ GWT Routing:                        │
│   - PASS (exit 0) → JUDGE           │
│   - FAIL (exit 1) → FIX             │
└─────────────────────────────────────┘
```

### FIX (LLM, fixer role)
```
┌─────────────────────────────────────┐
│ Input: artifact, failure evidence   │
│ Hook: before_step_starts            │
│   - shell: fix-format.py --artifact FILE --criteria YAML │
│   - shell: emit-prompt.py --role fixer --stage FIX --retry N │
│ LLM step: max_tokens≥1024           │
│ Hook: after_step_succeeds           │
│   - save_to: artifact_path (overwrite) │
│   - bookmark: retry_count           │
│ Output: fixed artifact file         │
│ RouteTo: CHECK (re-evaluate)        │
│ Limit: max 3 FIX attempts total     │
└─────────────────────────────────────┘
```

### JUDGE (LLM, judge role, blind)
```
┌─────────────────────────────────────┐
│ Input: artifact, rubric             │
│ Hook: before_step_starts            │
│   - shell: emit-prompt.py --role judge --stage JUDGE │
│   - shell: baseline.py hash --artifact artifact_path │
│ LLM step: model=9B (larger), max_tokens≥1024 │
│ Hook: after_step_succeeds           │
│   - save_to: verdict.md             │
│   - append_to: trace.jsonl          │
│   - bookmark: verdict (PASS/FAIL + reason) │
│ Output: VERDICT: PASS/FAIL          │
│ RouteTo: END                        │
└─────────────────────────────────────┘
```

---

## Bookmark Flow

```
SCREEN
  ↓ bookmarks.screen_valid
GEN
  ↓ bookmarks.content_hash (artifact SHA256)
CHECK
  ↓ bookmarks.check_result = {pass, checks, evidence}
  ↓ if FAIL: bookmarks.failure_evidence
FIX (if triggered)
  ↓ bookmarks.retry_count (increment per attempt)
  ↓ bookmarks.fixed_hash
JUDGE
  ↓ bookmarks.verdict = {verdict, reason, score}
  ↓ bookmarks.trace_entry (JSONL)
END
  ↓ final_report.json (merge all bookmarks)
```

---

## Variant Matrix Structure

```
Cases: N task specifications
  ├── case_01 (python script)
  ├── case_02 (json document)
  └── case_N (markdown report)

Model Configs: M configurations
  ├── cfg_01 (4B, temp=0.0, max_tokens=2048)
  ├── cfg_02 (9B, temp=0.0, max_tokens=4096)
  └── cfg_M (custom)

Total Runs: N × M = NM subworkflows
  Each runs SCREEN→GEN→CHECK→FIX→JUDGE cascade
  Each produces trace.jsonl entry
```

**Execution:** Sequential (no parallel agents per project rules). Each subworkflow cleans temp dir after completion.