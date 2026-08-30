# Self-Healing v2 — Workflow Design

DESIGN ONLY — no execution. All mechanics engine-proven (v1 Phase S + benchmark suites). Schema keys only; no new schema keys.

## 1. Design goals (user directive)

1. **Heavy model specialization** — 5 roles, script-routed per domain + failure class.
2. **Minimize LLM calls** — scripts/hooks do detection, routing, F1/F2 heals, reporting; LLM only where synthesis genuinely needed.
3. **Conditional LLM steps** — every LLM step gated by GWT flags; skip_step pre-inference.
4. **Loops per experiment evidence** — unrolled ×3 heal chain (v1-proven) + `iterate_values` sweeps (3/5/15/50-model benchmark-proven). `LoopConfig` validation/count loops = DEFERRED (no live evidence).

## 2. Model specialization map

| Role | Model | Invoked when |
|------|-------|--------------|
| `worker_general` | Qwen3-4B-Instruct-2507-Q4_K_M | default worker; research/analysis/legal/finance domains |
| `worker_coder` | Qwen2.5-Coder-3B-Instruct-Q8_0 | code/config artifacts (software-eng, scientific-compute transform-refine); auto-swap on F2 in code domains |
| `worker_fast` | Ministral-3-3B-Instruct-2512-Q4_K_M | complexity ≤2 generate/transform; F1 corrective retry (cheap second shot) |
| `heavy_replanner` | Qwen3-5-9B-Q4_K_M | F3/F4 replan heal; complexity ≥4 attempts |
| `judge` | Hermes-2-Pro-Mistral-7B.Q4_K_M | gray-zone acceptance only (θ ≤ R < θ+ε, or schema-clean but R marginal) |

Routing = `route_v2.py` (script): reads case `task.domain`/`task.complexity`/`task.archetype` → `routing.json`. Model swap mid-chain = next attempt's `generative_entity` differs per GWT-chosen branch (each attempt step variant hardwires its model — engine has no dynamic entity swap, so branch steps ARE the specialization).

## 3. Step graph (per case, sh-v2)

```
step_00_init        script  reset run_dir, write init event
step_01_route       script  domain→worker model, write routing.json
step_02_budget      script  call budget = f(complexity): cx≤2→{w:3,r:1,j:1}, cx≥3→{w:3,r:2,j:1}
step_03_attempt_1   LLM     worker per routing.json      [gate: always]
step_04_detect_1    script  deterministic detector suite
step_05_triage_1    script  R=ω·(C,S,E), class, flags {accept, need_replan, need_judge}
step_06_gate_1      script  writes gate flags; exit code = route decision
    GWT on gate exit:
      0 accept            → step_24_judge_gate
      1 heal-light        → step_07_heal_light_1      (F1/F2, script)
      2 heal-replan       → step_08_heal_replan_1     (F3/F4, LLM heavy, gated)
      3 final-fail        → step_22_final_fail
step_07_heal_light_1    script  F1 corrective template / F2 tool-reselect map
step_08_heal_replan_1   LLM     heavy_replanner       [gate: budget.r > 0 else degrade→heal_light]
step_09_attempt_2   LLM     worker (F1: worker_fast; F2 code domain: worker_coder; else same)
step_10_detect_2    script
step_11_triage_2    script
step_12_gate_2      script  same routing table
step_13_heal_light_2    script
step_14_heal_replan_2   LLM     [gate: budget.r > 0]
step_15_attempt_3   LLM     worker (escalate: complexity≥4 → heavy_replanner as worker)
step_16_detect_3    script
step_17_triage_3    script
step_18_gate_3      script  accept → judge_gate; else → final_fail (no round 4)
step_22_final_fail  script  outcome final_fail
step_24_judge_gate  script  computes need_judge; exit: 0=no-judge→accept_fast, 1=judge
step_25_judge       LLM     Hermes judge           [gate: budget.j > 0 AND need_judge]
step_26_accept      script  outcome accept (+judge verdict if used)
step_28_report      script  oracle: verdict, path, CALL BUDGET CHECK, gate flags check
step_29_end_pass    script  end_event pass   (report exit 0)
step_30_end_fail    script  end_event fail
step_31_end         script  terminal
```

