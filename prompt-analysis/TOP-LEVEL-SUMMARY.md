# TOP-LEVEL SUMMARY — last 1000 opencode prompts (06-23-26 → 08-29-26)

Aggregate of chunk-01..20 summaries (each: prompt-analysis/summaries/chunk-NN-summary.md).
Method: script extraction + mechanical dedupe (981 unique of 1000) → agent summaries → this aggregate.
**Pass 1** = domain aggregation. **Pass 2** = conceptual dedupe (same ask in different words merged; constraints preserved, never dropped).

---

## PASS 1 — AGGREGATED, DEDUPLICATED ASKS BY DOMAIN

### 1. Whitt execution engine / meta-workflow generator (dominant domain, ~40% of volume)
- Verify audits against LIVE SYSTEM REALITY, never claims: "what remains incomplete / not live system tested", "read output workflow AND actual run results, detailed report", "tell me simply what is not done" (ch01, 03, 09, 19)
- Parity then SURPASS opencode reliability: gap analysis — hooks/tools opencode has that engine lacks; unimplemented unified-schema .yml tool/skill/script usage (ch02)
- Fix ALL inefficiencies; FULLY VERIFY each before marking off; context dumps across branches/worktrees (ch09, 10)
- Workflow+script-driven everything: "ALL execution behavior from YAML hooks", "all LLM calls from the .yml workflow", workflow-native logging (ch09, 17)
- Version ladder discipline: make 1-3 more versions to all-pass, THEN efficiency version ("same result faster, more smaller steps, more specialized stages"); 10-20-minute version equally effective (ch15, 16, 17)
- Efficiency questions that recur: runtimes per version/case/subworkflow/section; per-prompt avg + variance; why is X faster; what do extra model loads buy; thinking budget 3500→2000 + dynamic 1000-5000 tradeoffs (ch16, 17, 18)
- Test-case campaigns: 50-case all-areas benchmark; 30 small-model-struggle inputs; 20 paragraph-long agentic cases modeled off own prompts (5 hard / 15 easier); 100 extra-hard in weak areas; 100 cases 1k-2k words "like the 3 hand-authored ones"; verify stats (all >1k? avg word length? longest/shortest?) (ch15, 17, 18, 19, 20)
- FULL-AGENTIC-SYSTEM genesis: "real goal = subworkflows usable like ATOMS to build a working meta-workflow generator"; stitched top-level control flow; run all experiments sequentially to 100% (ch19) — completed at 566/566 + v3-fast 13x speedup (ch20/current)
- Model infra: Qwen3.5-9B Q4 + q8_0 KV cache + thinking budget; CPU-first bring-up then GPU; constrained hardware (8GB VRAM RX 580 Vulkan ~5tps, 16GB RAM, ≤6GB models); model sweeps incl. weird names (Bonsai-27B Q1_0, gemma-3n-E4B, AesCoder-4B Q6, Hivemind, MiniCPM5-1B, Yi-6B-200K); load-settings recall from past comparisons (ch04, 07, 11, 12, 13, 16, 17)
- CRASH SAFETY: "you crashed my machine — undo it / debug cause, fix BEFORE continuing / put safeguards in repo / CPU-only for new models" (ch04, 13, 18)

### 2. Opencode platform engineering (the tool itself)
- Stop-stopping complaints → configure non-stop iteration until objective done; synchronous one-at-a-time, wait for shell (ch01) [→ became workspace rules]
- Context management: 32k default policy; context dumps for fresh workspaces; handoff docs; pause/checkpoint/resume; "THEN STOP. DO NOT CONTINUE EXECUTING JUST READ AND ANSWER"; clear todos/processes forcing continuation (ch01, 03, 11, 12, 19)
- "ALWAYS ASK WITH OPENCODE MULTIPLE CHOICE QUESTIONS" for decisions/open questions; 10-options-in-chat to pick from (ch10, 14, 19)
- Skills/plugins: pre-install security audit (opencode-timer-plugin); install real skills (D2, design, storybook, modern-react, neo4j, graph libs); skill registry hunt; global config integration after version upgrades; leverage subagent orchestration globally "safely in terms of token waste but effectively" (ch01, 09, 11, 20)
- Tool live-verification probes with EXACT reply formats ("Reply with exactly: OK", "FETCH_OK <title>", crawl4ai 2-page BFS, explore-subagent count, "2+2? one word") (ch20)
- Provider usage-number check (ch20)
- Prompt-analysis meta (seed of current task): analyze notable prompts of varying complexity, dedupe into types (ch20)

