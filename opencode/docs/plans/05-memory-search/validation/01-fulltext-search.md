# Validation Criteria: Task 01 - Full-Text Search

## Overview

Validate that full-text search system provides efficient indexing, query parsing, relevance ranking, and result highlighting over local memory using Tantivy with ADR-0006 compliance.

---

## Checkpoint Gate Criteria

### Checkpoint 1: Indexing Complete
- [ ] **Tantivy index implemented**: Index builder with schema
- [ ] **Schema definitions complete**: Fields for title, content, tags, metadata
- [ ] **Indexer interface defined**: Async operations for add/update/delete
- [ ] **Unit tests pass**: All indexing tests (`cargo test --package agentsdk-search`)

**Verification Commands:**
```bash
# Verify index builds
cargo check --package agentsdk-search

# Run indexing tests
cargo test --package agentsdk-search --lib indexing

# Expected output: All indexing tests pass
```

### Checkpoint 2: Query Parsing Functional
- [ ] **Query parser implemented**: Handles boolean operators, phrases, wildcards
- [ ] **Query validation works**: Rejects malformed queries gracefully
- [ ] **Query optimization works**: Simplifies and optimizes queries
- [ ] **Unit tests pass**: All parsing tests

**Verification Commands:**
```bash
# Run parser tests
cargo test --package agentsdk-search --lib query_parser

# Verify query edge cases
cargo test --package agentsdk-search --test query_edge_cases

# Expected output: All parser tests pass
```

### Checkpoint 3: Search and Ranking Working
- [ ] **Search execution works**: Returns ranked results
- [ ] **Relevance scoring works**: BM25 scoring implemented
- [ ] **Highlighting works**: Term highlighting in snippets
- [ ] **Pagination works**: Limit/offset pagination

**Verification Commands:**
```bash
# Run search tests
cargo test --package agentsdk-search --lib search

# Verify ranking
cargo test --package agentsdk-search --test ranking

# Expected output: All search tests pass
```

### Checkpoint 4: Performance Meets Targets
- [ ] **Index single document < 50ms**: Measured with benchmarks
- [ ] **Search operation < 100ms**: Measured with benchmarks
- [ ] **Index 1000 documents < 5 seconds**: Measured with benchmarks
- [ ] **Search across 10000 documents < 200ms**: Measured with benchmarks

**Verification Commands:**
```bash
# Run performance benchmarks
cargo bench --package agentsdk-search

# Verify indexing performance
cargo bench --bench search_bench bench_index_document

# Verify search performance
cargo bench --bench search_bench bench_search_10k

# Expected output: All latency targets met
```

---

## Functional Requirements

### Indexing

#### Document Indexing
- [ ] **Can index structured memory**
  - Test: `test_index_structured_memory()`
  - Command: `cargo test --package agentsdk-search test_index_structured_memory`
  - Expected: PASS, document appears in index

- [ ] **Can index unstructured memory**
  - Test: `test_index_unstructured_memory()`
  - Command: `cargo test --package agentsdk-search test_index_unstructured_memory`
  - Expected: PASS, document appears in index

- [ ] **Index updates on memory modification**
  - Test: `test_index_on_update()`
  - Command: `cargo test --package agentsdk-search test_index_on_update`
  - Expected: PASS, index reflects updated content

- [ ] **Index removes on memory deletion**
  - Test: `test_index_on_delete()`
  - Command: `cargo test --package agentsdk-search test_index_on_delete`
  - Expected: PASS, document removed from index

#### Batch Indexing
- [ ] **Can index batch of documents**
  - Test: `test_batch_index()`
  - Command: `cargo test --package agentsdk-search test_batch_index`
  - Expected: PASS, all documents indexed

- [ ] **Batch indexing handles errors gracefully**
  - Test: `test_batch_index_with_errors()`
  - Command: `cargo test --package agentsdk-search test_batch_index_with_errors`
  - Expected: PASS, valid documents indexed, errors logged

#### Index Management
- [ ] **Can create index with custom schema**
  - Test: `test_create_index_schema()`
  - Command: `cargo test --package agentsdk-search test_create_index_schema`
  - Expected: PASS, index created with specified fields

- [ ] **Can rebuild index from scratch**
  - Test: `test_rebuild_index()`
  - Command: `cargo test --package agentsdk-search test_rebuild_index`
  - Expected: PASS, index rebuilt with all documents

