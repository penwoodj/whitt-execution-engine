# Phase 02: CLI & Backends - Validation Criteria

**Phase Focus:** Command-line interface, model backends, tool execution, RAG, sub-workflows
**Entry Criteria:** Phases 00 and 01 complete
**Estimated Duration:** 4-5 weeks
**Blocking for:** Phases 03, 04, 05, 06, 07, 08

---

## Phase Overview

Phase 02 implements the user-facing CLI and backend connectivity. This phase enables workflows to interact with LLM backends, execute tools, perform RAG operations, and compose sub-workflows. The CLI provides commands for workflow management, configuration, and debugging.

**Critical Success Factors:**
1. CLI parses all commands without crashing
2. All 4 backends connect and stream responses
3. Tool permissions enforce security boundaries
4. Code generation produces compilable Rust
5. RAG indexes and retrieves relevant documents
6. Sub-workflows compose correctly with variable passing

---

## Phase Entry Criteria

**Status:** Must be verified before starting Phase 02

**Verification Commands:**
```bash
# Verify Phase 00 exit
cargo test --test phase_00_integration -- --test-threads=1

# Verify Phase 01 exit
cargo test --test phase_01_integration -- --test-threads=1

# Verify Phase 01 schema coverage
cargo run --bin schema_audit -- --phase 1 --output phase_01_coverage.md
# Expected: 100% coverage

# Verify Phase 01 ADR compliance
cargo run --bin adr_compliance -- --phase 1
# Expected: All constraints satisfied
```

**Prerequisites:**
- [ ] Phase 00 exit criteria verified (all 7 layers)
- [ ] Phase 01 exit criteria verified (all 7 layers)
- [ ] Phase 00 schema coverage 100%
- [ ] Phase 01 schema coverage 100%
- [ ] Phase 00 ADR compliance verified
- [ ] Phase 01 ADR compliance verified
- [ ] Cross-phase regression clean (Phases 00+01)
- [ ] Test infrastructure ready for backend testing
- [ ] Mock backends prepared for integration testing

**Blocking Violations:**
- Unresolved Phase 00 or 01 failures
- Schema coverage < 100% for either phase
- ADR compliance violations
- Cross-phase regression detected

---

## CLI Command Parsing

**Requirement:** CLI parses all commands correctly

### CLI Commands

#### 1. Workflow Commands

```bash
agentsdk run <workflow.yaml>
agentsdk validate <workflow.yaml>
agentsdk list
agentsdk status <workflow_id>
agentsdk cancel <workflow_id>
```

#### 2. Queue Commands

```bash
agentsdk queue list
agentsdk queue enqueue <workflow.yaml>
agentsdk queue cancel <workflow_id>
agentsdk queue clear
```

#### 3. Scheduler Commands

```bash
agentsdk scheduler start
agentsdk scheduler stop
agentsdk scheduler status
agentsdk scheduler config <config.yaml>
```

#### 4. Configuration Commands

```bash
agentsdk config init
agentsdk config get <key>
agentsdk config set <key> <value>
agentsdk config list
```

#### 5. Debug Commands

```bash
agentsdk debug <workflow.yaml> --dry-run
agentsdk debug <workflow.yaml> --verbose
agentsdk debug <workflow.yaml> --breakpoint <step_id>
```

### Verification Commands

```bash
# Test all CLI commands
cargo run --bin agentsdk -- --help
cargo run --bin agentsdk -- run --help
cargo run --bin agentsdk -- validate --help
cargo run --bin agentsdk -- queue --help
cargo run --bin agentsdk -- scheduler --help
cargo run --bin agentsdk -- config --help
cargo run --bin agentsdk -- debug --help

# Test command execution
cargo run --bin agentsdk -- run examples/workflows/simple.yaml
cargo run --bin agentsdk -- validate examples/workflows/simple.yaml
cargo run --bin agentsdk -- queue list
```

### Pass Criteria

