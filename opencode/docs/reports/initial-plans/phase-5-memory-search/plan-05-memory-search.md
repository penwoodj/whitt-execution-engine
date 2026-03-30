# Plan-05: Local Memory Retrieval, Web Search, and Web Scraping

**Plan ID**: plan-05
**Phase**: Phase 5 - Memory & Search
**Status**: Not Started
**Created**: 2026-03-27
**Related ADR**: ADR-0006 (Local memory retrieval web search and web scraping)
**Related Research Plan**: research-plan-04-quality-memory-search.yml
**Estimated Time**: 10-12 weeks
**Dependencies**: plan-00 (Foundation), plan-01 (MVP Queue), plan-02 (CLI & Backends), plan-03 (Glyphnova UI), plan-04 (Quality Loops)

---

## Overview

This plan implements layered retrieval system with local memory, search indexing, and external web search/scraping behind explicit policy gates. Retrieval combines exact/full-text with semantic retrieval for balanced results.

**Key Features**:
- **Local Memory Architecture**: Structured and unstructured memory for workflows
- **Two-Plane Retrieval**: Local structured/unstructured memory + external research/scraping
- **Hybrid Search**: Exact/full-text + semantic retrieval (not vector-only)
- **Search Indexing**: Efficient artifact discovery across workflow collection
- **External Search Adapters**: Behind explicit policy gates with opt-in
- **Web Scraping Compliance**: Robots.txt enforcement, scope restrictions
- **Provenance Capture**: Timestamps, extraction traces, artifacts for all web/search operations
- **Memory Artifact Versioning**: Stored in `.glyphnova/memory/` with versioned references
- **Memory Garbage Collection**: Prevent unbounded growth

**Key Deliverables**:
- Local memory storage layer (structured + unstructured)
- Search indexing and retrieval engine
- External search adapters (web search, scraping)
- Policy enforcement for external access
- Scraping compliance rules engine
- Provenance tracking and logging
- Memory garbage collection system
- Memory/search CLI and UI integration

---

## Code Review

### ADR-0006 Summary

**Decision**: Introduce retrieval in layered form with transpiler integration.

**Key Requirements**:
1. **Local Memory First**: Local memory added before remote search and scraping
2. **Two-Plane Retrieval**: Local structured/unstructured memory + external research/scraping behind policy gates
3. **Hybrid Search**: Exact/full-text + semantic retrieval (not vector-only)
4. **Web Provenance**: Records timestamps, extraction traces, artifacts
5. **Scraping Compliance**: Robots and scope restrictions enforced
6. **Memory Storage**: Stored in `.glyphnova/memory/` with versioned references

**Scope**:
- **Included**: Local memory architecture for workflows, search indexing and retrieval, external search adapters with policy gates, scraping compliance rules, provenance capture, memory artifact versioning and garbage collection
- **Excluded**: Fully autonomous web crawling, vector-only retrieval without exact search, pre-indexed remote corpora, distributed memory systems

**Positive Consequences**:
- Workflows can access local context for better decisions
- Search improves workflow discoverability across artifacts
- Scraping is controlled and auditable
- Retrieval balance enables both exact and semantic matching

**Negative Consequences**:
- More storage subsystems to manage
- Policy surface area increases complexity
- Search may impact performance for large artifact collections

### Requirements Review

From `requirements.mdc` and `schema-consolidated-report.md`:

**R18**: Tools and custom Rust tools as first-class nodes
**R30**: Observability with logging levels, summaries, graphs, reports, and provenance
**R31**: Auditable local artifacts, versioned runs, hashes, and privacy-preserving defaults
**R04**: Depth-over-speed and capability-normalized outcomes

**Validation Focus for v0.1.0**:
- Local memory stores and retrieves workflow artifacts reliably
- Search indexing supports efficient artifact discovery
- External search is behind explicit policy gates and opt-in
- Scraping respects robots.txt and scope restrictions
- Provenance records capture timestamps, extraction traces, and artifacts
- Memory garbage collection prevents unbounded growth

