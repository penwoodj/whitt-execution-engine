# Experiment Matrix — Hypotheses, Criteria, Ablations

Pre-registered per S48: acceptance criteria fixed BEFORE any live run.
Common metrics everywhere: pass rate (det-checked), tokens (fingerprint
ledger), win-depth, failure taxa (FORMAT/LEAK/CONTENT), route stability.
"Parity" = pass rate within 10pp of a `07-solve` control running the same
cases with 3× token budget (opencode-flagship analog: max scaffolding,
max budget), at ≤40% of its tokens. Rationale: fusion + S1 showed small-model
scaffolds match, not beat, unlimited-budget single agents on narrow tasks;
efficiency is our win condition.

Per-experiment ablation always includes: atom-REMOVED variant (proves the atom
carries the gain, per CCI finding that subsets beat all-in).

---

## 01-typed-envelope (S41, S37)
- **H1:** typed schema envelope cuts FORMAT-taxon failures ≥80% vs prose contract.
- **H2:** typed + error-class diagnostics cut CONTENT re-submits ≥30% vs typed alone.
- **Cases:** 36 = 3 conditions × 12 engine variants (reuse fusion engines).
  Condition A prose contract, B json_exact only, C typed + error-class echo.
- **Truths:** same as fusion engines (computed).
- **Ablation:** A vs B vs C paired; slice = FORMAT failure count.
- **Pass:** H1 and H2 on spoof (structural) + live smoke ≥15 cases each condition.

## 02-digest (E10, S44 O.i)
- **H1:** digest-solve pass rate ≥ full-prompt-solve − 2pp (information preserved).
- **H2:** digest tokens ≤ 30% of full prompt.
- **Cases:** 36 = 6 engines × 2 variants × 3 fact-positions (early/mid/late —
  Lost-in-Middle control). Same case solved under both presentations.
- **Truths:** engine-computed; digest oracle asserts all task digits present.
- **Ablation:** full vs digest vs digest-without-contract (proves contract
  re-attachment is the load-bearing slice).
- **Pass:** H1 within band on live; H2 is structural (spoof-verifiable now).

## 03-cache-addressing (S48, Nix/Bazel pattern)
- **H1:** stage-level content-addressed skip removes ≥60% of re-inference on
  case pairs sharing sub-solutions, with ZERO wrong-reuse verdicts.
- **Cases:** 40 = 20 pairs (shared-shape task + parameter-shifted rerun) + 10
  collision traps (same prefix, different required outcome — must NOT skip).
- **Truths:** engine-computed; collision traps have deliberately divergent truths.
- **Ablation:** no-cache vs cache; slice = tokens + wrong-reuse count.
- **Pass:** H1 spoof-demonstrated (skip machinery fires on matches, not on traps);
  live confirms token delta.

## 04-intent-classify (S46, S47, S49)
- **H1:** entropy/GWT pre-gate lane accuracy ≥85% vs case's true lane
  (lane = which chain first passes checks).
- **H2:** route stability ≥95% across 3 spoof seeds.
- **Cases:** 48 = 4 task classes (lookup / arithmetic / multi-dep / evidence)
  × 2 difficulties × 6 seeds. Plus 6 DECLINE cases (insufficient info — correct
  route is neither lane but an explicit unresolvable verdict).
- **Truths:** lane label computed by running rule-solver through both chains
  offline; DECLINE truths engineered (missing fact).
- **Ablation:** always-LIGHT vs always-HEAVY vs gate (S47 no-free-lunch check).
- **Pass:** H1+H2 spoof; live confirms gate matches observed win-depth.

## 05-gather (S45, S43, agentic-RAG ablation)
- **H1:** gate-driven 2-iteration gather ≥ 1-iter by ≥10pp on multi-fact cases.
- **H2:** 2-iter within 2pp of 5-iter (iteration depth saturates).
- **H3:** control cases (facts absent from corpus) → atom must emit
  NOT_FOUND verdict, not fabricate (S43 Output-Fabrication slice).
- **Cases:** 40 = 20 corpus-backed (2-4 facts scattered across 6-10 invented
  system files) + 10 partial (some facts present) + 10 control (absent).
  Corpora generated under `cases-shared/corpora/` with SeedRG-style invented
  entities + N8 digit discipline.
- **Truths:** facts placed by generator → computed from placement map.
- **Ablation:** iter depth {1,2,5}; evidence-buffer digest vs raw pile.
- **Pass:** H1-H3 live; spoof proves iteration gating + buffer mechanics.

## 06-plan (S48, PlanCompiler)
- **H1:** plan-then-solve ≥ direct-solve + 15pp on multi-dependency cases.
- **H2:** plan stage tokens ≤ 25% of total (cheap scaffold).
- **Cases:** 36 = 12 dependency graphs × 3 sizes (3/5/7 nodes). Truth = topologically-
  valid execution values (engine walks the DAG).
- **Ablation:** plan vs no-plan (07 control); invalid-plan-injection (plan with
  cycle → engine must reject, not execute).
