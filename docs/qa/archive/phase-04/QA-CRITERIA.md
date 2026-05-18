# QA Criteria — Phase 04: Memory & Search

**Schema**: `docs/schema/unified-workflow-schema.yml` (805 lines)
**Phase Plan**: `docs/plans/04-memory-search/plan.md` (258 lines, 9 tasks)
**Date**: 2026-04-26
**Status**: 🟡 IN PROGRESS — Implementation not started

---

## Summary Table

| # | QA Area | Schema Ref | Priority | Test Type |
|---|---------|------------|------------|
| 1 | Local Memory Storage (CRUD operations) | Lines 701-710 (workspace directories) | P0 | Unit, Integration |
| 2 | Full-Text Search (Tantivy indexing) | N/A (new feature) | P0 | Unit, Integration |
| 3 | Semantic Search (vector embeddings) | Lines 682-696 (memory/rag/embedding_model) | P0 | Unit, Integration |
| 4 | Hybrid Search Engine (fusion algorithms) | Lines 682-696 (memory/rag/retrieval) | P0 | Unit, Integration |
| 5 | External Search Adapters (policy gates) | Lines 623-649 (web_operations) | P1 | Unit, Integration |
| 6 | Web Scraping (robots.txt compliance) | Lines 632-634 (web_operations.scrape) | P1 | Unit, Integration |
| 7 | Provenance Tracking (immutable traces) | Lines 711-723 (workspace/backups) + N/A | P1 | Unit, Integration |
| 8 | Memory Garbage Collection (cleanup policies) | N/A (new feature) | P2 | Unit, Integration |
| 9 | Memory & Search Integration (workflow engine) | Lines 196-497 (agentic_workflow) | P0 | Integration, E2E |

---

## Area Details

### Area 1: Local Memory Storage

**Schema Ref**: Lines 701-710 (workspace directories: rag_knowledge_base)
**Plan Ref**: Task 00 (`docs/plans/04-memory-search/tasks/00-local-memory-storage.md`)
**Files**: `src/memory/storage.rs`, `src/memory/schema.rs`, `src/memory/versioning.rs`, `src/memory/operations.rs`

**Criteria**:
- Structured memory: Key-value stores support CRUD operations (Create, Read, Update, Delete)
- Unstructured memory: Free-text blobs, logs, conversation history stored in `./workspace/memory/unstructured/`
- Storage location: `./workspace/memory/` with versioned references
- Document collections: Support collections with metadata (created_at, updated_at, content_hash)
- Versioning: Versioned references for all stored documents (v1, v2, v3...)
- Operations: Atomic writes, reads are consistent, deletes remove all versions
- Schema validation: Memory schemas validate against type constraints
- Error handling: Proper error types for storage failures (disk full, invalid format, not found)

**Test Type**: Unit, Integration
**Priority**: P0

**Commands**:
```bash
# Unit tests
cargo test --lib memory::storage -- --test-threads=1

# Integration tests
cargo test --test memory_integration_test -- --test-threads=1 --nocapture

# Clippy check
cargo clippy --all-features -- -D warnings src/memory/
```

---

### Area 2: Full-Text Search

**Schema Ref**: N/A (new feature - not in schema yet)
**Plan Ref**: Task 01 (`docs/plans/04-memory-search/tasks/01-fulltext-search.md`)
**Files**: `src/search/fulltext.rs`, `src/search/index.rs`

**Criteria**:
- Tantivy indexing: Create in-memory tantivy indices for testing
- Exact search: Full-text search over local structured/unstructured memory
- Index building: Mock index builder operations for testing
- Query execution: Mock query execution with deterministic results
- Index persistence: Indices saved to `./workspace/memory/index/fulltext/`
- Search performance: Target < 100ms for local searches (per plan)
- Field indexing: Index document fields (title, content, metadata, tags)
- Boolean queries: Support AND/OR/NOT operators in search queries
- Phrase search: Support quoted phrase searches
- Wildcard search: Support wildcard patterns (* for multiple characters, ? for single)
- Fuzziness: Support fuzzy search with configurable distance

**Test Type**: Unit, Integration
**Priority**: P0

**Commands**:
```bash
# Unit tests with mock tantivy indices
cargo test --lib search::fulltext -- --test-threads=1

# Integration tests
cargo test --test fulltext_search_integration -- --test-threads=1 --nocapture

# Benchmark search performance
cargo test --bench fulltext_search_bench -- --test-threads=1
```

---

### Area 3: Semantic Search

**Schema Ref**: Lines 682-696 (memory/rag/embedding_model, retrieval)
**Plan Ref**: Task 02 (`docs/plans/04-memory-search/tasks/02-semantic-search.md`)
**Files**: `src/search/semantic.rs`

