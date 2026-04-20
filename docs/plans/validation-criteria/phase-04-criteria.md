# Phase 04: Memory & Search - Validation Criteria

**Phase Focus:** Local memory, search (full-text, semantic, hybrid), external search, scraping, provenance, GC
**Entry Criteria:** Phases 00, 01, 02, 03, and 04 complete
**Estimated Duration:** 3-4 weeks
**Blocking for:** Phases 05, 06, 07

---

## Phase Overview

Phase 04 implements memory and search capabilities. This phase provides local memory storage, full-text search, semantic search, hybrid search, external search with policy gates, web scraping with robots.txt respect, provenance tracking, and garbage collection.

**Critical Success Factors:**
1. Local memory stores and retrieves reliably
2. Full-text search works correctly
3. Semantic search works correctly
4. Hybrid search combines both
5. External search behind policy gates
6. Scraping respects robots.txt
7. Provenance captures timestamps
8. GC prevents unbounded growth

---

## Phase Entry Criteria

**Status:** Must be verified before starting Phase 04

**Verification Commands:**
```bash
# Verify Phases 00-04 exit
cargo test --test phase_00_integration -- --test-threads=1
cargo test --test phase_01_integration -- --test-threads=1
cargo test --test phase_02_integration -- --test-threads=1
cargo test --test phase_04_integration -- --test-threads=1
```

**Prerequisites:**
- [ ] Phases 00-04 exit criteria verified (all 7 layers)
- [ ] Schema coverage 100% for phases 00-04
- [ ] ADR compliance verified for phases 00-04
- [ ] Cross-phase regression clean (Phases 00-04)
- [ ] Vector database ready for semantic search
- [ ] Full-text index ready for search

**Blocking Violations:**
- Unresolved Phase 00-04 failures
- Schema coverage < 100% for any phase
- Cross-phase regression detected

---

## Local Memory

**Requirement:** Local memory stores and retrieves reliably

### Memory Operations

1. **Store:** Store data with key
2. **Retrieve:** Retrieve data by key
3. **Update:** Update existing data
4. **Delete:** Delete data by key
5. **List:** List all keys
6. **Exists:** Check if key exists

### Memory Structure

```
.memory/
├── data/
│   ├── workflow_1.json
│   ├── workflow_2.json
│   └── ...
├── index/
│   ├── full_text.idx
│   ├── semantic.idx
│   └── ...
├── metadata/
│   ├── provenance.json
│   └── timestamps.json
└── gc/
    └── deleted.json
```

### Verification Commands

```bash
# Test local memory operations
cargo test --lib memory::tests::store_operation
cargo test --lib memory::tests::retrieve_operation
cargo test --lib memory::tests::update_operation
cargo test --lib memory::tests::delete_operation
cargo test --lib memory::tests::list_operation
cargo test --lib memory::tests::exists_operation

# Test reliability
cargo test --lib memory::tests::concurrent_operations
cargo test --lib memory::tests::persistence_after_restart
```

### Pass Criteria

- [ ] All memory operations work correctly
- [ ] Data persists after restart
- [ ] Concurrent operations work safely
- [ ] No data corruption
- [ ] No memory leaks

### Evidence Required

- Memory operation test results
- Persistence test logs
- Concurrent operation test logs

---

## Full-Text Search

**Requirement:** Full-text search works correctly

### Search Features

1. **Keyword Search:** Search by keywords
2. **Phrase Search:** Search by exact phrases
3. **Boolean Search:** AND, OR, NOT operators
4. **Wildcard Search:** Wildcard patterns
5. **Proximity Search:** Words within distance
6. **Fuzzy Search:** Approximate matches

### Verification Commands

```bash
# Test full-text search
cargo test --lib search::tests::keyword_search
cargo test --lib search::tests::phrase_search
cargo test --lib search::tests::boolean_search
cargo test --lib search::tests::wildcard_search
cargo test --lib search::tests::proximity_search
cargo test --lib search::tests::fuzzy_search

# Test relevance ranking
cargo test --lib search::tests::relevance_ranking
```

### Pass Criteria