- [ ] All CLI commands parse without crashing
- [ ] Help text displays correctly for all commands
- [ ] Commands execute successfully with valid inputs
- [ ] Invalid inputs produce clear error messages
- [ ] Exit codes correct (0 for success, != 0 for error)

### Evidence Required

- CLI parsing test results
- Help text screenshots
- Command execution logs
- Error message samples

---

## Backend Connectivity

**Requirement:** All 4 backends connect and stream responses

### Backend Types

#### 1. OpenAI Backend

**Configuration:**
```yaml
backend:
  type: openai
  api_key: ${OPENAI_API_KEY}
  base_url: https://api.openai.com/v1
  model: gpt-4
  timeout: 30
```

**Expected Behavior:**
- Connect to OpenAI API
- Send request with prompt
- Stream response tokens
- Handle rate limits

#### 2. Anthropic Backend

**Configuration:**
```yaml
backend:
  type: anthropic
  api_key: ${ANTHROPIC_API_KEY}
  base_url: https://api.anthropic.com/v1
  model: claude-3-opus
  timeout: 60
```

**Expected Behavior:**
- Connect to Anthropic API
- Send request with prompt
- Stream response tokens
- Handle rate limits

#### 3. Local LLM Backend

**Configuration:**
```yaml
backend:
  type: local
  model_path: /path/to/model.gguf
  backend_type: llama.cpp
  timeout: 120
  context_length: 4096
```

**Expected Behavior:**
- Load local model
- Send request with prompt
- Stream response tokens
- Handle memory constraints

#### 4. Custom Backend

**Configuration:**
```yaml
backend:
  type: custom
  endpoint: http://localhost:8080/v1/generate
  auth_token: ${CUSTOM_AUTH_TOKEN}
  timeout: 30
  stream: true
```

**Expected Behavior:**
- Connect to custom endpoint
- Send request with prompt
- Stream response tokens
- Handle custom response format

### Verification Commands

```bash
# Test backend connections
cargo test --lib backends::tests::openai_connection
cargo test --lib backends::tests::anthropic_connection
cargo test --lib backends::tests::local_llm_connection
cargo test --lib backends::tests::custom_backend_connection

# Test response streaming
cargo test --lib backends::tests::openai_streaming
cargo test --lib backends::tests::anthropic_streaming
cargo test --lib backends::tests::local_llm_streaming
cargo test --lib backends::tests::custom_backend_streaming

# Test error handling
cargo test --lib backends::tests::rate_limit_handling
cargo test --lib backends::tests::timeout_handling
cargo test --lib backends::tests::connection_failure
```

### Pass Criteria

- [ ] All 4 backends connect successfully
- [ ] All backends stream responses
- [ ] Rate limits handled correctly
- [ ] Timeouts handled correctly
- [ ] Connection failures handled gracefully

### Evidence Required

- Backend connection test results
- Streaming test logs
- Error handling test results
- Backend-specific metrics

---

## Tool Permissions

**Requirement:** Tool permissions enforce security boundaries

### Permission Levels

#### 1. Allow (no restriction)

```yaml
tool:
  name: safe_calculation
  type: calculation
  permission: allow
```

**Expected Behavior:** Tool executes without restriction

#### 2. Require Approval (human gate)

```yaml
tool:
  name: file_delete
  type: file_operation
  permission: require_approval
  approval_timeout_ms: 300000
```

**Expected Behavior:** Tool blocks until human approval

#### 3. Deny (blocked)

```yaml
tool:
  name: dangerous_operation
  type: system
  permission: deny
```

**Expected Behavior:** Tool execution blocked permanently

### Permission Rules

1. **File Operations:** Require approval for delete, overwrite
2. **Network Operations:** Require approval for external API calls
3. **System Operations:** Deny process management, system config
4. **Resource Operations:** Allow with limits (CPU, memory)

### Verification Commands

