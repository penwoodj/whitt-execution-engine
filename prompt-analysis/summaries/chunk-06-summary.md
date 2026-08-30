# chunk-06 summary (prompts 0251-0300, 07-10 → 07-11)

## Media kiosk iteration (continues)
- Loading-screen wallpaper: analyze 3 PNGs at different load intervals → select best for desktop wallpaper (0261-0273) — indirect: heavy retry fan-out of same multimodal ask; retrieve results from background tasks bg_*
- Viewing state not saving between reboots (0280); GDBus panel-restart error recurrence (0274); --disable-blink-feature flag warning on screen (0277)
- Chromium should open in BACKGROUND (alt-tab accessible), not foreground (0285)
- "Iterate and read the actual screen state and use the app as you iterate. Don't stop until you have this [working]" (0288) — indirect: expects agent to self-verify via real UI usage
- Save/cancel button on ALL pages, not just one; "don't stop until all of this is done and verified" (0289)
- Remove broken sources; organize sources by which actually work; ensure sources supporting [TV/movies] (0294)
- Runtime triage: "global invalid source id on every movie"; notification button back; thumbnails gone but continue-watching thumbnails remain (0295-0296)
- Watched Idiocracy on TV fine → next title failed (0297)
- Provider upgrades: latest @p-stream/providers w/ emoji-named sources (Artemis, Cosmi[?]...); why can't we just add same provider links to our repo (0298-0299)
- PStream fork path: /home/jon/code/pirate-sprite/raspberry-pi-zero-w-v1.1/build/p-stream-src/ (0291, 0293)

## Indirect patterns
- Batch [search-mode]/[analyze-mode] + Ralph Loop invocations continue
- Multi-attempt identical multimodal prompts → retries when tool/format failed; values persistence
- Wants curated WORKING sources over quantity