**Criteria**:
- Vector embeddings: Generate embeddings using local embedding models
- Embedding model ref: `${models.embedding-model}` from schema
- Dimension: 768 dimensions per schema (line 691)
- Batch size: 32 per schema (line 692)
- Mock embeddings: Pre-computed embeddings for test data (avoid actual model inference)
- Similarity search: Cosine similarity calculations for vector matching
- Index persistence: Embedding index saved to `./workspace/memory/index/semantic/`
- Similarity threshold: 0.7 threshold from schema (line 695)
- Max results: 10 results max from schema (line 694)
- Include sources: Include source metadata in results from schema (line 696)
- Performance: Target < 100ms for semantic searches (per plan)

**Test Type**: Unit, Integration
**Priority**: P0

**Commands**:
```bash
# Unit tests with mock embedding models
cargo test --lib search::semantic -- --test-threads=1

# Integration tests
cargo test --test semantic_search_integration -- --test-threads=1 --nocapture

# Test with different embedding models
cargo test --lib semantic_search_model_test -- --test-threads=1
```

---

### Area 4: Hybrid Search Engine

**Schema Ref**: Lines 682-696 (memory/rag - retrieval parameters)
**Plan Ref**: Task 03 (`docs/plans/04-memory-search/tasks/03-search-query-engine.md`)
**Files**: `src/search/hybrid.rs`, `src/search/query.rs`, `src/search/ranking.rs`

**Criteria**:
- Fusion algorithm: Combine exact/fulltext search with semantic similarity results
- Local memory first: Exhaust local memory before external search (ADR-0006 constraint)
- Query parsing: Parse natural language queries into structured search queries
- Re-ranking: Re-rank combined results by relevance score
- Query orchestration: Route queries to appropriate search engine (fulltext, semantic, hybrid)
- Ranking: Scoring and re-ranking logic for result ordering
- Query validation: Validate query syntax and reject invalid queries
- Result merging: Merge duplicate results from different engines
- Fallback strategy: If semantic search fails, fall back to fulltext
- Performance: Target < 100ms for hybrid searches (per plan)

**Test Type**: Unit, Integration
**Priority**: P0

**Commands**:
```bash
# Unit tests for fusion algorithms
cargo test --lib search::hybrid -- --test-threads=1

# Unit tests for query parsing
cargo test --lib search::query -- --test-threads=1

# Unit tests for ranking
cargo test --lib search::ranking -- --test-threads=1

# Integration tests
cargo test --test hybrid_search_integration -- --test-threads=1 --nocapture
```

---

### Area 5: External Search Adapters

**Schema Ref**: Lines 623-649 (web_operations.fetch: allowed_domains, forbidden_domains, rate_limits)
**Plan Ref**: Task 04 (`docs/plans/04-memory-search/tasks/04-external-search-adapters.md`)
**Files**: `src/external/policy.rs`, `src/external/duckduckgo.rs`, `src/external/brave.rs`, `src/external/rate_limit.rs`

**Criteria**:
- Policy gates: Explicit user approval via policy gate mechanisms before external search (ADR-0006)
- DuckDuckGo adapter: Search API integration with DuckDuckGo
- Brave adapter: Search API integration with Brave
- Rate limiting: Enforce rate limits per schema (line 629: max_per_hour: 100)
- Allowed domains: Restrict to allowed_domains from schema (line 627)
- Forbidden domains: Block forbidden_domains from schema (line 628)
- Respect robots.txt: Web scraping respects robots.txt (ADR-0006 constraint)
- Cache results: Cached results with TTL to avoid duplicate requests
- HTTP client: Async HTTP client with timeout and retry logic
- Error handling: Handle API errors, rate limits, timeouts gracefully

**Test Type**: Unit, Integration
**Priority**: P1

**Commands**:
```bash
# Unit tests with mock HTTP clients
cargo test --lib external::adapters -- --test-threads=1

# Unit tests for policy gates
cargo test --lib external::policy -- --test-threads=1

# Integration tests with mock servers
cargo test --test external_search_integration -- --test-threads=1 --nocapture

# Test rate limiting
cargo test --test rate_limit_test -- --test-threads=1
```

---

### Area 6: Web Scraping

**Schema Ref**: Lines 632-634 (web_operations.scrape: respect_robots_txt, max_pages_per_domain)
**Plan Ref**: Task 05 (`docs/plans/04-memory-search/tasks/05-web-scraping.md`)
**Files**: `src/scraping/robots.rs`, `src/scraping/scope.rs`, `src/scraping/extractor.rs`