```bash
# Test permission enforcement
cargo test --lib permissions::tests::allow_permission
cargo test --lib permissions::tests::require_approval_permission
cargo test --lib permissions::tests::deny_permission

# Test permission escalation
cargo test --lib permissions::tests::permission_escalation_prevention

# Test permission override
cargo test --lib permissions::tests::admin_permission_override
```

### Pass Criteria

- [ ] Allow permission executes without restriction
- [ ] Require approval blocks until approval
- [ ] Deny permission blocks permanently
- [ ] Permission escalation prevented
- [ ] Admin override works for trusted users

### Evidence Required

- Permission enforcement test results
- Approval workflow logs
- Permission escalation test results
- Admin override test results

---

## Code Generation

**Requirement:** Code generation produces compilable Rust

### Code Generation Scenarios

#### 1. Simple Function

**Input:**
```
Generate a Rust function that adds two numbers
```

**Expected Output:**
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

#### 2. Struct with Methods

**Input:**
```
Generate a Rust struct for a Point with x and y fields and a distance method
```

**Expected Output:**
```rust
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}
```

#### 3. Async Function

**Input:**
```
Generate an async Rust function that fetches data from a URL
```

**Expected Output:**
```rust
async fn fetch_data(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let text = response.text().await?;
    Ok(text)
}
```

### Verification Commands

```bash
# Test code generation
cargo test --lib codegen::tests::simple_function_generation
cargo test --lib codegen::tests::struct_generation
cargo test --lib codegen::tests::async_function_generation

# Test compilability
cargo run --bin codegen -- "Generate a function" > /tmp/generated.rs
cargo check --bin test_codegen -- /tmp/generated.rs
# Expected: Compiles without errors

# Test property-based generation invariants
PROPTEST_NUMBER_OF_TESTS=100 cargo test --lib codegen::tests::compilability_invariant
```

### Pass Criteria

- [ ] Generated code compiles without errors
- [ ] Generated code uses correct Rust syntax
- [ ] Generated code includes necessary imports
- [ ] Generated code passes clippy

### Evidence Required

- Code generation test results
- Compilability test logs
- Clippy output for generated code

---

## RAG Indexing and Retrieval

**Requirement:** RAG indexes and retrieves relevant documents

### RAG Workflow

#### 1. Indexing

**Configuration:**
```yaml
rag:
  collection: "docs"
  documents:
    - path: /docs/*.md
    - path: /docs/*.txt
  chunk_size: 512
  chunk_overlap: 50
  embeddings_model: text-embedding-3-small
```

**Expected Behavior:**
- Load documents from paths
- Split into chunks
- Generate embeddings for chunks
- Store in vector database

#### 2. Retrieval

**Configuration:**
```yaml
rag:
  collection: "docs"
  query: "How do I use the CLI?"
  top_k: 5
  similarity_threshold: 0.7
```

**Expected Behavior:**
- Generate query embedding
- Search vector database
- Return top-k similar chunks
- Filter by similarity threshold

### Verification Commands

```bash
# Test indexing
cargo test --lib rag::tests::document_indexing
cargo test --lib rag::tests::chunking
cargo test --lib rag::tests::embedding_generation
cargo test --lib rag::tests::vector_storage

# Test retrieval
cargo test --lib rag::tests::query_embedding
cargo test --lib rag::tests::similarity_search
cargo test --lib rag::tests::top_k_retrieval
cargo test --lib rag::tests::threshold_filtering

# Test end-to-end RAG
cargo test --lib rag::tests::e2e_rag_workflow
```

### Pass Criteria

- [ ] Documents indexed correctly
- [ ] Chunks generated with correct size and overlap
- [ ] Embeddings generated successfully
- [ ] Vector storage works correctly
- [ ] Query retrieval returns relevant documents
- [ ] Top-k results sorted by similarity
- [ ] Threshold filtering excludes low similarity results

### Evidence Required

- Indexing test results
- Retrieval test results
- Vector database state verification
- Similarity scores for test queries

---

## Sub-Workflow Composition

