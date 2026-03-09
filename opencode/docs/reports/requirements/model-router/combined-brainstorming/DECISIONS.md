# Combined Brainstorming: Model Router Design Decisions

## Executive Summary

This document synthesizes ideas from OpenCode's original designs and ChatGPT's research-backed approach to create a unified model router design for the transpiler. It documents all considered options, trade-offs, and final decisions with rationale.

## Decision Matrix Overview

| Decision Area | Options Considered | Final Decision | Key Rationale |
|--------------|-------------------|----------------|----------------|
| **Routing Granularity** | Task-level, Spec-level, Two-layer | **Two-layer** | Separates concerns, enables independent optimization |
| **Retrieval Strategy** | Dense-only, Sparse-only, Hybrid | **Hybrid** | Research shows 15-30% improvement, covers both semantic and keyword needs |
| **Vector Database** | sqlite-vec, LanceDB, Qdrant | **sqlite-vec (MVP)** | Single-file, minimal complexity, adequate for < 500 rows |
| **Embedding Model** | all-MiniLM, Universal-SE, E5-small | **all-MiniLM-L6-v2** | Best balance of RAM (120MB), speed (50ms), accuracy (384-dim) |
| **Log Statistics** | Global, Similarity-aware, Bandit | **Similarity-aware KNN** | Tasks are heterogeneous, similar-task comparison is more accurate |
| **Ranking Approach** | Static weights, Learned model, Adaptive | **Adaptive weights** | Start proven, shift to data-driven as logs accumulate |
| **Verification** | Binary, Schema, Multi-stage | **Specialization-driven multi-stage** | Different tasks need different verification chains |
| **Retry Strategy** | Fixed, Budget-based, Adaptive | **Specialization-based budgets** | Match retry effort to task type (code=3, JSON=2) |
| **Failure Tracking** | Binary, Reasons, Mode-embedding | **Reason categorization → mode-embedding** | Progressive complexity, enables failure-aware routing |
| **Hardware Awareness** | Static config, Dynamic measurement | **Catalog + dynamic penalties** | Pre-specify tiers, adapt based on actual failures |

## Detailed Decision Analysis

### Decision 1: Routing Granularity

**Context**: The transpiler handles diverse YAML tasks ranging from simple config generation to complex multi-file code generation.

**Options Analyzed**:

1. **Task-Level Routing**
   - Analyze entire YAML input once
   - Select single best model
   - Execute until success or failure
   - **Pros**: Simple, single decision point
   - **Cons**: Fails on mixed-task YAMLs (e.g., "generate code AND tests AND documentation")
   - **Verdict**: Too inflexible for transpiler's diverse use cases

2. **Specialization-Level Routing**
   - Map YAML to single specialization category
   - Select models only from that category
   - Retry within category's models
   - **Pros**: Leverages CSV structure, good for focused tasks
   - **Cons**: YAMLs often cross category boundaries
   - **Verdict**: Better than task-level but still limited

3. **Two-Layer Routing**
   - Layer 1: Select top K specialization rows
   - Layer 2: Rank models from selected rows
   - **Pros**: Maximum flexibility, separates concerns, data-driven
   - **Cons**: More complex, requires two ranking stages
   - **Verdict**: **SELECTED** - Best fit for transpiler needs

**Decision**: Two-Layer Routing

**Rationale**:
- YAML inputs frequently contain multiple subtasks or mixed requirements
- Two-layer approach allows row selection based on overall task, then model selection based on performance
- Enables independent optimization of each layer (row selection vs model ranking)
- Matches both ChatGPT's recommendation and research on two-stage retrieval
- Layer 1 handles "what kind of task is this?"
- Layer 2 handles "which model is best for this specific task?"

### Decision 2: Retrieval Strategy

**Context**: Need to find relevant specializations from 95 CSV rows.

**Options Analyzed**:

