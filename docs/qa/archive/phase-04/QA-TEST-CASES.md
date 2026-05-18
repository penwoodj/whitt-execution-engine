# QA Test Cases — Phase 04: Memory & Search

**Schema**: `docs/schema/unified-workflow-schema.yml` (805 lines)
**Test ID Prefix**: P04
**Date**: 2026-04-26

---

## Test Case Summary Table

| Test ID | QA Area | Description | Command | Expected Result |
|----------|----------|-------------|----------|----------------|
| P04-001 | Local Memory Storage | Structured memory CRUD operations | cargo test --lib memory::storage::test_crud_operations | All CRUD operations pass |
| P04-002 | Local Memory Storage | Unstructured memory blob storage | cargo test --lib memory::storage::test_blob_storage | Blob storage works |
| P04-003 | Local Memory Storage | Document collection with metadata | cargo test --lib memory::storage::test_document_collections | Collections with metadata work |
| P04-004 | Local Memory Storage | Versioning system | cargo test --lib memory::storage::test_versioning | Versioning creates references correctly |
| P04-005 | Full-Text Search | Tantivy index creation | cargo test --lib search::fulltext::test_index_creation | Index created successfully |
| P04-006 | Full-Text Search | Exact query execution | cargo test --lib search::fulltext::test_exact_search | Returns matching documents |
| P04-007 | Full-Text Search | Boolean query (AND/OR/NOT) | cargo test --lib search::fulltext::test_boolean_queries | Boolean operators work correctly |
| P04-008 | Full-Text Search | Phrase search with quotes | cargo test --lib search::fulltext::test_phrase_search | Phrase search returns exact matches |
| P04-009 | Full-Text Search | Wildcard search patterns | cargo test --lib search::fulltext::test_wildcard_search | Wildcard patterns match correctly |
| P04-010 | Full-Text Search | Fuzzy search with distance | cargo test --lib search::fulltext::test_fuzzy_search | Fuzzy search returns near matches |
| P04-011 | Semantic Search | Vector embedding generation | cargo test --lib search::semantic::test_embedding_generation | Embeddings generated with 768 dimensions |
| P04-012 | Semantic Search | Cosine similarity calculation | cargo test --lib search::semantic::test_cosine_similarity | Similarity scores calculated correctly |
| P04-013 | Semantic Search | Embedding index persistence | cargo test --lib search::semantic::test_index_persistence | Index saved to `./workspace/memory/index/semantic/` |
| P04-014 | Semantic Search | Similarity threshold (0.7) | cargo test --lib search::semantic::test_similarity_threshold | Filters results below threshold |
| P04-015 | Hybrid Search Engine | Fusion algorithm | cargo test --lib search::hybrid::test_fusion_algorithm | Results combined correctly |
| P04-016 | Hybrid Search Engine | Query parsing | cargo test --lib search::query::test_query_parsing | Natural language parsed to structured query |
| P04-017 | Hybrid Search Engine | Re-ranking logic | cargo test --lib search::ranking::test_reranking | Results re-ranked by relevance |
| P04-018 | Hybrid Search Engine | Local memory first (ADR-0006) | cargo test --lib search::hybrid::test_local_first | Local memory exhausted before external |
| P04-019 | External Search Adapters | Policy gate enforcement | cargo test --lib external::policy::test_policy_gate | External search blocked without approval |
| P04-020 | External Search Adapters | DuckDuckGo API | cargo test --lib external::duckduckgo::test_search_api | DuckDuckGo search returns results |
| P04-021 | External Search Adapters | Brave API | cargo test --lib external::brave::test_search_api | Brave search returns results |
| P04-022 | External Search Adapters | Rate limiting (100/hour) | cargo test --lib external::rate_limit::test_rate_limit | Rate limit enforced correctly |
| P04-023 | External Search Adapters | Allowed/forbidden domains | cargo test --lib external::policy::test_domain_filters | Domain filters work correctly |
| P04-024 | External Search Adapters | Result caching with TTL | cargo test --lib external::policy::test_caching | Cached results returned when fresh |
| P04-025 | Web Scraping | Robots.txt parser | cargo test --lib scraping::robots::test_parser | Robots.txt parsed correctly |
| P04-026 | Web Scraping | Scope restrictions (ADR-0006) | cargo test --lib scraping::scope::test_restrictions | Disallow paths respected |
| P04-027 | Web Scraping | Content extraction | cargo test --lib scraping::extractor::test_content_extraction | Main content extracted correctly |
| P04-028 | Web Scraping | Max pages per domain (100) | cargo test --lib scraping::scope::test_max_pages | Enforces max_pages_per_domain |
| P04-029 | Web Scraping | HTML parsing | cargo test --lib scraping::extractor::test_html_parsing | HTML structure parsed correctly |
| P04-030 | Web Scraping | Crawl-delay enforcement | cargo test --lib scraping::scope::test_crawl_delay | Crawl-delay from robots.txt enforced |
| P04-031 | Provenance Tracking | Immutable timestamps | cargo test --lib provenance::models::test_timestamps | Timestamps cannot be modified |
| P04-032 | Provenance Tracking | Trace ID generation | cargo test --lib provenance::models::test_trace_ids | Unique trace IDs generated |
| P04-033 | Provenance Tracking | Content hashes (SHA-256) | cargo test --lib provenance::models::test_content_hashes | Hashes computed correctly |
| P04-034 | Provenance Tracking | Provenance store | cargo test --lib provenance::store::test_store | Provenance stored to `./workspace/provenance/` |
| P04-035 | Provenance Tracking | Timeline visualization | cargo test --lib provenance::timeline::test_timeline | Timeline generated from events |
| P04-036 | Provenance Tracking | Query by trace ID | cargo test --lib provenance::query::test_query_by_trace | Returns events for trace ID |
| P04-037 | Provenance Tracking | Event types (memory/search/web) | cargo test --lib provenance::models::test_event_types | All event types captured |
| P04-038 | Memory Garbage Collection | Age policy | cargo test --lib garbage::policy::test_age_policy | Deletes documents older than X days |
| P04-039 | Memory Garbage Collection | Size policy | cargo test --lib garbage::policy::test_size_policy | Deletes least-recently-used at threshold |
| P04-040 | Memory Garbage Collection | Refcount policy | cargo test --lib garbage::policy::test_refcount_policy | Deletes unreferenced documents |
| P04-041 | Memory Garbage Collection | Scheduler | cargo test --lib garbage::scheduler::test_scheduler | GC runs at configured interval |
| P04-042 | Memory Garbage Collection | Preview mode | cargo test --lib garbage::preview::test_preview | Preview shows what will be deleted |
| P04-043 | Memory Garbage Collection | Recovery | cargo test --lib garbage::recovery::test_recovery | Deleted data restored from backup |
| P04-044 | Memory Garbage Collection | GC logging | cargo test --lib garbage::policy::test_logging | GC operations logged with before/after |
| P04-045 | Memory & Search Integration | Workflow engine integration | cargo test --test workflow_integration::test_memory_search_workflow | Memory search works in workflows |
| P04-046 | Memory & Search Integration | Tool nodes | cargo test --test workflow_integration::test_memory_tools | Memory tool nodes execute correctly |
| P04-047 | Memory & Search Integration | CLI commands (store/search/delete/list) | cargo test --test cli_memory_search::test_cli_commands | CLI commands work correctly |
| P04-048 | Memory & Search Integration | UI memory browser | cargo test --test ui_memory_browser::test_browser | UI displays memory correctly |
| P04-049 | Memory & Search Integration | Variable interpolation | cargo test --test workflow_integration::test_variable_interpolation | Search results in workflow variables |
| P04-050 | Memory & Search Integration | RAG injection | cargo test --test workflow_integration::test_rag_injection | Retrieved context in LLM prompts |
| P04-051 | Memory & Search Integration | Error propagation | cargo test --test workflow_integration::test_error_propagation | Memory errors propagate to step results |
| P04-052 | Integration | End-to-end memory workflow | cargo run --bin agentsdk --run examples/memory-workflow.yaml | Workflow executes successfully |
| P04-053 | Integration | Cross-component communication | cargo test --test memory_integration::test_component_communication | All modules communicate correctly |
| P04-054 | Integration | Memory constraints enforcement | cargo test --test memory_integration::test_memory_constraints | Memory limits enforced correctly |

