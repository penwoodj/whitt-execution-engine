# ChatGPT Brainstorming: Model Router for Transpiler

## Overview

This document captures ChatGPT's approach to building a practical model router system, focusing on the transpiler's needs for local, memory-efficient model selection with retry logic and log-driven learning.

## Core Philosophy Differences

### OpenCode Approach (This Session)

- **Three alternative designs** with different tech stacks
- **Equal complexity levels** (Python, Rust, Node.js)
- **Focus on complete implementations** for each stack
- **Vector similarity only** retrieval
- **Basic log penalty scoring**

### ChatGPT Approach

- **Evolutionary progression** (3 phases, not alternatives)
- **All Python stack** for consistency
- **Hybrid retrieval** (dense + sparse + keyword + filters)
- **Similarity-aware Bayesian smoothing** for logs
- **Failure-mode memory** beyond just success tracking
- **Two-layer routing** (row selection + model ranking)

## Key Innovations from ChatGPT

### 1. Hybrid Retrieval Pipeline

**What It Is**:
- Stage 1: Dense vector search (semantic)
- Stage 2: Sparse keyword search (BM25/FTS)
- Stage 3: Hard constraint filtering
- Stage 4: Weighted fusion

**Why It Matters**:
- Dense search catches semantic meaning ("generate parser" ≈ "create struct")
- Sparse search catches exact keywords ("derive", "Deserialize")
- Constraints eliminate impossible matches (context too small, wrong output type)
- Fusion gives best of both worlds

**Data from Research**:
- BEIR benchmark: Hybrid outperforms dense by 8.5% and sparse by 6.9%
- Production RAG systems consistently show 15-30% improvement with hybrid
- Dense-only fails on exact technical terms
- Sparse-only fails on semantic variations

### 2. Similarity-Aware Log Statistics

**What It Is**:
Instead of global model statistics, compute statistics on **tasks similar to current task**:

```python
# Not this (global):
model_success_rate = total_successes / total_attempts

# But this (similarity-aware):
similar_tasks = find_knn(task_embedding, k=50)
model_success_rate = sum(t.success for t in similar_tasks if t.model == model) / len(similar_tasks)
```

**Why It Matters**:
- A model good at "code generation" may be bad at "YAML parsing"
- A model good at "Python" may be bad at "Rust"
- Similarity matching ensures apples-to-apples comparisons
- More accurate routing for heterogeneous task types

**Implementation**:
- Store task embeddings in logs
- KNN search on log embeddings at query time
- Compute per-model stats on similar subset
- Use cluster IDs for faster similarity search (batch clustering offline)

### 3. Bayesian Smoothing

**What It Is**:
Prevent a model with 2/2 wins from outranking one with 200/250 wins unfairly:

```python
# Without smoothing:
success_rate = 2 / 2 = 1.0  # Overconfident

# With smoothing (alpha=10):
global_rate = 0.70  # Overall baseline
smoothed_rate = (2 + 10*0.70) / (2 + 10) = 9/12 = 0.75  # More reasonable
```

**Why It Matters**:
- Small sample sizes are unreliable
- Bayesian approach combines prior (global rate) with evidence
- Alpha parameter controls prior influence
- Prevents overfitting to recent small samples

**Alpha Selection**:
- α = 5: Strong prior influence (conservative)
- α = 10: Moderate (default, recommended by ChatGPT)
- α = 20: Weak prior influence (more aggressive)

### 4. Failure-Mode Memory

**What It Is**:
Track not just whether models fail, but **how** they fail:

```python
failure_patterns = {
    "keeps returning markdown fences around JSON": 15 occurrences,
    "forgets required field X": 8 occurrences,
    "uses non-existent API": 3 occurrences,
    "OOM on large context": 2 occurrences
}
```

Then penalize models that fail similarly for similar tasks:

```python
# Get similar failures
similar_failures = query_failures_by_embedding(task_embedding)

# Apply penalty
penalty = sum(f.similarity for f in similar_failures if f.model == model)
```

**Why It Matters**:
- A model that consistently fails at JSON parsing shouldn't be tried for JSON tasks
- A model that always forgets required fields should be avoided for structured output
- Failure-mode awareness catches patterns that success rates miss
- Enables proactive avoidance, not reactive retrying

### 5. Two-Layer Routing Architecture

**What It Is**:

**Layer 1: Specialization Row Selection**
- Question: "What kind of task is this?"
- Output: Ranked list of relevant CSV rows
- Methods: Hybrid search + constraint filtering

**Layer 2: Model Candidate Ranking**
- Question: "Among models from winning rows, which is best?"
- Output: Ranked list of specific models to try
- Methods: Expansion + similarity-aware scoring

**Why It Matters**:
- Separates concerns (task classification vs model selection)
- Different strategies work best at each layer
- Enables targeted improvements per layer
- Prevents becoming a "fuzzy tag matcher"
- Supports learning at model level independent of row selection

### 6. Adaptive Weight Evolution

**What It Is**:
Shift the balance between static heuristics and log-driven scores over time:

**Early Phase (Few logs)**:
```python
final_score = (
    0.35 * row_relevance +
    0.10 * slot_bonus +
    0.10 * popularity_prior +
    0.10 * hardware_fit +
    0.35 * log_success_score  # Moderate weight
)
```

**Mature Phase (Many logs)**:
```python
final_score = (
    0.25 * row_relevance +      # Reduced
    0.05 * slot_bonus +         # Reduced
    0.05 * popularity_prior +    # Reduced
    0.10 * hardware_fit +
    0.55 * log_success_score   # Increased to dominate
)
```

**Why It Matters**:
- Starts with proven heuristics for cold start
- Gradually trusts data more as evidence accumulates
- Logs eventually dominate (55% weight) when confident
- Prevents overfitting to small early logs
- Matches intuition: learn from experience

### 7. Specialization-Driven Verifier Chains

**What It Is**:
Attach verification chains to **specialization rows**, not models:

```python
VERIFIER_REGISTRY = {
    "natural_language_to_correct_parsable_json": JsonVerifier(),
    "requirement_to_executable_rust_code_from_scratch": RustCodeVerifier(),
    "sql_query_generation": SqlVerifier(),
    # ... 95 total verifiers
}
```

Each verifier is multi-stage:

**JSON Verifier**:
1. Parse with strict parser
2. Schema validate (if provided)
3. Required fields present
4. No disallowed fields
5. Correct data types

**Code Verifier**:
1. Syntax parse (AST)
2. Compile check
3. Lint/typecheck
4. Run unit tests
5. No obvious bugs

**Why It Matters**:
- Different specializations have different verification needs
- "Strict JSON" requires parsing, "Code" requires compilation
- Verification failures provide rich learning signal
- Multi-stage catches issues early with minimal cost

### 8. Specialization-Specific Retry Budgets

**What It Is**:
Different specializations get different retry allocations:

| Task Type | Max Tries | Rationale |
|-----------|-----------|------------|
| Strict JSON | 2 | Format tasks fail fast, retries don't help |
| Code Generation | 3 | Can benefit from error-aware repair |
| Summarization | 2 | Rarely improves after multiple attempts |
| Debugging | 4 | Benefits from multiple repair cycles |

**Why It Matters**:
- Not all tasks benefit equally from retries
- Wastes time to retry when improvement is unlikely
- Allocates effort where it has highest ROI
- Matches human intuition: debugging needs more tries than parsing

### 9. Feature Engineering for Learned Ranking

**What It Is**:
For each `(task, row, model)` candidate, build comprehensive features:

**Dense Features**:
- Dense similarity score (0-1)
- Normalized dense score

**Sparse Features**:
- BM25 score
- Keyword match count
- Exact match on category
- Exact match on keywords

**Row Features**:
- Slot (top1=1.0, top2=0.82, top3=0.68, wildcard=0.55)
- Output rigidity match (binary)
- Context fit (0-1)
- Verification severity match (binary)

**Task Features**:
- Input length bucket (0-100, 100-1000, 1000-10000, >10000)
- Output type (one-hot encoded)
- Language (one-hot encoded)
- Edit vs scratch (binary)
- Context ratio: `estimated_needed_ctx / model_ctx`

**Log Features** (Per Model, Similarity-Aware):
- Verified success rate on similar tasks
- First-try success rate
- Average retries
- Average latency
- Compile/test/parse pass rate
- Recent decay-weighted success

**Hardware Features**:
- Average load time (normalized)
- Average inference time (normalized)
- OOM history (binary)
- Context overflow history (binary)

**Total**: ~30-40 features per candidate

