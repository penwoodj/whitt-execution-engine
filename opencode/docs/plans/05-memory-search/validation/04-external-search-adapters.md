# Validation Criteria: Task 04 - External Search Adapters

## Overview

Validate that external search adapters provide policy-gated access to DuckDuckGo and Brave search APIs with rate limiting, approval workflows, and ADR-0006 compliance.

---

## Checkpoint Gate Criteria

### Checkpoint 1: Policy Gates Implemented
- [ ] **Policy gate enforcement works**: All external search checked
- [ ] **User approval required by default**: Default deny unless approved
- [ ] **Policy decision is configurable**: Per-provider, per-user settings
- [ ] **Approval callback supported**: Async approval workflow
- [ ] **Unit tests pass**: All policy tests (`cargo test --package agentsdk-external`)

**Verification Commands:**
```bash
# Verify external crate builds
cargo check --package agentsdk-external

# Run policy gate tests
cargo test --package agentsdk-external --lib policy

# Expected output: All policy tests pass
```

### Checkpoint 2: Rate Limiting Functional
- [ ] **Rate limit enforcement works**: Per-provider limits enforced
- [ ] **Rate limits configurable**: Per-user, per-provider settings
- [ ] **Rate limit exceeded errors returned**: Graceful error on exceeded
- [ ] **Rate limits persist across requests**: Shared state for rate tracking
- [ ] **Unit tests pass**: All rate limit tests

**Verification Commands:**
```bash
# Run rate limit tests
cargo test --package agentsdk-external --lib rate_limit

# Verify rate limit enforcement
cargo test --package agentsdk-external test_rate_limit_enforcement

# Expected output: All rate limit tests pass
```

### Checkpoint 3: Search Adapters Working
- [ ] **DuckDuckGo adapter works**: Search API calls successful
- [ ] **Brave adapter works**: Search API calls successful
- [ ] **Results parsed correctly**: Structured results from APIs
- [ ] **Errors handled gracefully**: Network and API errors handled
- [ ] **Unit tests pass**: All adapter tests

**Verification Commands:**
```bash
# Run adapter tests
cargo test --package agentsdk-external --lib adapters

# Verify DuckDuckGo adapter
cargo test --package agentsdk-external test_duckduckgo_adapter

# Verify Brave adapter
cargo test --package agentsdk-external test_brave_adapter

# Expected output: All adapter tests pass
```

### Checkpoint 4: Performance Meets Targets
- [ ] **Policy check < 1ms**: Measured with benchmarks
- [ ] **Rate limit check < 1ms**: Measured with benchmarks
- [ ] **External search < 2s**: Measured with benchmarks
- [ ] **Adapter initialization < 100ms**: Measured with benchmarks

**Verification Commands:**
```bash
# Run performance benchmarks
cargo bench --package agentsdk-external

# Verify policy check performance
cargo bench --bench external_bench bench_policy_check

# Verify external search performance
cargo bench --bench external_bench bench_search_latency

# Expected output: All latency targets met
```

---

## Functional Requirements

### Policy Gates

#### Gate Enforcement
- [ ] **All external search requests pass policy gate**
  - Test: `test_policy_gate_enforcement()`
  - Command: `cargo test --package agentsdk-external test_policy_gate_enforcement`
  - Expected: PASS, no search bypasses policy gate

- [ ] **User approval required by default**
  - Test: `test_default_approval_required()`
  - Command: `cargo test --package agentsdk-external test_default_approval_required`
  - Expected: PASS, default denies without approval

- [ ] **Policy decision is configurable**
  - Test: `test_policy_configuration()`
  - Command: `cargo test --package agentsdk-external test_policy_configuration`
  - Expected: PASS, per-provider, per-user settings work

- [ ] **Approval callback supported**
  - Test: `test_approval_callback()`
  - Command: `cargo test --package agentsdk-external test_approval_callback`
  - Expected: PASS, async approval workflow works

#### Policy Decision
- [ ] **Approves requests when policy allows**
  - Test: `test_policy_approve()`
  - Command: `cargo test --package agentsdk-external test_policy_approve`
  - Expected: PASS, approved requests proceed

- [ ] **Denies requests when policy disallows**
  - Test: `test_policy_deny()`
  - Command: `cargo test --package agentsdk-external test_policy_deny`
  - Expected: PASS, denied requests blocked

- [ ] **Returns detailed denial reason**
  - Test: `test_denial_reason()`
  - Command: `cargo test --package agentsdk-external test_denial_reason`
  - Expected: PASS, reason includes policy details

#### Approval Workflow
- [ ] **Requests user approval for new search providers**
  - Test: `test_approval_request()`
  - Command: `cargo test --package agentsdk-external test_approval_request`
  - Expected: PASS, approval prompt shown