---

## Detailed Test Cases

### P04-001: Local Memory Storage - CRUD Operations

**Description**: Verify structured memory supports Create, Read, Update, Delete operations
**QA Area**: Area 1 - Local Memory Storage
**Priority**: P0
**Test Type**: Unit

**Test Command**:
```bash
cargo test --lib memory::storage::test_crud_operations -- --test-threads=1
```

**Expected Result**:
- Create operation: Document created with generated ID and metadata
- Read operation: Document retrieved by ID, content matches
- Update operation: Document content updated, version incremented
- Delete operation: Document removed, subsequent read returns NotFound error
- All operations: Atomic and consistent
- Error handling: Invalid ID returns NotFound, invalid data returns ValidationError

**Verification**:
```bash
# Check test output
cargo test --lib memory::storage::test_crud_operations -- --test-threads=1 2>&1 | grep -q "test result: ok"
```

---

### P04-005: Full-Text Search - Tantivy Index Creation

**Description**: Verify Tantivy indices are created correctly for full-text search
**QA Area**: Area 2 - Full-Text Search
**Priority**: P0
**Test Type**: Unit

**Test Command**:
```bash
cargo test --lib search::fulltext::test_index_creation -- --test-threads=1
```

**Expected Result**:
- Index created in `./workspace/memory/index/fulltext/`
- All document fields indexed (title, content, metadata, tags)
- Index is queryable immediately after creation
- Index persists across application restarts
- Mock index builder operations work correctly

