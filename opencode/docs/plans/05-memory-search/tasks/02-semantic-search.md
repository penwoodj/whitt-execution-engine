# Task 02: Semantic Search

**Estimated Time:** 1-2 weeks
**Dependencies:** Task 01 complete
**Priority:** HIGH (required for hybrid search)

## Overview

Implement semantic search using local embedding models, vector similarity calculations, and hybrid fusion with exact/fulltext search results.

## Files

### Create
- `crates/search/src/semantic.rs` - Vector embedding and similarity search
- `crates/search/src/hybrid.rs` - Hybrid fusion algorithms (exact + semantic)
- `crates/embeddings/Cargo.toml` - Embedding model crate manifest
- `crates/embeddings/src/lib.rs` - Public API
- `crates/embeddings/src/model.rs` - Embedding model interface
- `crates/embeddings/src/local.rs` - Local embedding model implementation

### Modify
- `crates/search/Cargo.toml` - Add embedding dependencies
- `Cargo.toml` - Add embeddings workspace member

### Test
- `crates/embeddings/tests/integration_test.rs` - Integration tests
- `crates/search/tests/semantic_test.rs` - Semantic search tests

---

## Step-by-Step Implementation

### Step 1: Create embeddings crate structure

```bash
mkdir -p crates/embeddings/src

cat > crates/embeddings/Cargo.toml << 'EOF'
[package]
name = "agentsdk-embeddings"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
tracing = "0.1"
candle-core = "0.3"
candle-transformers = "0.3"
candle-nn = "0.3"
tokenizers = "0.15"
half = "2.3"
EOF
```

Run: `cargo check --package agentsdk-embeddings`
Expected: SUCCESS

- [ ] **Step 1: Create embeddings crate structure**

### Step 2: Write embedding model interface

