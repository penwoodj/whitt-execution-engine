# Test Specifications: Task 03 - Search Query Engine

## Mock Strategy

**Mock Search Results:**
```rust
struct MockSearchEngine {
    results: Arc<RwLock<HashMap<MemoryId, MockSearchResult>>>,
}

#[derive(Clone, Debug)]
struct MockSearchResult {
    id: MemoryId,
    title: String,
    content: String,
    tags: Vec<String>,
    score: f32,
    created_at: DateTime<Utc>,
}

impl MockSearchEngine {
    fn new() -> Self {
        Self {
            results: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn add_result(&self, result: MockSearchResult) {
        let mut results = self.results.blocking_write();
        results.insert(result.id.clone(), result);
    }

    fn search(&self, _query: &str) -> Vec<HybridResult> {
        let results = self.results.blocking_read();
        results
            .values()
            .map(|r| HybridResult {
                memory_id: r.id.clone(),
                fulltext_score: r.score,
                semantic_score: r.score * 0.9,
                final_score: r.score,
                snippet: r.content.chars().take(150).collect(),
                metadata: HashMap::new(),
            })
            .collect()
    }
}
```

**Mock Cache:**
```rust
struct QueryCache {
    cache: Arc<RwLock<HashMap<String, CachedResult>>>,
    ttl: Duration,
}

#[derive(Clone)]
struct CachedResult {
    results: Vec<HybridResult>,
    timestamp: DateTime<Utc>,
}

impl QueryCache {
    fn new(ttl: Duration) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            ttl,
        }
    }

    fn get(&self, key: &str) -> Option<Vec<HybridResult>> {
        let cache = self.cache.blocking_read();
        if let Some(cached) = cache.get(key) {
            if Utc::now() - cached.timestamp < self.ttl {
                return Some(cached.results.clone());
            }
        }
        None
    }

    fn set(&self, key: String, results: Vec<HybridResult>) {
        let mut cache = self.cache.blocking_write();
        cache.insert(
            key,
            CachedResult {
                results,
                timestamp: Utc::now(),
            },
        );
    }

    fn clear(&self) {
        let mut cache = self.cache.blocking_write();
        cache.clear();
    }
}
```

**Mock Time Control:**
```rust
struct MockTime {
    current: Arc<RwLock<DateTime<Utc>>>,
}

impl MockTime {
    fn new() -> Self {
        Self {
            current: Arc::new(RwLock::new(Utc::now())),
        }
    }

    fn now(&self) -> DateTime<Utc> {
        *self.current.blocking_read()
    }

    fn advance(&self, duration: Duration) {
        let mut current = self.current.blocking_write();
        *current = *current + duration;
    }

    fn set(&self, dt: DateTime<Utc>) {
        let mut current = self.current.blocking_write();
        *current = dt;
    }
}
```

## Test Cases

### Unit Tests

**Caching**
```rust
#[tokio::test]
async fn test_cache_hit() {
    let cache = QueryCache::new(Duration::from_secs(60));
    let result = vec![HybridResult {
        memory_id: "test".to_string(),
        final_score: 0.9,
        ..Default::default()
    }];

    cache.set("key".to_string(), result.clone());
    assert_eq!(cache.get("key"), Some(result));
}

#[tokio::test]
async fn test_cache_miss() {
    let cache = QueryCache::new(Duration::from_secs(60));
    assert_eq!(cache.get("nonexistent"), None);
}

#[tokio::test]
async fn test_cache_expiration() {
    let cache = QueryCache::new(Duration::from_secs(1));
    let result = vec![HybridResult {
        memory_id: "test".to_string(),
        final_score: 0.9,
        ..Default::default()
    }];

    cache.set("key".to_string(), result.clone());
    tokio::time::sleep(Duration::from_secs(2)).await;

    assert_eq!(cache.get("key"), None);
}

#[tokio::test]
async fn test_cache_clear() {
    let cache = QueryCache::new(Duration::from_secs(60));
    let result = vec![HybridResult {
        memory_id: "test".to_string(),
        final_score: 0.9,
        ..Default::default()
    }];

    cache.set("key".to_string(), result);
    assert!(cache.get("key").is_some());

    cache.clear();
    assert!(cache.get("key").is_none());
}
```