**Requirement:** Sub-workflows compose correctly with variable passing

### Sub-Workflow Structure

**Parent Workflow:**
```yaml
workflow:
  name: parent_workflow
  variables:
    input_data: ${data}

  pipelines:
    - name: main_pipeline
      steps:
        - name: call_sub_workflow
          # v2: step type inferred from keys (generative_entity, tool, when, sub_workflow, loop)
          type: sub_workflow
          workflow: sub_workflow.yaml
          inputs:
            sub_input: ${workflow.input_data}
          outputs:
            parent_output: ${sub_workflow.result}
```

**Sub-Workflow:**
```yaml
workflow:
  name: sub_workflow
  variables:
    sub_input: ${input}

  pipelines:
    - name: sub_pipeline
      steps:
        - name: process
          # v2: step type inferred from keys (generative_entity, tool, when, sub_workflow, loop)
          type: llm
          model: gpt-4
          prompt: "Process: ${workflow.sub_input}"
          output_name: result
```

### Variable Passing

1. **Inputs:** Parent workflow passes variables to sub-workflow
2. **Outputs:** Sub-workflow passes results back to parent
3. **Scope:** Sub-workflow has isolated scope
4. **Error Handling:** Sub-workflow failure propagates to parent

### Verification Commands

```bash
# Test sub-workflow execution
cargo test --lib subworkflows::tests::basic_execution
cargo test --lib subworkflows::tests::input_passing
cargo test --lib subworkflows::tests::output_passing
cargo test --lib subworkflows::tests::scope_isolation

# Test nested sub-workflows
cargo test --lib subworkflows::tests::nested_subworkflows

# Test error handling
cargo test --lib subworkflows::tests::error_propagation
```

### Pass Criteria

- [ ] Sub-workflows execute successfully
- [ ] Input variables passed correctly
- [ ] Output variables returned correctly
- [ ] Sub-workflow scope isolated from parent
- [ ] Nested sub-workflows work
- [ ] Errors propagate correctly to parent

### Evidence Required

- Sub-workflow execution logs
- Input/output variable test results
- Scope isolation test results
- Error propagation test results

---

## Phase Exit Criteria

### Layer 1: Unit Tests

**Commands:**
```bash
cargo test --lib -- --test-threads=1
```

**Evidence:**
- All Phase 02 unit tests pass
- Test coverage >= 90%
- No clippy warnings

### Layer 2: Integration Tests

**Commands:**
```bash
cargo test --test '*' -- --test-threads=1
```

**Evidence:**
- All Phase 02 integration tests pass
- Backend connections work
- Tool permissions enforced

### Layer 3: Property Tests

**Commands:**
```bash
PROPTEST_NUMBER_OF_TESTS=1000 cargo test --lib property_based
```

**Evidence:**
- Code generation compilability holds for 100 iterations
- RAG retrieval invariants hold

### Layer 4: E2E Tests

**Commands:**
```bash
# Test CLI workflows
cargo run --bin agentsdk -- run examples/workflows/openai_workflow.yaml
cargo run --bin agentsdk -- run examples/workflows/tool_workflow.yaml
cargo run --bin agentsdk -- run examples/workflows/rag_workflow.yaml
cargo run --bin agentsdk -- run examples/workflows/subworkflow.yaml
```

**Evidence:**
- All example workflows execute
- CLI commands work
- Backend connections work

### Layer 5: System Log Validation

**Commands:**
```bash
# Verify backend scope logs
cargo run --bin agentsdk -- run examples/workflows/test.yaml 2>&1 | \
  jq -e 'select(.scope == "backend")'

# Verify tool scope logs
cargo run --bin agentsdk -- run examples/workflows/test.yaml 2>&1 | \
  jq -e 'select(.scope == "tool")'
```

**Evidence:**
- Backend scope logs emitted
- Tool scope logs emitted
- Logs include required fields

### Layer 6: Live CLI Verification

