# chunk-18 summary (prompts 0851-0900, 08-22 → 08-24)

## Harness research fan-out
- YAML-driven harness for small local LLMs (4B-9B) solving agentic [operator] tasks verifiably; worker AND judge both 4B; studying commercial agentic coding harness loops to steal patterns; non-mainstream patterns beyond ReAct/plan-execute (0859-0864)
- "Do additional web research on meta principles of agentic reasoning proving fruitful especially with cheap [models]" (0865)

## Deep results interrogation
- All layers of runtimes + IO for top-3 hardest cases (0866); per-case numbers (0867); inputs to cases 10/9/5/1 (0868); all failures + why each occurring (0869)
- Runtimes last 4 versions: total all-cases, per-case average, per-workflow (0872)
- Failure-cases report (0892); avg runtimes all levels last 4 versions (0893)

## Version engineering
- "Make a more efficient version than the v11 — .yml workflow driven with lots of [hooks/structure]" (0873)
- Spoofed-workflow improvements from findings (0875); "add all these improvements and tests as much as possible with NO LLM CALLS" (0876)
- Time estimates BEFORE running (0877, 0885, 0887)

## LLM-permission gating [signature pattern]
- "YOU CAN NOW RUN REAL LLM CALLS AND LOAD AND UNLOAD MODELS AND MAKE CALLS TO THEM. live system test and iterate" (0870)
- "NOW RUN LLM CALLS AND RUN EXPERIMENT AND LOAD AND UNLOAD MODELS AND ITERATE" (0878, 0889 "actually run")
- "Now build me 3 much more difficult and complicated test cases. DO NOT RUN LLM CALLS OR LOAD OR UNLOAD LLM MODELS" (0879)

## 100-case campaign (direct ancestor of current task)
- All categories + info on 20 current cases (0880)
- 1000-word version, 3 new test cases, NEW categories of agentic reasoning (0881)
- "Make the prompts in my normal prompting language" (0882); "what about making them much longer and more like my prompts?" (0883); "perfect, now show me 1 full test case" (0884)
- "Make 100 highly varied test cases like this one — widest variation of problems and wordings" (0886)
- 20 new more-complicated more-natural-language cases; live run until all 30 [pass] (0888)
- "You crashed my computer. Debug and find cause then fix BEFORE continuing" (0891)
- Avg word length per 100 cases (0894); longest + shortest input (0895)
- "Make all 100 like the 3 hand-authored ones, min 1k max 2k words. After that make [next version]" (0896-0897); "are all 100 test cases over 1k words?" (0898)
- Runtimes at all levels for all 100 + subsections (0899); total per workflow version (0900)
- Copyable prompt for harness-inspiration experiment: make 100 1k+ word inputs, run experiment after critically reviewing + fixing all test cases, then running and iterating (0901-0902)
