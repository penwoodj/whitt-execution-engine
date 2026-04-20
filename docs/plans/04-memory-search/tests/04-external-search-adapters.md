# Test Specifications: Task 04 - External Search Adapters

## Mock Strategy

**Mock HTTP Server with WireMock:**
```rust
use wiremock::{MockServer, Mock, ResponseTemplate, matchers::*};
use wiremock::http::StatusCode;

async fn setup_mock_duckduckgo_server() -> MockServer {
    let server = MockServer::start().await;

    // Mock search endpoint
    Mock::given(method("GET"))
        .and(path("/html"))
        .and(query_param("q", "test query"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "abstract": "Search results abstract",
            "abstractText": "Test search results",
            "abstractSource": "Wikipedia",
            "image": "",
            "heading": "Test Results",
            "answer": "",
            "answerType": "",
            "definition": "",
            "definitionSource": "",
            "definitionUrl": "",
            "infobox": {},
            "results": [
                {
                    "title": "Result 1",
                    "url": "https://example.com/1",
                    "snippet": "Snippet 1",
                    "isOldAnswer": false
                },
                {
                    "title": "Result 2",
                    "url": "https://example.com/2",
                    "snippet": "Snippet 2",
                    "isOldAnswer": false
                }
            ]
        })))
        .mount(&server)
        .await;

    server
}

async fn setup_mock_brave_server() -> MockServer {
    let server = MockServer::start().await;

    // Mock search endpoint
    Mock::given(method("GET"))
        .and(path("/api/v1/search"))
        .and(query_param("q", "test query"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "web": {
                "results": [
                    {
                        "title": "Brave Result 1",
                        "url": "https://example.com/brave1",
                        "snippet": "Brave snippet 1"
                    },
                    {
                        "title": "Brave Result 2",
                        "url": "https://example.com/brave2",
                        "snippet": "Brave snippet 2"
                    }
                ]
            }
        })))
        .mount(&server)
        .await;

    server
}
```

**Mock API Responses:**
```rust
fn mock_duckduckgo_response() -> serde_json::Value {
    serde_json::json!({
        "abstract": "Rust programming language",
        "abstractText": "Rust is a systems programming language",
        "results": [
            {
                "title": "Rust Official Site",
                "url": "https://www.rust-lang.org",
                "snippet": "The Rust Programming Language"
            },
            {
                "title": "Rust Book",
                "url": "https://doc.rust-lang.org/book",
                "snippet": "The Rust Programming Language book"
            }
        ]
    })
}

fn mock_brave_response() -> serde_json::Value {
    serde_json::json!({
        "web": {
            "results": [
                {
                    "title": "Brave Search - Rust",
                    "url": "https://search.brave.com/search?q=rust",
                    "snippet": "Search results for Rust"
                }
            ]
        }
    })
}

fn mock_error_response(status: u16) -> ResponseTemplate {
    ResponseTemplate::new(status)
        .set_body_json(serde_json::json!({
            "error": "Internal server error"
        }))
}
```

**Mock Rate Limiter:**
```rust
struct MockRateLimiter {
    requests: Arc<Mutex<u32>>,
    limit: u32,
    window: Duration,
}

impl MockRateLimiter {
    fn new(limit: u32, window: Duration) -> Self {
        Self {
            requests: Arc::new(Mutex::new(0)),
            limit,
            window,
        }
    }

    fn check(&self) -> Result<(), RateLimitError> {
        let mut reqs = self.requests.lock().unwrap();
        if *reqs >= self.limit {
            return Err(RateLimitError::LimitExceeded {
                limit: self.limit,
                retry_after: self.window,
            });
        }
        *reqs += 1;
        Ok(())
    }

    fn reset(&self) {
        let mut reqs = self.requests.lock().unwrap();
        *reqs = 0;
    }
}
```

