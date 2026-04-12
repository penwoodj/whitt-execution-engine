# Test Specifications: Task 04 - External Search Adapters

## Mock Strategy

**Mock HTTP Server:**
```rust
use wiremock::{MockServer, Mock, ResponseTemplate};

async fn setup_mock_server() -> MockServer {
    let server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/api/search"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_results))
        .mount(&server)
        .await;
    
    server
}
```

**Mock API Responses:**
```json
{
  "results": [
    {"title": "Test Result", "url": "http://test.com", "snippet": "..."}
  ]
}
```

## Test Cases

### Unit Tests

**Policy Gates**
```rust
#[tokio::test]
async fn test_policy_gate_allowed() {
    let gate = PolicyGate::new(false);
    let decision = gate.check_search("query", "provider").await.unwrap();
    assert!(matches!(decision, PolicyDecision::Allow));
}
```

**Rate Limiting**
```rust
#[tokio::test]
async fn test_rate_limit_enforcement() {
    let limiter = RateLimiterWrapper::new(2);
    
    // First request should pass
    assert!(limiter.check().is_ok());
    
    // Second request should pass
    assert!(limiter.check().is_ok());
    
    // Third request should fail
    assert!(limiter.check().is_err());
}
```

### Integration Tests

**DuckDuckGo Search**
- Mock DuckDuckGo API
- Execute search
- Verify results parsed
- Check error handling

**Brave Search**
- Mock Brave API
- Execute search
- Verify results parsed
- Check error handling

**Policy Enforcement**
- Test with approval required
- Test with approval denied
- Verify external search blocked

**Rate Limiting**
- Test rate limit enforcement
- Verify error messages
- Check rate limit headers

## Mock Dependencies

- `wiremock`: Mock HTTP server
- Pre-defined JSON responses: Test data
- Time control: Test rate limits