- [ ] All search types work correctly
- [ ] Results ranked by relevance
- [ ] Search performance acceptable
- [ ] No false negatives (missing results)
- [ ] Minimal false positives (irrelevant results)

### Evidence Required

- Full-text search test results
- Relevance ranking test logs
- Search performance metrics

---

## Semantic Search

**Requirement:** Semantic search works correctly

### Search Features

1. **Embedding Generation:** Generate embeddings for queries
2. **Vector Search:** Search by vector similarity
3. **Top-K Retrieval:** Return top K most similar results
4. **Similarity Threshold:** Filter by similarity threshold

### Verification Commands

```bash
# Test semantic search
cargo test --lib search::tests::embedding_generation
cargo test --lib search::tests::vector_search
cargo test --lib search::tests::top_k_retrieval
cargo test --lib search::tests::similarity_threshold

# Test accuracy
cargo test --lib search::tests::semantic_search_accuracy
```

### Pass Criteria

- [ ] Embeddings generated correctly
- [ ] Vector search works
- [ ] Top-K retrieval returns K results
- [ ] Similarity threshold filters correctly
- [ ] Search accuracy > 90%

### Evidence Required

- Semantic search test results
- Accuracy metrics
- Vector search logs

---

## Hybrid Search

**Requirement:** Hybrid search combines full-text and semantic search

### Search Strategy

1. **Execute full-text search**
2. **Execute semantic search**
3. **Combine results (merge, deduplicate)**
4. **Rank by combined score**
5. **Return top results

### Scoring

```
combined_score = 0.6 * full_text_score + 0.4 * semantic_score
```

### Verification Commands

```bash
# Test hybrid search
cargo test --lib search::tests::hybrid_search_merge
cargo test --lib search::tests::hybrid_search_deduplication
cargo test --lib search::tests::hybrid_search_ranking
cargo test --lib search::tests::hybrid_search_accuracy
```

### Pass Criteria

- [ ] Full-text and semantic search execute
- [ ] Results merged correctly
- [ ] Duplicates removed
- [ ] Combined ranking works
- [ ] Hybrid accuracy > individual search accuracy

### Evidence Required

- Hybrid search test results
- Merge and deduplication logs
- Combined ranking metrics

---

## External Search

**Requirement:** External search behind policy gates

### External Search Sources

1. **Google Search:** Web search
2. **Bing Search:** Web search
3. **Wikipedia:** Encyclopedia search
4. **GitHub:** Code search

### Policy Gates

1. **Allowed Domains:** Whitelist of allowed domains
2. **Blocked Domains:** Blacklist of blocked domains
3. **Rate Limits:** Maximum requests per minute
4. **Content Filters:** Block inappropriate content

### Verification Commands

```bash
# Test external search
cargo test --lib search::tests::google_search
cargo test --lib search::tests::bing_search
cargo test --lib search::tests::wikipedia_search
cargo test --lib search::tests::github_search

# Test policy gates
cargo test --lib search::tests::allowed_domains
cargo test --lib search::tests::blocked_domains
cargo test --lib search::tests::rate_limits
cargo test --lib search::tests::content_filters
```

### Pass Criteria

- [ ] All external search sources work
- [ ] Allowed domains enforced
- [ ] Blocked domains enforced
- [ ] Rate limits enforced
- [ ] Content filters work

### Evidence Required

- External search test results
- Policy gate test logs
- Rate limit logs

---

## Web Scraping

**Requirement:** Scraping respects robots.txt

### Scraping Features

1. **Robots.txt Parser:** Parse robots.txt
2. **Disallow Rules:** Respect disallow rules
3. **Crawl Delay:** Respect crawl-delay
4. **User-Agent:** Identify user-agent
5. **Rate Limiting:** Respect server rate limits

### Verification Commands

```bash
# Test web scraping
cargo test --lib scraping::tests::robots_txt_parser
cargo test --lib scraping::tests::disallow_rules
cargo test --lib scraping::tests::crawl_delay
cargo test --lib scraping::tests::user_agent
cargo test --lib scraping::tests::rate_limiting
```

### Pass Criteria

- [ ] Robots.txt parsed correctly
- [ ] Disallow rules respected
- [ ] Crawl delay respected
- [ ] User-agent identified
- [ ] Rate limits respected

