# Test Specifications: Task 01 - Full-Text Search

## Mock Strategy

**Mock Tantivy Index:**
```rust
struct TestFullTextSearchIndex {
    index: Index,
    reader: IndexReader,
    writer: IndexWriter,
}

impl TestFullTextSearchIndex {
    fn new() -> Self {
        // Use RamDirectory for in-memory testing
        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("id", STRING | STORED);
        schema_builder.add_text_field("content", TEXT | STORED);
        schema_builder.add_text_field("title", TEXT | STORED);
        schema_builder.add_text_field("tags", TEXT | STORED);
        let schema = schema_builder.build();

        let index = Index::create_in_ram(schema.clone());
        let writer = index.writer(50_000_000).unwrap();
        let reader = index.reader_builder().try_into().unwrap();

        Self { index, reader, writer }
    }

    fn add_document(&mut self, id: &str, title: &str, content: &str, tags: &[String]) {
        let schema = self.index.schema();
        let id_field = schema.get_field("id").unwrap();
        let title_field = schema.get_field("title").unwrap();
        let content_field = schema.get_field("content").unwrap();
        let tags_field = schema.get_field("tags").unwrap();

        let mut doc = Document::new();
        doc.add_text(id_field, id);
        doc.add_text(title_field, title);
        doc.add_text(content_field, content);
        for tag in tags {
            doc.add_text(tags_field, tag);
        }

        self.writer.add_document(doc).unwrap();
        self.writer.commit().unwrap();
        self.reader.reload().unwrap();
    }
}
```

**Test Data Fixtures:**
```rust
fn create_test_documents() -> Vec<IndexedDocument> {
    vec![
        IndexedDocument {
            id: "doc1".to_string(),
            title: "Rust Programming Language".to_string(),
            content: "Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.".to_string(),
            tags: vec!["rust".to_string(), "programming".to_string(), "systems".to_string()],
        },
        IndexedDocument {
            id: "doc2".to_string(),
            title: "Memory Management in Rust".to_string(),
            content: "Rust's ownership system ensures memory safety without garbage collection. It uses borrowing and lifetimes.".to_string(),
            tags: vec!["rust".to_string(), "memory".to_string(), "ownership".to_string()],
        },
        IndexedDocument {
            id: "doc3".to_string(),
            title: "Search Algorithms Overview".to_string(),
            content: "Search algorithms find specific items in data structures. Common types include binary search, linear search, and depth-first search.".to_string(),
            tags: vec!["algorithms".to_string(), "search".to_string(), "data structures".to_string()],
        },
        IndexedDocument {
            id: "doc4".to_string(),
            title: "Concurrency Patterns".to_string(),
            content: "Rust provides fearless concurrency through its ownership system. Async/await syntax makes writing concurrent code easier.".to_string(),
            tags: vec!["rust".to_string(), "concurrency".to_string(), "async".to_string()],
        },
        IndexedDocument {
            id: "doc5".to_string(),
            title: "Testing Best Practices".to_string(),
            content: "Effective testing requires unit tests, integration tests, and property-based tests. Use testing frameworks like proptest for Rust.".to_string(),
            tags: vec!["testing".to_string(), "rust".to_string(), "quality".to_string()],
        },
    ]
}

fn create_large_test_documents(count: usize) -> Vec<IndexedDocument> {
    (0..count)
        .map(|i| IndexedDocument {
            id: format!("doc-{:08}", i),
            title: format!("Document {}", i),
            content: format!("This is the content of document {}. It contains some test data for search functionality testing. {}", i, "x".repeat(50)),
            tags: vec![format!("tag-{}", i % 10), format!("category-{}", i % 5)],
        })
        .collect()
}
```

## Test Cases

### Unit Tests

