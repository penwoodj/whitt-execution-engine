# 12 — Tools and Research

## Executive Summary

This document catalogs tools, libraries, patterns, and OpenCode skills relevant to the meta-workflow generator project. Each entry includes what it does, how it helps, and setup requirements.

---

## 1. Prompt Decomposition for Small Models

### 1.1 Chain-of-Thought (CoT) Prompting

**What:** Technique where the model is prompted to reason step-by-step before producing an answer. Instead of asking for the final output directly, the prompt asks the model to show its reasoning.

**How it helps:** Small models (7B-9B) benefit disproportionately from CoT because it forces intermediate token generation that constrains the output space. For task decomposition, CoT prompts produce more structured, complete breakdowns.

**Pattern for SW1:**
```
Step 1: List all nouns and verbs in the prompt.
Step 2: Group related nouns into task candidates.
Step 3: For each candidate, estimate complexity (story points).
Step 4: For candidates ≥5 points, generate subtasks.
Step 5: Output the final task list.
```

**Setup:** No external tools needed. Implemented via prompt engineering in YAML workflow steps.

### 1.2 Few-Shot Prompting with Examples

**What:** Providing the model with 1-3 examples of input→output pairs before asking it to process the actual input.

**How it helps:** Small models need examples to understand output format expectations. Without examples, 9B models often produce inconsistent formatting, missing sections, or hallucinated structure.

**Pattern:** Include a `## EXAMPLE` section in the prompt with a sample input prompt and its expected task breakdown output.

**Setup:** Curate 2-3 high-quality examples from the baseline task breakdowns (see `baselines/baseline-14-*.md`, `baselines/baseline-15-*.md`).

### 1.3 Self-Consistency Sampling

**What:** Generate multiple outputs from the same prompt (with temperature > 0) and select the most consistent or majority-vote result.

**How it helps:** Compensates for small model unreliability. If 3 of 5 runs produce similar task breakdowns, that breakdown is likely correct.

**Pattern:** Run the same SW step 3-5 times with `temperature: 0.3`, then use a separate evaluation step to pick the best output.

**Setup:** Implement via `route_to` with multiple targets using the same model, then a GWT evaluation step to select the best.

---

## 2. Iterative Quality Improvement

### 2.1 Generate-Evaluate-Fix Loop

**What:** Three-step loop: (1) generate output, (2) evaluate against criteria, (3) fix issues. Repeat until evaluation passes or max iterations reached.

**How it helps:** Small models rarely produce correct output on the first pass. The evaluate-fix loop compensates by catching and correcting errors iteratively.

**Implementation in YAML:**
```yaml
step_03_evaluate:
  prompt: |
    Check the task breakdown against these criteria: ATOMIC, COMPLETE, GWT_READY, ...
    Output: PASS or FAIL with reasons.
  when:
    after_step_succeeds:
      - gwt:
          - given: output.contains("PASS")
            then: [step_05_assemble]
          - given: output.contains("FAIL")
            then: [step_04_fix]

step_04_fix:
  prompt: |
    The evaluation found these issues: {{bookmarks.eval_output}}
    Fix them. Output the corrected task breakdown.
  depends_on: [step_03_evaluate]
```

**Setup:** Already implemented in SW1-SW5 via GWT routing hooks.

### 2.2 Chunked Processing

**What:** Process large inputs in smaller chunks (e.g., 5 tasks at a time) rather than all at once.

**How it helps:** Small models have limited context windows and attention capacity. Processing 5 tasks produces better quality than processing 20 at once.

**Pattern:** Already implemented in SW1 as "groups of 5" chunking (GROUP A: T1-T5, GROUP B: T6-T10, etc.).

**Setup:** Configured in the SW1 step_02 prompt with explicit group boundaries.

### 2.3 Progressive Complexity Scaling

**What:** Start with small chunk sizes (3-5 items) and gradually increase as the model demonstrates competence.

**How it helps:** Prevents context overload early in the workflow. As the model produces correct output on small chunks, confidence increases to handle larger chunks.

**Pattern:** SW2 (desired output state) starts with 3-task chunks, increases to 5, then 7.

---

## 3. YAML Workflow Generation Validation

### 3.1 YAML Linting via Shell Hooks

**What:** Use `python3 -c "import yaml; yaml.safe_load(open('file.yml'))"` or `yq` to validate YAML syntax.

**How it helps:** Catches YAML syntax errors (indentation, quoting, type coercion) before the Rust parser rejects them.

**Implementation:**
```yaml
when:
  after_step_succeeds:
    - shell:
        command: "python3 -c \"import yaml; yaml.safe_load(open('{{bookmarks.output_file}}'))\""
        fail_on_error: true
```