**Validation Focus for v0.1.x and Later**:
- Search performance scales to large artifact collections
- Retrievable artifacts enable workflow re-execution and debugging
- Policy enforcement prevents unauthorized web or search access
- Scraping guardrails protect against robots violations and over-scraping
- Provenance metadata enables audit trails and compliance verification

### Transpiler Integration Points

- **WorkflowIR**: Should reference memory artifacts for context injection
- **Tool Nodes**: Should declare memory read/write effects
- **External Search Tools**: Require policy compilation and approval
- **Scraping Operations**: Require policy gates and provenance logging
- **Search Results**: Should integrate with `.glyphnova/` artifact references

### Research Plan Review

**research-plan-04-quality-memory-search.yml** is **COMPLETED** with 3 research domains (plan-05 uses domain 3):

3. **Memory and Search Architecture**: What memory architecture and retrieval strategy best support local-first agentic workflows?

**Key Findings**:
- Report outputs: `quality-memory-search-research-report.md`, `memory-architecture-options.csv`
- Quality gates met: Recommendations support local-first retrieval with provenance tracking

---

## Web Research

### Research Area 1: Local Memory Storage Patterns

**Research Question**: What storage patterns best support structured and unstructured local memory for workflows?

**Recommended Sources**:
- [SQLite](https://www.sqlite.org) - Embedded relational database
- [RocksDB](https://rocksdb.org) - Embedded key-value store
- [Redb](https://docs.rs/redb) - Rust embedded database
- [Sled](https://docs.rs/sled) - Rust embedded database
- [Redis](https://redis.io) - In-memory data structure store (for reference)

**Expected Findings**:
- Embedded vs external storage tradeoffs
- Structured vs unstructured storage patterns
- Indexing strategies for fast retrieval
- Versioning approaches for memory artifacts
- Garbage collection patterns

**Status**: Not Started

---

### Research Area 2: Hybrid Search Strategies

**Research Question**: What search strategies combine exact/full-text and semantic retrieval effectively?

**Recommended Sources**:
- [Tantivy](https://tantivy-search.github.io) - Rust full-text search engine
- [Meilisearch](https://www.meilisearch.com) - Search engine with typo tolerance
- [Elasticsearch](https://www.elastic.co) - Search and analytics engine
- [Chroma](https://www.trychroma.com) - Vector database
- [Weaviate](https://weaviate.io) - Vector search engine
- [HuggingFace MTEB](https://huggingface.co/spaces/mteb/leaderboard) - Embedding benchmarks

**Expected Findings**:
- Full-text search indexing patterns
- Semantic embedding generation
- Hybrid retrieval algorithms (RRF, reciprocal rank fusion)
- Score normalization and ranking
- Search result re-ranking strategies

**Status**: Not Started

---

### Research Area 3: Web Scraping Compliance

**Research Question**: What compliance patterns exist for web scraping with robots.txt and scope restrictions?

**Recommended Sources**:
- [robots.txt RFC](https://datatracker.ietf.org/doc/html/rfc9309) - Robots exclusion protocol
- [scrapy.org](https://scrapy.org) - Python scraping framework with compliance
- [robotexclusionrulesparser](https://www.npmjs.com/package/robotexclusionrulesparser) - Robots.txt parser
- [Beautiful Soup](https://www.crummy.com/software/BeautifulSoup/bs4/doc/) - HTML parsing
- [GitHub - scraping best practices](https://github.com) - Search for "ethical scraping" projects

**Expected Findings**:
- Robots.txt parsing and enforcement
- Scope restriction patterns (domain, path, rate limiting)
- Politeness and rate limiting strategies
- User agent best practices
- HTML parsing and extraction patterns

**Status**: Not Started

---

### Research Area 4: External Search APIs

**Research Question**: What external search APIs are available with policy-friendly terms?

**Recommended Sources**:
- [DuckDuckGo API](https://duckduckgo.com/api) - Privacy-friendly search
- [Brave Search API](https://brave.com/search/api) - Private search API
- [Google Custom Search API](https://developers.google.com/custom-search/v1/overview) - Google search (with policy gates)
- [Bing Search API](https://www.microsoft.com/en-us/bing/apis/bing-web-search-api) - Bing search (with policy gates)
- [Serper.dev](https://serper.dev) - Google Search API wrapper

**Expected Findings**:
- API authentication and rate limiting
- Query parameter options
- Response formats and parsing
- Privacy and policy considerations
- Cost and quota management

**Status**: Not Started

---

### Research Area 5: Provenance and Audit Logging

**Research Question**: What provenance patterns enable complete audit trails for web and search operations?

**Recommended Sources**:
- [W3C PROV](https://www.w3.org/TR/prov-overview/) - Provenance data model
- [Data provenance patterns](https://dl.acm.org) - Academic research
- [Git provenance](https://git-scm.com) - Version control provenance
- [Logfmt](https://brandur.org/logfmt) - Structured logging
- [OpenTelemetry](https://opentelemetry.io) - Observability and tracing

**Expected Findings**:
- Provenance data structures
- Trace linking and tracking
- Timestamp and hash handling
- Artifact provenance relationships
- Audit log query and visualization

**Status**: Not Started

---

## Implementation Plan

### Phase 1: Local Memory Storage

**Description**: Implement structured and unstructured local memory storage

**Tasks**:
1. Define memory storage schema (structured + unstructured)
2. Implement structured memory (key-value, relations)
3. Implement unstructured memory (documents, text blobs)
4. Add memory indexing (full-text, semantic)
5. Implement memory CRUD operations (create, read, update, delete)
6. Add memory versioning (with references)
7. Implement memory search query interface
8. Add memory storage in `.glyphnova/memory/`
9. Implement memory migration and backup

**Related Requirements**: ADR-0006 (local memory architecture)
**Verification Layers**: 1, 2, 3
**Status**: Not Started

---

### Phase 2: Search Indexing and Retrieval

**Description**: Implement hybrid search engine (exact + semantic)

**Tasks**:
1. Integrate Tantivy for full-text indexing
2. Implement semantic embedding generation (local models)
3. Define hybrid search algorithm (exact + semantic fusion)
4. Implement search query parsing
5. Add result ranking and re-ranking
6. Implement search filters (date, type, scope)
7. Add search result highlighting
8. Implement search performance optimization (caching, pagination)
9. Add search CLI commands and UI integration

**Related Requirements**: ADR-0006 (search indexing and retrieval)
**Verification Layers**: 1, 2, 3, 4
**Status**: Not Started

---

### Phase 3: External Search Adapters

**Description**: Implement external search and scraping adapters behind policy gates

**Tasks**:
1. Define external search policy gates (allowlist, require approval, blocklist)
2. Implement DuckDuckGo search adapter
3. Implement Brave search adapter
4. Implement web scraping adapter
5. Add robots.txt parser and enforcement
6. Implement scope restrictions (domain, path, rate limit)
7. Add extraction trace logging
8. Implement search result parsing and normalization
9. Add provenance capture (timestamps, URLs, artifacts)

**Related Requirements**: ADR-0006 (external search adapters with policy gates)
**Verification Layers**: 1, 2, 3, 4
**Status**: Not Started

---

### Phase 4: Policy Enforcement

**Description**: Implement policy enforcement for external search and scraping

**Tasks**:
1. Define search policy types (allow, require-approval, block)
2. Implement policy evaluation engine
3. Add policy compilation (from workflow policies)
4. Implement approval workflow (UI confirmation)
5. Add policy audit logging
6. Implement policy violation handling
7. Add policy versioning
8. Implement policy override with justification

**Related Requirements**: ADR-0006 (policy gates)
**Verification Layers**: 1, 2, 3
**Status**: Not Started

---

### Phase 5: Provenance Tracking

**Description**: Capture and store provenance for all web and search operations

**Tasks**:
1. Define provenance data structure (timestamp, trace, artifacts, hashes)
2. Implement provenance logging for all operations
3. Add extraction trace capture (DOM, selectors, content)
4. Implement artifact storage with provenance references
5. Add provenance query interface
6. Implement provenance visualization (timeline, graph)
7. Add provenance export (JSON, CSV)
8. Implement provenance verification (hash integrity)

**Related Requirements**: R30 (observability), R31 (auditable artifacts)
**Verification Layers**: 1, 2, 4
**Status**: Not Started

---

### Phase 6: Memory Garbage Collection

**Description**: Implement garbage collection to prevent unbounded memory growth

**Tasks**:
1. Define garbage collection policies (age, size, reference count)
2. Implement unused artifact detection
3. Add garbage collection scheduling (manual, automatic)
4. Implement garbage collection execution
5. Add GC logging and reporting
6. Implement GC preview (what will be deleted)
7. Add GC recovery (undo delete)
8. Implement memory size tracking and alerts

**Related Requirements**: ADR-0006 (memory garbage collection)
**Verification Layers**: 1, 2, 3
**Status**: Not Started

---

## Verification Strategy

### Layer 1: Unit Tests

**Scope**: Individual component testing

**Coverage Areas**:
- Memory storage CRUD operations
- Search indexing and query execution
- External search API calls
- Robots.txt parsing and enforcement
- Policy evaluation and gating
- Provenance data structure validity
- Garbage collection detection and execution

**Test Framework**: `cargo test --lib`

**Success Criteria**:
- 90%+ code coverage on core modules
- All storage operations tested (create, read, update, delete)
- All search modes tested (exact, semantic, hybrid)
- All policy types tested (allow, block, require-approval)

---

### Layer 2: Integration Tests

**Scope**: Multi-component interaction testing

**Coverage Areas**:
- Memory storage with search indexing
- Search retrieval from memory
- External search with policy gates
- Web scraping with robots.txt enforcement
- Provenance tracking end-to-end
- Garbage collection with memory references
- CLI and UI integration with all memory features

**Test Framework**: `cargo test --test '*'`

**Success Criteria**:
- Memory and search work together seamlessly
- Policy gates enforce correctly
- External search blocks without opt-in
- Scraping respects robots.txt
- Provenance is captured for all operations
- Garbage collection doesn't delete referenced artifacts

---

### Layer 3: Property-Based Tests

**Scope**: Edge cases and invariants

**Coverage Areas**:
- Memory storage consistency (CRUD invariants)
- Search result ordering (stable, deterministic)
- Policy enforcement completeness (no bypasses)
- Provenance link integrity (no broken references)
- Garbage collection correctness (never deletes referenced)
- Robots.txt parsing (always valid, handles edge cases)

**Test Framework**: `proptest` with 1000 iterations each

**Success Criteria**:
- No crashes on random inputs
- Invariants always hold (memory consistent, search deterministic, policy enforced)
- Properties verified with 1000+ iterations

---

### Layer 4: End-to-End Tests

**Scope**: Full memory, search, and scraping workflows

**Coverage Areas**:
- Store workflow artifacts in memory
- Search memory across all artifact types
- Run external search with policy approval
- Scrape web pages with robots.txt compliance
- Track provenance for all operations
- Run garbage collection
- Search and retrieve artifacts after GC
- Use memory in workflow context injection

**Test Framework**: `cargo test --test '*e2e*'`

**Success Criteria**:
- Complete memory workflows execute successfully
- Search finds correct artifacts
- External search is blocked without opt-in
- Scraping respects robots.txt and scope
- Provenance is captured and queryable
- Garbage collection prevents unbounded growth
- Workflows can use memory for context

---

## Verification Checkpoints

### Checkpoint 1: Local Memory Storage Working

**Target Date**: Week 2
**Verification Layers**: 1, 2, 3
**Sign-Off Criteria**:
- [ ] Structured memory stores and retrieves correctly
- [ ] Unstructured memory stores and retrieves correctly
- [ ] Memory indexing works
- [ ] Memory CRUD operations complete
- [ ] Memory versioning tracks changes
- [ ] Property tests verify memory consistency
- [ ] Integration tests pass with real artifacts

**Status**: Not Started

---

### Checkpoint 2: Hybrid Search Functional

**Target Date**: Week 4
**Verification Layers**: 1, 2, 3, 4
**Sign-Off Criteria**:
- [ ] Full-text search works
- [ ] Semantic search works
- [ ] Hybrid search combines results correctly
- [ ] Search ranking and re-ranking works
- [ ] Search filters and highlighting work
- [ ] Search CLI commands work
- [ ] Property tests verify search determinism
- [ ] E2E test: search and retrieve artifacts

**Status**: Not Started

---

### Checkpoint 3: External Search Adapters Working

**Target Date**: Week 6
**Verification Layers**: 1, 2, 3, 4
**Sign-Off Criteria**:
- [ ] DuckDuckGo search works
- [ ] Brave search works
- [ ] Web scraping works
- [ ] Robots.txt parser is correct
- [ ] Scope restrictions are enforced
- [ ] Extraction traces are captured
- [ ] Provenance is recorded
- [ ] E2E test: external search with opt-in

**Status**: Not Started

---

### Checkpoint 4: Policy Enforcement Complete

**Target Date**: Week 8
**Verification Layers**: 1, 2, 3
**Sign-Off Criteria**:
- [ ] Policy evaluation works for all types
- [ ] Policy gates enforce correctly (allow, block, require-approval)
- [ ] Approval workflow triggers correctly
- [ ] Policy violations are logged
- [ ] Policy overrides require justification
- [ ] Property tests verify policy completeness
- [ ] Integration tests pass with policy enforcement

**Status**: Not Started

---

### Checkpoint 5: Provenance Tracking Complete

**Target Date**: Week 9
**Verification Layers**: 1, 2, 4
**Sign-Off Criteria**:
- [ ] Provenance is captured for all operations
- [ ] Provenance data is structured correctly
- [ ] Extraction traces are complete
- [ ] Provenance query interface works
- [ ] Provenance visualization displays correctly
- [ ] E2E test: complete provenance tracking workflow

**Status**: Not Started

---

### Checkpoint 6: Memory Garbage Collection Working

**Target Date**: Week 10
**Verification Layers**: 1, 2, 3
**Sign-Off Criteria**:
- [ ] Garbage collection policies work (age, size, references)
- [ ] Unused artifacts are detected correctly
- [ ] GC execution deletes correctly
- [ ] GC logging is complete
- [ ] GC preview works
- [ ] GC recovery (undo) works
- [ ] Property tests verify GC correctness (no referenced deletions)
- [ ] Integration tests pass with GC

**Status**: Not Started

---

### Checkpoint 7: Memory/Search Integration Complete

**Target Date**: Week 12
**Verification Layers**: 1, 2, 4
**Sign-Off Criteria**:
- [ ] Workflows can inject memory context
- [ ] Tool nodes can read/write memory
- [ ] CLI memory commands work
- [ ] UI memory browser works
- [ ] Search integrates with artifact references
- [ ] External search tools require policy approval
- [ ] Scraping operations require policy gates
- [ ] E2E test: workflow using memory and search

**Status**: Not Started

---

## Progress Tracking

### Overall Progress

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Phases Complete | 7 | 0 | 0% |
| Verification Layers | 4 | 0 | 0% |
| Checkpoints Passed | 7 | 0 | 0% |
| Web Research Areas | 5 | 0 | 0% |

**Overall Status**: Not Started

---

### Phase Progress

| Phase | Description | Status | Completion |
|-------|-------------|--------|------------|
| 1 | Local Memory Storage | Not Started | 0% |
| 2 | Search Indexing and Retrieval | Not Started | 0% |
| 3 | External Search Adapters | Not Started | 0% |
| 4 | Policy Enforcement | Not Started | 0% |
| 5 | Provenance Tracking | Not Started | 0% |
| 6 | Memory Garbage Collection | Not Started | 0% |
| 7 | Memory/Search Integration | Not Started | 0% |

---

### Verification Layer Progress

| Layer | Description | Tests Written | Tests Passing | Coverage |
|-------|-------------|----------------|---------------|----------|
| 1 | Unit Tests | 0 | 0 | 0% |
| 2 | Integration Tests | 0 | 0 | 0% |
| 3 | Property-Based Tests | 0 | 0 | 0% |
| 4 | End-to-End Tests | 0 | 0 | 0% |

---

### Research Progress

| Research Area | Status | Evidence Collected | Synthesized |
|--------------|--------|-------------------|-------------|
| Local Memory Storage Patterns | Not Started | 0 | No |
| Hybrid Search Strategies | Not Started | 0 | No |
| Web Scraping Compliance | Not Started | 0 | No |
| External Search APIs | Not Started | 0 | No |
| Provenance and Audit Logging | Not Started | 0 | No |

---

## Dependencies

### Blocks

- plan-00 (Foundation): Needed for `.glyphnova/` structure and storage
- plan-01 (MVP Queue): Needed for workflow execution and artifact storage
- plan-02 (CLI & Backends): Needed for CLI memory commands
- plan-03 (Glyphnova UI): Needed for UI memory browser
- plan-04 (Quality Loops): Needed for quality metrics on search results

### Unblocks

- plan-06: Automation (depends on memory and search for automation)
- plan-07: Autonomy & Metrics (depends on all retrieval infrastructure)

### Integration Points

- **plan-00 (Foundation)**: Uses `.glyphnova/memory/` storage, provenance tracking
- **plan-01 (MVP Queue)**: Stores workflow artifacts in memory
- **plan-02 (CLI & Backends)**: CLI commands for memory and search
- **plan-03 (Glyphnova UI)**: Memory browser, search UI, provenance visualization
- **plan-04 (Quality Loops)**: Quality metrics for search results

---

## Quality Gates

### ADR-0006 Quality Gates

1. **Local Memory Reliability**: Local memory stores and retrieves workflow artifacts reliably
2. **Search Efficiency**: Search indexing supports efficient artifact discovery
3. **External Search Policy Gates**: External search is behind explicit policy gates and opt-in
4. **Scraping Compliance**: Scraping respects robots.txt and scope restrictions
5. **Provenance Completeness**: Provenance records capture timestamps, extraction traces, and artifacts
6. **Memory Garbage Collection**: Memory garbage collection prevents unbounded growth

### Critical Review Upstream Factors

1. **Memory Storage Performance**: Memory operations complete within 100ms for 10,000+ artifacts
2. **Search Accuracy**: Hybrid search finds relevant artifacts (precision > 0.8, recall > 0.7)
3. **Policy Enforcement Completeness**: No external search or scraping succeeds without policy approval
4. **Provenance Integrity**: Provenance links are never broken (all references valid)
5. **Garbage Collection Safety**: GC never deletes referenced artifacts (verified with property tests)
6. **Search Scalability**: Search performance scales linearly with artifact count (not exponential)
7. **Scraping Rate Limiting**: Scraping respects rate limits and doesn't violate robots.txt

---

## Next Steps

### Workflow for Execution

1. **Code Review**: Read ADR-0006 and related research plan findings
2. **Web Research**: Complete 5 research areas with evidence collection
3. **Update Plan**: Incorporate research findings into implementation plan
4. **Write Tests**: Start with unit tests for Phase 1 (Local Memory Storage)
5. **Implement Phase 1**: Build memory storage with test-driven development
6. **Verify Checkpoint 1**: Run Layer 1, 2, 3 tests, update plan with results
7. **Continue Phases**: Repeat test-first approach for phases 2-7
8. **All Checkpoints**: Verify each checkpoint with all applicable layers
9. **Update Plan**: Document all working code and verification results
10. **Proceed**: Mark plan complete and move to plan-06

### Starting Point

Begin with Phase 1 (Local Memory Storage):
1. Define memory storage schema
2. Implement structured memory
3. Implement unstructured memory
4. Write unit tests for storage operations
5. Write property tests for storage consistency
6. Implement memory versioning
7. Verify Checkpoint 1

---

## Execution Commands

### Verify All Tests

```bash
# Run all test layers
cargo test --lib              # Layer 1: Unit tests
cargo test --test '*'           # Layer 2: Integration tests
cargo test --test '*proptest*'  # Layer 3: Property-based tests
cargo test --test '*e2e*'       # Layer 4: End-to-end tests
```

### Verify Single Phase

```bash
# Verify Phase 1 (Local Memory Storage)
cargo test --lib memory
cargo test --test memory_integration
cargo test --test memory_proptest

# Verify Phase 2 (Search Indexing and Retrieval)
cargo test --lib search
cargo test --test search_integration
cargo test --test search_proptest
cargo test --test search_e2e
```

### Run Memory and Search Operations

```bash
# Store artifact in memory
yaml-to-rust-agentsdk memory store --type structured --key "workflow:123" --file workflow.yml

# Search memory
yaml-to-rust-agentsdk search --query "workflow for data processing" --mode hybrid

# Run external search (with policy approval)
yaml-to-rust-agentsdk search --external --query "Rust async patterns" --require-approval

# Scrape web page (with policy approval)
yaml-to-rust-agentsdk scrape --url https://example.com --require-approval

# Run garbage collection
yaml-to-rust-agentsdk memory gc --preview
yaml-to-rust-agentsdk memory gc --execute

# View provenance
yaml-to-rust-agentsdk provenance --operation search-123
```

### Use implement.sh Script

```bash
# Start from beginning
./implement.sh start plan-05

# Resume from checkpoint
./implement.sh resume plan-05 cp3

# Check progress
./implement.sh status plan-05

# Generate progress report
./implement.sh report plan-05
```

---

## References

### Related Documents

- **ADR-0006**: [Local memory retrieval web search and web scraping](../roadmap/adr-0006-memory-search-scraping.yml)
- **Research Plan 04**: [Quality loops and memory & search research](../roadmap/research-plan-04-quality-memory-search.yml)
- **Plan 00**: [Foundation Compiler Contract](phase-0-foundation/plan-00-foundation.md)
- **Plan 01**: [MVP Queue & Scheduler](phase-1-mvp-queue/plan-01-mvp-queue.md)
- **Plan 02**: [CLI & Backends](phase-2-cli-backends/plan-02-cli-backends.md)
- **Plan 03**: [Glyphnova UI](phase-3-glyphnova-ui/plan-03-glyphnova-ui.md)
- **Plan 04**: [Quality Loops](phase-4-quality-loops/plan-04-quality-loops.md)
- **Requirements**: [schema-consolidated-report.md](../requirements/schema-consolidated-report.md)

### Implementation References

- **Tantivy**: [tantivy-search.github.io](https://tantivy-search.github.io) - Full-text search engine
- **SQLite**: [sqlite.org](https://www.sqlite.org) - Embedded relational database
- **RocksDB**: [rocksdb.org](https://rocksdb.org) - Embedded key-value store
- **robots.txt RFC**: [RFC 9309](https://datatracker.ietf.org/doc/html/rfc9309) - Robots exclusion protocol
- **DuckDuckGo API**: [duckduckgo.com/api](https://duckduckgo.com/api) - Privacy-friendly search
- **OpenTelemetry**: [opentelemetry.io](https://opentelemetry.io) - Observability and tracing
- **W3C PROV**: [w3.org/TR/prov-overview](https://www.w3.org/TR/prov-overview/) - Provenance data model

### Tool and Plugin References

- **OpenCode Tools**: File operations, web browsing, shell execution, grep search
- **webfetch**: Web scraping and API calls
- **bash tool**: Command execution for memory operations
- **lsp_diagnostics**: Type checking and lint verification
- **glob tool**: File pattern matching for memory indexing

---

**Last Updated**: 2026-03-27
**Status**: Not Started
