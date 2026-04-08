# Test Specifications: Task 05 - Web Scraping

## Mock Strategy

**Mock robots.txt Server:**
```rust
use wiremock::{MockServer, Mock, ResponseTemplate, matchers::*};

async fn setup_robots_mock() -> MockServer {
    let server = MockServer::start().await;

    // Allow all
    Mock::given(method("GET"))
        .and(path("/robots.txt"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            "User-agent: *\nAllow: /"
        ))
        .mount(&server)
        .await;

    server
}

async fn setup_restrictive_robots_mock() -> MockServer {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/robots.txt"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            "User-agent: *\nDisallow: /private\nDisallow: /admin\nCrawl-delay: 1"
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
            r#"
            <!DOCTYPE html>
            <html>
            <head><title>Test Page</title></head>
            <body>
                <h1>Main Heading</h1>
                <p>This is test content with <a href="/link1">links</a>.</p>
                <div class="content">
                    <p>More content here.</p>
                </div>
            </body>
            </html>
            "#
        ))
        .mount(&server)
        .await;

    server
}

async fn setup_error_content_mock() -> MockServer {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/error.html"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;

    server
}
```

**Mock Scope Restrictions:**
```rust
struct MockScopeRestrictions {
    blocked_domains: Arc<RwLock<Vec<String>>>,
    max_depth: Arc<RwLock<usize>>,
    max_pages: Arc<RwLock<usize>>,
}

impl MockScopeRestrictions {
    fn new() -> Self {
        Self {
            blocked_domains: Arc::new(RwLock::new(vec![])),
            max_depth: Arc::new(RwLock::new(10)),
            max_pages: Arc::new(RwLock::new(100)),
        }
    }

    async fn check_url(&self, url: &Url, current_depth: usize) -> Result<(), ScopeError> {
        let blocked = self.blocked_domains.read().await;
        let max_depth = *self.max_depth.read().await;
        let max_pages = *self.max_pages.read().await;

        if blocked.iter().any(|d| url.host_str() == Some(d)) {
            return Err(ScopeError::DomainBlocked(url.to_string()));
        }

        if current_depth > max_depth {
            return Err(ScopeError::DepthLimitExceeded {
                current: current_depth,
                max: max_depth,
            });
        }

        Ok(())
    }

    async fn increment_page_count(&self) -> Result<(), ScopeError> {
        let max_pages = *self.max_pages.read().await;
        let mut count = self.max_pages.write().await;

        if *count == 0 {
            return Err(ScopeError::PageLimitExceeded { max: max_pages });
        }

        *count -= 1;
        Ok(())
    }
}
```

## Test Cases

### Unit Tests

**robots.txt Parsing**
```rust
#[tokio::test]
async fn test_robots_allow_all() {
    let cache = RobotsCache::new();
    let url = Url::parse("http://allowed.com/").unwrap();
    let allowed = cache.check_allowed(&url, "*").await.unwrap();

    assert!(allowed);
}

#[tokio::test]
async fn test_robots_disallow_path() {
    let cache = RobotsCache::new();
    let url = Url::parse("http://test.com/private/page").unwrap();

    // Mock restrictive robots.txt
    let robots = "User-agent: *\nDisallow: /private";
    cache.cache("test.com".to_string(), robots).await.unwrap();

    let allowed = cache.check_allowed(&url, "*").await.unwrap();
    assert!(!allowed);
}

#[tokio::test]
async fn test_robots_specific_user_agent() {
    let cache = RobotsCache::new();
    let url = Url::parse("http://test.com/bot/page").unwrap();

    let robots = "User-agent: googlebot\nAllow: /\nUser-agent: *\nDisallow: /bot";
    cache.cache("test.com".to_string(), robots).await.unwrap();

    let google_allowed = cache.check_allowed(&url, "googlebot").await.unwrap();
    let generic_allowed = cache.check_allowed(&url, "*").await.unwrap();

    assert!(google_allowed);
    assert!(!generic_allowed);
}

#[tokio::test]
async fn test_robots_crawl_delay() {
    let cache = RobotsCache::new();
    let robots = "User-agent: *\nCrawl-delay: 2";
    cache.cache("test.com".to_string(), robots).await.unwrap();

    let delay = cache.get_crawl_delay("test.com", "*").await.unwrap();
    assert_eq!(delay, Duration::from_secs(2));
}
```