Numbering strictly increasing in chain order (v1 iteration-4 lesson: engine iterates sorted-by-name; bad targets fall through silently). gen-v2 asserts all GWT targets exist.

## 4. Conditional LLM steps (gates)

Every LLM step carries `before_step_starts` GWT reading gate flags written by the preceding script step into `bookmarks.shell_output`:

- `attempt_N` (N≥2): reached only via GWT branch — structurally conditional.
- `heal_replan_N`: GWT `exit_code == 2` guaranteed-match route from gate; PLUS budget check inside script fallback (budget.r exhausted → script writes degraded heal, LLM step skipped via its own gate `budget_ok == 0 → skip_step`).
- `judge`: GWT on `judge_gate` exit: `0 → accept` (skip judge), `1 → judge`.
- Budget governor: `budget_v2.py` ledger appends each LLM step entry to `trace.jsonl`; gate scripts read remaining budget; ceiling 6 calls/case.

Gray zone definition: accept-flag true BUT (θ ≤ R < θ+ε) OR (schema clean ∧ confidence signal < 0.5). ε=0.05.

## 5. Loop strategy (evidence-based)

| Loop need | Mechanism | Evidence |
|-----------|-----------|----------|
| Heal-retry ×3 | Unrolled explicit steps + GWT routing | v1 6/6 green (iterations 3–5) |
| 100-case sweep | Driver bash loop over per-case workflows (rea+ safe-launch pattern) | rea+ 07-TRACKING + safe-launch.sh |
| Model sweep / calibration | `iterate_values` on `step.model_ref` | benchmark-3/5/15/50-models.yml live-proven |
| Schema-native validation loop | `loop.validation.exact_criteria` | 🔵 DEFERRED — schema+structs exist, zero live runs. First Phase S2 experiment will A/B unrolled vs validation-loop on 10 flagship cases. |

## 6. Failure → strategy → model table (v2)

| Class | Detection (deterministic) | Heal | Heal executor | Next-attempt worker |
|-------|--------------------------|------|---------------|---------------------|
| clean | all detectors pass, R≥θ+ε | none | — | — |
| F1 hallucination | schema-valid but hallucination_keys>0 or unsupported claims | corrective prompt template citing detector findings | script | worker_fast (cheap retry) |
| F2 tool/schema misuse | schema_missing, tool_errors>0, format violations | tool reselect map + format contract re-injection | script | worker_coder if code domain else same |
| F3 internal contradiction | contradiction markers, cross-field numeric conflicts | REPLAN: restate constraints, decompose, re-derive | LLM heavy_replanner | same worker w/ plan |
| F4 truncation/corruption | JSON parse fail, truncation markers, upstream_errors | REPLAN: shorten output plan, chunked emission, verify-then-emit | LLM heavy_replanner | complexity≥4→heavy as worker |
| persist (any) | failures at every attempt | none (budget exhausted) | — | final_fail |

## 7. Case schema v2 (extends v1)

```yaml
case_id: flg-01
version: 2
task:
  prompt: |        # 1000–1500 words
  expected_schema: {status: string, ...}   # top-level keys contract
  domain: software-eng        # 10 values
  archetype: generate-artifact # 5 values
  complexity: 4               # 1–5
  output_format: json         # json | markdown | hybrid
failure:
  class: clean                # clean|f1|f2|f3|f4
  fail_attempts: []           # attempts that fail (spoof schedule)
  signatures: {…}             # class-specific detector hints
expected:
  final: accept
  path: [attempt, detect, triage, gate, accept, report]
  attempts_to_success: 1
  llm_calls: {worker: 1, replan: 0, judge: 0}   # budget oracle
  gate_flags: {need_replan: false, need_judge: false}
budget:
  llm_call_ceiling: 6
```