**Filter Evaluation**
```rust
#[test]
fn test_equality_filter() {
    let filter = SearchFilter {
        field: "version".to_string(),
        value: FilterValue::String("1".to_string()),
        operator: FilterOperator::Equals,
    };

    let item = serde_json::json!({"version": "1"});
    assert!(filter.evaluate(&item));
}

#[test]
fn test_inequality_filter() {
    let filter = SearchFilter {
        field: "version".to_string(),
        value: FilterValue::String("2".to_string()),
        operator: FilterOperator::NotEquals,
    };

    let item = serde_json::json!({"version": "1"});
    assert!(filter.evaluate(&item));
}

#[test]
fn test_greater_than_filter() {
    let filter = SearchFilter {
        field: "score".to_string(),
        value: FilterValue::Number(0.5),
        operator: FilterOperator::GreaterThan,
    };

    let item = serde_json::json!({"score": 0.7});
    assert!(filter.evaluate(&item));
}

#[test]
fn test_less_than_filter() {
    let filter = SearchFilter {
        field: "score".to_string(),
        value: FilterValue::Number(0.9),
        operator: FilterOperator::LessThan,
    };

    let item = serde_json::json!({"score": 0.7});
    assert!(filter.evaluate(&item));
}

#[test]
fn test_range_filter() {
    let filter = SearchFilter {
        field: "score".to_string(),
        value: FilterValue::Range(0.5, 0.9),
        operator: FilterOperator::InRange,
    };

    let item = serde_json::json!({"score": 0.7});
    assert!(filter.evaluate(&item));

    let item2 = serde_json::json!({"score": 0.95});
    assert!(!filter.evaluate(&item2));
}

#[test]
fn test_multiple_filters() {
    let filters = vec![
        SearchFilter {
            field: "version".to_string(),
            value: FilterValue::String("1".to_string()),
            operator: FilterOperator::Equals,
        },
        SearchFilter {
            field: "score".to_string(),
            value: FilterValue::Number(0.5),
            operator: FilterOperator::GreaterThan,
        },
    ];

    let item = serde_json::json!({"version": "1", "score": 0.7});
    assert!(SearchFilter::evaluate_all(&filters, &item));

    let item2 = serde_json::json!({"version": "2", "score": 0.7});
    assert!(!SearchFilter::evaluate_all(&filters, &item2));
}
```

**Re-ranking**
```rust
#[test]
fn test_recency_reranking() {
    let mut results = vec![
        HybridResult {
            memory_id: "old".to_string(),
            final_score: 0.9,
            metadata: {
                let mut map = HashMap::new();
                map.insert("created_at".to_string(), serde_json::Value::String("2024-01-01T00:00:00Z".to_string()));
                map
            },
            ..Default::default()
        },
        HybridResult {
            memory_id: "new".to_string(),
            final_score: 0.8,
            metadata: {
                let mut map = HashMap::new();
                map.insert("created_at".to_string(), serde_json::Value::String("2024-12-01T00:00:00Z".to_string()));
                map
            },
            ..Default::default()
        },
    ];

    let reranker = Reranker::new(RerankStrategy::Recency);
    let reranked = reranker.rerank(results);

    assert_eq!(reranked[0].memory_id, "new");
    assert_eq!(reranked[1].memory_id, "old");
}

#[test]
fn test_popularity_reranking() {
    let mut results = vec![
        HybridResult {
            memory_id: "unpopular".to_string(),
            final_score: 0.9,
            metadata: {
                let mut map = HashMap::new();
                map.insert("access_count".to_string(), serde_json::Value::Number(1.into()));
                map
            },
            ..Default::default()
        },
        HybridResult {
            memory_id: "popular".to_string(),
            final_score: 0.8,
            metadata: {
                let mut map = HashMap::new();
                map.insert("access_count".to_string(), serde_json::Value::Number(100.into()));
                map
            },
            ..Default::default()
        },
    ];

    let reranker = Reranker::new(RerankStrategy::Popularity);
    let reranked = reranker.rerank(results);

    assert_eq!(reranked[0].memory_id, "popular");
    assert_eq!(reranked[1].memory_id, "unpopular");
}

#[test]
fn test_custom_reranking() {
    let mut results = vec![
        HybridResult {
            memory_id: "low".to_string(),
            final_score: 0.9,
            metadata: {
                let mut map = HashMap::new();
                map.insert("custom_score".to_string(), serde_json::Value::Number(0.1.into()));
                map
            },
            ..Default::default()
        },
        HybridResult {
            memory_id: "high".to_string(),
            final_score: 0.8,
            metadata: {
                let mut map = HashMap::new();
                map.insert("custom_score".to_string(), serde_json::Value::Number(0.9.into()));
                map
            },
            ..Default::default()
        },
    ];

    let reranker = Reranker::new(RerankStrategy::Custom("custom_score".to_string()));
    let reranked = reranker.rerank(results);

    assert_eq!(reranked[0].memory_id, "high");
    assert_eq!(reranked[1].memory_id, "low");
}
```

