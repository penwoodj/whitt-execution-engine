# Report 3: AdaptiveRouter Design

## Stack Overview

**Technology**: Python + Qdrant + FastEmbed + Ollama

This is the most scalable and search-native version, while still being local-first.

### Key Components

| Component | Technology | Purpose |
|-----------|-------------|----------|
| Vector Search | Qdrant | Hybrid retrieval + filtering |
| Embedding | FastEmbed | Lightweight vector encoding |
| Storage | Qdrant (local mode) | Multi-vector storage with metadata |
| Analytics | SQLite/DuckDB (separate) | Log storage and stats |
| Inference | Ollama API | Local LLM execution |

### Memory Footprint

| Item | Size (in-memory) | Size (on-disk) |
|-------|------------------|-----------------|
| Qdrant (100 rows) | ~50MB | ~100MB |
| FastEmbed model | ~80MB | ~80MB |
| SQLite/DuckDB | ~20MB | ~100MB |
| Python runtime | ~50MB | - |
| **Total** | **~200MB** | **~380MB** |

**Note**: Qdrant supports on-disk vectors to reduce RAM usage significantly.

## When to Choose This

Choose AdaptiveRouter when you want:
- ✅ Better online retrieval behavior
- ✅ More advanced hybrid search
- ✅ Easier growth to bigger model/task catalogs
- ✅ Memory tuning options (on-disk, quantization)
- ✅ Eventual multi-user local service
- ✅ Future-proof for large-scale deployment

## Core Idea

Use Qdrant as a live routing index with multi-stage query pipeline.

Store multiple views of each row:
- Dense routing vector
- Optional dense vector for tags only
- Optional sparse representation for lexical matching
- Metadata payload with all router columns

Then use one query pipeline with prefetch stages.

## ASCII Architecture Diagram

```
                 +----------------------+
                 |     task input       |
                 +----------+-----------+
                            |
                            v
                 +----------------------+
                 | trait extractor      |
                 | lang/type/strictness |
                 +----------+-----------+
                            |
                            v
                 +----------------------+
                 | FastEmbed query vec  |
                 +----------+-----------+
                            |
                            v
         +---------------------------------------------+
         | Qdrant Query API                            |
         | prefetch 1: dense semantic row search       |
         | prefetch 2: sparse / keyword row search     |
         | main query: filtered fusion over payload    |
         +-------------------+-------------------------+
                             |
                             v
                 +----------------------+
                 | top row candidates   |
                 +----------+-----------+
                            |
                            v
                 +----------------------+
                 | expand to models      |
                 | + log-aware ranking   |
                 +----------+-----------+
                            |
                            v
                 +----------------------+
                 | execute via Ollama   |
                 | try / verify / retry |
                 +----------+-----------+
                            |
                            v
                 +----------------------+
                 | write back logs      |
                 | update rolling stats |
                 +----------------------+
```

## Data Modeling in Qdrant

### One Point Per CSV Row

Each specialization becomes one point in the collection.

```python
from qdrant_client import QdrantClient
from qdrant_client.models import Distance, VectorParams, PointStruct

client = QdrantClient(url="http://localhost:6333")

# Create collection
client.create_collection(
    collection_name="specializations",
    vectors_config=VectorParams(
        size=384,  # FastEmbed dimension
        distance=Distance.COSINE
    )
)

# Insert row
client.upsert(
    collection_name="specializations",
    points=[PointStruct(
        id=row.id,
        vector=row.embedding,
        payload={
            "specialization_category": row.category,
            "synonyms": row.synonyms,
            "tags": row.tags,
            "preferred_input": row.input_shape,
            "preferred_output": row.output_shape,
            "rigidity": row.output_rigidity,
            "verifier_type": row.verification_need,
            "avoid_when": row.avoid_when,
            "top1_model": row.top1_model,
            "top2_model": row.top2_model,
            "top3_model": row.top3_model,
            "wildcard_model": row.wildcard_model,
            "top1_ctx": row.top1_ctx,
            "top2_ctx": row.top2_ctx,
            "top3_ctx": row.top3_ctx,
            "wildcard_ctx": row.wildcard_ctx,
            # ... more metadata
        }
    )]
)
```

### Named Vectors (Optional but Powerful)

Store multiple vectors per point for different retrieval strategies:

```python
client.create_collection(
    collection_name="specializations",
    vectors_config={
        "route_dense": VectorParams(size=384, distance=Distance.COSINE),
        "tags_dense": VectorParams(size=384, distance=Distance.COSINE),
        "failure_dense": VectorParams(size=384, distance=Distance.COSINE)  # Future use
    }
)
```

**Why named vectors?**
- `route_dense`: Semantic similarity for routing
- `tags_dense`: Tag-only similarity
- `failure_dense`: Learned from past failures (future)

### Sparse Payload for Lexical

