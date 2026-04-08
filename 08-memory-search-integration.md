# Test Specifications: Task 08 - Memory & Search Integration

## Mock Strategy

**Mock Memory Operations:**
```rust
struct MockMemoryOperations {
    memories: Arc<RwLock<HashMap<MemoryId, StructuredMemory>>>,
}
```

**Mock Search Engine:**
```rust
struct MockSearchEngine {
    results: Vec<HybridResult>,
}
```

## Test Cases

### Unit Tests

**Tool Integration**
```rust
#[tokio::test]
async fn test_memory_search_tool() {
    let tool = MemorySearchTool::new(
        mock_ops,
        mock_search_engine,
        mock_provenance,
    );
    
    let results = tool.search("query".to_string(), 10).await.unwrap();
    assert!(!results.is_empty());
}
```

**CLI Commands**
```rust
#[test]
fn test_cli_search_command() {
    // Test CLI argument parsing
    // Verify command execution
}
```

### Integration Tests

**Tool Workflow**
- Use memory storage tool
- Use memory search tool
- Use memory retrieve tool
- Verify end-to-end workflow

**CLI Search**
- Execute CLI search command
- Verify output format
- Check results accuracy

**CLI Store**
- Execute CLI store command
- Verify storage
- Check retrieval

**CLI List**
- Execute CLI list command
- Verify pagination
- Check filtering

**CLI Provenance**
- Execute CLI provenance command
- Verify trace display
- Check timeline output

**CLI GC**
- Execute CLI preview command
- Verify candidates listed
- Execute GC command
- Verify deletion

**Workflow Engine Integration**
- Register tools as workflow nodes
- Execute workflow with tools
- Verify context injection
- Check provenance tracking

**Error Handling**
- Test tool failures
- Test CLI error messages
- Verify graceful degradation

## Mock Dependencies

- `MockMemoryOperations`: In-memory storage
- `MockSearchEngine`: Pre-defined results
- `MockProvenanceStore`: Trace recording
- Mock CLI args: Test command parsing
