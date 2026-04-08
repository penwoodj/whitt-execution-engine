# Task 01: Full-Text Search

**Estimated Time:** 1-2 weeks
**Dependencies:** Task 00 complete
**Priority:** CRITICAL (blocks semantic search and hybrid search)

## Overview

Implement full-text search capabilities using Tantivy indexing with query parsing, ranking, highlighting, and pagination for local memory content.

## Files

### Create
- `crates/search/Cargo.toml` - Search crate manifest with tantivy dependency
- `crates/search/src/lib.rs` - Public API exports
- `crates/search/src/fulltext.rs` - Tantivy full-text search implementation
- `crates/search/src/query_parser.rs` - Query parsing and syntax
- `crates/search/src/ranking.rs` - Scoring and ranking
- `crates/search/src/highlighting.rs` - Result highlighting
- `crates/search/src/error.rs` - Search error types

### Modify
- `Cargo.toml` - Add search workspace member
- `crates/memory/Cargo.toml` - Add agentsdk-search as dependency for indexing

### Test
- `crates/search/tests/integration_test.rs` - Integration tests
- `crates/search/src/fulltext.rs` - Unit tests embedded

---

## Step-by-Step Implementation

### Step 1: Create search crate structure

```bash
# Create crate directory
mkdir -p crates/search/src

# Initialize Cargo.toml with tantivy
cat > crates/search/Cargo.toml << 'EOF'
[package]
name = "agentsdk-search"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
tracing = "0.1"
tantivy = "0.22"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.6", features = ["v4", "serde"] }
agentsdk-memory = { path = "../memory" }
EOF
```

Run: `cargo check --package agentsdk-search`
Expected: SUCCESS

- [ ] **Step 1: Create search crate structure**

### Step 2: Write error types

