# chunk-05 summary (prompts 0201-0250, 07-07 → 07-10)

## Media kiosk build (dominant)
- Provider research: consumet vs pstream source parity (0207); tmdb signup w/ fake email for API key; disposable-email rejected; anonymous email where; local VPN to mask IP before API use; leaked API key pasted (0208-0213, 0220)
- VPN client setup questions (NAT-PMP, VPN Accelerator, server choice, "how should I set this up before downloading?") (0214-0215)
- Runtime triage: sources empty → tmdb blocked error → sources present but not 70+ promised (0217-0219, 0229)
- Armbian reinstall option + custom kernel build explanation, "don't do it just answer"; local SD flash vs WiFi speed (0222-0223)
- Fan not spinning → check setting, drive mounted; boot loop (solid red light, no flashing) → read logs off SD over WiFi, fix then reinsert (0224-0228)
- Anonymity: "any other way this device could be tracked or associated with identity or PII?" (0230, 0233); fingerprint spoofing for Chromium kiosk (0236); "don't spoof concurrency or memory — it has 12GB lpddr5, tell [the truth]" (0237) — indirect: selective honesty-preserving spoofing
- Resolution stuck ~1200px not 1920 (0223 0235, 0237)
- Quality/config research: "look into what actually makes a difference quality and configuration wise, web search + think" (0238); ~7 sources per media vs 70+ promised (0239-0240)
- Rebrand PStream (movie-web React/TS Vite fork, nginx static) (0241)
- Skip onboarding flow every restart (0242)
- Extension load error /opt/chromium-extensions/; --no-sandbox warnings on screen (0243-0245)
- Remove startup password; then black screen; password back; USB hub (2A+2C→1C) feasibility, peripherals-only no power-in; ethernet-in + KB/mouse-out [adapter] question (0246-0250)
- Enable bluetooth; show/movie progress not saved → save browsing [progress] (0251)
- GDBus panel restart error triage (0253)

## Indirect patterns
- Privacy escalation: fake email → anonymous email → local VPN → IP masking → fingerprint → PII tracking audit
- Agent operates device over WiFi while user physically moves SD/hardware
- Tolerates pasting secrets (OpenVPN creds, API keys) in chat for setup help
- Wants promises (70+ sources) verified against reality