Create `crates/embeddings/src/model.rs`:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmbeddingError {
    #[error("Model load error: {0}")]
    ModelLoadError(String),

    #[error("Inference error: {0}")]
    InferenceError(String),

    #[error("Tokenization error: {0}")]
    TokenizationError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

pub trait EmbeddingModel: Send + Sync {
    fn embedding_dim(&self) -> usize;

    async fn embed(&self, text: &str) -> Result<Vec<f32>, EmbeddingError>;

    async fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, EmbeddingError>;
}
```

Run: `cargo check --package agentsdk-embeddings`
Expected: SUCCESS

- [ ] **Step 2: Write embedding model interface**

### Step 3: Write local embedding model

Create `crates/embeddings/src/local.rs`:

```rust
use super::model::{EmbeddingModel, EmbeddingError};
use candle_core::{Tensor, Device};
use candle_transformers::models::bert::{BertModel, Config};
use tokenizers::Tokenizer;
use std::path::Path;

pub struct LocalEmbeddingModel {
    model: BertModel,
    tokenizer: Tokenizer,
    device: Device,
    embedding_dim: usize,
}

impl LocalEmbeddingModel {
    pub fn new(
        model_path: impl AsRef<Path>,
        tokenizer_path: impl AsRef<Path>,
    ) -> Result<Self, EmbeddingError> {
        // Load model
        let device = Device::Cpu;
        let config = Config::from_file(model_path.as_ref().join("config.json"))
            .map_err(|e| EmbeddingError::ModelLoadError(format!("Failed to load config: {}", e)))?;

        let model = BertModel::load(model_path.as_ref(), &config, &device)
            .map_err(|e| EmbeddingError::ModelLoadError(format!("Failed to load model: {}", e)))?;

        // Load tokenizer
        let tokenizer = Tokenizer::from_file(tokenizer_path, true)
            .map_err(|e| EmbeddingError::TokenizationError(format!("Failed to load tokenizer: {}", e)))?;

        let embedding_dim = config.hidden_size;

        Ok(Self {
            model,
            tokenizer,
            device,
            embedding_dim,
        })
    }

    fn tokenize(&self, text: &str) -> Result<(Vec<u32>, Vec<Vec<u32>>), EmbeddingError> {
        let encoding = self.tokenizer
            .encode(text, true)
            .map_err(|e| EmbeddingError::TokenizationError(format!("Failed to tokenize: {}", e)))?;

        let ids = encoding.get_ids().to_vec();
        let attention_mask = encoding.get_attention_mask().iter()
            .map(|&x| vec![x])
            .collect();

        Ok((ids, attention_mask))
    }
}

#[async_trait::async_trait]
impl EmbeddingModel for LocalEmbeddingModel {
    fn embedding_dim(&self) -> usize {
        self.embedding_dim
    }

    async fn embed(&self, text: &str) -> Result<Vec<f32>, EmbeddingError> {
        let (token_ids, attention_mask) = self.tokenize(text)?;

        // Convert to tensors
        let input_ids = Tensor::new(&token_ids[..], &self.device)
            .reshape((1, token_ids.len()))
            .map_err(|e| EmbeddingError::InferenceError(format!("Failed to create input tensor: {}", e)))?;

        let mask = Tensor::new(
            attention_mask.iter().flatten().copied().collect::<Vec<_>>(),
            &self.device
        )
        .reshape((1, token_ids.len()))
        .map_err(|e| EmbeddingError::InferenceError(format!("Failed to create mask tensor: {}", e)))?;

        // Run inference
        let output = self.model
            .forward(&input_ids, Some(&mask), None, None)
            .map_err(|e| EmbeddingError::InferenceError(format!("Model inference failed: {}", e)))?;

        // Get [CLS] token embedding (mean pooling)
        let embedding = output.mean(1)
            .map_err(|e| EmbeddingError::InferenceError(format!("Failed to compute mean: {}", e)))?;

        let embedding_vec = embedding.to_vec1()
            .map_err(|e| EmbeddingError::InferenceError(format!("Failed to convert to vec: {}", e)))?;

        Ok(embedding_vec)
    }

    async fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, EmbeddingError> {
        let mut embeddings = Vec::with_capacity(texts.len());

        for text in texts {
            let embedding = self.embed(text).await?;
            embeddings.push(embedding);
        }

        Ok(embeddings)
    }
}
```

Run: `cargo check --package agentsdk-embeddings`
Expected: SUCCESS

- [ ] **Step 3: Write local embedding model**

### Step 4: Write embeddings lib.rs

Create `crates/embeddings/src/lib.rs`:

```rust
pub mod model;
pub mod local;

pub use model::{EmbeddingModel, EmbeddingError};
pub use local::LocalEmbeddingModel;
```

Run: `cargo check --package agentsdk-embeddings`
Expected: SUCCESS

- [ ] **Step 4: Write embeddings lib.rs**

### Step 5: Write semantic search index

Create `crates/search/src/semantic.rs`:

```rust
use crate::error::SearchError;
use agentsdk_embeddings::EmbeddingModel;
use agentsdk_memory::{MemoryId, MemoryOperations, StructuredMemory, UnstructuredMemory};
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Vector {
    pub memory_id: MemoryId,
    pub embedding: Vec<f32>,
}

pub struct SemanticIndex {
    vectors: HashMap<MemoryId, Vec<f32>>,
    embedding_dim: usize,
    model: Box<dyn EmbeddingModel>,
}

impl SemanticIndex {
    pub fn new(model: Box<dyn EmbeddingModel>) -> Self {
        let embedding_dim = model.embedding_dim();

        Self {
            vectors: HashMap::new(),
            embedding_dim,
            model,
        }
    }

    pub async fn index_structured(
        &mut self,
        memory: &StructuredMemory,
    ) -> Result<(), SearchError> {
        let text = self.extract_text_from_structured(memory);
        let embedding = self.model.embed(&text).await
            .map_err(|e| SearchError::QueryParseError(format!("Embedding failed: {}", e)))?;

        self.vectors.insert(memory.id.clone(), embedding);
        Ok(())
    }

    pub async fn index_unstructured(
        &mut self,
        memory: &UnstructuredMemory,
    ) -> Result<(), SearchError> {
        let embedding = self.model.embed(&memory.content).await
            .map_err(|e| SearchError::QueryParseError(format!("Embedding failed: {}", e)))?;

        self.vectors.insert(memory.id.clone(), embedding);
        Ok(())
    }

    pub async fn search(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<(MemoryId, f32)>, SearchError> {
        let query_embedding = self.model.embed(query).await
            .map_err(|e| SearchError::QueryParseError(format!("Query embedding failed: {}", e)))?;

        let mut results: Vec<(MemoryId, f32)> = self.vectors
            .iter()
            .map(|(id, embedding)| {
                let similarity = self.cosine_similarity(&query_embedding, embedding);
                (id.clone(), similarity)
            })
            .collect();

        // Sort by similarity (descending)
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Return top results
        Ok(results.into_iter().take(limit).collect())
    }

    fn cosine_similarity(&self, a: &[f32], b: &[f32]) -> f32 {
        assert_eq!(a.len(), b.len());

        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot_product / (norm_a * norm_b)
        }
    }

    fn extract_text_from_structured(&self, memory: &StructuredMemory) -> String {
        // Convert JSON to text representation
        memory.data.to_string()
    }
}
```

Run: `cargo check --package agentsdk-search`
Expected: SUCCESS

- [ ] **Step 5: Write semantic search index**

### Step 6: Write hybrid fusion

Create `crates/search/src/hybrid.rs`:

```rust
use crate::error::SearchError;
use crate::fulltext::FullTextSearchIndex;
use crate::query_parser::SearchQuery;
use crate::semantic::SemanticIndex;
use crate::ranking::SearchResult;
use agentsdk_memory::MemoryId;

#[derive(Debug, Clone)]
pub struct HybridResult {
    pub memory_id: MemoryId,
    pub fulltext_score: f32,
    pub semantic_score: f32,
    pub final_score: f32,
    pub result: SearchResult,
}

pub struct HybridSearchEngine {
    fulltext_index: FullTextSearchIndex,
    semantic_index: SemanticIndex,
    fulltext_weight: f32,
    semantic_weight: f32,
}

impl HybridSearchEngine {
    pub fn new(
        fulltext_index: FullTextSearchIndex,
        semantic_index: SemanticIndex,
    ) -> Self {
        Self {
            fulltext_index,
            semantic_index,
            fulltext_weight: 0.5,
            semantic_weight: 0.5,
        }
    }

    pub fn with_weights(
        mut self,
        fulltext_weight: f32,
        semantic_weight: f32,
    ) -> Self {
        self.fulltext_weight = fulltext_weight;
        self.semantic_weight = semantic_weight;
        self
    }

    pub async fn search(
        &self,
        query: &SearchQuery,
        limit: usize,
    ) -> Result<Vec<HybridResult>, SearchError> {
        // Run full-text search
        let parsed_query = query.parse(&self.fulltext_index)?;
        let searcher = self.fulltext_index.searcher();
        let top_docs = searcher
            .search(&parsed_query, &tantivy::collector::TopDocs::with_limit(limit * 2))
            .unwrap();

        // Convert to results
        let fulltext_results: Vec<(MemoryId, f32)> = top_docs
            .into_iter()
            .map(|(score, doc_address)| {
                let doc = searcher.doc(doc_address).unwrap();
                let id_field = self.fulltext_index.schema().get_field("id").unwrap();
                let memory_id = doc
                    .get_first(id_field)
                    .and_then(|val| val.as_str())
                    .unwrap_or("")
                    .to_string();

                let id = MemoryId::from_string(&memory_id).unwrap();
                (id, score as f32)
            })
            .collect();

        // Run semantic search
        let semantic_results = self.semantic_index
            .search(&query.raw, limit * 2)
            .await?;

        // Combine and re-rank
        self.fuse_results(fulltext_results, semantic_results, limit)
    }

    fn fuse_results(
        &self,
        fulltext_results: Vec<(MemoryId, f32)>,
        semantic_results: Vec<(MemoryId, f32)>,
        limit: usize,
    ) -> Result<Vec<HybridResult>, SearchError> {
        // Build lookup maps
        let fulltext_map: HashMap<MemoryId, f32> = fulltext_results.into_iter().collect();
        let semantic_map: HashMap<MemoryId, f32> = semantic_results.into_iter().collect();

        // Get all unique IDs
        let mut all_ids: std::collections::HashSet<MemoryId> = fulltext_map.keys().cloned().collect();
        all_ids.extend(semantic_map.keys().cloned());

        // Calculate hybrid scores
        let mut hybrid_results: Vec<HybridResult> = all_ids
            .into_iter()
            .map(|id| {
                let fulltext_score = fulltext_map.get(&id).copied().unwrap_or(0.0);
                let semantic_score = semantic_map.get(&id).copied().unwrap_or(0.0);

                // Normalize scores (simple min-max)
                let norm_fulltext = if fulltext_score > 0.0 {
                    fulltext_score.min(1.0)
                } else {
                    0.0
                };

                let norm_semantic = if semantic_score > 0.0 {
                    semantic_score.min(1.0)
                } else {
                    0.0
                };

                // Weighted combination
                let final_score = (norm_fulltext * self.fulltext_weight)
                    + (norm_semantic * self.semantic_weight);

                // Create a minimal SearchResult (we'd retrieve full details in practice)
                let result = SearchResult {
                    doc_address: tantivy::DocAddress::new(0, 0),
                    score: final_score,
                    memory_id: format!("{}", id.0),
                    snippet: String::new(),
                };

                HybridResult {
                    memory_id: id,
                    fulltext_score: norm_fulltext,
                    semantic_score: norm_semantic,
                    final_score,
                    result,
                }
            })
            .collect();

        // Sort by final score
        hybrid_results.sort_by(|a, b| b.final_score.partial_cmp(&a.final_score).unwrap());

        // Return top results
        Ok(hybrid_results.into_iter().take(limit).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weight_combination() {
        let fulltext = 0.8;
        let semantic = 0.6;
        let fulltext_weight = 0.7;
        let semantic_weight = 0.3;

        let combined = (fulltext * fulltext_weight) + (semantic * semantic_weight);
        assert!((combined - 0.74).abs() < 0.001);
    }
}
```

Run: `cargo check --package agentsdk-search`
Expected: SUCCESS

- [ ] **Step 6: Write hybrid fusion**

### Step 7: Update search lib.rs

Modify `crates/search/src/lib.rs`:

```rust
pub mod error;
pub mod fulltext;
pub mod query_parser;
pub mod ranking;
pub mod highlighting;
pub mod semantic;
pub mod hybrid;

pub use error::SearchError;
pub use fulltext::FullTextSearchIndex;
pub use query_parser::{SearchQuery, SearchFilter, FilterOperator};
pub use ranking::{SearchResult, SearchResultCollector, rank_results};
pub use highlighting::Highlighter;
pub use semantic::{SemanticIndex, Vector};
pub use hybrid::{HybridSearchEngine, HybridResult};
```

Run: `cargo check --package agentsdk-search`
Expected: SUCCESS

- [ ] **Step 7: Update search lib.rs**

### Step 8: Add to workspace and dependencies

Modify `Cargo.toml`:

Add to `[workspace.members]`:

```toml
members = [
    # ... existing members ...
    "crates/embeddings",
]
```

Modify `crates/search/Cargo.toml`:

```toml
[dependencies]
# ... existing dependencies ...
agentsdk-embeddings = { path = "../embeddings" }
```

Run: `cargo check --workspace`
Expected: SUCCESS

- [ ] **Step 8: Add to workspace and dependencies**

### Step 9: Write integration tests

Create `crates/embeddings/tests/integration_test.rs`:

```rust
use agentsdk_embeddings::EmbeddingModel;

#[tokio::test]
async fn test_model_dimensions() {
    // This test will need a real model or mock
    // For now, we'll create a mock model

    struct MockModel;

    #[async_trait::async_trait]
    impl EmbeddingModel for MockModel {
        fn embedding_dim(&self) -> usize {
            384
        }

        async fn embed(&self, text: &str) -> Result<Vec<f32>, agentsdk_embeddings::EmbeddingError> {
            // Return a deterministic embedding based on text length
            let dim = self.embedding_dim();
            Ok(vec![text.len() as f32 / dim as f32; dim])
        }

        async fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, agentsdk_embeddings::EmbeddingError> {
            let mut embeddings = Vec::new();
            for text in texts {
                embeddings.push(self.embed(text).await?);
            }
            Ok(embeddings)
        }
    }

    let model = MockModel;
    assert_eq!(model.embedding_dim(), 384);

    let embedding = model.embed("test").await.unwrap();
    assert_eq!(embedding.len(), 384);
}
```

Run: `cargo test --package agentsdk-embeddings --test integration_test`
Expected: All tests PASS

- [ ] **Step 9: Write integration tests**

### Step 10: Commit

```bash
git add crates/embeddings/ crates/search/ Cargo.toml
git commit -m "feat(Phase5-Task02): implement semantic search with local embedding models and hybrid fusion with fulltext search"
```

- [ ] **Step 10: Commit**

---

## Validation Criteria

See [validation/02-semantic-search.md](../validation/02-semantic-search.md)

## Test Specifications

See [tests/02-semantic-search.md](../tests/02-semantic-search.md)