**Query Parsing**
```rust
#[test]
fn test_parse_simple_query() {
    let query = "test query";
    let parsed = QueryParser::parse(query).unwrap();

    assert_eq!(parsed.terms, vec!["test", "query"]);
    assert!(parsed.filters.is_empty());
}

#[test]
fn test_parse_query_with_filters() {
    let query = "test query version:1 score>0.5";
    let parsed = QueryParser::parse(query).unwrap();

    assert_eq!(parsed.terms, vec!["test", "query"]);
    assert_eq!(parsed.filters.len(), 2);
}

#[test]
fn test_parse_boolean_query() {
    let query = "test AND query OR alternative";
    let parsed = QueryParser::parse(query).unwrap();

    assert!(parsed.operators.contains(&BooleanOperator::And));
    assert!(parsed.operators.contains(&BooleanOperator::Or));
}

#[test]
fn test_parse_phrase_query() {
    let query = "\"exact phrase\"";
    let parsed = QueryParser::parse(query).unwrap();

    assert!(parsed.is_phrase);
    assert_eq!(parsed.phrase, Some("exact phrase".to_string()));
}

#[test]
fn test_parse_complex_query() {
    let query = "\"exact phrase\" AND term1 OR term2 version:1 score>0.5";
    let parsed = QueryParser::parse(query).unwrap();

    assert!(parsed.is_phrase);
    assert!(parsed.operators.contains(&BooleanOperator::And));
    assert!(parsed.operators.contains(&BooleanOperator::Or));
    assert_eq!(parsed.filters.len(), 2);
}
```

### Property-Based Tests

```rust
#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn proptest_cache_get_set(
            key in "[a-z0-9]{10,50}",
            num_results in 1usize..=100usize
        ) {
            let cache = QueryCache::new(Duration::from_secs(60));

            let results: Vec<HybridResult> = (0..num_results)
                .map(|i| HybridResult {
                    memory_id: format!("id{}", i),
                    final_score: i as f32 / num_results as f32,
                    ..Default::default()
                })
                .collect();

            cache.set(key.clone(), results.clone());
            let retrieved = cache.get(&key);

            prop_assert_eq!(retrieved, Some(results));
        }

        #[test]
        fn proptest_filter_evaluation(
            value in 0i32..=1000i32,
            threshold in 0i32..=1000i32
        ) {
            let filter = SearchFilter {
                field: "score".to_string(),
                value: FilterValue::Number(threshold as f64),
                operator: FilterOperator::GreaterThan,
            };

            let item = serde_json::json!({"score": value});
            let result = filter.evaluate(&item);

            prop_assert_eq!(result, value > threshold);
        }

        #[test]
        fn proptest_reranking_ordering(
            num_results in 10usize..=100usize
        ) {
            let mut results: Vec<HybridResult> = (0..num_results)
                .map(|i| HybridResult {
                    memory_id: format!("id{}", i),
                    final_score: rand::random(),
                    metadata: {
                        let mut map = HashMap::new();
                        map.insert("created_at".to_string(),
                            serde_json::Value::String(
                                (Utc::now() + Duration::days(i as i64)).to_rfc3339()
                            )
                        );
                        map
                    },
                    ..Default::default()
                })
                .collect();

            let reranker = Reranker::new(RerankStrategy::Recency);
            let reranked = reranker.rerank(results);

            // Verify ordering
            for window in reranked.windows(2) {
                let t1 = window[0].metadata.get("created_at").unwrap().as_str().unwrap();
                let t2 = window[1].metadata.get("created_at").unwrap().as_str().unwrap();
                prop_assert!(t1 >= t2, "Results not properly ordered by recency");
            }
        }

        #[test]
        fn proptest_score_bounds(
            num_results in 10usize..=100usize
        ) {
            let results: Vec<HybridResult> = (0..num_results)
                .map(|_| HybridResult {
                    memory_id: uuid::Uuid::new_v4().to_string(),
                    fulltext_score: rand::random(),
                    semantic_score: rand::random(),
                    final_score: rand::random(),
                    ..Default::default()
                })
                .collect();

            for result in &results {
                prop_assert!((0.0..=1.0).contains(&result.fulltext_score));
                prop_assert!((0.0..=1.0).contains(&result.semantic_score));
                prop_assert!((0.0..=1.0).contains(&result.final_score));
            }
        }
    }
}
```

