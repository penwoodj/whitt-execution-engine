# Task 05: Web Scraping

**Estimated Time:** 1-2 weeks
**Dependencies:** Task 04 complete
**Priority:** CRITICAL (ADR-0006 requires robots.txt enforcement)

## Overview

Implement web scraping with robots.txt parser/enforcement, scope restrictions (domain/path/rate), and extraction traces. **CRITICAL:** MUST respect robots.txt per ADR-0006.

## Files

### Create
- `crates/scraping/Cargo.toml` - Scraping crate manifest
- `crates/scraping/src/lib.rs` - Public API exports
- `crates/scraping/src/robots.rs` - robots.txt parser
- `crates/scraping/src/scope.rs` - Scope restrictions
- `crates/scraping/src/extractor.rs` - Content extraction
- `crates/scraping/src/trace.rs` - Extraction traces

### Modify
- `Cargo.toml` - Add scraping workspace member

### Test
- `crates/scraping/tests/integration_test.rs` - Integration tests with mock robots.txt server

---

## Step-by-Step Implementation

### Step 1: Create scraping crate structure

```bash
mkdir -p crates/scraping/src

cat > crates/scraping/Cargo.toml << 'EOF'
[package]
name = "agentsdk-scraping"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
tracing = "0.1"
reqwest = { version = "0.11", features = ["json"] }
url = "2.5"
chrono = { version = "0.4", features = ["serde"] }
robotstxt = "0.4"
governor = "0.6"
EOF
```

Run: `cargo check --package agentsdk-scraping`
Expected: SUCCESS

- [ ] **Step 1: Create scraping crate structure**

### Step 2: Write error types

Create `crates/scraping/src/error.rs`:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScrapingError {
    #[error("robots.txt disallows access: {0}")]
    RobotsDisallowed(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Scope restriction violated: {0}")]
    ScopeViolation(String),

    #[error("Extraction error: {0}")]
    ExtractionError(String),

    #[error("Content not found")]
    NotFound,

    #[error("Invalid robots.txt: {0}")]
    InvalidRobotsTxt(String),
}
```

Run: `cargo check --package agentsdk-scraping`
Expected: SUCCESS

- [ ] **Step 2: Write error types**

### Step 3: Write robots.txt parser

Create `crates/scraping/src/robots.rs`:

```rust
use super::error::ScrapingError;
use robotstxt::Robotstxt;
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct RobotsInfo {
    pub allowed: bool,
    pub crawl_delay: Option<u64>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

pub struct RobotsCache {
    client: Client,
    cache: Arc<RwLock<HashMap<String, RobotsInfo>>>,
    cache_ttl: chrono::Duration,
}

impl RobotsCache {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_ttl: chrono::Duration::hours(24), // Cache for 24 hours
        }
    }

    pub async fn check_allowed(
        &self,
        url: &url::Url,
        user_agent: &str,
    ) -> Result<bool, ScrapingError> {
        let domain = url.host_str()
            .ok_or_else(|| ScrapingError::InvalidUrl("No domain found".to_string()))?;

        // Check cache
        {
            let cache = self.cache.read().await;
            if let Some(info) = cache.get(domain) {
                let cache_age = chrono::Utc::now() - info.last_updated;
                if cache_age < self.cache_ttl {
                    return Ok(info.allowed);
                }
            }
        }

        // Fetch and parse robots.txt
        let robots_url = format!("{}/robots.txt",
            url[..url::Position::BeforePath].as_str()
        );

        let response = self.client.get(&robots_url).send().await;
        let (allowed, crawl_delay) = match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    let robots_txt = resp.text().await?;
                    let robots = Robotstxt::parse(&robots_txt);

                    let allowed = robots
                        .allowed(user_agent, url.path());

                    let crawl_delay = robots
                        .crawl_delay(user_agent)
                        .map(|x| x.as_millis() as u64);

                    (allowed, crawl_delay)
                } else if resp.status() == 404 {
                    // No robots.txt means allow all
                    (true, None)
                } else {
                    return Err(ScrapingError::InvalidRobotsTxt(
                        format!("Failed to fetch robots.txt: {}", resp.status())
                    ));
                }
            }
            Err(e) => {
                // On network error, be conservative and deny
                tracing::warn!("Failed to fetch robots.txt for {}: {}", domain, e);
                return Err(ScrapingError::InvalidRobotsTxt(
                    format!("Network error: {}", e)
                ));
            }
        };

        // Update cache
        let info = RobotsInfo {
            allowed,
            crawl_delay,
            last_updated: chrono::Utc::now(),
        };

        {
            let mut cache = self.cache.write().await;
            cache.insert(domain.to_string(), info);
        }

        Ok(allowed)
    }

    pub async fn get_crawl_delay(
        &self,
        url: &url::Url,
    ) -> Option<u64> {
        let domain = url.host_str()?;
        let cache = self.cache.read().await;
        cache.get(domain)?.crawl_delay
    }

    pub async fn cleanup_expired(&self) {
        let mut cache = self.cache.write().await;
        cache.retain(|_, info| {
            let cache_age = chrono::Utc::now() - info.last_updated;
            cache_age < self.cache_ttl
        });
    }
}

