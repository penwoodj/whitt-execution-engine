# Validation Criteria: Task 08 - Memory & Search Integration

## Overview

Validate that memory and search integration provides tool integration, CLI commands, workflow engine integration, and ADR-0006 compliance.

---

## Checkpoint Gate Criteria

### Checkpoint 1: Tool Integration Complete
- [ ] **Memory search tool works**: Search functionality available
- [ ] **Memory storage tool works**: Store functionality available
- [ ] **Memory retrieve tool works**: Retrieve functionality available
- [ ] **Provenance tool works**: Trace functionality available
- [ ] **Unit tests pass**: All tool tests (`cargo test --package agentsdk-memory-search`)

**Verification Commands:**
```bash
# Verify integration crate builds
cargo check --package agentsdk-memory-search

# Run tool tests
cargo test --package agentsdk-memory-search --lib tools

# Expected output: All tool tests pass
```

### Checkpoint 2: CLI Integration Functional
- [ ] **Search command works**: CLI search functionality
- [ ] **Store command works**: CLI store functionality
- [ ] **Retrieve command works**: CLI retrieve functionality
- [ ] **List command works**: CLI list functionality
- [ ] **Provenance command works**: CLI trace functionality
- [ ] **GC command works**: CLI cleanup functionality
- [ ] **Unit tests pass**: All CLI tests

**Verification Commands:**
```bash
# Run CLI tests
cargo test --package agentsdk-memory-search --lib cli

# Verify CLI commands
cargo test --package agentsdk-memory-search test_cli_commands

# Expected output: All CLI tests pass
```

### Checkpoint 3: Workflow Engine Integration Working
- [ ] **Tools available as workflow nodes**: Workflow can call tools
- [ ] **Context injection works**: LLM context includes memory
- [ ] **Provenance tracking works**: Traces recorded for workflow operations
- [ ] **Error handling works**: Failures handled gracefully
- [ ] **Unit tests pass**: All workflow integration tests

**Verification Commands:**
```bash
# Run workflow integration tests
cargo test --package agentsdk-memory-search --lib workflow

# Verify workflow functionality
cargo test --package agentsdk-memory-search test_workflow_integration

# Expected output: All workflow tests pass
```

### Checkpoint 4: Performance Meets Targets
- [ ] **Tool invocation < 200ms (local ops)**: Measured with benchmarks
- [ ] **CLI command execution < 500ms (local ops)**: Measured with benchmarks
- [ ] **Workflow node execution < 500ms**: Measured with benchmarks

**Verification Commands:**
```bash
# Run performance benchmarks
cargo bench --package agentsdk-memory-search

# Verify tool invocation performance
cargo bench --bench integration_bench bench_tool_invocation

# Verify CLI performance
cargo bench --bench integration_bench bench_cli_execution

# Expected output: All latency targets met
```

---

## Functional Requirements

### Tool Integration

#### Memory Search Tool
- [ ] **Memory search tool works**
  - Test: `test_memory_search_tool()`
  - Command: `cargo test --package agentsdk-memory-search test_memory_search_tool`
  - Expected: PASS, search returns results

- [ ] **Tool accepts query parameters**
  - Test: `test_search_tool_params()`
  - Command: `cargo test --package agentsdk-memory-search test_search_tool_params`
  - Expected: PASS, query parameters processed

- [ ] **Tool returns formatted results**
  - Test: `test_search_tool_results()`
  - Command: `cargo test --package agentsdk-memory-search test_search_tool_results`
  - Expected: PASS, results formatted for LLM

#### Memory Storage Tool
- [ ] **Memory storage tool works**
  - Test: `test_memory_storage_tool()`
  - Command: `cargo test --package agentsdk-memory-search test_memory_storage_tool`
  - Expected: PASS, memory stored successfully

- [ ] **Tool accepts content parameters**
  - Test: `test_storage_tool_params()`
  - Command: `cargo test --package agentsdk-memory-search test_storage_tool_params`
  - Expected: PASS, content parameters processed

- [ ] **Tool returns memory ID**
  - Test: `test_storage_tool_return()`
  - Command: `cargo test --package agentsdk-memory-search test_storage_tool_return`
  - Expected: PASS, memory ID returned for reference

