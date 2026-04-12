# Validation Criteria: Task 03 - Search Query Engine

## Overview

Validate that search query engine provides hybrid query orchestration, caching, filtering, re-ranking, and pagination with ADR-0006 compliance.

---

## Checkpoint Gate Criteria

### Checkpoint 1: Query Execution Complete
- [ ] **Hybrid query execution works**: Combines fulltext + semantic
- [ ] **Filter evaluation implemented**: Equality, contains, comparison filters
- [ ] **Pagination implemented**: Limit/offset pagination
- [ ] **Unit tests pass**: All query tests (`cargo test --package agentsdk-search`)

**Verification Commands:**
```bash
# Verify query engine builds
cargo check --package agentsdk-search

# Run query tests
cargo test --package agentsdk-search --lib query_engine

# Expected output: All query tests pass
```

### Checkpoint 2: Caching Functional
- [ ] **Query result caching works**: Cached results returned
- [ ] **TTL enforcement works**: Expired cache invalidated
- [ ] **Cache invalidation works**: On index updates
- [ ] **Unit tests pass**: All cache tests

**Verification Commands:**
```bash
# Run cache tests
cargo test --package agentsdk-search --lib caching

# Verify cache behavior
cargo test --package agentsdk-search test_cache_behavior

# Expected output: All cache tests pass
```

### Checkpoint 3: Re-ranking Working
- [ ] **Re-ranking strategies implemented**: Recency, popularity, custom
- [ ] **Re-ranking improves results**: Better ranking after re-rank
- [ ] **Default strategy applied**: Applied automatically
- [ ] **Unit tests pass**: All re-ranking tests

**Verification Commands:**
```bash
# Run re-ranking tests
cargo test --package agentsdk-search --lib reranking

# Verify ranking improvement
cargo test --package agentsdk-search test_reranking_improvement

# Expected output: All re-ranking tests pass
```

### Checkpoint 4: Performance Meets Targets
- [ ] **Cached query < 5ms**: Measured with benchmarks
- [ ] **Uncached query < 200ms**: Measured with benchmarks
- [ ] **Cache cleanup efficient (< 1s for 1000 entries)**: Measured with benchmarks

**Verification Commands:**
```bash
# Run performance benchmarks
cargo bench --bench query_bench

# Verify cache performance
cargo bench --bench query_bench bench_cached_query

# Verify uncached performance
cargo bench --bench query_bench bench_uncached_query

# Expected output: All latency targets met
```

---

## Functional Requirements

### Query Execution

#### Hybrid Queries
- [ ] **Executes hybrid search queries**
  - Test: `test_hybrid_query()`
  - Command: `cargo test --package agentsdk-search test_hybrid_query`
  - Expected: PASS, combines fulltext and semantic results

- [ ] **Applies filters correctly**
  - Test: `test_filter_application()`
  - Command: `cargo test --package agentsdk-search test_filter_application`
  - Expected: PASS, filters applied to results

- [ ] **Re-ranks results by strategy**
  - Test: `test_reranking()`
  - Command: `cargo test --package agentsdk-search test_reranking`
  - Expected: PASS, results re-ranked

- [ ] **Returns paginated results**
  - Test: `test_pagination()`
  - Command: `cargo test --package agentsdk-search test_pagination`
  - Expected: PASS, correct page returned

#### Query Parsing
- [ ] **Parses query language**
  - Test: `test_query_language_parsing()`
  - Command: `cargo test --package agentsdk-search test_query_language_parsing`
  - Expected: PASS, query parsed into AST

- [ ] **Validates query parameters**
  - Test: `test_query_validation()`
  - Command: `cargo test --package agentsdk-search test_query_validation`
  - Expected: PASS, invalid queries rejected

- [ ] **Handles complex queries**
  - Test: `test_complex_queries()`
  - Command: `cargo test --package agentsdk-search test_complex_queries`
  - Expected: PASS, nested filters and logic handled

### Caching

#### Cache Operations
- [ ] **Caches query results**
  - Test: `test_cache_write()`
  - Command: `cargo test --package agentsdk-search test_cache_write`
  - Expected: PASS, results stored in cache

- [ ] **Retrieves cached results**
  - Test: `test_cache_read()`
  - Command: `cargo test --package agentsdk-search test_cache_read`
  - Expected: PASS, cached results returned

