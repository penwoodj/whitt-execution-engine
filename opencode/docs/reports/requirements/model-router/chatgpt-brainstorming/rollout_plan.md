# Recommended Rollout Plan

## Overview

ChatGPT recommends a phased rollout strategy to gradually evolve from a simple prototype to a sophisticated, log-driven routing system.

## The Three Phases

```
Phase A: TinyRouter (MVP)
    │
    └─▶ Phase B: LearnedRouter (Analytics + ML)
           │
           └─▶ Phase C: AdaptiveRouter (Advanced Retrieval)
```

### Timeline Estimate

| Phase | Duration | Key Milestones |
|-------|----------|----------------|
| Phase A | 2-4 weeks | Working prototype, logs flowing |
| Phase B | 4-8 weeks | Learned ranker, analytics |
| Phase C | 8-12 weeks | Full hybrid retrieval, online adaptation |

## Phase A: TinyRouter (MVP)

### Goal

Prove that the CSV can actually route tasks, validators work, and retry/switch logic works.

### Objectives

- [x] Prove CSV routing works
- [x] Verify validator chains work
- [x] Prove retry/switch logic works
- [x] Start collecting logs immediately

### Stack

- SQLite + sqlite-vec + FTS5 + FastEmbed + Ollama
- Single-file persistence
- Hand-tuned ranking weights

### Success Criteria

1. **Routing Accuracy**
   - Top 3 specializations contain correct category in > 80% of tasks
   - Avoid rows that would fail constraints

2. **Validator Coverage**
   - At least 10 verifier types implemented
   - Each specialization has associated verifier

3. **Log Quality**
   - 1000+ attempts logged
   - All fields populated (embedding, traits, outcomes)

4. **Retry Effectiveness**
   - > 70% of tasks succeed within 2 models
   - > 90% of tasks succeed within 3 models

### Phase Exit Criteria

Before moving to Phase B:

- [ ] Stable prototype running for 2+ weeks
- [ ] 500+ successful task completions
- [ ] At least 50 different specializations tested
- [ ] No critical bugs in retry/switch loop
- [ ] Clear pain points identified

### Deliverables

- Working TinyRouter system
- CSV data imported to SQLite
- Initial set of verifiers
- 500-1000 logged attempts
- Performance baseline metrics

## Phase B: LearnedRouter

### Goal

Move ranking logic from hand-tuned weights to learned model, with offline analytics.

### Objectives

- Keep your current CSV (no changes needed)
- Keep your validators (no changes needed)
- Keep your retry loop (no changes needed)
- Replace hand weights with a learned ranker

### Stack Migration

| Component | From | To |
|-----------|-------|-----|
| Storage | SQLite only | DuckDB + LanceDB |
| Ranking | Static weights | Learned model (XGBoost/LightGBM) |
| Retrieval | Manual fusion | LanceDB reranking |
| Analytics | Basic SQL | DuckDB aggregations |

### Migration Steps

#### 1. Export Logs from SQLite

```python
import sqlite3
import duckdb

# Export from SQLite
conn_sqlite = sqlite3.connect("tinyrouter.db")
cursor = conn_sqlite.cursor()

cursor.execute("SELECT * FROM task_logs")
logs = cursor.fetchall()

# Import to DuckDB
conn_duckdb = duckdb.connect("learnedrouter.db")
conn_duckdb.execute("CREATE TABLE task_logs AS SELECT * FROM logs")
```

#### 2. Set Up LanceDB

```python
import lancedb

# Create LanceDB instance
db = lancedb.connect("./lancedb")

# Import rows from SQLite
table = db.create_table("specializations", schema=...)
table.add(rows_from_sqlite)
```

#### 3. Feature Extraction

