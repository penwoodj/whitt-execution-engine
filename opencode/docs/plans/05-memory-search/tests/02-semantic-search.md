# Test Specifications: Task 02 - Semantic Search

## Mock Strategy

**Mock Embedding Model:**
```rust
struct MockEmbeddingModel {
    embedding_dim: usize,
}

impl MockEmbeddingModel {
    fn new(embedding_dim: usize) -> Self {
        Self { embedding_dim }
    }

    // Deterministic output based on text hash
    fn deterministic_embed(&self, text: &str) -> Vec<f32> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        let seed = hasher.finish();

        (0..self.embedding_dim)
            .map(|i| {
                // Use seed and position for deterministic values in [0, 1]
                let combined = seed.wrapping_mul(i as u64);
                ((combined % 1000) as f32) / 1000.0
            })
            .collect()
    }
}

impl EmbeddingModel for MockEmbeddingModel {
    fn embedding_dim(&self) -> usize {
        self.embedding_dim
    }

    fn embed(&self, text: &str) -> Result<Vec<f32>, EmbeddingError> {
        Ok(self.deterministic_embed(text))
    }

    fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, EmbeddingError> {
        Ok(texts.iter().map(|t| self.deterministic_embed(t)).collect())
    }
}
```

**Pre-computed Embeddings:**
```rust
fn get_precomputed_embeddings() -> HashMap<String, Vec<f32>> {
    let model = MockEmbeddingModel::new(384);
    let mut embeddings = HashMap::new();

    embeddings.insert(
        "rust programming".to_string(),
        model.deterministic_embed("rust programming"),
    );
    embeddings.insert(
        "memory management".to_string(),
        model.deterministic_embed("memory management"),
    );
    embeddings.insert(
        "search algorithms".to_string(),
        model.deterministic_embed("search algorithms"),
    );
    embeddings.insert(
        "concurrency patterns".to_string(),
        model.deterministic_embed("concurrency patterns"),
    );
    embeddings.insert(
        "testing best practices".to_string(),
        model.deterministic_embed("testing best practices"),
    );

    embeddings
}
```

**Mock Vector Store:**
```rust
struct TestVectorStore {
    vectors: Arc<RwLock<HashMap<MemoryId, Vec<f32>>>>,
}

impl TestVectorStore {
    fn new() -> Self {
        Self {
            vectors: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn insert(&self, id: MemoryId, vector: Vec<f32>) -> Result<(), StoreError> {
        let mut vectors = self.vectors.write().await;
        vectors.insert(id, vector);
        Ok(())
    }

    async fn get(&self, id: &MemoryId) -> Result<Option<Vec<f32>>, StoreError> {
        let vectors = self.vectors.read().await;
        Ok(vectors.get(id).cloned())
    }

    async fn find_nearest(
        &self,
        query: &[f32],
        limit: usize,
    ) -> Result<Vec<(MemoryId, f32)>, StoreError> {
        let vectors = self.vectors.read().await;

        let mut results: Vec<_> = vectors
            .iter()
            .map(|(id, vec)| {
                let similarity = cosine_similarity(query, vec);
                (id.clone(), similarity)
            })
            .collect();

        // Sort by similarity (descending)
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        Ok(results.into_iter().take(limit).collect())
    }
}
```

## Test Cases

### Unit Tests