- [ ] **Caches approval decisions**
  - Test: `test_approval_cache()`
  - Command: `cargo test --package agentsdk-external test_approval_cache`
  - Expected: PASS, repeated requests don't re-approve

- [ ] **Expires approval cache**
  - Test: `test_approval_cache_expiry()`
  - Command: `cargo test --package agentsdk-external test_approval_cache_expiry`
  - Expected: PASS, cache expires after TTL

### Rate Limiting

#### Rate Limit Enforcement
- [ ] **Enforces rate limits per provider**
  - Test: `test_provider_rate_limit()`
  - Command: `cargo test --package agentsdk-external test_provider_rate_limit`
  - Expected: PASS, provider limits enforced

- [ ] **Limits are configurable**
  - Test: `test_rate_limit_config()`
  - Command: `cargo test --package agentsdk-external test_rate_limit_config`
  - Expected: PASS, per-provider limits configurable

- [ ] **Rate limit exceeded errors returned**
  - Test: `test_rate_limit_exceeded_error()`
  - Command: `cargo test --package agentsdk-external test_rate_limit_exceeded_error`
  - Expected: PASS, clear error when limit exceeded

- [ ] **Rate limits persist across requests**
  - Test: `test_rate_limit_persistence()`
  - Command: `cargo test --package agentsdk-external test_rate_limit_persistence`
  - Expected: PASS, tracking shared across instances

#### Rate Limit Tracking
- [ ] **Tracks request counts**
  - Test: `test_request_tracking()`
  - Command: `cargo test --package agentsdk-external test_request_tracking`
  - Expected: PASS, accurate count of requests

- [ ] **Resets rate limits at interval**
  - Test: `test_rate_limit_reset()`
  - Command: `cargo test --package agentsdk-external test_rate_limit_reset`
  - Expected: PASS, limits reset after window

- [ ] **Handles concurrent requests correctly**
  - Test: `test_concurrent_rate_limiting()`
  - Command: `cargo test --package agentsdk-external test_concurrent_rate_limiting`
  - Expected: PASS, no race conditions

### Search Adapters

#### DuckDuckGo Adapter
- [ ] **DuckDuckGo adapter works**
  - Test: `test_duckduckgo_search()`
  - Command: `cargo test --package agentsdk-external test_duckduckgo_search`
  - Expected: PASS, search results returned

- [ ] **Results parsed correctly**
  - Test: `test_duckduckgo_parsing()`
  - Command: `cargo test --package agentsdk-external test_duckduckgo_parsing`
  - Expected: PASS, structured results from API response

- [ ] **Handles DuckDuckGo errors gracefully**
  - Test: `test_duckduckgo_error_handling()`
  - Command: `cargo test --package agentsdk-external test_duckduckgo_error_handling`
  - Expected: PASS, errors handled gracefully

#### Brave Adapter
- [ ] **Brave adapter works**
  - Test: `test_brave_search()`
  - Command: `cargo test --package agentsdk-external test_brave_search`
  - Expected: PASS, search results returned

- [ ] **Results parsed correctly**
  - Test: `test_brave_parsing()`
  - Command: `cargo test --package agentsdk-external test_brave_parsing`
  - Expected: PASS, structured results from API response

- [ ] **Handles Brave errors gracefully**
  - Test: `test_brave_error_handling()`
  - Command: `cargo test --package agentsdk-external test_brave_error_handling`
  - Expected: PASS, errors handled gracefully

#### Adapter Interface
- [ ] **Adapter selection works**
  - Test: `test_adapter_selection()`
  - Command: `cargo test --package agentsdk-external test_adapter_selection`
  - Expected: PASS, correct adapter selected by provider

- [ ] **Adapter configuration works**
  - Test: `test_adapter_configuration()`
  - Command: `cargo test --package agentsdk-external test_adapter_configuration`
  - Expected: PASS, adapter configured correctly

- [ ] **Fallback to alternative adapter**
  - Test: `test_adapter_fallback()`
  - Command: `cargo test --package agentsdk-external test_adapter_fallback`
  - Expected: PASS, fails over to alternative on error

---

## Performance Requirements

### Policy Gate Performance

- [ ] **Policy check < 1ms**
  - Benchmark: `bench_policy_check`
  - Command: `cargo bench --bench external_bench bench_policy_check`
  - Expected: Mean < 1.0 ms, p95 < 2.0 ms

- [ ] **Approval cache lookup < 1ms**
  - Benchmark: `bench_approval_cache`
  - Command: `cargo bench --bench external_bench bench_approval_cache`
  - Expected: Mean < 1.0 ms, p95 < 2.0 ms

