# Report 1: TinyRouter Design

## Stack Overview

**Technology**: Python + SQLite + sqlite-vec + FTS5 + FastEmbed + Ollama

This is the smallest, simplest version and the one to start with.

### Key Components

| Component | Technology | Purpose |
|-----------|-------------|----------|
| Embedding | FastEmbed (ONNX) | Lightweight vector encoding |
| Storage | SQLite | Single-file relational database |
| Vector Search | sqlite-vec (vec0) | KNN similarity search |
| Text Search | SQLite FTS5 | BM25 full-text search |
| Inference | Ollama API | Local LLM execution |

### Memory Footprint

| Item | Size |
|-------|------|
| SQLite + extensions | ~5MB |
| FastEmbed model | ~80MB |
| Pre-computed embeddings | ~150KB (95 rows) |
| Python runtime | ~50MB |
| **Total** | **~135MB** |

## When to Choose This

Choose TinyRouter when you want:
- ✅ Minimum RAM usage
- ✅ Minimum moving parts
- ✅ One local file database
- ✅ Easy debugging
- ✅ Easy backups
- ✅ Simple single-user local orchestration

## Core Idea

Everything lives in SQLite:

1. **router_rows** - CSV specializations
2. **FTS5 text index** - Full-text search on row text
3. **vec0 vector index** - KNN similarity search
4. **model_stats** - Per-model statistics
5. **task_logs** - Execution attempt logs
6. **verifier_results** - Validation outcomes
7. **hardware_profile** - Cached hardware measurements

## ASCII Architecture Diagram

```
             +----------------------+
             |   task input text    |
             +----------+-----------+
                        |
                        v
             +----------------------+
             | trait extractor      |
             | - language           |
             | - output type        |
             | - edit vs scratch    |
             | - context estimate   |
             +----------+-----------+
                        |
            +-----------+------------+
            |                        |
            v                        v
 +-------------------+      +----------------------+
 | FastEmbed vector  |      | SQLite FTS5 lookup   |
 | query embedding   |      | tags/synonyms/BM25   |
 +---------+---------+      +----------+-----------+
           |                           |
           v                           v
 +-------------------+      +----------------------+
 | sqlite-vec KNN    |      | sparse candidates    |
 | top K row matches |      | top K row matches    |
 +---------+---------+      +----------+-----------+
           \____________________  __________________/
                                \/
                    +---------------------------+
                    | row fusion + constraints  |
                    | dense + sparse + filters  |
                    +-------------+-------------+
                                  |
                                  v
                    +---------------------------+
                    | expand into model options |
                    +-------------+-------------+
                                  |
                                  v
                    +---------------------------+
                    | model ranker              |
                    | + popularity prior        |
                    | + hardware fit            |
                    | + log success score       |
                    +-------------+-------------+
                                  |
                                  v
                    +---------------------------+
                    | try / verify / retry      |
                    | via Ollama local API      |
                    +-------------+-------------+
                                  |
                    +-------------+-------------+
                    |                           |
                    v                           v
           +------------------+        +------------------+
           | success -> log   |        | fail -> next try |
           +------------------+        +------------------+
```

## Database Schema

### router_rows

```sql
CREATE TABLE router_rows (
    row_id INTEGER PRIMARY KEY,
    specialization_category TEXT NOT NULL,
    specialization_synonyms TEXT,
    related_tags TEXT,
    primary_strength TEXT,
    output_rigidity TEXT,
    context_need TEXT,
    verification_need TEXT,
    router_keywords TEXT,
    avoid_when TEXT,
    selection_notes TEXT
);
```

### row_models

```sql
CREATE TABLE row_models (
    row_id INTEGER,
    slot_name TEXT,  -- 'top1', 'top2', 'top3', 'wildcard'
    model_name TEXT,
    model_ctx INTEGER,
    slot_bonus REAL,
    PRIMARY KEY (row_id, slot_name),
    FOREIGN KEY (row_id) REFERENCES router_rows(row_id)
);
```

### model_catalog