### 3. Hardware / SBC / infrastructure
- SBC exhaustive research (Orange Pi/Banana Pi/RK3588, DDR4/5, 8-64GB, price-per-RAM, AliExpress listings) → bought Orange Pi Zero 3W 12GB LPDDR5 A733 (ch02)
- OS bring-up: lightweight Linux for whitt on ARM; SD flashing (Pi Zero W v1.1 identification, Raspberry Pi Imager); configure-sd.sh iterations; fish shell; boot-loop log triage over mount/SSH; journald; fan/resolution/autologin/USB-hub/ethernet/bluetooth (ch02, 03, 04, 05, 07)
- Gaming: Xbox controller + PoE2 Proton crashes on CachyOS RX 580, "web search forums and try real fixes" (ch03)
- Desktop apps: Bambu Studio, keet.io AppImage, LM Studio update (keep configs) (ch04, 07, 08)

### 4. Media kiosk (PStream → Tuluflix Max+)
- Self-hosted streaming aggregator on OPi: NO API keys, NO accounts, NO torrents (seeding); Chromium kiosk + extension; anonymity escalation (fake→anonymous email, VPN IP-masking, fingerprint spoofing but NOT lying about real specs, PII tracking audit) (ch04, 05)
- Provider engineering: emoji-named sources, zstream.mov route reuse, Cloudflare workarounds, public REST APIs, curated WORKING sources over quantity, rebrand, onboarding skip, viewing-state persistence, save/cancel on all pages (ch05, 06, 07)

### 5. Left-Right language (Rust compiler)
- Operator×type matrix spec via iterative batch Q&A (Batches 5-13, surgical edits, TBD resolution, conflict flags); confirmed VM bug fixes (2, 3, then 6 bugs); deprecated ops → HARD ERRORS; new String operators; live .lr demo runs; P08 debug diagnostic (ch01, 02, 03)

### 6. Business knowledge system (Hormozi / LexLocker)
- Books: full text + images from web; framework-diagram-vs-photo classification batches; delete non-diagrams; ORM-as-DAG in D2 with brutal design iteration ("looks terrible", readable colors, ledgers, horizontal grouping, themes); anti-bypass enforcement ("BYPASS DETECTED — re-do") (ch08, 09)
- APPLICATION.md per book: EVERY framework applied to user's business; /tmp part-a/part-b splits; business-model shift to voice-ticket-driven fully-local AI propagated everywhere; one self-contained index.html presenting 7 docs; LexLocker doc critical reviews (skimmability, completeness, framework alignment) (ch10)
- Business writing: biweekly consulting update, casual-not-buzzwordy tone loop ("I don't say 'just wrapped'") (ch08)

### 7. vision-graph-ui / whitt IDE (React)
- graph-ui-v1 branch; Vite 8 + React 19 + TS + Storybook 10.5.7 + styled-components; AGENTS.md rule enforcement (styled-components, caveman) (ch11, 12)
- Design iterations: Monokai→VS Code Dark, white-bg bug, zoom-out start, circle→hover-expand nodes, ChatGPT-style layout, voice-capture flow, graph sims per project (ch12, 13)
- Requirements methodology: dictated-vision FAITHFULNESS reviews, cycle-2 after "Why" rationale pass; gherkin GWT exhaustive user-flow coverage + whys; fail-without/pass-with next-version test gates; 141 GWT cases; plan suites S01-S11 + E1-E4 written then executed §-by-§ with test-repair tasks; LOCKED user requirements never descoped (ch14, 15, 16, 20)
- Agentic graph UI brainstorm: bubbles of light, glow/breathing, force physics, infinite canvas, voice-first, local STT, Coggle.it prior art, glyphnova prior art (ch13, 14)

### 8. Research/brainstorm modes
- "Brainstorm, no changes, web research, citations + links" then exit to experimental mode (ch13, 14)
- [CONTEXT]/[search-mode]/[analyze-mode] delegation fan-outs throughout
- Edu-report series (React hooks, lodash/fp, point-free, ES6 imports, scoped SCSS) (ch13)
- AI-compute-hardware consolidated report from personal notes (ch01, 02)
- GSD + agentic harness loop research; self-healing (arxiv 2605.06737) spoofed experiment (ch17, 19)

---

## PASS 2 — CONCEPTUAL DEDUPE (deeper pass)

Merges of same-intent items across domains (variants preserved under one line):

