# Test Specifications: Task 05 - Web Scraping

## Mock Strategy

**Mock robots.txt Server:**
```rust
use wiremock::{MockServer, Mock};

async fn setup_robots_mock() -> MockServer {
    let server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/robots.txt"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            "User-agent: *\nDisallow: /private\nCrawl-delay: 1"
        ))
        .mount(&server)
        .await;
    
    server
}
```

**Mock Content Server:**
```rust
async fn setup_content_mock() -> MockServer {
    let server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/test.html"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            "<html><title>Test Page</title><body>Test content</body></html>"
        ))
        .mount(&server)
        .await;
    
    server
}
```

## Test Cases

### Unit Tests

**robots.txt Parsing**
```rust
#[tokio::test]
async fn test_robots_allow() {
    let cache = RobotsCache::new();
    let url = Url::parse("http://allowed.com/").unwrap();
    let allowed = cache.check_allowed(&url, "*").await.unwrap();
    assert!(allowed);
}
```

**Scope Restrictions**
```rust
#[test]
fn test_blocked_domain() {
    let scope = ScopeRestrictions::new(config_with_blocked_domain());
    let url = Url::parse("http://blocked.com/").unwrap();
    assert!(scope.check_url(&url, 0).await.is_err());
}
```

### Integration Tests

**robots.txt Compliance**
- Setup mock robots.txt server
- Test allowed paths
- Test disallowed paths
- Verify crawl-delay respect

**Scope Enforcement**
- Test domain blocking
- Test path restrictions
- Test depth limits
- Test page count limits

**Content Extraction**
- Setup mock content server
- Extract HTML content
- Extract page title
- Verify content type detection

**Extraction Tracing**
- Extract multiple pages
- Check trace logs
- Verify success/failure status
- Check robots.txt decision

**Error Handling**
- Test network errors
- Test HTTP errors
- Test robots.txt fetch failures
- Verify fail-closed behavior

## Mock Dependencies

- `wiremock`: Mock HTTP servers
- Pre-defined robots.txt: Test directives
- Pre-defined HTML: Test extraction