- [ ] **Can optimize index**
  - Test: `test_optimize_index()`
  - Command: `cargo test --package agentsdk-search test_optimize_index`
  - Expected: PASS, index size reduced, search unchanged

### Search

#### Query Types
- [ ] **Can search by text query**
  - Test: `test_simple_search()`
  - Command: `cargo test --package agentsdk-search test_simple_search`
  - Expected: PASS, returns matching documents

- [ ] **Can search in multiple fields (content, title, tags)**
  - Test: `test_multi_field_search()`
  - Command: `cargo test --package agentsdk-search test_multi_field_search`
  - Expected: PASS, searches across specified fields

- [ ] **Returns relevant results ranked by score**
  - Test: `test_ranked_results()`
  - Command: `cargo test --package agentsdk-search test_ranked_results`
  - Expected: PASS, results sorted by relevance score

- [ ] **Supports pagination (limit/offset)**
  - Test: `test_pagination()`
  - Command: `cargo test --package agentsdk-search test_pagination`
  - Expected: PASS, correct page of results returned

### Query Parsing

#### Simple Queries
- [ ] **Parses simple text queries**
  - Test: `test_parse_simple_query()`
  - Command: `cargo test --package agentsdk-search test_parse_simple_query`
  - Expected: PASS, parsed into TermQuery

- [ ] **Handles whitespace correctly**
  - Test: `test_whitespace_handling()`
  - Command: `cargo test --package agentsdk-search test_whitespace_handling`
  - Expected: PASS, multiple terms handled correctly

#### Boolean Operators
- [ ] **Parses boolean operators (AND, OR, NOT)**
  - Test: `test_parse_boolean_operators()`
  - Command: `cargo test --package agentsdk-search test_parse_boolean_operators`
  - Expected: PASS, parsed into BooleanQuery

- [ ] **Handles operator precedence**
  - Test: `test_operator_precedence()`
  - Command: `cargo test --package agentsdk-search test_operator_precedence`
  - Expected: PASS, (NOT AND OR) precedence followed

- [ ] **Handles grouped expressions**
  - Test: `test_grouped_expressions()`
  - Command: `cargo test --package agentsdk-search test_grouped_expressions`
  - Expected: PASS, parentheses parsed correctly

#### Phrase Queries
- [ ] **Parses phrase queries (quoted strings)**
  - Test: `test_parse_phrase_query()`
  - Command: `cargo test --package agentsdk-search test_parse_phrase_query`
  - Expected: PASS, parsed into PhraseQuery

- [ ] **Handles phrase with wildcards**
  - Test: `test_phrase_with_wildcard()`
  - Command: `cargo test --package agentsdk-search test_phrase_with_wildcard`
  - Expected: PASS, phrase with wildcards parsed

#### Wildcards and Fuzzy
- [ ] **Parses wildcard queries (*, ?)**
  - Test: `test_parse_wildcard_query()`
  - Command: `cargo test --package agentsdk-search test_parse_wildcard_query`
  - Expected: PASS, parsed into WildcardQuery

- [ ] **Parses fuzzy queries (tilde)**
  - Test: `test_parse_fuzzy_query()`
  - Command: `cargo test --package agentsdk-search test_parse_fuzzy_query`
  - Expected: PASS, parsed into FuzzyQuery

#### Error Handling
- [ ] **Handles query syntax errors gracefully**
  - Test: `test_syntax_error_handling()`
  - Command: `cargo test --package agentsdk-search test_syntax_error_handling`
  - Expected: PASS, returns error with helpful message

- [ ] **Handles empty query**
  - Test: `test_empty_query_handling()`
  - Command: `cargo test --package agentsdk-search test_empty_query_handling`
  - Expected: PASS, returns error or empty result

- [ ] **Handles overly complex query**
  - Test: `test_complex_query_limit()`
  - Command: `cargo test --package agentsdk-search test_complex_query_limit`
  - Expected: PASS, rejects queries exceeding depth limit

### Ranking

#### BM25 Scoring
- [ ] **Results ranked by BM25 relevance score**
  - Test: `test_bm25_ranking()`
  - Command: `cargo test --package agentsdk-search test_bm25_ranking`
  - Expected: PASS, scores calculated correctly

- [ ] **Scores normalized for comparison**
  - Test: `test_score_normalization()`
  - Command: `cargo test --package agentsdk-search test_score_normalization`
  - Expected: PASS, scores in [0, 1] range

