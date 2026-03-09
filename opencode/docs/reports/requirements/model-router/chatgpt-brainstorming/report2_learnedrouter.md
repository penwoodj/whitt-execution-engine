# Report 2: LearnedRouter Design

## Stack Overview

**Technology**: Python + DuckDB + LanceDB + SentenceTransformers/FastEmbed + Ollama

This version is for when you want better analysis and better reranking without moving to a heavier distributed vector system.

### Key Components

| Component | Technology | Purpose |
|-----------|-------------|----------|
| Analytics Warehouse | DuckDB | Log storage and feature computation |
| Vector Store | LanceDB | Semantic retrieval + reranking |
| Embedding | SentenceTransformers/FastEmbed | Vector encoding |
| Reranker | Custom ML model | Learned ranking |
| Inference | Ollama API | Local LLM execution |

### Memory Footprint

| Item | Size |
|-------|------|
| DuckDB in-memory | ~20MB |
| LanceDB (100 rows) | ~50MB |
| Sentence Transformer | ~400MB (or ~80MB with FastEmbed) |
| Learned ranker | < 5MB |
| Python runtime | ~50MB |
| **Total** | **~525MB** (or ~205MB with FastEmbed) |

## When to Choose This

Choose LearnedRouter when you want:
- ✅ Better analytics on logs
- ✅ Better offline retraining
- ✅ More flexible reranking
- ✅ Cleaner batch jobs
- ✅ A path to learned ranking without much operational pain

## Core Idea

Split responsibilities:

1. **DuckDB** = Analytics warehouse + log feature store
2. **LanceDB** = Row vector store + fast semantic retrieval + optional reranking
3. **Python service** = Ranker, verifier, execution loop
4. **Ollama** = Local model runtime

## ASCII Architecture Diagram

```
        +---------------------------+
        |        task input         |
        +-------------+-------------+
                      |
                      v
        +---------------------------+
        | trait extractor           |
        | language/output/strictness|
        +-------------+-------------+
                      |
                      v
        +---------------------------+
        | embed query               |
        | FastEmbed or SBERT        |
        +-------------+-------------+
                      |
          +-----------+-----------+
          |                       |
          v                       v
+-------------------+   +------------------------+
| LanceDB vector    |   | DuckDB FTS/BM25        |
| top semantic rows |   | top lexical rows       |
+---------+---------+   +-----------+------------+
          \_________________   __________________/
                            \ /
                             v
              +------------------------------+
              | candidate row fusion         |
              | + hard constraint filtering  |
              +---------------+--------------+
                              |
                              v
              +------------------------------+
              | learned reranker             |
              | features from DuckDB logs    |
              +---------------+--------------+
                              |
                              v
              +------------------------------+
              | ranked models to try         |
              +---------------+--------------+
                              |
                              v
              +------------------------------+
              | Ollama execution loop        |
              | try / verify / retry / swap  |
              +---------------+--------------+
                              |
                              v
              +------------------------------+
              | write attempt logs to DuckDB |
              | nightly retrain ranker       |
              +------------------------------+
```

## What Changes vs TinyRouter

### The Big Difference

Instead of relying mostly on hand-tuned weights forever, this stack graduates to a **small learned ranker**.

That learned ranker can be:
- Logistic regression
- Small gradient boosted trees (XGBoost/LightGBM)
- Even just a calibrated linear model

**No giant ML pipeline needed.**

### Advantages

| Aspect | TinyRouter | LearnedRouter |
|---------|-------------|---------------|
| Ranking | Static weights | Learned from data |
| Analytics | Basic SQL queries | Rich aggregations |
| Reranking | Manual fusion | ML-based reranking |
| Offline training | Not applicable | Nightly retraining |
| Complexity | Low | Medium |

## Retrieval Flow

### A. Vector Retrieval in LanceDB

Store one vector per CSV row for routing text.