1. **Dense-Only (Vector Similarity)**
   - Embed all row text and task input
   - Cosine similarity search
   - **Pros**: Semantic understanding, handles variations, well-studied
   - **Cons**: Misses exact keywords (e.g., "derive Deserialize"), may rank semantically similar but task-inappropriate rows
   - **Verdict**: Good baseline but insufficient alone

2. **Sparse-Only (Keyword/FTS)**
   - BM25 on category, synonyms, tags, keywords
   - **Pros**: Exact keyword matching, precise for known patterns, fast
   - **Cons**: Misses semantic similarity, fails on synonyms/variations
   - **Verdict**: Good complement to dense but insufficient alone

3. **Hybrid (Dense + Sparse)**
   - Run both dense and sparse retrieval
   - Fuse results using weighted sum or RRF
   - **Pros**: Best of both worlds, covers semantic and lexical needs, research-backed
   - **Cons**: More complex, requires fusion tuning
   - **Verdict**: **SELECTED** - Superior for production use

**Research Evidence**:
- BEIR benchmark: Hybrid outperforms dense by 8.5% and sparse by 6.9%
- Production RAG systems: 15-30% improvement with hybrid
- Dense-only fails: "code generation" ≈ "data transformation" but "derive" ≠ "generate"
- Sparse-only fails: "derive" exact match but "create struct" similar but no match

**Decision**: Hybrid Retrieval with Weighted Fusion

**Rationale**:
- Transpiler tasks have both semantic requirements ("generate parser") and technical keywords ("derive", "serde")
- Weighted fusion (0.45*dense + 0.25*sparse + 0.10*filters) is simpler than RRF and sufficient for this scale
- Matches ChatGPT's recommendation and research best practices
- Provides robustness against edge cases in both retrieval methods

### Decision 3: Vector Database Technology

**Context**: Need efficient vector similarity search for minimal hardware (4-8GB RAM).

**Options Analyzed**:

1. **sqlite-vec**
   - Embedded SQLite extension for vector search
   - In-memory or on-disk storage
   - **Pros**: Single file, ~5MB overhead, simple setup, good for < 1000 rows, integrates with existing SQLite
   - **Cons**: Limited to basic KNN, no advanced features
   - **Verdict**: **SELECTED for MVP** - Ideal balance of simplicity and capability

2. **LanceDB**
   - Arrow-based, columnar storage
   - Supports vector search, analytics, reranking
   - **Pros**: Rich analytics, supports custom rerankers, better for > 1000 rows
   - **Cons**: More complex setup, ~50MB overhead, more dependencies
   - **Verdict**: Good upgrade path when analytics needs grow

3. **Qdrant**
   - Production-grade vector database
   - Supports hybrid queries, multiple vectors, quantization
   - **Pros**: Most features, scalable, hybrid queries built-in
   - **Cons**: High complexity (~200MB RAM for small collection), operational overhead, overkill for < 500 rows
   - **Verdict**: Future path for scale, not initial choice

**Hardware Analysis**:
- sqlite-vec: ~50MB RAM for 95 rows (negligible)
- LanceDB: ~50MB RAM + query overhead
- Qdrant: ~200MB RAM minimum (can use on-disk to reduce)

**Decision**: sqlite-vec for MVP, with LanceDB migration path

**Rationale**:
- Transpiler is a developer tool, not a high-traffic service
- 95 rows is well within sqlite-vec performance envelope
- Single-file SQLite is ideal for developer experience (easy backup, inspect, debug)
- LanceDB provides clear upgrade path when analytics needs increase
- Matches OpenCode's practical focus and ChatGPT's phased approach

### Decision 4: Embedding Model

**Context**: Need accurate semantic embeddings with minimal memory footprint.

**Options Analyzed**:

1. **all-MiniLM-L6-v2 (Sentence-BERT)**
   - 384-dimensional vectors
   - ~120MB model size
   - ~50ms inference time per text
   - **Pros**: Good accuracy, proven reliability, reasonable RAM, fast
   - **Cons**: Lower dimensionality than newer models
   - **Verdict**: **SELECTED** - Best balance

