# Agentic Reasoning Phase — Objectives, Methodology, Rules

Status: ACTIVE (spoof-proven, awaiting GPU for live A/B).
Suite: `cases/agentic/` — 20 cases (5 heavy multi-system, 15 light
single-system), truths in `fixtures/agentic-fixes.yml` (20/20
self-verified against check_lib).

## 1. Objectives

O1. Determine which solving TECHNIQUE wins on agentic exactness
    tasks: plan-then-solve (decompose), dual-attempt harvest (dual),
    or hint-carrying repair (repair). Live A/B on identical cases,
    identical models, identical budgets where shape permits.
O2. Produce per-technique quality (pass rate, win-depth
    distribution) + cost (stages executed, wall) from engine-native
    runs only — no driver, no out-of-YAML LLM calls.
O3. Extract failure taxonomy (which check class kills which stage)
    to drive the NEXT workflow version; taxonomy labels feed the
    routing decisions of future iterations.
O4. Keep the zero-LLM spoof loop as the permanent regression gate:
    no workflow or hook-script change ships without a green spoof
    battery.

## 2. Methodology

M1. Spoof-first: every structural change lands with spoof runs
    (scenario-driven outcomes, zero model loads) proving gates,
    GWT routing, chain walks, and artifact writing.
M2. Seeded-live second: live-mode YAMLs run with pre-passed checks
    to prove the real gate path (stage-emit + check-answer shells)
    end-to-end with zero inference.
M3. Live A/B last: same YAMLs, real models, one flag difference
    (--no-spoof at generation). Techniques compared on the SAME
    case set; win-depth = index of first passing checked stage.
M4. n=1 per technique initially; a second seed run only for the
    winner (variance estimate) before declaring a result.
M5. Fingerprint audit: every stage's actual prompt input is
    fingerprinted (sha256 of case+stage+prompt) to fingerprint.jsonl
    — any failed case can be replayed/triaged exactly (rea-live-pulse
    pattern).

## 3. Pass vs fail analysis — rules

R1. A case PASSES iff any checked stage produced
    check-{cid}-{stage}.json with passed=true. Winning stage =
    lowest-index checked stage that passed (order: attempt1/solve,
    attempt2/repair1, repair2).
R2. Unchecked stages (plan, select) can never win a case; their
    outputs are context only.
R3. Win-depth distribution {0,1,2} is the primary quality signal:
    depth 0 = first solver suffices (cheap), depth 2 = full chain
    burned (expensive). Technique A beats B at equal pass rate iff
    it wins at lower mean depth.
R4. Failure taxonomy: every failing check classifies as FORMAT
    (json_exact/line_count/all_caps/bullets), CONTENT
    (contains_required/numbers_must_sum_to), or LEAK
    (forbidden_phrases). A stage failing on FORMAT after a CONTENT
    pass elsewhere indicates extraction-style loss (answer existed,
    phrasing killed it) — repair-friendly. CONTENT failures at all
    stages indicate reasoning miss — escalate model, not repairs.
R5. Spoof runs NEVER count toward quality numbers; they assert
    structure only (20/20 expected by construction).
R6. Ties in live A/B (same pass rate, same mean depth) are broken
    by wall time, then by fewest stages executed.
R7. Truth edits invalidate prior runs: any change to
    agentic-fixes.yml requires re-running the affected technique
    before comparison (truths are the scoreboard).

## 4. Workflow logging rules (mandatory for every version)

L1. Every step logs via the after_step_succeeds log hook with
    event_fields [step_name, model_name, duration_ms, token_count]
    to {run_dir}/progress.log — engine-native, no side channels.
L2. Gate scripts (stage-emit/spoof-emit) MUST append one line per
    invocation to {run_dir}/outcomes.jsonl:
    {"case": cid, "stage": stage, "outcome": "PASS"|"SKIP"|"RUN",
     "ts": iso8601} — deterministic outcome ledger independent of
    engine log format.
L3. Gate scripts MUST append prompt fingerprints to
    {run_dir}/fingerprint.jsonl: {"case", "stage", "sha256",
    "tokens_approx"}.
L4. Routing tokens are QUOTED ("PASS"/"SKIP") — bare tokens fail
    GWT string equality and silently fall through to inference.
    (Load-bearing contract; regression-tested.)
L5. Terminal steps use before_step_starts skip_step — never a real
    inference call.
L6. Check artifacts carry the failing check names (failures[].check)
    — collector depends on them; never write a bare boolean.
L7. Runs are resumable by construction: gate short-circuits on any
    existing passing check for the case (skip-if-won).
L8. No gate script may load, call, or probe a model — spoof purity
    is the regression contract (test_hooks enforces via subprocess
    contract checks).

## 5. Current state

- 6 workflows (3 techniques x spoof/live), all validator PASS.
- Spoof battery: 20/20 x3, 0 loads. Seeded live battery: all routes
  fired, 0 loads. Hook-path tests: 9/9.
- Engine fixes this phase: router-zombie false positive (TDD),
  100-step cap (prior phase), terminal-step skip pattern.
- Pending: live A/B when GPU frees; winner gets second seed run.