```python
def build_feature_matrix():
    features = []

    for log in logs:
        feature = {
            # Dense similarity
            'dense_sim': log.dense_similarity,

            # Sparse score
            'sparse_score': log.bm25_score,

            # Slot bonus
            'slot_bonus': log.slot_bonus,

            # Output type match
            'output_type_match': int(log.output_type == log.expected_output),

            # Language match
            'language_match': int(log.language == log.expected_language),

            # Context fit
            'context_fit': min(log.model_ctx / log.estimated_tokens, 1.0),

            # Log statistics
            'success_rate': log.success_rate,
            'first_try_rate': log.first_try_rate,
            'avg_latency': log.avg_latency,
        }
        features.append(feature)

    return pd.DataFrame(features)
```

#### 4. Train Model

```python
from sklearn.model_selection import train_test_split
from xgboost import XGBClassifier

# Prepare data
X = build_feature_matrix()
y = X.pop('success')

# Split data
X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2)

# Train model
model = XGBClassifier(
    n_estimators=100,
    max_depth=6,
    learning_rate=0.1
)
model.fit(X_train, y_train)

# Evaluate
accuracy = model.score(X_test, y_test)
print(f"Model accuracy: {accuracy:.2%}")

# Save model
model.save("models/ranker_v1.json")
```

#### 5. Update Ranking Logic

```python
def rank_candidates_learned(candidates, model):
    # Extract features
    features = [extract_features(c) for c in candidates]

    # Predict success probability
    success_probs = model.predict_proba(features)[:, 1]

    # Blend with other scores
    for i, candidate in enumerate(candidates):
        candidate.final_score = (
            0.15 * candidate.popularity_prior +
            0.15 * candidate.hardware_fit +
            0.70 * success_probs[i]
        )

    return sorted(candidates, key=lambda c: c.final_score, reverse=True)
```

### Success Criteria

1. **Model Performance**
   - Learned model accuracy > 85%
   - Better than hand-tuned weights by > 5%

2. **Retrieval Quality**
   - Hybrid retrieval (dense + sparse) finds relevant rows
   - Top 3 accuracy maintained or improved

3. **Analytics Capability**
   - Dashboard working with DuckDB
   - Per-specialization metrics computed
   - Hardware profile tracking working

4. **Nightly Jobs**
   - Aggregates rebuilt automatically
   - Model retrained automatically
   - Snapshots created automatically

### Phase Exit Criteria

Before moving to Phase C:

- [ ] Learned model outperforming hand weights
- [ ] 5000+ attempts logged
- [ ] At least 100 different specializations tested
- [ ] Analytics pipeline stable
- [ ] Nightly jobs running for 2+ weeks
- [ ] Clear need for better retrieval (e.g., scale issues)

### Deliverables

- LearnedRouter system with DuckDB + LanceDB
- Trained ranking model (v1, v2, ...)
- Analytics dashboard
- 5000+ logged attempts
- Nightly adaptation pipeline

## Phase C: AdaptiveRouter

### Goal

Upgrade retrieval to Qdrant for better hybrid search and online adaptation.

### Objectives

- Keep learned ranker from Phase B
- Keep validators and retry loop
- Upgrade retrieval layer to Qdrant
- Add failure-mode memory

### Stack Migration

| Component | From | To |
|-----------|-------|-----|
| Vector Store | LanceDB | Qdrant |
| Retrieval | Manual | Qdrant query API |
| Hybrid Search | Custom fusion | Native Qdrant hybrid |
| Storage | DuckDB only | Qdrant + SQLite (logs) |

### Migration Steps

#### 1. Set Up Qdrant

```bash
# Local mode (development)
docker run -p 6333:6333 qdrant/qdrant

# Or persisted mode (production)
docker run -p 6333:6333 -v $(pwd)/qdrant:/qdrant/storage qdrant/qdrant
```

#### 2. Import Data to Qdrant

```python
from qdrant_client import QdrantClient

client = QdrantClient(url="http://localhost:6333")

# Create collection
client.create_collection(
    collection_name="specializations",
    vectors_config={
        "route_dense": VectorParams(size=384, distance=Distance.COSINE),
        "tags_dense": VectorParams(size=384, distance=Distance.COSINE)
    }
)

# Import rows
for row in rows_from_lancedb:
    client.upsert(
        collection_name="specializations",
        points=[PointStruct(
            id=row.id,
            vector={
                "route_dense": row.route_embedding,
                "tags_dense": row.tags_embedding
            },
            payload=row.metadata
        )]
    )
```

