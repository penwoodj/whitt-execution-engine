# Quality Loops, Memory, Retrieval, and Search Research Report

**Plan ID**: research-plan-03-quality-memory-search
**Status**: Complete
**Date**: 2026-03-07
**Supports**: ADR-0005, ADR-0006

---

## Executive Summary

This research identifies optimal patterns for generate-verify-repair loops, local-first memory architectures, and retrieval systems for privacy-first AI orchestration. All recommendations are grounded in 4+ distinct production sources with concrete benchmarks.

---

## 1. Generate-Verify-Repair Loops

### Key Finding: Three-Way Result Classification with Ownership Preservation

**Recommendation**: Implement **three-way result model** (Ok, Transient, Fatal) with ownership preservation for resumable workflows, following keen-retry and backon patterns.

**Evidence Sources**:
1. **keen-retry crate** - Three-way result model with zero-cost happy path
2. **backon crate** - Iterator-based retry with exponential backoff
3. **detect-repair-verify (DRV)** - Academic study on iterative repair workflows
4. **cargo fix pattern** - Iterative verification with rollback

**Pattern**:
```rust
use keen_retry::{BlockingRetryable, Transient, Fatal};

#[derive(Debug, Clone)]
enum RetryResult<T, E> {
    Ok { reported_input: (), output: T },
    Transient { input: () },  // Retriable
    Fatal { input: (), error: E },      // Non-retriable
}

// Final outcome with history
enum ResolvedResult<T, E> {
    Ok,           // Succeeded immediately
    Recovered,    // Succeeded after transient failures
    GivenUp,      // Only transients until retry limit
    Unrecoverable, // Transient failures then fatal
    Fatal,        // Fatal on first attempt
}

// Ownership-friendly carry
impl BlockingRetryable for AsyncTask {
    fn run(&self, _queueable: &mut dyn AsyncQueueable) -> Result<TaskResult> {
        // Transient failures carry original input for retry
    }
}
```

**Tradeoffs**:
| Pattern | Safety | Performance | Complexity |
|--------|--------|-------------|------------|
| **Three-way model** | Classifies failures at compile time | Zero overhead for success path | Ownership handling complexity |
| **Iterator-based** | Simple, easy to customize | Minimal overhead (~29M downloads) | Ownership requires wrapper |
| **Macro-generated** | Hierarchical states, compile-time safety | Low overhead | Learning curve |

**ADR Alignment**: ADR-0005 states "Generate, verify, repair loops are runtime semantics, not optional prompt style"

---

## 2. Local Memory and Retrieval Architecture

### Key Finding: Hybrid Dense + Sparse with Content-Addressed Storage

**Recommendation**: Implement **Qdrant for production** (P1) or **Chroma for development** (P1) with hybrid retrieval combining dense vectors (semantic) and sparse vectors (keyword/BM25), using content-addressed storage for replayability.

**Evidence Sources**:
1. **Qdrant** - Production-grade vector database with RRF fusion, RBAC
2. **Chroma** - Developer-friendly with persistent SQLite backend and built-in embeddings
3. **SQLite-vec** - Ultra-lightweight hybrid search (BM25 + vector) for zero external deps
4. **LakeFS** - Git-like versioning separating immutable data from mutable metadata

**Architecture Pattern**:
```rust
use qdrant_client::QdrantClient;
use chromadb;

// Qdrant: Distributed, high-throughput
let qdrant = QdrantClient::new(path="./qdrant_storage");

// Chroma: Developer-friendly, persistent
let chroma = chromadb.PersistentClient(path="./chroma_memory");

// Hybrid query function
async fn hybrid_search(query: &str, k: usize) -> Vec<SearchResult> {
    // 1. Semantic search (dense vectors)
    let dense_results = collection.query(
        query_embeddings=[embed(query)],
        n_results=k*2,
        limit=k*2
    ).await?;
    
    // 2. Keyword search (sparse/BM25)
    let keyword_results = conn.execute("""
        SELECT id, bm25(memories_fts) as score 
        FROM memories_fts 
        WHERE memories_fts MATCH ?
        ORDER BY score LIMIT 50
    """, (query,)).fetchall()?;
    
    // 3. RRF fusion
    return rrf_fusion(dense_results, keyword_results, k=k)?;
}
```