- **Pass:** H1 live; spoof proves plan parsing + DAG validation gates.

## 07-solve (control, S1, S44)
- **Purpose:** baseline lane + guess-resistance calibration.
- **H1:** narrow-scope CoT solve ≥ 70% on LIGHT variants (sanity floor).
- **H2:** guess baseline ≤ 5% on all engines (answer space sized).
- **Cases:** 36 = fusion engines reused (6 × 2 × 3 seeds).
- **Ablation:** none — this IS the control arm for 02/06/08/09.
- **Pass:** H1+H2 live. Guess baseline computed analytically + spoof-simulated.

## 08-replan (S39, TDP, Leni)
- **H1:** scoped replan after check-fail recovers ≥50% of near-miss cases.
- **H2:** recovery token cost ≤ 40% of full re-solve (scoped, digest-only).
- **Cases:** 40 = 20 near-miss (initial shape solvable but one rule violated →
  first answer wrong by exactly one field) + 20 hard-fail (first answer wrong
  broadly). Hazards named per S39 taxonomy (spec drift = misread rule,
  cross-source conflict = worked example contradicts core).
- **Truths:** engine-computed; "wrong-by-one-field" first-attempt artifacts
  spoof-generated to feed the replan stage deterministically.
- **Ablation:** retry-same vs replan-scoped vs full-re-solve.
- **Pass:** H1 live; H2 tokens from ledger; spoof proves replan triggers only
  on check-fail and scopes to failed rule.

## 09-sample-diverse (S1)
- **H1:** cross-model second sample converts ≥30% of same-model-stuck cases.
- **H2:** same-model N=2 converts < 10% of those (diversity, not sampling, is
  the active ingredient — S1 replication).
- **Cases:** 36 near-miss set (drawn from 08 near-miss pool + new).
- **Ablation:** same-model ×2 vs cross-model ×2 at matched token cost.
- **Pass:** H1 live, H2 live; spoof proves the escalate gate fires on
  stuck-detection (two consecutive failed checks).

## 10-execute-observe (S38, S39, S40, S43)
- **H1:** execute→observe→classify→recover chain resolves ≥60% of injected
  tool faults within budget {4,8} calls.
- **H2:** delta-perturbed observations (S40) caught by det re-verify ≥80%
  (silent-failure guard).
- **H3:** no-tool control cases → atom skips execution (S43 Unnecessary-Use).
- **Cases:** 60 = 5 fault classes (timeout, unreachable, garble, delta, schema
  drift) × 6 engine tasks × 2 budgets + 12 controls. Faults injected via
  scenario config (spoof emits faulty observations deterministically).
- **Truths:** task answer unchanged by fault (S39 recoverability); recovery
  path exists by construction.
- **Ablation:** no-recovery vs blind-retry vs classify-then-recover.
- **Pass:** H1-H3 live; spoof proves fault injection + classification + budget
  enforcement.

## 11-verify-blind (Leni, Rulers, fusion N5)
- **H1:** blind verifier lane flags ≥50% of planted-answer errors that pass
  det checks (det-miss class: plausible but rule-violating).
- **H2:** judge disagreement on correct answers ≤10% (precision guard).
- **Cases:** 40 = 20 planted-error answers (wrong by one rule application —
  engineered to pass json_exact shape but fail semantics... det check is the
  FULL truth compare, so det-miss class = cases where partial credit masks;
  constructed as near-truth variants) + 20 clean.
  NOTE: with exact-truth checks, planted errors that fail json_exact are
  caught by det lane; the blind-verifier value slice = errors in REASONING
  artifacts (plan/summary/observations), not final answers. Cases therefore
  target intermediate artifacts.
- **Ablation:** det-only vs det+blind; blind-sees-answer vs blind-sees-digest.
- **Pass:** H1 live; H2 live; spoof proves blind lane never gates final verdict.

## 12-shape-library (Agent Primitives pool, Nexus)
- **H1:** shape-reused workflow (hash-matched template + parameter re-bind)
  pass rate ≥ from-scratch − 5pp.
- **H2:** generation cost ≤ 30% of from-scratch (skip SW1-SW4 equivalent).
- **Cases:** 36 = 12 shapes × 3 parameterizations each. Shape = fusion engine
  skeleton; parameterization = different events/entities. 6 "shape-trap" pairs
  (similar surface, different engine — must NOT match).
- **Truths:** engine-computed per parameterization.
- **Ablation:** from-scratch vs reuse; trap-matching accuracy tracked.
- **Pass:** H1 live; spoof proves hash-match hits correct shape, misses traps.

---

## Cross-cutting register

- Spoof scenario discipline (fusion): win-index per case; UNCHECKED scaffolds;
  judge blind lane; ledgers (outcome/fingerprint).
- All case prompts 1000-3000 words, generator-asserted.
- All truths computed by rule simulation, asserted against own checks.
- Workflows: v10-identical schema shape; validate 9/9 before anything else.
- No LLM in this phase. Live variants generated alongside but not run.
