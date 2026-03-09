# OpenCode Brainstorming: Model Router for Transpiler

## Context and Requirements

### Transpiler Project Overview

The transpiler project is a YAML-to-Rust converter that uses local LLM models to transform YAML configurations into executable Rust code. The system has:

- **Input**: YAML configuration files describing agent tasks, workflows, or code generation requirements
- **Output**: Executable Rust code, configuration files, or documentation
- **Models**: Local LLM models running via llama.cpp or similar runtimes
- **CSV Data**: 95 specialization rows with model recommendations per task type

### Key Requirements for Model Router

1. **Local-Only**: Must run entirely on local hardware (no cloud services)
2. **Minimal Hardware**: Should work on consumer laptops (4-8GB RAM)
3. **CSV-Driven**: Routing decisions based on the 95-row specialization CSV
4. **Verification-Aware**: Must integrate with code parsing, compilation, and testing
5. **Log-Driven**: Must learn from past attempts to improve routing
6. **Retry-Smart**: Should retry, repair, and switch models intelligently

### Transpiler Workflow Integration Points

```
YAML Input → Task Analysis → [Model Router] → Model Selection
     ↓
Prompt Generation → Model Execution → Verification → Success/Failure
     ↓ (if failure)
[Model Router Retry Logic] → Try Next Model
```

## Core Design Decisions

### Decision 1: Routing Granularity

**Question**: Should routing happen at the task level, specialization level, or model level?

**Options**:

1. **Task-Level Routing**
   - Analyze entire YAML input
   - Select single best model upfront
   - No per-step routing
   - Simple but may not handle mixed tasks

2. **Specialization-Level Routing**
   - Map input to specialization category from CSV
   - Select models from that specialization's recommendations
   - Retry within specialization's models
   - Better for complex tasks

3. **Two-Layer Routing**
   - Layer 1: Select best specialization rows
   - Layer 2: Rank models from selected rows
   - Most flexible and data-driven

**OpenCode Position**: **Two-Layer Routing** is most appropriate

**Rationale**:
- YAML files often contain mixed subtasks (e.g., "parse config AND generate code AND run tests")
- Different specializations may be relevant for different subtasks
- Two-layer approach allows row selection based on overall task, then model selection based on performance
- Matches ChatGPT's recommendation and provides maximum flexibility

### Decision 2: Retrieval Strategy

**Question**: How should we find relevant specializations from the CSV?

**Options**:

1. **Dense-Only (Vector Similarity)**
   - Embedding of input and all row text
   - Cosine similarity search
   - Semantic understanding
   - Simple, fast

2. **Sparse-Only (Keyword/FTS)**
   - BM25 on category, synonyms, tags, keywords
   - Exact keyword matching
   - Precise for known patterns
   - Fast for exact matches

3. **Hybrid (Dense + Sparse)**
   - Combine vector similarity with BM25 scores
   - Fusion algorithms (weighted sum, RRF)
   - Best of both worlds
   - More complex

**OpenCode Position**: **Hybrid Retrieval with Weighted Fusion**

**Rationale**:
- YAML tasks have both semantic requirements ("generate a parser") and technical keywords ("struct", "serde", "derive")
- Dense-only fails on exact matches like "derive Deserialize"
- Sparse-only fails on semantic similarity ("data transformation" vs "schema conversion")
- Weighted fusion is simpler than RRF and sufficient for this scale
- Match ChatGPT's hybrid approach

### Decision 3: Vector Database Technology

**Question**: Which vector DB/storage system should we use for minimal hardware?

**Options**:

1. **sqlite-vec**
   - Embedded, single file
   - In-memory or on-disk
   - Integrates with SQLite
   - ~5MB overhead
   - Good for < 1000 rows

2. **LanceDB**
   - Arrow-based, columnar storage
   - Good analytics
   - Supports reranking
   - ~50MB for small collection
   - More complex setup

3. **Qdrant**
   - Production-grade vector DB
   - Supports hybrid queries
   - Multiple vector types
   - More operational complexity
   - Overkill for < 500 rows

**OpenCode Position**: **sqlite-vec for MVP, migrate to LanceDB if needed**

**Rationale**:
- Transpiler is a developer tool, not a production service
- Single-file SQLite is ideal for developer experience
- sqlite-vec provides vector search without extra dependencies
- 95 rows is small enough for sqlite-vec performance
- Can migrate to LanceDB when we need better analytics

### Decision 4: Embedding Model

**Question**: Which embedding model provides best balance of accuracy and memory for local use?

**Options**:

1. **all-MiniLM-L6-v2 (Sentence-BERT)**
   - 384-dimensional vectors
   - ~120MB model size
   - Fast inference (~50ms per text)
   - Good semantic understanding
   - Proven in production

2. **Universal Sentence Encoder**
   - 512-dimensional vectors
   - ~80MB model size
   - Slower inference (~80ms per text)
   - Deeper semantic understanding
   - Multi-lingual support

3. **E5-small / similar**
   - 384-dimensional vectors
   - ~130MB model size
   - Fastest inference (~30ms per text)
   - Good balance
   - More recent

**OpenCode Position**: **all-MiniLM-L6-v2 with FastEmbed**

**Rationale**:
- 384-dimensional vectors balance memory and accuracy
- ~120MB with quantization is acceptable for 4GB RAM systems
- FastEmbed uses ONNX Runtime for efficiency
- Proven reliability in production systems
- Adequate semantic understanding for YAML-to-code tasks

### Decision 5: Log-Aware Ranking

**Question**: How should we incorporate past performance into model selection?

**Options**:

1. **Global Statistics**
   - Track success rate per model overall
   - Simple to implement
   - Doesn't account for task differences
   - "One size fits all" approach

2. **Similarity-Aware Statistics**
   - Match current task to past similar tasks
   - Compute stats on similar subset
   - More accurate but more complex
   - Requires embedding of past tasks

3. **Multi-Armed Bandit**
   - Thompson sampling or UCB
   - Balances exploration and exploitation
   - Mathematically rigorous
   - Complex to implement and tune

**OpenCode Position**: **Similarity-Aware Statistics with KNN Matching**

**Rationale**:
- YAML tasks vary significantly in difficulty and type
- A model that's good at "code generation" may be bad at "parsing"
- KNN on task embeddings finds similar past attempts
- Compute success rate, first-try rate, average retries on similar tasks
- More accurate than global stats without full bandit complexity
- Matches ChatGPT's approach

### Decision 6: Verification Integration

**Question**: How should we verify model outputs before considering them successful?

**Options**:

1. **Binary Success/Fail**
   - Just check if output is present
   - Minimal validation
   - High false positive rate
   - Not useful for learning

2. **Schema-Based Validation**
   - Validate against expected schema (JSON, YAML, Rust struct)
   - More reliable
   - Good for structured output
   - Doesn't catch code logic errors

3. **Compile/Execute-Based Verification**
   - Try to compile generated Rust code
   - Run cargo check
   - Run unit tests if present
   - Most reliable for code
   - Expensive but accurate

**OpenCode Position**: **Multi-Stage Verification Chain**

**Rationale**:
- Different output types need different verification
- Use verification appropriate to specialization:
  - Structured output: Parse + schema validate
  - Code: cargo check + tests
  - Config: YAML parser
  - Summaries: Length + coverage
- Each stage provides signal for learning
- Fail reasons inform retry and ranking

### Decision 7: Retry Strategy

**Question**: How should we handle model failures and retries?

**Options**:

1. **Fixed Retries**
   - Each model gets N attempts regardless
   - Simple
   - Wastes attempts on hopeless cases
   - Doesn't learn

2. **Specialization-Based Budgets**
   - Different specializations get different retry counts
   - Code gen: 3 retries, JSON: 2 retries
   - Context-aware
   - Still static

3. **Adaptive Retry with Error Analysis**
   - Analyze error types
   - Recoverable errors get more retries
   - Fatal errors trigger immediate switch
   - Learn from error patterns
   - More complex but effective

**OpenCode Position**: **Specialization-Based Budgets with Recoverable Error Detection**

**Rationale**:
- YAML parsing tasks fail quickly (2-3 attempts max)
- Code generation benefits from repair cycles (3-4 attempts)
- Distinguish between recoverable (syntax error) and fatal (OOM) failures
- Match ChatGPT's specialization-specific budgets
- Easier than full adaptive while still being smart

### Decision 8: Failure Mode Tracking

**Question**: Should we track *how* models fail, not just *if* they fail?

**Options**:

1. **Success/Failure Only**
   - Binary tracking
   - Simple storage
   - No diagnostic value
   - Can't learn from failure patterns

2. **Failure Reason Logging**
   - Store error messages
   - Categorize failures
   - Some diagnostic value
   - Doesn't penalize repeated failure patterns

3. **Failure Mode Embedding**
   - Embed failure explanations
   - Match similar failures
   - Penalize models that fail similarly
   - Richer learning signal
   - More complex

**OpenCode Position**: **Failure Reason Categorization with Future Mode Embedding**

**Rationale**:
- Start with failure reason categories (syntax, compile, test, OOM, timeout)
- Categorization provides value immediately
- Future: embed reasons for failure-mode aware ranking
- Progressive complexity: categories → embeddings → penalties
- Aligns with ChatGPT's failure-mode memory concept