**Index Creation**
```rust
#[test]
fn test_create_tantivy_index() {
    let index = TestFullTextSearchIndex::new();
    let schema = index.index.schema();

    assert!(schema.get_field("id").is_ok());
    assert!(schema.get_field("content").is_ok());
    assert!(schema.get_field("title").is_ok());
    assert!(schema.get_field("tags").is_ok());
}

#[test]
fn test_index_schema_validation() {
    let index = TestFullTextSearchIndex::new();
    let schema = index.index.schema();

    let id_field = schema.get_field("id").unwrap();
    assert!(schema.is_stored(id_field));
    assert!(schema.is_string(id_field));

    let content_field = schema.get_field("content").unwrap();
    assert!(schema.is_text(content_field));
    assert!(schema.is_tokenized(content_field));
}
```

**Query Parsing**
```rust
#[test]
fn test_parse_simple_query() {
    let query = SearchQuery::new("rust programming");
    let parsed = FullTextSearchIndex::parse_query(query).unwrap();

    assert_eq!(parsed.terms().len(), 2);
    assert!(parsed.terms().contains(&"rust".to_string()));
    assert!(parsed.terms().contains(&"programming".to_string()));
}

#[test]
fn test_parse_phrase_query() {
    let query = SearchQuery::new("\"memory safety\"");
    let parsed = FullTextSearchIndex::parse_query(query).unwrap();

    assert!(parsed.is_phrase());
    assert_eq!(parsed.phrase().unwrap(), "memory safety");
}

#[test]
fn test_parse_boolean_and_query() {
    let query = SearchQuery::new("rust AND safety");
    let parsed = FullTextSearchIndex::parse_query(query).unwrap();

    assert!(parsed.is_boolean());
    assert_eq!(parsed.operator(), BooleanOperator::And);
}

#[test]
fn test_parse_boolean_or_query() {
    let query = SearchQuery::new("rust OR python");
    let parsed = FullTextSearchIndex::parse_query(query).unwrap();

    assert!(parsed.is_boolean());
    assert_eq!(parsed.operator(), BooleanOperator::Or);
}

#[test]
fn test_parse_not_query() {
    let query = SearchQuery::new("rust NOT python");
    let parsed = FullTextSearchIndex::parse_query(query).unwrap();

    assert!(parsed.has_negation());
    assert_eq!(parsed.terms().len(), 1);
}

#[test]
fn test_parse_field_specific_query() {
    let query = SearchQuery::new("title:rust");
    let parsed = FullTextSearchIndex::parse_query(query).unwrap();

    assert!(parsed.is_field_specific());
    assert_eq!(parsed.field().unwrap(), "title");
}
```

**Ranking**
```rust
#[test]
fn test_result_ranking_by_score() {
    let results = vec![
        SearchResult {
            id: "doc1".to_string(),
            score: 0.9,
            title: "High Score Document".to_string(),
            snippet: "...".to_string(),
        },
        SearchResult {
            id: "doc2".to_string(),
            score: 0.7,
            title: "Medium Score Document".to_string(),
            snippet: "...".to_string(),
        },
        SearchResult {
            id: "doc3".to_string(),
            score: 0.5,
            title: "Low Score Document".to_string(),
            snippet: "...".to_string(),
        },
    ];

    let ranked = FullTextSearchIndex::rank_results(results);
    assert_eq!(ranked[0].id, "doc1");
    assert_eq!(ranked[1].id, "doc2");
    assert_eq!(ranked[2].id, "doc3");
}

#[test]
fn test_bm25_score_calculation() {
    let doc_freq = 10;
    let corpus_size = 100;
    let term_freq = 5;
    let doc_length = 100;
    let avg_doc_length = 80;

    let score = FullTextSearchIndex::calculate_bm25(
        doc_freq,
        corpus_size,
        term_freq,
        doc_length,
        avg_doc_length,
    );

    assert!(score > 0.0);
    assert!(score.is_finite());
}

#[test]
fn test_tf_normalization() {
    let term_freq = 10;
    let normalized = FullTextSearchIndex::normalize_term_frequency(term_freq, 0.75);

    assert!(normalized > 0.0);
    assert!(normalized <= 1.0);
}
```

