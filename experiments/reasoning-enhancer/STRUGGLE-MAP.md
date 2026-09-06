# Struggle Map — 30 Small-Model Weakness Probes (case-s01..s30)

**Purpose:** input set for the next experiment. Each case isolates ONE
documented small-model failure mode with a mechanical check and a
satisfiable reference fix (lint-enforced). Run directly through a model
("does it struggle?") and/or through the enhancer ("does the scaffold
fix it?").

**Evidence sources:** `LIVE` = observed in this repo's benchmark runs
(9B = Qwen3-5-9B direct, 4B/1.7B in cascade — see
`results/SUMMARY-benchmark.md`); `LIT` = `RESEARCH.md` citations.

## Compliance / format discipline

| case | struggle mode | hypothesis | evidence |
|---|---|---|---|
| s01 | char-exact format | decorates exact tokens with fences/labels | LIVE — 32/58 9B subcheck failures were exact-format |
| s02 | word-budget floor | cannot hold a word window; undershoots floors | LIVE — 9 word-budget 9B failures |
| s03 | markdown restraint | defaults to `*`/bold even when banned | LIVE — 9 forbidden-phrase 9B failures |
| s08 | instruction override | follows first instruction, misses the correction | LIVE — b49/b50 both directions proven |
| s11 | copy fidelity | paraphrases when asked to transcribe verbatim | LIVE — 9B rewrote config values/order |
| s12 | key-name precision | hyphen/case near-miss drift | LIVE — `eviction_policy` vs `eviction`, `listen-port` |
| s13 | numeric format | inserts thousands commas by default | LIVE — `$4,653` vs `$4653` |
| s25 | nested structure | flattens or renames nested mappings | LIVE — b41 nested limits/health |
| s26 | JSON exact pairs | spacing/quoting drift on exact strings | LIVE — b39/b40 pair-format misses |
| s30 | answer-only discipline | wraps bare answers in scaffolding | LIVE — meta-collapse B12; "The port is 8443." |
| s28 | constraint stack | satisfies 3 of 4 simultaneous constraints | LIVE — 43/50 benchmark cases failed exactly 1 subcheck |

## Cognition / derivation

| case | struggle mode | hypothesis | evidence |
|---|---|---|---|
| s04 | enumeration counting | miscounts matching items in mixed lists | LIVE — bullet/check count failures |
| s05 | multistep arithmetic | one hop slips, error propagates | LIVE — 42-31-5=8 guess; 324/6=55 drift |
| s07 | positional indexing | wrong-end / off-by-one counting | LIT — DialCoT: <10B need explicit sequential steps |
| s10 | string reversal | echoes input or adds commentary | LIT — char-level ops degrade with scale |
| s09 | character-level ops | drops/normalizes punctuation in transforms | LIT — same class as s10 |
| s14 | mixed unit conversion | mixes scales mid-derivation | LIT — DialCoT decomposition evidence |
| s16 | boundary max | returns nearest round number, not constraint max | LIVE — b-class "largest not exceeding" |
| s21 | distinct vs total | counts occurrences instead of unique values | LIT — counting-mode confusion |
| s22 | sort by derived | sorts by given column, not computed property | LIVE — derived-field misses (b45-class) |
| s29 | occurrence counting | counts non-matching tokens or misses repeats | LIT — T1: sLMs fail verification needing exact counting |

## Semantics / pragmatics

| case | struggle mode | hypothesis | evidence |
|---|---|---|---|
| s06 | negation filtering | inverts "did NOT" to positive selection | LIT — negation is a documented SLM weak spot |
| s15 | temporal boundary | mis-rolls arithmetic across midnight | LIVE — b19 overlap reasoning |
| s17 | allocation no-restate | restates totals, breaking sum contracts | LIVE — b01 double-count trap |
| s18 | empty-set wording | writes digit where word form required | LIVE — b37 "0 items" vs "zero items" |
| s19 | distractor resistance | pulls irrelevant numbers into computation | LIVE — decoys needed 3 strengthening waves |
| s20 | false premise | fabricates the missing entity instead of rejecting | LIT — hallucination under pressure (CoVe motivation) |
| s23 | policy-filter extraction | lists all failures, not policy-matching subset | LIVE — b17 policy filter, b48 hash+size policy |
| s24 | cross-reference | binds pronoun to wrong entity across sentences | LIT — multi-sentence binding weakness |
| s27 | ambiguity detection | guesses instead of asking the clarifying question | LIT — SLMs answer rather than detect underspecification |

## Usage

```
# struggle probe (model alone) — reuse baseline machinery pattern:
#   emit-case --phase baseline, check-answer vs case checks
# enhance probe (workflow) — same pattern as benchmark enhance phase
python3 scripts/test-cases.py --no-draft-check \
  --cases-dir cases/struggle --fixtures fixtures/struggle-fixes.yml
```

Grading: `check_lib.run_checks` (same deterministic checks as the
50-case benchmark). A model "struggles" on a case iff the check fails;
the enhancer "fixes" it iff the workflow final passes the same check.
