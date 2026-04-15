# Task 04: External Search Adapters

**Estimated Time:** 1-2 weeks
**Dependencies:** Task 03 complete
**Priority:** CRITICAL (ADR-0006 compliance - external search ONLY after local)

## Overview

Implement external search adapters for DuckDuckGo and Brave with explicit policy gates, rate limiting, and caching. **CRITICAL:** External search MUST only be invoked after local memory is exhausted per ADR-0006.

## Files

### Create
- `crates/external/Cargo.toml` - External search crate manifest
- `crates/external/src/lib.rs` - Public API exports
- `crates/external/src/policy.rs` - Policy gate enforcement
- `crates/external/src/duckduckgo.rs` - DuckDuckGo adapter
- `crates/external/src/brave.rs` - Brave adapter
- `crates/external/src/rate_limit.rs` - Rate limiting
- `crates/external/src/models.rs` - Search result models

### Modify
- `Cargo.toml` - Add external workspace member
- `crates/search/src/engine.rs` - Integrate external search as fallback

### Test
- `crates/external/tests/integration_test.rs` - Integration tests with mock HTTP

---

## Step-by-Step Implementation

### Step 1: Create external search crate structure

```bash
mkdir -p crates/external/src

cat > crates/external/Cargo.toml << 'EOF'
[package]
name = "agentsdk-external"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
tracing = "0.1"
reqwest = { version = "0.11", features = ["json"] }
chrono = { version = "0.4", features = ["serde"] }
url = "2.5"
governor = "0.6"
EOF
```

Run: `cargo check --package agentsdk-external`
Expected: SUCCESS

- [ ] **Step 1: Create external search crate structure**

### Step 2: Write error types