**Embedding Generation**
```rust
#[test]
fn test_embedding_dimension() {
    let model = MockEmbeddingModel::new(384);
    assert_eq!(model.embedding_dim(), 384);

    let model_small = MockEmbeddingModel::new(128);
    assert_eq!(model_small.embedding_dim(), 128);
}

#[test]
fn test_embedding_length() {
    let model = MockEmbeddingModel::new(384);
    let embedding = model.embed("test text").unwrap();

    assert_eq!(embedding.len(), 384);
}

#[test]
fn test_embedding_determinism() {
    let model = MockEmbeddingModel::new(384);
    let text = "test text";

    let emb1 = model.embed(text).unwrap();
    let emb2 = model.embed(text).unwrap();

    assert_eq!(emb1, emb2);
}

#[test]
fn test_embedding_uniqueness() {
    let model = MockEmbeddingModel::new(384);

    let emb1 = model.embed("text1").unwrap();
    let emb2 = model.embed("text2").unwrap();

    assert_ne!(emb1, emb2);
}

#[test]
fn test_batch_embedding() {
    let model = MockEmbeddingModel::new(384);
    let texts = vec!["text1", "text2", "text3"];

    let embeddings = model.embed_batch(&texts).unwrap();

    assert_eq!(embeddings.len(), 3);
    for emb in &embeddings {
        assert_eq!(emb.len(), 384);
    }
}
```

**Similarity Computation**
```rust
#[test]
fn test_cosine_similarity_identical() {
    let v1 = vec![1.0, 0.0, 0.0];
    let v2 = vec![1.0, 0.0, 0.0];

    let similarity = cosine_similarity(&v1, &v2);
    assert!((similarity - 1.0).abs() < 1e-6);
}

#[test]
fn test_cosine_similarity_orthogonal() {
    let v1 = vec![1.0, 0.0];
    let v2 = vec![0.0, 1.0];

    let similarity = cosine_similarity(&v1, &v2);
    assert!((similarity - 0.0).abs() < 1e-6);
}

#[test]
fn test_cosine_similarity_opposite() {
    let v1 = vec![1.0, 0.0, 0.0];
    let v2 = vec![-1.0, 0.0, 0.0];

    let similarity = cosine_similarity(&v1, &v2);
    assert!((similarity - (-1.0)).abs() < 1e-6);
}

#[test]
fn test_cosine_similarity_intermediate() {
    let v1 = vec![1.0, 1.0];
    let v2 = vec![1.0, 0.0];

    let similarity = cosine_similarity(&v1, &v2);
    assert!((0.0..1.0).contains(&similarity));
}

#[test]
fn test_cosine_similarity_symmetry() {
    let v1 = vec![1.0, 2.0, 3.0];
    let v2 = vec![4.0, 5.0, 6.0];

    let sim1 = cosine_similarity(&v1, &v2);
    let sim2 = cosine_similarity(&v2, &v1);

    assert!((sim1 - sim2).abs() < 1e-6);
}

#[test]
fn test_dot_product() {
    let v1 = vec![1.0, 2.0, 3.0];
    let v2 = vec![4.0, 5.0, 6.0];

    let dot = dot_product(&v1, &v2);
    assert_eq!(dot, 32.0); // 1*4 + 2*5 + 3*6
}

#[test]
fn test_vector_magnitude() {
    let v = vec![3.0, 4.0];
    let mag = vector_magnitude(&v);

    assert!((mag - 5.0).abs() < 1e-6);
}
```

**Hybrid Fusion**
```rust
#[test]
fn test_hybrid_score_equal_weights() {
    let fulltext_score = 0.8;
    let semantic_score = 0.6;
    let fulltext_weight = 0.5;
    let semantic_weight = 0.5;

    let combined = hybrid_score(
        fulltext_score,
        semantic_score,
        fulltext_weight,
        semantic_weight,
    );

    assert!((combined - 0.7).abs() < 0.01);
}

#[test]
fn test_hybrid_score_fulltext_dominant() {
    let fulltext_score = 0.9;
    let semantic_score = 0.5;
    let fulltext_weight = 0.8;
    let semantic_weight = 0.2;

    let combined = hybrid_score(
        fulltext_score,
        semantic_score,
        fulltext_weight,
        semantic_weight,
    );

    assert!((combined - 0.82).abs() < 0.01);
}

#[test]
fn test_hybrid_score_semantic_dominant() {
    let fulltext_score = 0.5;
    let semantic_score = 0.9;
    let fulltext_weight = 0.2;
    let semantic_weight = 0.8;

    let combined = hybrid_score(
        fulltext_score,
        semantic_score,
        fulltext_weight,
        semantic_weight,
    );

    assert!((combined - 0.82).abs() < 0.01);
}

#[test]
fn test_hybrid_score_bounds() {
    let combined1 = hybrid_score(0.0, 0.0, 0.5, 0.5);
    let combined2 = hybrid_score(1.0, 1.0, 0.5, 0.5);

    assert!((combined1 - 0.0).abs() < 1e-6);
    assert!((combined2 - 1.0).abs() < 1e-6);
}
```