**Highlighting**
```rust
#[test]
fn test_snippet_generation() {
    let content = "Rust is a systems programming language that runs blazingly fast and prevents segfaults.";
    let terms = vec!["rust", "systems"];

    let snippet = FullTextSearchIndex::generate_snippet(content, terms, 150);
    assert!(snippet.contains("rust") || snippet.contains("systems"));
    assert!(snippet.len() <= 150);
}

#[test]
fn test_highlight_terms_in_snippet() {
    let snippet = "This is a rust example showing systems programming features.";
    let terms = vec!["rust", "systems"];

    let highlighted = FullTextSearchIndex::highlight_terms(snippet, terms);
    assert!(highlighted.contains("<mark>rust</mark>") || highlighted.contains("<mark>systems</mark>"));
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
        fn proptest_round_trip_document(
            id in "[a-z0-9-]{10,50}",
            title in "[a-zA-Z0-9 ]{10,100}",
            content in "[a-zA-Z0-9 ]{50,500}",
            num_tags in 0usize..=10usize
        ) {
            let mut index = TestFullTextSearchIndex::new();

            let tags: Vec<String> = (0..num_tags)
                .map(|i| format!("tag-{}", i))
                .collect();

            index.add_document(&id, &title, &content, &tags);

            // Search by ID
            let results = index.search(SearchQuery::new(&id), 10).unwrap();
            prop_assert_eq!(results.len(), 1);
            prop_assert_eq!(results[0].id, id);
        }

        #[test]
        fn proptest_search_consistency(
            query_term in "[a-z]{3,10}",
            num_docs in 10usize..=100usize
        ) {
            let mut index = TestFullTextSearchIndex::new();

            // Add documents
            for i in 0..num_docs {
                index.add_document(
                    &format!("doc-{}", i),
                    &format!("Title {}", i),
                    &format!("Content {} with {} repeated", i, query_term),
                    &[]
                );
            }

            // Search multiple times
            let query = SearchQuery::new(&query_term);
            let results1 = index.search(query.clone(), 100).unwrap();
            let results2 = index.search(query.clone(), 100).unwrap();

            // Results should be consistent
            prop_assert_eq!(results1.len(), results2.len());
            for (r1, r2) in results1.iter().zip(results2.iter()) {
                prop_assert_eq!(r1.id, r2.id);
                prop_assert_eq!(r1.score, r2.score);
            }
        }

        #[test]
        fn proptest_score_ordering(
            query_term in "[a-z]{3,10}",
            num_docs in 10usize..=50usize
        ) {
            let mut index = TestFullTextSearchIndex::new();

            for i in 0..num_docs {
                let content = if i % 3 == 0 {
                    format!("{} {} {}", query_term, query_term, query_term)
                } else if i % 3 == 1 {
                    format!("{} {}", query_term, query_term)
                } else {
                    format!("{}", query_term)
                };

                index.add_document(
                    &format!("doc-{}", i),
                    &format!("Doc {}", i),
                    &content,
                    &[]
                );
            }

            let results = index.search(SearchQuery::new(&query_term), num_docs).unwrap();

            // Verify scores are in descending order
            for window in results.windows(2) {
                prop_assert!(window[0].score >= window[1].score,
                    "Score ordering violated: {} > {}",
                    window[0].score, window[1].score);
            }
        }

        #[test]
        fn proptest_pagination_invariant(
            num_docs in 100usize..=1000usize,
            page_size in 10usize..=100usize
        ) {
            let mut index = TestFullTextSearchIndex::new();

            for i in 0..num_docs {
                index.add_document(
                    &format!("doc-{:08}", i),
                    &format!("Document {}", i),
                    &format!("Content {}", i),
                    &[]
                );
            }

            let query = SearchQuery::new("content");

            // Get all results
            let all_results = index.search(query.clone(), num_docs).unwrap();

            // Get paginated results
            let mut paginated_results = Vec::new();
            for page in 0..((num_docs + page_size - 1) / page_size) {
                let offset = page * page_size;
                let limit = page_size.min(num_docs - offset);
                let page_results = index.search(query.clone(), limit).unwrap();

                paginated_results.extend(page_results);
            }

            // Compare
            prop_assert_eq!(all_results.len(), paginated_results.len());
            prop_assert_eq!(all_results, paginated_results);
        }
    }
}
```