- [ ] **Supports custom ranking algorithms**
  - Test: `test_custom_ranking()`
  - Command: `cargo test --package agentsdk-search test_custom_ranking`
  - Expected: PASS, custom ranking function applied

#### Re-ranking
- [ ] **Can re-rank by recency**
  - Test: `test_recency_rerank()`
  - Command: `cargo test --package agentsdk-search test_recency_rerank`
  - Expected: PASS, newer documents promoted

- [ ] **Can re-rank by popularity**
  - Test: `test_popularity_rerank()`
  - Command: `cargo test --package agentsdk-search test_popularity_rerank`
  - Expected: PASS, frequently accessed documents promoted

### Highlighting

#### Term Highlighting
- [ ] **Highlights matching terms in results**
  - Test: `test_term_highlighting()`
  - Command: `cargo test --package agentsdk-search test_term_highlighting`
  - Expected: PASS, matching terms wrapped in highlight tags

- [ ] **Provides context snippets**
  - Test: `test_context_snippets()`
  - Command: `cargo test --package agentsdk-search test_context_snippets`
  - Expected: PASS, snippets include surrounding text

- [ ] **Configurable highlight tags**
  - Test: `test_highlight_tags()`
  - Command: `cargo test --package agentsdk-search test_highlight_tags`
  - Expected: PASS, custom highlight tags used

#### Snippet Generation
- [ ] **Generates snippets from best matches**
  - Test: `test_snippet_generation()`
  - Command: `cargo test --package agentsdk-search test_snippet_generation`
  - Expected: PASS, snippets show relevant context

- [ ] **Limits snippet length**
  - Test: `test_snippet_length_limit()`
  - Command: `cargo test --package agentsdk-search test_snippet_length_limit`
  - Expected: PASS, snippets respect max length

---

## Performance Requirements

### Indexing Performance

- [ ] **Index single document < 50ms**
  - Benchmark: `bench_index_document`
  - Command: `cargo bench --bench search_bench bench_index_document`
  - Expected: Mean < 50.0 ms, p95 < 75.0 ms

- [ ] **Index 1000 documents < 5 seconds**
  - Benchmark: `bench_index_1000`
  - Command: `cargo bench --bench search_bench bench_index_1000`
  - Expected: Mean < 5.0s, p95 < 6.0s

- [ ] **Index 10000 documents < 30 seconds**
  - Benchmark: `bench_index_10000`
  - Command: `cargo bench --bench search_bench bench_index_10000`
  - Expected: Mean < 30.0s, p95 < 35.0s

### Search Performance

- [ ] **Search operation < 100ms**
  - Benchmark: `bench_search_simple`
  - Command: `cargo bench --bench search_bench bench_search_simple`
  - Expected: Mean < 100.0 ms, p95 < 150.0 ms

- [ ] **Search across 10000 documents < 200ms**
  - Benchmark: `bench_search_10k`
  - Command: `cargo bench --bench search_bench bench_search_10k`
  - Expected: Mean < 200.0 ms, p95 < 250.0 ms

- [ ] **Complex boolean query < 250ms**
  - Benchmark: `bench_search_complex`
  - Command: `cargo bench --bench search_bench bench_search_complex`
  - Expected: Mean < 250.0 ms, p95 < 300.0 ms

### Memory Usage

- [ ] **Index memory usage efficient**
  - Test: `test_index_memory_usage()`
  - Command: `cargo test --package agentsdk-search test_index_memory_usage`
  - Expected: < 1GB for 10000 documents

- [ ] **Index size on disk reasonable**
  - Command: `du -sh .glyphnova/memory/index/fulltext/`
  - Expected: < 500MB for 10000 documents

---

## Accuracy

### Search Precision

- [ ] **Search finds relevant results (>90% precision)**
  - Test: `test_search_precision()`
  - Command: `cargo test --package agentsdk-search test_search_precision`
  - Expected: Precision > 0.90

- [ ] **Search returns most relevant results first**
  - Test: `test_ranking_order()`
  - Command: `cargo test --package agentsdk-search test_ranking_order`
  - Expected: NDCG@10 > 0.85

- [ ] **No false negatives for exact matches**
  - Test: `test_exact_match_recall()`
  - Command: `cargo test --package agentsdk-search test_exact_match_recall`
  - Expected: Recall = 1.0 for exact terms

### Query Interpretation