### Evidence Required

- Web scraping test results
- Robots.txt compliance logs

---

## Provenance

**Requirement:** Provenance captures timestamps

### Provenance Data

1. **Creation Timestamp:** When data was created
2. **Modification Timestamp:** When data was last modified
3. **Access Timestamp:** When data was last accessed
4. **Source:** Where data came from
5. **Author:** Who created/modified data

### Provenance Format

```json
{
  "data_id": "uuid",
  "created_at": "2026-04-06T10:00:00Z",
  "modified_at": "2026-04-06T11:00:00Z",
  "accessed_at": "2026-04-06T12:00:00Z",
  "source": "external_search",
  "author": "system"
}
```

### Verification Commands

```bash
# Test provenance
cargo test --lib provenance::tests::creation_timestamp
cargo test --lib provenance::tests::modification_timestamp
cargo test --lib provenance::tests::access_timestamp
cargo test --lib provenance::tests::source_tracking
cargo test --lib provenance::tests::author_tracking
```

### Pass Criteria

- [ ] All timestamps captured correctly
- [ ] Source tracked accurately
- [ ] Author tracked accurately
- [ ] Timestamps update correctly
- [ ] No missing provenance data

### Evidence Required

- Provenance test results
- Timestamp verification logs

---

## Garbage Collection

**Requirement:** GC prevents unbounded growth

### GC Features

1. **Deleted Data Cleanup:** Remove deleted data
2. **Expired Data Cleanup:** Remove expired data
3. **Orphan Cleanup:** Remove orphaned data
4. **Index Cleanup:** Rebuild indexes
5. **Compaction:** Compact storage

### GC Triggers

1. **Manual Trigger:** Trigger GC manually
2. **Automatic Trigger:** Trigger GC on threshold
3. **Scheduled Trigger:** Trigger GC periodically

### Verification Commands

```bash
# Test garbage collection
cargo test --lib gc::tests::deleted_data_cleanup
cargo test --lib gc::tests::expired_data_cleanup
cargo test --lib gc::tests::orphan_cleanup
cargo test --lib gc::tests::index_cleanup
cargo test --lib gc::tests::compaction

# Test GC triggers
cargo test --lib gc::tests::manual_trigger
cargo test --lib gc::tests::automatic_trigger
cargo test --lib gc::tests::scheduled_trigger

# Test unbounded growth prevention
cargo test --lib gc::tests::unbounded_growth_prevention
```

### Pass Criteria

- [ ] All cleanup types work
- [ ] All triggers work
- [ ] Unbounded growth prevented
- [ ] No data loss during GC
- [ ] GC completes in reasonable time

### Evidence Required

- GC test results
- Growth prevention logs
- GC performance metrics

---

## Phase Exit Criteria

### Layer 1: Unit Tests

**Commands:**
```bash
cargo test --lib -- --test-threads=1
```

**Evidence:**
- All Phase 04 unit tests pass
- Test coverage >= 90%
- No clippy warnings

### Layer 2: Integration Tests

**Commands:**
```bash
cargo test --test '*' -- --test-threads=1
```

**Evidence:**
- All Phase 04 integration tests pass
- Memory and search integrate correctly

### Layer 3: Property Tests

**Commands:**
```bash
PROPTEST_NUMBER_OF_TESTS=100 cargo test --lib property_based
```

**Evidence:**
- Search invariants hold for 100 iterations
- Memory persistence invariants hold

### Layer 4: E2E Tests

**Commands:**
```bash
# Execute search workflows
cargo run --bin agentsdk -- run examples/workflows/full_text_search.yaml
cargo run --bin agentsdk -- run examples/workflows/semantic_search.yaml
cargo run --bin agentsdk -- run examples/workflows/hybrid_search.yaml
cargo run --bin agentsdk -- run examples/workflows/scraping.yaml
```

**Evidence:**
- All search workflows execute
- Scraping respects robots.txt
- GC prevents unbounded growth

### Layer 5: System Log Validation

**Commands:**
```bash
# Verify automation scope logs
cargo run --bin agentsdk -- run examples/workflows/search.yaml 2>&1 | \
  jq -e 'select(.scope == "automation")'
```

