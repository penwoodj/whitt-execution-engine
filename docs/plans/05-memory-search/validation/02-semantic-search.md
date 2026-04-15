# Validation Criteria: Task 02 - Semantic Search

## Overview

Validate that semantic search system provides vector embeddings, similarity search, and hybrid fusion with full-text search using local embedding models with ADR-0006 compliance.

---

## Checkpoint Gate Criteria

### Checkpoint 1: Embedding Generation Complete
- [ ] **Embedding model loaded**: Local model initialized
- [ ] **Embedding generation works**: Single and batch generation
- [ ] **Embeddings have correct dimension**: Verified against model spec
- [ ] **Unit tests pass**: All embedding tests (`cargo test --package agentsdk-search`)

**Verification Commands:**
```bash
# Verify embedding model loads
cargo check --package agentsdk-search

# Run embedding tests
cargo test --package agentsdk-search --lib embeddings

# Expected output: All embedding tests pass
```

### Checkpoint 2: Similarity Search Functional
- [ ] **Similarity search implemented**: Cosine similarity queries
- [ ] **Vector index built**: Indexed embeddings
- [ ] **Similarity scores computed**: Scores in [0, 1] range
- [ ] **Unit tests pass**: All similarity tests

**Verification Commands:**
```bash
# Run similarity tests
cargo test --package agentsdk-search --lib similarity

# Verify scoring
cargo test --package agentsdk-search test_similarity_scoring

# Expected output: All similarity tests pass
```

### Checkpoint 3: Hybrid Fusion Working
- [ ] **Hybrid search implemented**: Combines fulltext + semantic
- [ ] **Score fusion works**: Normalizes and combines scores
- [ ] **Re-ranking works**: Final ranking after fusion
- [ ] **Unit tests pass**: All fusion tests

**Verification Commands:**
```bash
# Run fusion tests
cargo test --package agentsdk-search --lib fusion

# Verify fusion quality
cargo test --package agentsdk-search --test hybrid_quality

# Expected output: All fusion tests pass
```

### Checkpoint 4: Performance Meets Targets
- [ ] **Generate single embedding < 100ms**: Measured with benchmarks
- [ ] **Generate batch embeddings < 500ms (10 items)**: Measured with benchmarks
- [ ] **Similarity search < 50ms (1000 vectors)**: Measured with benchmarks
- [ ] **Hybrid search < 200ms**: Measured with benchmarks

**Verification Commands:**
```bash
# Run performance benchmarks
cargo bench --package agentsdk-search

# Verify embedding performance
cargo bench --bench semantic_bench bench_embedding_generation

# Verify similarity performance
cargo bench --bench semantic_bench bench_similarity_search

# Expected output: All latency targets met
```

---

## Functional Requirements

### Embedding Generation

#### Single Embedding
- [ ] **Generates embeddings for text**
  - Test: `test_generate_single_embedding()`
  - Command: `cargo test --package agentsdk-search test_generate_single_embedding`
  - Expected: PASS, returns vector of correct dimension

- [ ] **Embeddings are deterministic for same text**
  - Test: `test_embedding_determinism()`
  - Command: `cargo test --package agentsdk-search test_embedding_determinism`
  - Expected: PASS, same text produces identical embedding

#### Batch Embedding
- [ ] **Generates embeddings for batch texts**
  - Test: `test_generate_batch_embeddings()`
  - Command: `cargo test --package agentsdk-search test_generate_batch_embeddings`
  - Expected: PASS, returns matrix of embeddings

- [ ] **Batch generation efficient**: < 500ms for 10 items
  - Test: `test_batch_efficiency()`
  - Command: `cargo test --package agentsdk-search test_batch_efficiency`
  - Expected: PASS, batch time < 500ms

#### Embedding Properties
- [ ] **Embeddings have correct dimension**
  - Test: `test_embedding_dimension()`
  - Command: `cargo test --package agentsdk-search test_embedding_dimension`
  - Expected: PASS, vector length matches model spec

