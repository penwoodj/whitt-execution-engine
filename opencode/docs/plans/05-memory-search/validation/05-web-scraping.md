# Validation Criteria: Task 05 - Web Scraping

## Overview

Validate that web scraping system provides robots.txt compliance, scope restrictions, content extraction, extraction traces, and ADR-0006 compliance.

---

## Checkpoint Gate Criteria

### Checkpoint 1: robots.txt Compliance Complete
- [ ] **robots.txt parser implemented**: Fetches and parses directives
- [ ] **Disallow directives respected**: Blocks disallowed paths
- [ ] **Crawl-delay directive respected**: Throttles requests
- [ ] **robots.txt cached with TTL**: Efficient re-fetch
- [ ] **Unit tests pass**: All robots.txt tests (`cargo test --package agentsdk-scraping`)

**Verification Commands:**
```bash
# Verify scraping crate builds
cargo check --package agentsdk-scraping

# Run robots.txt tests
cargo test --package agentsdk-scraping --lib robots

# Expected output: All robots.txt tests pass
```

### Checkpoint 2: Scope Restrictions Functional
- [ ] **Allowed domains enforced**: Only whitelisted domains accessed
- [ ] **Blocked domains enforced**: Blacklisted domains blocked
- [ ] **Path patterns enforced**: URL patterns matched
- [ ] **Depth limits enforced**: Max crawl depth respected
- [ ] **Page count limits enforced**: Max pages respected
- [ ] **Unit tests pass**: All scope tests

**Verification Commands:**
```bash
# Run scope tests
cargo test --package agentsdk-scraping --lib scope

# Verify scope enforcement
cargo test --package agentsdk-scraping test_scope_enforcement

# Expected output: All scope tests pass
```

### Checkpoint 3: Content Extraction Working
- [ ] **HTML content extracted**: Content parsed correctly
- [ ] **Page title extracted**: Title metadata captured
- [ ] **Content type detected**: MIME type identified
- [ ] **Redirects handled**: Follows redirects correctly
- [ ] **Unit tests pass**: All extraction tests

**Verification Commands:**
```bash
# Run extraction tests
cargo test --package agentsdk-scraping --lib extraction

# Verify extraction quality
cargo test --package agentsdk-scraping test_extraction_quality

# Expected output: All extraction tests pass
```

### Checkpoint 4: Performance Meets Targets
- [ ] **robots.txt fetch < 500ms**: Measured with benchmarks
- [ ] **robots.txt parse < 10ms**: Measured with benchmarks
- [ ] **Scope check < 1ms**: Measured with benchmarks
- [ ] **Content extraction < 2s**: Measured with benchmarks

**Verification Commands:**
```bash
# Run performance benchmarks
cargo bench --package agentsdk-scraping

# Verify robots.txt fetch performance
cargo bench --bench scraping_bench bench_robots_fetch

# Verify extraction performance
cargo bench --bench scraping_bench bench_extraction

# Expected output: All latency targets met
```

---

## Functional Requirements

### robots.txt Compliance

#### robots.txt Fetching
- [ ] **Fetches robots.txt**
  - Test: `test_fetch_robots()`
  - Command: `cargo test --package agentsdk-scraping test_fetch_robots`
  - Expected: PASS, robots.txt retrieved

- [ ] **Handles missing robots.txt**
  - Test: `test_missing_robots()`
  - Command: `cargo test --package agentsdk-scraping test_missing_robots`
  - Expected: PASS, allows all when missing

- [ ] **Handles robots.txt fetch failure**
  - Test: `test_robots_fetch_failure()`
  - Command: `cargo test --package agentsdk-scraping test_robots_fetch_failure`
  - Expected: PASS, denies on fetch failure

#### robots.txt Parsing
- [ ] **Parses disallow directives**
  - Test: `test_parse_disallow()`
  - Command: `cargo test --package agentsdk-scraping test_parse_disallow`
  - Expected: PASS, disallow paths blocked

