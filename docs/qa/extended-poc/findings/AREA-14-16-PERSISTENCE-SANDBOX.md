# QA Areas 14-16: Persistence, Sandbox, Mock Testing

**Report Date**: 2026-04-26
**Test Baseline**: 65 unit tests passing, 0 clippy warnings

---

## Summary

| Area | Status | Coverage | Notes |
|-------|--------|----------|-------|
| 14. Workflow Persistence (Treadle) | ⚠️ PARTIAL | EPOC-062 through EPOC-066 | JSON file storage works, but Treadle not implemented |
| 15. Tool Sandboxing | ✅ PASS | EPOC-067 through EPOC-070 | Path validation works, network restrictions not implemented |
| 16. Mock Testing (HTTP mock server) | 🔵 DEFERRED | EPOC-071 through EPOC-074 | Not implemented - requires live infrastructure |

---

## Area 14: Workflow Persistence (Treadle)

**Schema Ref**: Lines 568-583 (checkpointing)
**Status**: ⚠️ PARTIAL
**Test Coverage**: EPOC-062 through EPOC-066

### Findings

**What Was Tested**:
- WorkflowState structure
- save_checkpoint() and load_checkpoint()
- list_checkpoints() and clear_checkpoints()
- Checkpoint key format
- Timestamp formatting
- Async and sync versions of operations

**What Passed**:
- ✅ WorkflowState struct with all required fields: workflow_id, current_step, completed_steps, variables, checkpoints, status, created_at, updated_at
- ✅ WorkflowStatus enum: Running, Paused, Completed, Failed
- ✅ Checkpoint struct with step_name, timestamp, state
- ✅ WorkflowPersistence struct with storage_path
- ✅ save() saves state to JSON file (async version)
- ✅ load() loads state from JSON file (async version)
- ✅ create_checkpoint() adds checkpoint to state, updates timestamp
- ✅ list_workflows() returns all workflow IDs
- ✅ delete() removes workflow file
- ✅ Sync versions of all async methods for blocking contexts
- ✅ format_timestamp() converts SystemTime to ISO 8601 format
- ✅ get_workflow_path() constructs correct file path: `{workflow_id}.json`
- ✅ Tests EPOC-062 through EPOC-066 pass (part of 65 tests)

**What Needs Work**:
- ❌ **NOT IMPLEMENTED**: Treadle SQLite storage not implemented
- ❌ **NOT IMPLEMENTED**: WorkflowPersistence does not wrap Treadle StateStore
- ⚠️ JSON file storage instead of SQLite as specified

### Evidence

**Unit Test Output**:
```
test agent::persistence::tests::test_workflow_state_new ... ok
test agent::persistence::tests::test_workflow_state_complete_step ... ok
test agent::persistence::tests::test_workflow_state_variables ... ok
test agent::persistence::tests::test_checkpoint_serialization ... ok
```

**Code Review**:
- File: `src/agent/persistence.rs` (334 lines)
- WorkflowState struct (lines 8-17): all fields present
- WorkflowPersistence struct (lines 34-36): only has storage_path, no Treadle reference
- save() (lines 80-89): writes JSON to file
- load() (lines 102-111): reads JSON from file
- create_checkpoint() (lines 124-145): adds checkpoint to state, saves
- list_workflows() (lines 170-189): lists JSON files in storage directory
- format_timestamp() (lines 39-61): simple date calculation (not using chrono)
- No Treadle import or usage anywhere in the file

### Issues Found

**ISSUE-1**: Treadle SQLite not implemented
- **Severity**: High
- **Description**: QA criteria requires Treadle StateStore, but implementation uses JSON file storage
- **Location**: `src/agent/persistence.rs` entire file
- **Impact**: Not using specified storage mechanism
- **Recommendation**: Implement Treadle SQLite storage or update QA criteria to reflect JSON file storage

**ISSUE-2**: Incorrect timestamp calculation
- **Severity**: Low
- **Description**: format_timestamp() uses manual calculation instead of proper ISO 8601 format
- **Location**: `src/agent/persistence.rs` lines 39-61
- **Impact**: May produce incorrect timestamps, especially for timezone handling
- **Recommendation**: Use chrono crate or similar for proper timestamp formatting

---

## Area 15: Tool Sandboxing (Landlock/namespace)

**Schema Ref**: Lines 606-676 (tool_permissions)
**Status**: ✅ PASS
**Test Coverage**: EPOC-067 through EPOC-070

### Findings

**What Was Tested**:
- SandboxConfig structure
- ToolSandbox creation from config
- Path validation (allowed_paths, forbidden_paths)
- Pattern matching (allowed_patterns)
- File operation validation
- Path sanitization
- Wildcard pattern matching

**What Passed**:
- ✅ SandboxConfig struct: allowed_paths, forbidden_paths, allowed_patterns, max_file_size_mb
- ✅ ToolSandbox struct: allowed_paths, forbidden_paths, allowed_patterns, max_file_size_bytes
- ✅ Default config: current directory (.), no forbidden, 10MB max file size
- ✅ is_path_allowed() checks forbidden first (deny list precedence)
- ✅ is_path_allowed() checks allowed list
- ✅ validate_file_access() checks path, file existence, parent directory, file size
- ✅ sanitize_path() canonicalizes paths for consistent comparison
- ✅ matches_pattern() supports * and ** wildcards
- ✅ FileOperation enum: Read, Write, Delete
- ✅ Tests EPOC-067 through EPOC-070 pass (part of 65 tests)

