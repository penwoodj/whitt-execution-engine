# Self-Healing Experiment — Hypotheses

**Version:** v1 | **Blueprint:** arxiv 2605.06737 | **Research base:** docs/web-research/

## H1 — Deterministic detection-first pipeline is engine-expressible

**Claim:** The paper's detect→score→classify→heal→re-execute loop can be fully expressed in whitt workflow YAML (hooks + GWT + shell actions) with NO engine code changes, using unrolled attempt chains instead of loops.

**Test:** 6 generated workflows pass structural validation and execute end-to-end via `whitt benchmark`, routing exactly per classification (trace.jsonl path assertion).

**Phase:** S (spoofed). Falsified if: any required decision needs engine modification, or routing diverges from expected path on any case.

## H2 — Class-routed healing beats blind retry on injected failures

**Claim:** Routing heal strategy by failure class (F1→corrective prompt, F2→tool reselect, F3/F4→replan) recovers more injected failures within 3 attempts than a naive same-prompt retry chain.

**Test (Phase S):** case design — persistent cases (f-persist) fail because scripted heal doesn't match class; matched cases recover at attempt 2. Phase L: A/B workflows (routed vs blind-retry) on same case suite, compare RSR.

**Baseline:** harness v9 fix-chain (blind escalation w/ temperature ladder) = the blind-retry baseline reusing existing infrastructure.

## H3 — R-score threshold gates healing correctly at θ=0.65

**Claim:** R = 0.35·C + 0.35·S + 0.30·E with θ=0.65 separates clean outputs from all 4 failure classes with zero false accepts/rejects on the spoofed suite.

**Test:** clean case → R ≥ 0.65 → accept without heal; every injected failure case → R < 0.65 → heal entered. report.json logs R per attempt for calibration review.

**Known risk (doc 10):** "confident errors" (high C hallucinations) — mitigated by S-component hallucination markers carrying F1 detection even when C high.

## H4 — Bounded 3-attempt budget suffices for recoverable failures

**Claim:** Per docs 02–05 convergence data (Reflexion 3–5, Self-Refine k=3–4, CRITIC n=3), recoverable spoofed failures all resolve by attempt 2–3; only non-recoverable (persist) cases consume full budget then fail cleanly.

**Test:** attempts-to-success distribution in report.json: f1/f2/f3/f4-single = 2 attempts; f4-multi = 3 attempts; f-persist = 3 attempts + fail.

## H5 — Zero-LLM phase proves orchestration correctness transferable to live phase

**Claim:** Because all decision logic (detectors, scoring, routing, budget) is deterministic and model-independent, spoof-phase green implies live-phase failures can ONLY originate from model output quality — a strictly smaller debugging surface.

**Test:** Phase S green → Phase L swap = replace agent_run.py spoof with real inference step, keep detect/classify/heal/report identical. (Deferred until user-gated.)

## Hypothesis → Metric Map

| Hypothesis | Metric / evidence |
|-----------|-------------------|
| H1 | validator 0-exit ×6; trace path match ×6; zero engine diffs |
| H2 | RSR routed (spoof: 4/5) vs blind baseline; class-routing correctness |
| H3 | FDA = 1.0 (no false accept/reject); per-attempt R values logged |
| H4 | attempts-to-success histogram vs expected |
| H5 | Phase S exit criteria (00-SCOPE) all green |