- [ ] **Parses allow directives**
  - Test: `test_parse_allow()`
  - Command: `cargo test --package agentsdk-scraping test_parse_allow`
  - Expected: PASS, allow paths permitted

- [ ] **Parses crawl-delay directive**
  - Test: `test_parse_crawl_delay()`
  - Command: `cargo test --package agentsdk-scraping test_parse_crawl_delay`
  - Expected: PASS, delay applied to requests

- [ ] **Parses user-agent directives**
  - Test: `test_parse_user_agent()`
  - Command: `cargo test --package agentsdk-scraping test_parse_user_agent`
  - Expected: PASS, rules apply per user-agent

#### robots.txt Caching
- [ ] **Caches robots.txt with TTL**
  - Test: `test_robots_cache()`
  - Command: `cargo test --package agentsdk-scraping test_robots_cache`
  - Expected: PASS, cache used until TTL expires

- [ ] **Respects cache TTL**
  - Test: `test_robots_cache_ttl()`
  - Command: `cargo test --package agentsdk-scraping test_robots_cache_ttl`
  - Expected: PASS, re-fetches after TTL

- [ ] **Invalidates cache on errors**
  - Test: `test_robots_cache_invalidation()`
  - Command: `cargo test --package agentsdk-scraping test_robots_cache_invalidation`
  - Expected: PASS, cache cleared on parse errors

#### robots.txt Enforcement
- [ ] **Respects disallow directives**
  - Test: `test_disallow_enforcement()`
  - Command: `cargo test --package agentsdk-scraping test_disallow_enforcement`
  - Expected: PASS, disallowed paths blocked

- [ ] **Respects crawl-delay directive**
  - Test: `test_crawl_delay_enforcement()`
  - Command: `cargo test --package agentsdk-scraping test_crawl_delay_enforcement`
  - Expected: PASS, requests delayed

- [ ] **Denies on robots.txt errors**
  - Test: `test_robots_error_deny()`
  - Command: `cargo test --package agentsdk-scraping test_robots_error_deny`
  - Expected: PASS, fail closed on errors

### Scope Restrictions

#### Domain Restrictions
- [ ] **Enforces allowed domains list**
  - Test: `test_allowed_domains()`
  - Command: `cargo test --package agentsdk-scraping test_allowed_domains`
  - Expected: PASS, only allowed domains accessed

- [ ] **Enforces blocked domains list**
  - Test: `test_blocked_domains()`
  - Command: `cargo test --package agentsdk-scraping test_blocked_domains`
  - Expected: PASS, blocked domains rejected

- [ ] **Handles domain wildcards**
  - Test: `test_domain_wildcards()`
  - Command: `cargo test --package agentsdk-scraping test_domain_wildcards`
  - Expected: PASS, wildcard patterns matched

#### Path Restrictions
- [ ] **Enforces path patterns**
  - Test: `test_path_patterns()`
  - Command: `cargo test --package agentsdk-scraping test_path_patterns`
  - Expected: PASS, only matching paths accessed

- [ ] **Handles path wildcards**
  - Test: `test_path_wildcards()`
  - Command: `cargo test --package agentsdk-scraping test_path_wildcards`
  - Expected: PASS, wildcard patterns matched

- [ ] **Enforces URL pattern matching**
  - Test: `test_url_pattern_matching()`
  - Command: `cargo test --package agentsdk-scraping test_url_pattern_matching`
  - Expected: PASS, URLs matched against patterns

#### Depth Limits
- [ ] **Enforces depth limits**
  - Test: `test_depth_limits()`
  - Command: `cargo test --package agentsdk-scraping test_depth_limits`
  - Expected: PASS, doesn't crawl beyond max depth

- [ ] **Tracks crawl depth**
  - Test: `test_depth_tracking()`
  - Command: `cargo test --package agentsdk-scraping test_depth_tracking`
  - Expected: PASS, depth tracked correctly

#### Page Count Limits
- [ ] **Enforces page count limits**
  - Test: `test_page_count_limits()`
  - Command: `cargo test --package agentsdk-scraping test_page_count_limits`
  - Expected: PASS, stops at max pages