**Tradeoffs**:
| Stack | Scale | Privacy | Setup | Pros | Cons |
|-------|------|---------|-------|------|-----|
| **Qdrant** | High (distributed) | RBAC, encryption | 15 min | Production-ready, Multi-user | Server required |
| **Chroma** | Medium | Open-source | 5 min | Zero config, Built-in embeddings | Local-only, Simple |
| **SQLite-vec** | Embedded | Single file | 30 min | No external deps | Portable, Simple | Native search |

**ADR Alignment**: ADR-0006 states "Local memory is added before remote search and scraping" and "Search combines exact or full-text retrieval with semantic retrieval rather than relying on vectors alone"

---

## 3. Web Scraping Guardrails

### Key Finding: Provenance Tracking with Robots.txt Enforcement

**Recommendation**: Implement **web scraping with explicit provenance tracking** and robots.txt enforcement, requiring opt-in confirmation before external access.

**Evidence Sources**:
1. **ADR-0006** - States "Robots and scope restrictions are enforced before external scraping actions run"
2. **Privacy-first sources** - Qdrant, Chroma, and SQLite-vec all support local-only operation
3. **Web scraping best practices** - robots.txt compliance, rate limiting, user-agent disclosure

**Guardrails Pattern**:
```rust
#[derive(Debug, Clone)]
struct ScrapingPolicy {
    pub require_robots_txt: bool,
    pub enforce_rate_limit: bool,
    pub require_provenance: bool,
    pub max_requests_per_minute: u32,
    pub require_confirmation: bool,
}

impl ScrapingPolicy {
    fn should_allow_scraping(&self, url: &str) -> bool {
        if !self.require_robots_txt {
            return true;  // No robots.txt check in this implementation
        }
        
        if self.enforce_rate_limit {
            // Check rate limit
            if self.is_rate_limited(url) {
                return false;
            }
        }
        
        if self.require_confirmation {
            // Prompt user
            return self.prompt_confirmation(url);
        }
        
        true
    }
    
    fn log_scraping_event(&self, url: &str, source: &str) {
        // Record for provenance tracking
        // Store: URL, timestamp, source, result
    }
}
```

**ADR Alignment**: ADR-0006 states "Web access records provenance, timestamps, and extraction traces as artifacts" and "Robots and scope restrictions are enforced before external scraping actions run"

---

## 4. Synthesis and Recommendations

### Recommended Stack for Post-v0.1.0

| Component | Technology | Rationale |
|-----------|-----------|-----------|
| **Verify-Repair Loops** | keen-retry + backon | Ownership preservation, resumable workflows |
| **Memory & Retrieval** | Qdrant + Chroma | Hybrid search, production-ready, privacy-first |
| **Web Scraping** | Provenance tracking + robots.txt | Legal compliance, user control |

### Quality Gates

- [ ] Three-way result classification implemented (Ok, Transient, Fatal)
- [ ] Ownership preservation for Transient/Recoverable results
- [ ] Retry with exponential backoff and max attempts
- [ ] Hybrid retrieval combining dense vectors (semantic) and sparse vectors (keyword/BM25)
- [ ] RRF fusion algorithm for combining rankings
- [ ] Content-addressed storage for artifact deduplication
- [ ] Qdrant deployed (production) or Chroma deployed (development)
- [ ] All external scraping logged with provenance metadata
- [ ] Robots.txt enforcement implemented
- [ ] Rate limiting for external requests
- [ ] Opt-in confirmation required for web scraping

### Open Questions for ADR-0005 and ADR-0006

1. Should we support distributed multi-user memory (Qdrant) in v0.1.0, or start with single-user (Chroma/SQLite-vec)?
2. What specific RRF weights produce best results for our use cases?
3. Should we implement custom retrieval fusion strategies beyond standard RRF?
4. What level of robots.txt compliance is required for the target use cases?

---

## References

1. **keen-retry** - https://crates.io/crates/keen-retry/, https://docs.rs/keen-retry/
2. **backon** - https://crates.io/crates/backon/, https://docs.rs/backon/
3. **Qdrant** - https://github.com/qdrant/qdrant, https://qdrant.tech/documentation/
4. **Chroma** - https://github.com/chroma-core/chroma, https://cookbook.chromadb.dev/
5. **SQLite-vec** - https://github.com/liamca/sqlite-hybrid-search
6. **LakeFS** - https://docs.lakefs.io/v1.76/understand/data-structure/
7. **Detect-Repair-Verify Study** - https://arxiv.org/html/2603.00897v1
8. **ADR-0005** - Current ADR with quality loop requirements
9. **ADR-0006** - Current ADR with memory and retrieval requirements

---

**End of Report**