**Why It Matters**:
- Rich feature set enables learned model to make nuanced decisions
- Separates different signal types (retrieval, logs, hardware)
- Supports both early (linear model) and mature (gradient boosted) stages
- Enables A/B testing of feature importance

### 10. Three-Phase Progressive Rollout

**What It Is**:
Evolve from simple to complex instead of building everything upfront:

**Phase A: TinyRouter** (2-4 weeks)
- Single-file SQLite
- Hand-tuned static weights
- No ML
- Proves concept, starts collecting logs

**Phase B: LearnedRouter** (4-8 weeks)
- DuckDB + LanceDB for analytics
- Learned ranker (logistic regression or XGBoost)
- Offline retraining
- Real log-driven ranking

**Phase C: AdaptiveRouter** (8-12 weeks)
- Qdrant for advanced retrieval
- Hybrid queries with prefetch
- Online adaptation
- Failure-mode embedding
- Scalable architecture

**Why It Matters**:
- Validates approach before heavy investment
- Starts collecting logs immediately
- Adds complexity only when needed
- Each phase builds on previous
- Reduces risk of over-engineering

## Research-Backed Decisions

### Hybrid Fusion Techniques

**Research Finding**:
Multiple fusion algorithms exist for combining dense and sparse results:

1. **Linear Interpolation** (Simplest):
   ```python
   score = α * dense_score + (1-α) * sparse_score
   ```
   - Easy to tune
   - Simple to implement
   - ChatGPT recommends for MVP

2. **Reciprocal Rank Fusion (RRF)**:
   ```python
   score = sum(1 / (k + rank_i) for i in methods)
   ```
   - Robust to score scale differences
   - Better for diverse retrieval methods
   - More complex

3. **Weighted Fusion**:
   ```python
   score = w1 * normalized_dense + w2 * normalized_sparse + w3 * other_signals
   ```
   - Flexible, incorporates additional signals
   - ChatGPT recommends: 0.45*dense + 0.25*sparse + 0.10*filters

**ChatGPT Recommendation**:
Start with weighted fusion, consider RRF for production optimization.

### Vector Database Comparison

**Research Finding**:
Different vector DBs have different trade-offs:

| DB | RAM (100 rows) | Latency | Complexity | Best For |
|-----|-----------------|----------|------------|----------|
| sqlite-vec | ~50MB | ~5ms | Low | MVP, single-file |
| LanceDB | ~50MB | ~10ms | Medium | Analytics + retrieval |
| Qdrant | ~200MB | ~50ms | High | Production, scale |

**ChatGPT Recommendation**:
Start with sqlite-vec (MVP), migrate to LanceDB for analytics, Qdrant for production scale.

### Embedding Model Selection

**Research Finding**:
Trade-offs between popular embedding models:

| Model | Dimension | RAM | Latency | Accuracy | Notes |
|--------|-----------|------|----------|----------|--------|
| all-MiniLM-L6-v2 | 384 | ~120MB, ~50ms | High | Best balance |
| Universal Sentence Encoder | 512 | ~80MB, ~80ms | Very High | Deeper understanding |
| E5-small | 384 | ~130MB, ~30ms | High | Fastest, newer |

**ChatGPT Recommendation**:
Use all-MiniLM-L6-v2 with FastEmbed (ONNX) for optimal balance.

### Quantization Strategies

**Research Finding**:
Quantization dramatically reduces memory with minimal accuracy loss:

- **FP16**: 2x memory reduction, ~1.5x faster
- **INT8**: 4x memory reduction, ~4x faster, <1% accuracy loss
- **Q4K**: 8x memory reduction, ~8x faster, ~2-3% accuracy loss

**ChatGPT Recommendation**:
Start with FP16, use INT8 for production if acceptable.

## ChatGPT's Architecture Patterns

### Phase A: TinyRouter Pattern

```text
[Single File DB]
  ├── router_rows (CSV data)
  ├── embeddings (vec0 virtual table)
  ├── model_stats (aggregated statistics)
  └── attempt_logs (execution history)

[Retrieval]
  ├── FastEmbed → input embedding
  ├── sqlite-vec KNN → top K rows
  ├── FTS5 BM25 → top K rows
  └── Weighted fusion → ranked rows

[Ranking]
  ├── Expand rows to models (top1, top2, top3, wildcard)
  ├── Apply slot bonuses
  ├── Compute static scores (35% relevance + 10% slot + 10% popularity + 10% hardware + 35% logs)
  └── Return ranked models

[Execution]
  ├── For each model (up to N):
  │   ├── Try up to K attempts
  │   ├── Verify with specialization chain
  │   ├── Log attempt
  │   └── Switch on failure
  └── Stop on success or exhaustion
```