**Commands:**
```bash
# Test all CLI commands
cargo run --bin agentsdk -- run examples/workflows/simple.yaml
cargo run --bin agentsdk -- validate examples/workflows/simple.yaml
cargo run --bin agentsdk -- queue list
cargo run --bin agentsdk -- scheduler status
cargo run --bin agentsdk -- config list
```

**Evidence:**
- All CLI commands work
- Error handling works
- Help text displays correctly

### Layer 7: Benchmark Performance

**Commands:**
```bash
cargo bench --bench phase_02_benchmarks
```

**Evidence:**
- Backend connection performance within baseline
- RAG retrieval performance acceptable
- No performance regression > 10%

---

## Cross-Phase Regression Tests

**Commands:**
```bash
# Run Phase 00 tests
cargo test --test phase_00_integration -- --test-threads=1

# Run Phase 01 tests
cargo test --test phase_01_integration -- --test-threads=1

# Verify Phase 00+01 schema coverage
cargo run --bin schema_audit -- --phase 0 --output phase_00_regression.md
cargo run --bin schema_audit -- --phase 1 --output phase_01_regression.md
```

**Pass Criteria:**
- [ ] All Phase 00 tests still pass
- [ ] All Phase 01 tests still pass
- [ ] Phase 00 schema coverage still 100%
- [ ] Phase 01 schema coverage still 100%

---

## Schema Coverage Audit

**Requirement:** 100% of Phase 02-owned fields implemented

### Phase 02-Owned Fields

**BackendSchema:**
- type
- api_key
- base_url
- model
- timeout
- stream

**ToolSchema:**
- name
- type
- permission
- approval_timeout_ms

**RAGSchema:**
- collection
- documents
- chunk_size
- chunk_overlap
- embeddings_model
- top_k
- similarity_threshold

### Verification Commands

```bash
cargo run --bin schema_audit -- --phase 2 --output coverage_report.md
```

**Expected Output:**
- 100% coverage for Phase 02-owned fields
- No unimplemented fields

---

## ADR Compliance

**Relevant ADRs:**
- ADR-003: CLI Backends Networking Boundary
- ADR-005: Tool Permission System
- ADR-006: RAG Architecture

### Verification Commands

```bash
cargo run --bin adr_compliance -- --phase 2
```

**Expected Output:**
- All Phase 02-relevant ADR constraints satisfied
- No outstanding ADR TODOs

---

## Anti-Goal-Drift Checklist

**Run after phase completion:**

1. **Requirements Drift:**
   - [ ] Implementation matches Phase 02 requirements
   - [ ] No features from later phases added

2. **Architecture Drift:**
   - [ ] Backends match ADR-003 boundaries
   - [ ] Tool permissions match ADR-005
   - [ ] RAG matches ADR-006

3. **Scope Creep:**
   - [ ] Only CLI/backend/tool/RAG features implemented
   - [ ] No UI or automation features added

---

## Evidence Storage

**Location:** `results/phase_02/`

**Contents:**
- `unit_test_results.json`
- `integration_test_output.log`
- `property_test_results.json`
- `e2e_execution_logs/` (CLI, backend, tool, RAG, sub-workflow workflows)
- `system_log_samples.json` (backend and tool scopes)
- `cli_verification/` (all CLI commands)
- `benchmarks/` (backend and RAG performance)
- `codegen_samples/` (generated code samples)
- `schema_coverage_report.md`
- `adr_compliance_report.md`
- `anti_goal_drift_checklist.md`
- `phase_00_regression/` (Phase 00 regression test results)
- `phase_01_regression/` (Phase 01 regression test results)

---

## Blocking Issues

**Cannot exit Phase 02 if:**
- Any verification layer fails
- Backend connection fails
- Tool permission not enforced
- Generated code doesn't compile
- RAG retrieval doesn't work
- Sub-workflow composition fails
- Phase 00 or 01 regression detected

---

**End of Phase 02 Criteria**
