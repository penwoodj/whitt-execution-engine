# 02 - VALIDATION CRITERIA (STRICT)

## REVISED 2026-06-21 — User clarified parity = info + code equivalence

Parity is NOT just analysis. Workflow must produce ACTUAL FILE CHANGES equivalent to what opencode would do. Resilient across varying prompts. Sustainable (no crashes).

## PARITY PASS = ALL CRITERIA MET

For meta-v6 to "pass" on a prompt, ALL of:

### Criterion 1: Workflow Parses
- [ ] SW5 output `workflow.yml` is valid YAML
- [ ] Passes `whitt validate` (or schema check)
- [ ] fix-yaml.py post-processor clean (no fixes needed OR fixes documented)

### Criterion 2: Workflow Executes Resiliently
- [ ] `whitt benchmark --workflow workflow.yml` completes exit 0
- [ ] All steps succeed OR documented graceful failures
- [ ] No Docker crashes mid-execution
- [ ] Survives varying prompt complexity (simple → complex)

### Criterion 3: Output Real (Not Refusals)
- [ ] Output artifacts contain SUBSTANTIVE content (not "I cannot access...")
- [ ] Refusal indicator strings absent from ≥95% of outputs
- [ ] Output addresses prompt ask (manual inspection)

### Criterion 4: ACTUAL FILE MODIFICATIONS (USER REQUIREMENT)
- [ ] For code prompts: workflow WRITES modified files to actual source paths (e.g., src/benchmark/runner.rs)
- [ ] Workflow includes step that READS source, GENERATES new content, WRITES back to source
- [ ] Modifications match what opencode would do (functional equivalence)
- [ ] Verification: cargo check/test runs via shell hook, captures result

### Criterion 5: Quality Bar (vs Baseline)
- [ ] Workflow output quality ≥ opencode baseline (scored 0-5)
- [ ] Or workflow output SURPASSES baseline on ≥2 dimensions
- [ ] No critical information missing vs baseline

## FAILURE MODES (HARD FAILS)

- Refusal text in outputs → HARD FAIL
- Empty output files → HARD FAIL
- YAML invalid after fix-yaml.py → HARD FAIL
- Workflow crashes Docker → HARD FAIL
- Same content copy-pasted across steps → HARD FAIL
- Model outputs template/placeholder instead of content → HARD FAIL
- **Workflow outputs only analysis text, no file modifications → HARD FAIL** (NEW)
- **Workflow can't survive 3 consecutive runs without manual intervention → HARD FAIL** (NEW)

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

## RESILIENCE REQUIREMENT (NEW)

Pipeline must be RESILIENT:
- Per-prompt Docker restart (current approach, stable)
- Recovery from model timeout/OOM
- No manual intervention mid-batch
- Varying prompt complexity handled gracefully

## AUTOMATION

Build `scripts/meta-v6/parity-check.sh`:
- Input: SW5 workflow.yml
- Output: JSON report with criterion scores
- Auto-detect refusals (regex on output files)
- Auto-validate YAML
- Auto-execute workflow + count steps succeeded
- Manual score input for criteria 4 and 5