**Setup:** Python3 with PyYAML, or `yq` (Go-based YAML processor).

### 3.2 Schema Validation

**What:** Validate generated YAML against the unified-workflow-schema.yml using a schema validator.

**How it helps:** Catches semantic errors (unknown fields, wrong types, missing required fields) that syntax validation misses.

**Implementation:** Custom Rust validation via `WorkflowFile::validate()` — already exists in the engine.

### 3.3 Round-Trip Serialization Test

**What:** Parse YAML → serialize → parse again → compare. If round-trip fails, the YAML has structural issues.

**How it helps:** Detects non-idempotent YAML constructs that confuse the model.

---

## 4. Task Complexity Scoring

### 4.1 Story Point Estimation

**What:** Assign relative complexity points (1, 2, 3, 5, 8, 13) based on effort estimation.

**How it helps:** Provides a quantitative basis for deciding which tasks need subtask decomposition (≥5 points = decompose).

**Reference Scale (mid-level engineering team):**
| Points | Effort | Description |
|--------|--------|-------------|
| 1 | < 4 hours | Trivial change, single file |
| 2 | 4-8 hours | Simple feature, 1-2 files |
| 3 | 1-2 days | Standard task, multi-file |
| 5 | 2-3 days | Complex task, needs decomposition |
| 8 | 3-5 days | Major feature, architectural impact |
| 13 | 1-2 weeks | Epic-level, split into multiple stories |

### 4.2 Cyclomatic Complexity Adaptation