- [ ] **Tracks page count**
  - Test: `test_page_count_tracking()`
  - Command: `cargo test --package agentsdk-scraping test_page_count_tracking`
  - Expected: PASS, count tracked accurately

### Content Extraction

#### HTML Extraction
- [ ] **Extracts HTML content**
  - Test: `test_html_extraction()`
  - Command: `cargo test --package agentsdk-scraping test_html_extraction`
  - Expected: PASS, HTML parsed and extracted

- [ ] **Extracts page title**
  - Test: `test_title_extraction()`
  - Command: `cargo test --package agentsdk-scraping test_title_extraction`
  - Expected: PASS, title metadata captured

- [ ] **Detects content type**
  - Test: `test_content_type_detection()`
  - Command: `cargo test --package agentsdk-scraping test_content_type_detection`
  - Expected: PASS, MIME type identified correctly

#### Content Processing
- [ ] **Handles redirects**
  - Test: `test_redirect_handling()`
  - Command: `cargo test --package agentsdk-scraping test_redirect_handling`
  - Expected: PASS, follows redirects to final URL

- [ ] **Handles encoding**
  - Test: `test_encoding_handling()`
  - Command: `cargo test --package agentsdk-scraping test_encoding_handling`
  - Expected: PASS, character encoding detected and handled

- [ ] **Strips scripts and styles**
  - Test: `test_script_stripping()`
  - Command: `cargo test --package agentsdk-scraping test_script_stripping`
  - Expected: PASS, scripts and styles removed

### Extraction Traces

#### Trace Recording
- [ ] **Logs all extraction attempts**
  - Test: `test_extraction_logging()`
  - Command: `cargo test --package agentsdk-scraping test_extraction_logging`
  - Expected: PASS, all attempts logged

- [ ] **Records success/failure status**
  - Test: `test_status_recording()`
  - Command: `cargo test --package agentsdk-scraping test_status_recording`
  - Expected: PASS, status recorded for all attempts

- [ ] **Records robots.txt decision**
  - Test: `test_robots_decision_recording()`
  - Command: `cargo test --package agentsdk-scraping test_robots_decision_recording`
  - Expected: PASS, robots.txt decision logged

- [ ] **Provides trace history**
  - Test: `test_trace_history()`
  - Command: `cargo test --package agentsdk-scraping test_trace_history`
  - Expected: PASS, full history available

---

## Performance Requirements

### robots.txt Performance

- [ ] **robots.txt fetch < 500ms**
  - Benchmark: `bench_robots_fetch`
  - Command: `cargo bench --bench scraping_bench bench_robots_fetch`
  - Expected: Mean < 500.0 ms, p95 < 750.0 ms

- [ ] **robots.txt parse < 10ms**
  - Benchmark: `bench_robots_parse`
  - Command: `cargo bench --bench scraping_bench bench_robots_parse`
  - Expected: Mean < 10.0 ms, p95 < 15.0 ms

### Scope Checking Performance

- [ ] **Scope check < 1ms**
  - Benchmark: `bench_scope_check`
  - Command: `cargo bench --bench scraping_bench bench_scope_check`
  - Expected: Mean < 1.0 ms, p95 < 2.0 ms

### Content Extraction Performance

- [ ] **Content extraction < 2s**
  - Benchmark: `bench_extraction`
  - Command: `cargo bench --bench scraping_bench bench_extraction`
  - Expected: Mean < 2.0s, p95 < 3.0s

---

## Accuracy

### robots.txt Accuracy

- [ ] **robots.txt disallow respected 100%**
  - Test: `test_disallow_accuracy()`
  - Command: `cargo test --package agentsdk-scraping test_disallow_accuracy`
  - Expected: PASS, all disallowed paths blocked

- [ ] **robots.txt allow respected 100%**
  - Test: `test_allow_accuracy()`
  - Command: `cargo test --package agentsdk-scraping test_allow_accuracy`
  - Expected: PASS, all allowed paths permitted