1. **Verify-then-report** (one concept, many phrasings): "what remains incomplete / no changes just answer" + "read output AND live results → report in same location" + "tell me simply what is not done" + "are all 100 over 1k words?" + "anything remaining?" + "was this built fully by X? if not why" + attribution audits + "THEN STOP just read and answer". → *User repeatedly demands ground-truth answers separated from action, and fact-checks deliverable properties after generation.*
2. **Iterate-until-done gating**: "don't stop until X" + "continue and don't stop" + ralph loops + "keep iterating until it's working" + "execute iteratively" + "don't pause" + crash-recovery "apologies, continue". → *Persistence directive, with explicit permission gates for expensive operations ("YOU CAN (NOW) RUN LLM CALLS / DO NOT RUN LLM CALLS") and stop gates ("THEN STOP").*
3. **Efficiency-version pattern**: "efficiency version iterations, same thing faster, more smaller steps, more specialized" + "10-20-minute version equally effective" + "make a more efficient version than vN" + "validate runtime additions without quality loss" + "safely in terms of token waste". → *Speed is a first-class requirement equal to correctness; never trade quality for it, restructure instead.*
4. **Test-case authoring loop** (one meta-process, ~8 instances): generate N cases modeled on own prompts → tone rejection ("none in natural language / make them much longer and more like my prompts") → length calibration (paragraph → 1k-2k words) → mix ratios (5 hard/15 easier) → category variation ("widest variation of problems and wordings", new categories) → property verification (word counts, longest/shortest) → critically-review-fix before running. *(This current task = same loop at 100-case scale.)*
5. **Results interrogation ladder**: status → per-version runtimes → per-case/per-subsection stats → inputs to specific cases (10/9/5/1) → failure explanations → proven/disproven → lessons learned → "3 new experiment ideas from these insights" → scoping correction ("way beyond scope — again"). → *Systematic post-hoc analysis before next iteration.*
6. **Checkpoint/continuity**: pause+checkpoint, resume-from-checkpoint, handoff docs, context dumps, "give me a prompt to start fresh", crash recovery, "continue from handoff". → *Session-continuity machinery across compactions/crashes/machines.*
7. **Interactive decision protocol**: multiple-choice opencode questions (ALWAYS, well-thought-out), 10-options-in-chat pick-one, "C"/"yes"/single-letter steering. → *User wants decisions surfaced as structured choices, not prose.*
8. **Physical+digital split for hardware**: user plugs/moves/reads screens; agent flashes/configures over mount/SSH/WiFi; shell-output pastes as the interface; fish-shell awareness. → *Agent as remote hands, user as physical hands.*
9. **Anti-shortcut enforcement**: "BYPASS DETECTED — re-do" + "undo the optimizations that broke everything" + "don't just answer what's easy" + guards after machine crashes + "make sure you web search forums and try real fixes". → *Shortcut detection + hard corrections; agent must do the real work.*
10. **Privacy/anonymity gradient** (kiosk-specific instance of a general trait): fake email → anonymous email → VPN → fingerprint → PII audit; no accounts/API-keys/torrents. → *Strong privacy defaults; honest spoofing only (never lie about real hardware specs).*
11. **Design-iteration loop**: casual-not-buzzwordy prose, D2 diagram readability, React theme tweaks — all share: immediate visceral verdict ("looks terrible", "perfect") + concrete fix + re-render + re-judge. → *Aesthetic feedback is iterative and verbal, expects agent to self-render and self-check.*
12. **Fan-out research then distill**: [search-mode]/[analyze-mode]/[CONTEXT] multi-agent asks + "distill the reports into model options" + "tell me all about" summaries + demo links. → *Parallel research, single distilled answer with citations.*

### Prompt-style fingerprint (for authoring new test prompts)
- Run-on directive chains: "then / after that / once / so", single paragraphs swallowing whole pipelines
- Explicit stop conditions + anti-skip language: "Don't stop until", "don't skip steps", "operate efficiently and quickly"
- Permission gating for costly ops (LLM loads) in CAPS
- Self-reference to own prompting habits ("modeled off my prompts in opencode", "like how I prompt")
- Mixed specificity: exact paths/counts/§-numbers AND vague gestures ("and so on", trailing "...")
- Task/GOAL/WORKDIR templates for delegation; [CONTEXT] wrappers for research fan-out
- Numerical targets freely invented (50 cases, 10-20 minutes, 5/15 ratio, 1k-2k words)
- Verdicts short and emotional: "perfect", "awesome!", "looks terrible", "sorry, ignore the previous prompt"
- Steering minimalism: "continue", "status?", "C", "yes", "you run"
- Corrections mid-stream without anger: "no, I want...", "actually let's...", "sorry pasted wrong thing"

### Constraint inventory (never-drop list)
- NO parallel/background agents in whitt workspace (synchronous rule); z.ai coding plan models only (GLM-5.3 orchestration); caveman terse output; hardware safety (never crash the machine, CPU-first for new models, no 2 models loaded); never lie about specs; no API keys/accounts where avoidable; verify before claiming; repo-root cleanliness; TDD for framework changes; schema-discipline for YAMLs.