```sql
CREATE TABLE model_catalog (
    model_name TEXT PRIMARY KEY,
    context_window INTEGER,
    runtime TEXT,  -- 'ollama', 'llama.cpp', etc.
    quant TEXT,    -- 'q4_k_m', 'q8_0', etc.
    memory_tier TEXT,  -- 'tiny', 'small', 'medium', 'large'
    avg_latency_ms REAL,
    popularity_prior REAL,
    enabled INTEGER DEFAULT 1
);
```

### task_logs

```sql
CREATE TABLE task_logs (
    attempt_id INTEGER PRIMARY KEY AUTOINCREMENT,
    ts INTEGER NOT NULL,
    task_text TEXT NOT NULL,
    task_embedding BLOB,
    row_id INTEGER,
    model_name TEXT,
    try_index INTEGER,
    prompt_len_bucket TEXT,  -- 'short', 'medium', 'long', 'huge'
    task_type TEXT,
    language TEXT,
    output_type TEXT,
    verifier_type TEXT,
    success INTEGER,
    fail_reason TEXT,
    parse_pass INTEGER,
    compile_pass INTEGER,
    test_pass INTEGER,
    latency_ms REAL,
    token_count INTEGER,
    FOREIGN KEY (row_id) REFERENCES router_rows(row_id)
);
```

### model_stats

```sql
CREATE TABLE model_stats (
    model_name TEXT PRIMARY KEY,
    total_attempts INTEGER DEFAULT 0,
    successes INTEGER DEFAULT 0,
    failures INTEGER DEFAULT 0,
    avg_attempts_to_success REAL DEFAULT 0,
    verified_success_rate REAL DEFAULT 0,
    first_try_success_rate REAL DEFAULT 0,
    avg_latency_ms REAL DEFAULT 0,
    last_updated INTEGER
);
```

### FTS5 Index

```sql
CREATE VIRTUAL TABLE router_fts USING fts5(
    row_id,
    specialization_category,
    specialization_synonyms,
    related_tags,
    router_keywords,
    selection_notes,
    content='router_rows',
    content_rowid='row_id'
);
```

### Vector Index

```sql
-- Load sqlite-vec extension
.load libsqlite_vec.so

-- Create vector table
CREATE VIRTUAL TABLE row_vectors USING vec0(
    row_id INTEGER PRIMARY KEY,
    embedding FLOAT[384]
);
```

## Retrieval Algorithm

### Step 1: Build Row Text

For each CSV row, build canonical routing text:

```
specialization_category |
specialization_synonyms |
related_synonym_tags |
primary_strength |
secondary_strengths |
preferred_input_shape |
preferred_output_shape |
router_keywords |
avoid_when |
selection_notes
```

**Example** (for `natural_language_to_correct_parsable_json`):
```
natural language to correct parsable json |
nl to json | free text to structured json | plain language to valid json |
json | structured-output | parsable | schema | object | machine-readable |
valid json emission |
schema following | extraction | parser safety |
plain text | doc snippet | html | requirements |
strict json object | array | schema shaped json |
json emit parsable object schema valid strict
```

### Step 2: Dense Retrieval

```python
import fastembed
import sqlite3

# Embed task
model = fastembed.TextEmbedding("all-MiniLM-L6-v2")
task_embedding = next(model.embed([task_text]))[0]

# Query sqlite-vec
cursor.execute("""
    SELECT row_id, distance
    FROM row_vectors
    ORDER BY distance
    LIMIT ?
""", (K_dense,))
dense_results = cursor.fetchall()
```

### Step 3: Sparse Retrieval

```sql
-- FTS5 BM25 search
SELECT row_id, rank
FROM router_fts
WHERE router_fts MATCH ?
ORDER BY rank
LIMIT ?;
```

### Step 4: Fuse Results

```
row_score =
  0.45 * dense_score +
  0.25 * sparse_score +
  0.10 * output_type_match +
  0.10 * language_match +
  0.10 * context_fit
```

**Then remove rows failing hard constraints:**
- Row says avoid strict JSON but task requires strict JSON
- Model contexts are too small
- Output rigidity mismatch is severe
- Edit task but row is only good for from-scratch generation