### Decision 9: Ranking Formula

**Question**: How should we combine different signals into a final model score?

**Options**:

1. **Static Weights**
   - Fixed formula: 0.4*relevance + 0.3*popularity + 0.3*success_rate
   - Simple
   - Doesn't adapt
   - Manual tuning required

2. **Learned Model**
   - Train classifier/regression to predict success
   - Data-driven
   - Requires training pipeline
   - Overkill for MVP

3. **Adaptive Weights**
   - Start with static weights
   - Shift weight toward log score as data accumulates
   - Formula: `early: 35% logs → mature: 55% logs`
   - Balanced approach

**OpenCode Position**: **Adaptive Weights Formula**

**Rationale**:
- Start with proven static weights
- Shift toward data-driven as logs accumulate
- No ML pipeline needed initially
- Can add learned model later if beneficial
- Matches ChatGPT's weight evolution approach

### Decision 10: Hardware Awareness

**Question**: How should we factor in local hardware constraints?

**Options**:

1. **Static Configuration**
   - User specifies RAM, CPU, GPU
   - Hard filters on model requirements
   - Brittle
   - Manual maintenance

2. **Dynamic Measurement**
   - Track actual performance per model
   - Penalize OOM, slow models
   - Self-adapting
   - Requires measurement

3. **Model Catalog Metadata**
   - Pre-specify hardware tiers for each model
   - Static but comprehensive
   - Easy to update
   - Good enough for local use

**OpenCode Position**: **Model Catalog with Dynamic Penalty Adjustments**

**Rationale**:
- CSV already has context windows
- Add memory_tier to model catalog
- Track OOM, timeout, and compile failures
- Apply penalties dynamically based on tracked metrics
- Best of static and dynamic approaches

## Architecture Overview

### High-Level Design

```
┌─────────────────────────────────────────────────────────────────────┐
│                       YAML Input                                │
└────────────────────────┬────────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────────────┐
│                   Task Analysis                               │
│  - Extract task type, language, complexity                      │
│  - Estimate token requirements                                  │
│  - Identify verification requirements                              │
└────────────────────────┬────────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────────────┐
│                   Layer 1: Row Selection                      │
│  ├─ Dense Retrieval (sqlite-vec, cosine similarity)            │
│  ├─ Sparse Retrieval (FTS5, BM25)                          │
│  ├─ Weighted Fusion (0.45*dense + 0.25*sparse)             │
│  ├─ Constraint Filtering (output type, language, context)          │
│  └─ Top K Specializations                                        │
└────────────────────────┬────────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────────────┐
│                   Layer 2: Model Expansion & Ranking            │
│  ├─ Expand rows to model candidates (top1, top2, top3, wildcard)│
│  ├─ Apply slot bonuses (1.0, 0.82, 0.68, 0.55)            │
│  ├─ Find similar past attempts (KNN on task embeddings)            │
│  ├─ Compute similarity-aware statistics                             │
│  ├─ Apply Bayesian smoothing                                      │
│  ├─ Compute adaptive scores                                         │
│  └─ Rank and return top N models                                  │
└────────────────────────┬────────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────────────┐
│                   Execution Loop                               │
│  ┌────────────────────────────────────────────────────────────────┐  │
│  │ Try model #1 (up to budget attempts)                       │  │
│  │  ├─ Execute model (llama.cpp local API)                    │  │
│  │  ├─ Verify output (multi-stage chain)                         │  │
│  │  ├─ Log attempt (success, failure, latency, tokens)             │  │
│  │  └─ If success → return                                     │  │
│  │                                                              │  │
│  │  If fail:                                                    │  │
│  │    ├─ Check if recoverable (syntax vs OOM)                   │  │
│  │    ├─ If recoverable: repair prompt, retry                     │  │
│  │    └─ If fatal: switch to next model                        │  │
│  └────────────────────────────────────────────────────────────────┘  │
│  [Repeat for top N models or until success]                       │
└────────────────────────┬────────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────────────┐
│                   Output & Log Update                         │
│  - Return successful result                                          │
│  - Update model statistics                                         │
│  - Decay old statistics                                           │
│  - Optionally retrain/adapt ranking                                │
└─────────────────────────────────────────────────────────────────────┘
```

### Component Responsibilities

