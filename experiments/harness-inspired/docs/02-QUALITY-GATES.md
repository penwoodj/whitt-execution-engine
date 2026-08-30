# Quality Gates: 5-Stage Pass/Fail Specifications

**Goal:** Define deterministic + LLM quality gates per layer with pass/fail criteria.

**Architecture:** Shell hooks for deterministic checks, LLM steps for semantic eval, GWT routing for retry loops.

**Tech Stack:** check-deterministic.py, fix-format.py, emit-prompt.py, GWT expressions.

---

## Gate 1: SCREEN (Pre-Flight Validation)

**Type:** Deterministic shell-only (no LLM)

**Trigger:** `before_step_starts` on SCREEN step

**Input:** Case YAML path

**Output:** `bookmarks.screen_valid` (bool)

**Pass/Fail Criteria:**
- PASS: YAML parses, `deterministic_checks` key exists, criteria non-empty
- FAIL: YAML invalid, missing keys, empty criteria

**Script:** `emit-prompt.py --case FILE --role worker --stage SCREEN`

**On PASS:** RouteTo GEN

**On FAIL:** SkipRemaining (skip entire case)

**Max Retries:** 0 (pre-flight, no retry)

---

## Gate 2: FORMAT CHECK (Deterministic)

**Type:** Deterministic shell-only (no LLM)

**Trigger:** `after_step_succeeds` on GEN or FIX steps

**Input:** Artifact path, criteria YAML

**Output:** Exit code 0=PASS, 1=FAIL. Bookmark: `{pass, checks, evidence, score, hash}`

**Pass/Fail Criteria:**
- PASS: All 12 check types satisfied
- FAIL: Any check fails

**Checks (12 types from check-deterministic.py):**
1. `contains_required`: Word/phrase present in output
2. `forbidden_phrases`: Prohibited text absent
3. `max_words`: Word count ≤ N
4. `min_words`: Word count ≥ N
5. `json_valid`: Parses as JSON
6. `yaml_valid`: Parses as YAML
7. `contains_regex`: Pattern matches
8. `exact_match`: Output equals golden text
9. `file_exists`: File path exists
10. `max_lines`: Line count ≤ N
11. `min_lines`: Line count ≥ N
12. Artifact exists: File path readable

**Script:** `check-deterministic.py --artifact FILE --criteria YAML --out result.json`

**On PASS:** RouteTo JUDGE (format OK, semantic eval next)

**On FAIL:** RouteTo FIX (format broken, fix first)

**Max Retries:** N/A (FIX step retries instead)

---

## Gate 3: FORMAT FIX (Deterministic)

**Type:** Deterministic shell-only (no LLM)

**Trigger:** On CHECK fail, routes to FIX step

**Input:** Artifact path, criteria YAML, failure evidence

**Output:** Fixed artifact file, bookmark: `retry_count`

**Pass/Fail Criteria:**
- PASS: Fix applied successfully, artifact modified
- FAIL: Fix script errors, artifact unchanged