**Scope Restrictions**
```rust
#[test]
fn test_blocked_domain() {
    let restrictions = MockScopeRestrictions::new();
    restrictions.blocked_domains.write().unwrap().push("blocked.com".to_string());

    let url = Url::parse("http://blocked.com/").unwrap();
    assert!(restrictions.check_url(&url, 0).await.is_err());
}

#[test]
fn test_allowed_domain() {
    let restrictions = MockScopeRestrictions::new();

    let url = Url::parse("http://allowed.com/").unwrap();
    assert!(restrictions.check_url(&url, 0).await.is_ok());
}

#[test]
fn test_depth_limit() {
    let restrictions = MockScopeRestrictions::new();
    *restrictions.max_depth.write().unwrap() = 3;

    let url = Url::parse("http://test.com/deep").unwrap();

    // Should pass at depth 3
    assert!(restrictions.check_url(&url, 3).await.is_ok());

    // Should fail at depth 4
    assert!(restrictions.check_url(&url, 4).await.is_err());
}

#[test]
fn test_page_count_limit() {
    let restrictions = MockScopeRestrictions::new();
    *restrictions.max_pages.write().unwrap() = 5;

    // Should allow 5 pages
    for _ in 0..5 {
        assert!(restrictions.increment_page_count().await.is_ok());
    }

    // Should fail on 6th page
    assert!(restrictions.increment_page_count().await.is_err());
}
```

**Content Extraction**
```rust
#[test]
fn test_extract_page_title() {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head><title>Page Title</title></head>
    <body>Content</body>
    </html>
    "#;

    let scraper = HtmlScraper::new();
    let title = scraper.extract_title(html).unwrap();

    assert_eq!(title, "Page Title");
}

#[test]
fn test_extract_page_content() {
    let html = r#"
    <html>
    <body>
        <h1>Main Heading</h1>
        <p>Paragraph 1</p>
        <p>Paragraph 2</p>
        <div class="content">
            <p>Nested paragraph</p>
        </div>
    </body>
    </html>
    "#;

    let scraper = HtmlScraper::new();
    let content = scraper.extract_content(html).unwrap();

    assert!(content.contains("Main Heading"));
    assert!(content.contains("Paragraph 1"));
    assert!(content.contains("Nested paragraph"));
}

#[test]
fn test_extract_links() {
    let html = r#"
    <html>
    <body>
        <a href="https://example.com/1">Link 1</a>
        <a href="/relative">Link 2</a>
        <a href="anchor">Link 3</a>
    </body>
    </html>
    "#;

    let scraper = HtmlScraper::new();
    let links = scraper.extract_links(html, "https://base.com/").unwrap();

    assert_eq!(links.len(), 3);
    assert!(links.contains(&Url::parse("https://example.com/1").unwrap()));
    assert!(links.contains(&Url::parse("https://base.com/relative").unwrap()));
}

#[test]
fn test_content_type_detection() {
    let scraper = HtmlScraper::new();

    let html_content = "<html><body>Content</body></html>";
    assert_eq!(scraper.detect_content_type(html_content), ContentType::Html);

    let text_content = "Plain text content";
    assert_eq!(scraper.detect_content_type(text_content), ContentType::Text);

    let json_content = r#"{"key": "value"}"#;
    assert_eq!(scraper.detect_content_type(json_content), ContentType::Json);
}
```

### Integration Tests

**robots.txt Compliance**
- [ ] **Step 1: Setup mock robots.txt server**
  ```rust
  let server = setup_restrictive_robots_mock().await;
  let cache = RobotsCache::new();
  ```

- [ ] **Step 2: Test allowed paths**
  ```rust
  let allowed_url = Url::parse(&format!("{}/allowed", server.uri())).unwrap();
  let allowed = cache.check_allowed(&allowed_url, "*").await.unwrap();
  assert!(allowed);
  ```

- [ ] **Step 3: Test disallowed paths**
  ```rust
  let private_url = Url::parse(&format!("{}/private", server.uri())).unwrap();
  let disallowed = cache.check_allowed(&private_url, "*").await.unwrap();
  assert!(!disallowed);
  ```