#### Memory Retrieve Tool
- [ ] **Memory retrieve tool works**
  - Test: `test_memory_retrieve_tool()`
  - Command: `cargo test --package agentsdk-memory-search test_memory_retrieve_tool`
  - Expected: PASS, memory content retrieved

- [ ] **Tool accepts ID parameters**
  - Test: `test_retrieve_tool_params()`
  - Command: `cargo test --package agentsdk-memory-search test_retrieve_tool_params`
  - Expected: PASS, ID parameters processed

- [ ] **Tool returns formatted content**
  - Test: `test_retrieve_tool_results()`
  - Command: `cargo test --package agentsdk-memory-search test_retrieve_tool_results`
  - Expected: PASS, content formatted for LLM

#### Provenance Tool
- [ ] **Provenance tool works**
  - Test: `test_provenance_tool()`
  - Command: `cargo test --package agentsdk-memory-search test_provenance_tool`
  - Expected: PASS, trace information retrieved

- [ ] **Tool accepts trace ID parameters**
  - Test: `test_provenance_tool_params()`
  - Command: `cargo test --package agentsdk-memory-search test_provenance_tool_params`
  - Expected: PASS, trace ID parameters processed

- [ ] **Tool returns formatted trace**
  - Test: `test_provenance_tool_results()`
  - Command: `cargo test --package agentsdk-memory-search test_provenance_tool_results`
  - Expected: PASS, trace formatted for LLM

### CLI Integration

#### Search Command
- [ ] **Search command works**
  - Test: `test_search_command()`
  - Command: `cargo test --package agentsdk-memory-search test_search_command`
  - Expected: PASS, CLI executes search

- [ ] **Command accepts query arguments**
  - Test: `test_search_command_args()`
  - Command: `cargo test --package agentsdk-memory-search test_search_command_args`
  - Expected: PASS, query arguments processed

- [ ] **Command outputs formatted results**
  - Test: `test_search_command_output()`
  - Command: `cargo test --package agentsdk-memory-search test_search_command_output`
  - Expected: PASS, results printed in readable format

#### Store Command
- [ ] **Store command works**
  - Test: `test_store_command()`
  - Command: `cargo test --package agentsdk-memory-search test_store_command`
  - Expected: PASS, CLI stores memory

- [ ] **Command accepts content arguments**
  - Test: `test_store_command_args()`
  - Command: `cargo test --package agentsdk-memory-search test_store_command_args`
  - Expected: PASS, content arguments processed

- [ ] **Command outputs memory ID**
  - Test: `test_store_command_output()`
  - Command: `cargo test --package agentsdk-memory-search test_store_command_output`
  - Expected: PASS, ID printed to console

#### Retrieve Command
- [ ] **Retrieve command works**
  - Test: `test_retrieve_command()`
  - Command: `cargo test --package agentsdk-memory-search test_retrieve_command`
  - Expected: PASS, CLI retrieves memory

- [ ] **Command accepts ID arguments**
  - Test: `test_retrieve_command_args()`
  - Command: `cargo test --package agentsdk-memory-search test_retrieve_command_args`
  - Expected: PASS, ID arguments processed

- [ ] **Command outputs content**
  - Test: `test_retrieve_command_output()`
  - Command: `cargo test --package agentsdk-memory-search test_retrieve_command_output`
  - Expected: PASS, content printed to console

#### List Command
- [ ] **List command works**
  - Test: `test_list_command()`
  - Command: `cargo test --package agentsdk-memory-search test_list_command`
  - Expected: PASS, CLI lists memories

- [ ] **Command accepts filter arguments**
  - Test: `test_list_command_args()`
  - Command: `cargo test --package agentsdk-memory-search test_list_command_args`
  - Expected: PASS, filter arguments processed

- [ ] **Command outputs formatted list**
  - Test: `test_list_command_output()`
  - Command: `cargo test --package agentsdk-memory-search test_list_command_output`
  - Expected: PASS, list printed in readable format