2. **Universal Sentence Encoder**
   - 512-dimensional vectors
   - ~80MB model size
   - ~80ms inference time
   - **Pros**: Deeper semantic understanding, multi-lingual
   - **Cons**: Slower, higher dimension (more memory for storage)
   - **Verdict**: Good alternative if all-MiniLM underperforms

3. **E5-small / Similar**
   - 384-dimensional vectors
   - ~130MB model size
   - ~30ms inference time
   - **Pros**: Fastest, newer, good accuracy
   - **Cons**: Slightly more RAM than all-MiniLM
   - **Verdict**: Worth testing, but all-MiniLM is safer initial choice

**Research Evidence**:
- FastEmbed with ONNX Runtime: Efficient, minimal dependencies
- MTEB leaderboard: all-MiniLM-L6-v2 strong retrieval performance
- Quantization: FP16 reduces memory 2x, INT8 reduces 4x with <1% accuracy loss
- 384 dimensions: Good balance of semantic richness and storage efficiency

**Decision**: all-MiniLM-L6-v2 with FastEmbed (ONNX)

**Rationale**:
- 384-dimensional vectors balance memory (1536 bytes per vector) and accuracy
- ~120MB with quantization is acceptable for 4GB RAM systems
- FastEmbed uses ONNX Runtime for CPU efficiency (no PyTorch overhead)
- Proven reliability in production systems
- Adequate semantic understanding for YAML-to-code tasks
- Can migrate to E5-small if accuracy proves insufficient

### Decision 5: Log-Aware Ranking

**Context**: Must incorporate past performance without overfitting.

**Options Analyzed**:

1. **Global Statistics**
   - Track success rate per model across all tasks
   - Simple: `success_rate = total_successes / total_attempts`
   - **Pros**: Simple, easy to understand
   - **Cons**: Doesn't account for task differences, "one size fits all"
   - **Verdict**: Too naive for heterogeneous tasks

2. **Similarity-Aware Statistics**
   - Match current task to past similar tasks (KNN on embeddings)
   - Compute stats on similar subset only
   - More accurate for task-specific performance
   - **Pros**: Accounts for task heterogeneity, more accurate routing
   - **Cons**: More complex, requires KNN search
   - **Verdict**: **SELECTED** - Better accuracy justified by complexity

3. **Multi-Armed Bandit (Thompson Sampling, UCB)**
   - Mathematical framework for exploration/exploitation
   - Optimizes for long-term reward
   - **Pros**: Theoretically optimal, balances exploration
   - **Cons**: Complex to implement and tune, overkill for this scale
   - **Verdict**: Not necessary for initial implementation

**Research Evidence**:
- Heterogeneous tasks: A model good at "code generation" ≠ good at "YAML parsing"
- Similarity matching: Proven in RAG systems for better relevance
- KNN overhead: Acceptable for < 10,000 logs with proper indexing
- Cluster optimization: Can use pre-computed clusters for O(1) lookup

**Decision**: Similarity-Aware Statistics with KNN Matching

**Rationale**:
- Transpiler tasks vary significantly in difficulty and type
- Global statistics would unfairly penalize/benefit models based on unrelated tasks
- KNN on task embeddings finds semantically similar past attempts
- Compute per-model statistics (success rate, first-try rate, avg retries) on similar subset
- More accurate than global stats without full bandit complexity
- Matches ChatGPT's approach and research on similarity-aware retrieval

### Decision 6: Verification Integration

**Context**: Different output types need different verification strategies.

**Options Analyzed**:

1. **Binary Success/Fail**
   - Check if output is present/non-empty
   - **Pros**: Simple
   - **Cons**: High false positive rate, no diagnostic value
   - **Verdict**: Insufficient

2. **Schema-Based Validation**
   - Validate against expected schema (JSON, YAML, Rust struct)
   - **Pros**: More reliable for structured output
   - **Cons**: Doesn't catch code logic errors
   - **Verdict**: Good for structured output only

