# Source: 2025–2026 Self-Healing Agent Landscape (multi-source)

- **Retrieved:** 2026-08-24 (websearch sweep)
- **Primary paper under test:** 2605.06737 (see doc 01)

## Adjacent 2026 Work

| Paper | Core idea | Relevance here |
|-------|-----------|----------------|
| Self-Healing Agentic Orchestrators (2606.01416) | orchestrator-level heal, workflow rewrites | closest peer — we implement heal INSIDE workflow YAML via hooks instead of external orchestrator |
| Doctor-RAG (2604.00865) | failure-aware repair for agentic RAG | repair-loop pattern; RAG-specific detectors |
| Graph-Based Self-Healing Tool Routing (2603.01548) | reroute tool calls on failure via graph | F2 tool re-selection lineage |
| AgentTether (2607.06273) | graph-guided runtime intervention | intervention points = our hook triggers |
| SHIELD (2601.19174) | auto-heal resource exhaustion | infra-level heal; deferred (engine concern) |
| VIGIL (2512.07094) | reflective runtime for self-healing agents | reflection loop = Reflexion lineage |
| AgentFail dataset | agent failure benchmark | case-suite methodology analogue |
| DoctorAgent-RL (2505.19630) | RL-trained clinical multi-agent | domain app, not method source |

## Position Statement

This experiment = primary paper (2605.06737) framework implemented on whitt execution engine with:
- detection = deterministic first-line suite (Guardrails doc 11) + spoofed LLM-judge signals
- reliability = R = ω₁C + ω₂S + ω₃E, θ=0.65
- healing = class-routed (F1 corrective prompt / F2 tool reselect / F3+F4 replan), bounded 3 attempts (docs 02–06)
- evaluation = TSR/FDA/RSR with injected-failure ground truth

Differentiator vs 2606.01416: heal logic lives in workflow YAML hooks + GWT expressions (engine-native, auditable, deterministic), not external orchestrator code.

## Not Cited As Evidence

Any secondary blog numbers (e.g., "+27% TSR") — unverified, excluded from hypothesis baselines.
