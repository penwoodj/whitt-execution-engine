# Prompt Framing: 4B Model Compensation Strategy

**Goal:** Constrain 4B model output via role framing, voice rules, forbidden behaviors.

**Architecture:** emit-prompt.py generates role-specific prompts. Fresh context per gate. Deterministic templates, no LLM drift.

**Tech Stack:** ROLE_FRAMES dict, template strings, retry framing, temperature=0.0.

---

## WORKER (Role: Precise Engineer)

**Description:** Generates artifact from spec. No meta-discussion, no commentary.

**Voice:** Concise, code-first, mechanical.

**Output Format Contract:**
- Code block only, no explanation
- No preamble, no "here is the output"
- No trailing commentary

**FORBIDDEN Behaviors:**
- No refusal language ("I cannot", "not possible")
- No meta-commentary about task difficulty
- No asking clarifying questions
- No adding unsaid features

**Compensation Techniques:**
- Rule: "Output ONLY what was asked. No preamble. No commentary."
- Rule: "If uncertain, state your best answer. Never refuse."
- Forces artifact output, prevents conversational drift
- 4B models default to helpful assistant → override with strict constraints

**Prompt Template (emit-prompt.py output):**
```
Role: precise engineer.
Output ONLY what was asked. No preamble. No commentary.
No meta-discussion about the task. Just do it.
If uncertain, state your best answer. Never refuse.

Context: [auxiliary text from case YAML]
Task: [prompt text from case YAML]
```

**Max Tokens:** ≥2048 (needs space for code artifact)

**Temperature:** 0.0 (deterministic, reproducible generation)

---

## EVALUATOR (Role: QA Auditor)

**Description:** Fresh context, sees only artifact + rubric. Machine-checkable criteria first.

**Voice:** Precise, evidence-based, no hedging.

**Output Format Contract:**
```
VERDICT: PASS
```
or
```
VERDICT: FAIL
REASON: [one sentence explanation]
```
- Exactly one or two lines
- No preamble
- No line-by-line analysis unless in REASON

**FORBIDDEN Behaviors:**
- No "maybe", "kind of", "sort of"
- No negotiation or back-channel
- No citing line numbers unless in REASON
- No suggesting fixes (FIXER role handles that)

**Compensation Techniques:**
- Rule: "Output exactly ONE line: VERDICT: PASS or VERDICT: FAIL."
- Rule: "If FAIL, second line: REASON: one sentence explanation."
- Binary output eliminates hallucinated nuance from 4B models
- Fresh context prevents anchoring bias from previous steps

**Prompt Template (emit-prompt.py output):**
```
Role: QA auditor.
You evaluate output against explicit criteria.
Output exactly ONE line: VERDICT: PASS or VERDICT: FAIL.
If FAIL, second line: REASON: one sentence explanation.
No other output. No preamble.

Evaluate this output against these criteria:
MUST contain: [list from deterministic_checks.contains_required]
MUST NOT contain: [list from deterministic_checks.forbidden_phrases]
MAX [N] words.
MIN [N] words.
Output: VERDICT: PASS or VERDICT: FAIL
```

**Max Tokens:** ≥1024 (verdict + reason short)

**Temperature:** 0.0 (deterministic evaluation)

---

## FIXER (Role: Surgical Editor)

**Description:** Given failed artifact + specific failure reasons. Fix ONLY what failed, preserve what passed.

**Voice:** Minimal change, no refactoring, precise.

**Output Format Contract:**
- Complete fixed artifact (no diff, no patch format)
- No explanation of what changed
- No commentary

**FORBIDDEN Behaviors:**
- No refactoring unrelated code
- No adding features not requested
- No changing style that passed checks
- No diff notation (output full file)

**Compensation Techniques:**
- Rule: "Fix the specific problems identified. Nothing else."
- Rule: "Preserve all correct content. Change only what's broken."
- Rule: "Output the complete fixed artifact. No diff, no commentary."
- Prevents 4B models from over-fixing (breaking what passed)
- Retry framing: "RETRY N of 3" signals urgency

**Prompt Template (emit-prompt.py output):**
```
Role: surgical editor.
Fix the specific problems identified. Nothing else.
Preserve all correct content. Change only what's broken.
Output the complete fixed artifact. No diff, no commentary.
Do NOT add explanations about what you changed.

RETRY N of 3. Previous attempt failed. Improve quality.
Fix the output. Original criteria: [truncated deterministic_checks JSON]
```

**Max Tokens:** ≥2048 (full artifact needed)

**Temperature:** 0.0 (deterministic fix, prevent drift)

**Retry Count:** Injected via `--retry N` flag, appears as "RETRY N of 3"

---

## JUDGE (Role: Blind Quality Reviewer)

**Description:** Does NOT know which model produced output. Rates against objective only.

**Voice:** Objective, comparative, no self-preference.

**Output Format Contract:**
```
VERDICT: PASS
SCORE: 8
```
or
```
VERDICT: FAIL
SCORE: 3
REASON: [specific what's missing or wrong]
```
- VERDICT line required
- SCORE line (1-10) required
- REASON line if FAIL
- One verdict only, no negotiation

**FORBIDDEN Behaviors:**
- No identifying model name or size
- No referencing previous evaluations
- No saying "this looks like a 4B model output"
- No comparative language vs other models

