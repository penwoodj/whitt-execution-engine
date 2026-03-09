# Model Router System Design 1: Python/SQLite/NumPy Stack

## Executive Summary

A lightweight, locally-run model routing system that uses vector similarity matching on CSV metadata to intelligently select and rank local LLM models. The system incorporates log-based feedback to improve routing decisions over time, with minimal hardware requirements suitable for consumer-grade hardware.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                         Input Text Prompt                            │
└────────────────────────┬────────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────────────┐
│                   Vector Encoder Service                              │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  Sentence-BERT Mini-LM (120MB)                              │   │
│  │  - Encodes prompt to 384-dim vector                         │   │
│  │  - Pre-computed embeddings for CSV rows                     │   │
│  └──────────────────────────────────────────────────────────────┘   │
└────────────────────┬────────────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────────────┐
│                   Similarity Search Engine                            │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  NumPy Cosine Similarity Matrix                               │   │
│  │  - Input embedding vs pre-computed vectors                    │   │
│  │  - Returns top-K closest specializations                      │   │
│  └──────────────────────────────────────────────────────────────┘   │
└────────────────────┬────────────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────────────┐
│                   Ranking Engine (Multi-Stage)                       │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  Stage 1: Popularity Rank (CSV static)                       │   │
│  │  Stage 2: Log-Based Penalty (Dynamic)                         │   │
│  │  Stage 3: Context Size Match                                  │   │
│  │  Stage 4: Final Weighted Score                               │   │
│  └──────────────────────────────────────────────────────────────┘   │
└────────────────────┬────────────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────────────┐
│                   Agentic Execution Engine                            │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  Retry Controller (Configurable N tries)                     │   │
│  │  - Try model 1 (top rank) → verify result                     │   │
│  │  - If fail: Try model 2 → verify result                      │   │
│  │  - Continue until success or list exhausted                  │   │
│  └──────────────────────────────────────────────────────────────┘   │
└────────────────────┬────────────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────────────┐
│                   Log Collector & Analyzer                          │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  SQLite: logs.db                                             │   │
│  │  - Prompt hash, model used, attempts, success, latency      │   │
│  │  - Aggregated by specialization_category                     │   │
│  └──────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
```

## Technology Stack

### Core Components

| Component | Technology | Memory | Rationale |
|-----------|-----------|---------|-----------|
| Vector Encoder | Sentence-BERT (all-MiniLM-L6-v2) | ~120MB | Small, fast, good semantic encoding |
| Numerical Computing | NumPy | ~50MB | Efficient matrix operations |
| Database | SQLite3 | <5MB | Zero-config, embedded |
| CSV Handling | pandas | ~100MB | Easy data manipulation |
| HTTP Server | FastAPI | ~50MB | Fast async API |

### Total Memory Footprint
- **Minimal runtime**: ~325MB (cold start, no models loaded)
- **With embeddings**: ~450MB (pre-computed vectors in RAM)

## Data Schema

### SQLite Schema

```sql
-- Model specializations (loaded from CSV)
CREATE TABLE specializations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    row_kind TEXT NOT NULL,
    specialization_category TEXT NOT NULL,
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

-- Pre-computed embeddings (384-dim vectors stored as binary)
CREATE TABLE embeddings (
    specialization_id INTEGER PRIMARY KEY,
    embedding BLOB NOT NULL,  -- 384 * 4 bytes = 1536 bytes
    FOREIGN KEY (specialization_id) REFERENCES specializations(id)
);

-- Execution logs
CREATE TABLE execution_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    prompt_hash TEXT NOT NULL,  -- SHA256 for deduplication
    prompt_prefix TEXT NOT NULL,  -- First 200 chars for reference
    selected_models TEXT NOT NULL,  -- JSON array of model names tried
    successful_model TEXT,  -- Model that produced valid output
    attempts INTEGER DEFAULT 0,
    success BOOLEAN,
    latency_ms REAL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_prompt_hash (prompt_hash),
    INDEX idx_specialization (specialization_category)
);