For hybrid search, store sparse keywords:

```python
payload = {
    "keywords": ["json", "parsing", "structured"],
    "keyword_weights": [1.5, 1.2, 1.0],  # Optional weights
    "category": "structured_output_and_schema",
    # ... other fields
}
```

## Query Pipeline

Qdrant's hybrid/multi-stage query support makes it ideal for this use case.

### Stage 1: Dense Prefetch

Get semantic top 30 rows.

```python
from qdrant_client.models import Prefetch, QueryVector

dense_prefetch = Prefetch(
    query=QueryVector(vector=task_embedding),
    using="route_dense",
    limit=30
)
```

### Stage 2: Sparse/Lexical Prefetch

Get keyword top 30 rows.

```python
sparse_prefetch = Prefetch(
    query=None,
    filter={
        "must": [
            {"key": "keywords", "match": {"any": task_keywords}}
        ]
    },
    limit=30
)
```

### Stage 3: Constrained Main Query

Filter payload by constraints:

```python
from qdrant_client.models import Filter

constraint_filter = Filter(
    must=[
        # Output type must match
        {"key": "preferred_output", "match": {"value": task.output_type}},

        # Context window must be sufficient
        {
            "key": "top1_ctx",
            "range": {"gte": task.estimated_tokens}
        },

        # Must not be in avoid_when
        {"key": "avoid_when", "match": {"value": task.avoid_condition, "invert": True}}
    ]
)
```

### Stage 4: Full Query with Fusion

```python
results = client.search(
    collection_name="specializations",
    query_vector=task_embedding,
    prefetch=[dense_prefetch, sparse_prefetch],
    query_filter=constraint_filter,
    limit=10,
    with_payload=True,
    score_threshold=0.3  # Minimum similarity
)
```

## Why This Stack Helps

Unlike SQLite or DuckDB versions, Qdrant makes it easier to grow into:

1. **Multi-vector retrieval** - Named vectors for different purposes
2. **Hybrid retrieval** - Dense + sparse in one query
3. **Memory tuning** - On-disk vectors, quantization options
4. **Advanced search** - Prefetch stages, complex filters
5. **Future scale** - Easy path to distributed Qdrant

### Growth Path

| Phase | Router Size | Storage | RAM |
|-------|-------------|----------|------|
| Phase 1 | < 100 rows | In-memory | ~50MB |
| Phase 2 | 100-1000 rows | Persisted local | ~200MB |
| Phase 3 | 1000+ rows | Local server | ~500MB+ |
| Phase 4 | 10K+ rows | Distributed cluster | Multi-server |

## Log Integration

### Separate Stores

**Keep attempt logs in SQLite or DuckDB, NOT in Qdrant.**

This separation gives you:
- **Qdrant** = Low-latency candidate retrieval
- **SQLite/DuckDB** = Ranking stats and log aggregates

### Log Storage (SQLite)

```sql
CREATE TABLE attempt_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ts INTEGER NOT NULL,
    task_embedding BLOB,  -- For similarity search
    row_id INTEGER,
    model_name TEXT,
    success BOOLEAN,
    latency_ms REAL,
    fail_reason TEXT,
    FOREIGN KEY (row_id) REFERENCES router_rows(id)
);

-- Vector similarity index
CREATE VIRTUAL TABLE task_vecs USING vec0(
    id INTEGER PRIMARY KEY,
    embedding FLOAT[384]
);
```

## Online Ranking

This stack is ideal for an online ranker with decay.

### Recent Success Score

For each model, compute success on recent similar tasks:

```python
def compute_recent_score(model_name, task_embedding):
    # Find similar past attempts (last 200)
    similar = query_similar_attempts(task_embedding, limit=200)

    # Filter by model
    model_attempts = [a for a in similar if a.model == model_name]

    if not model_attempts:
        return 0.5  # Neutral prior

    # Compute stats
    stats = {
        'verified_success': sum(a.success for a in model_attempts) / len(model_attempts),
        'first_try': sum(a.success and a.try_idx == 1 for a in model_attempts) / len(model_attempts),
        'format_pass': sum(a.parse_pass or a.compile_pass for a in model_attempts) / len(model_attempts),
        'latency': np.mean([a.latency_ms for a in model_attempts])
    }

    # Weighted score
    recent_score = (
        0.55 * stats['verified_success'] +
        0.20 * stats['first_try'] +
        0.15 * stats['format_pass'] +
        0.10 * max(1 - stats['latency'] / 30000, 0)
    )

    return recent_score
```

### Final Score Blend

```
final_score =
  0.30 * row_relevance +
  0.10 * slot_bonus +
  0.05 * popularity_prior +
  0.10 * hardware_fit +
  0.45 * recent_score
```

**This is the strongest version of "logs heavily weighted."**

## Retry Loop