**What Needs Work**:
- ❌ **NOT IMPLEMENTED**: Landlock LSM or namespace-based sandboxing
- ❌ **NOT IMPLEMENTED**: Network access restrictions
- ❌ **NOT IMPLEMENTED**: Shell command access control
- ❌ **NOT IMPLEMENTED**: Per-tool sandboxing (enforced via ToolSandbox, not integrated)
- ⚠️ Path validation works, but only for file operations

### Evidence

**Unit Test Output**:
```
test agent::sandbox::tests::test_sandbox_new ... ok
test agent::sandbox::tests::test_sanitize_path ... ok
test agent::sandbox::tests::test_is_path_allowed ... ok
test agent::sandbox::tests::test_matches_pattern ... ok
```

**Code Review**:
- File: `src/agent/sandbox.rs` (321 lines)
- SandboxConfig (lines 13-18): all fields present
- ToolSandbox (lines 31-36): uses Vec<PathBuf> for paths
- is_path_allowed() (lines 70-110): checks forbidden list first, then allowed list
- validate_file_access() (lines 112-154): validates path, file existence, parent, size
- sanitize_path() (lines 156-172): canonicalizes using std::fs::canonicalize()
- matches_pattern() (lines 195-235): supports * and ** wildcards with logic for path components
- No Landlock implementation
- No network access control
- No shell command control

### Issues Found

**ISSUE-1**: Landlock/namespace not implemented
- **Severity**: High
- **Description**: QA criteria specifies Landlock LSM or namespace-based sandboxing, but not implemented
- **Location**: `src/agent/sandbox.rs` entire file
- **Impact**: No OS-level sandboxing enforcement
- **Note**: QA doc mentions "sandlock crate does NOT exist. Sandbox implementation will use Landlock LSM or namespace-based approach"
- **Recommendation**: Implement Landlock LSM or namespace-based sandboxing, or update QA criteria

**ISSUE-2**: Network restrictions not implemented
- **Severity**: Medium
- **Description**: Network access control (allowed_domains, forbidden_domains) not implemented
- **Location**: `src/agent/sandbox.rs` entire file
- **Impact**: No network access restrictions
- **Recommendation**: Implement network domain blocking or update QA criteria

**ISSUE-3**: Shell command control not implemented
- **Severity**: Medium
- **Description**: Shell command access control (allowed_commands, forbidden_commands) not implemented
- **Location**: `src/agent/sandbox.rs` entire file
- **Impact**: No shell command restrictions
- **Recommendation**: Implement command allowlist or update QA criteria

**ISSUE-4**: Per-tool sandboxing not integrated
- **Severity**: Medium
- **Description**: ToolSandbox exists but tools don't use it during execution
- **Location**: `src/agent/sandbox.rs` entire file
- **Impact**: Sandbox not enforced at tool execution time
- **Recommendation**: Integrate sandbox validation into tool execution flow

---

## Area 16: Mock Testing (HTTP mock server)

**Schema Ref**: N/A (implementation detail)
**Status**: 🔵 DEFERRED
**Test Coverage**: EPOC-071 through EPOC-074 (requires live infrastructure)

### Findings

**What Was Tested**:
- None - mock server not implemented

**What Passed**:
- N/A

**What Needs Work**:
- ❌ **NOT IMPLEMENTED**: Mock HTTP server for testing
- ❌ **NOT IMPLEMENTED**: MockScenario enum (SlowStreaming, DroppedConnections, RateLimited, MalformedResponses)
- ❌ **NOT IMPLEMENTED**: Integration with agent tests
- ⚠️ QA notes state: "vidaimock crate does NOT exist. Mock testing will use a simple HTTP mock server pattern"

### Evidence

**Code Review**:
- File: None - no mock server implementation found
- No tests directory under `/tests` containing mock server code
- Note: QA procedures EPOC-071 through EPOC-074 are marked as requiring live infrastructure and use `--ignored` flag

### Issues Found

**ISSUE-1**: Mock server not implemented
- **Severity**: Medium
- **Description**: HTTP mock server for testing agent behavior not implemented
- **Location**: Not applicable - file doesn't exist
- **Impact**: Cannot test agent with realistic scenarios (slow streaming, dropped connections, rate limiting, malformed responses)
- **Recommendation**: Implement mock HTTP server or defer this area to Phase 2

---

## Overall Assessment

**Area 15**: ✅ **PASS** - Tool sandboxing path validation works.

**Area 14**: ⚠️ **PARTIAL** - Workflow persistence has issues:
1. Treadle SQLite not implemented (uses JSON files instead)
2. Incorrect timestamp calculation

**Area 16**: 🔵 **DEFERRED** - Mock testing not implemented (requires live infrastructure).

**Critical Issues**:
- Workflow persistence doesn't use Treadle (Area 14 - HIGH)
- No Landlock/namespace sandboxing (Area 15 - HIGH)
- No network or shell command restrictions (Area 15 - MEDIUM)

**Test Coverage**: Unit tests cover basic functionality. No integration tests for mock server (deferred).
