# chunk-04 summary (prompts 0151-0200, 07-01 → 07-07)

## Whitt engine / model infra
- llama.cpp Docker RX 580 RADV/Vulkan w/ Qwen3.5-9B-Q4_K_M contexts (0163)
- SW4 workflow translation checks (0164); models/ dir + meta-v6.yml model choice (0165)
- GPU CRASH incident: "something you're doing is crashing my machine when you run the workflow. undo the optimizations that broke everything" (0169); "you used my graphics card in a way that caused it to crash. when trying new models do CPU only just to get a baseline first" (0171) — indirect: safety-first model testing, baseline-then-GPU discipline
- "Continue and don't stop. after you've tested multiple models including the weirder named promising model version, at least [N]" (0172) — breadth model sweeps
- "Start with your batches and don't stop until it's done" (0174)
- Time-based model comparison to iterate on an objective: smaller=faster but dumber, [larger=slower but smarter] (0177) — indirect: speed/quality tradeoff framing
- Model crash → "restart the service" patterns

## Desktop apps / peripherals
- Install Bambu Studio app (0166)
- Raspberry Pi Zero W attached; identify v1 vs v2 hardware; via data cable; identified v1.1 → flash SD w/ new OS, "no changes just answer"; Raspberry Pi Imager install on Linux (not in apt); OS image download for BCM2835 ARMv6 512MB (0178-0183)
- Research PStream GitHub project: description, sys reqs, Pi compat; custom apps as OS-like systems on Pi; PStream Chrome extension server expansion (0185-0188)
- PStream service not typing → restart service; still broken → "get it working again" (0189-0190)
- "Option 1 and extension first, make what you need in subfolder" (0191)
- No microSD reader → [start with what we have]; renamed folder (0192)

## Media streaming kiosk (begins)
- PIVOT to new hardware: Orange Pi [Zero 3W] arrived (0193)
- flash-sd.sh --skip-download run, sudo, chromium requirement for extension (0194-0196)
- Self-hosted streaming aggregator on Pi/OPi: NO API keys (no TMDB/Trakt), NO accounts (0197-0198)
- "Does [Magnetio] torrent providers stream and not seed?" — seeding concern (0199)
- NO torrents, no API keys; PStream/movie-web provider architecture research; CloudStream Android-only → [need self-hosted]; HTTP-streaming aggregator hunt (0201-0204)

## Indirect patterns
- Privacy/anonymity concerns emerge (no accounts, no seeding)
- Iterative hardware bring-up w/ agent over WiFi
- Research delegation via [CONTEXT] prompts (multiple parallel subagent asks per decision)