### Scope Restrictions Accuracy

- [ ] **Scope violations prevented 100%**
  - Test: `test_scope_accuracy()`
  - Command: `cargo test --package agentsdk-scraping test_scope_accuracy`
  - Expected: PASS, no violations allowed

### Content Extraction Accuracy

- [ ] **Content extracted correctly**
  - Test: `test_extraction_accuracy()`
  - Command: `cargo test --package agentsdk-scraping test_extraction_accuracy`
  - Expected: PASS, content matches source

- [ ] **Title extracted correctly**
  - Test: `test_title_accuracy()`
  - Command: `cargo test --package agentsdk-scraping test_title_accuracy`
  - Expected: PASS, title matches <title> tag

---

## Error Handling

### robots.txt Errors

- [ ] **robots.txt fetch failure = deny**
  - Test: `test_robots_fetch_failure_handling()`
  - Command: `cargo test --package agentsdk-scraping test_robots_fetch_failure_handling`
  - Expected: PASS, denies when fetch fails

- [ ] **robots.txt parse failure = deny**
  - Test: `test_robots_parse_failure_handling()`
  - Command: `cargo test --package agentsdk-scraping test_robots_parse_failure_handling`
  - Expected: PASS, denies when parse fails

### Scope Errors

- [ ] **Scope violation = deny**
  - Test: `test_scope_violation_handling()`
  - Command: `cargo test --package agentsdk-scraping test_scope_violation_handling`
  - Expected: PASS, denies on violation

- [ ] **Invalid scope configuration rejected**
  - Test: `test_invalid_scope_config()`
  - Command: `cargo test --package agentsdk-scraping test_invalid_scope_config`
  - Expected: PASS, invalid config rejected

### Extraction Errors

- [ ] **Network errors handled**
  - Test: `test_network_error_handling()`
  - Command: `cargo test --package agentsdk-scraping test_network_error_handling`
  - Expected: PASS, network errors logged and failed

- [ ] **HTTP errors handled**
  - Test: `test_http_error_handling()`
  - Command: `cargo test --package agentsdk-scraping test_http_error_handling`
  - Expected: PASS, HTTP errors logged and failed

---

## Schema Compliance

### Trace Schema

- [ ] **Extraction trace structure matches schema**
  - Test: `test_trace_schema()`
  - Command: `cargo test --package agentsdk-scraping test_trace_schema`
  - Expected: PASS, all required fields present

### Content Schema

- [ ] **Extracted content structure matches schema**
  - Test: `test_content_schema()`
  - Command: `cargo test --package agentsdk-scraping test_content_schema`
  - Expected: PASS, all required fields present

---

## ADR-0006 Compliance

### robots.txt Enforcement

- [ ] **robots.txt MUST be checked before all requests**
  - Test: `test_robots_required()`
  - Command: `cargo test --package agentsdk-scraping test_robots_required`
  - Expected: PASS, no request skips robots.txt check

- [ ] **Scope restrictions MUST be enforced**
  - Test: `test_scope_required()`
  - Command: `cargo test --package agentsdk-scraping test_scope_required`
  - Expected: PASS, all requests checked against scope

- [ ] **crawl-delay MUST be respected**
  - Test: `test_crawl_delay_required()`
  - Command: `cargo test --package agentsdk-scraping test_crawl_delay_required`
  - Expected: PASS, delays applied per directive

### Extraction Traces

- [ ] **Extraction traces MUST be recorded**
  - Test: `test_traces_required()`
  - Command: `cargo test --package agentsdk-scraping test_traces_required`
  - Expected: PASS, all operations logged

- [ ] **Fail closed on robots.txt errors**
  - Test: `test_fail_closed_policy()`
  - Command: `cargo test --package agentsdk-scraping test_fail_closed_policy`
  - Expected: PASS, denies on robots.txt errors

---

## Integration Points

### Memory Integration

- [ ] **Extracted content stored in memory**
  - Test: `test_memory_storage_integration()`
  - Command: `cargo test --package agentsdk-scraping test_memory_storage_integration`
  - Expected: PASS, content stored after extraction