#### Provenance Command
- [ ] **Provenance command works**
  - Test: `test_provenance_command()`
  - Command: `cargo test --package agentsdk-memory-search test_provenance_command`
  - Expected: PASS, CLI shows trace

- [ ] **Command accepts trace ID arguments**
  - Test: `test_provenance_command_args()`
  - Command: `cargo test --package agentsdk-memory-search test_provenance_command_args`
  - Expected: PASS, trace ID arguments processed

- [ ] **Command outputs formatted trace**
  - Test: `test_provenance_command_output()`
  - Command: `cargo test --package agentsdk-memory-search test_provenance_command_output`
  - Expected: PASS, trace printed in readable format

#### GC Command
- [ ] **GC command works**
  - Test: `test_gc_command()`
  - Command: `cargo test --package agentsdk-memory-search test_gc_command`
  - Expected: PASS, CLI executes GC

- [ ] **Command accepts preview argument**
  - Test: `test_gc_command_args()`
  - Command: `cargo test --package agentsdk-memory-search test_gc_command_args`
  - Expected: PASS, preview argument processed

- [ ] **Command outputs GC results**
  - Test: `test_gc_command_output()`
  - Command: `cargo test --package agentsdk-memory-search test_gc_command_output`
  - Expected: PASS, results printed in readable format

### Workflow Engine Integration

#### Tool Nodes
- [ ] **Tools available as workflow nodes**
  - Test: `test_workflow_nodes_available()`
  - Command: `cargo test --package agentsdk-memory-search test_workflow_nodes_available`
  - Expected: PASS, tools registered as nodes

- [ ] **Tool nodes accept parameters**
  - Test: `test_workflow_node_params()`
  - Command: `cargo test --package agentsdk-memory-search test_workflow_node_params`
  - Expected: PASS, node parameters validated

- [ ] **Tool nodes return results**
  - Test: `test_workflow_node_results()`
  - Command: `cargo test --package agentsdk-memory-search test_workflow_node_results`
  - Expected: PASS, results passed to workflow

#### Context Injection
- [ ] **Context injection works**
  - Test: `test_context_injection()`
  - Command: `cargo test --package agentsdk-memory-search test_context_injection`
  - Expected: PASS, memory content in LLM context

- [ ] **Context formatted correctly**
  - Test: `test_context_formatting()`
  - Command: `cargo test --package agentsdk-memory-search test_context_formatting`
  - Expected: PASS, context in expected format

- [ ] **Context size managed**
  - Test: `test_context_size_management()`
  - Command: `cargo test --package agentsdk-memory-search test_context_size_management`
  - Expected: PASS, context respects size limits

#### Provenance Tracking
- [ ] **Provenance tracking works**
  - Test: `test_workflow_provenance()`
  - Command: `cargo test --package agentsdk-memory-search test_workflow_provenance`
  - Expected: PASS, traces recorded for workflow ops

- [ ] **Workflow IDs in traces**
  - Test: `test_workflow_id_in_traces()`
  - Command: `cargo test --package agentsdk-memory-search test_workflow_id_in_traces`
  - Expected: PASS, workflow ID in trace metadata

#### Error Handling
- [ ] **Tools handle errors gracefully**
  - Test: `test_tool_error_handling()`
  - Command: `cargo test --package agentsdk-memory-search test_tool_error_handling`
  - Expected: PASS, errors returned to workflow

- [ ] **Workflow nodes fail appropriately**
  - Test: `test_workflow_node_failure()`
  - Command: `cargo test --package agentsdk-memory-search test_workflow_node_failure`
  - Expected: PASS, workflow fails with error

---

## Performance Requirements

### Tool Invocation

- [ ] **Tool invocation < 200ms (local ops)**
  - Benchmark: `bench_tool_invocation`
  - Command: `cargo bench --bench integration_bench bench_tool_invocation`
  - Expected: Mean < 200.0 ms, p95 < 300.0 ms

### CLI Execution

- [ ] **CLI command execution < 500ms (local ops)**
  - Benchmark: `bench_cli_execution`
  - Command: `cargo bench --bench integration_bench bench_cli_execution`
  - Expected: Mean < 500.0 ms, p95 < 750.0 ms

