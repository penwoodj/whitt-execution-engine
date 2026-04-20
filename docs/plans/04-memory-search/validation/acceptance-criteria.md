# Phase 04 Memory & Search - Acceptance Criteria

## Overview

Complete acceptance criteria for Phase 04 Memory & Search, verifying all ADR-0005 constraints, performance targets, and integration requirements.

---

## ADR-0005 Constraints Verification

### Constraint 1: Local Memory First

- [ ] **Local memory stores/retrieves reliably (1000+ artifacts)**
  - Test: `test_local_memory_capacity()`
  - Command: `cargo test --package agentsdk-memory --test acceptance_local_capacity`
  - Expected: PASS, 1000+ artifacts stored and retrieved successfully

- [ ] **Local memory exhausted before external search**
  - Test: `test_local_exhaustion()`
  - Command: `cargo test --package agentsdk-memory-search --test acceptance_local_exhaustion`
  - Expected: PASS, external search only called when local results insufficient

- [ ] **No premature external search calls**
  - Test: `test_no_premature_external()`
  - Command: `cargo test --package agentsdk-external --test acceptance_no_premature`
  - Expected: PASS, external search only after local threshold

### Constraint 2: Two Retrieval Planes

- [ ] **Plane 1: Local memory implemented (structured + unstructured)**
  - Test: `test_local_planes_complete()`
  - Command: `cargo test --package agentsdk-memory --test acceptance_local_planes`
  - Expected: PASS, both structured and unstructured memory working

- [ ] **Plane 2: External search behind policy gates**
  - Test: `test_external_plane_gated()`
  - Command: `cargo test --package agentsdk-external --test acceptance_external_gated`
  - Expected: PASS, all external searches require approval

- [ ] **Results from both planes can be merged**
  - Test: `test_plane_merging()`
  - Command: `cargo test --package agentsdk-search --test acceptance_plane_merging`
  - Expected: PASS, results from both planes combined

### Constraint 3: Hybrid Search

- [ ] **Full-text search works with tantivy**
  - Test: `test_fulltext_tantivy()`
  - Command: `cargo test --package agentsdk-search --test acceptance_fulltext_tantivy`
  - Expected: PASS, tantivy index functional

- [ ] **Query latency < 100ms for 10K docs**
  - Benchmark: `bench_fulltext_10k`
  - Command: `cargo bench --bench search_bench bench_fulltext_10k`
  - Expected: Mean < 100.0 ms, p95 < 150.0 ms

- [ ] **Semantic search works with local embedding model**
  - Test: `test_semantic_local_model()`
  - Command: `cargo test --package agentsdk-search --test acceptance_semantic_local`
  - Expected: PASS, local embedding model functional

- [ ] **Hybrid search combines exact + semantic correctly**
  - Test: `test_hybrid_combination()`
  - Command: `cargo test --package agentsdk-search --test acceptance_hybrid_combination`
  - Expected: PASS, fusion algorithm combines both correctly

- [ ] **Vectors not used alone (always with fulltext)**
  - Test: `test_no_standalone_vectors()`
  - Command: `cargo test --package agentsdk-search --test acceptance_no_standalone`
  - Expected: PASS, vector search only available through hybrid

### Constraint 4: External Search Policy Gates

- [ ] **External search behind policy gates (require approval)**
  - Test: `test_policy_gate_required()`
  - Command: `cargo test --package agentsdk-external --test acceptance_policy_gate`
  - Expected: PASS, all external searches require user approval

- [ ] **Rate limits enforced**
  - Test: `test_rate_limits_enforced()`
  - Command: `cargo test --package agentsdk-external --test acceptance_rate_limits`
  - Expected: PASS, no requests exceed configured limits

- [ ] **User consent required for each search**
  - Test: `test_user_consent_required()`
  - Command: `cargo test --package agentsdk-external --test acceptance_consent`
  - Expected: PASS, each search requires explicit consent

### Constraint 5: Web Scraping Constraints

- [ ] **Scraping respects robots.txt (reject non-compliant domains)**
  - Test: `test_robots_compliance()`
  - Command: `cargo test --package agentsdk-scraping --test acceptance_robots`
  - Expected: PASS, disallowed paths blocked 100%

- [ ] **Robots.txt checked before all requests**
  - Test: `test_robots_first()`
  - Command: `cargo test --package agentsdk-scraping --test acceptance_robots_first`
  - Expected: PASS, no request skips robots.txt check