```python
import lancedb
from fastembed import TextEmbedding

# Initialize
model = TextEmbedding("all-MiniLM-L6-v2")
db = lancedb.connect("./lancedb")

# Create table
table = db.create_table(
    "specializations",
    schema=[
        ("row_id", int),
        ("vector", list[float]),
        ("category", str),
        ("synonyms", str),
        ("tags", str),
        # ... other metadata
    ]
)

# Insert rows
for row in csv_rows:
    embedding = next(model.embed([row.routing_text]))[0]
    table.add([{
        "row_id": row.id,
        "vector": embedding.tolist(),
        "category": row.category,
        # ... metadata
    }])
```

**Query for top semantic matches:**

```python
query_embedding = next(model.embed([task_text]))[0]

results = table.search(query_embedding).limit(10).to_pandas()
```

### B. FTS Retrieval in DuckDB

Load row text into DuckDB and build FTS index.

```python
import duckdb

conn = duckdb.connect(":memory:")

# Create table
conn.execute("""
    CREATE TABLE router_rows (
        row_id INTEGER,
        routing_text TEXT,
        category TEXT,
        tags TEXT
    )
""")

# Insert rows
conn.execute("INSERT INTO router_rows VALUES (?, ?, ?, ?)", rows)

# Build FTS index
conn.execute("PRAGMA create_fts_index(router_rows, 'fts_idx', 'routing_text')")
```

**Query for top lexical matches:**

```python
results = conn.execute("""
    SELECT row_id, rank
    FROM fts_idx
    WHERE routing_text MATCH ?
    ORDER BY rank
    LIMIT ?
""", [query_text, K_sparse]).fetchall()
```

**Important**: DuckDB FTS indexes do not auto-refresh when source table changes. Rebuild whenever CSV changes, not on every log write.

### C. Optional Reranking

If you want a stronger middle stage:

**Option 1: LanceDB Reranking**

LanceDB explicitly supports built-in rerankers and custom rerankers.

```python
from lancedb.rerankers import CrossEncoderReranker

reranker = CrossEncoderReranker("cross-encoder-model")

results = table.search(query_embedding)\
    .limit(30)\
    .rerank(reranker=reranker, query_text=task_text)\
    .limit(10)
```

**Option 2: Custom Reranker**

```python
class CustomReranker:
    def __init__(self, model):
        self.model = model

    def rerank(self, results, query_text):
        # Extract features
        features = self.extract_features(results, query_text)

        # Predict scores
        scores = self.model.predict_proba(features)

        # Reorder results
        results['score'] = scores
        return results.sort_values('score', ascending=False)
```

## Learned Feature Set

For each `(task, row, model)` candidate, build features like:

### Dense Features
- Dense similarity score
- Normalized dense score (0-1)

### Sparse Features
- BM25 score
- Keyword match count
- Exact match on category
- Exact match on keywords

### Row Features
- Slot (`top1`, `top2`, `top3`, `wildcard`)
- Slot bonus (1.0, 0.82, 0.68, 0.55)
- Output rigidity match (binary)
- Context fit (0-1)
- Verification severity match (binary)

### Task Features
- Input length bucket (0-100, 100-1000, 1000-10000, >10000)
- Output type (one-hot encoded)
- Language (one-hot encoded)
- Edit vs scratch (binary)
- Context ratio: `estimated_needed_ctx / model_ctx`

### Log Features (Per Model)
- Verified success rate on similar tasks
- First-try success rate on similar tasks
- Average retries on similar tasks
- Compile/test/parse pass rate on similar tasks
- Average latency on similar tasks (normalized)
- Recent decay-weighted success

### Hardware Features
- Average load time (normalized)
- Average inference time (normalized)
- OOM history (binary)
- Context overflow history (binary)

**Total features**: ~30-40 features per candidate

## Learned Score

### Training

```python
from sklearn.linear_model import LogisticRegression

# Build feature matrix
X = build_feature_matrix(candidates)
# Binary labels: success (1) or failure (0)
y = extract_labels(logs)

# Train model
model = LogisticRegression(max_iter=1000)
model.fit(X, y)
```