### Workflow Node Execution

- [ ] **Workflow node execution < 500ms**
  - Benchmark: `bench_workflow_node`
  - Command: `cargo bench --bench integration_bench bench_workflow_node`
  - Expected: Mean < 500.0 ms, p95 < 750.0 ms

---

## Accuracy

### Tool Accuracy

- [ ] **Tool results correct**
  - Test: `test_tool_results_correct()`
  - Command: `cargo test --package agentsdk-memory-search test_tool_results_correct`
  - Expected: PASS, results match expected output

- [ ] **Tool error messages helpful**
  - Test: `test_tool_error_messages()`
  - Command: `cargo test --package agentsdk-memory-search test_tool_error_messages`
  - Expected: PASS, errors include context

### CLI Accuracy

- [ ] **CLI output correct**
  - Test: `test_cli_output_correct()`
  - Command: `cargo test --package agentsdk-memory-search test_cli_output_correct`
  - Expected: PASS, output matches expected format

- [ ] **CLI error messages helpful**
  - Test: `test_cli_error_messages()`
  - Command: `cargo test --package agentsdk-memory-search test_cli_error_messages`
  - Expected: PASS, errors include guidance

### Workflow Integration Accuracy

- [ ] **Workflow integration works end-to-end**
  - Test: `test_workflow_e2e()`
  - Command: `cargo test --package agentsdk-memory-search test_workflow_e2e`
  - Expected: PASS, complete workflow executes

---

## Error Handling

### Tool Errors

- [ ] **Tools handle errors gracefully**
  - Test: `test_tool_error_handling()`
  - Command: `cargo test --package agentsdk-memory-search test_tool_error_handling`
  - Expected: PASS, errors propagated correctly

- [ ] **Tools return helpful error messages**
  - Test: `test_tool_error_messages()`
  - Command: `cargo test --package agentsdk-memory-search test_tool_error_messages`
  - Expected: PASS, error messages actionable

### CLI Errors

- [ ] **CLI shows helpful error messages**
  - Test: `test_cli_error_messages()`
  - Command: `cargo test --package agentsdk-memory-search test_cli_error_messages`
  - Expected: PASS, errors include suggestions

- [ ] **CLI handles invalid arguments gracefully**
  - Test: `test_cli_invalid_args()`
  - Command: `cargo test --package agentsdk-memory-search test_cli_invalid_args`
  - Expected: PASS, usage shown on invalid args

### Workflow Errors

- [ ] **Workflow nodes fail appropriately**
  - Test: `test_workflow_node_failure()`
  - Command: `cargo test --package agentsdk-memory-search test_workflow_node_failure`
  - Expected: PASS, workflow stops on failure

- [ ] **Workflow errors include context**
  - Test: `test_workflow_error_context()`
  - Command: `cargo test --package agentsdk-memory-search test_workflow_error_context`
  - Expected: PASS, errors include node and parameters

---

## ADR-0006 Compliance

### Local Memory First

- [ ] **Local memory used before external search**
  - Test: `test_local_first()`
  - Command: `cargo test --package agentsdk-memory-search test_local_first`
  - Expected: PASS, local results used before external

- [ ] **External search behind policy gates**
  - Test: `test_external_policy_gates()`
  - Command: `cargo test --package agentsdk-memory-search test_external_policy_gates`
  - Expected: PASS, external search requires approval

### Provenance Tracking

- [ ] **All operations track provenance**
  - Test: `test_operations_provenance()`
  - Command: `cargo test --package agentsdk-memory-search test_operations_provenance`
  - Expected: PASS, all operations have traces

### Policy Gates

- [ ] **Policy gates enforced for external search**
  - Test: `test_policy_gates_enforced()`
  - Command: `cargo test --package agentsdk-memory-search test_policy_gates_enforced`
  - Expected: PASS, no external search bypasses gates

---

## Integration Points

### Tool System Integration

- [ ] **Tools registered with tool system**
  - Test: `test_tool_registration()`
  - Command: `cargo test --package agentsdk-memory-search test_tool_registration`
  - Expected: PASS, tools available to agents