**Verification**:
```bash
# Check test output
cargo test --lib search::fulltext::test_index_creation -- --test-threads=1 2>&1 | grep -q "test result: ok"
```

---

### P04-011: Semantic Search - Vector Embedding Generation

**Description**: Verify vector embeddings are generated with correct dimensions
**QA Area**: Area 3 - Semantic Search
**Priority**: P0
**Test Type**: Unit

**Test Command**:
```bash
cargo test --lib search::semantic::test_embedding_generation -- --test-threads=1
```

**Expected Result**:
- Embedding vector has 768 dimensions (per schema line 691)
- Batch size of 32 processed correctly (per schema line 692)
- Uses `${models.embedding-model}` from schema (lines 154-157)
- Mock embeddings used in tests (no actual model inference)
- Pre-computed embeddings for test data are deterministic

**Verification**:
```bash
# Check test output
cargo test --lib search::semantic::test_embedding_generation -- --test-threads=1 2>&1 | grep -q "test result: ok"
```

---

### P04-018: Hybrid Search Engine - Local Memory First (ADR-0006)

**Description**: Verify external search is ONLY invoked after local memory exhausted
**QA Area**: Area 4 - Hybrid Search Engine
**Priority**: P0
**Test Type**: Unit

**Test Command**:
```bash
cargo test --lib search::hybrid::test_local_first -- --test-threads=1
```

**Expected Result**:
- Query searches local memory (fulltext + semantic) first
- External search adapter NOT called if local results found
- External search ONLY called when local memory returns empty
- Logs show "local_memory_exhausted" before external search
- ADR-0006 constraint enforced: Local memory first is not bypassed

**Verification**:
```bash
# Check test output
cargo test --lib search::hybrid::test_local_first -- --test-threads=1 2>&1 | grep -q "test result: ok"
```

---

### P04-019: External Search Adapters - Policy Gate Enforcement

**Description**: Verify policy gates enforce user consent before external search
**QA Area**: Area 5 - External Search Adapters
**Priority**: P1
**Test Type**: Unit

**Test Command**:
```bash
cargo test --lib external::policy::test_policy_gate -- --test-threads=1
```

**Expected Result**:
- External search blocked when user_approval == false
- Policy gate prompts user for approval
- After user approval, external search executes
- Policy gate records approval in provenance trace
- ADR-0006 constraint enforced: Policy gates cannot be bypassed