-- Aggregated statistics for ranking
CREATE TABLE model_stats (
    model_name TEXT PRIMARY KEY,
    total_attempts INTEGER DEFAULT 0,
    successes INTEGER DEFAULT 0,
    failures INTEGER DEFAULT 0,
    avg_attempts_to_success REAL DEFAULT 0,
    penalty_score REAL DEFAULT 0.0,
    last_updated DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

## Core Algorithm: Vector Similarity + Ranking

### Step 1: Embedding Generation

```python
from sentence_transformers import SentenceTransformer
import numpy as np

class VectorEncoder:
    def __init__(self):
        self.model = SentenceTransformer('all-MiniLM-L6-v2')
        self.dim = 384

    def encode_text(self, text: str) -> np.ndarray:
        """Encode text to 384-dim vector"""
        return self.model.encode(text, normalize_embeddings=True)

    def encode_specialization(self, row: dict) -> np.ndarray:
        """Combine multiple fields for richer embedding"""
        text_parts = [
            row['specialization_category'],
            row['specialization_synonyms'],
            row['router_keywords'],
            row['primary_strength'],
            row['secondary_strengths']
        ]
        combined = ' | '.join(filter(None, text_parts))
        return self.encode_text(combined)
```

### Step 2: Similarity Search

```python
def find_closest_specializations(
    input_embedding: np.ndarray,
    all_embeddings: np.ndarray,  # Shape: (N, 384)
    top_k: int = 10
) -> list[tuple[int, float]]:
    """Find top-K closest specializations using cosine similarity"""
    # Cosine similarity for normalized vectors = dot product
    similarities = np.dot(all_embeddings, input_embedding)
    top_indices = np.argsort(similarities)[::-1][:top_k]
    return [(int(i), float(similarities[i])) for i in top_indices]
```

### Step 3: Ranking Algorithm

```python
def rank_models(
    specialization_id: int,
    similarity_score: float,
    conn: sqlite3.Connection,
    config: dict
) -> list[tuple[str, float]]:
    """
    Multi-stage ranking:
    1. Base similarity score
    2. Popularity weight (from CSV order)
    3. Log penalty (reduce models that frequently retry)
    4. Context size match (optional)
    """
    cursor = conn.cursor()

    # Get specialization data
    cursor.execute('SELECT * FROM specializations WHERE id = ?', (specialization_id,))
    spec = cursor.fetchone()

    # Get models for this specialization (top1, top2, top3, wildcard)
    models = [
        (spec['top1_model'], 1.0, spec['top1_ctx']),
        (spec['top2_model'], 0.9, spec['top2_ctx']),
        (spec['top3_model'], 0.8, spec['top3_ctx']),
        (spec['wildcard_model'], 0.6, spec['wildcard_ctx'])
    ]

    ranked = []
    for model_name, base_rank, ctx_size in models:
        if not model_name:
            continue

        # Stage 1: Base score from similarity and position
        base_score = similarity_score * base_rank

        # Stage 2: Apply log-based penalty
        cursor.execute(
            'SELECT penalty_score FROM model_stats WHERE model_name = ?',
            (model_name,)
        )
        result = cursor.fetchone()
        penalty = result[0] if result else 0.0

        # Penalty: 0.0 = no penalty, 1.0 = worst
        # Higher penalty = lower score
        log_adjusted_score = base_score * (1.0 - penalty)

        # Stage 3: Context size preference (optional)
        ctx_weight = config.get('context_weight', 0.1)
        ctx_bonus = min(ctx_size / 524288, 1.0) * ctx_weight  # Normalize to 512K

        # Stage 4: Final weighted score
        final_score = log_adjusted_score + ctx_bonus

        ranked.append((model_name, final_score))

    # Sort by final score descending
    ranked.sort(key=lambda x: x[1], reverse=True)
    return ranked
```

### Step 4: Log Penalty Calculation

```python
def update_model_stats(
    conn: sqlite3.Connection,
    model_name: str,
    prompt_hash: str,
    attempts: int,
    success: bool,
    latency_ms: float
):
    """
    Update statistics and calculate penalty score.
    Penalty formula:
    - High retry count for similar prompts = higher penalty
    - Success rate weighted by prompt similarity
    """
    cursor = conn.cursor()

    # Get current stats
    cursor.execute(
        'SELECT total_attempts, successes, failures FROM model_stats WHERE model_name = ?',
        (model_name,)
    )
    current = cursor.fetchone()

    if not current:
        cursor.execute(
            '''INSERT INTO model_stats
               (model_name, total_attempts, successes, failures, avg_attempts_to_success)
               VALUES (?, ?, ?, ?, ?)''',
            (model_name, 1, 1 if success else 0, 0 if success else 1, attempts)
        )
    else:
        total_attempts, successes, failures = current
        new_total = total_attempts + 1
        new_successes = successes + (1 if success else 0)
        new_failures = failures + (0 if success else 1)

        # Calculate rolling average attempts to success
        if success:
            current_avg = cursor.execute(
                'SELECT avg_attempts_to_success FROM model_stats WHERE model_name = ?',
                (model_name,)
            ).fetchone()[0] or 0
            new_avg = ((current_avg * successes) + attempts) / new_successes
        else:
            new_avg = cursor.execute(
                'SELECT avg_attempts_to_success FROM model_stats WHERE model_name = ?',
                (model_name,)
            ).fetchone()[0] or 0

        # Calculate penalty score (0.0 to 1.0)
        # Formula: weighted combination of:
        # 1. Failure rate (higher = worse)
        # 2. Avg attempts per success (higher = worse)
        failure_rate = new_failures / new_total
        attempt_penalty = min(new_avg / 10.0, 1.0)  # Cap at 10 attempts
        penalty = (failure_rate * 0.7) + (attempt_penalty * 0.3)

        cursor.execute(
            '''UPDATE model_stats SET
               total_attempts = ?,
               successes = ?,
               failures = ?,
               avg_attempts_to_success = ?,
               penalty_score = ?,
               last_updated = CURRENT_TIMESTAMP
               WHERE model_name = ?''',
            (new_total, new_successes, new_failures, new_avg, penalty, model_name)
        )

    # Log execution
    cursor.execute(
        '''INSERT INTO execution_logs
           (prompt_hash, prompt_prefix, selected_models, successful_model,
            attempts, success, latency_ms)
           VALUES (?, ?, ?, ?, ?, ?, ?)''',
        (prompt_hash, prompt_hash[:200], json.dumps([]), model_name if success else None,
         attempts, success, latency_ms)
    )

    conn.commit()
```

## Agentic Execution Flow

```python
class AgenticExecutor:
    def __init__(self, config: dict):
        self.config = config
        self.max_retries = config.get('max_retries_per_model', 2)
        self.max_models = config.get('max_models_to_try', 3)

    async def execute_with_retry(
        self,
        prompt: str,
        ranked_models: list[tuple[str, float]],
        verifier: Callable[[str], bool]
    ) -> tuple[str, str]:
        """
        Execute task with model retry logic:
        - Try each model in ranked order
        - Each model gets N attempts (configurable)
        - Move to next model if all attempts fail
        - Stop when verifier passes or models exhausted
        """
        prompt_hash = hashlib.sha256(prompt.encode()).hexdigest()
        start_time = time.time()

        for model_idx, (model_name, score) in enumerate(ranked_models[:self.max_models]):
            model_attempts = 0
            model_success = False

            for attempt in range(self.max_retries):
                model_attempts += 1

                try:
                    # Execute model (placeholder for actual LLM call)
                    result = await self._call_model(model_name, prompt)

                    # Verify result
                    if verifier(result):
                        model_success = True
                        latency_ms = (time.time() - start_time) * 1000

                        # Update stats (success)
                        update_model_stats(
                            self.conn, model_name, prompt_hash,
                            model_attempts, True, latency_ms
                        )

                        return result, model_name

                except Exception as e:
                    print(f"Model {model_name} attempt {attempt + 1} failed: {e}")

            # All attempts for this model failed
            latency_ms = (time.time() - start_time) * 1000
            update_model_stats(
                self.conn, model_name, prompt_hash,
                model_attempts, False, latency_ms
            )

        # All models failed
        raise RuntimeError(f"All {self.max_models} models failed after {model_idx + 1} attempts")
```

## Configuration

```yaml
# router_config.yaml
embedding:
  model: all-MiniLM-L6-v2
  device: cpu  # or cuda
  batch_size: 32

routing:
  top_k_specializations: 10
  top_k_models: 5
  similarity_threshold: 0.3  # Minimum similarity to consider
  context_weight: 0.1  # Weight for context size bonus

retry:
  max_retries_per_model: 2
  max_models_to_try: 3
  timeout_seconds: 30

logging:
  db_path: ./data/router_logs.db
  retention_days: 30
  analyze_interval_minutes: 60

penalty:
  failure_rate_weight: 0.7
  attempt_penalty_weight: 0.3
  max_penalty: 1.0
  decay_factor: 0.99  # Daily decay to avoid over-penalization
```

## Deployment Script

```bash
#!/bin/bash
# deploy.sh

# Create virtual environment
python3 -m venv venv
source venv/bin/activate

# Install dependencies
pip install -r requirements.txt

# Initialize database
python scripts/init_db.py --csv-path ./model_routing_data.csv

# Pre-compute embeddings
python scripts/precompute_embeddings.py

# Start API server
uvicorn main:app --host 0.0.0.0 --port 8000
```

## API Interface

```python
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel

app = FastAPI()

class RouterRequest(BaseModel):
    prompt: str
    context_requirements: dict = {}
    max_models: int = 3

class RouterResponse(BaseModel):
    ranked_models: list[dict]
    top_specialization: dict
    similarity_scores: list[float]

@app.post("/route", response_model=RouterResponse)
async def route_prompt(request: RouterRequest):
    encoder = get_encoder()
    conn = get_db_connection()

    # Encode input
    input_embedding = encoder.encode_text(request.prompt)

    # Find similar specializations
    all_embeddings = load_all_embeddings(conn)
    matches = find_closest_specializations(input_embedding, all_embeddings, top_k=10)

    # Rank models for best match
    top_spec_id, top_score = matches[0]
    ranked_models = rank_models(top_spec_id, top_score, conn, config)

    # Format response
    return RouterResponse(
        ranked_models=[
            {"model": m, "score": s}
            for m, s in ranked_models[:request.max_models]
        ],
        top_specialization=get_specialization(conn, top_spec_id),
        similarity_scores=[score for _, score in matches[:5]]
    )
```

## Performance Characteristics

| Metric | Value | Notes |
|--------|-------|-------|
| Cold start time | ~2s | Load model + embeddings |
| Embedding time | ~50ms | Single prompt |
| Similarity search | ~5ms | 100 rows, NumPy |
| Full route (incl ranking) | ~100ms | End-to-end |
| Memory (idle) | ~50MB | Without model |
| Memory (active) | ~450MB | With model + embeddings |
| Max QPS | ~50 | Single thread |

## Advantages

1. **Low complexity**: Pure Python, minimal dependencies
2. **Fast iteration**: Easy to modify algorithms
3. **Good tooling**: Rich Python ecosystem
4. **Fast prototyping**: Quick to implement features

## Limitations

1. **Embedding model size**: 120MB adds to memory
2. **Python GIL**: Limited parallelism
3. **SQLite write contention**: Can bottleneck under high load
4. **No GPU acceleration**: Without CUDA, embedding is CPU-bound

## Extensions

1. **FAISS integration**: For similarity search with >100K rows
2. **Quantized embeddings**: Reduce memory to ~200MB
3. **Async DB**: Use aiosqlite for concurrent writes
4. **Batch routing**: Process multiple prompts in parallel
5. **REST API**: Add endpoints for model management
