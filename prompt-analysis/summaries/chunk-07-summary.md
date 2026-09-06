# chunk-07 summary (prompts 0301-0350, 07-11 → 07-20)

## PStream → Tuluflix Max+ (kiosk continues)
- Rebrand to "Tuluflix Max+" (0305)
- Z-Stream (formerly P-Stream) research: how Chrome extension enables emoji-named sources; how main bundle detects extension installed (0306-0307)
- TV show sources blocked by Cloudflare (Friends, R&M, Westworld, Silo, STD) — consider self-hosted [scraper/proxy] (0310, 0312)
- Want PUBLIC REST APIs (not HTML scrapers) returning playable stream URLs (0311)
- "Can't I just use the same routes as zstream.mov backend if all my sources are exhausted?" (0314)
- PStream Enhanced extension at /opt/chromium-extensions/pstream-enhanced/, manifest analysis (0317-0318)
- Indirect: provider-legality/pragmatism boundary — reuse upstream routes when own sources fail

## Model testing on whitt engine
- Try Yi-6B-200K-Airo-Claude-Puffin-Q4_K_M w/ meta workflow generator; report both [output + execution] (0319)
- Get it working with Qwen3.5-9B Q4, decent thinking-token budget, q8_0 KV-cache quantization (0327)
- "Do everything fully on the CPU with normal RAM until you get it working" (0328) — indirect: CPU-first bring-up, GPU after
- Add MiniCPM5-1B-F16 to testing mix at the end; continue, don't stop (0342)
- "Stop and finish things up so we can start live system testing" (0343)
- Add all remaining features discussed in Q&A, then tell me when to eject drive + start up (0344)

## Desktop apps / infra
- Install "keet io app" via AppImage, working in applications folder (0329-0332)
- Imbue video-call camera failure → xdg-desktop-portal-gtk + rtkit install/enable via pacman (0333-0338)
- New microSD for OrangePi: install new OS on it from the opi [itself] (0339)
- configure-sd.sh runs: ssh key errors, "remember I'm using a fish shell" (0346-0350) — indirect: shell-aware command generation