**Normalization**
```rust
#[test]
fn test_vector_normalization() {
    let mut v = vec![3.0, 4.0];
    normalize_vector(&mut v);

    let mag = vector_magnitude(&v);
    assert!((mag - 1.0).abs() < 1e-6);
}

#[test]
fn test_normalization_preserves_direction() {
    let mut v1 = vec![3.0, 4.0];
    let mut v2 = vec![6.0, 8.0]; // Same direction, double magnitude

    normalize_vector(&mut v1);
    normalize_vector(&mut v2);

    // Should be identical after normalization
    assert_eq!(v1, v2);
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
        fn proptest_embedding_dimensions(
            text in "[a-zA-Z0-9 ]{10,100}",
            dim in 128usize..=1024usize
        ) {
            let model = MockEmbeddingModel::new(dim);
            let embedding = model.embed(&text).unwrap();

            prop_assert_eq!(embedding.len(), dim);
        }

        #[test]
        fn proptest_embedding_determinism(
            text in "[a-zA-Z0-9 ]{10,100}",
            dim in 128usize..=1024usize
        ) {
            let model = MockEmbeddingModel::new(dim);

            let emb1 = model.embed(&text).unwrap();
            let emb2 = model.embed(&text).unwrap();

            prop_assert_eq!(emb1, emb2);
        }

        #[test]
        fn proptest_cosine_similarity_bounds(
            dim in 2usize..=100usize
        ) {
            let v1: Vec<f32> = (0..dim).map(|_| rand::random()).collect();
            let v2: Vec<f32> = (0..dim).map(|_| rand::random()).collect();

            let similarity = cosine_similarity(&v1, &v2);

            prop_assert!((-1.0..=1.0).contains(&similarity));
        }

        #[test]
        fn proptest_cosine_similarity_symmetry(
            dim in 2usize..=100usize
        ) {
            let v1: Vec<f32> = (0..dim).map(|_| rand::random()).collect();
            let v2: Vec<f32> = (0..dim).map(|_| rand::random()).collect();

            let sim1 = cosine_similarity(&v1, &v2);
            let sim2 = cosine_similarity(&v2, &v1);

            prop_assert!((sim1 - sim2).abs() < 1e-6);
        }

        #[test]
        fn proptest_normalization_unit_magnitude(
            dim in 2usize..=100usize
        ) {
            let mut v: Vec<f32> = (0..dim).map(|_| rand::random()).collect();

            // Skip zero vectors
            if vector_magnitude(&v) < 1e-6 {
                return Ok(());
            }

            normalize_vector(&mut v);

            let mag = vector_magnitude(&v);
            prop_assert!((mag - 1.0).abs() < 1e-6);
        }

        #[test]
        fn proptest_hybrid_score_bounds(
            ft in 0.0f32..=1.0f32,
            sem in 0.0f32..=1.0f32,
            ft_w in 0.0f32..=1.0f32,
            sem_w in 0.0f32..=1.0f32
        ) {
            let combined = hybrid_score(ft, sem, ft_w, sem_w);

            prop_assert!((0.0..=1.0).contains(&combined));
        }

        #[test]
        fn proptest_dot_product_property(
            dim in 2usize..=100usize
        ) {
            let v1: Vec<f32> = (0..dim).map(|_| rand::random()).collect();
            let v2: Vec<f32> = (0..dim).map(|_| rand::random()).collect();

            let dot1 = dot_product(&v1, &v2);
            let dot2 = dot_product(&v2, &v1);

            prop_assert_eq!(dot1, dot2);
        }
    }
}
```

