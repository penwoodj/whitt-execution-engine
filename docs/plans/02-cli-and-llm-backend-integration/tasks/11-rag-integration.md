# Task 11: RAG Integration

**Files:**
- Create: `src/rag/mod.rs`
- Create: `src/rag/index.rs`
- Create: `src/rag/embeddings.rs`
- Create: `src/rag/retrieval.rs`
- Create: `src/rag/context.rs`
- Modify: `src/lib.rs` (add rag module)
- Test: `tests/rag/rag_test.rs`

---

## Overview

Implement RAG (Retrieval-Augmented Generation) integration with document indexing, embedding generation, semantic retrieval, and context injection for enhancing LLM responses with knowledge base content.

---

## Implementation Steps

### Step 1: Create document indexing

- [ ] **Step 1.1: Write indexer**

```rust
// src/rag/index.rs
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub title: String,
    pub content: String,
    pub metadata: DocumentMetadata,
    pub embedding: Option<Vec<f32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub source: String,
    pub created_at: u64,
    pub modified_at: Option<u64>,
    pub tags: Vec<String>,
    pub category: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DocumentIndexer {
    base_path: PathBuf,
}

impl DocumentIndexer {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }

    pub fn index_file(&self, file_path: &Path) -> Result<Document> {
        let content = std::fs::read_to_string(file_path)
            .context("Failed to read file")?;

        let metadata = std::fs::metadata(file_path)
            .context("Failed to read file metadata")?;

        let modified = metadata.modified()
            .ok()
            .map(|t| t.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());

        let document = Document {
            id: uuid::Uuid::new_v4().to_string(),
            title: file_path.file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled")
                .to_string(),
            content,
            metadata: DocumentMetadata {
                source: file_path.to_string_lossy().to_string(),
                created_at: metadata.created()
                    .ok()
                    .map(|t| t.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs())
                    .unwrap_or(0),
                modified_at: modified,
                tags: vec![],
                category: None,
            },
            embedding: None,
        };

        // Save document to index
        self.save_document(&document)?;

        Ok(document)
    }

    pub fn index_directory(&self, dir_path: &Path) -> Result<Vec<Document>> {
        let mut documents = Vec::new();

        for entry in std::fs::read_dir(dir_path)
            .context("Failed to read directory")?
        {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                match self.index_file(&path) {
                    Ok(doc) => documents.push(doc),
                    Err(e) => {
                        eprintln!("Failed to index file {}: {}", path.display(), e);
                    }
                }
            }
        }

        Ok(documents)
    }

    fn save_document(&self, document: &Document) -> Result<()> {
        let doc_dir = self.base_path.join("documents");
        std::fs::create_dir_all(&doc_dir)
            .context("Failed to create documents directory")?;

        let doc_file = doc_dir.join(format!("{}.json", document.id));
        let json = serde_json::to_string_pretty(document)
            .context("Failed to serialize document")?;

        std::fs::write(doc_file, json)
            .context("Failed to write document")?;

        Ok(())
    }

    pub fn load_document(&self, id: &str) -> Result<Document> {
        let doc_file = self.base_path.join("documents").join(format!("{}.json", id));
        let content = std::fs::read_to_string(&doc_file)
            .context("Failed to read document")?;

        serde_json::from_str(&content)
            .context("Failed to deserialize document")
    }

    pub fn list_documents(&self) -> Result<Vec<Document>> {
        let doc_dir = self.base_path.join("documents");
        let mut documents = Vec::new();

        if !doc_dir.exists() {
            return Ok(documents);
        }

        for entry in std::fs::read_dir(&doc_dir)
            .context("Failed to read documents directory")?
        {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(doc) = self.load_document(
                    path.file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("")
                ) {
                    documents.push(doc);
                }
            }
        }

        Ok(documents)
    }

    pub fn delete_document(&self, id: &str) -> Result<()> {
        let doc_file = self.base_path.join("documents").join(format!("{}.json", id));
        std::fs::remove_file(doc_file)
            .context("Failed to delete document")?;
        Ok(())
    }
}
```

- [ ] **Step 1.2: Commit**

```bash
git add src/rag/index.rs
git commit -m "feat(rag): add document indexer"
```

