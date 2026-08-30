# chunk-13 summary (prompts 0601-0650, 08-09 → 08-11)

## Model hunt on constrained hardware
- Expansive search of DOWNLOADED models for ones that fit + perform: ≤6GB, 8GB VRAM AMD RX 580 (no ROCm, Vulkan ~5 tps), 16GB RAM (0618-0619, 0625-0629)
- Replacement for Qwen3.5-9B-coder; NEW coding LLMs 2025-2026 fitting 8GB VRAM via Ollama (0628-0629)
- Handoff doc in ~/code/working-1bcoder-chat-harness; iteratively try Qwen3-4B-Ins[truct]; handoff gap-fill questions; "do you have everything you need to continue live system testing iterations with the new models + comparing" (0630-0633)
- Execute iteratively, don't stop (0634); live system test various objectives with the new 4b model, keep iterating until working (0635)
- Coggle.it open-source clones hunt (0621-0622)
- ADR-0011 spec context (0623)

## Edu-report generation (React/lodash curriculum)
- .md reports for intermediate React dev building [todo app]: React hooks; lodash/fp pure FP; point-free flow(); ES6 destructured imports; per-component scoped SCSS w/ concrete patterns (0637-0641)
- Todo app render verify via screenshot (/tmp/todo-02-added.png): title, input [visible] (0642-0643)
- "Was this built fully by the local model with 1bcoder? if not why" (0644) — attribution audit
- "Use 9b and 4b" (0645)

## Brainstorm/research mode (memory strategies)
- "Let's brainstorm, no changes; web research; report findings with citations + links" (0646-0648)
- Core reason each strategy works + tradeoffs + run speeds at large vs low context (0649)
- Apply strategies to my yml framework; generate example yml schemas for the new way of specifying [them] (0650)
- Ring strategy with OrangePi 3W's 12GB DDR5? (0651 boundary → chunk 14)

## Meta-workflow reasoning
- Where does Qwen3.5-9B struggle most in meta workflow generator (0652)
- "We are brainstorming/searching/reading — need more high-level reasoning in less [time]" (0653)
- 3 memory-management strategies + models loaded with minimal context [combined] (0654)
- How meta workflow generator works at ALL levels — simple scoped tree summaries (0655)
- Talk through my thoughts while I read [your output] (0656)
- Yaml experiment: show me just ymls; refine + brainstorm in chat; an ATOM for iterative reasoning/thinking of how to answer [— not an atom version of the meta workflow generator] (0657-0658)
- Exit brainstorm → experimental mode: make a new top [level experiment] (0659)
- "You crashed my machine. Put safeguards in place in the repo so this doesn't happen, then continue the experiments" (0660)
