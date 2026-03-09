# Common Routing Logic for All Systems

This document outlines the foundational routing logic that applies to all three system designs (TinyRouter, LearnedRouter, AdaptiveRouter).

## 1. What CSV Should Drive

Each CSV row becomes a routing record containing:

### Metadata
- `specialization_category` - Primary classification
- `specialization_synonyms` - Alternative names
- `related_synonym_tags` - Searchable tags

### Model Recommendations
- `top1_model` + `top1_ctx` - Best model + context window
- `top2_model` + `top2_ctx` - Second best
- `top3_model` + `top3_ctx` - Third best
- `wildcard_model` + `wildcard_ctx` - Fallback model

### Strength & Context
- `primary_strength` - What this model does best
- `secondary_strengths` - Additional capabilities
- `preferred_input_shape` - Input format hints
- `preferred_output_shape` - Output format hints
- `context_need` - Low/medium/high
- `output_rigidity` - Strict vs flexible output

### Verification
- `verification_need` - How strict validation should be
- `router_keywords` - Direct match terms
- `avoid_when` - Conditions to skip this row
- `selection_notes` - Human-readable guidance

## 2. What Selector Does

For each new task, the selector executes this pipeline:

```
Input Task
    │
    ▼
[Infer Task Traits]
    • language (Python, JavaScript, etc.)
    • output type (JSON, YAML, code, prose)
    • edit vs scratch
    • context estimate
    • verification severity
    │
    ▼
[Retrieve CSV Rows]
    • Dense similarity (vector search)
    • Sparse keyword match (BM25/FTS)
    • Constraint filtering
    │
    ▼
[Expand Rows to Models]
    • Each row → 4 model candidates
    • top1, top2, top3, wildcard
    │
    ▼
[Rank Candidates]
    • Row relevance score
    • Slot bonus (top1=1.0, wildcard=0.55)
    • Popularity prior
    • Hardware fit
    • Log success score
    │
    ▼
[Try / Verify / Retry / Switch]
    • Execute with N models
    • Each model gets k tries
    • Verify after each try
    • Switch on failure
    │
    ▼
[Log Everything]
    • Task traits
    • Model used
    • Attempts
    • Success/fail
    • Validation results
    │
    ▼
[Feed Logs Back]
    • Update future rankings
    • Adjust penalties
    • Improve scores
```

## 3. Core Ranking Idea

### A. Row Relevance

**Dense Similarity**
- Compute cosine similarity between input embedding and row embedding
- Row embedding built from: category, synonyms, tags, strengths, keywords

**Sparse Keyword Match**
- BM25 score from FTS
- Match on: category, synonyms, tags, keywords

**Constraint Match**
- Output type compatibility (e.g., strict JSON task → JSON-capable rows)
- Language match (e.g., Python code → Python-capable rows)
- Edit vs scratch alignment
- Context window sufficiency
- Verification severity fit
- Avoid_when exclusions

**Row Score Formula**
```
row_score =
  0.45 * dense_similarity +
  0.25 * sparse_score +
  0.10 * output_type_match +
  0.10 * language_match +
  0.10 * context_fit
```

### B. Model Score

#### Initial Formula (Early, with limited logs)
```
final_model_score =
  0.35 * row_relevance +
  0.10 * slot_bonus +
  0.10 * popularity_prior +
  0.10 * hardware_fit +
  0.35 * log_success_score
```

#### Mature Formula (Later, with rich logs)
```
final_model_score =
  0.25 * row_relevance +
  0.05 * slot_bonus +
  0.05 * popularity_prior +
  0.10 * hardware_fit +
  0.55 * log_success_score
```

**Key progression**: As logs accumulate, log_success_score weight increases from 35% to 55%.

### Slot Bonus Values

```
top1     = 1.00
top2     = 0.82
top3     = 0.68
wildcard  = 0.55
```

If same model appears from multiple rows, merge by max row score + small repeat bonus.

## 4. Log Score Design

### What to Log

For each attempt, store:
- Input embedding (for similarity matching)
- Winning specialization row(s)
- Chosen model
- Prompt length bucket
- Task category
- Language
- Output type
- Verification type
- Success/fail
- Fail reason
- Retries used
- Latency
- Tokens consumed
- Parse pass / compile pass / test pass / human pass

### Log Success Score Formula

```
log_success_score =
  0.50 * verified_success_rate_on_similar_tasks +
  0.20 * first_try_success_rate +
  0.15 * low_retry_rate +
  0.10 * format_or_compile_pass_rate +
  0.05 * latency_score
```