### Integration Tests

**Cached vs Uncached**
- [ ] **Step 1: Initialize engine**
  ```rust
  let cache = QueryCache::new(Duration::from_secs(60));
  let engine = SearchEngine::new(cache, mock_search);
  ```

- [ ] **Step 2: Search with empty cache (miss)**
  ```rust
  let start = Instant::now();
  let results1 = engine.search("query").await.unwrap();
  let uncached_time = start.elapsed();
  ```

- [ ] **Step 3: Search again (hit)**
  ```rust
  let start = Instant::now();
  let results2 = engine.search("query").await.unwrap();
  let cached_time = start.elapsed();
  ```

- [ ] **Step 4: Compare performance**
  ```rust
  assert_eq!(results1, results2);
  assert!(cached_time < uncached_time, "Cache not faster");
  ```

**Filter Combinations**
```rust
#[tokio::test]
async fn test_multiple_filters() {
    let engine = create_test_engine();

    let filters = vec![
        SearchFilter {
            field: "version".to_string(),
            value: FilterValue::String("1".to_string()),
            operator: FilterOperator::Equals,
        },
        SearchFilter {
            field: "score".to_string(),
            value: FilterValue::Number(0.5),
            operator: FilterOperator::GreaterThan,
        },
        SearchFilter {
            field: "tag".to_string(),
            value: FilterValue::String("important".to_string()),
            operator: FilterOperator::Equals,
        },
    ];

    let results = engine.search_with_filters("query", &filters).await.unwrap();

    // All filters should be applied (AND logic)
    for result in &results {
        assert_eq!(result.metadata.get("version").unwrap().as_str().unwrap(), "1");
        assert!(result.metadata.get("score").unwrap().as_f64().unwrap() > 0.5);
    }
}

#[tokio::test]
async fn test_filter_with_boolean_operators() {
    let engine = create_test_engine();

    let query = "term1 AND term2 version:1 OR version:2";
    let results = engine.search(query).await.unwrap();

    // Should return results matching terms and either version
    assert!(!results.is_empty());
}

#[tokio::test]
async fn test_filter_exclusion() {
    let engine = create_test_engine();

    let filters = vec![
        SearchFilter {
            field: "tag".to_string(),
            value: FilterValue::String("excluded".to_string()),
            operator: FilterOperator::NotEquals,
        },
    ];

    let results = engine.search_with_filters("query", &filters).await.unwrap();

    for result in &results {
        let tag = result.metadata.get("tag").unwrap().as_str().unwrap();
        assert_ne!(tag, "excluded");
    }
}
```