3. **Multi-Stage Verification Chain**
   - Specialization-driven: different chains for different output types
   - Each stage can pass/fail independently
   - **Pros**: Most comprehensive, rich learning signal, adapts to task type
   - **Cons**: More complex
   - **Verdict**: **SELECTED** - Best approach

**Verification Chain Examples**:

**JSON/YAML**:
1. Parse with strict parser
2. Schema validate (if provided)
3. Required fields present
4. No disallowed fields
5. Correct data types

**Code (Rust)**:
1. Syntax parse (AST equivalent)
2. Build check (cargo check)
3. Lint/typecheck (clippy)
4. Run unit tests (cargo test)
5. No obvious bugs

**Summary**:
1. Length within bounds
2. Section coverage
3. Required entities present
4. Not too verbose/brief

**Decision**: Specialization-Driven Multi-Stage Verification

**Rationale**:
- Different specializations have different verification needs
- "Strict JSON" needs parsing, "Code" needs compilation
- Multi-stage provides multiple signals for learning
- Each stage provides immediate feedback (fail fast on early stages)
- Rich failure information helps ranking and retry logic
- Matches ChatGPT's specialization-driven verifier chains

### Decision 7: Retry Strategy

**Context**: How many times to retry each model before switching?

**Options Analyzed**:

1. **Fixed Retries**
   - Each model gets N attempts (e.g., 3) regardless of task
   - **Pros**: Simple, predictable
   - **Cons**: Wastes attempts on hopeless cases, doesn't adapt
   - **Verdict**: Too naive

2. **Specialization-Based Budgets**
   - Different specializations get different retry counts
   - Code gen: 3 retries, JSON: 2 retries, Summary: 2 retries, Debug: 4 retries
   - **Pros**: Context-aware, efficient, matches task characteristics
   - **Cons**: Still static
   - **Verdict**: Good balance

3. **Adaptive Retry with Error Analysis**
   - Analyze error types (recoverable vs fatal)
   - Recoverable: retry with repair prompt
   - Fatal: immediate switch
   - Learn from error patterns
   - **Pros**: Most intelligent, maximizes success
   - **Cons**: Complex, requires error classification
   - **Verdict**: Future enhancement

**Research Evidence**:
- Format tasks (JSON, YAML): Fail fast, retries don't help much
- Code generation: Benefits from error-aware repair cycles
- Debugging: Needs multiple tries, each attempt provides more context
- Fail types: Syntax errors often recoverable, OOM always fatal

**Decision**: Specialization-Based Budgets with Recoverable Error Detection

**Rationale**:
- Different task types benefit differently from retries
- YAML parsing fails on syntax → 2 attempts max
- Code generation can be fixed with compiler error feedback → 3 attempts
- Distinguish between recoverable (syntax) and fatal (OOM, timeout)
- Matches ChatGPT's specialization-specific budgets
- Easier than full adaptive while still being smart
- Can add full error analysis in Phase C

### Decision 8: Failure Mode Tracking

**Context**: Should we track *how* models fail?

**Options Analyzed**:

1. **Success/Failure Only**
   - Binary tracking only
   - **Pros**: Simple storage
   - **Cons**: No diagnostic value, can't learn patterns
   - **Verdict**: Insufficient

2. **Failure Reason Categorization**
   - Store error messages and categories
   - Categories: syntax, compile, test, OOM, timeout, parse, schema
   - **Pros**: Diagnostic value, enables some learning
   - **Cons**: Doesn't penalize repeated patterns
   - **Verdict**: Good intermediate

3. **Failure Mode Embedding**
   - Embed failure explanations
   - Match similar failures across tasks
   - Penalize models that fail similarly
   - **Pros**: Richest learning signal, proactive avoidance
   - **Cons**: Most complex
   - **Verdict**: **Long-term goal, start with categorization**

**Research Evidence**:
- Failure patterns: Models consistently fail at JSON parsing vs. code generation
- Failure-mode awareness: Production systems track failure embeddings
- Example: "keeps returning markdown fences" → Penalize for JSON tasks
- Benefit: Enables proactive model avoidance, not reactive retrying