- [ ] **Scope restrictions enforced**
  - Test: `test_scope_enforcement()`
  - Command: `cargo test --package agentsdk-scraping --test acceptance_scope`
  - Expected: PASS, no violations of allowed/blocked domains

- [ ] **Fail closed on robots.txt errors**
  - Test: `test_robots_fail_closed()`
  - Command: `cargo test --package agentsdk-scraping --test acceptance_fail_closed`
  - Expected: PASS, denies on fetch/parse errors

### Constraint 6: Provenance Tracking

- [ ] **Provenance captures timestamps for ALL operations**
  - Test: `test_timestamp_coverage()`
  - Command: `cargo test --package agentsdk-provenance --test acceptance_timestamps`
  - Expected: PASS, 100% of operations have timestamps

- [ ] **Timestamps are immutable**
  - Test: `test_timestamp_immutability()`
  - Command: `cargo test --package agentsdk-provenance --test acceptance_immutable`
  - Expected: PASS, timestamps cannot be modified

- [ ] **Content hashes for all memory/web ops**
  - Test: `test_hash_coverage()`
  - Command: `cargo test --package agentsdk-provenance --test acceptance_hashes`
  - Expected: PASS, all content operations include SHA-256 hash

- [ ] **All provenance is queryable**
  - Test: `test_queryable_provenance()`
  - Command: `cargo test --package agentsdk-provenance --test acceptance_queryable`
  - Expected: PASS, all traces queryable

### Constraint 7: Memory Artifacts

- [ ] **Memory artifacts in ./workspace/memory/ with versioned references**
  - Test: `test_memory_location()`
  - Command: `cargo test --package agentsdk-memory --test acceptance_location`
  - Expected: PASS, all artifacts in correct directory

- [ ] **Versioned references maintained**
  - Test: `test_versioned_refs()`
  - Command: `cargo test --package agentsdk-memory --test acceptance_versioned`
  - Expected: PASS, all versions accessible via references

### Constraint 8: Garbage Collection

- [ ] **GC prevents unbounded growth (verify with memory pressure test)**
  - Test: `test_gc_prevents_growth()`
  - Command: `cargo test --package agentsdk-garbage --test acceptance_growth`
  - Expected: PASS, memory stays under configured limits

- [ ] **GC policies configurable**
  - Test: `test_gc_policies_configurable()`
  - Command: `cargo test --package agentsdk-garbage --test acceptance_policies`
  - Expected: PASS, all policies can be customized

- [ ] **Preview shows what will be deleted**
  - Test: `test_gc_preview()`
  - Command: `cargo test --package agentsdk-garbage --test acceptance_preview`
  - Expected: PASS, preview accurate to actual deletion

- [ ] **Recovery can restore deleted data**
  - Test: `test_gc_recovery()`
  - Command: `cargo test --package agentsdk-garbage --test acceptance_recovery`
  - Expected: PASS, deleted items recoverable

---

## Performance Targets Verification

### Memory Storage Performance

- [ ] **Create operation < 10ms**
  - Benchmark: `bench_memory_create`
  - Command: `cargo bench --bench memory_bench bench_create`
  - Expected: Mean < 10.0 ms, p95 < 15.0 ms

- [ ] **Read operation < 5ms**
  - Benchmark: `bench_memory_read`
  - Command: `cargo bench --bench memory_bench bench_read`
  - Expected: Mean < 5.0 ms, p95 < 8.0 ms

- [ ] **Update operation < 15ms**
  - Benchmark: `bench_memory_update`
  - Command: `cargo bench --bench memory_bench bench_update`
  - Expected: Mean < 15.0 ms, p95 < 20.0 ms

- [ ] **Delete operation < 10ms**
  - Benchmark: `bench_memory_delete`
  - Command: `cargo bench --bench memory_bench bench_delete`
  - Expected: Mean < 10.0 ms, p95 < 15.0 ms

- [ ] **List operation < 50ms for 1000 items**
  - Benchmark: `bench_memory_list`
  - Command: `cargo bench --bench memory_bench bench_list_1000`
  - Expected: Mean < 50.0 ms, p95 < 75.0 ms

### Full-Text Search Performance

- [ ] **Index single document < 50ms**
  - Benchmark: `bench_index_document`
  - Command: `cargo bench --bench search_bench bench_index_single`
  - Expected: Mean < 50.0 ms, p95 < 75.0 ms