- [ ] **Respects cache TTL**
  - Test: `test_cache_ttl()`
  - Command: `cargo test --package agentsdk-search test_cache_ttl`
  - Expected: PASS, expired cache not returned

- [ ] **Invalidates cache on index updates**
  - Test: `test_cache_invalidation()`
  - Command: `cargo test --package agentsdk-search test_cache_invalidation`
  - Expected: PASS, cache cleared on index change

- [ ] **Cleanup expired cache entries**
  - Test: `test_cache_cleanup()`
  - Command: `cargo test --package agentsdk-search test_cache_cleanup`
  - Expected: PASS, expired entries removed

#### Cache Configuration
- [ ] **Cache size limit enforced**
  - Test: `test_cache_size_limit()`
  - Command: `cargo test --package agentsdk-search test_cache_size_limit`
  - Expected: PASS, oldest entries evicted when full

- [ ] **TTL configurable**
  - Test: `test_cache_ttl_config()`
  - Command: `cargo test --package agentsdk-search test_cache_ttl_config`
  - Expected: PASS, TTL applied per configuration

### Re-ranking

#### Re-ranking Strategies
- [ ] **Supports recency re-ranking**
  - Test: `test_recency_reranking()`
  - Command: `cargo test --package agentsdk-search test_recency_reranking`
  - Expected: PASS, newer documents promoted

- [ ] **Supports popularity re-ranking**
  - Test: `test_popularity_reranking()`
  - Command: `cargo test --package agentsdk-search test_popularity_reranking`
  - Expected: PASS, frequently accessed documents promoted

- [ ] **Supports custom re-ranking functions**
  - Test: `test_custom_reranking()`
  - Command: `cargo test --package agentsdk-search test_custom_reranking`
  - Expected: PASS, custom ranking function applied

- [ ] **Default re-ranking strategy applied**
  - Test: `test_default_reranking()`
  - Command: `cargo test --package agentsdk-search test_default_reranking`
  - Expected: PASS, default strategy used when not specified

#### Re-ranking Logic
- [ ] **Re-ranking improves result order**
  - Test: `test_reranking_improvement()`
  - Command: `cargo test --package agentsdk-search test_reranking_improvement`
  - Expected: PASS, NDCG improved after re-ranking

- [ ] **Handles ties correctly**
  - Test: `test_tie_handling()`
  - Command: `cargo test --package agentsdk-search test_tie_handling`
  - Expected: PASS, ties broken deterministically

### Filter Evaluation

#### Filter Types
- [ ] **Evaluates equality filters**
  - Test: `test_equality_filter()`
  - Command: `cargo test --package agentsdk-search test_equality_filter`
  - Expected: PASS, only matching results returned

- [ ] **Evaluates contains filters**
  - Test: `test_contains_filter()`
  - Command: `cargo test --package agentsdk-search test_contains_filter`
  - Expected: PASS, only matching results returned

- [ ] **Evaluates comparison filters (>, <, >=, <=)**
  - Test: `test_comparison_filter()`
  - Command: `cargo test --package agentsdk-search test_comparison_filter`
  - Expected: PASS, only matching results returned

#### Filter Combination
- [ ] **Combines multiple filters with AND logic**
  - Test: `test_and_filters()`
  - Command: `cargo test --package agentsdk-search test_and_filters`
  - Expected: PASS, only results matching all filters returned

- [ ] **Combines multiple filters with OR logic**
  - Test: `test_or_filters()`
  - Command: `cargo test --package agentsdk-search test_or_filters`
  - Expected: PASS, results matching any filter returned

- [ ] **Handles nested filter logic**
  - Test: `test_nested_filters()`
  - Command: `cargo test --package agentsdk-search test_nested_filters`
  - Expected: PASS, complex filter logic evaluated correctly

### Pagination

#### Pagination Operations
- [ ] **Supports limit parameter**
  - Test: `test_limit_pagination()`
  - Command: `cargo test --package agentsdk-search test_limit_pagination`
  - Expected: PASS, returns at most limit results

- [ ] **Supports offset parameter**
  - Test: `test_offset_pagination()`
  - Command: `cargo test --package agentsdk-search test_offset_pagination`
  - Expected: PASS, skips offset results

