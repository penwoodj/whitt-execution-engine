# Cross-References — Phase 04: Memory & Search

**Phase**: 04 - Memory & Search
**Schema Version**: 2.0.0
**Last Updated**: 2026-04-26

---

## Schema References

| QA Area | Schema Section | Line Range | Key Fields |
|----------|---------------|--------------|-------------|
| Local Memory Storage | workspace directories | 701-710 | rag_knowledge_base, backups |
| Semantic Search | memory/rag | 682-696 | embedding_model, retrieval (similarity_threshold, max_results, include_sources) |
| External Search Adapters | web_operations.fetch | 623-649 | allowed_domains, forbidden_domains, rate_limits (max_per_hour) |
| Web Scraping | web_operations.scrape | 632-634 | respect_robots_txt, max_pages_per_domain |
| Memory & Search Integration | agentic_workflow | 196-497 | steps, variables, tool integration |

---

## Plan References

| QA Area | Plan File | Line Range | Key Tasks |
|----------|-----------|--------------|-------------|
| Local Memory Storage | [tasks/00-local-memory-storage.md](../plans/04-memory-search/tasks/00-local-memory-storage.md) | N/A | CRUD operations, versioning |
| Full-Text Search | [tasks/01-fulltext-search.md](../plans/04-memory-search/tasks/01-fulltext-search.md) | N/A | Tantivy indexing, query execution |
| Semantic Search | [tasks/02-semantic-search.md](../plans/04-memory-search/tasks/02-semantic-search.md) | N/A | Vector embeddings, similarity search |
| Hybrid Search Engine | [tasks/03-search-query-engine.md](../plans/04-memory-search/tasks/03-search-query-engine.md) | N/A | Fusion algorithms, ranking |
| External Search Adapters | [tasks/04-external-search-adapters.md](../plans/04-memory-search/tasks/04-external-search-adapters.md) | N/A | Policy gates, DuckDuckGo, Brave |
| Web Scraping | [tasks/05-web-scraping.md](../plans/04-memory-search/tasks/05-web-scraping.md) | N/A | Robots.txt parser, content extraction |
| Provenance Tracking | [tasks/06-provenance-tracking.md](../plans/04-memory-search/tasks/06-provenance-tracking.md) | N/A | Immutable timestamps, trace IDs |
| Memory Garbage Collection | [tasks/07-memory-garbage-collection.md](../plans/04-memory-search/tasks/07-memory-garbage-collection.md) | N/A | GC policies, preview, recovery |
| Memory & Search Integration | [tasks/08-memory-search-integration.md](../plans/04-memory-search/tasks/08-memory-search-integration.md) | N/A | Workflow engine, tool nodes, CLI |

---

## Related QA Areas

| Related Phase | Related QA Area | Relationship |
|---------------|------------------|-------------|
| Phase 03 (Quality Loops) | Validation | Memory search results validated in quality loops |
| Phase 05 (Automation) | Experiment Tracking | Memory operations tracked in experiments |
| Phase 06 (Autonomy & Metrics) | Metrics | Memory search latency instrumented in metrics |
| Phase 07 (Final Validation) | Integration Tests | Memory search included in E2E tests |

---

## ADR References

| ADR | Title | Relevance |
|------|-------|-----------|
| ADR-0006 | Memory & Search Architecture | Core ADR governing this phase |
| ADR-0007 | Automation Governance | External search policy gates relate to automation |
| ADR-0008 | Autonomous Execution Safety | Memory search used in autonomous workflows |

---

## External Documentation

| Document | Path | Purpose |
|----------|--------|---------|
| Unified Workflow Schema | docs/schema/unified-workflow-schema.yml | Schema reference (805 lines) |
| Validation Framework | docs/plans/validation-criteria/framework.md | 7-layer validation system |
| Extended POC QA | docs/qa/extended-poc/QA-AREAS-EXTENDED-POC.md | QA format reference |
| AGENTS.md | AGENTS.md | QA area format, verification protocol |