- [ ] **Index 1000 documents < 5 seconds**
  - Benchmark: `bench_index_1000`
  - Command: `cargo bench --bench search_bench bench_index_1000`
  - Expected: Mean < 5.0s, p95 < 6.0s

- [ ] **Search operation < 100ms**
  - Benchmark: `bench_search_operation`
  - Command: `cargo bench --bench search_bench bench_search`
  - Expected: Mean < 100.0 ms, p95 < 150.0 ms

- [ ] **Search across 10000 documents < 200ms**
  - Benchmark: `bench_search_10k`
  - Command: `cargo bench --bench search_bench bench_search_10k`
  - Expected: Mean < 200.0 ms, p95 < 250.0 ms

### Semantic Search Performance

- [ ] **Generate single embedding < 100ms**
  - Benchmark: `bench_embedding_single`
  - Command: `cargo bench --bench semantic_bench bench_single_embedding`
  - Expected: Mean < 100.0 ms, p95 < 150.0 ms

- [ ] **Generate batch embeddings < 500ms (10 items)**
  - Benchmark: `bench_embedding_batch_10`
  - Command: `cargo bench --bench semantic_bench bench_batch_10`
  - Expected: Mean < 500.0 ms, p95 < 600.0 ms

- [ ] **Similarity search < 50ms (1000 vectors)**
  - Benchmark: `bench_similarity_1k`
  - Command: `cargo bench --bench semantic_bench bench_similarity_1k`
  - Expected: Mean < 50.0 ms, p95 < 75.0 ms

- [ ] **Hybrid search < 200ms**
  - Benchmark: `bench_hybrid_search`
  - Command: `cargo bench --bench search_bench bench_hybrid`
  - Expected: Mean < 200.0 ms, p95 < 250.0 ms

### Query Engine Performance

- [ ] **Cached query < 5ms**
  - Benchmark: `bench_cached_query`
  - Command: `cargo bench --bench query_bench bench_cached`
  - Expected: Mean < 5.0 ms, p95 < 10.0 ms

- [ ] **Uncached query < 200ms**
  - Benchmark: `bench_uncached_query`
  - Command: `cargo bench --bench query_bench bench_uncached`
  - Expected: Mean < 200.0 ms, p95 < 250.0 ms

- [ ] **Cache cleanup efficient (< 1s for 1000 entries)**
  - Benchmark: `bench_cache_cleanup`
  - Command: `cargo bench --bench query_bench bench_cleanup`
  - Expected: Mean < 1.0s, p95 < 1.5s

### External Search Performance

- [ ] **Policy check < 1ms**
  - Benchmark: `bench_policy_check`
  - Command: `cargo bench --bench external_bench bench_policy`
  - Expected: Mean < 1.0 ms, p95 < 2.0 ms

- [ ] **Rate limit check < 1ms**
  - Benchmark: `bench_rate_limit`
  - Command: `cargo bench --bench external_bench bench_rate_limit`
  - Expected: Mean < 1.0 ms, p95 < 2.0 ms

- [ ] **External search < 2s**
  - Benchmark: `bench_external_search`
  - Command: `cargo bench --bench external_bench bench_search`
  - Expected: Mean < 2.0s, p95 < 3.0s

### Web Scraping Performance

- [ ] **robots.txt fetch < 500ms**
  - Benchmark: `bench_robots_fetch`
  - Command: `cargo bench --bench scraping_bench bench_robots`
  - Expected: Mean < 500.0 ms, p95 < 750.0 ms

- [ ] **robots.txt parse < 10ms**
  - Benchmark: `bench_robots_parse`
  - Command: `cargo bench --bench scraping_bench bench_parse`
  - Expected: Mean < 10.0 ms, p95 < 15.0 ms

- [ ] **Scope check < 1ms**
  - Benchmark: `bench_scope_check`
  - Command: `cargo bench --bench scraping_bench bench_scope`
  - Expected: Mean < 1.0 ms, p95 < 2.0 ms

- [ ] **Content extraction < 2s**
  - Benchmark: `bench_extraction`
  - Command: `cargo bench --bench scraping_bench bench_extract`
  - Expected: Mean < 2.0s, p95 < 3.0s

### Provenance Tracking Performance

