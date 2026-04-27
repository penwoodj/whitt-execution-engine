# Task 8: Policy Compiler

**Goal:** Implement deterministic policy field compilation with inheritance and override rules.

**Estimated Time:** 5 hours

**Dependencies:** Task 6

**Files:**
- Modify: `src/policy.rs` (implement policy compilation)
- Create: `tests/policy_test.rs` (policy compilation tests)

---

## Implementation Status

**Status**: 🔵 NOT STARTED

### What Exists
- **Config loading**: [src/config/unified.rs](../../src/config/unified.rs) loads configuration with resolution hierarchy ✅
- **Tool permissions**: [src/agent/sandbox.rs](../../src/agent/sandbox.rs) has path-based validation ✅

### What's Missing
- **Policy compiler module** not implemented:
  - `src/policy.rs` - NOT IMPLEMENTED (plan expects this file location)
  - `compile_policies()` function - NOT IMPLEMENTED
  - `compile_logging_policy()` function - NOT IMPLEMENTED
  - `compile_tool_permissions_policy()` function - NOT IMPLEMENTED

- **Policy types** not implemented (per plan):
  - `CompiledPolicy` struct - NOT IMPLEMENTED
  - `LoggingPolicy` struct - NOT IMPLEMENTED
  - `ToolPermissionsPolicy` struct - NOT IMPLEMENTED
  - Logging and tool permissions compilation - NOT IMPLEMENTED

- **Inheritance rules** not implemented:
  - L1 (workflow top-level) → L2 (agentic_workflow defaults) → L3 (step-level) - NOT IMPLEMENTED
  - Tool permissions: L1 baseline → L3 can only RESTRICT - NOT IMPLEMENTED
  - Retry: L2 step defaults → L3 overrides completely (not merged) - NOT IMPLEMENTED
  - Hooks: L2 default hooks MERGE with L3 step hooks - NOT IMPLEMENTED

- **Test file** not created:
  - `tests/policy_test.rs` - NOT CREATED

### QA Coverage
- **Status**: No dedicated QA tests for policy compilation
- **Coverage**: From EPOC Extended POC findings:
  - **AREA-15 TOOL SANDBOX** (Area 15) - ✅ PASS - Path-based sandboxing implemented

### Evidence
- **No policy compiler**: No policy compilation code exists ✅
- **Build**: ✅ `cargo build` passes (without policy compiler module)
- **No policy tests**: No test file `tests/policy_test.rs` exists ✅

---

## QA Cross-References

- **QA Criteria**: ['$qa_criteria']('$file')
- **Test Cases**: ['$test_case']('$file')
- **Schema Ref**: $schema_ref
