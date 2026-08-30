# 13-synthesis — Research & Reasoning

## Why this atom exists
Real opencode prompts are multi-part ("look at X AND tell me Y"). A
replacement system must either solve monolithically or split-merge;
ParaManager proves a 4B CAN orchestrate — but ours is untrained, so the
orchestration must live in engine gates, not model judgment (RCA rule).

## Sources
- **S50 ParaManager (arXiv 2604.17009):** Qwen3-4B-Instruct-2507 (OUR
  fmt model) as master orchestrator w/ parallel subtask decomposition;
  frozen summarizer Ms does trajectory→answer synthesis; decoupling
  planning from solving "reduces long-context interference and limits
  error propagation". Validates both the split stage and the separate
  merge stage.
- **S53 Orchestrator-Worker catalogs (AgentPatterns.ai, agentpatternscatalog
  2026):** workers must not coordinate (design smell); synthesis must
  reconcile conflicts not concatenate; over-spawning + premature
  termination are the failure modes; effort-scaling rules (simple=1,
  moderate=2-4 workers). Our 2-worker fixed split = moderate tier.
- **Controlled MAS eval (via S53):** only 1/6 multi-agent workflows beat
  single-agent baseline → A-1 ablation is mandatory, not optional.
- **S52 Conductor (2512.04388):** workflows as (subtask, agent,
  access-list) triples — access-list = our PRIORS map exactly; 7B
  conductor beats individual frontier workers via topology design.
- **S51 Parallel-Synthesis (2606.14672):** text-concat synthesis matches
  KV-cache synthesis on 7/9 datasets — text merge is sufficient at our
  scale; no cache plumbing needed.
- **Rasal orchestrator (2402.16713):** decompose → parallel → compile
  pattern origin; weak eval but the shape stuck.
- **P5 decompose-separable-only (fusion, proven):** pairs chosen so
  parts share zero rules (quota+backoff etc.) — separability by
  construction.

## Reasoning chain
1. Two-rule-set prompts dilute 4B attention (E10 analog at task level).
2. Split stage (unchecked scaffold) names the parts; workers get
   part-scoped priors via PRIORS wiring (access-list).
3. Conflict band: extract_b fails det check → resolve (replan style)
   re-derives part B only — scoped recovery, not full re-solve.
4. merge assembles {part_a, part_b} — the contract's shape makes
   blind-merge of a WRONG part fail json_exact, so the check itself
   enforces H2.

## What would change our mind
- If A-1 (direct solve) matches H-lane on conflict-free cases AND costs
  less, restrict split-merge to ≥3 parts (where dilution actually bites).
- If resolve-stage recovery fails but direct re-solve works, drop
  resolve, wire extract_b-fail → solve_b2 directly (simpler ladder).

## Relation to opencode parity
Multi-part prompts are the modal opencode request. This atom + 06-plan
are what let a 4B system hold compound intent without flagship context.
