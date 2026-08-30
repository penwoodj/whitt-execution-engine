# chunk-03 summary (prompts 0101-0150, 06-28 → 07-01)

## SBC purchase + OS setup
- Bought Orange Pi Zero 3W 12GB LPDDR5; next steps to get Linux OS on it; can it run CachyOS; wants lightweight OS to run ~/code/whitt-execution-engine on device (0117-0121)

## Whitt engine
- "Give me an extremely exhaustive prompt for the whitt execution engine opencode workspace to implement what is needed" (0123) — indirect: prompt-engineering own directives for reuse
- Implement + verify ALL missing items/gaps; build integration + e2e testing WITHOUT running a model (0127)
- Read output workflow + actual live run results → detailed report in same location (0134)
- "What from this prompt remains incomplete, no changes, exhaustively answer" (0135, recurring audit pattern)
- Continue + make exhaustive, document ALL gaps, then iterate with as [much as possible] (0136-0137)
- SW pipeline progress check: P10 SW2 started, tasks.md 17712 bytes (0116)

## Left-Right language
- Fix 6 VM bugs from Batch 13 answers (0122)
- Convert deprecated type-check ops `?!` `?//` to HARD ERRORS in vm.rs (0129)
- Add 3 new String operator behaviors (0130)
- 10 multiple-choice questions iteratively, fixing/implementing as answered (0131) — interactive spec-driven dev

## Gaming / desktop Linux
- Xbox controller on CachyOS, w/ Steam Path of Exile 2; dongle USB ID 045e; connection confirmations; crash triage ("it crashed again when trying to start it"); "make sure you are web searching forums and trying real fixes" (0140-0155)
- PoE2 Proton Experimental 11.0 + RX 580 RADV crash contexts (0156-0158)
- "Stop what you are doing, tell me exhaustively how we can make iterations much faster with same hardware" (0159) — indirect: speed obsession w/o hardware change
- "No changes just answer exhaustively, don't go beyond searching/reasoning/web research/processing" (0160) — read-only answer mode

## Opencode meta
- Context-dump prompt for new empty workspace, nothing lost (0138)
- "Continue if you have next steps, or stop and ask for clarification if unsure" (0084 pattern recurring)
- Ralph loop invocations (0110-0111, 0146-0147, 0151-0152)