**Decision**: Start with Categorization, Add Embedding Later

**Rationale**:
- Progressive complexity is best for practical implementation
- Categorization provides immediate value
- Future: embed reasons for failure-mode aware ranking
- Aligns with ChatGPT's failure-mode memory concept
- Don't over-engineer initial implementation

### Decision 9: Ranking Formula

**Context**: How to combine different signals into final score?

**Options Analyzed**:

1. **Static Weights**
   - Fixed formula: `0.4*relevance + 0.3*popularity + 0.3*success_rate`
   - **Pros**: Simple, no training
   - **Cons**: Doesn't adapt, manual tuning
   - **Verdict**: Good starting point

2. **Learned Model**
   - Train classifier/regression to predict success
   - Data-driven
   - **Pros**: Optimal given data, can discover non-linear patterns
   - **Cons**: Requires training pipeline, overkill for MVP
   - **Verdict**: Future enhancement (Phase B)

3. **Adaptive Weights**
   - Start with static weights
   - Shift weight toward log score as data accumulates
   - Early: `0.35*relevance + 0.35*logs`
   - Mature: `0.25*relevance + 0.55*logs`
   - **Pros**: Data-driven without ML complexity, evolves with usage
   - **Cons**: Requires careful tuning of evolution schedule
   - **Verdict**: **SELECTED** - Best balance

**Evolution Schedule**:

| Phase | Logs Threshold | Log Weight | Rationale |
|-------|--------------|------------|-----------|
| Phase A (MVP) | 0-500 | 35% | Not enough data, rely on heuristics |
| Phase B (Early) | 500-2000 | 45% | Some confidence, start trusting data |
| Phase C (Mature) | 2000+ | 55% | High confidence, let data drive |

**Decision**: Adaptive Weights with Evolution

**Rationale**:
- Starts with proven heuristics for cold start
- Shifts toward data-driven as logs accumulate
- No ML pipeline needed initially
- Can add learned model later if beneficial
- Matches ChatGPT's weight evolution approach
- Gradual adaptation prevents overfitting to small early logs

### Decision 10: Hardware Awareness

**Context**: Factor in local hardware constraints (RAM, CPU, GPU).

**Options Analyzed**:

1. **Static Configuration**
   - User specifies RAM, CPU, GPU in config
   - Hard filters on model requirements
   - **Pros**: Predictable, easy to understand
   - **Cons**: Brittle, requires manual updates
   - **Verdict**: Good for static environments, not dynamic

2. **Dynamic Measurement**
   - Track actual performance per model
   - Penalize OOM, slow models
   - Self-adapting based on observed behavior
   - **Pros**: Automatically adapts to hardware
   - **Cons**: Requires measurement infrastructure, noisy data
   - **Verdict**: Best for production, overkill for local tool

3. **Model Catalog with Dynamic Penalties**
   - Pre-specify hardware tiers in catalog (tiny, small, medium, large)
   - Track OOM, timeout, compile failures
   - Apply penalties dynamically based on tracked metrics
   - **Pros**: Good baseline with adaptive adjustments
   - **Cons**: Requires manual catalog updates
   - **Verdict**: **SELECTED** - Best balance

**Decision**: Model Catalog with Dynamic Penalties

**Rationale**:
- CSV already has context windows (hardware proxy)
- Add memory_tier to model catalog (static baseline)
- Track OOM, timeout, and compile failures (dynamic adjustments)
- Apply penalties dynamically based on tracked metrics
- Best of static and dynamic approaches
- Matches both approaches without over-engineering

## Final Architecture Design

### System Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                      YAML Input                                 │
└────────────────────────┬────────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────────────┐
│                   Task Analyzer                                │
│  - Extract: task type, language, complexity, format            │
│  - Estimate: token requirements, verification type                │
└────────────────────────┬────────────────────────────────────────────┘
                     │
          ┌────────────────┴────────────────┐
          │                                 │
          ▼                                 ▼