**Evidence:**
- Automation scope logs emitted
- Logs include timestamp, scope, event, message

### Layer 6: Live CLI Verification

**Commands:**
```bash
# Test memory commands
cargo run --bin agentsdk -- memory store --key test --value "hello"
cargo run --bin agentsdk -- memory retrieve --key test
cargo run --bin agentsdk -- memory list

# Test search commands
cargo run --bin agentsdk -- search full-text --query "test"
cargo run --bin agentsdk -- search semantic --query "test"
cargo run --bin agentsdk -- search hybrid --query "test"

# Test GC command
cargo run --bin agentsdk -- gc run
```

**Evidence:**
- Memory commands work
- Search commands work
- GC command works

### Layer 7: Benchmark Performance

**Commands:**
```bash
cargo bench --bench phase_05_benchmarks
```

**Evidence:**
- Search performance acceptable
- Memory operations performant
- GC completes in reasonable time
- No performance regression > 10%

---

## Cross-Phase Regression Tests

**Commands:**
```bash
# Run all prior phase tests
cargo test --test phase_00_integration -- --test-threads=1
cargo test --test phase_01_integration -- --test-threads=1
cargo test --test phase_02_integration -- --test-threads=1
cargo test --test phase_04_integration -- --test-threads=1
```

**Pass Criteria:**
- [ ] All Phase 00 tests still pass
- [ ] All Phase 01 tests still pass
- [ ] All Phase 02 tests still pass
- [ ] All Phase 04 tests still pass

---

## Schema Coverage Audit

**Requirement:** 100% of Phase 04-owned fields implemented

### Phase 04-Owned Fields

**MemorySchema:**
- memory_id
- key
- value
- created_at
- modified_at
- accessed_at

**SearchSchema:**
- search_type
- query
- similarity_threshold
- top_k

**ScrapingSchema:**
- url
- respect_robots_txt
- user_agent
- rate_limit

**ProvenanceSchema:**
- data_id
- created_at
- modified_at
- source
- author

### Verification Commands

```bash
cargo run --bin schema_audit -- --phase 5 --output coverage_report.md
```

**Expected Output:**
- 100% coverage for Phase 04-owned fields
- No unimplemented fields

---

## ADR Compliance

**Relevant ADRs:**
- ADR-009: Memory Architecture
- ADR-010: Search Architecture

### Verification Commands

```bash
cargo run --bin adr_compliance -- --phase 5
```

**Expected Output:**
- All Phase 04-relevant ADR constraints satisfied
- No outstanding ADR TODOs

---

## Anti-Goal-Drift Checklist

**Run after phase completion:**

1. **Requirements Drift:**
   - [ ] Implementation matches Phase 04 requirements
   - [ ] No features from later phases added

2. **Architecture Drift:**
   - [ ] Memory matches ADR-009
   - [ ] Search matches ADR-010

3. **Scope Creep:**
   - [ ] Only memory and search features implemented
   - [ ] No automation or autonomy features added

---

## Evidence Storage

**Location:** `results/phase_05/`

**Contents:**
- `unit_test_results.json`
- `integration_test_output.log`
- `property_test_results.json`
- `e2e_execution_logs/` (search workflows)
- `system_log_samples.json` (automation scope)
- `cli_verification/` (memory and search commands)
- `benchmarks/` (search and memory performance)
- `search_samples/` (search results)
- `scraping_logs/` (robots.txt compliance)
- `provenance_data/` (provenance samples)
- `gc_logs/` (garbage collection logs)
- `schema_coverage_report.md`
- `adr_compliance_report.md`
- `anti_goal_drift_checklist.md`
- `phase_00_regression/`
- `phase_01_regression/`
- `phase_02_regression/`
- `phase_04_regression/`

---

## Blocking Issues

**Cannot exit Phase 04 if:**
- Any verification layer fails
- Memory operations don't work reliably
- Search doesn't work correctly
- External search not behind policy gates
- Scraping doesn't respect robots.txt
- Provenance not captured
- GC doesn't prevent unbounded growth
- Any prior phase regression detected

---

**End of Phase 04 Criteria**