- [ ] **Tool metadata complete**
  - Test: `test_tool_metadata()`
  - Command: `cargo test --package agentsdk-memory-search test_tool_metadata`
  - Expected: PASS, all required metadata present

### CLI Integration

- [ ] **Commands registered with CLI**
  - Test: `test_cli_registration()`
  - Command: `cargo test --package agentsdk-memory-search test_cli_registration`
  - Expected: PASS, commands available to users

- [ ] **Command help complete**
  - Test: `test_cli_help()`
  - Command: `cargo test --package agentsdk-memory-search test_cli_help`
  - Expected: PASS, help text available for all commands

### Workflow Engine Integration

- [ ] **Tools integrated as workflow nodes**
  - Test: `test_workflow_integration()`
  - Command: `cargo test --package agentsdk-memory-search test_workflow_integration`
  - Expected: PASS, workflow can use tools

- [ ] **Context injection functional**
  - Test: `test_context_injection_integration()`
  - Command: `cargo test --package agentsdk-memory-search test_context_injection_integration`
  - Expected: PASS, memory injected into LLM context

---

## Log Verification Patterns

### Tool Operation Logs

- [ ] **Tool invocations logged**
  - Grep: `grep '"operation":"tool_invoke"' ./workspace/logs/integration.log | wc -l`
  - Expected: Count equals number of tool calls

- [ ] **Tool names logged**
  - Grep: `grep '"operation":"tool_invoke"' ./workspace/logs/integration.log | jq -r '.tool_name'`
  - Expected: Tool name present for all calls

- [ ] **Tool parameters logged**
  - Grep: `grep '"operation":"tool_invoke"' ./workspace/logs/integration.log | jq -r '.parameters'`
  - Expected: Parameters present for all calls

### CLI Operation Logs

- [ ] **CLI command invocations logged**
  - Grep: `grep '"operation":"cli_command"' ./workspace/logs/integration.log | wc -l`
  - Expected: Count equals number of CLI commands

- [ ] **Command names logged**
  - Grep: `grep '"operation":"cli_command"' ./workspace/logs/integration.log | jq -r '.command'`
  - Expected: Command name present for all invocations

### Workflow Integration Logs

- [ ] **Workflow node executions logged**
  - Grep: `grep '"operation":"workflow_node"' ./workspace/logs/integration.log | wc -l`
  - Expected: Count equals number of node executions

- [ ] **Workflow IDs logged**
  - Grep: `grep '"operation":"workflow_node"' ./workspace/logs/integration.log | jq -r '.workflow_id'`
  - Expected: Workflow ID present for all node executions

---

## Test Coverage

### Unit Tests

- [ ] **Unit tests for tools**
  - Command: `cargo test --package agentsdk-memory-search --lib tools`
  - Expected: All tool tests pass

- [ ] **Unit tests for CLI**
  - Command: `cargo test --package agentsdk-memory-search --lib cli`
  - Expected: All CLI tests pass

- [ ] **Unit tests for workflow integration**
  - Command: `cargo test --package agentsdk-memory-search --lib workflow`
  - Expected: All workflow tests pass

### Integration Tests

- [ ] **Integration tests for full workflow**
  - Command: `cargo test --package agentsdk-memory-search --test integration_test`
  - Expected: All integration tests pass

### End-to-End Workflow Tests

- [ ] **End-to-end workflow tests**
  - Command: `cargo test --package agentsdk-memory-search --test e2e_workflow`
  - Expected: All e2e tests pass

---

## Final Checklist

### Implementation Complete
- [ ] Tool integration implemented and tested
- [ ] CLI commands functional
- [ ] Workflow engine integration working
- [ ] Performance meets all targets

### ADR-0006 Compliant
- [ ] Local memory used before external search
- [ ] All operations track provenance
- [ ] Policy gates enforced for external search

### Integration Ready
- [ ] Works with tool system
- [ ] Works with CLI
- [ ] Works with workflow engine

### Documentation Complete
- [ ] API documentation generated
- [ ] CLI usage documented
- [ ] Workflow integration documented

### Tests Passing
- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] All e2e tests pass
- [ ] Test coverage > 90%

**Total Lines:** ~570 lines