**Criteria**:
- Robots.txt parser: Parse robots.txt files correctly
- Scope restrictions: Respect robots.txt scope restrictions (ADR-0006 constraint)
- Max pages per domain: Enforce max_pages_per_domain from schema (line 634)
- Content extraction: Extract main content, navigation elements, metadata
- Parse HTML: Parse HTML structure to extract text, links, structured data
- Crawl-delay: Enforce crawl-delay from robots.txt (if specified)
- User-agent: Use appropriate user-agent string for requests
- Disallow paths: Respect Disallow directives from robots.txt
- Allow paths: Only crawl allowed paths from robots.txt
- Sitemap parsing: Parse sitemap.xml if available for discovery
- Error handling: Handle malformed HTML, network errors, timeouts

**Test Type**: Unit, Integration
**Priority**: P1

**Commands**:
```bash
# Unit tests for robots.txt parser
cargo test --lib scraping::robots -- --test-threads=1

# Unit tests for scope restrictions
cargo test --lib scraping::scope -- --test-threads=1

# Unit tests for content extraction
cargo test --lib scraping::extractor -- --test-threads=1

# Integration tests with mock HTTP servers
cargo test --test web_scraping_integration -- --test-threads=1 --nocapture
```

---

### Area 7: Provenance Tracking

**Schema Ref**: Lines 711-723 (workspace/backups) + N/A (new feature)
**Plan Ref**: Task 06 (`docs/plans/04-memory-search/tasks/06-provenance-tracking.md`)
**Files**: `src/provenance/models.rs`, `src/provenance/store.rs`, `src/provenance/query.rs`, `src/provenance/timeline.rs`

**Criteria**:
- Immutable timestamps: All operations record immutable timestamps (ADR-0006 constraint)
- Trace IDs: All operations record trace IDs for reconstruction (ADR-0006 constraint)
- Content hashes: All operations record content hashes for integrity (ADR-0006 constraint)
- Provenance store: Store provenance data in `./workspace/provenance/`
- Timeline visualization: Generate timeline of operations for audit
- Query interface: Query provenance by trace ID, time range, operation type
- Event types: Memory operations (create/read/update/delete), search operations, web operations
- Metadata capture: Capture who, what, when, why for each operation
- Trace chains: Link related operations via trace IDs
- Immutable event log: Event log in `./workspace/provenance/events/` cannot be modified

**Test Type**: Unit, Integration
**Priority**: P1

**Commands**:
```bash
# Unit tests for provenance models
cargo test --lib provenance::models -- --test-threads=1

# Unit tests for provenance store
cargo test --lib provenance::store -- --test-threads=1

# Unit tests for provenance query
cargo test --lib provenance::query -- --test-threads=1

# Integration tests
cargo test --test provenance_integration -- --test-threads=1 --nocapture
```

---

### Area 8: Memory Garbage Collection

**Schema Ref**: N/A (new feature - not in schema yet)
**Plan Ref**: Task 07 (`docs/plans/04-memory-search/tasks/07-memory-garbage-collection.md`)
**Files**: `src/garbage/policy.rs`, `src/garbage/scheduler.rs`, `src/garbage/preview.rs`, `src/garbage/recovery.rs`

**Criteria**:
- GC policies: Configurable policies (age/size/refcount based)
- Age policy: Delete documents older than X days
- Size policy: Delete least-recently-used documents when total size exceeds threshold
- Refcount policy: Delete documents not referenced by any provenance trace
- Scheduler: Scheduled GC runs (configurable interval)
- Preview mode: Dry-run mode shows what will be deleted without deleting
- Recovery: Restore deleted data from backup if rollback enabled
- Safe deletion: Only delete after verification (not referenced, not pinned)
- GC logging: Log all GC operations with before/after state
- Backup creation: Create backup before GC operations

**Test Type**: Unit, Integration
**Priority**: P2

**Commands**:
```bash
# Unit tests for GC policies
cargo test --lib garbage::policy -- --test-threads=1

# Unit tests for scheduler
cargo test --lib garbage::scheduler -- --test-threads=1

# Unit tests for preview
cargo test --lib garbage::preview -- --test-threads=1

# Integration tests
cargo test --test garbage_collection_integration -- --test-threads=1 --nocapture

# Test recovery
cargo test --test garbage_recovery_test -- --test-threads=1
```

---

### Area 9: Memory & Search Integration

**Schema Ref**: Lines 196-497 (agentic_workflow steps), Lines 701-710 (workspace directories)
**Plan Ref**: Task 08 (`docs/plans/04-memory-search/tasks/08-memory-search-integration.md`)
**Files**: `src/tools/memory-search.rs`, integration with workflow engine, CLI, UI