## Model Expansion

Each winning row contributes 4 candidates:

| Slot | Model | Bonus |
|-------|--------|--------|
| top1 | `spec.top1_model` | 1.00 |
| top2 | `spec.top2_model` | 0.82 |
| top3 | `spec.top3_model` | 0.68 |
| wildcard | `spec.wildcard_model` | 0.55 |

**Deduplication**: If same model appears from multiple rows, merge by max row score + small repeat bonus.

## Ranking Formula

### Initial Score

```python
final_model_score = (
    0.35 * row_score +
    0.10 * slot_bonus +
    0.10 * popularity_prior +
    0.10 * hardware_fit +
    0.35 * log_success_score
)
```

### Hardware Fit Calculation

Use local measurements stored in `hardware_profile`:

```python
def calculate_hardware_fit(model_name):
    profile = get_hardware_profile(model_name)

    if profile.oom_count > 0:
        return 0.1  # Huge penalty

    if profile.context_overflow_count > 0:
        return 0.5  # Medium penalty

    if profile.avg_load_time_ms > 5000:
        return 0.8  # Small penalty

    return 1.0  # No penalty
```

## Log-Aware Ranking

### Find Similar Past Attempts

For each new task, find similar past attempts:

```sql
-- KNN search on task embeddings
SELECT attempt_id, model_name, success, latency_ms, token_count
FROM task_logs
WHERE task_embedding MATCH ?
ORDER BY distance
LIMIT 50;
```

Then filter by:
- Same output type
- Same language
- Same verifier type
- Same edit/scratch mode

### Compute Per-Model Statistics

```python
for model in candidate_models:
    similar_attempts = get_similar_attempts(model)

    stats = {
        'verified_success_rate': similar_attempts.successes / similar_attempts.total,
        'first_try_success_rate': (similar_attempts.successes_with_try_1) / similar_attempts.total,
        'avg_retries': similar_attempts.total_retries / similar_attempts.total,
        'avg_latency': similar_attempts.avg_latency_ms,
        'parse_pass_rate': similar_attempts.parse_passes / similar_attempts.total,
        'compile_pass_rate': similar_attempts.compile_passes / similar_attempts.total,
        'test_pass_rate': similar_attempts.test_passes / similar_attempts.total
    }
```

### Bayesian Smoothing

```python
def smooth_success(successes, attempts, global_rate, alpha=10):
    return (successes + alpha * global_rate) / (attempts + alpha)
```

Use `alpha = 10` to start, adjust based on confidence.

### Log Success Score

```python
log_success_score = (
    0.50 * stats['verified_success_rate'] +
    0.20 * stats['first_try_success_rate'] +
    0.15 * (1 - min(stats['avg_retries'], 1)) +  # Low retry = high score
    0.10 * (stats['parse_pass_rate'] or stats['compile_pass_rate'] or 0) +
    0.05 * max(1 - stats['avg_latency'] / 30000, 0)  # Favor < 30s latency
)
```

## Retry / Switch Loop

```python
async def execute_with_retry(task, ranked_models, verifier_chain):
    prompt_hash = hash(task.text)

    for model_idx, (model_name, score) in enumerate(ranked_models[:N]):
        max_tries = get_max_tries_for_specialization(task.spec)

        for try_idx in range(max_tries):
            # Run model via Ollama
            result = await ollama_generate(model_name, task.prompt)

            # Verify
            verdict = await verifier_chain.verify(result, task)

            # Log attempt
            log_attempt(
                prompt_hash=prompt_hash,
                model_name=model_name,
                try_index=try_idx,
                success=verdict.pass,
                fail_reason=verdict.fail_reason,
                latency_ms=verdict.latency_ms,
                parse_pass=verdict.parse_pass,
                compile_pass=verdict.compile_pass,
                test_pass=verdict.test_pass
            )

            if verdict.pass:
                return result

            if verdict.recoverable:
                # Repair prompt and retry
                task.prompt = repair_prompt(task, result, verdict.errors)
            else:
                # Try next model
                break

    raise ExhaustedModelsError()
```

