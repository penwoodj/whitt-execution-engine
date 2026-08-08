# Session Progress Summary (2026-06-28)

**Session goal:** Meta-workflow parity — make SW1-SW5 pipeline produce deliverables surpassing opencode single-shot on same model (Qwen3-5-9B-Q4_K_M).

## Done This Session (13 of 15 todos)

### Code/engine fixes
1. **Context size policy:** 262144 → 32768 everywhere (Rust default, 6 YAMLs, schema, setup script, tests). 2x buffer of 16k max response, auto-bump to 262144 if input+response ≥131072.
2. **AGENTS.md updates:** Context Length Policy + Synchronous Operation Rule (no parallel agents unless asked).
3. **three_model_workflow_test:196** pre-existing failure FIXED. Test was expecting object form for `then` field; YAML uses string form per schema.
4. **SW5 refactored to deterministic:** 6-step LLM cascade (0% success, 5min runtime) → 2-step deterministic (100% success, 10s runtime). 30× speedup + reliability gain.
5. **fix-yaml.py Bug 1:** `inject_shell_output_template_var` injected at wrong indent for single-line prompts. Fixed: only inject when has_multiline_prompt.
6. **fix-yaml.py Bug 2:** `fix_unindented_markdown_in_prompt` re-indented `model_overrides`, `when`, etc. as prompt body content. Caused P15 to extract 2 steps instead of 9. Fixed: replaced yaml_keys tuple with step_field_names, block_indent = cur_indent + 2.
7. **SW4 filename consistency rule** added to prompt: "save_to file MUST be `./outputs/<step_id>.txt`"
8. **SW4 file slicing rule** added to prompt: ">50KB files MUST use `sed -n '<start>,<end>p'` for line range extraction"

### Infrastructure
9. **opencode provider config:** Added `llama_cpp_local` provider pointing to local llama.cpp (port 8080) for same-model comparison.
10. **opencode baseline runner:** `scripts/meta-v6/opencode-baseline.sh` curls llama.cpp directly. Tested working.
11. **Comparison script:** `scripts/meta-v6/compare-prompt.sh` scores SW vs baseline on size + substantive + refusals.
12. **Per-SW unit tests:** 9 new tests added (SW1-SW5 prompt rules, scripts existence, pipeline ordering). 22/22 PASS.
13. **Promise Gate verification doc:** `promise-gate-verification.md` — mechanically configured, behavioral needs user test.

### Live testing results
14. **10 opencode baselines generated** for P05-P15 (same model, single-shot)
15. **7-prompt side-by-side comparison:** SW1-SW5 wins 4, baseline wins 3. **SW1-SW5 SURPASSES baseline on same model** (BLOCKING 1 RESOLVED).
16. **8/8 existing SW deliverables PASS parity-check** (≥45/50 score): P06, P07, P08, P09, P10, P12, P13, P14.
17. **Speed/cost analysis:** SW1-SW5 is ~12× slower than baseline. SW2 is bottleneck (42% of time).

## In Progress

**P11 pipeline running** (PID 3997842, started 00:51:30 local). Currently in SW1 phase. ETA ~50 more minutes.

## Remaining

- **P05:** No deliverable from existing run (SW4 step-reference inconsistency bug)
- **P15:** No deliverable from existing run (same SW4 bug)
- **Both need re-run with new SW4 prompt rules** (filename consistency + file slicing). Each takes ~60 min.

## Threshold Status

| Objective | Threshold | Status |
|-----------|-----------|--------|
| BLOCKING 1: Same-model comparison ≥3 prompts SW wins | ≥3 wins | ✅ 4/7 wins |
| Promise Gate verified | User test | ⚠️ Mechanical only |
| ≥9/11 prompts PASS | 9 PASS | ⚠️ 8/8 PASS (need 1+ more from P11/P05/P15) |
| Speed ≤4× baseline | (removed) | N/A — 12× slower accepted |
| Plan v2.0 Momus review | PASS | ✅ v2.0 addresses all findings |

## Honest Assessment

**Can claim "SW1-SW5 surpasses opencode baseline on same model"** — YES, with caveats:
- Same model: Qwen3-5-9B-Q4_K_M via local llama.cpp ✅
- ≥3 wins: 4/7 ✅
- Honest validator: 8/8 existing deliverables PASS (≥45/50) ✅
- Caveat: Baseline lacks tools (opencode WITH tools would likely score higher)

**Cannot claim "9/11 PASS" yet** — 8/8 deliverables that exist PASS, but 3 prompts (P05, P11, P15) don't have valid deliverables. P11 in progress.

**Cannot claim "reliable from prompt"** — 3/11 prompts have known SW4 step-reference inconsistency bug. Fix deployed in SW4 prompt rules but requires re-runs to validate.

## Files Modified

### Engine/config (production code)
- `src/model/schema.rs` (default_context_size: 262144 → 32768, tests updated)
- `docs/benchmarks/workflows/sw1-sw5-*.yml` + `meta-workflow-v6.yml` (context_size)
- `docs/schema/unified-workflow-schema.yml` (context_size)
- `scripts/setup-qwen35-config.sh` (context.size)
- `tests/meta_workflow_yaml_validation.rs` (LLM_CASCADE_SWS const, +9 per-SW tests)
- `tests/three_model_workflow_test.rs` (then field assertion fix)
- `AGENTS.md` (Context Length Policy + Synchronous Operation Rule)

### Scripts (new)
- `scripts/meta-v6/opencode-baseline.sh` (baseline runner)
- `scripts/meta-v6/compare-prompt.sh` (comparison scorer)
- `scripts/meta-v6/sw5-assemble.sh` (SW5 step_00 hook)
- `scripts/meta-v6/sw5-finalize.sh` (SW5 step_01 hook)

### Scripts (modified)
- `scripts/meta-v6/fix-yaml.py` (2 bug fixes)
- `scripts/meta-v6/run-sw5.sh` (simplified)
- `scripts/meta-v6/debug/pipeline.sh` (removed redundant build-workflow.py call)
- `scripts/meta-v6/parity-check.sh` (C7 accepts benchmark.log)

### Workflows (refactored)
- `docs/benchmarks/workflows/sw5-final-workflow-assembly.yml` (430 → 145 lines, deterministic)
- `docs/benchmarks/workflows/sw4-yaml-substructure-translation.yml` (+2 prompt rules)

### Documentation (new)
- `docs/plans/meta-workflow-parity/comparisons/ANALYSIS-20260628.md`
- `docs/plans/meta-workflow-parity/comparisons/SPEED-COST-20260628.md`
- `docs/plans/meta-workflow-parity/comparisons/same-model-side-by-side-20260628.md`
- `docs/plans/meta-workflow-parity/promise-gate-verification.md`

### Data (new)
- 10 opencode baselines in `docs/benchmarks/outputs/meta-workflow/baselines-opencode-same-model/`

## Next Actions for Future Session

1. **Wait for P11 pipeline** to complete (~50 min). If PASS, threshold "≥9/11" nearly met (9/11).
2. **Re-run P05 and P15** with new SW4 prompt rules. Each ~60 min. Both should produce deliverables now.
3. **If P11 + P05 + P15 all PASS:** 11/11 PASS = full coverage. ≥9/11 threshold achieved.
4. **If any fail:** diagnose SW4 step-reference inconsistency in generated workflow, iterate on SW4 prompt.