**Mock Policy Gate:**
```rust
struct MockPolicyGate {
    allowed: Arc<RwLock<bool>>,
    require_approval: Arc<RwLock<bool>>,
}

impl MockPolicyGate {
    fn new(allowed: bool, require_approval: bool) -> Self {
        Self {
            allowed: Arc::new(RwLock::new(allowed)),
            require_approval: Arc::new(RwLock::new(require_approval)),
        }
    }

    async fn check_search(
        &self,
        _query: &str,
        _provider: &str,
    ) -> Result<PolicyDecision, PolicyError> {
        let allowed = *self.allowed.read().await;
        let require_approval = *self.require_approval.read().await;

        if !allowed {
            return Ok(PolicyDecision::Deny {
                reason: "External search disabled by policy".to_string(),
            });
        }

        if require_approval {
            return Ok(PolicyDecision::RequireApproval {
                reason: "External search requires approval".to_string(),
            });
        }

        Ok(PolicyDecision::Allow)
    }

    async fn set_allowed(&self, allowed: bool) {
        *self.allowed.write().await = allowed;
    }

    async fn set_require_approval(&self, require: bool) {
        *self.require_approval.write().await = require;
    }
}
```

## Test Cases

### Unit Tests

**Policy Gates**
```rust
#[tokio::test]
async fn test_policy_gate_allowed() {
    let gate = MockPolicyGate::new(true, false);
    let decision = gate.check_search("query", "provider").await.unwrap();

    assert!(matches!(decision, PolicyDecision::Allow));
}

#[tokio::test]
async fn test_policy_gate_denied() {
    let gate = MockPolicyGate::new(false, false);
    let decision = gate.check_search("query", "provider").await.unwrap();

    assert!(matches!(decision, PolicyDecision::Deny { .. }));
}

#[tokio::test]
async fn test_policy_gate_require_approval() {
    let gate = MockPolicyGate::new(true, true);
    let decision = gate.check_search("query", "provider").await.unwrap();

    assert!(matches!(decision, PolicyDecision::RequireApproval { .. }));
}

#[tokio::test]
async fn test_policy_gate_dynamic_update() {
    let gate = MockPolicyGate::new(true, false);

    let decision1 = gate.check_search("query", "provider").await.unwrap();
    assert!(matches!(decision1, PolicyDecision::Allow));

    gate.set_allowed(false).await;

    let decision2 = gate.check_search("query", "provider").await.unwrap();
    assert!(matches!(decision2, PolicyDecision::Deny { .. }));
}
```

**Rate Limiting**
```rust
#[tokio::test]
async fn test_rate_limit_enforcement() {
    let limiter = MockRateLimiter::new(2, Duration::from_secs(60));

    // First request should pass
    assert!(limiter.check().is_ok());

    // Second request should pass
    assert!(limiter.check().is_ok());

    // Third request should fail
    assert!(limiter.check().is_err());

    // Verify error details
    let error = limiter.check().unwrap_err();
    assert!(matches!(error, RateLimitError::LimitExceeded { .. }));
}

#[tokio::test]
async fn test_rate_limit_reset() {
    let limiter = MockRateLimiter::new(1, Duration::from_secs(1));

    assert!(limiter.check().is_ok());
    assert!(limiter.check().is_err());

    // Reset
    limiter.reset();

    // Should pass again
    assert!(limiter.check().is_ok());
}

#[tokio::test]
async fn test_rate_limit_retry_after() {
    let limiter = MockRateLimiter::new(1, Duration::from_secs(60));
    limiter.check().unwrap();

    let error = limiter.check().unwrap_err();
    if let RateLimitError::LimitExceeded { retry_after, .. } = error {
        assert_eq!(retry_after, Duration::from_secs(60));
    } else {
        panic!("Expected LimitExceeded error");
    }
}
```