- [ ] **Record trace < 1ms**
  - Benchmark: `bench_record_trace`
  - Command: `cargo bench --bench provenance_bench bench_record`
  - Expected: Mean < 1.0 ms, p95 < 2.0 ms

- [ ] **Query by trace ID < 5ms**
  - Benchmark: `bench_query_trace_id`
  - Command: `cargo bench --bench provenance_bench bench_query_id`
  - Expected: Mean < 5.0 ms, p95 < 10.0 ms

- [ ] **Query by operation type < 50ms**
  - Benchmark: `bench_query_operation`
  - Command: `cargo bench --bench provenance_bench bench_query_operation`
  - Expected: Mean < 50.0 ms, p95 < 75.0 ms

- [ ] **Build timeline < 100ms**
  - Benchmark: `bench_build_timeline`
  - Command: `cargo bench --bench provenance_bench bench_timeline`
  - Expected: Mean < 100.0 ms, p95 < 150.0 ms

### Garbage Collection Performance

- [ ] **Policy evaluation < 1ms per item**
  - Benchmark: `bench_policy_eval`
  - Command: `cargo bench --bench garbage_bench bench_policy`
  - Expected: Mean < 1.0 ms, p95 < 2.0 ms

- [ ] **Preview generation < 1s for 1000 items**
  - Benchmark: `bench_preview`
  - Command: `cargo bench --bench garbage_bench bench_preview`
  - Expected: Mean < 1.0s, p95 < 1.5s

- [ ] **Backup creation < 5s for 1000 items**
  - Benchmark: `bench_backup`
  - Command: `cargo bench --bench garbage_bench bench_backup`
  - Expected: Mean < 5.0s, p95 < 6.0s

- [ ] **Restore operation < 5s for 1000 items**
  - Benchmark: `bench_restore`
  - Command: `cargo bench --bench garbage_bench bench_restore`
  - Expected: Mean < 5.0s, p95 < 6.0s

### Integration Performance

- [ ] **Tool invocation < 200ms (local ops)**
  - Benchmark: `bench_tool_invocation`
  - Command: `cargo bench --bench integration_bench bench_tool`
  - Expected: Mean < 200.0 ms, p95 < 300.0 ms

- [ ] **CLI command execution < 500ms (local ops)**
  - Benchmark: `bench_cli_execution`
  - Command: `cargo bench --bench integration_bench bench_cli`
  - Expected: Mean < 500.0 ms, p95 < 750.0 ms

- [ ] **Workflow node execution < 500ms**
  - Benchmark: `bench_workflow_node`
  - Command: `cargo bench --bench integration_bench bench_node`
  - Expected: Mean < 500.0 ms, p95 < 750.0 ms

---

## Integration Verification

### Tool System Integration

- [ ] **Memory search tool works**
  - Test: `test_tool_search()`
  - Command: `cargo test --package agentsdk-memory-search --test acceptance_tool_search`
  - Expected: PASS, search functional via tools

- [ ] **Memory storage tool works**
  - Test: `test_tool_storage()`
  - Command: `cargo test --package agentsdk-memory-search --test acceptance_tool_storage`
  - Expected: PASS, storage functional via tools

- [ ] **Memory retrieve tool works**
  - Test: `test_tool_retrieve()`
  - Command: `cargo test --package agentsdk-memory-search --test acceptance_tool_retrieve`
  - Expected: PASS, retrieval functional via tools

### CLI Integration

- [ ] **Search command works**
  - Test: `test_cli_search()`
  - Command: `cargo test --package agentsdk-memory-search --test acceptance_cli_search`
  - Expected: PASS, search functional via CLI

- [ ] **Store command works**
  - Test: `test_cli_store()`
  - Command: `cargo test --package agentsdk-memory-search --test acceptance_cli_store`
  - Expected: PASS, storage functional via CLI

- [ ] **Retrieve command works**
  - Test: `test_cli_retrieve()`
  - Command: `cargo test --package agentsdk-memory-search --test acceptance_cli_retrieve`
  - Expected: PASS, retrieval functional via CLI

- [ ] **List command works**
  - Test: `test_cli_list()`
  - Command: `cargo test --package agentsdk-memory-search --test acceptance_cli_list`
  - Expected: PASS, listing functional via CLI

- [ ] **Provenance command works**
  - Test: `test_cli_provenance()`
  - Command: `cargo test --package agentsdk-memory-search --test acceptance_cli_provenance`
  - Expected: PASS, provenance functional via CLI

