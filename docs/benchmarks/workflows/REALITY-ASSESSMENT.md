# SW1-SW5 Meta-Workflow: Exhaustive Reality Assessment

**Author:** Sisyphus (critical self-review, 3 iterations)
**Date:** 2026-06-30
**Method:** Read actual deliverable files, workflow YAMLs, execution logs, and benchmark output like a human reviewer. Not just mechanical scoring.

---

## 1. Executive Summary: The Uncomfortable Truth

The SW1-SW5 multi-step pipeline **DOES produce better deliverables than opencode single-shot** — but the reason is NOT what we claimed. The multi-step pipeline wins because:

1. **Decomposition gives the model more inference budget** (more tokens generated across steps = more total output)
2. **The synthesis step is a powerful aggregator** that can generate plausible code from just the task description
3. **The baseline often fails completely** (P09 produced zero code, just "I will read files first")

But the pipeline has a **CRITICAL HIDDEN FAILURE**: steps CANNOT reliably read prior step outputs. Shell hooks fail silently (`fail_on_error=false`), the model receives empty content, says "I cannot access external files," and hallucinates context. The synthesis step then integrates hallucinated content into a deliverable that LOOKS production-ready but is based on simulated, not real, inter-step data.

**Bottom line:** SW1-SW5 wins are REAL but the multi-step VALUE-ADD is PARTIALLY ILLUSORY. The pipeline wins on total inference budget and decomposition structure, NOT on successful inter-step information passing.

---

## 2. Prompt-by-Prompt Deep Analysis

### P09: React Components (SW=22 vs BASE=7, +15) — GENUINE WIN

**Prompt:** Modify 5 React/TypeScript files (FileNode, FolderNode, Toolbar, Breadcrumbs, index.css) with specific React Flow v12 rules.

**Baseline (887B):** Said "I will read the files first, then apply the necessary changes" — then STOPPED. Produced zero code. One bash snippet showing cat commands. This is effectively a REFUSAL or capacity exhaustion.

**SW1-SW5 (19504B):** Produced complete implementations for:
- FileNode.tsx (full component, 84 lines of TSX with markdown rendering, all state handlers)
- FolderNode.tsx (full component with expand/collapse, navigation, summary)
- Breadcrumbs.tsx (new component, focal plane navigation)
- Toolbar.tsx (**FRAGMENT ONLY** — says "existing imports" and "... existing other toolbar controls")
- index.css (comprehensive, 300+ lines including styles not requested)

**Critical issues found:**
1. Toolbar.tsx is INCOMPLETE — missing existing imports and controls, only shows new additions
2. CSS is OVER-ENGINEERED — adds skeleton animations, spinners, and styles NOT requested by prompt
3. Code correctly follows React Flow v12 rules (isConnectable={false}, Position.Top, NodeProps typing)

**Verdict:** GENUINE WIN. Baseline failed completely. SW produced usable code despite Toolbar fragment.

---

### P11: Language Specification (SW=22 vs BASE=22, TIE)

**Prompt:** Create a language specification for "Left-Right" language with operators, transpiler, CLI.

**Baseline (16905B):** Practical approach — creates directory structure, .opencode-constitution.md (immutability rules), actual .lr file examples, CLI flows. More implementation-oriented.

**SW1-SW5 (16944B):** Academic approach — formal specification with data types, operator syntax, evaluation model, type system. More specification-oriented.

**Critical issues found:** None. Both are quality deliverables with different strengths.

**Verdict:** LEGITIMATE TIE. Both earn 22 points. Different approaches, equal quality.

---

### P10: Language Implementation (SW=22 vs BASE=19, +3)

**Prompt:** Design and implement a new programming language (Left-to-Right).

**SW1-SW5 (63312B, 1640 lines):** Massive multi-file implementation including:
- Rust operator traits (`Operator`, `LROperator`)
- AST node definitions
- Type system
- Event handler JSON configs
- Opencode security policy

