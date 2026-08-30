# chunk-17 summary (prompts 0801-0850, 08-18 → 08-22)

## Model sweep + version ladder
- Add models to test list: /run/media/jon/data/models/lmstudio-community/Bonsai-27B-...; Qwen3-4B-Instruct-2507 Q4, gemma-3n-E4B Q4, Qwen3-4B-Hivemind-Hrtic Q4, AesCoder-4B Q6, Bonsai-27B Q1_0, nvidia_Orchestrator... (0803, 0805)
- "Execute and do both versions in sequence" (0806)
- "Make 1-3 more versions trying to get all cases passing. Next version after that: make it effic[ient]" (0807)
- 3.2-vs-4 difference (0808); "make a 10-20 minute version just as effective at solving problems" (0809)
- "Are there any subworkflows in the most recent version?" (0810); "is the v6 written in a yml workflow?" (0811); "I want WORKFLOW NATIVE behavior. If you need logging add it during hooks" (0812)

## Thinking-budget + engine mechanics
- Tradeoffs moving thinking budget 3500→2000, examples where more/less [helps] (0814)
- Dynamic thinking budget 1000-5000 by input, how would that [work] (0815)
- "So what made the python version so much faster?" (0817); "what do we get for all those extra model loads?" (0818)
- Next experiment: ALL llm calls from the .yml workflow, all models [managed by yml] (0819)

## Harness research + harness-inspired experiment
- "Tell me about the get shit done harness and other harnesses for llms and how they work" ×2 (0820-0821)
- Design new experiment applying LLM-eval-harness meta-principles to YAML-driven workflow engine; GSD harness patterns; commercial agentic harness loops (context assembly, tool dispatch, verification) — steal patterns (0823-0826)
- Write TOP-LEVEL experiment design doc (0827); then THREE design docs (0838)

## Test-case authoring campaign (key precedent)
- 20 new agentic reasoning test cases, ~paragraph long, modeled off [my prompts] (0834/0836); "don't run a workflow" (0835)
- "Make 3 new examples about twice as long and complex" (0839); "perfect, now make 17 more like those → all 20 as test cases for next few .yml workflow versions" (0840)
- "YOU CAN NOW RUN LLM CALLS continue and don't stop iterating until it's really working" ×2 (0841, 0843)
- "Continue iterating on the workflow until the full objective of the experiment is complete" (0844)
- "Are all 20 cases like the last 3 you showed me?" (0845); "make the ratio more like 5 and 15" [hard:easy] (0846)
- What are the 3 new versions trying different strategies doing? (0847)
- Build workflows for next reasoning-enhancer-plus .yml version ALSO build out... (0848-0849)
- Additional docs: next phase of agentic reasoning testing + objectives + e[xpectations] (0851)
- "Explain the hi str 01 case and why it fails" ×2 (0852-0853); compare reasoning-plus test cases vs problems [we tackle] (0854); "no — the 20 agentic test cases, 5 more complicated + 15 less but all agentic, inside reasoning-enhancer-plus" (0855-0856)