impl Default for RobotsCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_robots_cache_creation() {
        let cache = RobotsCache::new();
        assert_eq!(cache.cache_ttl, chrono::Duration::hours(24));
    }
}
```

Run: `cargo check --package agentsdk-scraping`
Expected: SUCCESS

- [ ] **Step 3: Write robots.txt parser**

### Step 4: Write scope restrictions

Create `crates/scraping/src/scope.rs`:

```rust
use super::error::ScrapingError;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct ScopeConfig {
    pub allowed_domains: HashSet<String>,
    pub blocked_domains: HashSet<String>,
    pub allowed_path_patterns: Vec<String>,
    pub blocked_path_patterns: Vec<String>,
    pub max_depth: Option<usize>,
    pub max_pages_per_domain: Option<usize>,
}

impl Default for ScopeConfig {
    fn default() -> Self {
        Self {
            allowed_domains: HashSet::new(),
            blocked_domains: HashSet::new(),
            allowed_path_patterns: vec![],
            blocked_path_patterns: vec![],
            max_depth: Some(3),
            max_pages_per_domain: Some(100),
        }
    }
}

pub struct ScopeRestrictions {
    config: Arc<RwLock<ScopeConfig>>,
    page_counts: Arc<RwLock<HashMap<String, usize>>>,
}

impl ScopeRestrictions {
    pub fn new(config: ScopeConfig) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
            page_counts: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn check_url(
        &self,
        url: &url::Url,
        depth: usize,
    ) -> Result<(), ScrapingError> {
        let config = self.config.read().await;

        // Check domain
        let domain = url.host_str()
            .ok_or_else(|| ScrapingError::InvalidUrl("No domain found".to_string()))?;

        if !config.allowed_domains.is_empty() {
            if !config.allowed_domains.contains(domain) {
                return Err(ScrapingError::ScopeViolation(
                    format!("Domain '{}' not in allowed list", domain)
                ));
            }
        }

        if config.blocked_domains.contains(domain) {
            return Err(ScrapingError::ScopeViolation(
                format!("Domain '{}' is blocked", domain)
            ));
        }

        // Check depth
        if let Some(max_depth) = config.max_depth {
            if depth > max_depth {
                return Err(ScrapingError::ScopeViolation(
                    format!("Depth {} exceeds max depth {}", depth, max_depth)
                ));
            }
        }

        // Check page count
        if let Some(max_pages) = config.max_pages_per_domain {
            let mut page_counts = self.page_counts.write().await;
            let count = page_counts.entry(domain.to_string()).or_insert(0);

            if *count >= max_pages {
                return Err(ScrapingError::ScopeViolation(
                    format!("Page count {} exceeds max pages {} for domain '{}'",
                        count, max_pages, domain)
                ));
            }

            *count += 1;
        }

        Ok(())
    }

    pub async fn reset_page_count(&self, domain: &str) {
        let mut page_counts = self.page_counts.write().await;
        page_counts.remove(domain);
    }

    pub async fn update_config(&self, config: ScopeConfig) {
        let mut cfg = self.config.write().await;
        *cfg = config;
    }
}

impl Default for ScopeRestrictions {
    fn default() -> Self {
        Self::new(ScopeConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope_config_default() {
        let config = ScopeConfig::default();
        assert_eq!(config.max_depth, Some(3));
        assert_eq!(config.max_pages_per_domain, Some(100));
    }
}
```

Run: `cargo check --package agentsdk-scraping`
Expected: SUCCESS

- [ ] **Step 4: Write scope restrictions**

### Step 5: Write content extractor

Create `crates/scraping/src/extractor.rs`:

```rust
use super::error::ScrapingError;
use super::robots::RobotsCache;
use super::scope::ScopeRestrictions;
use reqwest::Client;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone)]
pub struct ExtractedContent {
    pub url: String,
    pub title: Option<String>,
    pub content: String,
    pub content_type: String,
    pub extracted_at: chrono::DateTime<chrono::Utc>,
    pub content_length: usize,
}

pub struct ContentExtractor {
    client: Client,
    robots_cache: Arc<RobotsCache>,
    scope: Arc<ScopeRestrictions>,
    user_agent: String,
    default_delay: Duration,
}

impl ContentExtractor {
    pub fn new(
        robots_cache: Arc<RobotsCache>,
        scope: Arc<ScopeRestrictions>,
    ) -> Self {
        Self {
            client: Client::builder()
                .user_agent("AgentSDK-Scraper/1.0")
                .build()
                .unwrap(),
            robots_cache,
            scope,
            user_agent: "AgentSDK-Scraper/1.0".to_string(),
            default_delay: Duration::from_millis(1000),
        }
    }