### Similarity-Aware Matching

**Not**: Global model statistics

**Instead**: Statistics on tasks similar to current task

How to find similar tasks:
- Nearest task embeddings
- Same output type
- Same language
- Same verifier type
- Same edit/scratch mode

### Bayesian Smoothing

To prevent a model with 2/2 wins from outranking one with 200/250 wins unfairly:

```
smoothed_success =
  (successes + alpha * global_success_rate) /
  (attempts + alpha)
```

Use `alpha = 10` to start. Adjust based on confidence in global rate.

## 5. Retry / Switch Loop

### Per-Task Logic

```
for model in ranked_models[:N]:
    for try_idx in 1..max_tries_per_model:
        result = run_model(model, prompt)

        verdict = verify(result, verifier_chain)
        log_attempt(...)

        if verdict.pass:
            return success

        if verdict.recoverable:
            prompt = repair_prompt(task, result, verdict.errors)
            continue  # retry same model
        else:
            break  # switch to next model

return failure
```

### Specialization-Specific Retry Budgets

Different tasks deserve different retry budgets:

| Task Type | Max Tries | Rationale |
|-----------|-----------|------------|
| Strict JSON | 2 | Format tasks fail fast |
| Code generation | 3 | Can benefit from repair cycles |
| Summarization | 2 | Rarely improves after retries |
| Debugging | 4 | Benefits from multiple repair attempts |

## 6. Validator Chain

Verifiers are **specialization-driven**, not model-driven.

### JSON/YAML Verifier
```
1. Parse with strict parser
2. Schema validate (if schema provided)
3. Required fields present
4. No disallowed fields
5. Correct data types
```

### Shell / SQL / GraphQL Verifier
```
1. Syntax check (parser)
2. Dry-run or EXPLAIN (if available)
3. Placeholders accounted for
4. No dangerous patterns (rm -rf, DROP TABLE)
```

### Code Verifier
```
1. Compile / lint / typecheck
2. Run unit tests
3. Static analysis checks
4. No obvious bugs
5. Meets style requirements
```

### Summary Verifier
```
1. Length within bounds
2. Section coverage (intro, body, conclusion)
3. Required entities present
4. Not too verbose
5. Not too brief
```

### Classification Verifier
```
1. Label is in allowed set
2. Confidence score (if available)
3. No contradictory labels
4. Format correct (JSON, plain text)
```

### Plan Verifier
```
1. Required sections present
2. No missing dependencies
3. Steps are actionable
4. No circular dependencies
5. Optional: second-pass checker model
```

## 7. Hardware Fit Scoring

Use local measurements to compute hardware compatibility:

### Metrics to Track
- Average load time per model
- Average inference time
- OOM (out of memory) count
- Context overflow count
- Disk usage

### Penalty Examples
| Condition | Penalty | Reason |
|-----------|----------|---------|
| OOM | Huge (0.9) | Model cannot run |
| Context truncation | Medium (0.5) | Model needs more context |
| Slow but successful | Small (0.1) | Acceptable but annoying |
| Fast and reliable | None (0.0) | Ideal |

## 8. Two-Layer Ranking Philosophy

### Layer 1: Specialization Row

**Question**: "What kind of task is this?"

**Output**: Ranked list of relevant CSV rows

**Inputs**: Task traits, hybrid search (dense + sparse), constraints

### Layer 2: Model Candidate

**Question**: "Among models suggested by winning rows, which one is best on this machine, for this verifier, on tasks like this, with recent logs?"

**Output**: Ranked list of model candidates with execution scores

**Inputs**: Winning rows, model catalog, hardware stats, log statistics

**Why This Matters**

- Separates concerns (routing vs ranking)
- Enables row-level learning independent of model-level learning
- Allows different strategies for each layer
- Makes system actually improve over time instead of becoming a static tag matcher

## 9. Failure-Mode Memory

Advanced systems should track not just success, but **how models fail**:

### Failure Embedding

When a model fails:
1. Embed the failure explanation
2. Store with task and model
3. Later, penalize models that fail similarly for similar tasks

### Example Failure Patterns

| Pattern | Example | Implication |
|---------|----------|-------------|
| "keeps returning markdown fences around JSON" | Parser fail | Penalize for JSON tasks |
| "forgets required field X" | Schema fail | Penalize for structured output |
| "uses non-existent API" | Compile fail | Penalize for that language/framework |
| "compiles but fails tests" | Test fail | Penalize for code-gen tasks |

This gives failure-mode memory, not just success memory.
