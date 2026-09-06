# Test Case Suite — 170 Cases

Generated from the meta-analysis of 1000 real user prompts (`../TOP-LEVEL-SUMMARY.md`). Every case matches the user's documented language, prompt-organization patterns, and ask shapes, and is fully runnable + validatable inside `/tmp/opencode/` workspaces (no repo files, no network, no LLM calls).

## Tiers

| Tier | Count | Word band | Actual | Shape | Files |
|------|-------|-----------|--------|-------|-------|
| A | 100 | 1000–1500 | 1020–1128 | long-form multi-stage: full pipelines, harnesses, simulators w/ verification + efficiency demands | `tier-A-1000plus/case-A-NNN-*.md` |
| B | 20 | ~350 | 301–337 | medium single-topic: one tool + verify + paste-in-chat | `tier-B-350/case-B-NNN-*.md` |
| C | 50 | 50–100 | 55–69 | short steering-style: one small /tmp task | `tier-C-50-100/case-C-NNN-*.md` |

Tier-A cases 001–003 are the hand-approved originals (inventory-pipeline, regression-harness, capacity-planner); 004–100 are generated.

## Criteria (all 170, mechanically verified)

1. Word count within tier band (body words; `#` headers excluded)
2. `/tmp/opencode/` workspace isolation stated
3. Runnable + self-validatable: fixtures self-generated, verifier demanded, exit codes visible
4. Unique per case: openings unique by construction; content uniqueness measured **beyond** the shared voice bank
5. User-voice fingerprint: run-on directive chains, lowercase register, "don't stop until" gates, anti-vibe verification demands, mid-stream style rules, small-decisions-delegated ("names and orderings are yours")

## Voice-bank ceiling (design decision)

Cases are assembled from unique per-case skeletons (subject, entities, numeric facts, asks, messiness quirks) + a shared bank of connective sentences drawn from the fingerprint in `../TOP-LEVEL-SUMMARY.md`. The bank sentences repeat **by design** — the real prompt corpus reuses stock phrases across prompts ("don't stop until", "paste it in chat", "not vibes"), and stripping them would make the cases less faithful, not more. Uniqueness therefore lives in skeleton + lead + asks (unique by construction); the verifier (`../scripts/tc-verify.py`) excludes bank n-grams via `bank_exclusions()` and checks uniqueness of the remaining prose. Sharing threshold: 10-gram across >20 files fails **only if not bank-derived**.

## Routing value

Lane spread mirrors the top-level workflow router's atoms: tier-A = HEAVY 40 / SYNTH 57 / LIGHT 10 (lane in each file's title line); tier-B mixed LIGHT/SYNTH; tier-C mostly LIGHT single-hop. Domain coverage: logs/monitoring/incident, data wrangling, finance/ops reconciliation, scheduling/constraint solving, simulators (rate limiting, breakers, schedulers, backups), file hygiene, config/schema audits, text/document processing, git archaeology, stats/reporting.

## Regenerate / verify

```bash
python3 prompt-analysis/scripts/test-case-gen.py   # tier-A 004-100 (seeded, deterministic)
python3 prompt-analysis/scripts/tc-c-gen.py        # tier-C 001-050
python3 prompt-analysis/scripts/tc-verify.py       # verify all 170 + write INDEX.csv
```

Tier-B is hand-authored (generation scripts `tc-b-extend.py` applied voice extensions; originals live in git history). `INDEX.csv` = machine index (tier, file, words, case_number).

## Known limitations

- Tier-A word ceiling 1128 < 1500 band max: targets 1030–1250 with fill; acceptable, band is a floor not an aim-point
- Bank sentences occasionally appear in 2+ cases verbatim (≤ ~20 bank paragraphs rotate) — matches real-corpus behavior
- All fixtures synthetic; no case requires network, Docker, or models by design