- [ ] **Embeddings are normalized**
  - Test: `test_embedding_normalization()`
  - Command: `cargo test --package agentsdk-search test_embedding_normalization`
  - Expected: PASS, L2 norm = 1.0

- [ ] **Similar text has high similarity**
  - Test: `test_similar_text_similarity()`
  - Command: `cargo test --package agentsdk-search test_similar_text_similarity`
  - Expected: PASS, similarity > 0.8 for similar text

### Similarity Search

#### Vector Indexing
- [ ] **Indexes embeddings for fast search**
  - Test: `test_index_embeddings()`
  - Command: `cargo test --package agentsdk-search test_index_embeddings`
  - Expected: PASS, embeddings indexed and searchable

- [ ] **Updates index on new embeddings**
  - Test: `test_index_update()`
  - Command: `cargo test --package agentsdk-search test_index_update`
  - Expected: PASS, new embeddings added to index

- [ ] **Removes embeddings from index**
  - Test: `test_index_remove()`
  - Command: `cargo test --package agentsdk-search test_index_remove`
  - Expected: PASS, removed embeddings no longer searchable

#### Similarity Computation
- [ ] **Computes cosine similarity correctly**
  - Test: `test_cosine_similarity()`
  - Command: `cargo test --package agentsdk-search test_cosine_similarity`
  - Expected: PASS, matches manual calculation

- [ ] **Returns most similar results**
  - Test: `test_similar_results()`
  - Command: `cargo test --package agentsdk-search test_similar_results`
  - Expected: PASS, results sorted by similarity

- [ ] **Handles edge cases (zero vectors, etc.)**
  - Test: `test_edge_cases()`
  - Command: `cargo test --package agentsdk-search test_edge_cases`
  - Expected: PASS, handles zero vectors, empty input

#### Search Querying
- [ ] **Queries index with embedding**
  - Test: `test_query_by_embedding()`
  - Command: `cargo test --package agentsdk-search test_query_by_embedding`
  - Expected: PASS, returns k most similar

- [ ] **Queries index with text**
  - Test: `test_query_by_text()`
  - Command: `cargo test --package agentsdk-search test_query_by_text`
  - Expected: PASS, text embedded and queried

- [ ] **Supports k parameter**
  - Test: `test_k_parameter()`
  - Command: `cargo test --package agentsdk-search test_k_parameter`
  - Expected: PASS, returns exactly k results

### Hybrid Fusion

#### Score Combination
- [ ] **Combines fulltext and semantic scores**
  - Test: `test_score_combination()`
  - Command: `cargo test --package agentsdk-search test_score_combination`
  - Expected: PASS, combined score uses both sources

- [ ] **Uses configurable weights**
  - Test: `test_fusion_weights()`
  - Command: `cargo test --package agentsdk-search test_fusion_weights`
  - Expected: PASS, weights applied correctly

- [ ] **Normalizes scores before fusion**
  - Test: `test_score_normalization()`
  - Command: `cargo test --package agentsdk-search test_score_normalization`
  - Expected: PASS, both score ranges normalized to [0, 1]

#### Result Ranking
- [ ] **Ranks by combined score**
  - Test: `test_fusion_ranking()`
  - Command: `cargo test --package agentsdk-search test_fusion_ranking`
  - Expected: PASS, results sorted by combined score

- [ ] **Handles missing scores gracefully**
  - Test: `test_missing_scores()`
  - Command: `cargo test --package agentsdk-search test_missing_scores`
  - Expected: PASS, uses available scores only

#### Fusion Strategies
- [ ] **Supports weighted average fusion**
  - Test: `test_weighted_average_fusion()`
  - Command: `cargo test --package agentsdk-search test_weighted_average_fusion`
  - Expected: PASS, weighted average computed correctly

- [ ] **Supports Reciprocal Rank Fusion (RRF)**
  - Test: `test_rrf_fusion()`
  - Command: `cargo test --package agentsdk-search test_rrf_fusion`
  - Expected: PASS, RRF ranking computed correctly