### Inference

```python
def score_candidate(candidate, model):
    features = extract_features(candidate)
    success_prob = model.predict_proba([features])[0][1]

    # Blend with other scores
    final_score = (
        0.15 * candidate.popularity_prior +
        0.15 * candidate.hardware_fit +
        0.70 * success_prob
    )

    return final_score
```

### Model Evolution

**Early phase** (few logs):
- Use hand-tuned weights (like TinyRouter)

**Middle phase** (hundreds of logs):
- Train linear model
- Blend with hand weights

**Mature phase** (thousands of logs):
- Use gradient boosted trees
- Let ML dominate ranking
- Reduce hand weight contribution

## Similarity-Aware Logs

In DuckDB, keep both raw logs and precomputed aggregates.

### Raw Logs Table

```sql
CREATE TABLE attempt_logs (
    attempt_id BIGINT PRIMARY KEY,
    ts TIMESTAMP,
    task_text TEXT,
    task_embedding FLOAT[384],
    row_id INTEGER,
    model_name TEXT,
    try_index INTEGER,
    task_type TEXT,
    language TEXT,
    output_type TEXT,
    verifier_type TEXT,
    success BOOLEAN,
    fail_reason TEXT,
    latency_ms FLOAT,
    token_count INTEGER
);

-- Create vector index for similarity search
PRAGMA create_fts_index(attempt_logs, 'task_vec_idx',
                        'task_embedding', metric='cosine');
```

### Aggregates

Precompute per-group statistics:

```sql
-- Per model × specialization
CREATE AGGREGATE stats_model_spec AS
SELECT
    model_name,
    specialization_id,
    COUNT(*) as total_attempts,
    AVG(CAST(success AS INTEGER)) as success_rate,
    AVG(CAST(try_index = 1 AND success AS INTEGER)) as first_try_rate,
    AVG(try_index) as avg_tries,
    AVG(latency_ms) as avg_latency
FROM attempt_logs
GROUP BY model_name, specialization_id;

-- Per model × output type
CREATE AGGREGATE stats_model_output AS
SELECT model_name, output_type, ...
GROUP BY model_name, output_type;

-- Per model × verifier
CREATE AGGREGATE stats_model_verifier AS
SELECT model_name, verifier_type, ...
GROUP BY model_name, verifier_type;
```

### Similarity Buckets

To make logs highly relevant to "similar prompts" without doing full nearest-neighbor over every old task:

1. **Compute task embedding** for each attempt
2. **Cluster embeddings offline** (e.g., K-means with 100 clusters)
3. **Store `task_cluster_id`** in logs
4. **Keep per-cluster model success stats**

```sql
-- Precomputed clusters
CREATE TABLE task_clusters (
    cluster_id INTEGER,
    centroid FLOAT[384]
);

-- Per-cluster stats
CREATE AGGREGATE stats_cluster AS
SELECT
    cluster_id,
    model_name,
    COUNT(*) as total_attempts,
    AVG(CAST(success AS INTEGER)) as cluster_success_rate
FROM attempt_logs
GROUP BY cluster_id, model_name;
```

### Cluster-Aware Ranking

```sql
-- Get cluster for current task
SELECT cluster_id FROM task_clusters
ORDER BY array_distance(centroid, ?::FLOAT[384])
LIMIT 1;

-- Get cluster-local stats
SELECT model_name, cluster_success_rate
FROM stats_cluster
WHERE cluster_id = ?

-- Get specialization stats
SELECT model_name, success_rate
FROM stats_model_spec
WHERE specialization_id = ?

-- Get global stats
SELECT model_name, success_rate
FROM stats_model_global

-- Blend
log_success =
  0.45 * cluster_success +
  0.30 * specialization_success +
  0.15 * verifier_success +
  0.10 * global_success
```

**This is a strong compromise between precision and simplicity.**

## Retry / Verification Policy

### Specialization-Specific Retry Budgets

Different tasks deserve different retry budgets:

| Task Type | Max Tries | Rationale |
|-----------|-----------|------------|
| Strict JSON | 2 | Format tasks fail fast |
| Code generation | 3 | Can benefit from repair cycles |
| Summarization | 2 | Rarely improves after retries |
| Debugging | 4 | Benefits from multiple repair attempts |

### Implementation

```python
RETRY_BUDGETS = {
    'structured_output': 2,
    'code_generation': 3,
    'summarization': 2,
    'debugging': 4,
    'default': 2
}

def get_max_tries(specialization):
    for pattern, tries in RETRY_BUDGETS.items():
        if pattern in specialization.category.lower():
            return tries
    return RETRY_BUDGETS['default']
```

## Nightly Adaptation Job

Every night, run these jobs:

```python
async def nightly_adaptation():
    # 1. Rebuild aggregates
    rebuild_aggregates()

    # 2. Retrain ranker
    retrain_ranker()

    # 3. Refresh per-model hardware stats
    refresh_hardware_stats()

    # 4. Recalculate popularity prior decay
    decay_popularity()

    # 5. Snapshot new router artifacts
    snapshot_artifacts()
```

### 1. Rebuild Aggregates

```python
def rebuild_aggregates():
    conn.execute("DROP TABLE IF EXISTS stats_model_spec")
    conn.execute("""
        CREATE AGGREGATE stats_model_spec AS
        SELECT model_name, specialization_id,
               COUNT(*) as total_attempts,
               AVG(CAST(success AS INTEGER)) as success_rate
        FROM attempt_logs
        GROUP BY model_name, specialization_id
    """)
```

### 2. Retrain Ranker

```python
def retrain_ranker():
    # Extract features from recent logs
    features = extract_features_from_logs(last_n=10000)

    # Train new model
    model = train_gradient_boosted_trees(features)

    # Save model
    model.save("models/ranker_v2.joblib")
```

### 3. Refresh Hardware Stats

```python
def refresh_hardware_stats():
    models = get_all_models()

    for model in models:
        stats = measure_hardware_stats(model)
        update_model_catalog(model.name, stats)
```

### 4. Decay Popularity Prior

```python
def decay_popularity():
    conn.execute("""
        UPDATE model_catalog
        SET popularity_prior = popularity_prior * 0.99
    """)
```

### 5. Snapshot Artifacts

```python
def snapshot_artifacts():
    import shutil
    from datetime import datetime

    timestamp = datetime.now().strftime("%Y%m%d")
    snapshot_dir = f"snapshots/router_{timestamp}"

    shutil.copytree("models", f"{snapshot_dir}/models")
    shutil.copy("duckdb.db", f"{snapshot_dir}/duckdb.db")
    shutil.copytree("lancedb", f"{snapshot_dir}/lancedb")

    # Keep last 7 snapshots
    cleanup_old_snapshots(keep=7)
```

## Why This Is Strong

1. **Much better use of logs** - Aggregated, similarity-aware statistics
2. **Much better offline analysis** - DuckDB analytics capabilities
3. **Cleaner feature engineering** - Structured feature set in one place
4. **Still local-first** - No external services needed
5. **Not operationally heavy** - Nightly batch jobs, not real-time ML

## Weak Points

1. **More moving parts** - DuckDB + LanceDB + Python ML
2. **Two storage systems** - More complexity than SQLite-only
3. **FTS refresh is batch-oriented** - Manual rebuilds required
4. **Slightly more RAM and disk** - Two stores + ML model

## Minimal-Hardware Recommendation

This still works well on modest hardware if:

- LanceDB collection is small (< 1000 rows)
- DuckDB is local file-based (in-memory mode optional)
- Embeddings are small (use FastEmbed vs full Sentence Transformers)
- Retrain ranker offline, not in real-time

**Hardware**: 8GB+ RAM recommended

**When to use**:
- You want best "practical local ranking system"
- You have thousands of logged attempts
- You're ready to graduate from static weights
- Not yet ready for full distributed vector DB