| Component | Responsibility | Technologies |
|-----------|----------------|---------------|
| **Task Analyzer** | Extract traits from YAML | Custom Python |
| **Vector Encoder** | Generate embeddings | FastEmbed (ONNX) |
| **Dense Retrieval** | Semantic search | sqlite-vec |
| **Sparse Retrieval** | Keyword search | SQLite FTS5 |
| **Row Selector** | Fusion & filtering | Custom Python |
| **KNN Matcher** | Find similar past attempts | sqlite-vec on logs |
| **Statistic Computer** | Calculate similarity-aware stats | NumPy/Pandas |
| **Ranker** | Combine signals into scores | Custom Python |
| **Executor** | Run models via llama.cpp | subprocess/HTTP |
| **Verifier Chain** | Multi-stage validation | JSONschema, cargo |
| **Logger** | Track all attempts | SQLite |
| **Adaptation** | Update statistics and weights | Custom Python (async) |

## Data Structures

### SQLite Schema

```sql
-- Specializations (from CSV)
CREATE TABLE specializations (
    id INTEGER PRIMARY KEY,
    row_kind TEXT,
    specialization_category TEXT,
    specialization_synonyms TEXT,
    related_synonym_tags TEXT,
    top1_model TEXT,
    top1_ctx INTEGER,
    top2_model TEXT,
    top2_ctx INTEGER,
    top3_model TEXT,
    top3_ctx INTEGER,
    wildcard_model TEXT,
    wildcard_ctx INTEGER,
    primary_strength TEXT,
    secondary_strengths TEXT,
    preferred_input_shape TEXT,
    preferred_output_shape TEXT,
    task_granularity TEXT,
    context_need TEXT,
    output_rigidity TEXT,
    verification_need TEXT,
    recommended_decoding TEXT,
    latency_bias TEXT,
    router_keywords TEXT,
    avoid_when TEXT,
    selection_notes TEXT
);

-- Pre-computed embeddings
CREATE TABLE embeddings (
    spec_id INTEGER PRIMARY KEY,
    embedding BLOB  -- 384 * 4 bytes = 1536 bytes
);

-- Model catalog (hardware info)
CREATE TABLE model_catalog (
    model_name TEXT PRIMARY KEY,
    context_window INTEGER,
    memory_tier TEXT,  -- 'tiny', 'small', 'medium', 'large'
    avg_latency_ms REAL,
    oom_count INTEGER DEFAULT 0,
    timeout_count INTEGER DEFAULT 0,
    popularity_prior REAL DEFAULT 1.0,
    enabled INTEGER DEFAULT 1
);

-- Attempt logs
CREATE TABLE attempt_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ts INTEGER NOT NULL,
    task_embedding BLOB,
    task_text TEXT,
    task_type TEXT,
    language TEXT,
    output_type TEXT,
    spec_id INTEGER,
    model_name TEXT,
    try_index INTEGER,
    success BOOLEAN,
    fail_reason TEXT,
    parse_pass BOOLEAN,
    compile_pass BOOLEAN,
    test_pass BOOLEAN,
    latency_ms REAL,
    token_count INTEGER
);

-- Model statistics (aggregated)
CREATE TABLE model_stats (
    model_name TEXT PRIMARY KEY,
    total_attempts INTEGER DEFAULT 0,
    successes INTEGER DEFAULT 0,
    failures INTEGER DEFAULT 0,
    avg_attempts_to_success REAL DEFAULT 0,
    verified_success_rate REAL DEFAULT 0,
    last_updated INTEGER
);

-- FTS5 index for sparse search
CREATE VIRTUAL TABLE spec_fts USING fts5(
    spec_id,
    specialization_category,
    specialization_synonyms,
    related_synonym_tags,
    router_keywords,
    selection_notes,
    content='specializations',
    content_rowid='id'
);
```

### Key Data Flows

1. **CSV Import → Specializations Table**
2. **Row Text Building → Embeddings Table**
3. **Task Input → Task Embedding → KNN Search**
4. **Similarity Stats → Model Stats Table**
5. **Execution → Attempt Logs Table**
6. **Aggregation Job → Model Stats Table**

## OpenCode-Specific Considerations

### 1. YAML Task Types

The transpiler likely handles several distinct task types based on the CSV:

- **Code Generation** (various languages)
- **Code Editing/Patching** (existing code modifications)
- **Structured Output** (JSON, YAML, configs)
- **Summarization** (documentation, code explanations)
- **Debugging/Repair** (fixing errors)
- **Planning** (workflow breakdown)

Each type needs:
- Different verification strategies
- Different retry budgets
- Different model preferences

### 2. Rust-Specific Verification

For Rust code generation tasks, verification should include:

```python
rust_verifier = [
    ("parse", check_rust_syntax),           # ast.parse
    ("cargo_check", run_cargo_check),      # cargo check
    ("compile", attempt_compilation),        # rustc
    ("clippy", run_clippy),               # cargo clippy
    ("tests", run_unit_tests)               # cargo test
]
```