---

## Performance Requirements

### Embedding Generation

- [ ] **Generate single embedding < 100ms**
  - Benchmark: `bench_single_embedding`
  - Command: `cargo bench --bench semantic_bench bench_single_embedding`
  - Expected: Mean < 100.0 ms, p95 < 150.0 ms

- [ ] **Generate batch embeddings < 500ms (10 items)**
  - Benchmark: `bench_batch_embedding_10`
  - Command: `cargo bench --bench semantic_bench bench_batch_embedding_10`
  - Expected: Mean < 500.0 ms, p95 < 600.0 ms

- [ ] **Generate 100 embeddings < 3 seconds**
  - Benchmark: `bench_batch_embedding_100`
  - Command: `cargo bench --bench semantic_bench bench_batch_embedding_100`
  - Expected: Mean < 3.0s, p95 < 3.5s

### Similarity Search

- [ ] **Similarity search < 50ms (1000 vectors)**
  - Benchmark: `bench_similarity_search_1k`
  - Command: `cargo bench --bench semantic_bench bench_similarity_search_1k`
  - Expected: Mean < 50.0 ms, p95 < 75.0 ms

- [ ] **Similarity search < 200ms (10000 vectors)**
  - Benchmark: `bench_similarity_search_10k`
  - Command: `cargo bench --bench semantic_bench bench_similarity_search_10k`
  - Expected: Mean < 200.0 ms, p95 < 250.0 ms

### Hybrid Search

- [ ] **Hybrid search < 200ms**
  - Benchmark: `bench_hybrid_search`
  - Command: `cargo bench --bench semantic_bench bench_hybrid_search`
  - Expected: Mean < 200.0 ms, p95 < 250.0 ms

- [ ] **Hybrid fusion < 50ms**
  - Benchmark: `bench_fusion`
  - Command: `cargo bench --bench semantic_bench bench_fusion`
  - Expected: Mean < 50.0 ms, p95 < 75.0 ms

---

## Accuracy

### Embedding Quality

- [ ] **Similarity scores in range [0, 1]**
  - Test: `test_similarity_range()`
  - Command: `cargo test --package agentsdk-search test_similarity_range`
  - Expected: PASS, all scores in valid range

- [ ] **Similarity of identical text = 1.0**
  - Test: `test_identical_similarity()`
  - Command: `cargo test --package agentsdk-search test_identical_similarity`
  - Expected: PASS, identical text returns 1.0

- [ ] **Semantic search finds conceptually similar results**
  - Test: `test_semantic_similarity()`
  - Command: `cargo test --package agentsdk-search test_semantic_similarity`
  - Expected: PASS, semantic queries return relevant results

### Fusion Quality

- [ ] **Hybrid fusion improves over individual searches**
  - Test: `test_fusion_improvement()`
  - Command: `cargo test --package agentsdk-search test_fusion_improvement`
  - Expected: NDCG@10 improved by >10%

- [ ] **Combination preserves top results from both sources**
  - Test: `test_fusion_diversity()`
  - Command: `cargo test --package agentsdk-search test_fusion_diversity`
  - Expected: PASS, top results include best from both

---

## Error Handling

### Embedding Errors

- [ ] **Handles empty input**
  - Test: `test_empty_input_handling()`
  - Command: `cargo test --package agentsdk-search test_empty_input_handling`
  - Expected: PASS, returns error or empty result

- [ ] **Handles very long input**
  - Test: `test_long_input_handling()`
  - Command: `cargo test --package agentsdk-search test_long_input_handling`
  - Expected: PASS, truncates or rejects with error

- [ ] **Handles model loading errors**
  - Test: `test_model_loading_error()`
  - Command: `cargo test --package agentsdk-search test_model_loading_error`
  - Expected: PASS, graceful error on model load failure