---

### Step 2: Create embedding generation

- [ ] **Step 2.1: Write embedding generator**

```rust
// src/rag/embeddings.rs
use super::index::Document;
use super::super::backends::{LlmBackend, ChatRequest, Message};
use anyhow::{Context, Result};
use std::sync::Arc;

pub struct EmbeddingGenerator {
    backend: Arc<dyn LlmBackend>,
    model_name: String,
    max_context: usize,
}

impl EmbeddingGenerator {
    pub fn new(backend: Arc<dyn LlmBackend>, model_name: String, max_context: usize) -> Self {
        Self {
            backend,
            model_name,
            max_context,
        }
    }

    pub async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        // For now, use a simple hash-based embedding as placeholder
        // In production, this would call an actual embedding model
        self.simple_embedding(text)
    }

    pub async fn generate_document_embeddings(
        &self,
        documents: &mut Vec<Document>,
    ) -> Result<()> {
        for document in documents {
            let embedding = self.generate_embedding(&document.content).await?;
            document.embedding = Some(embedding);
        }
        Ok(())
    }

    pub async fn generate_batch_embeddings(
        &self,
        texts: &[String],
    ) -> Result<Vec<Vec<f32>>> {
        let mut embeddings = Vec::new();

        for text in texts {
            let embedding = self.generate_embedding(text).await?;
            embeddings.push(embedding);
        }

        Ok(embeddings)
    }

    fn simple_embedding(&self, text: &str) -> Result<Vec<f32>> {
        // Simple hash-based embedding as placeholder
        // In production, use actual embedding model (e.g., sentence-transformers)
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        const EMBEDDING_SIZE: usize = 384; // Common embedding size

        let mut embedding = Vec::with_capacity(EMBEDDING_SIZE);
        let mut hasher = DefaultHasher::new();

        let words: Vec<&str> = text.split_whitespace().collect();

        for i in 0..EMBEDDING_SIZE {
            if i < words.len() {
                words[i].hash(&mut hasher);
                let hash = hasher.finish();
                embedding.push((hash % 10000) as f32 / 10000.0);
                hasher = DefaultHasher::new();
            } else {
                embedding.push(0.0);
            }
        }

        Ok(embedding)
    }

    pub fn compute_similarity(&self, embedding1: &[f32], embedding2: &[f32]) -> f32 {
        // Cosine similarity
        let dot_product: f32 = embedding1.iter()
            .zip(embedding2.iter())
            .map(|(a, b)| a * b)
            .sum();

        let magnitude1: f32 = embedding1.iter().map(|x| x * x).sum::<f32>().sqrt();
        let magnitude2: f32 = embedding2.iter().map(|x| x * x).sum::<f32>().sqrt();

        if magnitude1 == 0.0 || magnitude2 == 0.0 {
            0.0
        } else {
            dot_product / (magnitude1 * magnitude2)
        }
    }
}
```

- [ ] **Step 2.2: Commit**

```bash
git add src/rag/embeddings.rs
git commit -m "feat(rag): add embedding generation with similarity"
```

---

### Step 3: Create retrieval

- [ ] **Step 3.1: Write retrieval**

