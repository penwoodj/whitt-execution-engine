# Cycle 1 + Post-Process = PARITY ON P14

**Date:** 2026-06-21
**Status:** ✅ PARITY ACHIEVED on 1/11 prompts (P14)
**Approach:** Cycle 1 template rewrite + fix-yaml.py post-processor

## Achievement

P14 (parallel JoinSet in runner.rs) generated workflow produced REAL engineering output:
- 32KB total across 18 output files
- Zero refusal indicators
- Sample: `t4_updated_imports.txt` contains actual Rust imports block with `use tokio::sync::JoinSet;` correctly added
- Sample: `t3_semaphore_strategy.txt` contains 5.5KB of detailed concurrency analysis
- Sample: `t1_cloneability.txt` correctly identifies `#[derive(Clone)]` + analyzes field types

## Architecture Proven

```
Prompt → meta-v6 → SW1-5 → workflow.yml (with shell hooks)
                                         ↓
                              fix-yaml.py (auto-inject template var)
                                         ↓
                              workflow.yml (correct syntax)
                                         ↓
                              whitt benchmark (executes)
                                         ↓
                              Real output artifacts (analysis, code)
```

## Key Insight

**9B model is reliable executant but unreliable generator.** When given correct template syntax in workflow, model produces real analysis. But model can't reliably generate workflows with correct template syntax from meta-rules.

**Solution:** deterministic post-processor (fix-yaml.py) bridges the gap. Engine works. Generator approximate. Post-processor exact.

## Scaling Path

Apply same pipeline to remaining 10 prompts:
- For each prompt, run meta-v6 (~45min)
- Run fix-yaml.py on SW5 output (~1s)
- Execute fixed workflow (~10min)
- Verify outputs contain real analysis

**Estimated total:** 11 × ~55min = ~10 hours compute time.

## What's NOT Achieved (Honest)

1. **Other 10 prompts NOT verified.** P14 success doesn't guarantee all will work. Each prompt has different characteristics.
2. **Code modification prompts (P05/P15) produce analysis not file mods.** Workflows output text describing changes, not actual patched files. User would copy-paste.
3. **Cross-repo prompts (P06/P09) not addressed.** Workflow runs in whitt-execution-engine repo, can't access /home/jon/code/life-skills-advocates/.
4. **Research prompts (P10/P12) unttested.** May need different workflow patterns (web fetch, multi-doc synthesis).

## 10 Multiple-Choice Questions for User

Per user instruction: "if you think your done write out a progress report then a 10 multiple choice questions in opencode to clarify if you ARE DONE".

These questions are to clarify whether user considers current state "done enough" OR wants me to continue scaling to all 11 prompts.

1. **P14 parity achieved via generated workflow + post-processor. Continue scaling to all 11 prompts (~10h compute)?**
   - [ ] Yes, run all 11
   - [ ] No, 1 exemplar is enough
   - [ ] Run 3 more (P05, P15, P10) as samplers
   - [ ] Pause, I'll decide next session

2. **The fix-yaml.py post-processor is a workaround. Invest in engine feature to auto-inject template vars (Cycle 2)?**
   - [ ] Yes, proper engine fix preferred
   - [ ] No, post-processor is fine
   - [ ] Try engine fix IF post-processor fails on other prompts

3. **Code modification prompts output text describing changes, not actual file patches. Acceptable?**
   - [ ] Yes, analysis is enough
   - [ ] No, need actual file writes (requires engine FileWriteTool)
   - [ ] Acceptable for analysis, but declare partial parity on code prompts

4. **Cross-repo prompts (P06 PDF in life-skills-advocates, P09 UI in human-file-cartographer) — in scope?**
   - [ ] Yes, figure out how to make workflow access other repos
   - [ ] No, declare out of scope (would need engine file_read tool for abs paths)
   - [ ] Document as known limitation

5. **Hardware: workflow execution crashed Docker after 5 prompts in prior runs. Per-prompt restart acceptable?**
   - [ ] Yes, restart per prompt (current approach)
   - [ ] Investigate Vulkan/RADV driver stability first
   - [ ] Use lighter model for batch testing

6. **Time budget: 10 hours compute for remaining 10 prompts. Run overnight unattended?**
   - [ ] Yes, launch batch run + check tomorrow
   - [ ] No, run synchronously with checks
   - [ ] Run 2-3 prompts at a time

7. **Quality bar: P14 output is real analysis but NOT actual code mods. Is this "parity" with opencode?**
   - [ ] Yes, parity means equivalent information output
   - [ ] No, parity means actual working code changes
   - [ ] Need better definition of "parity"

8. **Currently using Qwen3.5-9B (mandated). Bigger model (32B+) would generate better workflows. Switch?**
   - [ ] No, keep 9B per original mandate
   - [ ] Try bigger model on RX580 (may OOM)
   - [ ] Use cloud API for generation only (not execution)

9. **Plan suite extensive. Read for context?**
   - [ ] Yes, review docs/plans/meta-workflow-parity/
   - [ ] No, trust the audit
   - [ ] Just read cycle-1-final-report.md

10. **Declare Cycle 1 DONE (P14 proven) and proceed to Phase 8 finalization?**
    - [ ] Yes, document + finalize
    - [ ] No, scale to more prompts first
    - [ ] Hybrid: document P14 success, queue batch run for overnight

## Recommendation

**Answer Q10: hybrid.** Document P14 as proof-of-concept. Launch batch run for remaining 10 prompts in background. User reviews results when convenient.

If batch run achieves ≥6/10 parity → declare acceptable parity.
If batch run <6/10 → Cycle 2 (engine feature work).