#### 3. Update Retrieval Logic

```python
from qdrant_client.models import Prefetch, Filter

def retrieve_with_qdrant(task):
    # Build prefetched queries
    dense_prefetch = Prefetch(
        query=QueryVector(vector=task.embedding),
        using="route_dense",
        limit=30
    )

    sparse_prefetch = Prefetch(
        query=None,
        filter=build_keyword_filter(task.keywords),
        limit=30
    )

    # Build constraint filter
    constraint_filter = Filter(
        must=[
            {"key": "preferred_output", "match": {"value": task.output_type}},
            {
                "key": "top1_ctx",
                "range": {"gte": task.estimated_tokens}
            }
        ]
    )

    # Execute query
    results = client.search(
        collection_name="specializations",
        query_vector=task.embedding,
        prefetch=[dense_prefetch, sparse_prefetch],
        query_filter=constraint_filter,
        limit=10
    )

    return results
```

#### 4. Add Failure-Mode Memory

```python
def log_failure_with_pattern(attempt, fail_reason):
    # Embed failure reason
    failure_embedding = embed_text(fail_reason)

    # Store in SQLite for similarity search
    conn.execute("""
        INSERT INTO failure_patterns
        (model_name, task_embedding, failure_embedding, fail_reason, ts)
        VALUES (?, ?, ?, ?, ?)
    """, (
        attempt.model,
        attempt.task_embedding,
        failure_embedding,
        fail_reason,
        datetime.now()
    ))

    # Store in Qdrant for fast retrieval
    client.upsert(
        collection_name="failures",
        points=[PointStruct(
            id=generate_id(),
            vector=failure_embedding,
            payload={
                "model": attempt.model,
                "reason": fail_reason,
                "task_id": attempt.task_id
            }
        )]
    )

def compute_failure_penalty(model, task_embedding):
    # Find similar failures
    results = client.search(
        collection_name="failures",
        query_vector=task_embedding,
        query_filter=Filter(
            must=[{"key": "model", "match": {"value": model}}]
        ),
        limit=10
    )

    if not results:
        return 0.0

    # Compute penalty
    penalty = sum(r.score for r in results) / len(results)
    return min(penalty, 0.5)
```

#### 5. Enable Online Ranking Updates

```python
def update_ranking_online(model, task, success):
    # Update recent statistics
    update_recent_stats(model, task, success)

    # Decay old statistics (exponential moving average)
    decay_old_stats()

    # Optionally, update model in real-time
    # (for online learning algorithms)
```

### Success Criteria

1. **Retrieval Performance**
   - Query latency < 50ms (local Qdrant)
   - Top 3 accuracy maintained or improved
   - Hybrid search working correctly

2. **Failure-Aware Ranking**
   - Models penalized for repeated failure patterns
   - Failure patterns identified and stored
   - Success rate improved by > 5%

3. **Scalability**
   - System handles 1000+ specializations
   - Query performance degrades gracefully
   - Memory usage stable with on-disk vectors

4. **Online Adaptation**
   - Rankings adjust in real-time based on recent performance
   - Nightly retraining reduced or eliminated
   - System improves continuously

### Phase Exit Criteria

- [ ] Qdrant retrieval working correctly
- [ ] Failure-mode memory improving rankings
- [ ] System handling scale (1000+ rows)
- [ ] Online adaptation effective
- [ ] Clear path to distributed deployment

### Deliverables

- AdaptiveRouter system with Qdrant
- Failure-mode memory system
- Online adaptation pipeline
- Hybrid query optimization
- Distributed deployment playbook

## Rollout Timeline

### Months 1-2: Phase A (TinyRouter)