**Verification**:
```bash
# Check test output
cargo test --lib external::policy::test_policy_gate -- --test-threads=1 2>&1 | grep -q "test result: ok"
```

---

### P04-025: Web Scraping - Robots.txt Parser

**Description**: Verify robots.txt parser correctly interprets directives
**QA Area**: Area 6 - Web Scraping
**Priority**: P1
**Test Type**: Unit

**Test Command**:
```bash
cargo test --lib scraping::robots::test_parser -- --test-threads=1
```

**Expected Result**:
- Robots.txt parsed correctly
- User-agent directives respected
- Disallow paths identified correctly
- Allow paths identified correctly
- Crawl-delay directive parsed (if present)
- Sitemap directive parsed (if present)
- Invalid robots.txt handled gracefully
- ADR-0006 constraint enforced: robots.txt respected

**Verification**:
```bash
# Check test output
cargo test --lib scraping::robots::test_parser -- --test-threads=1 2>&1 | grep -q "test result: ok"
```

---

### P04-031: Provenance Tracking - Immutable Timestamps

**Description**: Verify timestamps are immutable and recorded for all operations
**QA Area**: Area 7 - Provenance Tracking
**Priority**: P1
**Test Type**: Unit

**Test Command**:
```bash
cargo test --lib provenance::models::test_timestamps -- --test-threads=1
```

**Expected Result**:
- Timestamps recorded as ISO-8601 strings
- Timestamps cannot be modified after recording
- Timestamps are immutable (stored with write-once semantics)
- All operations (create/read/update/delete) record timestamps
- Timestamp precision includes timezone information
- ADR-0006 constraint enforced: immutable timestamps

**Verification**:
```bash
# Check test output
cargo test --lib provenance::models::test_timestamps -- --test-threads=1 2>&1 | grep -q "test result: ok"
```

---

### P04-045: Memory & Search Integration - Workflow Engine Integration

**Description**: Verify memory search capabilities are exposed to workflow engine
**QA Area**: Area 9 - Memory & Search Integration
**Priority**: P0
**Test Type**: Integration

**Test Command**:
```bash
cargo test --test workflow_integration::test_memory_search_workflow -- --test-threads=1 --nocapture
```

**Expected Result**:
- Memory search tool executes in workflow step
- Search results available as step output
- Variable interpolation works (`{{step.search_result.output}}`)
- RAG context injected into LLM prompts
- Memory operations don't block workflow execution
- Errors propagate to step results
- Integration test completes successfully

**Verification**:
```bash
# Check test output
cargo test --test workflow_integration::test_memory_search_workflow -- --test-threads=1 --nocapture 2>&1 | grep -q "test result: ok"
```

---

### P04-052: Integration - End-to-End Memory Workflow

**Description**: Verify complete memory workflow executes end-to-end
**QA Area**: Area 9 - Memory & Search Integration
**Priority**: P0
**Test Type**: E2E

**Test Command**:
```bash
cargo run --bin agentsdk --run docs/plans/04-memory-search/validation/example-memory-workflow.yaml
```

**Expected Result**:
- Workflow loads successfully from YAML
- Memory is stored in local memory
- Search query executes over local memory
- Results are returned and used in subsequent steps
- Provenance trace records all operations
- No errors or panics
- Output file created with search results
- Exit code is 0

**Verification**:
```bash
# Check execution succeeded
if cargo run --bin agentsdk --run docs/plans/04-memory-search/validation/example-memory-workflow.yaml; then
  echo "E2E test PASSED"
else
  echo "E2E test FAILED"
  exit 1
fi

# Check output file exists
ls -lh docs/plans/04-memory-search/validation/output/
```

---

## Performance Test Cases

### P04-PERF-001: Local Full-Text Search Latency

**Description**: Verify full-text search latency meets < 100ms target
**Priority**: P0
**Test Type**: Benchmark

**Test Command**:
```bash
cargo test --bench fulltext_search_latency -- --test-threads=1
```