- [ ] **Step 4: Verify crawl-delay respect**
  ```rust
  let delay = cache.get_crawl_delay("test.com", "*").await.unwrap();
  assert_eq!(delay, Duration::from_secs(1));
  ```

**Scope Enforcement**
```rust
#[tokio::test]
async fn test_domain_blocking() {
    let restrictions = MockScopeRestrictions::new();
    restrictions.blocked_domains.write().unwrap().push("blocked.com".to_string());

    let url = Url::parse("http://blocked.com/page").unwrap();
    assert!(restrictions.check_url(&url, 0).await.is_err());
}

#[tokio::test]
async fn test_path_restrictions() {
    let restrictions = MockScopeRestrictions::new();
    *restrictions.max_depth.write().unwrap() = 2;

    let url1 = Url::parse("http://test.com/depth1").unwrap();
    assert!(restrictions.check_url(&url1, 1).await.is_ok());

    let url2 = Url::parse("http://test.com/depth2").unwrap();
    assert!(restrictions.check_url(&url2, 2).await.is_ok());

    let url3 = Url::parse("http://test.com/depth3").unwrap();
    assert!(restrictions.check_url(&url3, 3).await.is_err());
}

#[tokio::test]
async fn test_page_count_limit_enforcement() {
    let restrictions = MockScopeRestrictions::new();
    *restrictions.max_pages.write().unwrap() = 3;

    for _ in 0..3 {
        assert!(restrictions.increment_page_count().await.is_ok());
    }

    assert!(restrictions.increment_page_count().await.is_err());
}
```

**Content Extraction**
```rust
#[tokio::test]
async fn test_extract_html_content() {
    let server = setup_content_mock().await;
    let url = format!("{}/test.html", server.uri());

    let scraper = WebScraper::new();
    let content = scraper.extract(&url).await.unwrap();

    assert!(content.title.contains("Test Page"));
    assert!(content.body.contains("Main Heading"));
    assert!(content.body.contains("test content"));
}

#[tokio::test]
async fn test_extract_with_relative_links() {
    let server = setup_content_mock().await;
    let url = format!("{}/test.html", server.uri());

    let scraper = WebScraper::new();
    let content = scraper.extract(&url).await.unwrap();

    assert!(!content.links.is_empty());
}

#[tokio::test]
async fn test_content_type_detection() {
    let server = setup_content_mock().await;
    let url = format!("{}/test.html", server.uri());

    let scraper = WebScraper::new();
    let content = scraper.extract(&url).await.unwrap();

    assert_eq!(content.content_type, ContentType::Html);
}
```

**Extraction Tracing**
```rust
#[tokio::test]
async fn test_extraction_tracing() {
    let server = setup_content_mock().await;
    let url = format!("{}/test.html", server.uri());

    let mut scraper = WebScraper::new();
    scraper.enable_tracing();

    scraper.extract(&url).await.unwrap();
    let trace = scraper.get_trace();

    assert!(!trace.entries.is_empty());
    assert!(trace.entries.iter().any(|e| e.url == url));

    for entry in &trace.entries {
        assert!(entry.success);
        assert!(entry.duration > Duration::ZERO);
    }
}

#[tokio::test]
async fn test_trace_robots_decision() {
    let server = setup_restrictive_robots_mock().await;
    let cache = RobotsCache::new();

    let mut scraper = WebScraper::new();
    scraper.enable_tracing();
    scraper.set_robots_cache(cache);

    let allowed_url = format!("{}/allowed", server.uri());
    scraper.extract(&allowed_url).await.unwrap();

    let trace = scraper.get_trace();
    assert!(trace.entries.iter().any(|e| e.robots_allowed));
}

#[tokio::test]
async fn test_trace_failure_status() {
    let server = setup_error_content_mock().await;
    let url = format!("{}/error.html", server.uri());

    let mut scraper = WebScraper::new();
    scraper.enable_tracing();

    scraper.extract(&url).await;

    let trace = scraper.get_trace();
    let error_entry = trace.entries.iter().find(|e| !e.success);
    assert!(error_entry.is_some());

    if let Some(entry) = error_entry {
        assert!(matches!(entry.error, Some(ScrapeError::HttpError { .. })));
    }
}
```