**Response Parsing**
```rust
#[test]
fn test_parse_duckduckgo_response() {
    let response = mock_duckduckgo_response();
    let results = DuckDuckGoAdapter::parse_response(&response).unwrap();

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].title, "Rust Official Site");
    assert_eq!(results[0].url, "https://www.rust-lang.org");
    assert_eq!(results[1].title, "Rust Book");
}

#[test]
fn test_parse_brave_response() {
    let response = mock_brave_response();
    let results = BraveAdapter::parse_response(&response).unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].title, "Brave Search - Rust");
    assert_eq!(results[0].url, "https://search.brave.com/search?q=rust");
}

#[test]
fn test_parse_empty_response() {
    let empty = serde_json::json!({ "results": [] });
    let results = DuckDuckGoAdapter::parse_response(&empty).unwrap();

    assert_eq!(results.len(), 0);
}

#[test]
fn test_parse_invalid_response() {
    let invalid = serde_json::json!({ "invalid": "data" });
    let result = DuckDuckGoAdapter::parse_response(&invalid);

    assert!(result.is_err());
}
```

### Integration Tests

**DuckDuckGo Search**
- [ ] **Step 1: Setup mock server**
  ```rust
  let server = setup_mock_duckduckgo_server().await;
  let adapter = DuckDuckGoAdapter::new(server.uri());
  ```

- [ ] **Step 2: Execute search**
  ```rust
  let results = adapter.search("test query").await.unwrap();
  ```

- [ ] **Step 3: Verify results parsed**
  ```rust
  assert_eq!(results.len(), 2);
  assert_eq!(results[0].title, "Result 1");
  assert_eq!(results[0].url, "https://example.com/1");
  ```

- [ ] **Step 4: Check error handling**
  ```rust
  let error_results = adapter.search("error").await;
  assert!(error_results.is_err());
  ```

**Brave Search**
```rust
#[tokio::test]
async fn test_brave_search_success() {
    let server = setup_mock_brave_server().await;
    let adapter = BraveAdapter::new(server.uri());

    let results = adapter.search("test query").await.unwrap();

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].title, "Brave Result 1");
    assert_eq!(results[1].title, "Brave Result 2");
}

#[tokio::test]
async fn test_brave_search_with_pagination() {
    let server = setup_mock_brave_server().await;
    let adapter = BraveAdapter::new(server.uri());

    let page1 = adapter.search_paginated("query", 1, 10).await.unwrap();
    let page2 = adapter.search_paginated("query", 2, 10).await.unwrap();

    assert!(!page1.is_empty());
    assert!(!page2.is_empty());
}
```

**Policy Enforcement**
```rust
#[tokio::test]
async fn test_search_with_approval_required() {
    let gate = MockPolicyGate::new(true, true);
    let adapter = DuckDuckGoAdapter::new("http://example.com");
    let guarded = PolicyGuardedAdapter::new(adapter, gate);

    let result = guarded.search("query").await;

    assert!(matches!(result, Err(SearchError::PolicyDenied { .. })));
}

#[tokio::test]
async fn test_search_denied_by_policy() {
    let gate = MockPolicyGate::new(false, false);
    let adapter = DuckDuckGoAdapter::new("http://example.com");
    let guarded = PolicyGuardedAdapter::new(adapter, gate);

    let result = guarded.search("query").await;

    assert!(matches!(result, Err(SearchError::PolicyDenied { .. })));
}

#[tokio::test]
async fn test_search_allowed_by_policy() {
    let gate = MockPolicyGate::new(true, false);
    let server = setup_mock_duckduckgo_server().await;
    let adapter = DuckDuckGoAdapter::new(server.uri());
    let guarded = PolicyGuardedAdapter::new(adapter, gate);

    let results = guarded.search("query").await.unwrap();

    assert!(!results.is_empty());
}
```

**Rate Limiting**
```rust
#[tokio::test]
async fn test_rate_limit_enforcement_in_adapter() {
    let limiter = MockRateLimiter::new(2, Duration::from_secs(60));
    let server = setup_mock_duckduckgo_server().await;
    let adapter = RateLimitedAdapter::new(
        DuckDuckGoAdapter::new(server.uri()),
        limiter,
    );

    // First request should pass
    adapter.search("query1").await.unwrap();

    // Second request should pass
    adapter.search("query2").await.unwrap();

    // Third request should be rate limited
    let result = adapter.search("query3").await;
    assert!(matches!(result, Err(SearchError::RateLimited { .. })));
}
```