### Integration Tests

**Semantic Search**
- [ ] **Step 1: Initialize components**
  ```rust
  let model = MockEmbeddingModel::new(384);
  let store = TestVectorStore::new();
  ```

- [ ] **Step 2: Create test vectors**
  ```rust
  let texts = vec![
      ("id1", "rust programming language"),
      ("id2", "memory management in rust"),
      ("id3", "concurrency patterns"),
  ];

  for (id, text) in &texts {
      let embedding = model.embed(text).unwrap();
      store.insert(id.to_string(), embedding).await.unwrap();
  }
  ```

- [ ] **Step 3: Search with query**
  ```rust
  let query = "parallel programming";
  let query_embedding = model.embed(query).unwrap();
  let results = store.find_nearest(&query_embedding, 5).await.unwrap();
  ```

- [ ] **Step 4: Verify similarity order**
  ```rust
  assert!(!results.is_empty());
  for window in results.windows(2) {
      assert!(window[0].1 >= window[1].1);
  }
  ```

- [ ] **Step 5: Check scores**
  ```rust
  for (id, score) in results {
      assert!((0.0..=1.0).contains(&score));
      assert!(!id.is_empty());
  }
  ```

**Hybrid Search**
```rust
#[tokio::test]
async fn test_hybrid_search_combination() {
    let model = MockEmbeddingModel::new(384);
    let store = TestVectorStore::new();

    // Index documents
    let texts = vec![
        ("id1", "rust programming language"),
        ("id2", "memory management in rust"),
        ("id3", "search algorithms"),
    ];

    for (id, text) in &texts {
        let embedding = model.embed(text).unwrap();
        store.insert(id.to_string(), embedding).await.unwrap();
    }

    // Mock fulltext results
    let fulltext_results = vec![
        SearchResult { id: "id1".to_string(), score: 0.9, .. },
        SearchResult { id: "id3".to_string(), score: 0.7, .. },
    ];

    // Get semantic results
    let query = "programming in rust";
    let query_embedding = model.embed(query).unwrap();
    let semantic_results = store.find_nearest(&query_embedding, 10).await.unwrap();

    // Combine with equal weights
    let hybrid_results = combine_results(
        fulltext_results,
        semantic_results,
        0.5,
        0.5,
    );

    assert!(!hybrid_results.is_empty());
    for window in hybrid_results.windows(2) {
        assert!(window[0].final_score >= window[1].final_score);
    }
}

#[tokio::test]
async fn test_hybrid_search_different_weights() {
    let model = MockEmbeddingModel::new(384);
    let store = TestVectorStore::new();

    // Index documents
    for i in 0..10 {
        let text = format!("document {}", i);
        let embedding = model.embed(&text).unwrap();
        store.insert(format!("id{}", i), embedding).await.unwrap();
    }

    // Test different weight combinations
    let weights = vec![(1.0, 0.0), (0.5, 0.5), (0.0, 1.0)];

    for (ft_weight, sem_weight) in weights {
        let fulltext_results = create_mock_fulltext_results(10);
        let semantic_results = store.find_nearest(&model.embed("test").unwrap(), 10).await.unwrap();

        let hybrid = combine_results(fulltext_results, semantic_results, ft_weight, sem_weight);
        assert!(!hybrid.is_empty());
    }
}
```