**Criteria**:
- Workflow engine: Expose memory search capabilities to workflow engine
- Tool nodes: Tool nodes expose memory search operations in workflows
- CLI commands: CLI commands for memory search (store, search, delete, list)
- UI integration: Memory browser UI component for viewing and searching memory
- Variable interpolation: Use memory search results in workflow variables (`{{step.search_result.output}}`)
- Memory tools: file_read, file_write, memory_store, memory_search, memory_delete tools
- RAG injection: Inject retrieved context into LLM prompts
- Search output: Structured output with sources, relevance scores, content
- Performance: Memory operations integrate without blocking workflow execution
- Error handling: Memory operation errors propagate to workflow step results

**Test Type**: Integration, E2E
**Priority**: P0

**Commands**:
```bash
# Integration tests for workflow engine
cargo test --test workflow_integration -- --test-threads=1 --nocapture

# Integration tests for tool nodes
cargo test --test memory_tools_integration -- --test-threads=1 --nocapture

# CLI tests
cargo test --test cli_memory_search -- --test-threads=1 --nocapture

# E2E tests with example workflows
for workflow in docs/plans/04-memory-search/validation/*.yaml; do
  cargo run --bin agentsdk --run "$workflow" || exit 1
done

# UI integration tests
cargo test --test ui_memory_browser -- --test-threads=1 --nocapture
```

---

## ADR-0006 Compliance Validation

- [ ] Local memory storage implemented and functional
- [ ] Full-text search over local memory works correctly
- [ ] Semantic search over local memory works correctly
- [ ] Hybrid search combines exact + semantic results properly
- [ ] External search ONLY invoked after local memory exhausted
- [ ] Policy gates enforce user consent before external search
- [ ] robots.txt parser correctly interprets directives
- [ ] Web scraping respects robots.txt scope restrictions
- [ ] All operations record provenance timestamps
- [ ] All operations record trace IDs for reconstruction
- [ ] All operations record content hashes for integrity
- [ ] Garbage collection policies are configurable
- [ ] GC preview shows what will be deleted
- [ ] GC recovery can restore deleted data (if rollback enabled)
- [ ] Memory search integration with workflow engine works
- [ ] Tool nodes expose memory search capabilities
- [ ] CLI commands for memory search work
- [ ] UI browser integration works

---

## Performance Targets

| Metric | Target | Validation Method |
|--------|--------|------------------|
| Local search latency (fulltext) | < 100ms | Benchmark test |
| Local search latency (semantic) | < 100ms | Benchmark test |
| Local search latency (hybrid) | < 100ms | Benchmark test |
| External search latency | < 2000ms | Benchmark test |
| Memory storage throughput | > 1000 docs/sec | Benchmark test |
| GC cleanup time | < 30s | Benchmark test |

---

## Phase Exit Criteria

Phase 04 is complete when:

1. **All 9 tasks** are implemented and passing their validation criteria
2. **All tests** pass with proper mock strategies
3. **ADR-0006 compliance** verified through full checklist
4. **Integration tests** demonstrate end-to-end memory & search workflow
5. **Documentation** covers all APIs, configuration, and usage patterns
6. **Performance benchmarks** meet targets (search latency < 100ms for local, < 2s for external)
7. **Memory constraints** are documented and enforced
8. **Provenance tracking** is complete and queryable

---

**End of QA Criteria for Phase 04**

## Related Tasks

This QA criteria document covers the following plan tasks:

- **Task 00**: [Local Memory Storage](../plans/04-memory-search/tasks/00-local-memory-storage.md) — Implements structured/unstructured memory with CRUD operations
- **Task 01**: [Full-Text Search](../plans/04-memory-search/tasks/01-fulltext-search.md) — Builds Tantivy-based search index over stored memory
- **Task 02**: [Semantic Search](../plans/04-memory-search/tasks/02-semantic-search.md) — Implements vector embeddings and similarity search
- **Task 03**: [Search Query Engine](../plans/04-memory-search/tasks/03-search-query-engine.md) — Orchestrates hybrid search with caching and re-ranking
- **Task 04**: [External Search Adapters](../plans/04-memory-search/tasks/04-external-search-adapters.md) — Provides policy-gated external search via DuckDuckGo/Brave
- **Task 05**: [Web Scraping](../plans/04-memory-search/tasks/05-web-scraping.md) — Scrapes web content with robots.txt compliance
- **Task 06**: [Provenance Tracking](../plans/04-memory-search/tasks/06-provenance-tracking.md) — Tracks immutable traces of memory sources
- **Task 07**: [Memory Garbage Collection](../plans/04-memory-search/tasks/07-memory-garbage-collection.md) — Implements cleanup policies for stale memory
- **Task 08**: [Memory & Search Integration](../plans/04-memory-search/tasks/08-memory-search-integration.md) — Integrates all components into workflow engine