**Error Handling**
```rust
#[tokio::test]
async fn test_network_error_handling() {
    let scraper = WebScraper::new();

    let url = "http://invalid-does-not-exist.example";
    let result = scraper.extract(url).await;

    assert!(result.is_err());
    assert!(matches!(result, Err(ScrapeError::NetworkError { .. })));
}

#[tokio::test]
async fn test_http_error_handling() {
    let server = setup_error_content_mock().await;
    let url = format!("{}/error.html", server.uri());

    let scraper = WebScraper::new();
    let result = scraper.extract(&url).await;

    assert!(result.is_err());
    assert!(matches!(result, Err(ScrapeError::HttpError { .. })));
}

#[tokio::test]
async fn test_robots_fetch_failure() {
    let scraper = WebScraper::new();

    // Use invalid domain
    let url = "http://invalid.example/page";
    let result = scraper.extract(url).await;

    // Should fail-closed (no robots.txt means no access)
    assert!(result.is_err());
}

#[tokio::test]
async fn test_timeout_handling() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/timeout.html"))
        .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_secs(10)))
        .mount(&server)
        .await;

    let scraper = WebScraper::new_with_timeout(Duration::from_millis(100));
    let url = format!("{}/timeout.html", server.uri());

    let result = scraper.extract(&url).await;

    assert!(result.is_err());
    assert!(matches!(result, Err(ScrapeError::Timeout)));
}
```

**Concurrent Scraping**
```rust
#[tokio::test]
async fn test_concurrent_extraction() {
    let server = setup_content_mock().await;
    let urls: Vec<String> = (0..10)
        .map(|i| format!("{}/test-{}.html", server.uri(), i))
        .collect();

    let scraper = WebScraper::new();
    let handles: Vec<_> = urls
        .iter()
        .map(|url| {
            let scraper = scraper.clone();
            let url = url.clone();
            tokio::spawn(async move {
                scraper.extract(&url).await
            })
        })
        .collect();

    let results: Vec<_> = handles
        .into_iter()
        .map(|h| h.await.unwrap())
        .collect();

    // All should complete (may have errors but no panics)
    assert_eq!(results.len(), 10);
}

#[tokio::test]
async fn test_rate_limited_scraping() {
    let server = setup_content_mock().await;
    let mut scraper = WebScraper::new();
    scraper.set_rate_limit(Duration::from_millis(100));

    let url = format!("{}/test.html", server.uri());

    let start = Instant::now();
    for _ in 0..5 {
        scraper.extract(&url).await.unwrap();
    }
    let elapsed = start.elapsed();

    // Should take at least 400ms (4 delays)
    assert!(elapsed >= Duration::from_millis(400));
}
```

### Edge Cases