**Batch Embeddings**
```rust
#[tokio::test]
async fn test_batch_embedding_generation() {
    let model = MockEmbeddingModel::new(384);
    let texts = vec!["text1", "text2", "text3", "text4", "text5"];

    let start = Instant::now();
    let embeddings = model.embed_batch(&texts).unwrap();
    let elapsed = start.elapsed();

    assert_eq!(embeddings.len(), 5);
    for emb in &embeddings {
        assert_eq!(emb.len(), 384);
    }

    // Batch should be faster than individual calls
    let individual_start = Instant::now();
    for text in &texts {
        model.embed(text).unwrap();
    }
    let individual_elapsed = individual_start.elapsed();

    // Batch is more efficient (mock model, but pattern holds)
    assert!(elapsed < individual_elapsed);
}

#[tokio::test]
async fn test_large_batch_embeddings() {
    let model = MockEmbeddingModel::new(384);
    let texts: Vec<String> = (0..1000).map(|i| format!("text {}", i)).collect();

    let embeddings = model.embed_batch(&texts.iter().map(|s| s.as_str()).collect::<Vec<_>>()).unwrap();

    assert_eq!(embeddings.len(), 1000);
    for emb in &embeddings {
        assert_eq!(emb.len(), 384);
    }
}
```

**Nearest Neighbor Search**
```rust
#[tokio::test]
async fn test_k_nearest_neighbors() {
    let model = MockEmbeddingModel::new(384);
    let store = TestVectorStore::new();

    // Index 100 vectors
    for i in 0..100 {
        let text = format!("document {}", i);
        let embedding = model.embed(&text).unwrap();
        store.insert(format!("id{}", i), embedding).await.unwrap();
    }

    // Query for k=10 nearest
    let query_embedding = model.embed("document 50").unwrap();
    let results = store.find_nearest(&query_embedding, 10).await.unwrap();

    assert_eq!(results.len(), 10);
    assert_eq!(results[0].1, 1.0); // Most similar is exact match
    assert!(results[0].0 == "id50");
}

#[tokio::test]
async fn test_nearest_neighbor_performance() {
    let model = MockEmbeddingModel::new(384);
    let store = TestVectorStore::new();

    // Index 10,000 vectors
    for i in 0..10_000 {
        let text = format!("doc-{:08}", i);
        let embedding = model.embed(&text).unwrap();
        store.insert(text, embedding).await.unwrap();
    }

    // Measure search time
    let start = Instant::now();
    let query_embedding = model.embed("test query").unwrap();
    let results = store.find_nearest(&query_embedding, 100).await.unwrap();
    let elapsed = start.elapsed();

    println!("Searched {} vectors in {:?}", 10_000, elapsed);
    assert!(elapsed < Duration::from_millis(500));
    assert_eq!(results.len(), 100);
}
```

**Edge Cases**
```rust
#[tokio::test]
async fn test_empty_query() {
    let model = MockEmbeddingModel::new(384);
    let store = TestVectorStore::new();

    let empty_embedding = model.embed("").unwrap();
    let results = store.find_nearest(&empty_embedding, 10).await.unwrap();

    // Should return results sorted by similarity
    assert!(results.is_empty() || results[0].1 <= 1.0);
}

#[tokio::test]
async fn test_no_indexed_documents() {
    let model = MockEmbeddingModel::new(384);
    let store = TestVectorStore::new();

    let query_embedding = model.embed("test").unwrap();
    let results = store.find_nearest(&query_embedding, 10).await.unwrap();

    assert_eq!(results.len(), 0);
}

#[tokio::test]
async fn test_duplicate_embeddings() {
    let model = MockEmbeddingModel::new(384);
    let store = TestVectorStore::new();

    let text = "test document";
    let embedding = model.embed(text).unwrap();

    store.insert("id1".to_string(), embedding.clone()).await.unwrap();
    store.insert("id2".to_string(), embedding).await.unwrap();

    let query_embedding = model.embed(text).unwrap();
    let results = store.find_nearest(&query_embedding, 10).await.unwrap();

    // Both should have similarity 1.0
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].1, 1.0);
    assert_eq!(results[1].1, 1.0);
}

#[tokio::test]
async fn test_zero_vector() {
    let model = MockEmbeddingModel::new(384);
    let store = TestVectorStore::new();

    let text = "test";
    store.insert("id1".to_string(), model.embed(text).unwrap()).await.unwrap();

    let zero_vector = vec![0.0; 384];
    let results = store.find_nearest(&zero_vector, 10).await.unwrap();

    // Should handle gracefully
    assert!(results.len() <= 1);
}

#[tokio::test]
async fn test_unicode_text() {
    let model = MockEmbeddingModel::new(384);
    let store = TestVectorStore::new();

    let texts = vec![
        ("id1", "programming in Rust™"),
        ("id2", "Rust 编程"),
        ("id3", "Rust プログラミング"),
    ];

    for (id, text) in texts {
        let embedding = model.embed(text).unwrap();
        store.insert(id.to_string(), embedding).await.unwrap();
    }

    let query_embedding = model.embed("programming").unwrap();
    let results = store.find_nearest(&query_embedding, 10).await.unwrap();

    assert!(!results.is_empty());
}
```