┌──────────────────────┐         ┌───────────────────────────┐
│  Layer 1: Row      │         │  Task Embedding         │
│  Selection          │         │  (FastEmbed)            │
│                    │         └───────────────────────────┘
│  ├─ Dense Search    │                    │
│  │  (sqlite-vec)   │                    │
│  ├─ Sparse Search   │                    │
│  │  (FTS5 BM25)     │                    │
│  ├─ Weighted Fusion │                    │
│  └─ Top K Rows      │                    │
└──────────┬───────────┘                    │
           │                                │
           │                                │
           ▼                                ▼
┌──────────────────────┐         ┌───────────────────────────┐
│  Layer 2: Model    │         │  KNN on Logs           │
│  Expansion & Rank  │         │  (Similarity-Aware)      │
│                    │         │  - Find similar tasks    │
│  ├─ Expand (4 per │         │  - Compute per-model stats│
│  │     row)         │         │  - Bayesian smoothing    │
│  ├─ Slot Bonus     │         └───────────────────────────┘
│  ├─ Similar Stats  │
│  ├─ Adaptive Score │
│  └─ Top N Models  │
└──────────┬───────────┘
           │
           │
           ▼
┌─────────────────────────────────────────────────────────────────────┐
│                   Execution Loop                                │
│  ┌─────────────────────────────────────────────────────────────┐  │
│  │ For each model (up to N):                              │  │
│  │   For each attempt (up to budget):                        │  │
│  │     ├─ Execute (llama.cpp)                               │  │
│  │     ├─ Verify (specialization chain)                      │  │
│  │     ├─ Log (success, failure, latency, tokens)             │  │
│  │     └─ If success → return                             │  │
│  │     If fail:                                            │  │
│  │       ├─ Recoverable? → repair prompt, retry            │  │
│  │       └─ Fatal? → switch to next model                 │  │
│  └─────────────────────────────────────────────────────────────┘  │
└────────────────────────┬────────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────────────┐
│                   Output & Log Update                            │
│  - Return successful result                                       │
│  - Update model statistics                                         │
│  - Decay old statistics (nightly)                               │
│  - Refresh hardware profile                                         │
└─────────────────────────────────────────────────────────────────────┘
```

### Component Specifications

| Component | Technology | Memory | Notes |
|-----------|-------------|---------|-------|
| **Vector Encoder** | FastEmbed + all-MiniLM-L6-v2 | ~120MB | ONNX Runtime, quantized |
| **Dense Search** | sqlite-vec | ~50MB | In-memory vectors, < 5ms query |
| **Sparse Search** | SQLite FTS5 | ~10MB | BM25, integrated |
| **Storage** | SQLite | ~100MB | Single file, all data |
| **KNN Search** | sqlite-vec on logs | +50MB | Similarity-aware matching |
| **Statistics** | NumPy/Pandas | +50MB | Computation and aggregation |
| **Executor** | llama.cpp subprocess | - | External process |
| **Total RAM** | | **~380MB** | Well within 4GB budget |

## Phased Implementation Plan

### Phase 1: TinyRouter (Weeks 1-4)

**Goal**: Prove concept, start collecting logs

**Components**:
- SQLite with sqlite-vec and FTS5
- FastEmbed for embeddings
- Static weight ranking (35% logs initially)
- Specialization-based retry budgets
- Multi-stage verification chains

**Success Criteria**:
- [ ] Routing accuracy > 80% (top 3 rows)
- [ ] 70% tasks succeed in ≤ 2 models
- [ ] 1000+ attempts logged
- [ ] All verification types implemented

### Phase 2: LearnedRouter (Weeks 5-12)

**Goal**: Real log-driven ranking with analytics

**Components**:
- Add DuckDB for analytics warehouse
- Add LanceDB for advanced retrieval
- Train logistic regression or XGBoost ranker
- Offline retraining pipeline (nightly)
- Cluster-based similarity search (optional)

**Success Criteria**:
- [ ] Learned model accuracy > 85%
- [ ] Outperforms static weights by > 5%
- [ ] 5000+ attempts logged
- [ ] Nightly jobs running stably

### Phase 3: AdaptiveRouter (Weeks 13-24)

**Goal**: Advanced retrieval with online adaptation

**Components**:
- Migrate to Qdrant for hybrid queries
- Implement failure-mode embeddings
- Add online adaptation (real-time stats updates)
- Multi-vector storage (dense, tags, failure)
- Quantization for memory optimization

**Success Criteria**:
- [ ] Hybrid retrieval working correctly
- [ ] Failure-mode awareness improving routing
- [ ] System handling 1000+ specializations
- [ ] Online adaptation effective

## Non-Goals (What We're Not Prioritizing)

1. **Distributed Deployment** - Single-user local tool, not multi-tenant service
2. **Real-Time Online ML** - Nightly batch adaptation is sufficient, simpler
3. **Complex ML Pipeline** - Keep it maintainable, avoid over-engineering
4. **Cloud Fallback** - Purely local system required
5. **GPU Acceleration** - CPU-optimized is sufficient for local use
6. **Production HA/Scaling** - Not a high-traffic service

## Risk Mitigation

### Risk 1: Overfitting to Recent Logs

**Mitigation**:
- Temporal decay (24-hour half-life)
- Minimum attempt threshold (20 attempts)
- Blend recent with long-term rates

### Risk 2: Cold Start Problem

**Mitigation**:
- Start with hand-tuned static weights
- Reduce log-weight contribution initially (35% vs 55%)
- Use popularity prior from CSV

### Risk 3: Verification False Negatives

**Mitigation**:
- Multi-stage with early exit
- Separate "hard" vs "soft" verification
- Debug mode with detailed logging

### Risk 4: Model Context Overflow

**Mitigation**:
- Pre-filter models by estimated tokens
- Log overflow events for penalties
- Warn user if all candidates under-provisioned

### Risk 5: Specialization Explosion

**Mitigation**:
- Use row_kind as coarse filter
- Limit top K rows (10-20)
- Deduplicate model candidates

## Key Takeaways

### What This Combined Approach Achieves

1. **Practicality** - Starts simple, evolves based on needs
2. **Research-Backed** - Decisions supported by benchmarks and industry practices
3. **Local-First** - Designed for 4-8GB RAM, no cloud dependencies
4. **Data-Driven** - Learns from logs with Bayesian smoothing
5. **Adaptive** - Weights shift from heuristics to evidence over time
6. **Hybrid Retrieval** - Combines semantic and lexical search
7. **Similarity-Aware** - Matches logs to similar tasks, not globally
8. **Failure-Mode Aware** - Tracks how models fail, not just if they fail
9. **Specialization-Driven** - Different logic for different task types
10. **Progressive Complexity** - MVP → Learned → Adaptive phases

### Why This Approach Works for Transpiler

1. **Handles Diverse Tasks** - YAML inputs range from simple configs to complex code gen
2. **Rust-Aware** - Verification includes cargo check, clippy, tests
3. **Developer Tool Focus** - Single-file SQLite, easy debugging, no complex ops
4. **Minimal Hardware** - ~380MB RAM, works on any laptop
5. **Fast Time to Value** - Phase 1 complete in 4 weeks
6. **Clear Upgrade Path** - Each phase builds on previous
7. **Rich Learning Signal** - Verification chains provide detailed failure info

## Conclusion

This combined design synthesizes the best ideas from both OpenCode and ChatGPT approaches while making clear, research-backed decisions for each component.

**Key Differentiator**: This is not a single static design but a **progressive evolution** that starts with a simple, working prototype and grows into a sophisticated, data-driven system based on actual usage patterns.

**Immediate Next Step**: Implement Phase 1 (TinyRouter) with sqlite-vec, FastEmbed, static weights, and specialization-driven verification chains.
