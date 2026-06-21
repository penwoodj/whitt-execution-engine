# 02 - VALIDATION CRITERIA (STRICT)

## PARITY PASS = ALL CRITERIA MET

For meta-v6 to "pass" on a prompt, ALL of:

### Criterion 1: Workflow Parses
- [ ] SW5 output `workflow.yml` is valid YAML
- [ ] Passes `whitt validate` (or schema check)
- [ ] No fix-yaml.py post-processing needed (or minimal)

### Criterion 2: Workflow Executes
- [ ] `whitt benchmark --workflow workflow.yml` completes exit 0
- [ ] All steps succeed OR documented graceful failures
- [ ] No Docker crashes mid-execution

### Criterion 3: Output Real (Not Refusals)
- [ ] Output artifacts contain SUBSTANTIVE content (not "I cannot access...")
- [ ] Refusal indicator strings absent from ≥95% of outputs:
      - "I cannot access"
      - "I don't have access"
      - "As an AI"
      - "I'm unable to"
      - "However, I can suggest"
- [ ] Output addresses prompt ask (manual inspection)

### Criterion 4: Objective Addressed
- [ ] For code prompts: actual code changes/modifications proposed or made
- [ ] For research prompts: actual analysis/findings documented
- [ ] For doc prompts: actual structured documentation produced
- [ ] Output usable as starting point for human reviewer

### Criterion 5: Quality Bar (vs Baseline)
- [ ] Meta-v6 output quality ≥ opencode baseline (scored 0-5)
- [ ] Or meta-v6 output SURPASSES baseline on ≥2 dimensions
- [ ] No critical information missing vs baseline

## FAILURE MODES (HARD FAILS)

- Refusal text in outputs → HARD FAIL
- Empty output files → HARD FAIL
- YAML invalid after fix-yaml.py → HARD FAIL
- Workflow crashes Docker → HARD FAIL
- Same content copy-pasted across steps → HARD FAIL
- Model outputs template/placeholder instead of content → HARD FAIL

## SCORING RUBRIC

Per prompt, per criterion, score 0-5:
- 0 = total failure
- 1 = severe gap
- 2 = major gap
- 3 = partial
- 4 = minor gap
- 5 = full pass

**Pass = ≥4/5 on ALL 5 criteria**

## PARITY THRESHOLD

- 11/11 prompts pass = FULL PARITY ✅
- 8-10/11 prompts pass = ACCEPTABLE PARITY ⚠️ (with documented reasons)
- <8/11 prompts pass = NO PARITY ❌ (iterate or escalate)

## AUTOMATION

Build `scripts/meta-v6/parity-check.sh`:
- Input: SW5 workflow.yml
- Output: JSON report with criterion scores
- Auto-detect refusals (regex on output files)
- Auto-validate YAML
- Auto-execute workflow + count steps succeeded
- Manual score input for criteria 4 and 5