**Fix Types (10 from fix-format.py):**
1. `strip_patterns`: Remove regex patterns
2. `replace_pairs`: Text replacements (old→new)
3. `strip_leading_whitespace`: Remove left indentation
4. `strip_trailing_whitespace`: Remove right whitespace
5. `deduplicate_blank_lines`: Collapse >2 blank lines
6. `ensure_trailing_newline`: Add final newline
7. `strip_markdown_fences`: Remove ``` fences
8. `strip_thinking_tags`: Remove `<think>` and `<reasoning>` tags
9. `extract_regex`: Extract first regex match
10. `force_json_object`/`force_yaml_document`: Enforce structure

**Script:** `fix-format.py --artifact FILE --criteria YAML --out fixed.txt`

**After Fix:** RouteTo CHECK (re-run Gate 2)

**Max Retries:** 3 total FIX attempts per case (tracked via `bookmark.retry_count`)

**On Exhaustion:** Fail workflow (max retries exceeded)

---

## Gate 4: SEMANTIC EVAL (LLM)

**Type:** LLM step with fresh context

**Trigger:** On CHECK pass, routes to EVALUATOR step (or JUDGE directly if combined)

**Input:** Artifact content, rubric/criteria, role framing

**Output:** `VERDICT: PASS` or `VERDICT: FAIL` + `REASON: ...`

**Pass/Fail Criteria:**
- PASS: Output meets functional requirements, correct logic, spec adherence
- FAIL: Logic error, missing requirements, incorrect implementation

**Role:** Evaluator (QA auditor, machine-checkable criteria first)

**Voice:** Precise, evidence-based, no hedging

**Output Format:**
```
VERDICT: PASS/FAIL
REASON: one sentence explanation
```

**Prompt Generation:** `emit-prompt.py --role evaluator --stage CHECK --case FILE`

**Fresh Context:** Only artifact + rubric, no history of GEN/CHECK/FIX

**On PASS:** RouteTo JUDGE

**On FAIL:** RouteTo FIX (semantic fix needed, re-run CHECK)

**Max Retries:** 3 total attempts (shared with FORMAT FIX retry count)

**Temperature:** 0.0 (deterministic, reproducible)

**Max Tokens:** ≥1024

---

## Gate 5: BLIND JUDGE (LLM)

**Type:** LLM step with fresh context

**Trigger:** After SEMANTIC EVAL pass, routes to JUDGE step

**Input:** Artifact content, objective/rubric, role framing

**Output:** `VERDICT: PASS` or `VERDICT: FAIL` + `REASON: ...` + quality score (1-10)

**Pass/Fail Criteria:**
- PASS: Quality ≥8/10, meets objective
- FAIL: Quality <8/10, misses objective

**Role:** Judge (blind quality reviewer, NO model identity info)

**Voice:** Objective, comparative, no self-preference

**Output Format:**
```
VERDICT: PASS/FAIL
REASON: specific what's missing or wrong
SCORE: 1-10
```

**Prompt Generation:** `emit-prompt.py --role judge --stage JUDGE --case FILE`

**Fresh Context:** Only artifact + objective, no identifying tokens

**Blinding:** Pre-processing removes model name, timestamps, metadata

**On PASS:** Workflow complete, log to trace

**On FAIL:** Workflow fails, log to trace

**Max Retries:** 0 (final verdict, no retry)

**Temperature:** 0.0 (reproducible blind evaluation)

**Max Tokens:** ≥1024

**Model:** Use larger model for judge (9B) vs worker (4B) to reduce self-preference

---

## Routing Flow Summary

```
START
  ↓
SCREEN (Gate 1)
  PASS → GEN → CHECK (Gate 2)
  FAIL → SkipRemaining

CHECK (Gate 2)
  PASS → JUDGE (Gate 5)
  FAIL → FIX (Gate 3)

FIX (Gate 3)
  Fixed → CHECK (re-run Gate 2)
  Exhausted → FAIL

JUDGE (Gate 5)
  PASS → DONE
  FAIL → DONE (fail logged)
```

**Retry Tracking:**
- `bookmark.retry_count` increments per FIX attempt
- GWT expression: `bookmarks.retry_count < 3` → allow FIX
- GWT expression: `bookmarks.retry_count >= 3` → fail workflow

**Two-Lane Correlation:**
- Deterministic PASS (Gate 2) + Semantic PASS (Gate 4) → JUDGE
- Deterministic FAIL → FIX (semantic not evaluated)
- Semantic FAIL → FIX (re-semantic after fix)

**Evidence Accumulation:**
- Gate 1: `bookmarks.screen_valid`
- Gate 2: `bookmarks.check_result = {pass, checks, evidence, score, hash}`
- Gate 3: `bookmarks.retry_count`, `bookmarks.fixed_hash`
- Gate 4: `bookmarks.semantic_verdict = {verdict, reason}`
- Gate 5: `bookmarks.final_verdict = {verdict, reason, score}`

All bookmarks merged to `final_report.json` at workflow end.