Create `crates/external/src/error.rs`:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExternalSearchError {
    #[error("Policy gate denied: {0}")]
    PolicyDenied(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("JSON parse error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("No search results found")]
    NoResults,
}
```

Run: `cargo check --package agentsdk-external`
Expected: SUCCESS

- [ ] **Step 2: Write error types**

### Step 3: Write policy gates

Create `crates/external/src/policy.rs`:

```rust
use super::error::ExternalSearchError;
use std::collections::HashSet;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub enum PolicyDecision {
    Allow,
    Deny(String),
}

#[derive(Debug, Clone)]
pub struct PolicyGate {
    allowed_domains: Arc<RwLock<HashSet<String>>>,
    blocked_domains: Arc<RwLock<HashSet<String>>>,
    require_approval: bool,
    approval_callback: Option<ApprovalCallback>,
}

type ApprovalCallback = Arc<dyn Fn(&str, &str) -> bool + Send + Sync>;

impl PolicyGate {
    pub fn new(require_approval: bool) -> Self {
        Self {
            allowed_domains: Arc::new(RwLock::new(HashSet::new())),
            blocked_domains: Arc::new(RwLock::new(HashSet::new())),
            require_approval,
            approval_callback: None,
        }
    }

    pub fn with_approval_callback(mut self, callback: ApprovalCallback) -> Self {
        self.approval_callback = Some(callback);
        self
    }

    pub fn add_allowed_domain(&self, domain: String) {
        let mut allowed = self.allowed_domains.write().unwrap();
        allowed.insert(domain);
    }

    pub fn add_blocked_domain(&self, domain: String) {
        let mut blocked = self.blocked_domains.write().unwrap();
        blocked.insert(domain);
    }

    pub async fn check_search(
        &self,
        query: &str,
        provider: &str,
    ) -> Result<PolicyDecision, ExternalSearchError> {
        // Check blocked domains (if URL contains domain)
        let blocked = self.blocked_domains.read().unwrap();
        for domain in blocked.iter() {
            if query.to_lowercase().contains(&domain.to_lowercase()) {
                return Ok(PolicyDecision::Deny(format!(
                    "Domain '{}' is blocked by policy", domain
                )));
            }
        }

        // Check approval requirement
        if self.require_approval {
            if let Some(callback) = &self.approval_callback {
                let approved = callback(query, provider);
                if !approved {
                    return Ok(PolicyDecision::Deny(
                        "User approval denied for external search".to_string()
                    ));
                }
            } else {
                return Ok(PolicyDecision::Deny(
                    "External search requires approval but no callback configured".to_string()
                ));
            }
        }

        Ok(PolicyDecision::Allow)
    }

    pub fn is_allowed_domain(&self, domain: &str) -> bool {
        let allowed = self.allowed_domains.read().unwrap();
        allowed.contains(domain) || allowed.is_empty()
    }
}

impl Default for PolicyGate {
    fn default() -> Self {
        Self::new(true) // Require approval by default for security
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_gate_blocked() {
        let gate = PolicyGate::new(false);
        gate.add_blocked_domain("malicious.com".to_string());

        let decision = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(gate.check_search("malicious.com content", "test"));

        assert!(matches!(decision, Ok(PolicyDecision::Deny(_))));
    }

    #[test]
    fn test_policy_gate_allowed() {
        let gate = PolicyGate::new(false);

        let decision = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(gate.check_search("test query", "duckduckgo"));

        assert!(matches!(decision, Ok(PolicyDecision::Allow)));
    }
}
```

Run: `cargo check --package agentsdk-external`
Expected: SUCCESS

- [ ] **Step 3: Write policy gates**

### Step 4: Write rate limiting

Create `crates/external/src/rate_limit.rs`:

```rust
use super::error::ExternalSearchError;
use governor::{
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter,
};
use std::num::NonZeroU32;

pub struct RateLimiterWrapper {
    limiter: RateLimiter<NotKeyed, InMemoryState, DefaultClock>,
}

impl RateLimiterWrapper {
    pub fn new(requests_per_second: u32) -> Self {
        let quota = Quota::per_second(NonZeroU32::new(requests_per_second).unwrap());
        let limiter = RateLimiter::direct(quota);

        Self { limiter }
    }

    pub async fn acquire(&self) -> Result<(), ExternalSearchError> {
        self.limiter.until_ready().await;
        Ok(())
    }

    pub fn check(&self) -> Result<(), ExternalSearchError> {
        self.limiter.check()
            .map_err(|_| ExternalSearchError::RateLimitExceeded(
                "Rate limit exceeded".to_string()
            ))
    }
}

impl Default for RateLimiterWrapper {
    fn default() -> Self {
        Self::new(5) // 5 requests per second by default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_creation() {
        let limiter = RateLimiterWrapper::new(10);
        assert!(limiter.check().is_ok());
    }
}
```

Run: `cargo check --package agentsdk-external`
Expected: SUCCESS

- [ ] **Step 4: Write rate limiting**

### Step 5: Write search result models

Create `crates/external/src/models.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
    pub provider: String,
    pub score: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
    pub total_results: Option<usize>,
    pub query: String,
    pub provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalSearchRequest {
    pub query: String,
    pub limit: usize,
    pub offset: usize,
    pub provider: String,
}
```

Run: `cargo check --package agentsdk-external`
Expected: SUCCESS

- [ ] **Step 5: Write search result models**

### Step 6: Write DuckDuckGo adapter

Create `crates/external/src/duckduckgo.rs`:

```rust
use super::error::ExternalSearchError;
use super::models::{SearchResult, SearchResponse, ExternalSearchRequest};
use super::policy::PolicyGate;
use super::rate_limit::RateLimiterWrapper;
use reqwest::Client;
use std::sync::Arc;

pub struct DuckDuckGoAdapter {
    client: Client,
    policy_gate: Arc<PolicyGate>,
    rate_limiter: RateLimiterWrapper,
    base_url: String,
}

impl DuckDuckGoAdapter {
    pub fn new(policy_gate: Arc<PolicyGate>) -> Self {
        Self {
            client: Client::new(),
            policy_gate,
            rate_limiter: RateLimiterWrapper::default(),
            base_url: "https://duckduckgo.com/html".to_string(),
        }
    }

    pub async fn search(
        &self,
        request: &ExternalSearchRequest,
    ) -> Result<SearchResponse, ExternalSearchError> {
        // Check policy gate
        let decision = self.policy_gate.check_search(&request.query, "duckduckgo").await?;
        if let PolicyDecision::Deny(reason) = decision {
            return Err(ExternalSearchError::PolicyDenied(reason));
        }

        // Check rate limit
        self.rate_limiter.acquire().await?;

        // Build request
        let url = format!(
            "{}?q={}&num={}",
            self.base_url,
            urlencoding::encode(&request.query),
            request.limit
        );

        // Execute request
        let response = self.client.get(&url).send().await?;

        // Parse HTML response (DuckDuckGo returns HTML)
        let html = response.text().await?;
        let results = self.parse_duckduckgo_html(&html)?;

        if results.is_empty() {
            return Err(ExternalSearchError::NoResults);
        }

        Ok(SearchResponse {
            results,
            total_results: Some(results.len()),
            query: request.query.clone(),
            provider: "duckduckgo".to_string(),
        })
    }

    fn parse_duckduckgo_html(&self, html: &str) -> Result<Vec<SearchResult>, ExternalSearchError> {
        // Parse DuckDuckGo HTML results
        // This is a simplified parser - in production, use a proper HTML parser
        let mut results = Vec::new();

        // Find result elements using regex (simplified)
        let result_regex = regex::Regex::new(r#"<a[^>]*class="result__a"[^>]*href="([^"]*)"[^>]*>([^<]*)</a>"#)?;
        let snippet_regex = regex::Regex::new(r#"<a[^>]*class="result__snippet"[^>]*>([^<]*)</a>"#)?;

        for caps in result_regex.captures_iter(html) {
            let url = caps.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
            let title = caps.get(2).map(|m| m.as_str()).unwrap_or("").to_string();

            // Extract snippet
            let snippet = if let Some(caps) = snippet_regex.captures(html) {
                caps.get(1).map(|m| m.as_str()).unwrap_or("").to_string()
            } else {
                String::new()
            };

            results.push(SearchResult {
                title,
                url,
                snippet,
                provider: "duckduckgo".to_string(),
                score: None,
            });
        }

        Ok(results)
    }
}

// Add regex dependency to Cargo.toml in Step 8
```

Run: `cargo check --package agentsdk-external`
Expected: ERROR (regex dependency missing)

- [ ] **Step 6: Write DuckDuckGo adapter**

### Step 7: Write Brave adapter

Create `crates/external/src/brave.rs`:

```rust
use super::error::ExternalSearchError;
use super::models::{SearchResult, SearchResponse, ExternalSearchRequest};
use super::policy::PolicyGate;
use super::rate_limit::RateLimiterWrapper;
use reqwest::Client;
use std::sync::Arc;

pub struct BraveAdapter {
    client: Client,
    policy_gate: Arc<PolicyGate>,
    rate_limiter: RateLimiterWrapper,
    api_key: Option<String>,
    base_url: String,
}

impl BraveAdapter {
    pub fn new(policy_gate: Arc<PolicyGate>, api_key: Option<String>) -> Self {
        Self {
            client: Client::new(),
            policy_gate,
            rate_limiter: RateLimiterWrapper::default(),
            api_key,
            base_url: "https://api.search.brave.com/res/v1/web/search".to_string(),
        }
    }

    pub async fn search(
        &self,
        request: &ExternalSearchRequest,
    ) -> Result<SearchResponse, ExternalSearchError> {
        // Check policy gate
        let decision = self.policy_gate.check_search(&request.query, "brave").await?;
        if let PolicyDecision::Deny(reason) = decision {
            return Err(ExternalSearchError::PolicyDenied(reason));
        }

        // Check rate limit
        self.rate_limiter.acquire().await?;

        // Build request
        let mut req = self.client
            .post(&self.base_url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "q": request.query,
                "count": request.limit,
                "offset": request.offset,
            }));

        // Add API key if provided
        if let Some(key) = &self.api_key {
            req = req.header("X-Subscription-Token", key);
        }

        // Execute request
        let response = req.send().await?;

        // Parse JSON response
        let json: serde_json::Value = response.json().await?;
        let results = self.parse_brave_response(&json)?;

        if results.is_empty() {
            return Err(ExternalSearchError::NoResults);
        }

        Ok(SearchResponse {
            results,
            total_results: json["web"]["total_results"]
                .as_u64()
                .map(|x| x as usize),
            query: request.query.clone(),
            provider: "brave".to_string(),
        })
    }

    fn parse_brave_response(&self, json: &serde_json::Value) -> Result<Vec<SearchResult>, ExternalSearchError> {
        let mut results = Vec::new();

        if let Some(web_results) = json["web"]["results"].as_array() {
            for item in web_results {
                if let (Some(title), Some(url), Some(description)) = (
                    item["title"].as_str(),
                    item["url"].as_str(),
                    item["description"].as_str(),
                ) {
                    results.push(SearchResult {
                        title: title.to_string(),
                        url: url.to_string(),
                        snippet: description.to_string(),
                        provider: "brave".to_string(),
                        score: item["score"].as_f64().map(|x| x as f32),
                    });
                }
            }
        }

        Ok(results)
    }
}
```

Run: `cargo check --package agentsdk-external`
Expected: SUCCESS

- [ ] **Step 7: Write Brave adapter**

### Step 8: Update Cargo.toml and write lib.rs

Modify `crates/external/Cargo.toml` to add regex dependency:

```toml
[dependencies]
# ... existing dependencies ...
regex = "1.10"
```

Create `crates/external/src/lib.rs`:

```rust
pub mod error;
pub mod policy;
pub mod rate_limit;
pub mod models;
pub mod duckduckgo;
pub mod brave;

pub use error::ExternalSearchError;
pub use policy::{PolicyGate, PolicyDecision};
pub use rate_limit::RateLimiterWrapper;
pub use models::{SearchResult, SearchResponse, ExternalSearchRequest};
pub use duckduckgo::DuckDuckGoAdapter;
pub use brave::BraveAdapter;
```

Run: `cargo check --package agentsdk-external`
Expected: SUCCESS

- [ ] **Step 8: Update Cargo.toml and write lib.rs**

### Step 9: Add to workspace

Modify `Cargo.toml`:

Add to `[workspace.members]`:

```toml
members = [
    # ... existing members ...
    "crates/external",
]
```

Run: `cargo check --workspace`
Expected: SUCCESS

- [ ] **Step 9: Add to workspace**

### Step 10: Write integration tests

Create `crates/external/tests/integration_test.rs`:

```rust
use agentsdk_external::{PolicyGate, PolicyDecision, DuckDuckGoAdapter, BraveAdapter};
use std::sync::Arc;

#[tokio::test]
async fn test_policy_gate_allow() {
    let gate = PolicyGate::new(false);

    let decision = gate.check_search("test query", "duckduckgo").await.unwrap();
    assert!(matches!(decision, PolicyDecision::Allow));
}

#[tokio::test]
async fn test_policy_gate_deny_blocked() {
    let gate = PolicyGate::new(false);
    gate.add_blocked_domain("blocked.com".to_string());

    let decision = gate.check_search("blocked.com content", "test").await.unwrap();
    assert!(matches!(decision, PolicyDecision::Deny(_)));
}

#[tokio::test]
async fn test_rate_limiter() {
    use agentsdk_external::RateLimiterWrapper;

    let limiter = RateLimiterWrapper::new(5);

    // Should allow first request
    assert!(limiter.check().is_ok());
}

// Note: Actual HTTP tests would require mock server or integration with real APIs
// These tests focus on policy and rate limiting
```

Run: `cargo test --package agentsdk-external --test integration_test`
Expected: All tests PASS

- [ ] **Step 10: Write integration tests**

### Step 11: Commit

```bash
git add crates/external/ Cargo.toml
git commit -m "feat(Phase5-Task04): implement external search adapters with policy gates, rate limiting, and DuckDuckGo/Brave support (ADR-0006 compliant: external only after local)"
```

- [ ] **Step 11: Commit**

---

## ADR-0006 Critical Requirements

**MUST ENFORCE:**

1. **Policy Gates:** All external search requests MUST pass through policy gate
2. **User Approval:** Default policy requires explicit user approval
3. **Rate Limiting:** MUST enforce rate limits to prevent API abuse
4. **Fallback Only:** External search MUST be secondary to local memory
5. **Explicit Consent:** Each external search requires explicit policy decision

**FORBIDDEN:**

- ❌ External search without policy gate check
- ❌ Bypassing rate limits
- ❌ Using external search before local memory exhausted
- ❌ Auto-approving external search without user consent

---

## Validation Criteria

See [validation/04-external-search-adapters.md](../validation/04-external-search-adapters.md)

## Test Specifications

See [tests/04-external-search-adapters.md](../tests/04-external-search-adapters.md)