Create `crates/search/src/error.rs`:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SearchError {
    #[error("Index error: {0}")]
    IndexError(String),

    #[error("Query parse error: {0}")]
    QueryParseError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Memory error: {0}")]
    MemoryError(#[from] agentsdk_memory::MemoryError),

    #[error("No results found")]
    NoResults,

    #[error("Invalid query: {0}")]
    InvalidQuery(String),
}
```

Run: `cargo check --package agentsdk-search`
Expected: SUCCESS

- [ ] **Step 2: Write error types**

### Step 3: Write full-text search index

Create `crates/search/src/fulltext.rs`:

```rust
use crate::error::SearchError;
use agentsdk_memory::{MemoryId, MemoryOperations, StructuredMemory, UnstructuredMemory, MemoryType};
use tantivy::{
    schema::*,
    Index, IndexWriter, ReloadPolicy, Searcher,
    query::QueryParser,
    collector::TopDocs,
    DocAddress,
};
use std::path::PathBuf;
use std::sync::Arc;

pub struct FullTextSearchIndex {
    index: Index,
    schema: Schema,
    index_path: PathBuf,
}

impl FullTextSearchIndex {
    pub fn new(base_path: impl AsRef<std::path::Path>) -> Result<Self, SearchError> {
        let index_path = base_path.as_ref().join("fulltext_index");

        let schema = Self::build_schema();
        let index = Index::create_in_dir(&index_path, schema.clone())
            .map_err(|e| SearchError::IndexError(format!("Failed to create index: {}", e)))?;

        Ok(Self {
            index,
            schema,
            index_path,
        })
    }

    fn build_schema() -> Schema {
        Schema::builder()
            .add_text_field("id", STRING | STORED)
            .add_text_field("content", TEXT | STORED)
            .add_text_field("title", TEXT | STORED)
            .add_text_field("tags", TEXT | STORED)
            .add_text_field("memory_type", STRING | STORED)
            .add_date_field("created_at", STORED)
            .add_date_field("updated_at", STORED)
            .add_u64_field("version", STORED)
            .build()
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    pub async fn index_memory(
        &self,
        memory: &StructuredMemory,
    ) -> Result<(), SearchError> {
        let mut writer = self.index.writer(50_000_000)
            .map_err(|e| SearchError::IndexError(format!("Failed to create writer: {}", e)))?;

        let id = self.schema.get_field("id").unwrap();
        let content = self.schema.get_field("content").unwrap();
        let title = self.schema.get_field("title").unwrap();
        let tags = self.schema.get_field("tags").unwrap();
        let memory_type = self.schema.get_field("memory_type").unwrap();
        let created_at = self.schema.get_field("created_at").unwrap();
        let updated_at = self.schema.get_field("updated_at").unwrap();
        let version = self.schema.get_field("version").unwrap();

        let mut doc = tantivy::Document::new();
        doc.add_text(id, memory.id.as_str());
        doc.add_text(content, &memory.data.to_string());
        doc.add_text(title, format!("memory-{}", memory.id.as_str()));
        doc.add_text(tags, &memory.tags.join(" "));
        doc.add_text(memory_type, "structured");

        let created_ts = tantivy::DateTime::from_timestamp_millis(
            memory.created_at.timestamp_millis()
        ).unwrap();
        doc.add_date(created_at, created_ts);

        let updated_ts = tantivy::DateTime::from_timestamp_millis(
            memory.updated_at.timestamp_millis()
        ).unwrap();
        doc.add_date(updated_at, updated_ts);

        doc.add_u64(version, memory.version);

        writer.add_document(doc)?;
        writer.commit()
            .map_err(|e| SearchError::IndexError(format!("Failed to commit: {}", e)))?;

        Ok(())
    }

    pub async fn index_unstructured_memory(
        &self,
        memory: &UnstructuredMemory,
    ) -> Result<(), SearchError> {
        let mut writer = self.index.writer(50_000_000)
            .map_err(|e| SearchError::IndexError(format!("Failed to create writer: {}", e)))?;

        let id = self.schema.get_field("id").unwrap();
        let content = self.schema.get_field("content").unwrap();
        let title = self.schema.get_field("title").unwrap();
        let tags = self.schema.get_field("tags").unwrap();
        let memory_type = self.schema.get_field("memory_type").unwrap();
        let created_at = self.schema.get_field("created_at").unwrap();
        let updated_at = self.schema.get_field("updated_at").unwrap();
        let version = self.schema.get_field("version").unwrap();

        let mut doc = tantivy::Document::new();
        doc.add_text(id, memory.id.as_str());
        doc.add_text(content, &memory.content);
        doc.add_text(title, format!("unstructured-{}", memory.id.as_str()));
        doc.add_text(tags, &memory.tags.join(" "));
        doc.add_text(memory_type, "unstructured");

        let created_ts = tantivy::DateTime::from_timestamp_millis(
            memory.created_at.timestamp_millis()
        ).unwrap();
        doc.add_date(created_at, created_ts);

        let updated_ts = tantivy::DateTime::from_timestamp_millis(
            memory.updated_at.timestamp_millis()
        ).unwrap();
        doc.add_date(updated_at, updated_ts);

        doc.add_u64(version, memory.version);

        writer.add_document(doc)?;
        writer.commit()
            .map_err(|e| SearchError::IndexError(format!("Failed to commit: {}", e)))?;

        Ok(())
    }

    pub fn create_query_parser(&self) -> QueryParser {
        let content = self.schema.get_field("content").unwrap();
        let title = self.schema.get_field("title").unwrap();
        let tags = self.schema.get_field("tags").unwrap();

        QueryParser::for_index(
            &self.index,
            vec![content, title, tags],
        )
    }

    pub fn searcher(&self) -> Searcher {
        let reader = self.index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()
            .unwrap();

        reader.searcher()
    }
}
```

Run: `cargo check --package agentsdk-search`
Expected: SUCCESS

- [ ] **Step 3: Write full-text search index**

### Step 4: Write query parser

Create `crates/search/src/query_parser.rs`:

```rust
use crate::error::SearchError;
use crate::fulltext::FullTextSearchIndex;
use tantivy::{query::Query, QueryParser};

pub struct SearchQuery {
    pub raw: String,
    pub limit: usize,
    pub offset: usize,
    pub filters: Vec<SearchFilter>,
}

#[derive(Debug, Clone)]
pub struct SearchFilter {
    pub field: String,
    pub value: String,
    pub operator: FilterOperator,
}

#[derive(Debug, Clone)]
pub enum FilterOperator {
    Equals,
    Contains,
    GreaterThan,
    LessThan,
}

impl SearchQuery {
    pub fn new(raw: String) -> Self {
        Self {
            raw,
            limit: 10,
            offset: 0,
            filters: vec![],
        }
    }

    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    pub fn with_filter(mut self, filter: SearchFilter) -> Self {
        self.filters.push(filter);
        self
    }

    pub fn parse(&self, index: &FullTextSearchIndex) -> Result<Box<dyn Query>, SearchError> {
        let parser = index.create_query_parser();

        // Parse the raw query
        let query = parser.parse_query(&self.raw)
            .map_err(|e| SearchError::QueryParseError(format!("Invalid query: {}", e)))?;

        Ok(query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_construction() {
        let query = SearchQuery::new("test query".to_string())
            .with_limit(20)
            .with_offset(10)
            .with_filter(SearchFilter {
                field: "tags".to_string(),
                value: "important".to_string(),
                operator: FilterOperator::Equals,
            });

        assert_eq!(query.raw, "test query");
        assert_eq!(query.limit, 20);
        assert_eq!(query.offset, 10);
        assert_eq!(query.filters.len(), 1);
    }
}
```

Run: `cargo check --package agentsdk-search`
Expected: SUCCESS

- [ ] **Step 4: Write query parser**

### Step 5: Write ranking

Create `crates/search/src/ranking.rs`:

```rust
use crate::fulltext::FullTextSearchIndex;
use tantivy::{DocAddress, Score, Searcher};

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub doc_address: DocAddress,
    pub score: Score,
    pub memory_id: String,
    pub snippet: String,
}

pub struct SearchResultCollector {
    pub limit: usize,
}

impl SearchResultCollector {
    pub fn new(limit: usize) -> Self {
        Self { limit }
    }
}

pub fn rank_results(
    searcher: &Searcher,
    top_docs: Vec<(Score, DocAddress)>,
    schema: &tantivy::Schema,
) -> Vec<SearchResult> {
    top_docs
        .into_iter()
        .map(|(score, doc_address)| {
            let doc = searcher.doc(doc_address).unwrap();

            let id_field = schema.get_field("id").unwrap();
            let memory_id = doc
                .get_first(id_field)
                .and_then(|val| val.as_str())
                .unwrap_or("")
                .to_string();

            let content_field = schema.get_field("content").unwrap();
            let content = doc
                .get_first(content_field)
                .and_then(|val| val.as_str())
                .unwrap_or("");

            let snippet = if content.len() > 200 {
                format!("{}...", &content[..200])
            } else {
                content.to_string()
            };

            SearchResult {
                doc_address,
                score,
                memory_id,
                snippet,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_results_empty() {
        let results = vec![];
        let schema = tantivy::Schema::builder()
            .add_text_field("id", tantivy::STRING | tantivy::STORED)
            .add_text_field("content", tantivy::TEXT | tantivy::STORED)
            .build();

        let ranked = rank_results(&schema, &results, &schema);
        assert!(ranked.is_empty());
    }
}
```

Run: `cargo check --package agentsdk-search`
Expected: SUCCESS

- [ ] **Step 5: Write ranking**

### Step 6: Write highlighting

Create `crates/search/src/highlighting.rs`:

```rust
use crate::fulltext::FullTextSearchIndex;
use crate::rankering::SearchResult;
use tantivy::{Searcher, DocAddress};

pub struct Highlighter {
    pub pre_tag: String,
    pub post_tag: String,
}

impl Highlighter {
    pub fn new() -> Self {
        Self {
            pre_tag: "<mark>".to_string(),
            post_tag: "</mark>".to_string(),
        }
    }

    pub fn with_tags(pre_tag: String, post_tag: String) -> Self {
        Self { pre_tag, post_tag }
    }

    pub fn highlight_result(
        &self,
        searcher: &Searcher,
        doc_address: &DocAddress,
        schema: &tantivy::Schema,
        query_terms: &[String],
    ) -> String {
        let doc = searcher.doc(*doc_address).unwrap();

        let content_field = schema.get_field("content").unwrap();
        let content = doc
            .get_first(content_field)
            .and_then(|val| val.as_str())
            .unwrap_or("");

        // Simple highlighting: wrap matching terms
        let mut highlighted = content.to_string();

        for term in query_terms {
            let term_lower = term.to_lowercase();
            if let Some(pos) = highlighted.to_lowercase().find(&term_lower) {
                let before = &highlighted[..pos];
                let match_text = &highlighted[pos..pos + term.len()];
                let after = &highlighted[pos + term.len()..];
                highlighted = format!(
                    "{}{}{}{}",
                    before, self.pre_tag, match_text, self.post_tag
                );
            }
        }

        highlighted
    }
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::new()
    }
}
```

Run: `cargo check --package agentsdk-search`
Expected: SUCCESS

- [ ] **Step 6: Write highlighting**

### Step 7: Write lib.rs exports

Create `crates/search/src/lib.rs`:

```rust
pub mod error;
pub mod fulltext;
pub mod query_parser;
pub mod ranking;
pub mod highlighting;

pub use error::SearchError;
pub use fulltext::FullTextSearchIndex;
pub use query_parser::{SearchQuery, SearchFilter, FilterOperator};
pub use ranking::{SearchResult, SearchResultCollector, rank_results};
pub use highlighting::Highlighter;
```

Run: `cargo check --package agentsdk-search`
Expected: SUCCESS

- [ ] **Step 7: Write lib.rs exports**

### Step 8: Add to workspace and memory dependencies

Modify `Cargo.toml` in workspace root:

Add to `[workspace.members]`:

```toml
members = [
    # ... existing members ...
    "crates/search",
]
```

Modify `crates/memory/Cargo.toml`:

```toml
[dependencies]
# ... existing dependencies ...
agentsdk-search = { path = "../search" }
```

Run: `cargo check --workspace`
Expected: SUCCESS

- [ ] **Step 8: Add to workspace and memory dependencies**

### Step 9: Write unit tests

Add tests to `crates/search/src/fulltext.rs` (bottom of file):

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_index_creation() {
        let temp_dir = TempDir::new().unwrap();
        let index = FullTextSearchIndex::new(temp_dir.path()).unwrap();

        assert!(index.index_path.exists());
    }

    #[tokio::test]
    async fn test_index_schema() {
        let temp_dir = TempDir::new().unwrap();
        let index = FullTextSearchIndex::new(temp_dir.path()).unwrap();
        let schema = index.schema();

        assert!(schema.get_field("id").is_ok());
        assert!(schema.get_field("content").is_ok());
        assert!(schema.get_field("title").is_ok());
        assert!(schema.get_field("tags").is_ok());
    }
}
```

Add `tempfile` to `crates/search/Cargo.toml`:

```toml
[dev-dependencies]
tempfile = "3.8"
```

Run: `cargo test --package agentsdk-search`
Expected: All tests PASS

- [ ] **Step 9: Write unit tests**

### Step 10: Write integration tests

Create `crates/search/tests/integration_test.rs`:

```rust
use agentsdk_search::{FullTextSearchIndex, SearchQuery};
use agentsdk_memory::{MemoryOperations, MemoryId};
use tempfile::TempDir;

#[tokio::test]
async fn test_full_search_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let index = FullTextSearchIndex::new(temp_dir.path()).unwrap();
    let ops = MemoryOperations::new(temp_dir.path());
    ops.initialize().await.unwrap();

    // Create a memory
    let id = ops
        .create_structured(
            serde_json::json!({"title": "Test Document", "content": "This is a test content for search"}),
            vec!["search".to_string(), "test".to_string()],
            std::collections::HashMap::new(),
        )
        .await
        .unwrap();

    // Index it
    let memory = ops.get_structured(&id, None).await.unwrap();
    index.index_memory(&memory).await.unwrap();

    // Search for it
    let query = SearchQuery::new("test content".to_string());
    let parsed = query.parse(&index).unwrap();

    let searcher = index.searcher();
    let top_docs = searcher
        .search(&parsed, &tantivy::collector::TopDocs::with_limit(10))
        .unwrap();

    assert!(!top_docs.is_empty());
}
```

Run: `cargo test --package agentsdk-search --test integration_test`
Expected: All tests PASS

- [ ] **Step 10: Write integration tests**

### Step 11: Commit

```bash
git add crates/search/ Cargo.toml
git commit -m "feat(Phase5-Task01): implement full-text search with Tantivy indexing, query parsing, ranking, and highlighting"
```

- [ ] **Step 11: Commit**

---

## Validation Criteria

See [validation/01-fulltext-search.md](../validation/01-fulltext-search.md)

## Test Specifications

See [tests/01-fulltext-search.md](../tests/01-fulltext-search.md)