    pub async fn extract(
        &self,
        url: &url::Url,
        depth: usize,
    ) -> Result<ExtractedContent, ScrapingError> {
        // Check robots.txt
        let allowed = self.robots_cache
            .check_allowed(url, &self.user_agent)
            .await?;

        if !allowed {
            return Err(ScrapingError::RobotsDisallowed(
                format!("robots.txt disallows access to {}", url.as_str())
            ));
        }

        // Check scope
        self.scope.check_url(url, depth).await?;

        // Respect crawl delay
        if let Some(delay_ms) = self.robots_cache.get_crawl_delay(url).await {
            sleep(Duration::from_millis(delay_ms)).await;
        } else {
            sleep(self.default_delay).await;
        }

        // Fetch content
        let response = self.client.get(url.as_str()).send().await?;

        if !response.status().is_success() {
            return Err(ScrapingError::NotFound);
        }

        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("text/html")
            .to_string();

        let content = response.text().await?;

        // Extract title (for HTML)
        let title = if content_type.contains("html") {
            self.extract_title_from_html(&content)
        } else {
            None
        };

        Ok(ExtractedContent {
            url: url.as_str().to_string(),
            title,
            content,
            content_type,
            extracted_at: chrono::Utc::now(),
            content_length: content.len(),
        })
    }

    fn extract_title_from_html(&self, html: &str) -> Option<String> {
        let title_regex = regex::Regex::new(r"<title>([^<]+)</title>").ok()?;
        title_regex.captures(html)?.get(1).map(|m| m.as_str().to_string())
    }

    pub fn with_user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = user_agent;
        self
    }

    pub fn with_default_delay(mut self, delay: Duration) -> Self {
        self.default_delay = delay;
        self
    }
}

// Add regex dependency to Cargo.toml in Step 7
```

Run: `cargo check --package agentsdk-scraping`
Expected: ERROR (regex dependency missing)

- [ ] **Step 5: Write content extractor**

### Step 6: Write extraction traces

Create `crates/scraping/src/trace.rs`:

```rust
use super::extractor::ExtractedContent;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionTrace {
    pub url: String,
    pub depth: usize,
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
    pub success: bool,
    pub error_message: Option<String>,
    pub robots_allowed: bool,
    pub content_length: Option<usize>,
}

pub struct TraceLogger {
    traces: Vec<ExtractionTrace>,
}

impl TraceLogger {
    pub fn new() -> Self {
        Self {
            traces: Vec::new(),
        }
    }

    pub fn start_extraction(&mut self, url: String, depth: usize) -> usize {
        let trace = ExtractionTrace {
            url,
            depth,
            started_at: Utc::now(),
            completed_at: Utc::now(), // Will be updated
            success: false,
            error_message: None,
            robots_allowed: false,
            content_length: None,
        };

        self.traces.push(trace);
        self.traces.len() - 1
    }

    pub fn complete_extraction(
        &mut self,
        trace_id: usize,
        success: bool,
        error_message: Option<String>,
        robots_allowed: bool,
        content: Option<&ExtractedContent>,
    ) {
        if let Some(trace) = self.traces.get_mut(trace_id) {
            trace.completed_at = Utc::now();
            trace.success = success;
            trace.error_message = error_message;
            trace.robots_allowed = robots_allowed;
            trace.content_length = content.map(|c| c.content_length);
        }
    }

    pub fn get_traces(&self) -> &[ExtractionTrace] {
        &self.traces
    }

    pub fn get_successful_count(&self) -> usize {
        self.traces.iter().filter(|t| t.success).count()
    }

    pub fn get_failed_count(&self) -> usize {
        self.traces.iter().filter(|t| !t.success).count()
    }
}

impl Default for TraceLogger {
    fn default() -> Self {
        Self::new()
    }
}
```

Run: `cargo check --package agentsdk-scraping`
Expected: SUCCESS

- [ ] **Step 6: Write extraction traces**

### Step 7: Update Cargo.toml and write lib.rs

Modify `crates/scraping/Cargo.toml` to add regex dependency:

```toml
[dependencies]
# ... existing dependencies ...
regex = "1.10"
```

Create `crates/scraping/src/lib.rs`:

```rust
pub mod error;
pub mod robots;
pub mod scope;
pub mod extractor;
pub mod trace;