**Expected Result**:
- p50 latency < 100ms
- p95 latency < 150ms
- p99 latency < 200ms
- No significant regression from baseline

**Verification**:
```bash
# Parse benchmark results
cargo test --bench fulltext_search_latency -- --test-threads=1 2>&1 | grep -q "p50.*< 100ms"
```

---

### P04-PERF-002: Local Semantic Search Latency

**Description**: Verify semantic search latency meets < 100ms target
**Priority**: P0
**Test Type**: Benchmark

**Test Command**:
```bash
cargo test --bench semantic_search_latency -- --test-threads=1
```

**Expected Result**:
- p50 latency < 100ms
- p95 latency < 150ms
- p99 latency < 200ms
- No significant regression from baseline

**Verification**:
```bash
# Parse benchmark results
cargo test --bench semantic_search_latency -- --test-threads=1 2>&1 | grep -q "p50.*< 100ms"
```

---

### P04-PERF-003: Hybrid Search Latency

**Description**: Verify hybrid search latency meets < 100ms target
**Priority**: P0
**Test Type**: Benchmark

**Test Command**:
```bash
cargo test --bench hybrid_search_latency -- --test-threads=1
```

**Expected Result**:
- p50 latency < 100ms
- p95 latency < 150ms
- p99 latency < 200ms
- No significant regression from baseline

**Verification**:
```bash
# Parse benchmark results
cargo test --bench hybrid_search_latency -- --test-threads=1 2>&1 | grep -q "p50.*< 100ms"
```

---

### P04-PERF-004: External Search Latency

**Description**: Verify external search latency meets < 2s target
**Priority**: P1
**Test Type**: Benchmark

**Test Command**:
```bash
cargo test --bench external_search_latency -- --test-threads=1
```

**Expected Result**:
- p50 latency < 2000ms
- p95 latency < 3000ms
- p99 latency < 5000ms
- No significant regression from baseline

**Verification**:
```bash
# Parse benchmark results
cargo test --bench external_search_latency -- --test-threads=1 2>&1 | grep -q "p50.*< 2000ms"
```

---

### P04-PERF-005: Memory Storage Throughput

**Description**: Verify memory storage throughput meets > 1000 docs/sec target
**Priority**: P0
**Test Type**: Benchmark

**Test Command**:
```bash
cargo test --bench memory_storage_throughput -- --test-threads=1
```

**Expected Result**:
- Throughput > 1000 docs/sec
- No significant regression from baseline
- Storage operations are atomic

**Verification**:
```bash
# Parse benchmark results
cargo test --bench memory_storage_throughput -- --test-threads=1 2>&1 | grep -q "throughput.*> 1000"
```

---

### P04-PERF-006: GC Cleanup Time

**Description**: Verify garbage collection cleanup time meets < 30s target
**Priority**: P2
**Test Type**: Benchmark

**Test Command**:
```bash
cargo test --bench gc_cleanup_time -- --test-threads=1
```

**Expected Result**:
- Cleanup time < 30s
- No significant regression from baseline
- All scheduled deletions completed

**Verification**:
```bash
# Parse benchmark results
cargo test --bench gc_cleanup_time -- --test-threads=1 2>&1 | grep -q "cleanup.*< 30s"
```

---

## Mock Strategy Notes

### Full-Text Search (Tantivy)
- Create in-memory tantivy indices for testing
- Mock index builder operations
- Mock query execution with deterministic results
- Avoid filesystem dependencies in tests

### Semantic Search (Embeddings)
- Mock embedding model with deterministic outputs
- Pre-computed embeddings for test data
- Mock vector similarity calculations
- Avoid actual model inference in tests

### External Search (HTTP)
- Mock HTTP responses for search APIs
- Test rate limiting with time control
- Test policy gate enforcement without network calls
- Mock API rate limit headers

### Web Scraping (Robots.txt)
- Mock HTTP server with robots.txt endpoints
- Test scope restrictions with mock responses
- Test extraction traces without actual web requests
- Mock crawl-delay enforcement

---

**End of QA Test Cases for Phase 04**