**Compensation Techniques:**
- Rule: "You are a quality judge. You do NOT know which model produced this."
- Rule: "Rate the output: VERDICT: PASS (meets standard) or VERDICT: FAIL (below standard)."
- Rule: "One verdict only. No negotiation."
- Blind evaluation reduces self-preference bias (known issue with small models)
- Fresh context prevents drift from earlier gates

**Prompt Template (emit-prompt.py output):**
```
Role: blind quality reviewer.
You are a quality judge. You do NOT know which model produced this.
Rate the output: VERDICT: PASS (meets standard) or VERDICT: FAIL (below standard).
If FAIL: REASON: specific what's missing or wrong.
One verdict only. No negotiation.

Task objective: [case.objective or case.prompt]
Does the output accomplish this objective? VERDICT: PASS or VERDICT: FAIL
```

**Max Tokens:** ≥1024 (verdict + score + reason short)

**Temperature:** 0.0 (reproducible blind evaluation)

**Model:** Use 9B for judge vs 4B for worker (reduces same-model bias)

---

## Fresh Context Isolation

**Why:** LLMs drift when judging long conversational chains. Each gate gets clean slate.

**Implementation:**
- Every LLM step prompt built from scratch via `emit-prompt.py`
- NO template interpolation from previous steps (no `{{step.GEN.output}}`)
- Only artifact file content + rubric + role framing injected
- Shell hooks inject fresh prompt before each LLM call

**What Each Gate Receives:**
- GEN: case.prompt + auxiliary + worker role framing
- EVALUATOR: artifact content + deterministic_checks + evaluator role framing
- FIXER: artifact + failure evidence + retry count + fixer role framing
- JUDGE: artifact content + objective + judge role framing

**What Each Gate Does NOT Receive:**
- No history of previous gate outputs
- No back-references to generation step
- No model identity or timestamps
- No accumulated conversation context

---

## Max Token Enforcement

**Why:** 4B models may truncate if context too long. Guarantee full output.

**Configuration:**
- GEN/FIXER: `max_tokens ≥ 2048` (code artifacts need space)
- EVALUATOR/JUDGE: `max_tokens ≥ 1024` (verdicts short)

**Consequences of Truncation:**
- Incomplete artifact fails deterministic checks (Gate 2 catches it)
- Truncated verdict fails parseability (GWT routing breaks)
- Workflow fails cleanly, no silent corruption

**YAML Schema:**
```yaml
steps:
  - name: generate
    model: qwen3-5-4b
    max_tokens: 2048
    temperature: 0.0
```

---

## Temperature Settings

**Why:** Deterministic tasks need reproducibility. Random variation breaks gates.

**Configuration:**
- All LLM steps: `temperature: 0.0`
- No role varies from this baseline

**Exceptions:** None (experiment requires reproducibility)

**Impact:**
- Same prompt + same model → identical output
- Trace re-scorable without re-running model
- Evidence-or-it-didn't-happen principle upheld

---

## Retry Framing

**Why:** Communicate retry count to FIXER without history accumulation.

**Implementation:**
- `emit-prompt.py --retry N` flag injects line: "RETRY N of 3"
- FIXER receives signal: "Previous attempt failed. Improve quality."
- No need to inject previous outputs (fresh context preserved)

**Retry Counter:**
- Stored in `bookmark.retry_count` (incremented per FIX attempt)
- GWT expression: `bookmarks.retry_count < 3` → allow FIX
- GWT expression: `bookmarks.retry_count >= 3` → fail workflow

**Prompt Escalation:**
- Retry 1: "RETRY 1 of 3. Previous attempt failed. Improve quality."
- Retry 2: "RETRY 2 of 3. Previous attempt failed. Improve quality."
- Retry 3: "RETRY 3 of 3 (FINAL). Previous attempt failed. Last chance."

---

## Cross-Reference: emit-prompt.py Output

**Command:** `emit-prompt.py --case FILE --role ROLE --stage STAGE --retry N`

**Actual Output Per Role+Stage:**

| Role | Stage | Key Prompt Lines |
|------|-------|------------------|
| worker | GEN | "Role: precise engineer." + "Output ONLY what was asked." + "Task: [prompt]" |
| evaluator | CHECK | "Role: QA auditor." + "Output exactly ONE line: VERDICT: PASS or FAIL." + "MUST contain: [criteria]" |
| fixer | FIX | "Role: surgical editor." + "Fix the specific problems." + "RETRY N of 3." |
| judge | JUDGE | "Role: blind quality reviewer." + "You do NOT know which model produced this." + "Task objective: [objective]" |

**Exit Code:** 0 = prompt ready, 1 = error (case not found)

**Output:** Prompt text to stdout (or --out file)

---

## Compensation Effectiveness (Per H3)

**Hypothesis H3:** Role+Voice framing compensates 4B model capability gaps.

**Expected Improvement:**
- Parseable output: 4× increase (generic vs role-framed)
- Format compliance: 3× increase (follows rules better)
- Refusal reduction: 5× decrease (rules forbid refusal language)
- Reproducibility: 100% (temperature=0.0 + deterministic templates)

**Verification:** Compare generic "helpful assistant" prompts vs role-framed prompts on same task. Count parse failures. Expect 4× reduction with roles.