Same general loop as TinyRouter and LearnedRouter, with one more trick:

### Failure-Mode Memory

When a model fails, embed the **failure explanation** and store it.

Later, when similar failures recur, penalize models that fail in the same way for similar tasks.

```python
def log_failure(attempt, fail_reason):
    # Embed the failure reason
    failure_embedding = embed_text(fail_reason)

    # Store for future matching
    store_failure_pattern(
        model=attempt.model,
        task_embedding=attempt.task_embedding,
        failure_embedding=failure_embedding,
        fail_reason=fail_reason
    )
```

### Example Failure Patterns

| Failure Pattern | Example | Implication |
|---------------|----------|-------------|
| "keeps returning markdown fences around JSON" | Parser fail | Penalize for JSON tasks |
| "forgets required field X" | Schema fail | Penalize for structured output |
| "uses non-existent TypeScript API" | Compile fail | Penalize for TS code |
| "compiles but fails tests" | Test fail | Penalize for code-gen |

### Failure-Aware Penalty

```python
def compute_failure_penalty(model, task_embedding):
    # Find similar failures
    similar_failures = query_similar_failures(task_embedding, model)

    if not similar_failures:
        return 0.0  # No penalty

    # Penalty based on frequency and similarity
    total_failures = len(similar_failures)
    avg_similarity = np.mean([f.similarity for f in similar_failures])

    penalty = (total_failures / 100) * avg_similarity
    return min(penalty, 0.5)  # Cap at 0.5
```

## Memory-Minimizing Mode

Qdrant supports several options for reducing memory footprint:

### On-Disk Vectors

```python
client.create_collection(
    collection_name="specializations",
    vectors_config=VectorParams(
        size=384,
        distance=Distance.COSINE,
        on_disk=True  # Store vectors on disk
    )
)
```

**Trade-off**: Slower search, but RAM usage drops dramatically.

### Quantization

```python
from qdrant_client.models import ScalarQuantization, QuantizationConfig

client.update_collection(
    collection_name="specializations",
    quantization_config=QuantizationConfig(
        scalar=ScalarQuantization(
            type="int8",  # Or "fp16"
            quantile=0.99
        )
    )
)
```

**Benefits**:
- 4x less memory (int8 vs float32)
- Minimal accuracy loss
- Still supports approximate search

### Optimistic Indexing

```python
client.create_index(
    collection_name="specializations",
    field_name="specialization_category",
    field_schema="keyword"
)
```

**Trade-off**: Faster filtering, slower insertion.

## Local Rollout Path

### Phase 1: In-Memory Qdrant

```python
client = QdrantClient(
    location=":memory:",  # In-memory mode
    prefer_grpc=False
)
```

- Tiny collection
- All in RAM
- Fastest queries
- For development

### Phase 2: Persisted Local

```python
client = QdrantClient(
    path="./qdrant_data",  # Persisted storage
    prefer_grpc=False
)
```

- Persisted local path
- On-disk vectors (optional)
- Quantized vectors (optional)
- For small production

### Phase 3: Local Server

```bash
# Run Qdrant as local service
docker run -p 6333:6333 -v $(pwd)/qdrant_data:/qdrant/storage qdrant/qdrant
```

```python
client = QdrantClient(url="http://localhost:6333")
```

- Local server container
- Better resource isolation
- Easy to scale horizontally later
- For medium production

### Phase 4: Distributed (Future)

- Multiple Qdrant nodes
- Load balancing
- High availability
- For large production

## Why This Is Strong

1. **Strongest search layer** - Multi-vector, hybrid, prefetch
2. **Best future growth** - Clear path to scale
3. **Best hybrid retrieval** - Dense + sparse + filters in one query
4. **Easiest path to advanced retrieval** - Qdrant has rich features
5. **Good for combining semantic + sparse + filtered search**

## Weak Points

1. **More operationally complex** - Qdrant service to manage
2. **More than you need for tiny catalog** - Overkill for < 100 rows
3. **Less inspectable than SQLite-only** - No direct SQL queries
4. **Logs still belong in second store** - For best analytics

## Minimal-Hardware Recommendation

Use this only if:

- You expect router catalog to expand significantly (> 500 rows)
- You want online hybrid retrieval now
- You are okay with one more moving part (Qdrant service)
- You have 8GB+ RAM for in-memory mode

**Hardware**: 8GB+ RAM recommended (or 4GB with on-disk vectors)

**When to use**:
- Router catalog will grow large
- You need advanced retrieval now
- You're planning for multi-user scenarios
- Ready to invest in Qdrant learning curve

**When to start elsewhere**:
- Catalog is small (< 200 rows)
- Single-user scenario
- Want fastest time to working prototype
- Learning Qdrant is not priority

Start with TinyRouter or LearnedRouter, migrate to AdaptiveRouter when needed.