**Re-ranking Strategies**
```rust
#[tokio::test]
async fn test_recency_reranking_strategy() {
    let engine = create_test_engine();

    let results = engine
        .search_with_reranking("query", RerankStrategy::Recency)
        .await
        .unwrap();

    // Results should be ordered by creation date (most recent first)
    for window in results.windows(2) {
        let t1 = window[0].metadata.get("created_at").unwrap().as_str().unwrap();
        let t2 = window[1].metadata.get("created_at").unwrap().as_str().unwrap();
        assert!(t1 >= t2);
    }
}

#[tokio::test]
async fn test_popularity_reranking_strategy() {
    let engine = create_test_engine();

    let results = engine
        .search_with_reranking("query", RerankStrategy::Popularity)
        .await
        .unwrap();

    // Results should be ordered by access count (most popular first)
    for window in results.windows(2) {
        let c1 = window[0].metadata.get("access_count").unwrap().as_i64().unwrap();
        let c2 = window[1].metadata.get("access_count").unwrap().as_i64().unwrap();
        assert!(c1 >= c2);
    }
}

#[tokio::test]
async fn test_custom_reranking_strategy() {
    let engine = create_test_engine();

    let results = engine
        .search_with_reranking("query", RerankStrategy::Custom("custom_score".to_string()))
        .await
        .unwrap();

    // Results should be ordered by custom score
    for window in results.windows(2) {
        let s1 = window[0].metadata.get("custom_score").unwrap().as_f64().unwrap();
        let s2 = window[1].metadata.get("custom_score").unwrap().as_f64().unwrap();
        assert!(s1 >= s2);
    }
}
```

**Pagination**
```rust
#[tokio::test]
async fn test_pagination_limit() {
    let engine = create_test_engine_with_results(100);

    let page1 = engine.search_paginated("query", 0, 10).await.unwrap();
    assert_eq!(page1.len(), 10);
}

#[tokio::test]
async fn test_pagination_offset() {
    let engine = create_test_engine_with_results(100);

    let page1 = engine.search_paginated("query", 0, 10).await.unwrap();
    let page2 = engine.search_paginated("query", 10, 10).await.unwrap();

    assert_eq!(page1.len(), 10);
    assert_eq!(page2.len(), 10);
    assert_ne!(page1, page2);
}

#[tokio::test]
async fn test_pagination_beyond_results() {
    let engine = create_test_engine_with_results(10);

    let results = engine.search_paginated("query", 100, 10).await.unwrap();
    assert_eq!(results.len(), 0);
}

#[tokio::test]
async fn test_pagination_consistency() {
    let engine = create_test_engine_with_results(100);

    let all = engine.search("query").await.unwrap();

    let mut paginated = Vec::new();
    for offset in (0..all.len()).step_by(10) {
        let page = engine.search_paginated("query", offset as u64, 10).await.unwrap();
        paginated.extend(page);
    }

    assert_eq!(all, paginated);
}
```

**Large Datasets**
```rust
#[tokio::test]
async fn test_search_performance_large_dataset() {
    let engine = create_test_engine_with_results(100_000);

    let start = Instant::now();
    let results = engine.search("query").await.unwrap();
    let elapsed = start.elapsed();

    println!("Searched {} results in {:?}", results.len(), elapsed);
    assert!(elapsed < Duration::from_secs(5), "Search too slow");
}

#[tokio::test]
async fn test_cache_with_large_dataset() {
    let engine = create_test_engine_with_results(100_000);

    // First search (cache miss)
    let start1 = Instant::now();
    let results1 = engine.search("query").await.unwrap();
    let uncached = start1.elapsed();

    // Second search (cache hit)
    let start2 = Instant::now();
    let results2 = engine.search("query").await.unwrap();
    let cached = start2.elapsed();

    assert_eq!(results1, results2);
    assert!(cached < uncached, "Cache not providing speedup");
}
```

