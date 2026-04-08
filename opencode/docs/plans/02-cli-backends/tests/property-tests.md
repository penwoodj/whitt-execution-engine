# Property-Based Tests Specification

This document specifies the property-based tests required for Phase 2. Property-based testing verifies invariants and properties across many inputs.

---

## Test Organization

```
tests/property/
├── streaming_parser_properties.rs
├── permission_logic_properties.rs
├── backend_selection_properties.rs
└── similarity_computation_properties.rs
```

---

## Streaming Parser Properties

### SSE Parser Properties
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn prop_sse_parser_preserves_chunks(
        chunks in prop::collection::vec(any::<String>(), 0..10)
    ) {
        // Property: SSE parser preserves all valid chunks
    }

    #[test]
    fn prop_sse_parser_handles_empty_stream() {
        // Property: Empty SSE stream produces no chunks
    }

    #[test]
    fn prop_sse_parser_done_marker_terminates(
        prefix in prop::collection::vec(any::<String>(), 0..5)
    ) {
        // Property: [DONE] marker terminates stream
    }

    #[test]
    fn prop_sse_parser_robust_to_whitespace(
        data in any::<String>(),
        whitespace in "[ \t\n\r]+"
    ) {
        // Property: Whitespace between chunks doesn't break parsing
    }
}
```

### NDJSON Parser Properties
```rust
proptest! {
    #[test]
    fn prop_ndjson_parser_preserves_objects(
        objects in prop::collection::vec(any::<serde_json::Value>(), 0..10)
    ) {
        // Property: NDJSON parser preserves all valid JSON objects
    }

    #[test]
    fn prop_ndjson_parser_ignores_empty_lines(
        objects in prop::collection::vec(any::<serde_json::Value>(), 0..5)
    ) {
        // Property: Empty lines are ignored
    }

    #[test]
    fn prop_ndjson_parser_malformed_handling(
        valid_objects in prop::collection::vec(any::<serde_json::Value>(), 1..5),
        invalid_json in "[^\\n]*\\{[^}]*"
    ) {
        // Property: Malformed JSON objects don't crash parser
    }
}
```

---

## Permission Logic Properties

### Allow/Deny Logic
```rust
proptest! {
    #[test]
    fn prop_allow_deny_list_exclusivity(
        allow_list in prop::collection::hash_set("[a-z]+", 0..5),
        deny_list in prop::collection::hash_set("[a-z]+", 0..5),
        tool_name in "[a-z]+"
    ) {
        // Property: Tool in both allow and deny lists is denied
    }

    #[test]
    fn prop_default_policy_consistency(
        policy in prop::sample::select(vec!["allow", "deny"]),
        allow_list in prop::collection::hash_set("[a-z]+", 0..5),
        tool_name in "[a-z]+"
    ) {
        // Property: Default policy determines outcome when lists are empty
    }

    #[test]
    fn prop_step_permissions_override(
        global_allow in prop::collection::hash_set("[a-z]+", 0..5),
        global_deny in prop::collection::hash_set("[a-z]+", 0..5),
        step_allow in prop::collection::hash_set("[a-z]+", 0..5),
        step_deny in prop::collection::hash_set("[a-z]+", 0..5),
        tool_name in "[a-z]+"
    ) {
        // Property: Step permissions override global permissions
    }
}
```

### Guardrails Properties
```rust
proptest! {
    #[test]
    fn prop_path_traversal_detection(
        base_path in "[a-zA-Z0-9/_-]+",
        traversal_depth in 1..10
    ) {
        // Property: Path traversal detected for any depth
    }

    #[test]
    fn prop_command_injection_detection(
        command in "[a-z]+",
        injection in proptest::sample::select(vec![
            "; ", " && ", " || ", "| ", "`", "$(", "evalue"
        ])
    ) {
        // Property: All injection patterns are detected
    }

    #[test]
    fn prop_allowed_paths_enforcement(
        allowed_paths in prop::collection::vec("[a-zA-Z0-9/_-]+", 1..3),
        test_path in "[a-zA-Z0-9/_-]+"
    ) {
        // Property: Paths outside allowed list are rejected
    }
}
```

---

## Backend Selection Properties

### Fallback Chain Properties
```rust
proptest! {
    #[test]
    fn prop_fallback_terminates(
        backends in prop::collection::vec("[a-z]+", 1..5),
        unhealthy_count in 0..5
    ) {
        // Property: Fallback chain always terminates
    }

    #[test]
    fn prop_fallback_exhaustion(
        backends in prop::collection::vec("[a-z]+", 1..3)
    ) {
        // Property: All backends unhealthy triggers error
    }

    #[test]
    fn prop_backend_selection_deterministic(
        config in any::<String>(),
        backends in prop::collection::vec("[a-z]+", 1..5)
    ) {
        // Property: Backend selection is deterministic for same config
    }
}
```

---

## Similarity Computation Properties

### Cosine Similarity Properties
```rust
proptest! {
    #[test]
    fn prop_cosine_similarity_range(
        vec1 in prop::collection::vec(-1.0_f32..1.0_f32, 10..100),
        vec2 in prop::collection::vec(-1.0_f32..1.0_f32, 10..100)
    ) {
        // Property: Cosine similarity is always in [-1, 1]
    }

    #[test]
    fn prop_cosine_similarity_symmetric(
        vec in prop::collection::vec(-1.0_f32..1.0_f32, 10..100)
    ) {
        // Property: Cosine similarity is symmetric
    }

    #[test]
    fn prop_cosine_similarity_identity(
        vec in prop::collection::vec(-1.0_f32..1.0_f32, 10..100)
    ) {
        // Property: Cosine similarity of identical vectors is 1
    }

    #[test]
    fn prop_cosine_similarity_orthogonal(
        vec1 in prop::collection::vec(-1.0_f32..1.0_f32, 10..100)
    ) {
        // Property: Orthogonal vectors have similarity 0
    }
}
```

---

## Workflow Graph Properties

### Cycle Detection Properties
```rust
proptest! {
    #[test]
    fn prop_cycle_detection_completeness(
        nodes in prop::collection::vec("[a-z]+", 1..10),
        edges in prop::collection::vec(
            (prop::sample::vec("[a-z]+", 1..2), 1..10usize),
            0..20
        )
    ) {
        // Property: Cycle detection finds all cycles
    }

    #[test]
    fn prop_topological_sort_dag(
        nodes in prop::collection::vec("[a-z]+", 1..10)
    ) {
        // Property: Topological sort succeeds for DAGs
    }
}
```

---

## Running Property Tests

```bash
# Run all property tests
cargo test --test '*_properties'

# Run specific property test
cargo test prop_sse_parser_preserves_chunks --test '*_properties'

# Run with custom test count
cargo test prop_* --test '*_properties' -- --test-threads=1

# Run with maximum test cases (default: 100)
cargo test prop_* --test '*_properties' -- --max-cases=1000

# Run with seed for reproducibility
cargo test prop_* --test '*_properties' -- --seed 0x12345678
```

---

## Property Test Goals

| Component | Property Count | Coverage |
|----------|----------------|----------|
| Streaming Parsers | 8 | 90% |
| Permission Logic | 9 | 95% |
| Backend Selection | 3 | 85% |
| Similarity Computation | 4 | 95% |
| Workflow Graph | 2 | 80% |
| **Total** | **26** | **88%** |