- [ ] **Extraction traces stored in provenance**
  - Test: `test_provenance_integration()`
  - Command: `cargo test --package agentsdk-scraping test_provenance_integration`
  - Expected: PASS, traces recorded in provenance

### External Search Integration

- [ ] **Web scraping works with external search**
  - Test: `test_external_search_integration()`
  - Command: `cargo test --package agentsdk-scraping test_external_search_integration`
  - Expected: PASS, content scraped from search results

---

## Log Verification Patterns

### robots.txt Logs

- [ ] **robots.txt fetch attempts logged**
  - Grep: `grep '"operation":"robots_fetch"' .glyphnova/logs/scraping.log | wc -l`
  - Expected: Count equals number of fetch attempts

- [ ] **robots.txt parse results logged**
  - Grep: `grep '"operation":"robots_parse"' .glyphnova/logs/scraping.log | wc -l`
  - Expected: Count equals number of parse operations

- [ ] **robots.txt decisions logged**
  - Grep: `grep '"operation":"robots_check"' .glyphnova/logs/scraping.log | jq -r '.decision'`
  - Expected: Decision present for all checks

### Extraction Logs

- [ ] **Extraction attempts logged**
  - Grep: `grep '"operation":"extract"' .glyphnova/logs/scraping.log | wc -l`
  - Expected: Count equals number of extractions

- [ ] **Extraction status logged**
  - Grep: `grep '"operation":"extract"' .glyphnova/logs/scraping.log | jq -r '.status'`
  - Expected: Status present for all extractions

- [ ] **Extraction duration logged**
  - Grep: `grep '"operation":"extract"' .glyphnova/logs/scraping.log | jq -r '.duration_ms'`
  - Expected: Duration in milliseconds

### Error Logs

- [ ] **Scope violations logged**
  - Grep: `grep '"event":"scope_violation"' .glyphnova/logs/scraping.log | wc -l`
  - Expected: Count equals number of violations

- [ ] **robots.txt errors logged**
  - Grep: `grep '"event":"robots_error"' .glyphnova/logs/scraping.log | jq -r '.error'`
  - Expected: Error details present for all errors

---

## Test Coverage

### Unit Tests

- [ ] **Unit tests for robots.txt parser**
  - Command: `cargo test --package agentsdk-scraping --lib robots`
  - Expected: All robots.txt tests pass

- [ ] **Unit tests for scope restrictions**
  - Command: `cargo test --package agentsdk-scraping --lib scope`
  - Expected: All scope tests pass

- [ ] **Unit tests for extraction**
  - Command: `cargo test --package agentsdk-scraping --lib extraction`
  - Expected: All extraction tests pass

### Integration Tests

- [ ] **Integration tests with mock robots.txt server**
  - Command: `cargo test --package agentsdk-scraping --test integration_test`
  - Expected: All integration tests pass

### Mock HTTP Response Tests

- [ ] **Mock HTTP response tests**
  - Command: `cargo test --package agentsdk-scraping --test mock_http`
  - Expected: All mock tests pass

---

## Final Checklist

### Implementation Complete
- [ ] robots.txt compliance implemented
- [ ] Scope restrictions functional
- [ ] Content extraction working
- [ ] Extraction traces recorded
- [ ] Performance meets all targets

### ADR-0006 Compliant
- [ ] robots.txt MUST be checked before all requests
- [ ] Scope restrictions MUST be enforced
- [ ] crawl-delay MUST be respected
- [ ] Extraction traces MUST be recorded
- [ ] Fail closed on robots.txt errors

### Integration Ready
- [ ] Works with memory storage
- [ ] Works with provenance tracking
- [ ] Works with external search

### Documentation Complete
- [ ] API documentation generated
- [ ] robots.txt support documented
- [ ] Scope configuration documented

### Tests Passing
- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] All benchmarks meet targets
- [ ] Test coverage > 90%

**Total Lines:** ~570 lines
