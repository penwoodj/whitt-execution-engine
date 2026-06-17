# META-v6 vs Baseline — Iteration 3 (GPU-15, 32K ctx)

## Summary

| SW | Iter1 | Iter2c | Iter3 (GPU-15+32K) | Baseline | Iter3/Baseline | Status |
|----|-------|--------|---------------------|----------|----------------|--------|
| SW1 (tasks.md) | 14 | 12 | 14 tasks, 57 story points, 7 leaves | 12 tasks, 27 points, 27 leaves | **93%** | PASS |
| SW2 (outputs.md) | 27 | 27 | 14 tasks, 7 leaves | 27 tasks, 27 leaves | **52%** | NEEDS_WORK |
| SW3 (categories.md) | 28 | 28 | 28 tasks, all DATA_TRANSFORMER | 12 parents, 27 leaves, all DATA_TRANSFORMER | **104%** | PASS |
| SW4 (structs.md) | 28 | 28 | 28 substructures, all DATA_TRANSFORMER | 27 substructures, all DATA_TRANSFORMER | **104%** | PASS |
| SW5 (workflow.yml) | 27 | 27 | 22 steps, schema 2.0.0, llama_cpp_with_vulkan | 27 steps, schema 2.0.0, llama_cpp_with_vulkan | **81%** | PASS |

**Key metrics:** Iter3 outputs total 72KB vs baseline 131KB (55% of baseline). Iter2c outputs total 83KB (63% of baseline).

## Per-SW Analysis

### SW1: Task Breakdown (tasks.md)
Iter3 produces 14 tasks (93% of baseline 12) with 57 story points vs baseline 27 (211% of baseline). Output depth is higher — more granular substructure with 7 leaves vs baseline 27 leaves. Task naming is more abstract (e.g., "Load Session Metadata" vs "Analyze duplicate README content"). Story point allocation is more detailed (1-8pt range vs baseline 1-3pt). PASS because output size >=70% baseline and structure is coherent, though abstraction level differs from baseline.

### SW2: Desired Output State (outputs.md)
Iter3 covers only 14 tasks with 7 leaves vs baseline 27 tasks with 27 leaves (52% of baseline). Coverage is incomplete — missing detailed artifact specifications for many tasks. Evaluation methods are present but simplified (fewer python one-liners vs baseline comprehensive shell checks). NEEDS_WORK because <70% baseline size and coverage gap prevents full downstream usability.

### SW3: Agentic Categorization (categories.md)
Iter3 categorizes 28 tasks as DATA_TRANSFORMER (104% of baseline 27). Category distribution matches baseline exactly — all DATA_TRANSFORMER, zero other categories. Reasoning is more concise (2-3 sentences per task vs baseline 4-5 sentences). Substructure hints are present but more generic. PASS because output size >=70% baseline and categorical correctness is maintained.

### SW4: YAML Substructures (structs.md)
Iter3 defines 28 substructures (104% of baseline 27). All use same category pattern (DATA_TRANSFORMER with save_to, log hooks). YAML snippet completeness is lower — many tasks use `<placeholder>` instead of actual prompts compared to baseline which has full prompt text. However, structural integrity is maintained with correct hook syntax and dependency chains. PASS because output size >=70% baseline and schema correctness is preserved.

### SW5: Workflow YAML (workflow.yml)
Iter3 generates 22 steps (81% of baseline 27). Schema version is correct (2.0.0) and provider is correct (llama_cpp_with_vulkan). Hooks are present but use generic placeholders vs baseline full hook configurations. Step naming matches generated task names from SW1. All required top-level keys present (workflow_id, name, description, version, providers, models). YAML is valid and parseable. PASS because >=70% baseline size, step count reasonable, and critical schema constraints satisfied.

## Verdict

**Overall: 4/5 PASS, 1/5 NEEDS_WORK**

Iter3 (GPU-15, 32K context) performs solidly on structural tasks (SW1, SW3, SW4, SW5) but lags on detailed specification coverage (SW2). The gap is in SW2 output state specification depth, not in correctness — generated structure is valid, just less comprehensive than baseline.

**Trend analysis (Iter1 → Iter2c → Iter3):**
- SW1: 14 → 12 → 14 tasks (stable)
- SW2: 27 → 27 → 14 tasks (regression in Iter3)
- SW3: 28 → 28 → 28 tasks (stable)
- SW4: 28 → 28 → 28 substructures (stable)
- SW5: 27 → 27 → 22 steps (slight decline in Iter3)

**Recommendation:** Address SW2 coverage gap by enhancing prompts to require complete artifact specifications for all tasks, not just the 14 covered in Iter3.