### Integration Tests

**Index and Search**
- [ ] **Step 1: Create test documents**
  ```rust
  let docs = create_test_documents();
  ```

- [ ] **Step 2: Index all documents**
  ```rust
  let mut index = TestFullTextSearchIndex::new();
  for doc in &docs {
      index.add_document(&doc.id, &doc.title, &doc.content, &doc.tags);
  }
  ```

- [ ] **Step 3: Search for terms**
  ```rust
  let query = SearchQuery::new("rust programming");
  let results = index.search(query, 10).unwrap();
  ```

- [ ] **Step 4: Verify results**
  ```rust
  assert!(!results.is_empty());
  assert!(results.iter().any(|r| r.id == "doc1"));
  ```

- [ ] **Step 5: Check ranking**
  ```rust
  for window in results.windows(2) {
      assert!(window[0].score >= window[1].score);
  }
  ```

**Boolean Operators**
```rust
#[tokio::test]
async fn test_and_operator() {
    let mut index = TestFullTextSearchIndex::new();
    index.add_document("d1", "Rust Programming", "Rust is great for programming", &[]);
    index.add_document("d2", "Python Programming", "Python is great for programming", &[]);
    index.add_document("d3", "Rust Safety", "Rust ensures safety", &[]);

    let query = SearchQuery::new("rust AND programming");
    let results = index.search(query, 10).unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "d1");
}

#[tokio::test]
async fn test_or_operator() {
    let mut index = TestFullTextSearchIndex::new();
    index.add_document("d1", "Rust", "Rust content", &[]);
    index.add_document("d2", "Python", "Python content", &[]);
    index.add_document("d3", "Go", "Go content", &[]);

    let query = SearchQuery::new("rust OR python");
    let results = index.search(query, 10).unwrap();

    assert_eq!(results.len(), 2);
    let ids: Vec<_> = results.iter().map(|r| r.id.clone()).collect();
    assert!(ids.contains(&"d1".to_string()));
    assert!(ids.contains(&"d2".to_string()));
}

#[tokio::test]
async fn test_not_operator() {
    let mut index = TestFullTextSearchIndex::new();
    index.add_document("d1", "Rust", "Rust content", &[]);
    index.add_document("d2", "Python", "Python content", &[]);
    index.add_document("d3", "Go", "Go content", &[]);

    let query = SearchQuery::new("rust NOT python");
    let results = index.search(query, 10).unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "d1");
}
```

**Phrase Queries**
```rust
#[tokio::test]
async fn test_exact_phrase_match() {
    let mut index = TestFullTextSearchIndex::new();
    index.add_document("d1", "Test", "This is a memory safety test", &[]);
    index.add_document("d2", "Test", "This is about memory and safety separately", &[]);

    let query = SearchQuery::new("\"memory safety\"");
    let results = index.search(query, 10).unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "d1");
}

#[tokio::test]
async fn test_phrase_highlighting() {
    let mut index = TestFullTextSearchIndex::new();
    index.add_document("d1", "Test", "This is a memory safety example", &[]);

    let query = SearchQuery::new("\"memory safety\"");
    let results = index.search(query, 10).unwrap();

    assert_eq!(results.len(), 1);
    assert!(results[0].snippet.contains("<mark>memory safety</mark>"));
}
```