## Verifier Design

Attach verifier chains by **specialization row**, not by model.

### Registry

```python
VERIFIERS = {
    'natural_language_to_correct_parsable_json': JsonVerifier(),
    'requirement_to_executable_python_code_from_scratch': PythonCodeVerifier(),
    'sql_query_generation': SqlVerifier(),
    # ... etc
}
```

### Example: JSON Verifier

```python
class JsonVerifier:
    async def verify(self, result, task):
        errors = []

        # 1. Parse with strict parser
        try:
            data = json.loads(result)
        except json.JSONDecodeError as e:
            errors.append(f"Invalid JSON: {e}")
            return Verdict(pass=False, recoverable=True, errors=errors)

        # 2. Schema validate (if schema provided)
        if task.schema:
            try:
                jsonschema.validate(data, task.schema)
            except jsonschema.ValidationError as e:
                errors.append(f"Schema validation failed: {e}")
                return Verdict(pass=False, recoverable=True, errors=errors)

        # 3. Required fields present
        if task.required_fields:
            for field in task.required_fields:
                if field not in data:
                    errors.append(f"Missing required field: {field}")
                    recoverable = True
                else:
                    recoverable = False

        # 4. No disallowed fields
        if task.disallowed_fields:
            for field in task.disallowed_fields:
                if field in data:
                    errors.append(f"Disallowed field present: {field}")
                    return Verdict(pass=False, recoverable=True, errors=errors)

        # 5. Correct data types
        if task.field_types:
            for field, expected_type in task.field_types.items():
                if field in data and not isinstance(data[field], expected_type):
                    errors.append(f"Field {field} has wrong type")
                    recoverable = True

        if errors:
            return Verdict(pass=False, recoverable=recoverable, errors=errors)

        return Verdict(pass=True, recoverable=False, errors=[])
```

### Example: Python Code Verifier

```python
class PythonCodeVerifier:
    async def verify(self, result, task):
        errors = []

        # 1. Syntax parse
        try:
            ast.parse(result)
        except SyntaxError as e:
            errors.append(f"Syntax error: {e}")
            return Verdict(pass=False, recoverable=True, errors=errors)

        # 2. Optional unit tests
        if task.tests:
            try:
                exec(result, globals())
                for test in task.tests:
                    if not test():
                        errors.append(f"Test failed: {test.__name__}")
            except Exception as e:
                errors.append(f"Execution error: {e}")
                recoverable = True

        # 3. Imports allowed list
        if task.allowed_imports:
            tree = ast.parse(result)
            for node in ast.walk(tree):
                if isinstance(node, ast.Import):
                    for alias in node.names:
                        if alias.name not in task.allowed_imports:
                            errors.append(f"Disallowed import: {alias.name}")
                            recoverable = True

        # 4. Function name present
        if task.required_function:
            tree = ast.parse(result)
            functions = [node.name for node in ast.walk(tree) if isinstance(node, ast.FunctionDef)]
            if task.required_function not in functions:
                errors.append(f"Required function missing: {task.required_function}")
                recoverable = True

        if errors:
            return Verdict(pass=False, recoverable=recoverable, errors=errors)

        return Verdict(pass=True, recoverable=False, errors=[])
```

## Why This Is Strong

1. **Extremely small footprint** - < 150MB RAM
2. **Trivial to inspect** - Single SQLite file
3. **Logs and router live together** - Easy correlation
4. **Easy to back up and version** - Copy one file
5. **Best first implementation** - Fastest to working prototype

## Weak Points

1. **Limited concurrent scale** - SQLite write locks
2. **Weaker analytics** - No advanced aggregation
3. **Limited multi-stage search** - Manual fusion only
4. **Ranking features get awkward** - Once logs grow large

## Minimal-Hardware Recommendation

**Start here first** if you want something that works on almost anything:

- ✅ Python 3.10+
- ✅ FastEmbed CPU (no GPU needed)
- ✅ SQLite file (no server)
- ✅ Ollama local inference (no API keys)
- ✅ One process or two processes only

**Hardware**: Any modern laptop with 4GB+ RAM