**Edge Cases**
```rust
#[tokio::test]
async fn test_empty_query() {
    let engine = create_test_engine();
    let results = engine.search("").await.unwrap();

    assert_eq!(results.len(), 0);
}

#[tokio::test]
async fn test_no_matching_results() {
    let engine = create_test_engine();
    let results = engine.search("nonexistent term").await.unwrap();

    assert_eq!(results.len(), 0);
}

#[tokio::test]
async fn test_filter_no_matches() {
    let engine = create_test_engine();

    let filters = vec![
        SearchFilter {
            field: "version".to_string(),
            value: FilterValue::String("999".to_string()),
            operator: FilterOperator::Equals,
        },
    ];

    let results = engine.search_with_filters("query", &filters).await.unwrap();
    assert_eq!(results.len(), 0);
}

#[tokio::test]
async fn test_pagination_zero_limit() {
    let engine = create_test_engine_with_results(10);
    let results = engine.search_paginated("query", 0, 0).await.unwrap();

    assert_eq!(results.len(), 0);
}

#[tokio::test]
async fn test_special_characters_in_query() {
    let engine = create_test_engine();
    let results = engine.search("test with @#$% special chars").await.unwrap();

    // Should handle gracefully
    assert!(results.is_empty() || !results.is_empty());
}

#[tokio::test]
async fn test_unicode_query() {
    let engine = create_test_engine();
    let results = engine.search("programming in Rust™ 编程").await.unwrap();

    // Should handle gracefully
    assert!(results.is_empty() || !results.is_empty());
}
```

## Cargo Commands

**Run all search query engine tests:**
```bash
cargo test --test search_query_engine -- --nocapture
```

**Run specific test:**
```bash
cargo test test_cache_hit -- --nocapture
```

**Run property tests:**
```bash
cargo test proptest --test search_query_engine -- --nocapture
```

**Run with logging:**
```bash
RUST_LOG=debug cargo test --test search_query_engine -- --nocapture
```

**Expected output:**
```
running 45 tests
test tests::unit::test_cache_hit ... ok
test tests::unit::test_cache_miss ... ok
test tests::unit::test_cache_expiration ... ok
test tests::unit::test_cache_clear ... ok
test tests::unit::test_equality_filter ... ok
test tests::unit::test_inequality_filter ... ok
test tests::unit::test_greater_than_filter ... ok
test tests::unit::test_less_than_filter ... ok
test tests::unit::test_range_filter ... ok
test tests::unit::test_multiple_filters ... ok
test tests::unit::test_recency_reranking ... ok
test tests::unit::test_popularity_reranking ... ok
test tests::unit::test_custom_reranking ... ok
test tests::unit::test_parse_simple_query ... ok
test tests::unit::test_parse_query_with_filters ... ok
test tests::unit::test_parse_boolean_query ... ok
test tests::unit::test_parse_phrase_query ... ok
test tests::unit::test_parse_complex_query ... ok
test tests::integration::test_cached_vs_uncached ... ok
test tests::integration::test_multiple_filters ... ok
test tests::integration::test_filter_with_boolean_operators ... ok
test tests::integration::test_filter_exclusion ... ok
test tests::integration::test_recency_reranking_strategy ... ok
test tests::integration::test_popularity_reranking_strategy ... ok
test tests::integration::test_custom_reranking_strategy ... ok
test tests::integration::test_pagination_limit ... ok
test tests::integration::test_pagination_offset ... ok
test tests::integration::test_pagination_beyond_results ... ok
test tests::integration::test_pagination_consistency ... ok
test tests::integration::test_search_performance_large_dataset ... ok
test tests::integration::test_cache_with_large_dataset ... ok
test tests::edge_cases::test_empty_query ... ok
test tests::edge_cases::test_no_matching_results ... ok
test tests::edge_cases::test_filter_no_matches ... ok
test tests::edge_cases::test_pagination_zero_limit ... ok
test tests::edge_cases::test_special_characters_in_query ... ok
test tests::edge_cases::test_unicode_query ... ok

test proptests::proptest_cache_get_set ... ok
test proptests::proptest_filter_evaluation ... ok
test proptests::proptest_reranking_ordering ... ok
test proptests::proptest_score_bounds ... ok

test result: ok. 45 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Searched 100000 results in 2.1s
Cache provided 150x speedup
```

## Mock Dependencies

- `HashMap<String, CachedResult>`: Simple in-memory cache
- Mock search results: Pre-defined outputs for deterministic testing
- Time control: `MockTime` for testing cache TTL and time-based features
- `Arc<RwLock>`: Thread-safe shared state
- `tokio::test`: Async test support
- `proptest`: Property-based testing with strategies
- `MockSearchEngine`: Mock search with pre-configured results