```
Week 1-2:  Setup and Implementation
  - Install SQLite, FastEmbed, Ollama
  - Import CSV data
  - Implement basic routing
  - Implement verifiers (5-10 types)

Week 3-4:  Testing and Refinement
  - Test routing on various tasks
  - Refine ranking weights
  - Implement retry/switch loop
  - Start logging

Week 5-6:  Data Collection
  - Collect 500-1000 attempts
  - Analyze log quality
  - Identify pain points
  - Prepare for Phase B
```

### Months 3-5: Phase B (LearnedRouter)

```
Week 1-2:  Migration Planning
  - Set up DuckDB + LanceDB
  - Export logs from SQLite
  - Import to new storage

Week 3-6:  Feature Engineering
  - Build feature extraction pipeline
  - Train initial model
  - Evaluate vs hand weights

Week 7-10: Analytics Pipeline
  - Build DuckDB aggregates
  - Implement nightly jobs
  - Create basic dashboard

Week 11-14:  Refinement
  - Collect 5000+ attempts
  - Retrain model weekly
  - Improve features
  - Prepare for Phase C
```

### Months 6-9: Phase C (AdaptiveRouter)

```
Week 1-4:  Qdrant Setup
  - Install and configure Qdrant
  - Import data to Qdrant
  - Implement hybrid queries

Week 5-8:  Failure Memory
  - Add failure pattern tracking
  - Implement failure-aware penalties
  - Test impact on ranking

Week 9-12:  Optimization
  - Tune Qdrant parameters
  - Implement on-disk vectors
  - Add quantization
  - Scale testing

Week 13-16:  Productionization
  - Deploy to production
  - Monitor performance
  - Implement distributed path
  - Documentation
```

## Risk Mitigation

### Phase A Risks

| Risk | Impact | Mitigation |
|-------|---------|-------------|
| Poor routing accuracy | High | Start with hand-crafted test cases, tune weights |
| Verifier complexity | Medium | Start with simple validators, add over time |
| Log quality issues | High | Log all fields, validate schema early |
| Ollama availability | Low | Have backup model runtime |

### Phase B Risks

| Risk | Impact | Mitigation |
|-------|---------|-------------|
| Model overfitting | High | Regularization, cross-validation |
| Feature engineering complexity | Medium | Start with simple features, iterate |
| Nightly job failures | Medium | Monitoring, alerts, rollback plans |
| DuckDB performance | Low | Use in-memory mode for testing |

### Phase C Risks

| Risk | Impact | Mitigation |
|-------|---------|-------------|
| Qdrant operational complexity | High | Use local mode first, containerize later |
| Failure pattern noise | Medium | Validate patterns before penalizing |
| Online adaptation instability | High | Conservative updates, A/B testing |
| Migration complexity | Low | Keep old system running in parallel |

## Rollback Strategy

If any phase fails or underperforms:

1. **Keep previous system running** - Don't delete old code
2. **A/B test new system** - Compare metrics before full migration
3. **Gradual cutover** - Route % of traffic to new system
4. **Rollback plan** - Document steps to revert to previous phase

## Success Metrics

Track these metrics throughout rollout:

| Metric | Phase A Target | Phase B Target | Phase C Target |
|--------|---------------|---------------|---------------|
| Routing accuracy (top 3) | 80% | 85% | 90% |
| Tasks succeeding in 2 models | 70% | 75% | 80% |
| Tasks succeeding in 3 models | 90% | 92% | 95% |
| Average latency per task | < 10s | < 8s | < 5s |
| System uptime | 95% | 98% | 99% |
| Log coverage | 100 fields | 120 fields | 150 fields |

## Conclusion

This phased rollout approach ensures:

1. **Quick validation** - Phase A proves the concept quickly
2. **Data-driven evolution** - Each phase uses logs from previous phase
3. **Controlled complexity** - Add complexity only when needed
4. **Clear exit criteria** - Know when to move to next phase
5. **Rollback safety** - Can always revert to previous phase

**Start with Phase A (TinyRouter) today.**