### Rate Limit Performance

- [ ] **Rate limit check < 1ms**
  - Benchmark: `bench_rate_limit_check`
  - Command: `cargo bench --bench external_bench bench_rate_limit_check`
  - Expected: Mean < 1.0 ms, p95 < 2.0 ms

### External Search Performance

- [ ] **External search < 2s**
  - Benchmark: `bench_external_search`
  - Command: `cargo bench --bench external_bench bench_external_search`
  - Expected: Mean < 2.0s, p95 < 3.0s

- [ ] **Adapter initialization < 100ms**
  - Benchmark: `bench_adapter_init`
  - Command: `cargo bench --bench external_bench bench_adapter_init`
  - Expected: Mean < 100.0 ms, p95 < 150.0 ms

---

## Accuracy

### Policy Gate Accuracy

- [ ] **Policy decisions correct**
  - Test: `test_policy_accuracy()`
  - Command: `cargo test --package agentsdk-external test_policy_accuracy`
  - Expected: PASS, 100% correct decisions

- [ ] **Approval cache consistent with policy**
  - Test: `test_approval_cache_consistency()`
  - Command: `cargo test --package agentsdk-external test_approval_cache_consistency`
  - Expected: PASS, cache matches policy decisions

### Rate Limit Accuracy

- [ ] **Rate limits enforced correctly**
  - Test: `test_rate_limit_accuracy()`
  - Command: `cargo test --package agentsdk-external test_rate_limit_accuracy`
  - Expected: PASS, no requests exceed configured limits

- [ ] **Rate limit reset timing accurate**
  - Test: `test_rate_limit_reset_accuracy()`
  - Command: `cargo test --package agentsdk-external test_rate_limit_reset_accuracy`
  - Expected: PASS, resets at configured interval

### Search Results Accuracy

- [ ] **Search results from API returned accurately**
  - Test: `test_search_result_accuracy()`
  - Command: `cargo test --package agentsdk-external test_search_result_accuracy`
  - Expected: PASS, results match API response

- [ ] **Results parsed and structured correctly**
  - Test: `test_result_parsing_accuracy()`
  - Command: `cargo test --package agentsdk-external test_result_parsing_accuracy`
  - Expected: PASS, all fields parsed correctly

---

## Error Handling

### Policy Errors

- [ ] **Policy denial handled gracefully**
  - Test: `test_policy_denial_handling()`
  - Command: `cargo test --package agentsdk-external test_policy_denial_handling`
  - Expected: PASS, denial error returned clearly

- [ ] **Policy configuration errors handled**
  - Test: `test_policy_config_error()`
  - Command: `cargo test --package agentsdk-external test_policy_config_error`
  - Expected: PASS, invalid config rejected

### Rate Limit Errors

- [ ] **Rate limit exceeded handled**
  - Test: `test_rate_limit_exceeded_handling()`
  - Command: `cargo test --package agentsdk-external test_rate_limit_exceeded_handling`
  - Expected: PASS, clear error with retry-after

### Adapter Errors

- [ ] **API errors handled**
  - Test: `test_api_error_handling()`
  - Command: `cargo test --package agentsdk-external test_api_error_handling`
  - Expected: PASS, API errors propagated with context

- [ ] **Network errors handled**
  - Test: `test_network_error_handling()`
  - Command: `cargo test --package agentsdk-external test_network_error_handling`
  - Expected: PASS, network errors propagated with context

---

## Schema Compliance

### Policy Configuration Schema

- [ ] **Policy configuration matches schema**
  - Test: `test_policy_schema()`
  - Command: `cargo test --package agentsdk-external test_policy_schema`
  - Expected: PASS, all required fields present

### Rate Limit Schema

- [ ] **Rate limit configuration matches schema**
  - Test: `test_rate_limit_schema()`
  - Command: `cargo test --package agentsdk-external test_rate_limit_schema`
  - Expected: PASS, all required fields present

### Search Result Schema

- [ ] **Search result structure matches schema**
  - Test: `test_search_result_schema()`
  - Command: `cargo test --package agentsdk-external test_search_result_schema`
  - Expected: PASS, all required fields present

---

## ADR-0006 Compliance

### Local Memory First

- [ ] **External search ONLY after local memory exhausted**
  - Test: `test_local_first()`
  - Command: `cargo test --package agentsdk-external test_local_first`
  - Expected: PASS, external search not called when local results available

- [ ] **Local results exhausted before external search**
  - Test: `test_local_exhaustion_check()`
  - Command: `cargo test --package agentsdk-external test_local_exhaustion_check`
  - Expected: PASS, external search only after threshold