- [ ] **Boolean operators interpreted correctly**
  - Test: `test_boolean_correctness()`
  - Command: `cargo test --package agentsdk-search test_boolean_correctness`
  - Expected: Correct result sets for all operators

- [ ] **Phrase queries interpreted correctly**
  - Test: `test_phrase_correctness()`
  - Command: `cargo test --package agentsdk-search test_phrase_correctness`
  - Expected: Only exact phrase matches returned

- [ ] **Wildcard patterns expanded correctly**
  - Test: `test_wildcard_correctness()`
  - Command: `cargo test --package agentsdk-search test_wildcard_correctness`
  - Expected: All matching terms found

---

## Error Handling

### Query Errors

- [ ] **Handles empty queries gracefully**
  - Test: `test_empty_query_error()`
  - Command: `cargo test --package agentsdk-search test_empty_query_error`
  - Expected: Returns error or empty result

- [ ] **Handles malformed query syntax**
  - Test: `test_malformed_query_error()`
  - Command: `cargo test --package agentsdk-search test_malformed_query_error`
  - Expected: Returns error with helpful message

- [ ] **Returns empty result set for no matches**
  - Test: `test_no_matches_empty_result()`
  - Command: `cargo test --package agentsdk-search test_no_matches_empty_result`
  - Expected: Empty result set, no error

### Index Errors

- [ ] **Handles index errors gracefully**
  - Test: `test_index_error_handling()`
  - Command: `cargo test --package agentsdk-search test_index_error_handling`
  - Expected: Error logged, search continues with available data

- [ ] **Recovers from corrupted index**
  - Test: `test_corrupted_index_recovery()`
  - Command: `cargo test --package agentsdk-search test_corrupted_index_recovery`
  - Expected: Index rebuilt from memory storage

---

## Schema Compliance

### Index Schema

- [ ] **Fields match schema specification**
  - Test: `test_index_schema()`
  - Command: `cargo test --package agentsdk-search test_index_schema`
  - Expected: All required fields present, types correct

- [ ] **Field properties configured correctly**
  - Test: `test_field_properties()`
  - Command: `cargo test --package agentsdk-search test_field_properties`
  - Expected: Text, integer, date fields have correct properties

- [ ] **Facet fields configured for filtering**
  - Test: `test_facet_fields()`
  - Command: `cargo test --package agentsdk-search test_facet_fields`
  - Expected: Facet fields indexed for fast filtering

---

## ADR-0006 Compliance

### Search Index Location

- [ ] **Search indexes stored in `.glyphnova/memory/index/fulltext/`**
  - Test: `test_index_location()`
  - Command: `cargo test --package agentsdk-search test_index_location`
  - Expected: Index files in correct directory

- [ ] **No external index storage**
  - Test: `test_local_index_only()`
  - Command: `cargo test --package agentsdk-search test_local_index_only`
  - Expected: No network calls during index operations

### Provenance Tracking

- [ ] **All search operations track provenance**
  - Test: `test_search_provenance()`
  - Command: `cargo test --package agentsdk-search test_search_provenance`
  - Expected: Search events logged with trace ID

- [ ] **Search results include content hashes**
  - Test: `test_result_content_hashes()`
  - Command: `cargo test --package agentsdk-search test_result_content_hashes`
  - Expected: Each result includes SHA-256 hash

- [ ] **Index operations record timestamps**
  - Test: `test_index_timestamps()`
  - Command: `cargo test --package agentsdk-search test_index_timestamps`
  - Expected: All index operations include ISO 8601 timestamps

### Local-First Priority

- [ ] **Full-text search works entirely locally**
  - Test: `test_local_search()`
  - Command: `cargo test --package agentsdk-search test_local_search`
  - Expected: No external API calls

- [ ] **No fallback to external search**
  - Test: `test_no_external_fallback()`
  - Command: `cargo test --package agentsdk-search test_no_external_fallback`
  - Expected: Empty results propagated, no external access

---

## Integration Points

### Local Memory Integration

- [ ] **Auto-indexes on memory storage**
  - Test: `test_auto_index_on_store()`
  - Command: `cargo test --package agentsdk-search test_auto_index_on_store`
  - Expected: Memory automatically indexed when stored

- [ ] **Auto-updates on memory update**
  - Test: `test_auto_update_on_memory_update()`
  - Command: `cargo test --package agentsdk-search test_auto_update_on_memory_update`
  - Expected: Index updated when memory changes