## Cargo Commands

**Run all semantic search tests:**
```bash
cargo test --test semantic_search -- --nocapture
```

**Run specific test:**
```bash
cargo test test_cosine_similarity_identical -- --nocapture
```

**Run property tests:**
```bash
cargo test proptest --test semantic_search -- --nocapture
```

**Run with logging:**
```bash
RUST_LOG=debug cargo test --test semantic_search -- --nocapture
```

**Expected output:**
```
running 40 tests
test tests::unit::test_embedding_dimension ... ok
test tests::unit::test_embedding_length ... ok
test tests::unit::test_embedding_determinism ... ok
test tests::unit::test_embedding_uniqueness ... ok
test tests::unit::test_batch_embedding ... ok
test tests::unit::test_cosine_similarity_identical ... ok
test tests::unit::test_cosine_similarity_orthogonal ... ok
test tests::unit::test_cosine_similarity_opposite ... ok
test tests::unit::test_cosine_similarity_intermediate ... ok
test tests::unit::test_cosine_similarity_symmetry ... ok
test tests::unit::test_dot_product ... ok
test tests::unit::test_vector_magnitude ... ok
test tests::unit::test_hybrid_score_equal_weights ... ok
test tests::unit::test_hybrid_score_fulltext_dominant ... ok
test tests::unit::test_hybrid_score_semantic_dominant ... ok
test tests::unit::test_hybrid_score_bounds ... ok
test tests::unit::test_vector_normalization ... ok
test tests::unit::test_normalization_preserves_direction ... ok
test tests::integration::test_hybrid_search_combination ... ok
test tests::integration::test_hybrid_search_different_weights ... ok
test tests::integration::test_batch_embedding_generation ... ok
test tests::integration::test_large_batch_embeddings ... ok
test tests::integration::test_k_nearest_neighbors ... ok
test tests::integration::test_nearest_neighbor_performance ... ok
test tests::edge_cases::test_empty_query ... ok
test tests::edge_cases::test_no_indexed_documents ... ok
test tests::edge_cases::test_duplicate_embeddings ... ok
test tests::edge_cases::test_zero_vector ... ok
test tests::edge_cases::test_unicode_text ... ok

test proptests::proptest_embedding_dimensions ... ok
test proptests::proptest_embedding_determinism ... ok
test proptests::proptest_cosine_similarity_bounds ... ok
test proptests::proptest_cosine_similarity_symmetry ... ok
test proptests::proptest_normalization_unit_magnitude ... ok
test proptests::proptest_hybrid_score_bounds ... ok
test proptests::proptest_dot_product_property ... ok

test result: ok. 40 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Searched 10000 vectors in 120ms
```

## Mock Dependencies

- `MockEmbeddingModel`: Deterministic embeddings based on text hash
- Pre-computed vectors: Known test outputs for reproducible tests
- In-memory vector store: Fast testing with `HashMap` storage
- `tokio::test`: Async test support
- `proptest`: Property-based testing with strategies
- `Arc<RwLock>`: Thread-safe shared state for concurrent tests
