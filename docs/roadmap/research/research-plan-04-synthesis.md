## Synthesis and Implementation Plan Updates (Plan 04)

- Verifier/Repair Loops: implement layered verifier with deterministic repair steps, idempotent replay semantics, and artifact generation for traceability.
- Local Memory Architecture: bounded in-memory cache with eviction (LRU) and optional spill-to-disk; ensure zero-copy handoffs when possible.
- Retrieval/Search: two-plane model (local memory index + remote vector retrieval backed by Qdrant); policy gates for external lookups.
- Scraping/Provenance: enforce robots and provenance capture; artifactize scraped content with timestamps and source metadata; ensure privacy defaults.
- Benchmarking: integrate Criterion benchmarks on hot paths (indexing, retrieval latency, repair cycles); baseline vs improved metrics and regression detection.
- Observability: instrument with OpenTelemetry to collect traces/metrics for memory, latency, and repair rates; add dashboards.
- ADR-driven plan alignment: map each ADR (0005, 0006, 0007, 0008) to concrete plan items and acceptance criteria.