```rust
// src/rag/retrieval.rs
use super::index::{Document, DocumentIndexer};
use super::embeddings::EmbeddingGenerator;
use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct RetrievalResult {
    pub document: Document,
    pub similarity: f32,
}

pub struct DocumentRetriever {
    indexer: DocumentIndexer,
    embedding_generator: EmbeddingGenerator,
}

impl DocumentRetriever {
    pub fn new(
        indexer: DocumentIndexer,
        embedding_generator: EmbeddingGenerator,
    ) -> Self {
        Self {
            indexer,
            embedding_generator,
        }
    }

    pub async fn retrieve(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<RetrievalResult>> {
        // Generate query embedding
        let query_embedding = self.embedding_generator.generate_embedding(query).await?;

        // Get all documents
        let documents = self.indexer.list_documents()?;

        // Filter documents with embeddings
        let mut results: Vec<RetrievalResult> = documents
            .into_iter()
            .filter_map(|doc| doc.embedding.as_ref().map(|emb| (doc, emb)))
            .map(|(doc, embedding)| {
                let similarity = self.embedding_generator.compute_similarity(
                    &query_embedding,
                    embedding,
                );
                RetrievalResult { document: doc, similarity }
            })
            .filter(|r| r.similarity > 0.0)
            .collect();

        // Sort by similarity (descending)
        results.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());

        // Take top results
        results.truncate(limit);

        Ok(results)
    }

    pub async fn retrieve_by_category(
        &self,
        query: &str,
        category: &str,
        limit: usize,
    ) -> Result<Vec<RetrievalResult>> {
        let mut results = self.retrieve(query, limit).await?;

        // Filter by category
        results.retain(|r| {
            r.document.metadata.category.as_ref().map_or(false, |c| c == category)
        });

        Ok(results)
    }

    pub async fn retrieve_by_tags(
        &self,
        query: &str,
        tags: &[String],
        limit: usize,
    ) -> Result<Vec<RetrievalResult>> {
        let mut results = self.retrieve(query, limit).await?;

        // Filter by tags (document must have all specified tags)
        results.retain(|r| {
            tags.iter().all(|tag| r.document.metadata.tags.contains(tag))
        });

        Ok(results)
    }
}
```

- [ ] **Step 3.2: Commit**

```bash
git add src/rag/retrieval.rs
git commit -m "feat(rag): add semantic retrieval"
```

---

### Step 4: Create context injection

- [ ] **Step 4.1: Write context injector**

```rust
// src/rag/context.rs
use super::retrieval::{RetrievalResult, DocumentRetriever};
use super::super::backends::Message;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct ContextInjection {
    max_context_length: usize,
    retrieval_limit: usize,
}

impl ContextInjection {
    pub fn new(max_context_length: usize, retrieval_limit: usize) -> Self {
        Self {
            max_context_length,
            retrieval_limit,
        }
    }

    pub async fn inject_context(
        &self,
        retriever: &DocumentRetriever,
        query: &str,
        messages: &mut Vec<Message>,
    ) -> Result<ContextMetadata> {
        // Retrieve relevant documents
        let results = retriever.retrieve(query, self.retrieval_limit).await?;

        if results.is_empty() {
            return Ok(ContextMetadata {
                documents_used: 0,
                context_injected: false,
            });
        }

        // Build context string
        let context = self.build_context_string(&results);

        // Insert context as system message
        let context_message = Message::System {
            content: format!(
                "You have access to the following relevant information:\n\n{}\n\nUse this information to answer the user's question.",
                context
            ),
        };

        messages.insert(0, context_message);

        Ok(ContextMetadata {
            documents_used: results.len(),
            context_injected: true,
        })
    }

    fn build_context_string(&self, results: &[RetrievalResult]) -> String {
        let mut context = String::new();

        for (i, result) in results.iter().enumerate() {
            context.push_str(&format!(
                "Document {} (similarity: {:.2}):\n",
                i + 1,
                result.similarity
            ));
            context.push_str(&format!("Title: {}\n", result.document.title));
            context.push_str(&format!("Source: {}\n", result.document.metadata.source));

            // Truncate content if needed
            let content = if result.document.content.len() > self.max_context_length {
                format!("{}...", &result.document.content[..self.max_context_length])
            } else {
                result.document.content.clone()
            };

            context.push_str(&format!("Content:\n{}\n\n", content));
        }

        context
    }
}

#[derive(Debug, Clone)]
pub struct ContextMetadata {
    pub documents_used: usize,
    pub context_injected: bool,
}
```

- [ ] **Step 4.2: Write module exports**

```rust
// src/rag/mod.rs
pub mod index;
pub mod embeddings;
pub mod retrieval;
pub mod context;

pub use index::{Document, DocumentMetadata, DocumentIndexer};
pub use embeddings::{EmbeddingGenerator};
pub use retrieval::{DocumentRetriever, RetrievalResult};
pub use context::{ContextInjection, ContextMetadata};
```

- [ ] **Step 4.3: Update lib.rs**

```rust
// src/lib.rs
pub mod rag;

pub use rag::*;
```

- [ ] **Step 4.4: Commit**