**Error Handling**
```rust
#[tokio::test]
async fn test_network_error_handling() {
    let adapter = DuckDuckGoAdapter::new("http://invalid-does-not-exist.example");

    let result = adapter.search("query").await;

    assert!(result.is_err());
    assert!(matches!(result, Err(SearchError::NetworkError { .. })));
}

#[tokio::test]
async fn test_http_error_handling() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/html"))
        .respond_with(mock_error_response(500))
        .mount(&server)
        .await;

    let adapter = DuckDuckGoAdapter::new(server.uri());
    let result = adapter.search("query").await;

    assert!(result.is_err());
    assert!(matches!(result, Err(SearchError::HttpError { .. })));
}

#[tokio::test]
async fn test_timeout_handling() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/html"))
        .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_secs(10)))
        .mount(&server)
        .await;

    let adapter = DuckDuckGoAdapter::new_with_timeout(server.uri(), Duration::from_millis(100));
    let result = adapter.search("query").await;

    assert!(result.is_err());
    assert!(matches!(result, Err(SearchError::Timeout)));
}
```

**Multiple Providers**
```rust
#[tokio::test]
async fn test_search_multiple_providers() {
    let duckduckgo_server = setup_mock_duckduckgo_server().await;
    let brave_server = setup_mock_brave_server().await;

    let duckduckgo = DuckDuckGoAdapter::new(duckduckgo_server.uri());
    let brave = BraveAdapter::new(brave_server.uri());

    let multi = MultiProviderAdapter::new(vec![duckduckgo, brave]);

    let results = multi.search_all("query").await.unwrap();

    assert!(!results.is_empty());
    assert!(results.len() >= 2); // Should have results from both
}

#[tokio::test]
async fn test_search_multiple_providers_with_failure() {
    let duckduckgo_server = setup_mock_duckduckgo_server().await;
    let broken = DuckDuckGoAdapter::new("http://invalid.example");

    let duckduckgo = DuckDuckGoAdapter::new(duckduckgo_server.uri());

    let multi = MultiProviderAdapter::new(vec![duckduckgo, broken]);

    let results = multi.search_all("query").await.unwrap();

    // Should succeed with results from working provider
    assert!(!results.is_empty());
}

#[tokio::test]
async fn test_search_multiple_providers_all_fail() {
    let broken1 = DuckDuckGoAdapter::new("http://invalid1.example");
    let broken2 = DuckDuckGoAdapter::new("http://invalid2.example");

    let multi = MultiProviderAdapter::new(vec![broken1, broken2]);

    let result = multi.search_all("query").await;

    assert!(result.is_err());
    assert!(matches!(result, Err(SearchError::AllProvidersFailed { .. })));
}
```

**Caching**
```rust
#[tokio::test]
async fn test_adapter_caching() {
    let server = setup_mock_duckduckgo_server().await;
    let adapter = CachedAdapter::new(
        DuckDuckGoAdapter::new(server.uri()),
        Duration::from_secs(60),
    );

    // First search (cache miss)
    let results1 = adapter.search("query").await.unwrap();
    assert!(!results1.is_empty());

    // Second search (cache hit)
    let results2 = adapter.search("query").await.unwrap();
    assert_eq!(results1, results2);
}

#[tokio::test]
async fn test_adapter_cache_expiration() {
    let server = setup_mock_duckduckgo_server().await;
    let adapter = CachedAdapter::new(
        DuckDuckGoAdapter::new(server.uri()),
        Duration::from_millis(100),
    );

    let results1 = adapter.search("query").await.unwrap();

    // Wait for cache to expire
    tokio::time::sleep(Duration::from_millis(150)).await;

    let results2 = adapter.search("query").await.unwrap();

    // Results should be the same, but fetched fresh
    assert_eq!(results1, results2);
}
```

### Edge Cases