## 8. Script kit v2 (all deterministic, no LLM)

| Script | Role |
|--------|------|
| `route_v2.py` | case meta → routing.json (worker model, budget) |
| `budget_v2.py` | call ledger init/charge; gate reads |
| `agent_run_v2.py` | spoof worker: domain-flavored output + class/attempt failure injection |
| `detect_v2.py` | domain-aware detector suite → detection.json |
| `triage_v2.py` | R scoring + class + gate flags → triage.json (exit = route code) |
| `heal_light_v2.py` | F1/F2 scripted heals → heal_N.json |
| `heal_replan_v2.py` | F3/F4 LLM step wrapper: Phase S = spoof replan text; Phase L = heavy model prompt |
| `judge_v2.py` | judge step wrapper: Phase S = spoof verdict; Phase L = Hermes |
| `outcome_v2.py` | accept/final_fail outcomes |
| `report_v2.py` | oracle: verdict+path+class seq vs expected, LLM call budget check, gate flag check, suite TSR/FDA/RSR + call-economy metric |
| `end_event_v2.py` | reset/init/pass/fail trace events |
| `gen-cases-v2.py` | compose 90 matrix cases (content library, no LLM) |
| `gen-v2.py` | per-case workflow YAML generator (this design) |

## 9. Dry-run walkthrough — 10 flagship cases vs design

| flg | class(fails) | Gate decisions expected | LLM calls | Verdict |
|-----|--------------|------------------------|-----------|---------|
| 01 | clean() | gate_1 exit 0 → judge_gate exit 0 (R≥θ+ε) → accept | w1 | ACC@1 |
| 02 | F1(@1) | gate_1 exit 1 → heal_light_1 (script) → attempt_2 worker_fast | w2 | ACC@2 |
| 03 | F2(@1) | gate_1 exit 1 → heal_light_1 (tool reselect) → attempt_2 worker_coder (code domain) | w2 | ACC@2 |
| 04 | F3(@1) | gate_1 exit 2 → heal_replan_1 LLM heavy → attempt_2 | w2+r1 | ACC@2 |
| 05 | F4(@1,@2) | gate_1 exit 2 → replan LLM → attempt_2 fails F4 → gate_2 exit 2 → replan_2 LLM → attempt_3 | w3+r2 | ACC@3 |
| 06 | F1p(@1,@2,@3) | heal-light ×2 → attempt_3 fails → gate_3 exit 3 → final_fail | w3 | FA@3 |
| 07 | F2(@1) | heal_light tool-reselect → attempt_2 coder | w2 | ACC@2 |
| 08 | F3(@1) | replan LLM → attempt_2 | w2+r1 | ACC@2 |
| 09 | F4(@1,@2) | replan ×2 → attempt_3 (cx≥4 → heavy worker) | w3+r2 | ACC@3 |
| 10 | F4p(@1,@2,@3) | replan, replan (budget.r=2 exhausts), attempt_3 fails → final_fail | w3+r2 | FA@3 |

Call totals flagship set: 23 worker + 6 replan + 0 judge = 29 (vs 30 naive ×3-attempt-always + no early exit). Full 100-suite ≈ ≤152 vs 300 naive.

## 10. Verification checklist for Phase S2 (next run session — user-gated)

1. `gen-cases-v2.py` emits 90 matrix cases; all 100 validate (case schema lint in gen script).
2. `gen-v2.py` ×100 → validate-workflow.py ×100 exit 0.
3. 10 flagship live zero-LLM: gate exit codes per walkthrough table §9; report_v2 oracle 10/10 incl. budget + gate-flag checks.
4. Full sweep 100 cases; suite TSR target 91/100 (88 accept + judge-gray still accept), FDA 100% of injected failures detected, RSR ≈ 79/91.
5. Zero-LLM gate grep = 0 events ×100.
6. A/B: unrolled vs `loop.validation` variant on 10 flagship (first live evidence for LoopConfig).
