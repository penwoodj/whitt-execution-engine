# chunk-11 summary (prompts 0501-0550, 08-08)

## Whitt ecosystem + local agentic coding research
- UI-implications reports for whitt-* repos (agent-queue-engine, execution-engine, hardware, model-router; umbrella /home/jon/code/whitt) (0501-0504, 0509-0511)
- Best agent coding UI for LOCAL models, agentic processes optimized for small models; fully offline (no internet/cloud); small models 7B-13B Q4/Q5; internet-cut filesystem work (0505-0508)
- Hardware frame: 16GB DDR4 + 8GB VRAM on CachyOS; deep details vibe-local + 1bcoder (0516-0518)
- LexLocker top-level docs critical review: skimmability, completeness, Hormozi-framework alignment (0519)
- ONE self-contained modern HTML at hormozi/index.html presenting 7 markdown docs (0520-0521)
- Consolidate ui-implications subfolders into one folder, drop prefix, then stop (0522)

## vision-graph-ui project (whitt UI) begins
- New branch graph-ui-v1 off initial-creation; build + hello-world React (0524)
- Install 1bcoder (0525); list all supported models (0526)
- Storybook in subfolder w/ hello-world component; "don't start the server just yet, set it up so I can run it" (0527-0528, 0538)
- `npx storybook ai setup` follow instructions precisely (0537)
- AGENT FILE: "full visibility into storybook", an agent file where you ALWAYS follow the rules (0529) → install/create agent SKILLS for storybook, modern [react]... (0530)
- Create PROJECT-LEVEL opencode skills: storybook, modern-react, neo4j, 4× graph-graphics libraries (0531-0534)
- Reuse llama.cpp ("I already have llama.cpp so let's use that") (0536)
- Model picker from ANY model in data-drive models folder (0544); serve-coder.sh 563-model menu runs (0545-0546)
- aider with llama.cpp → pivot: "actually let's do ollama with 1bcoder" (0547-0548)
- Agent-config enforcement: styled-components + library must [be used] in subproject config (0550-0551)
- Pause + checkpoint questions [before] compaction so checkpoint continues task with full context (0539)
- AGENTS.md Rev 2 strict rules (caveman...) as project context (0540)

## Steering/meta
- "it is 3.5 not 3-5" [exact model naming] (0558 boundary); status for each?; give-me-a-command-only-if-needed; pause/checkpoint/resume cycles; "C" [pick option]; yes