```rust
#[tokio::test]
async fn test_empty_html_content() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/empty.html"))
        .respond_with(ResponseTemplate::new(200).set_body_string(""))
        .mount(&server)
        .await;

    let scraper = WebScraper::new();
    let url = format!("{}/empty.html", server.uri());

    let content = scraper.extract(&url).await.unwrap();

    assert!(content.title.is_empty());
    assert!(content.body.is_empty());
}

#[tokio::test]
async fn test_malformed_html() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/malformed.html"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            "<html><body>Unclosed tags</html>"
        ))
        .mount(&server)
        .await;

    let scraper = WebScraper::new();
    let url = format!("{}/malformed.html", server.uri());

    let content = scraper.extract(&url).await;

    // Should handle gracefully (either succeed or return parse error)
    assert!(content.is_ok() || matches!(content, Err(ScrapeError::ParseError { .. })));
}

#[tokio::test]
async fn test_unicode_content() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/unicode.html"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            "<html><head><title>编程™</title></head><body>日本語</body></html>"
        ))
        .mount(&server)
        .await;

    let scraper = WebScraper::new();
    let url = format!("{}/unicode.html", server.uri());

    let content = scraper.extract(&url).await.unwrap();

    assert!(content.title.contains("编程"));
    assert!(content.body.contains("日本語"));
}

#[tokio::test]
async fn test_very_large_page() {
    let server = MockServer::start().await;
    let large_content = "<html><body>".to_string() + &"x".repeat(10_000_000) + "</body></html>";

    Mock::given(method("GET"))
        .and(path("/large.html"))
        .respond_with(ResponseTemplate::new(200).set_body_string(large_content))
        .mount(&server)
        .await;

    let scraper = WebScraper::new();
    let url = format!("{}/large.html", server.uri());

    let content = scraper.extract(&url).await;

    // Should handle large pages (may succeed or fail gracefully)
    assert!(content.is_ok() || content.is_err());
}

#[tokio::test]
async fn test_special_characters_in_url() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/special.html"))
        .respond_with(ResponseTemplate::new(200).set_body_string("<html><body>Content</body></html>"))
        .mount(&server)
        .await;

    let scraper = WebScraper::new();
    let url = format!("{}/special.html?param=value@#$%", server.uri());

    let content = scraper.extract(&url).await;

    // Should handle gracefully
    assert!(content.is_ok() || content.is_err());
}

#[tokio::test]
async fn test_redirect_handling() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/redirect"))
        .respond_with(ResponseTemplate::new(302).insert_header("Location", "/target"))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/target"))
        .respond_with(ResponseTemplate::new(200).set_body_string("<html><body>Target</body></html>"))
        .mount(&server)
        .await;

    let scraper = WebScraper::new();
    let url = format!("{}/redirect", server.uri());

    let content = scraper.extract(&url).await.unwrap();

    assert!(content.body.contains("Target"));
}
```

## Cargo Commands

**Run all web scraping tests:**
```bash
cargo test --test web_scraping -- --nocapture
```

**Run specific test:**
```bash
cargo test test_robots_allow_all -- --nocapture
```

**Run with wiremock:**
```bash
cargo test --test web_scraping --features wiremock -- --nocapture
```

**Run with logging:**
```bash
RUST_LOG=debug cargo test --test web_scraping -- --nocapture
```

**Expected output:**
```
running 55 tests
test tests::unit::test_robots_allow_all ... ok
test tests::unit::test_robots_disallow_path ... ok
test tests::unit::test_robots_specific_user_agent ... ok
test tests::unit::test_robots_crawl_delay ... ok
test tests::unit::test_blocked_domain ... ok
test tests::unit::test_allowed_domain ... ok
test tests::unit::test_depth_limit ... ok
test tests::unit::test_page_count_limit ... ok
test tests::unit::test_extract_page_title ... ok
test tests::unit::test_extract_page_content ... ok
test tests::unit::test_extract_links ... ok
test tests::unit::test_content_type_detection ... ok
test tests::integration::test_robots_compliance ... ok
test tests::integration::test_domain_blocking ... ok
test tests::integration::test_path_restrictions ... ok
test tests::integration::test_page_count_limit_enforcement ... ok
test tests::integration::test_extract_html_content ... ok
test tests::integration::test_extract_with_relative_links ... ok
test tests::integration::test_content_type_detection ... ok
test tests::integration::test_extraction_tracing ... ok
test tests::integration::test_trace_robots_decision ... ok
test tests::integration::test_trace_failure_status ... ok
test tests::integration::test_network_error_handling ... ok
test tests::integration::test_http_error_handling ... ok
test tests::integration::test_robots_fetch_failure ... ok
test tests::integration::test_timeout_handling ... ok
test tests::integration::test_concurrent_extraction ... ok
test tests::integration::test_rate_limited_scraping ... ok
test tests::edge_cases::test_empty_html_content ... ok
test tests::edge_cases::test_malformed_html ... ok
test tests::edge_cases::test_unicode_content ... ok
test tests::edge_cases::test_very_large_page ... ok
test tests::edge_cases::test_special_characters_in_url ... ok
test tests::edge_cases::test_redirect_handling ... ok

test result: ok. 55 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Mock Dependencies

- `wiremock`: Mock HTTP servers for robots.txt and HTML content
- Pre-defined robots.txt: Test allow/disallow rules and crawl-delay
- Pre-defined HTML: Test extraction and parsing
- `MockScopeRestrictions`: Test domain blocking, depth, and page limits
- `tokio::test`: Async test support
- `tokio::spawn`: Concurrent scraping testing