---

## Task File References

| Task ID | Task File | Description |
|----------|-----------|-------------|
| 00 | [tasks/00-local-memory-storage.md](../plans/04-memory-search/tasks/00-local-memory-storage.md) | Local memory storage foundation |
| 01 | [tasks/01-fulltext-search.md](../plans/04-memory-search/tasks/01-fulltext-search.md) | Full-text search with Tantivy |
| 02 | [tasks/02-semantic-search.md](../plans/04-memory-search/tasks/02-semantic-search.md) | Semantic search with vector embeddings |
| 03 | [tasks/03-search-query-engine.md](../plans/04-memory-search/tasks/03-search-query-engine.md) | Hybrid search query engine |
| 04 | [tasks/04-external-search-adapters.md](../plans/04-memory-search/tasks/04-external-search-adapters.md) | External search adapters (DuckDuckGo, Brave) |
| 05 | [tasks/05-web-scraping.md](../plans/04-memory-search/tasks/05-web-scraping.md) | Web scraping with robots.txt compliance |
| 06 | [tasks/06-provenance-tracking.md](../plans/04-memory-search/tasks/06-provenance-tracking.md) | Provenance tracking with immutable traces |
| 07 | [tasks/07-memory-garbage-collection.md](../plans/04-memory-search/tasks/07-memory-garbage-collection.md) | Memory garbage collection |
| 08 | [tasks/08-memory-search-integration.md](../plans/04-memory-search/tasks/08-memory-search-integration.md) | Memory & search integration to workflow engine |

---

## Test File References

| Test File | Purpose |
|-----------|---------|
| tests/00-local-memory-storage.md | Mock storage strategies |
| tests/01-fulltext-search.md | Mock tantivy index |
| tests/02-semantic-search.md | Mock embedding models |
| tests/03-search-query-engine.md | Mock search results |
| tests/04-external-search-adapters.md | Mock HTTP clients |
| tests/05-web-scraping.md | Mock robots.txt server |
| tests/06-provenance-tracking.md | Mock provenance store |
| tests/07-memory-garbage-collection.md | Mock GC scheduler |
| tests/08-memory-search-integration.md | Integration test mocks |

---

## Validation File References

| Validation File | Purpose |
|----------------|---------|
| validation/00-local-memory-storage.md | Validation criteria for task 00 |
| validation/01-fulltext-search.md | Validation criteria for task 01 |
| validation/02-semantic-search.md | Validation criteria for task 02 |
| validation/03-search-query-engine.md | Validation criteria for task 03 |
| validation/04-external-search-adapters.md | Validation criteria for task 04 |
| validation/05-web-scraping.md | Validation criteria for task 05 |
| validation/06-provenance-tracking.md | Validation criteria for task 06 |
| validation/07-memory-garbage-collection.md | Validation criteria for task 07 |
| validation/08-memory-search-integration.md | Validation criteria for task 08 |

---

## Implementation Checklist

- [ ] Task 00: Local Memory Storage implemented and passing tests
- [ ] Task 01: Full-Text Search implemented and passing tests
- [ ] Task 02: Semantic Search implemented and passing tests
- [ ] Task 03: Hybrid Search Engine implemented and passing tests
- [ ] Task 04: External Search Adapters implemented and passing tests
- [ ] Task 05: Web Scraping implemented and passing tests
- [ ] Task 06: Provenance Tracking implemented and passing tests
- [ ] Task 07: Memory Garbage Collection implemented and passing tests
- [ ] Task 08: Memory & Search Integration implemented and passing tests
- [ ] All unit tests passing (cargo test --lib)
- [ ] All integration tests passing (cargo test --test)
- [ ] ADR-0006 compliance verified
- [ ] Performance benchmarks meet targets
- [ ] Documentation complete
- [ ] LSP diagnostics clean on all changed files
- [ ] Build passes (cargo build --release --all-features)

---

**End of Cross-References for Phase 04**
