# Human-Judgment Sampling Protocol — DEPRECATED

**Status:** DEPRECATED 2026-08-06 per user direction.
**Replacement:** `scripts/meta-v6/llm-judge.py` (LLM-as-judge automated reviewer).
**Kept for:** Fallback if LLM judge unavailable, or when LLM judge returns PARTIAL.

---

## Original Protocol (superseded)

**Scope:** Minimal human review on top of `parity-check.sh` + `semantic-check.py` to catch Goodhart cases that heuristic validators miss.

**Why:** Per AGENTS.md, `parity-check.sh` is the gate but it can be gamed. `semantic-check.py` (C9) covers keyword/structure/refusal paraphrases. The remaining gap is **sophisticated gaming** — generators that stuff keywords without semantic understanding, or produce plausible-sounding but wrong content.

## Sampling Rate

| Cycle Phase | Prompts to Review | Selection |
|-------------|-------------------|-----------|
| Active iteration (cycle N) | **2 random** from PASS set | `shuf` from prompts that scored ≥50/60 |
| Pre-release | **All passing** prompts | 100% review |

**Minimum N=2 per cycle.** Maximum bounded by cycle size. Reviews focus on **workflow specification deliverables** (YAML output of SW1-SW5), not intermediate step outputs.

## Review Checklist (per sampled prompt)

Reviewer answers each YES/NO/PARTIAL:

1. **Objective alignment**: Does the workflow YAML accomplish the prompt's stated goal?
2. **Step semantic correctness**: Are the SW1-SW5 step prompts/logic semantically valid (not just keyword matches)?
3. **Output deliverable quality**: Does the executed deliverable actually solve the user's problem?
4. **No subtle gaming**: Is the generator producing real understanding vs keyword-stuffing?
5. **No unstated refusals**: Is the model genuinely engaging, not politely deflecting?

**Veto rule:** Any PARTIAL or NO on Q1 (objective alignment) overrides validator PASS. Cycle cannot exit until vetoed prompt is fixed.

## Procedure

```bash
# 1. After cycle completes, identify PASS set
PASSED_PROMPTS=$(ls docs/benchmarks/outputs/meta-workflow/cycle-N-results/ | grep -E "PASS$")

# 2. Random sample 2
SAMPLED=$(echo "$PASSED_PROMPTS" | shuf -n 2)

# 3. For each, run parity-check.sh WITH prompt-file to get C9 score
for p in $SAMPLED; do
    bash scripts/meta-v6/parity-check.sh \
        <workflow.yml> <deliverable> <exec-dir> <prompt-file> \
        > docs/qa/cycle-N-human-sampling/$p.log
done

# 4. Reviewer fills checklist into docs/qa/cycle-N-human-sampling/$p.review.md
# 5. Cycle exits only if all sampled reviews pass Q1 veto check
```

## Review File Template

`docs/qa/cycle-N-human-sampling/<prompt-id>.review.md`:

```markdown
# Human Review: <prompt-id>

**Reviewer:** <name>
**Date:** YYYY-MM-DD
**Validator scores:** parity=NN/60, semantic=NN/10

## Checklist
- [ ] Q1 objective alignment: YES/NO/PARTIAL — <one sentence>
- [ ] Q2 step semantic correctness: YES/NO/PARTIAL — <one sentence>
- [ ] Q3 output deliverable quality: YES/NO/PARTIAL — <one sentence>
- [ ] Q4 no subtle gaming: YES/NO/PARTIAL — <one sentence>
- [ ] Q5 no unstated refusals: YES/NO/PARTIAL — <one sentence>

## Veto Decision
- [ ] CYCLE PROCEEDS (Q1 = YES)
- [ ] CYCLE BLOCKED (Q1 = NO/PARTIAL — requires fix before exit)

## Notes
<optional observations>
```

## Time Budget

- Per-prompt review: **5-10 min** (read workflow YAML + deliverable, fill checklist)
- Per cycle: **10-20 min** for 2 sampled prompts
- Pre-release full review: **30-60 min** for 6-11 prompts

## When to Expand Sampling

Trigger expanded sampling (N=5 or 100%) when:
- 2 consecutive cycles produce a sampled FAIL
- Generator changed significantly (new prompts, new SW logic)
- Pre-release gate (always 100%)

## Integration with parity-check.sh

`semantic-check.py` (C9) is the **automated first filter**. Human sampling is the **second filter on what passes C9**. Layered defense:

```
Generator output
     ↓
parity-check.sh C1-C8 (existing checks)
     ↓ PASS (>= threshold)
semantic-check.py C9 (Goodhart heuristic)
     ↓ PASS (>= 6/10)
Human sampling (N=2 per cycle)
     ↓ PASS (Q1 = YES)
Cycle exit
```

## Out of Scope

- LLM-as-judge automated review (considered, rejected for time cost per user direction)
- Review of intermediate SW1-SW5 step outputs (only final workflow + deliverable)
- Review of archived/cycle-N-history prompts (only current cycle)