- [ ] **Handles inference errors**
  - Test: `test_inference_error()`
  - Command: `cargo test --package agentsdk-search test_inference_error`
  - Expected: PASS, error propagated with context

### Search Errors

- [ ] **Handles index errors gracefully**
  - Test: `test_index_error_handling()`
  - Command: `cargo test --package agentsdk-search test_index_error_handling`
  - Expected: PASS, error logged, search fails gracefully

- [ ] **Handles missing embeddings**
  - Test: `test_missing_embedding_handling()`
  - Command: `cargo test --package agentsdk-search test_missing_embedding_handling`
  - Expected: PASS, skips documents without embeddings

---

## Schema Compliance

### Embedding Schema

- [ ] **Embedding vectors have correct schema**
  - Test: `test_embedding_schema()`
  - Command: `cargo test --package agentsdk-search test_embedding_schema`
  - Expected: PASS, vector dimension matches model spec

- [ ] **Metadata preserved with embeddings**
  - Test: `test_embedding_metadata()`
  - Command: `cargo test --package agentsdk-search test_embedding_metadata`
  - Expected: PASS, document metadata stored with embeddings

---

## ADR-0006 Compliance

### Semantic Index Location

- [ ] **Semantic index stored in `./workspace/memory/index/semantic/`**
  - Test: `test_semantic_index_location()`
  - Command: `cargo test --package agentsdk-search test_semantic_index_location`
  - Expected: Index files in correct directory

- [ ] **No external embedding API calls**
  - Test: `test_local_embedding_model()`
  - Command: `cargo test --package agentsdk-search test_local_embedding_model`
  - Expected: No network calls during embedding generation

### Provenance Tracking

- [ ] **All embedding operations track provenance**
  - Test: `test_embedding_provenance()`
  - Command: `cargo test --package agentsdk-search test_embedding_provenance`
  - Expected: Embedding events logged with trace ID

- [ ] **Embedding generation records model version**
  - Test: `test_model_version_tracking()`
  - Command: `cargo test --package agentsdk-search test_model_version_tracking`
  - Expected: Model version included in metadata

### Hybrid Search Constraint

- [ ] **Semantic search ONLY combined with fulltext (not vectors alone)**
  - Test: `test_no_standalone_semantic()`
  - Command: `cargo test --package agentsdk-search test_no_standalone_semantic`
  - Expected: PASS, semantic search only available through hybrid

- [ ] **Hybrid search ALWAYS includes fulltext component**
  - Test: `test_hybrid_requires_fulltext()`
  - Command: `cargo test --package agentsdk-search test_hybrid_requires_fulltext`
  - Expected: PASS, hybrid search fails without fulltext results

---

## Integration Points

### Full-Text Search Integration

- [ ] **Full-text results can be fused with semantic**
  - Test: `test_fulltext_fusion()`
  - Command: `cargo test --package agentsdk-search test_fulltext_fusion`
  - Expected: PASS, results from both sources merged

- [ ] **Score normalization works across both sources**
  - Test: `test_cross_source_normalization()`
  - Command: `cargo test --package agentsdk-search test_cross_source_normalization`
  - Expected: PASS, both score ranges normalized

### Memory Integration

- [ ] **Auto-generates embeddings on memory storage**
  - Test: `test_auto_embedding_on_store()`
  - Command: `cargo test --package agentsdk-search test_auto_embedding_on_store`
  - Expected: PASS, embeddings generated automatically

- [ ] **Updates embeddings on memory update**
  - Test: `test_auto_embedding_on_update()`
  - Command: `cargo test --package agentsdk-search test_auto_embedding_on_update`
  - Expected: PASS, embeddings regenerated on update

- [ ] **Removes embeddings on memory delete**
  - Test: `test_auto_embedding_on_delete()`
  - Command: `cargo test --package agentsdk-search test_auto_embedding_on_delete`
  - Expected: PASS, embeddings removed from index

---

## Log Verification Patterns

### Embedding Operation Logs