- [ ] **Auto-removes on memory delete**
  - Test: `test_auto_remove_on_memory_delete()`
  - Command: `cargo test --package agentsdk-search test_auto_remove_on_memory_delete`
  - Expected: Index entry removed when memory deleted

### Semantic Search Integration

- [ ] **Full-text results can be combined with semantic**
  - Test: `test_hybrid_combination()`
  - Command: `cargo test --package agentsdk-search test_hybrid_combination`
  - Expected: Results from both sources merged

- [ ] **Scores are normalized before fusion**
  - Test: `test_score_normalization_for_fusion()`
  - Command: `cargo test --package agentsdk-search test_score_normalization_for_fusion`
  - Expected: Both score ranges normalized to [0, 1]

---

## Log Verification Patterns

### Index Operation Logs

- [ ] **Index operations logged with trace ID**
  - Grep: `grep '"operation":"index"' .glyphnova/logs/search.log | jq -r '.trace_id' | wc -l`
  - Expected: Count equals number of index operations

- [ ] **Document IDs logged**
  - Grep: `grep '"operation":"index"' .glyphnova/logs/search.log | jq -r '.document_id'`
  - Expected: All document IDs present

- [ ] **Index duration logged**
  - Grep: `grep '"operation":"index"' .glyphnova/logs/search.log | jq -r '.duration_ms'`
  - Expected: Duration in milliseconds for all operations

### Search Operation Logs

- [ ] **Search operations logged with query**
  - Grep: `grep '"operation":"search"' .glyphnova/logs/search.log | jq -r '.query'`
  - Expected: Query string present for all searches

- [ ] **Search results count logged**
  - Grep: `grep '"operation":"search"' .glyphnova/logs/search.log | jq -r '.result_count'`
  - Expected: Number of results returned

- [ ] **Search latency logged**
  - Grep: `grep '"operation":"search"' .glyphnova/logs/search.log | jq -r '.duration_ms'`
  - Expected: Latency in milliseconds for all searches

### Error Logs

- [ ] **Search errors logged with context**
  - Grep: `grep '"level":"error"' .glyphnova/logs/search.log | jq -r '.error'`
  - Expected: Error messages include query and operation details

- [ ] **Index errors logged**
  - Grep: `grep '"level":"error"' .glyphnova/logs/search.log | grep '"operation":"index"' | wc -l`
  - Expected: All index errors captured

---

## Test Coverage

### Unit Tests

- [ ] **Unit tests for indexing**
  - Command: `cargo test --package agentsdk-search --lib indexing`
  - Expected: All indexing tests pass

- [ ] **Unit tests for query parsing**
  - Command: `cargo test --package agentsdk-search --lib query_parser`
  - Expected: All parsing tests pass

- [ ] **Unit tests for ranking**
  - Command: `cargo test --package agentsdk-search --lib ranking`
  - Expected: All ranking tests pass

### Integration Tests

- [ ] **Integration tests for search workflow**
  - Command: `cargo test --package agentsdk-search --test integration_test`
  - Expected: All integration tests pass

- [ ] **Integration tests with memory storage**
  - Command: `cargo test --package agentsdk-search --test memory_integration`
  - Expected: All memory integration tests pass

### Performance Benchmarks

- [ ] **Performance benchmarks**
  - Command: `cargo bench --bench search_bench`
  - Expected: All benchmarks complete, targets met

- [ ] **Accuracy benchmarks**
  - Command: `cargo test --package agentsdk-search --test accuracy_benchmarks`
  - Expected: Precision, recall, NDCG metrics collected

---

## Final Checklist

### Implementation Complete
- [ ] Indexing implemented and tested
- [ ] Query parsing functional
- [ ] Search and ranking working
- [ ] Highlighting functional
- [ ] Performance meets all targets

### ADR-0006 Compliant
- [ ] Index in `.glyphnova/memory/index/fulltext/`
- [ ] All search operations track provenance
- [ ] Search results include content hashes
- [ ] No external search dependencies

### Integration Ready
- [ ] Auto-indexes on memory operations
- [ ] Works with semantic search
- [ ] Results can be fused with semantic

### Documentation Complete
- [ ] API documentation generated
- [ ] Query syntax guide provided
- [ ] Performance characteristics documented

### Tests Passing
- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] All benchmarks meet targets
- [ ] Test coverage > 90%

**Total Lines:** ~550 lines