- [ ] **Supports cursor-based pagination**
  - Test: `test_cursor_pagination()`
  - Command: `cargo test --package agentsdk-search test_cursor_pagination`
  - Expected: PASS, pagination via cursor

#### Pagination Consistency
- [ ] **Pagination is deterministic**
  - Test: `test_pagination_determinism()`
  - Command: `cargo test --package agentsdk-search test_pagination_determinism`
  - Expected: PASS, same page returned for same query

- [ ] **Handles pagination at boundaries**
  - Test: `test_pagination_boundaries()`
  - Command: `cargo test --package agentsdk-search test_pagination_boundaries`
  - Expected: PASS, empty page beyond results

---

## Performance Requirements

### Caching Performance

- [ ] **Cached query < 5ms**
  - Benchmark: `bench_cached_query`
  - Command: `cargo bench --bench query_bench bench_cached_query`
  - Expected: Mean < 5.0 ms, p95 < 10.0 ms

- [ ] **Cache hit rate > 80% (typical workload)**
  - Test: `test_cache_hit_rate()`
  - Command: `cargo test --package agentsdk-search test_cache_hit_rate`
  - Expected: Hit rate > 0.80

- [ ] **Cache cleanup efficient (< 1s for 1000 entries)**
  - Benchmark: `bench_cache_cleanup`
  - Command: `cargo bench --bench query_bench bench_cache_cleanup`
  - Expected: Mean < 1.0s, p95 < 1.5s

### Query Execution Performance

- [ ] **Uncached query < 200ms**
  - Benchmark: `bench_uncached_query`
  - Command: `cargo bench --bench query_bench bench_uncached_query`
  - Expected: Mean < 200.0 ms, p95 < 250.0 ms

- [ ] **Complex query < 300ms**
  - Benchmark: `bench_complex_query`
  - Command: `cargo bench --bench query_bench bench_complex_query`
  - Expected: Mean < 300.0 ms, p95 < 400.0 ms

### Re-ranking Performance

- [ ] **Re-ranking < 50ms (100 results)**
  - Benchmark: `bench_reranking`
  - Command: `cargo bench --bench query_bench bench_reranking`
  - Expected: Mean < 50.0 ms, p95 < 75.0 ms

---

## Accuracy

### Cache Accuracy

- [ ] **Cached results identical to fresh results**
  - Test: `test_cache_accuracy()`
  - Command: `cargo test --package agentsdk-search test_cache_accuracy`
  - Expected: PASS, cached results match fresh query

- [ ] **Cache invalidation prevents stale results**
  - Test: `test_cache_invalidation_accuracy()`
  - Command: `cargo test --package agentsdk-search test_cache_invalidation_accuracy`
  - Expected: PASS, stale results not returned

### Re-ranking Accuracy

- [ ] **Re-ranking improves result order**
  - Test: `test_reranking_accuracy()`
  - Command: `cargo test --package agentsdk-search test_reranking_accuracy`
  - Expected: PASS, NDCG improved after re-ranking

- [ ] **Filters correctly exclude results**
  - Test: `test_filter_accuracy()`
  - Command: `cargo test --package agentsdk-search test_filter_accuracy`
  - Expected: PASS, only matching results returned

---

## Error Handling

### Query Errors

- [ ] **Handles invalid filters**
  - Test: `test_invalid_filter_error()`
  - Command: `cargo test --package agentsdk-search test_invalid_filter_error`
  - Expected: PASS, returns error with helpful message

- [ ] **Handles cache errors gracefully**
  - Test: `test_cache_error_handling()`
  - Command: `cargo test --package agentsdk-search test_cache_error_handling`
  - Expected: PASS, query continues with fresh results

- [ ] **Returns error for invalid queries**
  - Test: `test_invalid_query_error()`
  - Command: `cargo test --package agentsdk-search test_invalid_query_error`
  - Expected: PASS, error returned with validation details

---

## ADR-0006 Compliance

### Local Memory First

- [ ] **Local memory exhausted before external search**
  - Test: `test_local_first_search()`
  - Command: `cargo test --package agentsdk-search test_local_first_search`
  - Expected: PASS, no external search calls for local queries

- [ ] **Query engine prioritizes local results**
  - Test: `test_local_priority()`
  - Command: `cargo test --package agentsdk-search test_local_priority`
  - Expected: PASS, local results preferred

### Provenance Tracking