**What:** Adapt software metric (McCabe's cyclomatic complexity) to task complexity by counting decision points in the task description.

**How it helps:** Objective measure of task branching complexity. Tasks with many "if/then" conditions are inherently more complex.

### 4.3 Subtask Count Heuristic

**What:** The number of subtasks required to decompose a task IS its complexity score.

**How it helps:** Self-referential metric — if a task needs 5 subtasks, it's a 5-point task by definition.

---

## 5. Local Model Benchmarking

### 5.1 llama.cpp Benchmark Mode

**What:** Built-in `llama-bench` tool that measures tokens/second for prompt processing and generation.

**How it helps:** Establishes baseline performance for Qwen 3.5-9b on the target hardware.

**Command:**
```bash
docker exec whitt-llama-server llama-bench -m /models/Qwen3.5-9B.gguf -p 512 -n 128
```

### 5.2 Custom Benchmark via Whitt Engine

**What:** Use the Whitt benchmark runner itself to measure end-to-end workflow performance.

**How it helps:** Captures real-world performance including model loading, inference, hook execution, and file I/O.

**Command:**
```bash
cargo run --release -- benchmark --workflow docs/benchmarks/workflows/benchmark-1-model-iterative.yml
```

### 5.3 Token Efficiency Metrics

**What:** Track tokens generated vs. tokens useful in final output.

**How it helps:** Identifies prompt inefficiencies. If 80% of generated tokens are discarded during fix loops, the prompt needs improvement.

---

## 6. Relevant Rust Crates

### 6.1 `serde_yaml` / `serde_saphyr`

**What:** YAML serialization/deserialization for Rust.

**How it helps:** Core dependency for parsing workflow YAML files. `serde_saphyr` is the newer fork used by this project.

### 6.2 `tokio`

**What:** Async runtime for Rust.

**How it helps:** Powers parallel inference (tokio::JoinSet for concurrent HTTP calls), semaphore-based concurrency control, and async file I/O for hooks.

### 6.3 `reqwest`

**What:** HTTP client for Rust.

**How it helps:** Sends chat completion requests to llama.cpp server, handles SSE streaming for token-by-token output.

### 6.4 `garde`

**What:** Validation crate for Rust structs.

**How it helps:** Validates ModelSpec fields (model name format, port range, etc.) at deserialization time.

---

## 7. OpenCode Skills Relevant to This Project

### 7.1 `/test-driven-development`

**When:** Writing unit tests AFTER live system validation confirms behavior.

**How:** Provides TDD workflow guidance. In this project, we write tests only after confirming live behavior — TDD skill helps structure these post-hoc tests.

### 7.2 `/systematic-debugging`

**When:** Debugging SW output quality issues or engine crashes.

**How:** Provides structured debugging methodology (reproduce → isolate → hypothesize → test → fix).

### 7.3 `/verification-before-completion`

**When:** Before claiming any SW iteration is "done."

**How:** Enforces running verification commands and confirming output before making success claims.

### 7.4 `/commit`

**When:** Committing changes.

**How:** Ensures conventional commit format with proper scope and body.

### 7.5 `/brainstorming`

**When:** Designing new sub-workflow steps or evaluation criteria.

**How:** Explores requirements before implementation. Useful for designing SW2-SW5 evaluation logic.

### 7.6 `/writing-plans`

**When:** Creating or updating plan files.

**How:** Structures multi-step plans with clear deliverables and verification steps.

---

## 8. External Tools to Consider

### 8.1 `yq` (Mike Farah's Go YAML processor)

**What:** Command-line YAML processor (like `jq` for YAML).

**Install:** `wget -qO /usr/local/bin/yq https://github.com/mikefarah/yq/releases/latest/download/yq_linux_amd64 && chmod +x /usr/local/bin/yq`

**Use Case:** Shell hooks that validate or transform generated YAML:
```bash
yq '.agentic_workflow.steps | keys' workflow.yml
```

### 8.2 `jq`

**What:** Command-line JSON processor.

**Use Case:** Processing benchmark logs that are in JSON format, extracting metrics for quality tracking.

### 8.3 `entr`

**What:** Run arbitrary commands when files change.

**Use Case:** Auto-running SW validation when the output .md file changes during iteration:
```bash
echo "output/tasks.md" | entr -c python3 validate_tasks.py /_
```

### 8.4 `watchexec`

**What:** Modern file watcher that runs commands on change.

**Install:** `cargo install watchexec-cli`

**Use Case:** Similar to `entr` but more featureful. Auto-re-run SW evaluation on file save.

---

## 9. Patterns for Small Model Reliability

### 9.1 Output Format Constraints

**Pattern:** Constrain output to a rigid format (e.g., numbered list with specific headers) to reduce hallucination.

**Example:** "Output MUST follow this format: `T{n}: {title} [STORY: {1|2|3|5|8|13}]` — no other format accepted."

### 9.2 Negative Prompting

**Pattern:** Explicitly state what NOT to do.

**Example:** "Do NOT include code blocks. Do NOT add preamble. Do NOT use bullet points — use numbered list only."

### 9.3 Temperature Management

**Pattern:** Use temperature 0.0-0.2 for structured output (task lists, YAML), 0.3-0.5 for evaluation, 0.7+ for creative generation.

**Implementation:** Per-step `model_overrides.temperature` in YAML workflow.

### 9.4 Max Tokens Budgeting

**Pattern:** Set `max_tokens` to 1.5× expected output length. Too low truncates; too high wastes compute and risks hallucination.

### 9.5 System Prompt Anchoring

**Pattern:** Use a system prompt that establishes role and constraints, then repeat key constraints in the user prompt.

**Example:** System: "You are a task decomposition expert. Output only numbered tasks." User: "Decompose this prompt. Remember: numbered tasks only."

---

## 10. Research References

### 10.1 Qwen Model Family
- **Qwen 3.5 Technical Report:** https://qwenlm.github.io/blog/
- **Quantization Guide:** Q4_K_M balances quality/speed for CPU inference
- **Context Window:** 262144 tokens native, but effective attention degrades past 32k without flash attention

### 10.2 llama.cpp Configuration
- **Vulkan Backend:** Use `--flash-attn on` (safe), avoid `--cont-batching` (KV cache serialization)
- **Q8_0 KV Cache:** Reduces memory 50% vs F16, minimal quality loss
- **CPU Threads:** Match physical core count, not logical (hyperthreading hurts)

### 10.3 Small Model Limitations
- **Context Degradation:** Models <10B lose coherence past 8k tokens of input
- **Instruction Following:** 9B models need explicit, short instructions — implicit expectations fail
- **Format Consistency:** Without few-shot examples, 9B models produce inconsistent output formats across runs

---

## Summary Table

| Category | Tool/Pattern | Status | Priority |
|----------|-------------|--------|----------|
| Decomposition | Chain-of-Thought | ✅ Implemented in SW1 | Done |
| Decomposition | Few-Shot Examples | 🔄 Add to SW1 prompt | High |
| Quality Loop | Generate-Evaluate-Fix | ✅ Implemented in SW1-SW5 | Done |
| Quality Loop | Chunked Processing | ✅ Groups of 5 in SW1 | Done |
| Validation | YAML Linting | 🔄 Add shell hook | Medium |
| Validation | Schema Validation | ✅ WorkflowFile::validate() | Done |
| Benchmarking | llama-bench | 🔄 Document command | Low |
| Benchmarking | Token Efficiency | 🔄 Add metric logging | Medium |
| Small Model | Temperature Management | ✅ Per-step overrides | Done |
| Small Model | Negative Prompting | 🔄 Add to prompts | Medium |