**Key Characteristics**:
- One SQLite file for everything
- Hand-tuned weights initially
- No ML training needed
- Fastest to implement

### Phase B: LearnedRouter Pattern

```text
[Two Storage Systems]
  ├── DuckDB (Analytics Warehouse)
  │   ├── attempt_logs (raw)
  │   ├── stats_model_spec (aggregated)
  │   ├── stats_model_output (aggregated)
  │   └── stats_cluster (similarity buckets)
  └── LanceDB (Vector Store)
      ├── specializations table
      └── Custom reranker

[Retrieval]
  ├── FastEmbed → input embedding
  ├── LanceDB search → top semantic rows
  ├── DuckDB FTS → top lexical rows
  └── Learned reranker → re-ranked rows

[Ranking]
  ├── Extract features (30-40 per candidate)
  ├── Logistic regression or XGBoost
  ├── Predict success probability
  └── Blend: 15% popularity + 15% hardware + 70% learned

[Execution]  [Same as Phase A]

[Adaptation]
  ├── Nightly: Rebuild aggregates
  ├── Nightly: Retrain ranker
  └── Weekly: Refresh hardware stats
```

**Key Characteristics**:
- Rich analytics for learning
- Offline retraining pipeline
- Learned model replaces some hand weights
- Still local-first

### Phase C: AdaptiveRouter Pattern

```text
[Qdrant Vector Store]
  ├── Named vectors:
  │   ├── route_dense (primary)
  │   ├── tags_dense (optional)
  │   └── failure_dense (future)
  └── Rich payload for all constraints

[Retrieval]
  ┌─────────────────────────────────────────────────────┐
  │ Qdrant Query API                            │
  │   ├─ Prefetch 1: Dense semantic (top 30)    │
  │   ├─ Prefetch 2: Sparse keyword (top 30)   │
  │   └─ Main query: Filtered fusion              │
  │       (output type, language, context, avoid)    │
  └─────────────────────────────────────────────────────┘
  ↓
  Top row candidates with fusion scores

[Ranking]
  ├── Expand to models
  ├── Compute recent score (last 200 similar)
  ├── Failure-mode penalty (from similar failures)
  └── Blend: 30% relevance + 5% slot + 5% popularity + 10% hardware + 50% recent

[Execution]  [Same as Phase A & B]

[Adaptation]
  ├── Real-time: Update statistics after each attempt
  ├── Online: Adjust scores based on recent performance
  └── Optional: Learn failure-mode embeddings
```

**Key Characteristics**:
- Most advanced retrieval
- Online adaptation possible
- Failure-mode awareness
- Scalable architecture

## ChatGPT-Specific Recommendations

### 1. Cluster-Based Similarity Search

Instead of KNN over all logs every time:
- **Offline**: Cluster task embeddings (K-means, 100 clusters)
- **Query**: Find cluster ID, use cluster-local stats
- **Benefit**: O(1) query vs O(n) KNN
- **Complexity**: Small overhead for clustering

### 2. Exponential Decay for Temporal Weighting

Give more weight to recent logs:

```python
def temporal_decay(attempts):
    now = time.time()
    weighted_sum = 0
    weight_sum = 0
    for attempt in attempts:
        age_hours = (now - attempt.ts) / 3600
        decay = math.exp(-age_hours / 24)  # 24-hour half-life
        weighted_sum += attempt.success * decay
        weight_sum += decay
    return weighted_sum / weight_sum
```

### 3. Nightly Adaptation Pipeline

Consistent automated improvement:

```python
async def nightly_job():
    # 1. Rebuild aggregates
    rebuild_stats()

    # 2. Retrain ranker
    model = retrain_ranker(last_n=10000)

    # 3. Decay popularity priors
    decay_priors()

    # 4. Update hardware profiles
    refresh_hardware_stats()

    # 5. Snapshot artifacts
    snapshot_version()
```

### 4. A/B Testing Framework

Experiment with routing strategies:

```python
AB_TEST_CONFIG = {
    "control": {"weights": CONTROL_WEIGHTS},
    "variant_a": {"weights": VARIANT_A_WEIGHTS},
    "variant_b": {"weights": VARIANT_B_WEIGHTS}
}

# Route 10% to each variant, compare success rates
```

### 5. Multi-Stage Verification with Early Exit

Don't run full verification chain if early stages fail:

```python
def verify_with_early_exit(output, verifier_chain):
    for stage in verifier_chain:
        result = stage.run(output)
        if not result.pass:
            log(stage, result)
            return Verdict(pass=False, stage=stage, recoverable=result.recoverable)
    return Verdict(pass=True)
```

**Benefits**:
- Faster verification (fail fast)
- Rich failure information (which stage failed)
- Lower latency on failures

## Transpiler-Specific Insights

### YAML Task Analysis

The transpiler needs to understand YAML to route effectively:

**YAML Fields for Routing**:
- `agent: task_type` → Maps to specialization
- `tools: [...]` → May influence model choice
- `requirements: [...]` → May need specific capabilities
- `output: format` → Influences verification type
- `context: {...}` → Influences context window need

**Example Mappings**:
```yaml
# Input
agent:
  type: "code_generation"
  language: "rust"
  scope: "from_scratch"

# Routes to
specialization_category: "requirement_to_executable_rust_code_from_scratch"
preferred_output_shape: "rust code block"
verification_need: "very-high"
```

### Rust-Specific Considerations

**Model Capabilities for Rust**:
- Must handle borrow checker concepts
- Should understand lifetimes
- Should know common crates (serde, tokio, etc.)
- Should understand attribute macros

**Verification for Rust**:
1. **Syntax**: `ast.parse()` for Python, equivalent for Rust
2. **Build**: `cargo check` (fastest compilation check)
3. **Clippy**: `cargo clippy` (linting)
4. **Tests**: `cargo test` (unit tests)
5. **Docs**: Ensure documentation compiles

### Development Workflow Integration

**How Router Fits in Transpiler**:

```python
# Transpiler main flow
def transpile_yaml(yaml_path):
    # 1. Parse and analyze YAML
    task = analyze_yaml(yaml_path)
    logger.info(f"Task: {task.type}, Language: {task.language}")

    # 2. Route to best models
    models = router.route(task)
    logger.info(f"Ranked models: {[m.name for m in models]}")

    # 3. Execute with retry
    for model in models:
        try:
            result = execute_model(model, task.prompt)
            verified = verify(result, task.verification_type)

            if verified.success:
                logger.info(f"Success with {model.name}")
                return result

        except ExecutionError as e:
            logger.warning(f"Model {model.name} failed: {e}")
            continue

    # 4. All models failed
    logger.error("All models failed")
    raise TranspilerError("No model could complete task")
```

## Key Takeaways

### What ChatGPT Emphasizes Over Basic Approaches

1. **Hybrid over single method** - Combine dense + sparse
2. **Similarity-aware over global** - Match to similar tasks
3. **Two-layer over single-layer** - Separate concerns
4. **Progressive over all-at-once** - MVP → learned → adaptive
5. **Failure-mode over binary** - Track how, not just if
6. **Adaptive over static** - Let data drive decisions
7. **Specialization-aware over one-size** - Different logic per task type
8. **Verification-chain over single check** - Multi-stage validation
9. **Learning from logs** - Bayesian smoothing, temporal decay
10. **Nightly adaptation** - Continuous improvement pipeline

### What Makes ChatGPT's Approach Distinct

1. **Research-backed** - Cites benchmarks and industry practices
2. **Evolutionary** - Designed to grow from simple to complex
3. **Production-oriented** - Addresses failure modes, adaptation
4. **Analytics-focused** - Emphasizes learning from logs
5. **Failure-mode aware** - Penalizes patterns, not just outcomes
6. **Hybrid-first** - Recommends combining retrieval methods
7. **Similarity-driven** - Match logs by task similarity, not globally

## Conclusion

ChatGPT's approach provides a comprehensive, research-backed, evolution-friendly design for the transpiler's model router. It balances simplicity for MVP with sophistication for production, emphasizing data-driven learning and continuous adaptation.

The key insight is to start simple but design for growth, letting the system evolve from hand-tuned rules to learned models based on actual usage data.