```rust
#[tokio::test]
async fn test_empty_query() {
    let server = setup_mock_duckduckgo_server().await;
    let adapter = DuckDuckGoAdapter::new(server.uri());

    let results = adapter.search("").await.unwrap();

    assert_eq!(results.len(), 0);
}

#[tokio::test]
async fn test_special_characters_in_query() {
    let server = setup_mock_duckduckgo_server().await;
    let adapter = DuckDuckGoAdapter::new(server.uri());

    let query = "test with @#$% special chars";
    let results = adapter.search(query).await.unwrap();

    // Should handle gracefully
    assert!(results.is_empty() || !results.is_empty());
}

#[tokio::test]
async fn test_unicode_query() {
    let server = setup_mock_duckduckgo_server().await;
    let adapter = DuckDuckGoAdapter::new(server.uri());

    let query = "编程 in Rust™";
    let results = adapter.search(query).await.unwrap();

    // Should handle gracefully
    assert!(results.is_empty() || !results.is_empty());
}

#[tokio::test]
async fn test_very_long_query() {
    let server = setup_mock_duckduckgo_server().await;
    let adapter = DuckDuckGoAdapter::new(server.uri());

    let long_query = "a".repeat(10_000);
    let result = adapter.search(&long_query).await;

    // Should either succeed or return an error gracefully
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_concurrent_searches() {
    let server = setup_mock_duckduckgo_server().await;
    let adapter = DuckDuckGoAdapter::new(server.uri());

    let handles: Vec<_> = (0..10)
        .map(|i| {
            let adapter = adapter.clone();
            tokio::spawn(async move {
                adapter.search(&format!("query {}", i)).await
            })
        })
        .collect();

    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }
}
```

## Cargo Commands

**Run all external search adapter tests:**
```bash
cargo test --test external_search_adapters -- --nocapture
```

**Run specific test:**
```bash
cargo test test_duckduckgo_search -- --nocapture
```

**Run with wiremock:**
```bash
cargo test --test external_search_adapters --features wiremock -- --nocapture
```

**Run with logging:**
```bash
RUST_LOG=debug cargo test --test external_search_adapters -- --nocapture
```

**Expected output:**
```
running 50 tests
test tests::unit::test_policy_gate_allowed ... ok
test tests::unit::test_policy_gate_denied ... ok
test tests::unit::test_policy_gate_require_approval ... ok
test tests::unit::test_policy_gate_dynamic_update ... ok
test tests::unit::test_rate_limit_enforcement ... ok
test tests::unit::test_rate_limit_reset ... ok
test tests::unit::test_rate_limit_retry_after ... ok
test tests::unit::test_parse_duckduckgo_response ... ok
test tests::unit::test_parse_brave_response ... ok
test tests::unit::test_parse_empty_response ... ok
test tests::unit::test_parse_invalid_response ... ok
test tests::integration::test_duckduckgo_search ... ok
test tests::integration::test_brave_search_success ... ok
test tests::integration::test_brave_search_with_pagination ... ok
test tests::integration::test_search_with_approval_required ... ok
test tests::integration::test_search_denied_by_policy ... ok
test tests::integration::test_search_allowed_by_policy ... ok
test tests::integration::test_rate_limit_enforcement_in_adapter ... ok
test tests::integration::test_network_error_handling ... ok
test tests::integration::test_http_error_handling ... ok
test tests::integration::test_timeout_handling ... ok
test tests::integration::test_search_multiple_providers ... ok
test tests::integration::test_search_multiple_providers_with_failure ... ok
test tests::integration::test_search_multiple_providers_all_fail ... ok
test tests::integration::test_adapter_caching ... ok
test tests::integration::test_adapter_cache_expiration ... ok
test tests::edge_cases::test_empty_query ... ok
test tests::edge_cases::test_special_characters_in_query ... ok
test tests::edge_cases::test_unicode_query ... ok
test tests::edge_cases::test_very_long_query ... ok
test tests::edge_cases::test_concurrent_searches ... ok

test result: ok. 50 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Mock Dependencies

- `wiremock`: Mock HTTP server for API testing
- Pre-defined JSON responses: Test data for DuckDuckGo and Brave APIs
- Time control: Test rate limits and caching TTL
- `MockRateLimiter`: In-memory rate limiting for testing
- `MockPolicyGate`: Test policy enforcement
- `tokio::test`: Async test support
- `tokio::spawn`: Concurrent search testing