- [ ] **Search cache respects provenance**
  - Test: `test_cache_provenance()`
  - Command: `cargo test --package agentsdk-search test_cache_provenance`
  - Expected: PASS, cache entries include trace IDs

- [ ] **All search operations track provenance**
  - Test: `test_search_provenance()`
  - Command: `cargo test --package agentsdk-search test_search_provenance`
  - Expected: PASS, all queries logged with trace ID

---

## Integration Points

### Full-Text Search Integration

- [ ] **Queries full-text index**
  - Test: `test_fulltext_query()`
  - Command: `cargo test --package agentsdk-search test_fulltext_query`
  - Expected: PASS, full-text search executed

### Semantic Search Integration

- [ ] **Queries semantic index**
  - Test: `test_semantic_query()`
  - Command: `cargo test --package agentsdk-search test_semantic_query`
  - Expected: PASS, semantic search executed

### Hybrid Fusion Integration

- [ ] **Combines fulltext and semantic results**
  - Test: `test_hybrid_combination()`
  - Command: `cargo test --package agentsdk-search test_hybrid_combination`
  - Expected: PASS, results from both sources fused

---

## Log Verification Patterns

### Query Execution Logs

- [ ] **Query execution logged**
  - Grep: `grep '"operation":"query"' ./workspace/logs/search.log | wc -l`
  - Expected: Count equals number of queries

- [ ] **Query parameters logged**
  - Grep: `grep '"operation":"query"' ./workspace/logs/search.log | jq -r '.query'`
  - Expected: Query parameters present

- [ ] **Query duration logged**
  - Grep: `grep '"operation":"query"' ./workspace/logs/search.log | jq -r '.duration_ms'`
  - Expected: Duration in milliseconds

### Cache Logs

- [ ] **Cache hits logged**
  - Grep: `grep '"cache":"hit"' ./workspace/logs/search.log | wc -l`
  - Expected: Count equals number of cache hits

- [ ] **Cache misses logged**
  - Grep: `grep '"cache":"miss"' ./workspace/logs/search.log | wc -l`
  - Expected: Count equals number of cache misses

- [ ] **Cache invalidations logged**
  - Grep: `grep '"operation":"cache_invalidate"' ./workspace/logs/search.log | wc -l`
  - Expected: Count equals number of invalidations

### Re-ranking Logs

- [ ] **Re-ranking operations logged**
  - Grep: `grep '"operation":"rerank"' ./workspace/logs/search.log | wc -l`
  - Expected: Count equals number of re-rankings

- [ ] **Re-ranking strategy logged**
  - Grep: `grep '"operation":"rerank"' ./workspace/logs/search.log | jq -r '.strategy'`
  - Expected: Strategy present for all re-rankings

---

## Test Coverage

### Unit Tests

- [ ] **Unit tests for query parsing**
  - Command: `cargo test --package agentsdk-search --lib query_parser`
  - Expected: All parsing tests pass

- [ ] **Unit tests for caching**
  - Command: `cargo test --package agentsdk-search --lib caching`
  - Expected: All cache tests pass

- [ ] **Unit tests for re-ranking**
  - Command: `cargo test --package agentsdk-search --lib reranking`
  - Expected: All re-ranking tests pass

### Integration Tests

- [ ] **Integration tests for search workflow**
  - Command: `cargo test --package agentsdk-search --test integration_test`
  - Expected: All integration tests pass

### Performance Benchmarks

- [ ] **Performance benchmarks**
  - Command: `cargo bench --bench query_bench`
  - Expected: All benchmarks complete, targets met

---

## Final Checklist

### Implementation Complete
- [ ] Query execution implemented and tested
- [ ] Caching functional
- [ ] Re-ranking working
- [ ] Filters working
- [ ] Pagination implemented

### ADR-0006 Compliant
- [ ] Local memory exhausted before external search
- [ ] Search cache respects provenance
- [ ] All search operations track provenance

### Integration Ready
- [ ] Queries full-text and semantic indexes
- [ ] Combines results from both sources
- [ ] Cache invalidates on index updates

### Documentation Complete
- [ ] API documentation generated
- [ ] Query language documented
- [ ] Cache configuration documented

### Tests Passing
- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] All benchmarks meet targets
- [ ] Test coverage > 90%

**Total Lines:** ~500 lines
