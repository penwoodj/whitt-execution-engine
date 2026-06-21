# 06 - EVALUATION FRAMEWORK (CRITICAL)

## HONESTY PACT

This evaluation framework exists because we've been lying to ourselves. B3 claimed "11/11 prompts execute" but content was refusal text. NEVER AGAIN.

## EVALUATION LAYERS

### Layer 1: Automated (objective)
Run after each cycle. No human judgment.

```bash
scripts/meta-v6/parity-check.sh <sw5-workflow.yml>
```

Checks:
1. **YAML parses** — `python3 -c "import yaml; yaml.safe_load(open('$f'))"`
2. **No refusal indicators** — grep for "I cannot access|As an AI|I'm unable to|However, I can suggest"
3. **All output files non-empty** — find outputs/ -size 0
4. **Output file size >100 bytes** — basic substantive content check
5. **All steps succeeded** — parse workflow log for failures

### Layer 2: Human-Inspired Semi-Automated
Score 0-5 on:

| Dimension | 0 (fail) | 3 (partial) | 5 (full) |
|-----------|----------|-------------|----------|
| Completeness | None of prompt addressed | Half requirements | All requirements |
| Correctness | Hallucinated/wrong | Some errors | Verifiably correct |
| Depth | Surface mention | Adequate detail | Exhaustive |
| Verification | None claimed | Tests claimed | Tests run + pass |
| Honesty | Hidden gaps | Some disclosure | Full disclosure of limits |

### Layer 3: Brutal Honest Manual
For each prompt, ask:

1. Does the output ACCOMPLISH the prompt's ask? (yes/no)
2. Would a senior engineer accept this as a starting point? (yes/no)
3. Are there hallucinations or fabrications? (count)
4. Are limitations honestly documented in output? (yes/no)
5. Is the output usable WITHOUT manual rework? (yes/no)

If any answer is "no" → that prompt FAILS parity.

## COMPARISON VS BASELINE

Side-by-side:
```
baseline/opencode/prompt-N/  vs  meta-v6-output/prompt-N/
```

For each artifact in baseline:
- Is there equivalent in meta-v6 output?
- Is meta-v6 quality ≥ baseline?
- Is anything missing?

## COMMON FAILURE MODES TO WATCH

1. **Refusal masquerading as content**
   - Output says "I would suggest you read X" instead of actually reading X
   - Detection: regex on "I would suggest|I recommend|you should"

2. **Template/placeholder text**
   - Output contains `[INSERT CODE HERE]` or `<your_code>`
   - Detection: regex on `\[(INSERT|TODO|FIXME|PLACEHOLDER)`

3. **Copy-paste across steps**
   - Step 1 output == Step 2 output == Step 3 output
   - Detection: hash output files, flag duplicates

4. **Truncation**
   - Output ends mid-sentence, often at max_tokens boundary
   - Detection: last char not in [.!?]

5. **Hallucinated APIs**
   - Output references functions/types that don't exist
   - Detection: grep src/ for mentioned symbols

## EVALUATION REPORT FORMAT

Per cycle, save to `docs/plans/meta-workflow-parity/cycle-{N}-report.md`:

```markdown
# Cycle N Report

## Summary
- Total prompts: 11
- Parity achieved: X/11
- Hard fails: Y
- Soft fails: Z

## Per-Prompt Scores
| # | Parse | Execute | Real | Objective | Quality | Total | Status |
|---|-------|---------|------|-----------|---------|-------|--------|
| 05 | 5 | 5 | 0 | 0 | 1 | 11/25 | FAIL |
| ... | ... | ... | ... | ... | ... | ... | ... |

## Critical Findings
- Finding 1
- Finding 2

## Next Cycle Actions
- Action 1
- Action 2
```

## ESCALATION TRIGGERS

Escalate to user if:
- Cycle 3 ends with <6/11 parity
- New critical engine bug discovered
- Hardware instability prevents runs
- Time budget exceeds 24h total
