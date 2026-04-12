# Phase 5: Memory & Search Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a robust memory and search system with local storage, hybrid search capabilities, external search integration, provenance tracking, and garbage collection for the AgentSDK Execution Engine.

**Architecture:** Dual-retrieval-plane architecture with local structured/unstructured memory (priority) and external search behind explicit policy gates. Hybrid search combining exact/fulltext matching with semantic similarity (vector embeddings). Provenance tracking for all memory and web operations with immutable timestamps and trace chains.

**Tech Stack:** Rust (async runtime), tantivy (full-text indexing), local embedding models (semantic), tokio (async), serde (serialization), thiserror (error handling), tracing (logging), reqwest (HTTP), robots-txt parser.

---

## Phase Overview

**Estimated Time:** 10-12 weeks
**Dependencies:** Phase 0 (Foundation), Phase 1 (Workflow Engine), Phase 2 (Tool System), Phase 3 (Context Management), Phase 4 (State Management) must be complete

### Key ADR-0006 Constraints

1. **LOCAL MEMORY FIRST:** External search is secondary to local memory - system must exhaust local memory before making external search calls
2. **POLICY GATES:** All external search requires explicit user approval via policy gate mechanisms
3. **HYBRID SEARCH:** Not vectors alone - combine exact/fulltext search with semantic similarity
4. **ROBOTS.TXT ENFORCEMENT:** Web scraping must respect robots.txt with strict scope restrictions
5. **PROVENANCE TRACKING:** All operations must record immutable timestamps, trace IDs, and content hashes

### Retrieval Planes

**Plane 1: Local Memory (Primary)**
- Structured memory: Key-value stores, document collections, metadata
- Unstructured memory: Free-text blobs, logs, conversation history
- Storage location: `./workspace/memory/` with versioned references
- Search: Hybrid (exact + semantic) over local content

**Plane 2: External Search (Secondary)**
- Policy-gated access to DuckDuckGo, Brave, and other search APIs
- Rate-limited, respect robots.txt
- Requires explicit user consent per search operation
- Cached results with TTL

---

## Implementation Task Order

Tasks are ordered by dependency and ADR-0006 priority:

1. **Task 00: Local Memory Storage** - Foundation for all memory operations
2. **Task 01: Full-Text Search** - Exact/fulltext search over local memory
3. **Task 02: Semantic Search** - Vector embeddings and similarity search
4. **Task 03: Search Query Engine** - Hybrid search orchestration layer
5. **Task 04: External Search Adapters** - Policy-gated external search (ONLY after local)
6. **Task 05: Web Scraping** - robots.txt compliant content extraction
7. **Task 06: Provenance Tracking** - Immutable trace records for all operations
8. **Task 07: Memory Garbage Collection** - Cleanup and recovery mechanisms
9. **Task 08: Memory & Search Integration** - Expose to workflow engine, tools, CLI, UI

---

## File Structure Overview

```
agent-sdk/
├── crates/
│   ├── memory/
│   │   ├── src/
│   │   │   ├── storage.rs       # Memory storage interface & implementation
│   │   │   ├── schema.rs        # Memory schemas (structured/unstructured)
│   │   │   ├── versioning.rs    # Versioned references
│   │   │   ├── operations.rs    # CRUD operations
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   ├── search/
│   │   ├── src/
│   │   │   ├── fulltext.rs      # Tantivy full-text search
│   │   │   ├── semantic.rs      # Vector embeddings & similarity
│   │   │   ├── hybrid.rs        # Hybrid fusion algorithms
│   │   │   ├── query.rs         # Query parsing & execution
│   │   │   ├── ranking.rs       # Scoring & re-ranking
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   ├── external/
│   │   ├── src/
│   │   │   ├── policy.rs        # Policy gate enforcement
│   │   │   ├── duckduckgo.rs    # DuckDuckGo adapter
│   │   │   ├── brave.rs         # Brave adapter
│   │   │   ├── rate_limit.rs    # Rate limiting
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   ├── scraping/
│   │   ├── src/
│   │   │   ├── robots.rs        # robots.txt parser
│   │   │   ├── scope.rs         # Scope restrictions
│   │   │   ├── extractor.rs     # Content extraction
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   ├── provenance/
│   │   ├── src/
│   │   │   ├── models.rs        # Provenance data structures
│   │   │   ├── store.rs         # Provenance storage
│   │   │   ├── query.rs         # Provenance query interface
│   │   │   ├── timeline.rs      # Timeline visualization
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   └── garbage/
│       ├── src/
│       │   ├── policy.rs        # GC policies (age/size/refcount)
│       │   ├── scheduler.rs     # GC scheduling
│       │   ├── preview.rs       # GC preview/dry-run
│       │   ├── recovery.rs      # Recovery & rollback
│       │   └── lib.rs
│       └── Cargo.toml
├── tools/
│   └── memory-search/           # Tool implementations
├── cli/
│   └── memory-search/           # CLI commands
└── ui/
    └── memory-browser/          # UI integration

glyphnova/memory/                 # Local storage directory
├── structured/
│   ├── collections/
│   └── documents/
├── unstructured/
│   ├── logs/
│   └── history/
└── index/                       # Search indices
    ├── fulltext/
    └── semantic/
```