- [ ] **Embedding generation logged with trace ID**
  - Grep: `grep '"operation":"embedding"' ./workspace/logs/search.log | jq -r '.trace_id' | wc -l`
  - Expected: Count equals number of embedding operations

- [ ] **Model version logged**
  - Grep: `grep '"operation":"embedding"' ./workspace/logs/search.log | jq -r '.model_version'`
  - Expected: Model version present for all operations

- [ ] **Embedding duration logged**
  - Grep: `grep '"operation":"embedding"' ./workspace/logs/search.log | jq -r '.duration_ms'`
  - Expected: Duration in milliseconds for all operations

### Similarity Search Logs

- [ ] **Similarity search logged**
  - Grep: `grep '"operation":"similarity_search"' ./workspace/logs/search.log | wc -l`
  - Expected: Count equals number of similarity searches

- [ ] **Search k parameter logged**
  - Grep: `grep '"operation":"similarity_search"' ./workspace/logs/search.log | jq -r '.k'`
  - Expected: k parameter present for all searches

### Hybrid Fusion Logs

- [ ] **Fusion operations logged**
  - Grep: `grep '"operation":"fusion"' ./workspace/logs/search.log | wc -l`
  - Expected: Count equals number of fusions

- [ ] **Fusion weights logged**
  - Grep: `grep '"operation":"fusion"' ./workspace/logs/search.log | jq -r '.weights'`
  - Expected: Weights present for all fusions

- [ ] **Fusion strategy logged**
  - Grep: `grep '"operation":"fusion"' ./workspace/logs/search.log | jq -r '.strategy'`
  - Expected: Strategy present for all fusions

### Error Logs

- [ ] **Embedding errors logged with context**
  - Grep: `grep '"level":"error"' ./workspace/logs/search.log | grep embedding | jq -r '.error'`
  - Expected: Error messages include text and operation details

---

## Test Coverage

### Unit Tests

- [ ] **Unit tests for embedding generation**
  - Command: `cargo test --package agentsdk-search --lib embeddings`
  - Expected: All embedding tests pass

- [ ] **Unit tests for similarity computation**
  - Command: `cargo test --package agentsdk-search --lib similarity`
  - Expected: All similarity tests pass

- [ ] **Unit tests for hybrid fusion**
  - Command: `cargo test --package agentsdk-search --lib fusion`
  - Expected: All fusion tests pass

### Integration Tests

- [ ] **Integration tests for hybrid search workflow**
  - Command: `cargo test --package agentsdk-search --test integration_test`
  - Expected: All integration tests pass

- [ ] **Integration tests with memory storage**
  - Command: `cargo test --package agentsdk-search --test memory_integration`
  - Expected: All memory integration tests pass

### Accuracy Benchmarks

- [ ] **Accuracy benchmarks for semantic search**
  - Command: `cargo test --package agentsdk-search --test accuracy_benchmarks`
  - Expected: Semantic search accuracy metrics collected

- [ ] **Accuracy benchmarks for hybrid fusion**
  - Command: `cargo test --package agentsdk-search --test fusion_quality`
  - Expected: Fusion improvement metrics collected

---

## Final Checklist

### Implementation Complete
- [ ] Embedding generation implemented and tested
- [ ] Similarity search functional
- [ ] Hybrid fusion working
- [ ] Performance meets all targets

### ADR-0006 Compliant
- [ ] Semantic index in `./workspace/memory/index/semantic/`
- [ ] All embedding operations track provenance
- [ ] Semantic search ONLY combined with fulltext

### Integration Ready
- [ ] Auto-generates embeddings on memory operations
- [ ] Works with full-text search
- [ ] Fusion improves over individual searches

### Documentation Complete
- [ ] API documentation generated
- [ ] Fusion strategies documented
- [ ] Performance characteristics documented

### Tests Passing
- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] All benchmarks meet targets
- [ ] Test coverage > 90%

**Total Lines:** ~530 lines