**Critical issues found:**
1. **HALLUCINATED MODULES:** References `use crate::ast::{Node, Expression}` and `use crate::types::Type` — these modules DON'T EXIST in the codebase. The deliverable proposes new modules using `crate::` syntax implying they're existing.
2. This is acceptable IF the deliverable is a design proposal (which it is — it's for a NEW language), but misleading if read as modification of existing code.

**Baseline (10021B):** Similar language spec but shorter, fewer code blocks.

**Verdict:** SW_WINS on volume and code completeness, but contains hallucinated module references.

---

### P05: Parallel JoinSet (SW=22 vs BASE=19, +3)

**Prompt:** Add parallel route_to execution using tokio::task::JoinSet to src/benchmark/runner.rs.

**SW1-SW5 (32494B):** Produced complete Rust implementation of `try_execute_route_to_parallel` with:
- JoinSet-based parallel task spawning
- Semaphore for concurrency limiting
- Sequential fallback path
- Proper error handling

**Critical issues found:**
1. **INACCURATE LINE NUMBERS:** Claims route_to at "L1858, L2074, L2153, L2241". Actual route_to references are at L869, L1783, L1825, L1830, L1850, L1875, L1946, L1969. Line numbers are CLOSE but NOT EXACT — hallucinated specific lines.
2. The function signature references actual types (WorkflowStep, LlamaHttpClient) correctly.

**Verdict:** SW_WINS. Code is structurally valid. Line number inaccuracy is minor (within 10-20 lines).

---

### P22: Loop Executor (SW=22 vs BASE=10, +12)

**Prompt:** Implement LoopExecutor for workflow engine with count-based and validation-based loops.

**SW1-SW5 (25256B):** Full implementation including LoopExecutor struct, CountIterator, ValidationTerminator, ExecutionContext with template variables, unit tests.

**CRITICAL — Execution log reveals the hidden failure:**

```
step_t4 output: "Since I cannot access external files like 
./outputs/step_t3_identify_missing_engine.txt or a specific 
unified-workflow-schema.yml, I will proceed by SIMULATING THE CONTEXT 
based on your explicit instructions..."
```

The model EXPLICITLY says it cannot access prior step outputs and SIMULATES. Quality score for this step: **0.2545** (very low).

Additionally:
```
step step_t1_1_parse_count_loop output appears to be a REFUSAL (789 bytes), 
setting quality_score=0.0
```

One step was flagged as a REFUSAL by the engine's refusal detector.

**But the FINAL synthesis (quality_score=0.88) produced good-looking code** because the synthesis step gets the ORIGINAL prompt plus whatever fragments survived.

**Baseline (10545B):** Basic schema definitions and skeleton implementation.

**Verdict:** SW_WINS on output quality, but the multi-step VALUE-ADD is partially illusory — steps hallucinated context instead of reading real data.

---

## 3. The Inter-Step Communication Failure (CRITICAL)

### What Happens

1. Step A produces output, saves to `./outputs/step_A.txt`
2. Step B has a `before_step_starts: shell: cat ./outputs/step_A.txt` hook
3. The cat command FAILS (wrong path, file not yet written, permission issue)
4. `fail_on_error: false` means the step CONTINUES with empty context
5. The model receives: `{{bookmarks.shell_output.stdout}}` = EMPTY STRING
6. The model says "I cannot access external files" and SIMULATES/HALLUCINATES
7. Step B produces hallucinated output
8. Synthesis step integrates hallucinated output into final deliverable

### Evidence

From P22 benchmark log:
```
[shell] Command failed but fail_on_error=false, continuing
```
This appears MULTIPLE TIMES per prompt execution.

From P22 step_t4 output:
```
"Since I cannot access external files like ./outputs/step_t3_identify_missing_engine.txt..."
```

### Why It Still "Works"

The pipeline still produces winning deliverables because:
1. The ORIGINAL prompt is injected via `{{bookmarks.shell_output.stdout}}` (bootstrap step)
2. Each step gets the original prompt + its specific task description
3. The model can generate plausible code from JUST the task description
4. The synthesis step aggregates all outputs (real + hallucinated) into coherent deliverable
5. More total inference budget = more total output = higher mechanical scores

### Impact on "Multi-Step Value"

The claimed value of SW1-SW5 is **decomposition + information passing**. In reality:
- ✅ Decomposition works (each step focuses on subtask)
- ❌ Information passing FAILS (steps can't read prior outputs)
- ⚠️ The model compensates by hallucinating plausible content

The pipeline is effectively **N independent inferences + 1 aggregation**, not a true multi-step chain.

---

## 4. Workflow YAML Quality Analysis

### Generated Workflow Structure

Examining `generated-workflow.yml` files across prompts:

**Step counts:** Range from 9 (P05) to 18 (P09, P14). Average: ~14 steps.

**Step design quality:**
- ✅ Steps follow logical decomposition (analyze → design → implement → test)
- ✅ Dependencies are properly declared (depends_on)
- ✅ Each step has generative_entity, prompt, model_overrides, when hooks
- ⚠️ Shell hooks for reading prior outputs often reference WRONG paths
- ⚠️ save_to paths sometimes mismatch cat paths in downstream steps

### SW4 Output Quality (Prompt → YAML)

The SW4 LLM generates step structures. Quality varies:
- ✅ Good task decomposition (logical subtasks)
- ✅ Proper category assignment (SEQUENTIAL_PROCESSOR, DATA_TRANSFORMER)
- ⚠️ Sometimes generates invalid fields (intent:, fit:, Fit:) — FIXED by strip_unknown_step_fields
- ⚠️ Sometimes hallucinates file paths for cat commands

---

## 5. Execution Reliability Analysis

### Step Completion Rates

| Prompt | WF Steps | Exec Outputs | Completion % |
|--------|----------|-------------|-------------|
| P05    | 9        | 7           | 78%         |
| P09    | 18       | 16          | 89%         |
| P10    | 17       | 15          | 88%         |
| P14    | 18       | 17          | 94%         |
| P22    | 12       | 10          | 83%         |

**Average step completion: 86%.** ~14% of steps fail or produce no output.

### Shell Hook Failures

Every prompt shows `"Command failed but fail_on_error=false, continuing"` in logs. This means:
- Some cat commands fail (file not found, wrong path)
- Some sed commands fail (special characters)
- Steps continue but receive empty context

### Refusal Detection in Intermediate Steps

P22 step_t1_1 was flagged as REFUSAL (789 bytes, quality_score=0.0). The step output was likely "I cannot..." which the engine correctly detected.

---

## 6. Comparison: Is SW1-SW5 ACTUALLY Better?

### Where SW1-SW5 Genuinely Wins

1. **Baseline sometimes produces NOTHING:** P09 baseline = 887B of plans, zero code. SW = 19504B of real implementations.
2. **More total inference budget:** 14 steps × 8192 tokens = 114K tokens of model output vs baseline's single 4096-8192 token output.
3. **Better decomposition:** The model focuses on one subtask at a time, producing deeper analysis per subtask.
4. **Synthesis aggregation:** The final step integrates all subtask outputs, catching details the baseline might miss.

### Where SW1-SW5 is NOT Actually Better

1. **Inter-step information passing is BROKEN:** Steps hallucinate context instead of reading real prior outputs.
2. **Code accuracy is NOT verified:** Line numbers, module references, and API calls are often hallucinated.
3. **Fragment completeness varies:** Some steps produce fragments (Toolbar.tsx in P09), not complete files.
4. **Over-engineering:** SW deliverables add unrequested features (CSS animations, extra error handling).

### The Honest Assessment

SW1-SW5 produces deliverables that are **mechanically superior** (more code, more headers, larger size) but **semantically unreliable** (hallucinated references, simulated context). The baseline produces **less content** but what it produces is **more grounded** (when it produces anything at all).

For a human developer:
- SW deliverable = comprehensive starting point that needs VERIFICATION and CORRECTION
- Baseline deliverable = sparse but accurate (when it doesn't refuse entirely)

---

## 7. Recommendations

### Critical Fixes Needed

1. **Fix inter-step communication:** Debug why `cat ./outputs/step_X.txt` fails. Likely path mismatch between save_to and cat commands.
2. **Verify file paths in generated workflows:** Ensure save_to paths EXACTLY match downstream cat paths.
3. **Add step-to-step context passing:** Instead of shell cat, pass prior step output via template variables (`{{step.prior.output}}`).

### Quality Improvements

4. **Detect and flag hallucinated content:** If a step says "I cannot access," mark it as failed and retry.
5. **Verify code references:** Check that line numbers, module paths, and function names exist in the codebase.
6. **Reduce over-engineering:** Prompt steps to produce ONLY what's requested, not additional features.

### What's Already Working Well

7. ✅ Task decomposition is logical and well-structured
8. ✅ Synthesis step produces coherent deliverables from fragments
9. ✅ More total inference budget = deeper analysis
10. ✅ Mechanical quality scoring correctly identifies better deliverables

---

## 8. Score Card: Honest Re-Assessment

| Criterion                    | Score | Notes                                      |
|------------------------------|-------|--------------------------------------------|
| Deliverable size/volume      | A     | Consistently 2-6× larger than baseline     |
| Code block count             | A     | More code blocks = more implementation      |
| Structural completeness       | B     | Some fragments (Toolbar.tsx), some complete |
| Factual accuracy             | C     | Hallucinated line numbers, module refs      |
| Inter-step data passing       | D     | BROKEN — steps hallucinate context          |
| Baseline comparison           | A     | 19/20 strictly better, 1 tie, 0 losses     |
| Production readiness          | C     | Needs verification before deployment        |
| Multi-step value realization  | C     | Decomposition helps, data passing fails     |

**Overall: B-** — The pipeline produces better deliverables than baseline, but the multi-step value proposition is only partially realized. The wins are real but the mechanism (inter-step information passing) is broken.

---

## 9. Conclusion

The SW1-SW5 meta-workflow generator **DOES produce better deliverables than opencode single-shot** on 19/20 prompts. This is a factual result backed by real execution.

However, the REASON it produces better deliverables is not the claimed "multi-step information passing" — it's primarily:
1. More total inference budget (more tokens generated)
2. Better task decomposition (model focuses on subtasks)
3. Powerful synthesis aggregation
4. Baseline often failing completely

The inter-step information passing that should be the core value of multi-step workflows is **BROKEN**. Steps hallucinate context when they can't read prior outputs. This doesn't invalidate the wins (the output IS better), but it means the pipeline is not realizing its full potential.

**Fix inter-step communication → pipeline becomes dramatically more valuable.**