---

## Task References

Each task is detailed in a separate file:

- **Task 00:** [tasks/00-local-memory-storage.md](tasks/00-local-memory-storage.md)
- **Task 01:** [tasks/01-fulltext-search.md](tasks/01-fulltext-search.md)
- **Task 02:** [tasks/02-semantic-search.md](tasks/02-semantic-search.md)
- **Task 03:** [tasks/03-search-query-engine.md](tasks/03-search-query-engine.md)
- **Task 04:** [tasks/04-external-search-adapters.md](tasks/04-external-search-adapters.md)
- **Task 05:** [tasks/05-web-scraping.md](tasks/05-web-scraping.md)
- **Task 06:** [tasks/06-provenance-tracking.md](tasks/06-provenance-tracking.md)
- **Task 07:** [tasks/07-memory-garbage-collection.md](tasks/07-memory-garbage-collection.md)
- **Task 08:** [tasks/08-memory-search-integration.md](tasks/08-memory-search-integration.md)

---

## Validation Criteria

Detailed validation criteria are defined in the [validation/](validation/) directory:

- [validation/00-local-memory-storage.md](validation/00-local-memory-storage.md)
- [validation/01-fulltext-search.md](validation/01-fulltext-search.md)
- [validation/02-semantic-search.md](validation/02-semantic-search.md)
- [validation/03-search-query-engine.md](validation/03-search-query-engine.md)
- [validation/04-external-search-adapters.md](validation/04-external-search-adapters.md)
- [validation/05-web-scraping.md](validation/05-web-scraping.md)
- [validation/06-provenance-tracking.md](validation/06-provenance-tracking.md)
- [validation/07-memory-garbage-collection.md](validation/07-memory-garbage-collection.md)
- [validation/08-memory-search-integration.md](validation/08-memory-search-integration.md)

---

## Test Specifications

Test specifications with mock strategies are defined in the [tests/](tests/) directory:

- [tests/00-local-memory-storage.md](tests/00-local-memory-storage.md) - Mock storage strategies
- [tests/01-fulltext-search.md](tests/01-fulltext-search.md) - Mock tantivy index
- [tests/02-semantic-search.md](tests/02-semantic-search.md) - Mock embedding models
- [tests/03-search-query-engine.md](tests/03-search-query-engine.md) - Mock search results
- [tests/04-external-search-adapters.md](tests/04-external-search-adapters.md) - Mock HTTP clients
- [tests/05-web-scraping.md](tests/05-web-scraping.md) - Mock robots.txt server
- [tests/06-provenance-tracking.md](tests/06-provenance-tracking.md) - Mock provenance store
- [tests/07-memory-garbage-collection.md](tests/07-memory-garbage-collection.md) - Mock GC scheduler
- [tests/08-memory-search-integration.md](tests/08-memory-search-integration.md) - Integration test mocks

---

## Mock Strategy Summary

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

## ADR-0006 Compliance Checklist

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

## Phase 5 Exit Criteria

Phase 5 is complete when:

1. **All 9 tasks** are implemented and passing their validation criteria
2. **All tests** pass with proper mock strategies
3. **ADR-0006 compliance** verified through full checklist
4. **Integration tests** demonstrate end-to-end memory & search workflow
5. **Documentation** covers all APIs, configuration, and usage patterns
6. **Performance benchmarks** meet targets (search latency < 100ms for local, < 2s for external)
7. **Memory constraints** are documented and enforced
8. **Provenance tracking** is complete and queryable

---

## Next Steps

After Phase 5 completion, proceed to:

- **Phase 6: Orchestration & Coordination**
- **Phase 7: Observability & Monitoring**
- **Phase 8: Performance Optimization**

---

**Document Version:** 1.0
**Last Updated:** 2025-04-06
**Status:** Ready for Implementation