pub use error::ScrapingError;
pub use robots::{RobotsCache, RobotsInfo};
pub use scope::{ScopeConfig, ScopeRestrictions};
pub use extractor::{ContentExtractor, ExtractedContent};
pub use trace::{ExtractionTrace, TraceLogger};
```

Run: `cargo check --package agentsdk-scraping`
Expected: SUCCESS

- [ ] **Step 7: Update Cargo.toml and write lib.rs**

### Step 8: Add to workspace

Modify `Cargo.toml`:

Add to `[workspace.members]`:

```toml
members = [
    # ... existing members ...
    "crates/scraping",
]
```

Run: `cargo check --workspace`
Expected: SUCCESS

- [ ] **Step 8: Add to workspace**

### Step 9: Write integration tests

Create `crates/scraping/tests/integration_test.rs`:

```rust
use agentsdk_scraping::{RobotsCache, ScopeConfig, ScopeRestrictions, ContentExtractor, TraceLogger};
use std::sync::Arc;

#[tokio::test]
async fn test_robots_cache() {
    let cache = RobotsCache::new();

    // Test with a known URL
    let url = url::Url::parse("https://example.com/").unwrap();
    let allowed = cache.check_allowed(&url, "*").await.unwrap();

    // example.com allows all crawlers
    assert!(allowed);
}

#[tokio::test]
async fn test_scope_restrictions() {
    let config = ScopeConfig {
        allowed_domains: vec!["example.com".to_string()].into_iter().collect(),
        ..Default::default()
    };

    let scope = ScopeRestrictions::new(config);

    let allowed_url = url::Url::parse("https://example.com/page").unwrap();
    assert!(scope.check_url(&allowed_url, 0).await.is_ok());

    let blocked_url = url::Url::parse("https://blocked.com/page").unwrap();
    assert!(scope.check_url(&blocked_url, 0).await.is_err());
}

#[tokio::test]
async fn test_trace_logger() {
    let mut logger = TraceLogger::new();

    let trace_id = logger.start_extraction("https://example.com".to_string(), 0);

    logger.complete_extraction(
        trace_id,
        true,
        None,
        true,
        None,
    );

    let traces = logger.get_traces();
    assert_eq!(traces.len(), 1);
    assert_eq!(logger.get_successful_count(), 1);
    assert_eq!(logger.get_failed_count(), 0);
}
```

Run: `cargo test --package agentsdk-scraping --test integration_test`
Expected: All tests PASS

- [ ] **Step 9: Write integration tests**

### Step 10: Commit

```bash
git add crates/scraping/ Cargo.toml
git commit -m "feat(Phase5-Task05): implement web scraping with robots.txt parser/enforcement, scope restrictions, and extraction traces (ADR-0006 compliant)"
```

- [ ] **Step 10: Commit**

---

## ADR-0006 Critical Requirements

**MUST ENFORCE:**

1. **robots.txt Compliance:** MUST parse and respect robots.txt for all requests
2. **Scope Restrictions:** MUST enforce domain/path/depth/rate limits
3. **Crawl Delay:** MUST respect crawl-delay directive from robots.txt
4. **Extraction Traces:** MUST log all extraction attempts
5. **Fail Closed:** On robots.txt fetch failure, deny access

**FORBIDDEN:**

- ❌ Accessing URLs disallowed by robots.txt
- ❌ Ignoring crawl-delay directives
- ❌ Exceeding scope restrictions
- ❌ Requesting content without trace logging
- ❌ Network failures bypassing robots.txt checks

---

## Validation Criteria

See [validation/05-web-scraping.md](../validation/05-web-scraping.md)

## Test Specifications

See [tests/05-web-scraping.md](../tests/05-web-scraping.md)


---

## QA Cross-References

### QA Criteria
- **QA Area**: Area 6 - Web Scraping
- **QA Criteria**: [../../qa/phase-04/QA-CRITERIA.md#area-6-web-scraping](../../qa/phase-04/QA-CRITERIA.md#area-6-web-scraping)
- **Priority**: P1
- **Test Types**: Unit, Integration

### Test Cases
- **Test Cases**: [../../qa/phase-04/QA-TEST-CASES.md](../../qa/phase-04/QA-TEST-CASES.md)
- **Key Tests**:
  - P04-025: HTML parsing
  - P04-026: Rate limiting per-domain
  - P04-027: Robots.txt compliance
  - P04-028: Content extraction
  - P04-029: Request timeout
  - P04-030: Cache invalidation

### Schema References
- **Schema File**: [../../../schema/unified-workflow-schema.yml](../../../schema/unified-workflow-schema.yml)
- **Schema Section**: Lines 623-649 (web_operations.fetch/scrape: timeouts, policies, caching)
- **Key Fields**:
  - `web_operations.fetch.timeout_seconds` (line 630)
  -   - `web_operations.fetch.respect_robots_txt` (line 632)
  -   - `web_operations.scrape.parse_html` (line 635)

### Related Documentation
- **Cross-References**: [../../qa/phase-04/CROSS-REF.md](../../qa/phase-04/CROSS-REF.md)
- **Phase Plan**: [../plan.md](../plan.md)