### Policy Gates

- [ ] **Policy gate MUST approve all external searches**
  - Test: `test_policy_gate_required()`
  - Command: `cargo test --package agentsdk-external test_policy_gate_required`
  - Expected: PASS, no external search bypasses policy

- [ ] **Rate limits enforced**
  - Test: `test_rate_limit_enforcement_required()`
  - Command: `cargo test --package agentsdk-external test_rate_limit_enforcement_required`
  - Expected: PASS, all requests checked against rate limits

- [ ] **User consent required for each search**
  - Test: `test_user_consent_required()`
  - Command: `cargo test --package agentsdk-external test_user_consent_required`
  - Expected: PASS, approval required per search

---

## Integration Points

### Query Engine Integration

- [ ] **Query engine calls external search after local**
  - Test: `test_query_engine_integration()`
  - Command: `cargo test --package agentsdk-external test_query_engine_integration`
  - Expected: PASS, external search called after local exhausted

- [ ] **External results merged with local**
  - Test: `test_result_merging()`
  - Command: `cargo test --package agentsdk-external test_result_merging`
  - Expected: PASS, results merged and deduplicated

### CLI Integration

- [ ] **CLI commands respect policy gates**
  - Test: `test_cli_policy_integration()`
  - Command: `cargo test --package agentsdk-external test_cli_policy_integration`
  - Expected: PASS, CLI prompts for approval

---

## Log Verification Patterns

### Policy Gate Logs

- [ ] **Policy gate decisions logged**
  - Grep: `grep '"operation":"policy_gate"' .glyphnova/logs/external.log | wc -l`
  - Expected: Count equals number of policy checks

- [ ] **Approval decisions logged**
  - Grep: `grep '"decision":"approved"' .glyphnova/logs/external.log | wc -l`
  - Expected: Count equals number of approvals

- [ ] **Denial reasons logged**
  - Grep: `grep '"decision":"denied"' .glyphnova/logs/external.log | jq -r '.reason'`
  - Expected: Reason present for all denials

### Rate Limit Logs

- [ ] **Rate limit checks logged**
  - Grep: `grep '"operation":"rate_limit_check"' .glyphnova/logs/external.log | wc -l`
  - Expected: Count equals number of checks

- [ ] **Rate limit exceeded logged**
  - Grep: `grep '"event":"rate_limit_exceeded"' .glyphnova/logs/external.log | wc -l`
  - Expected: Count equals number of exceeded events

### External Search Logs

- [ ] **External search requests logged**
  - Grep: `grep '"operation":"external_search"' .glyphnova/logs/external.log | wc -l`
  - Expected: Count equals number of searches

- [ ] **Search provider logged**
  - Grep: `grep '"operation":"external_search"' .glyphnova/logs/external.log | jq -r '.provider'`
  - Expected: Provider present for all searches

- [ ] **Search duration logged**
  - Grep: `grep '"operation":"external_search"' .glyphnova/logs/external.log | jq -r '.duration_ms'`
  - Expected: Duration in milliseconds

---

## Test Coverage

### Unit Tests

- [ ] **Unit tests for policy gates**
  - Command: `cargo test --package agentsdk-external --lib policy`
  - Expected: All policy tests pass

- [ ] **Unit tests for rate limiting**
  - Command: `cargo test --package agentsdk-external --lib rate_limit`
  - Expected: All rate limit tests pass

- [ ] **Unit tests for adapters**
  - Command: `cargo test --package agentsdk-external --lib adapters`
  - Expected: All adapter tests pass

### Integration Tests

- [ ] **Integration tests with mock HTTP**
  - Command: `cargo test --package agentsdk-external --test integration_test`
  - Expected: All integration tests pass

### Mock API Response Tests

- [ ] **Mock API response tests**
  - Command: `cargo test --package agentsdk-external --test mock_api`
  - Expected: All mock tests pass

---

## Final Checklist

### Implementation Complete
- [ ] Policy gates implemented and tested
- [ ] Rate limiting functional
- [ ] Search adapters working
- [ ] Performance meets all targets

### ADR-0006 Compliant
- [ ] External search ONLY after local memory exhausted
- [ ] Policy gate MUST approve all external searches
- [ ] Rate limits enforced
- [ ] User consent required for each search

### Integration Ready
- [ ] Query engine integration works
- [ ] CLI integration works
- [ ] Results merged with local

### Documentation Complete
- [ ] API documentation generated
- [ ] Policy configuration documented
- [ ] Rate limit configuration documented

### Tests Passing
- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] All benchmarks meet targets
- [ ] Test coverage > 90%

**Total Lines:** ~550 lines