**Pagination**
```rust
#[tokio::test]
async fn test_pagination_limit() {
    let mut index = TestFullTextSearchIndex::new();

    for i in 0..20 {
        index.add_document(
            &format!("d{}", i),
            &format!("Doc {}", i),
            "test content",
            &[]
        );
    }

    let query = SearchQuery::new("test");
    let results = index.search(query, 5).unwrap();

    assert_eq!(results.len(), 5);
}

#[tokio::test]
async fn test_pagination_offset() {
    let mut index = TestFullTextSearchIndex::new();

    for i in 0..20 {
        index.add_document(
            &format!("d{:02}", i),
            &format!("Doc {:02}", i),
            "test content",
            &[]
        );
    }

    let query = SearchQuery::new("test");
    let results1 = index.search(query.clone(), 5).unwrap();
    let results2 = index.search(query.clone(), 5).unwrap();

    // Get next page (offset 5)
    let results_page2 = index.search_with_offset(query.clone(), 5, 5).unwrap();

    assert_ne!(results1, results_page2);
    assert_eq!(results_page2.len(), 5);
}

#[tokio::test]
async fn test_pagination_beyond_results() {
    let mut index = TestFullTextSearchIndex::new();

    for i in 0..10 {
        index.add_document(&format!("d{}", i), "Test", "test content", &[]);
    }

    let query = SearchQuery::new("test");
    let results = index.search_with_offset(query, 100, 1000).unwrap();

    assert_eq!(results.len(), 0);
}
```

**Large Datasets**
```rust
#[tokio::test]
async fn test_large_dataset_indexing() {
    let mut index = TestFullTextSearchIndex::new();
    let docs = create_large_test_documents(100_000);

    let start = Instant::now();
    for doc in &docs {
        index.add_document(&doc.id, &doc.title, &doc.content, &doc.tags);
    }
    let indexing_time = start.elapsed();

    println!("Indexed {} documents in {:?}", docs.len(), indexing_time);
    assert!(indexing_time < Duration::from_secs(60), "Indexing too slow");
}

#[tokio::test]
async fn test_large_dataset_search() {
    let mut index = TestFullTextSearchIndex::new();
    let docs = create_large_test_documents(100_000);

    for doc in &docs {
        index.add_document(&doc.id, &doc.title, &doc.content, &doc.tags);
    }

    // Test search performance
    let start = Instant::now();
    let query = SearchQuery::new("test");
    let results = index.search(query, 100).unwrap();
    let search_time = start.elapsed();

    println!("Searched {} documents in {:?}", docs.len(), search_time);
    assert!(search_time < Duration::from_millis(500), "Search too slow");
    assert!(!results.is_empty());
}

#[tokio::test]
async fn test_concurrent_indexing() {
    let mut index = TestFullTextSearchIndex::new();
    let num_threads = 10;
    let docs_per_thread = 1000;

    let handles: Vec<_> = (0..num_threads)
        .map(|thread_id| {
            let mut index = index.clone();
            tokio::spawn(async move {
                for i in 0..docs_per_thread {
                    index.add_document(
                        &format!("doc-{}-{:06}", thread_id, i),
                        &format!("Title {}", i),
                        &format!("Content {}", i),
                        &[]
                    );
                }
            })
        })
        .collect();

    for handle in handles {
        handle.await.unwrap();
    }

    // Verify all documents indexed
    let query = SearchQuery::new("content");
    let results = index.search(query, num_threads * docs_per_thread).unwrap();
    assert_eq!(results.len(), num_threads * docs_per_thread);
}
```