- [ ] **GC command works**
  - Test: `test_cli_gc()`
  - Command: `cargo test --package agentsdk-memory-search --test acceptance_cli_gc`
  - Expected: PASS, GC functional via CLI

### Workflow Engine Integration

- [ ] **Tools available as workflow nodes**
  - Test: `test_workflow_nodes()`
  - Command: `cargo test --package agentsdk-memory-search --test acceptance_workflow_nodes`
  - Expected: PASS, tools accessible as nodes

- [ ] **Context injection works**
  - Test: `test_context_injection()`
  - Command: `cargo test --package agentsdk-memory-search --test acceptance_context`
  - Expected: PASS, memory content in LLM context

- [ ] **Provenance tracking works**
  - Test: `test_workflow_provenance()`
  - Command: `cargo test --package agentsdk-memory-search --test acceptance_workflow_provenance`
  - Expected: PASS, traces recorded for workflow ops

- [ ] **Error handling works**
  - Test: `test_workflow_errors()`
  - Command: `cargo test --package agentsdk-memory-search --test acceptance_workflow_errors`
  - Expected: PASS, failures handled gracefully

---

## Verification Layers

### Layer 1: Unit Tests

- [ ] **All unit tests pass**
  - Command: `cargo test --workspace`
  - Expected: All unit tests pass (>95% code coverage)

- [ ] **No critical issues in code**
  - Command: `cargo clippy --workspace`
  - Expected: No clippy warnings or errors

### Layer 2: Integration Tests

- [ ] **All integration tests pass**
  - Command: `cargo test --test '*integration*'`
  - Expected: All integration tests pass

- [ ] **Cross-crate integration works**
  - Command: `cargo test --test '*cross*'`
  - Expected: All cross-crate tests pass

### Layer 3: Performance Benchmarks

- [ ] **All performance targets met**
  - Command: `cargo bench --workspace`
  - Expected: All benchmarks meet latency targets

- [ ] **No performance regressions**
  - Command: `cargo bench --workspace`
  - Expected: Performance stable or improved

### Layer 4: Schema Compliance

- [ ] **All schemas match specifications**
  - Command: `cargo test --test '*schema*'`
  - Expected: All schema tests pass

- [ ] **JSON schemas validated**
  - Command: `cat ./workspace/schemas/*.json | jq .`
  - Expected: All schemas valid

### Layer 5: ADR-0006 Compliance

- [ ] **All ADR-0006 constraints satisfied**
  - Command: `cargo test --test '*adr0006*'`
  - Expected: All ADR-0006 tests pass

- [ ] **No violations of local-first principle**
  - Command: `cargo test --test '*local_first*'`
  - Expected: No external search without local exhaustion

### Layer 6: Error Handling

- [ ] **All error cases tested**
  - Command: `cargo test --test '*error*'`
  - Expected: All error handling tests pass

- [ ] **Error messages helpful and actionable**
  - Command: `cargo test --test '*error_message*'`
  - Expected: Error messages include guidance

### Layer 7: Log Verification

- [ ] **All operations logged**
  - Command: `grep -c '"operation":' ./workspace/logs/*.log`
  - Expected: Count matches operation count

- [ ] **Logs include required fields**
  - Command: `jq -r 'keys' ./workspace/logs/*.log | head -1`
  - Expected: Required fields present

---

## Final Checklist

### Implementation Complete
- [ ] All 9 tasks implemented and passing their validation criteria
- [ ] All tests pass with proper mock strategies
- [ ] ADR-0006 compliance verified through full checklist
- [ ] Integration tests demonstrate end-to-end memory & search workflow

### Performance Met
- [ ] Search latency < 100ms for local
- [ ] Search latency < 2s for external
- [ ] All latency targets met
- [ ] No performance regressions

### Memory Constraints
- [ ] Memory constraints documented and enforced
- [ ] GC prevents unbounded growth
- [ ] Memory usage efficient

### Provenance Complete
- [ ] Provenance tracking is complete
- [ ] Provenance is queryable
- [ ] All operations have trace IDs

### Documentation Complete
- [ ] Documentation covers all APIs
- [ ] Configuration documented
- [ ] Usage patterns documented

### Tests Passing
- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] All benchmarks meet targets
- [ ] Test coverage > 90%

**Total Lines:** ~560 lines