```bash
git add src/rag/context.rs src/rag/mod.rs src/lib.rs
git commit -m "feat(rag): add context injection"
```

---

### Step 5: Write tests

- [ ] **Step 5.1: Write integration tests**

```rust
// tests/rag/rag_test.rs
use whitt_execution_engine::rag::{
    DocumentIndexer, Document, DocumentMetadata, EmbeddingGenerator,
    DocumentRetriever, ContextInjection,
};
use whitt_execution_engine::backends::MockBackend;
use std::sync::Arc;
use tempfile::TempDir;

#[test]
fn test_index_document() {
    let temp_dir = TempDir::new().unwrap();
    let indexer = DocumentIndexer::new(temp_dir.path().to_path_buf());

    // Create a test file
    let test_file = temp_dir.path().join("test.txt");
    std::fs::write(&test_file, "Test content").unwrap();

    let document = indexer.index_file(&test_file).unwrap();

    assert_eq!(document.title, "test.txt");
    assert_eq!(document.content, "Test content");
    assert_eq!(document.metadata.source, test_file.to_string_lossy().to_string());
}

#[tokio::test]
async fn test_embedding_generation() {
    let backend = Arc::new(MockBackend::new("test"));
    let generator = EmbeddingGenerator::new(backend, "test-model".to_string(), 384);

    let embedding = generator.generate_embedding("Hello world").await.unwrap();

    assert!(!embedding.is_empty());
    assert_eq!(embedding.len(), 384);
}

#[tokio::test]
async fn test_retrieval() {
    let temp_dir = TempDir::new().unwrap();
    let indexer = DocumentIndexer::new(temp_dir.path().to_path_buf());
    let backend = Arc::new(MockBackend::new("test"));
    let generator = EmbeddingGenerator::new(backend, "test-model".to_string(), 384);

    // Index a document
    let test_file = temp_dir.path().join("test.txt");
    std::fs::write(&test_file, "Test document about AI").unwrap();

    let mut doc = indexer.index_file(&test_file).unwrap();
    generator.generate_document_embeddings(&mut vec![doc.clone()]).await.unwrap();
    indexer.save_document(&doc).unwrap();

    // Retrieve
    let retriever = DocumentRetriever::new(indexer, generator);
    let results = retriever.retrieve("AI", 10).await.unwrap();

    assert_eq!(results.len(), 1);
    assert!(results[0].similarity > 0.0);
}

#[tokio::test]
async fn test_context_injection() {
    let temp_dir = TempDir::new().unwrap();
    let indexer = DocumentIndexer::new(temp_dir.path().to_path_buf());
    let backend = Arc::new(MockBackend::new("test"));
    let generator = EmbeddingGenerator::new(backend, "test-model".to_string(), 384);
    let retriever = DocumentRetriever::new(indexer.clone(), generator);
    let injector = ContextInjection::new(1000, 5);

    let mut messages = vec![];

    let metadata = injector.inject_context(&retriever, "test query", &mut messages).await.unwrap();

    // Without documents, context shouldn't be injected
    assert_eq!(metadata.documents_used, 0);
    assert!(!metadata.context_injected);
}
```

- [ ] **Step 5.2: Commit**

```bash
git add tests/rag/rag_test.rs
git commit -m "test(rag): add RAG integration tests"
```

---

## Summary

This task implements RAG integration including:

1. **Document indexing** with file and directory support
2. **Embedding generation** (placeholder for actual model integration)
3. **Semantic retrieval** with similarity scoring
4. **Context injection** for enhancing LLM prompts
5. **Comprehensive tests** for all RAG components

**Key Features:**
- File and directory indexing
- Document metadata management
- Embedding generation (placeholder)
- Cosine similarity computation
- Top-k retrieval
- Category and tag filtering
- Context length management
- Automatic context injection into messages

**Future Enhancements:**
- Integration with actual embedding models (sentence-transformers)
- Vector database integration
- Incremental indexing
- Document chunking
- Hybrid retrieval (semantic + keyword)

**Next:** Task 12 - Self-Improvement Loop

---

## QA Cross-References

- **QA Criteria**: ['$qa_criteria']('$file')
- **Test Cases**: ['$test_case']('$file')
- **Schema Ref**: $schema_ref
