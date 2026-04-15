# Transpiler Feature Matrix (R0054-R0075)

## Overview

The transpiler feature matrix covers two complementary requirements for schema management in the AutoAgents SDK workflow transpiler:

- **R0054**: Schema definition system
- **R0075**: Schema documentation generator

Together, these ensure workflows are validated against a formal schema and that developers have accurate, up-to-date documentation.

---

## R0054: Schema Definition

### Purpose
Define canonical YAML grammar for workflows. Establish a formal schema specifying node types, fields, constraints, and versioning for YAML workflow definitions.

### Scope
- Schema design for JSON/YAML schema
- Schema versioning and migration support
- Validation rules for workflow definitions

### What It Enables
- **Validation**: Workflows validate against schema before compilation
- **Auto-completion**: IDE support for workflow authoring
- **Tool Integration**: Schema-driven code generation
- **Evolution**: Schema supports new versions via migrations

### How It Works
1. Schema defined in `schema.rs` with node types: `step`, `tool`, `model`, `verify`, `repair`, `review`, `effect`
2. Validation rules specified via constraints and type checking
3. Version field enables schema migrations
4. Invalid fields raise errors at parse time

### Examples

#### Valid Schema Entry
```rust
pub struct StepConfig {
    pub name: String,
    pub type: StepType,
    pub inputs: Vec<InputVar>,
    pub when: Option<WhenConfig>,
    pub tools: Option<Vec<ToolRef>>,
    // ... more fields
}
```

#### Invalid Workflow (Caught by Schema)
```yaml
steps:
  - name: "fetch_data"
    # Missing required field: type
    inputs: ["url"]
```

**Result**: Validation error at compile time, not runtime.

### Status
- Part of transpiler Layer 1 (Input & Parsing)
- Uses `serde-saphyr` for YAML parsing with garde validation
- Dependencies: `garde` for validation rules

---

## R0075: Schema Documentation Generator

### Purpose
Automatically produce reference documentation for each node type, field, example usage, and version changes. Keep docs in sync with schema.

### Scope
- Generate markdown or HTML documentation from schema definitions
- List each node type with field descriptions
- Show allowed values and examples
- Document version changes

### What It Enables
- **Developer Adoption**: Self-documenting schema reduces learning curve
- **Accuracy**: Docs auto-update after schema changes
- **Reduced Guesswork**: Clear field descriptions and allowed values

### How It Works
1. Schema definitions parsed by documentation generator
2. Each struct and field extracted
3. Doc comments (/// or /** */) rendered to markdown
4. Examples extracted from test fixtures
5. Output written to `docs/generated/` directory

### Examples

#### Generated Documentation Output
```markdown
# Step Configuration

## Fields

| Field | Type | Required | Description | Default |
|--------|------|-----------|-------------|----------|
| name | String | Yes | Unique identifier for this step | - |
| type | StepType | Yes | Type of step: simple, loop, parallel | simple |
| inputs | Vec<InputVar> | No | Input variables for this step | [] |
| when | Option<WhenConfig> | No | Conditional execution hooks | None |

## Example

```yaml
steps:
  - name: "fetch_data"
    type: "simple"
    inputs:
      - name: "url"
        value: "https://api.example.com/data"
```

## Version Changes

- v2.0: Added `when` field for conditional execution
- v1.0: Initial schema with basic step types
```

### Status
- Part of transpiler Layer 7 (CLI & Documentation)
- Implemented in `docs.rs` module
- Triggered after schema changes or manually via CLI

---

## Relationship: R0054 + R0075

The two requirements work together:

1. **R0054** defines the schema (authoritative source of truth)
2. **R0075** reads the schema and generates documentation
3. Developers write workflows using the schema (validated)
4. Developers reference generated docs for field usage
5. When schema changes, docs auto-regenerate

### Workflow

```
schema.rs (R0054) ──► docs.rs (R0075) ──► docs/generated/
                                                    │
                                                    ▼
                                            workflow authors
                                                    (validate + reference)
```

---

## Integration with Unified Schema

The transpiler's internal schema definition system aligns with the unified workflow schema v2.0:

- Unified schema defines **YAML structure** (what workflows look like)
- Transpiler schema defines **Rust types** (internal representation)
- Mapping between them handled by `schema.rs` validation layer
- Version changes in unified schema trigger transpiler schema migrations

### Example Mapping

| Unified Schema Field | Transpiler Schema Type | Rust Struct |
|--------------------|-----------------------|---------------|
| `steps` | `Vec<StepConfig>` | `struct StepConfig` |
| `type` | `StepType` | `enum StepType { Simple, Loop, Parallel }` |
| `inputs` | `Vec<InputVar>` | `struct InputVar` |
| `when` | `Option<WhenConfig>` | `struct WhenConfig` |

---

## Current Implementation Status

- **R0054**: Partially implemented (schema.rs exists, but not all node types defined)
- **R0075**: Not yet implemented (docs.rs module stub exists but generator logic incomplete)
- Both planned for **Phase 04: Quality Loops** (foundation work)

---

## Next Steps

1. Complete R0054: Define all node types from unified schema in Rust
2. Implement R0075: Build docs.rs generator with Askama templates
3. Add CLI subcommand: `transpiler generate-docs`
4. Integrate with schema migrations for version updates
5. Add golden tests for generated documentation