**Edge Cases**
```rust
#[tokio::test]
async fn test_empty_query() {
    let mut index = TestFullTextSearchIndex::new();
    index.add_document("d1", "Test", "test content", &[]);

    let query = SearchQuery::new("");
    let results = index.search(query, 10).unwrap();

    assert_eq!(results.len(), 0);
}

#[tokio::test]
async fn test_no_results() {
    let mut index = TestFullTextSearchIndex::new();
    index.add_document("d1", "Test", "test content", &[]);

    let query = SearchQuery::new("nonexistent term");
    let results = index.search(query, 10).unwrap();

    assert_eq!(results.len(), 0);
}

#[tokio::test]
async fn test_special_characters_in_query() {
    let mut index = TestFullTextSearchIndex::new();
    index.add_document("d1", "Test", "test with special chars: @#$%", &[]);

    let query = SearchQuery::new("special chars");
    let results = index.search(query, 10).unwrap();

    assert_eq!(results.len(), 1);
}

#[tokio::test]
async fn test_unicode_search() {
    let mut index = TestFullTextSearchIndex::new();
    index.add_document("d1", "Test", "Programming in Rust is programming Rust™", &[]);

    let query = SearchQuery::new("programming");
    let results = index.search(query, 10).unwrap();

    assert_eq!(results.len(), 1);
}

#[tokio::test]
async fn test_case_insensitive_search() {
    let mut index = TestFullTextSearchIndex::new();
    index.add_document("d1", "Test", "Rust programming language", &[]);

    let lowercase = SearchQuery::new("rust");
    let uppercase = SearchQuery::new("RUST");
    let mixed = SearchQuery::new("RuSt");

    let results1 = index.search(lowercase, 10).unwrap();
    let results2 = index.search(uppercase, 10).unwrap();
    let results3 = index.search(mixed, 10).unwrap();

    assert_eq!(results1.len(), 1);
    assert_eq!(results2.len(), 1);
    assert_eq!(results3.len(), 1);
}
```

## Cargo Commands

**Run all fulltext search tests:**
```bash
cargo test --test fulltext_search -- --nocapture
```

**Run specific test:**
```bash
cargo test test_and_operator -- --nocapture
```

**Run property tests:**
```bash
cargo test proptest --test fulltext_search -- --nocapture
```

**Run with logging:**
```bash
RUST_LOG=debug cargo test --test fulltext_search -- --nocapture
```

**Expected output:**
```
running 35 tests
test tests::unit::test_create_tantivy_index ... ok
test tests::unit::test_index_schema_validation ... ok
test tests::unit::test_parse_simple_query ... ok
test tests::unit::test_parse_phrase_query ... ok
test tests::unit::test_parse_boolean_and_query ... ok
test tests::unit::test_parse_boolean_or_query ... ok
test tests::unit::test_parse_not_query ... ok
test tests::unit::test_parse_field_specific_query ... ok
test tests::unit::test_result_ranking_by_score ... ok
test tests::unit::test_bm25_score_calculation ... ok
test tests::unit::test_tf_normalization ... ok
test tests::unit::test_snippet_generation ... ok
test tests::unit::test_highlight_terms_in_snippet ... ok
test tests::integration::test_and_operator ... ok
test tests::integration::test_or_operator ... ok
test tests::integration::test_not_operator ... ok
test tests::integration::test_exact_phrase_match ... ok
test tests::integration::test_phrase_highlighting ... ok
test tests::integration::test_pagination_limit ... ok
test tests::integration::test_pagination_offset ... ok
test tests::integration::test_pagination_beyond_results ... ok
test tests::integration::test_large_dataset_indexing ... ok
test tests::integration::test_large_dataset_search ... ok
test tests::integration::test_concurrent_indexing ... ok
test tests::edge_cases::test_empty_query ... ok
test tests::edge_cases::test_no_results ... ok
test tests::edge_cases::test_special_characters_in_query ... ok
test tests::edge_cases::test_unicode_search ... ok
test tests::edge_cases::test_case_insensitive_search ... ok

test proptests::proptest_round_trip_document ... ok
test proptests::proptest_search_consistency ... ok
test proptests::proptest_score_ordering ... ok
test proptests::proptest_pagination_invariant ... ok

test result: ok. 35 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Indexed 100000 documents in 45.2s
Searched 100000 documents in 120ms
```

## Mock Dependencies

- `tantivy::RamDirectory`: In-memory index for fast testing
- `tantivy::Schema`: Schema definition with STRING, TEXT, STORED flags
- Pre-computed test data: Deterministic results for testing
- Mock schema: Minimal fields (id, content, title, tags) for testing
- `tokio::test`: Async test support
- `proptest`: Property-based testing with strategies
