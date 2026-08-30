# chunk-02 summary (prompts 0051-0100, 06-28 → 06-28)

## SBC / hardware research (dominant theme)
- Exhaustive SBC model research: ALL Orange Pi w/ DDR5 ≥8GB; ALL Orange Pi DDR4 ≥16GB (32/64GB); ALL Banana Pi w/ 32/64GB DDR4; AliExpress listings w/ 32/64GB; ALL RK3588/RK3588S boards 8/12/16GB+; COMPLETE current catalogs Orange Pi + Banana Pi (0070-0080)
- Price/reality checks: Allwinner A733 12GB LPDDR5 $150 claim verify; "no opi 5 pro is currently $350"; radxa cubie no 16GB, 8GB >$300; Orange Pi 4 Pro 12GB $275 (0095-0101)
- Purchase decision: Orange Pi Zero 3W 12GB LPDDR5 A733 octa-core 3TOPS NPU — as TV/media streamer + general use (0102-0103)
- Indirect: wants verified specs vs marketing claims; price-to-RAM ratio driving decisions
- Power/USB feasibility for OPi: 5V/3A fixed USB-C, 20W PSU questions, non-powered hub w/ data USB-C, keyboard+mouse BT vs hub, HDMI+KB+mouse+32GB drive through one port (0105-0109)

## AI compute hardware report
- Consolidated AI compute hardware report from personal notes (/home/jon, "mainly in chat") — multiple restarts after crashes/restarts (0059-0067) — indirect: session continuity matters, retries same task after interruption

## Whitt engine
- Gap analysis: what hooks/tools does opencode have that engine lacks (0091); what in unified schema .yml re tool/skill/script usage is unimplemented (0092); what remaining gaps to SURPASS opencode reliability (0093); "what else could we add to help?" (0094)
- Indirect: benchmark framing = parity then surpass opencode

## Opencode meta
- Distill other chat's report into specific model options (0087-0088)
- Steering: continue ×2, status, "what are you doing?"
- Crash recovery: "computer restart continue" (0063)

## Left-Right language
- Fix 3 confirmed VM bugs (lr-vm/src/vm.rs, lexer/parser if needed) + tests (0056); update operator-type-matrix Batch 11+12 surgical (0057)
