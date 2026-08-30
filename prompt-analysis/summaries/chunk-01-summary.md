# chunk-01 summary (prompts 0001-0050, 06-23 → 06-28)

Deduplicated asks (direct + indirect):

## Whitt engine / meta-workflow parity
- Audit meta-workflow-parity project ground truth: 11 test prompts, parity-check.sh scoring, cycle-3 outputs, parity claims vs goal (0009-0012)
- Review/rigorously check 00-COMPREHENSIVE-PLAN.md against reality (0040)
- Assess live infra state: llama.cpp Vulkan Docker, SW1-SW5 pipeline (0018-0019)
- Indirect: wants audits to cite verified reality, not claims; "what remains incomplete / not live system tested" as recurring audit frame (0037)

## Opencode behavior/configuration
- Complains opencode stops iterating; asks why, wants config fix so it keeps iterating until objective believed done (0025-0026)
- Demands synchronous one-at-a-time execution, no pauses, wait for shell (0014) [→ became Synchronous Operation Rule]
- Asks to use multiple-choice questions in opencode interactively (0032)
- Ralph Loop command invocations (0001, 0028, 0034-0035, 0038-0039)
- Context-dump prompt for new workspace, nothing lost (implied, later explicit 0138)
- "status then continue" pattern; caveman mode request (0046)
- Distill another chat's response into model options (0087-0088, chunk 2 boundary)

## Left-Right language (Rust compiler)
- Operator×type behavior matrix spec doc build; batch Q&A updates (Batches 5-12), mark TBDs, flag conflicts, surgical corrections (0036, 0048, 0051, 0054, 0057)
- Fix confirmed VM semantic bugs in compiler/crates/lr-vm/src/vm.rs; reconcile tests (0052, 0056)
- Avoid redundant questions re documented behavior (0031)
- Recall operators discussed earlier in chat (0041-0042)
- Live-system-testing demos: real .lr files run w/ captured output (0053)
- Debug diagnostic markdown for P08 prompt as baseline comparison (0033)

## Plugin security research
- Install opencode plugin (timestamps) only after verifying secure/safe from GitHub (0003)
- Research opencode-* npm visual plugins (0004); audit opencode-timer-plugin v1.0.5 pre-install (0005)
- Indirect: pre-install security audit workflow for npm packages

## AI compute hardware notes
- Consolidated report of AI compute hardware mentioned in personal notes under /home/jon (0059-0067 series)
- Indirect: notes span chats; wants consolidation into single report

## Misc
- Context length decision: 32k default (2x longest ~16k response), faster, increase when needed (0050) [→ Context Length Policy]
- "continue"/"stop"/"what are you doing?" steering prompts