This provides multiple signals for learning and helps catch errors early.

### 3. Dependency Awareness

The transpiler likely has awareness of:

- Target Rust version
- Available crates and dependencies
- Project structure
- Build system (Cargo, Make, etc.)

Router should be able to:
- Filter models that can't handle required Rust features
- Pass context about project structure to models
- Avoid suggesting models with incompatible approaches

### 4. Incremental Development

As a developer tool, the router should support:

- **Hot reloading** of CSV updates
- **A/B testing** of routing strategies
- **Debug mode** with detailed logging
- **Stats export** for analysis
- **Config validation** on startup

## Potential Pitfalls and Mitigations

### Pitfall 1: Overfitting to Recent Logs

**Issue**: If a model has 10 recent successes, it may be over-weighted even if long-term performance is poor.

**Mitigation**:
- Use decay function for temporal weighting
- Minimum attempt threshold (e.g., 20 attempts before trusting stats)
- Blend recent success rate with long-term rate

### Pitfall 2: Cold Start Problem

**Issue**: No logs initially, router has no learning signal.

**Mitigation**:
- Start with hand-tuned static weights
- Use popularity prior from CSV ordering
- Reduce log-weight contribution initially (35% vs 55%)
- Gradually increase as logs accumulate

### Pitfall 3: Verification False Negatives

**Issue**: Correct output fails verification due to overly strict rules.

**Mitigation**:
- Multi-stage verification with early exit on pass
- Separate "hard" vs "soft" verification
- Log verification failures for analysis
- Allow manual override in debug mode

### Pitfall 4: Model Context Overflow

**Issue**: Task requires more tokens than model's context window.

**Mitigation**:
- Pre-filter models based on estimated token count
- Log context overflow events for hardware penalties
- Prefer models with larger context when estimate is high
- Warn user if all candidates are under-provisioned

### Pitfall 5: Specialization Explosion

**Issue**: Many similar specializations dilute ranking signals.

**Mitigation**:
- Group related specializations (aggregate categories)
- Use row_kind as coarse filter
- Deduplicate model candidates with max score + repeat bonus
- Limit top K rows to avoid noise

## OpenCode Recommendations Summary

### Immediate Implementation (MVP)

1. **Two-layer routing** with hybrid retrieval
2. **sqlite-vec** for vector search
3. **all-MiniLM-L6-v2** with FastEmbed
4. **Static weights** initially, adaptive later
5. **Multi-stage verification** per specialization
6. **Specialization-based retry budgets**
7. **Similarity-aware statistics** (not global)

### Future Enhancements

1. **Failure-mode embedding** for smarter penalties
2. **Bayesian smoothing** with optimal alpha
3. **Nightly adaptation** jobs
4. **A/B testing framework**
5. **Analytics dashboard**
6. **LanceDB migration** for better analytics
7. **Cross-encoder reranker** for retrieval

### Non-Goals

What OpenCode is NOT prioritizing for the transpiler router:

1. **Distributed deployment** - Single-user local tool
2. **Multi-tenant isolation** - Developer's personal machine
3. **Real-time online learning** - Nightly batch adaptation is sufficient
4. **Complex ML pipeline** - Keep it simple and maintainable
5. **Cloud fallback** - Purely local system

## Integration Points with Transpiler

### Entry Points

```python
# In transpiler main workflow
def process_yaml(yaml_file):
    task = analyze_yaml(yaml_file)
    ranked_models = router.route(task)
    result = execute_with_retry(task, ranked_models)
    return result
```

### Configuration

```yaml
# transpiler/config.yaml
router:
  db_path: ./data/router.db
  max_models_to_try: 3
  enable_logging: true
  debug_mode: false

  retry:
    default_budget: 2
    code_gen_budget: 3
    structured_output_budget: 2

  models:
    runtime: llama.cpp  # or ollama
    local_models_dir: ./models

  adaptation:
    nightly_rebuild: true
    weight_evolution: true
```

### Extensions

The router should be extensible for:
- New specializations (add to CSV)
- New models (add to catalog)
- New verification types (register verifiers)
- New ranking features (extend score computation)

## Conclusion

OpenCode's approach to the model router emphasizes:

1. **Practicality over sophistication** - Start with what works
2. **Developer experience** - Simple setup, easy debugging
3. **Data-driven evolution** - Learn from logs gradually
4. **Local-first** - No cloud dependencies
5. **Maintainable** - Clear code, well-documented

The router should feel like a natural part of the transpiler's workflow, not a separate complex system.
