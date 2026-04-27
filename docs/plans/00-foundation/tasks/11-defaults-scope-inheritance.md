# Task 11: Defaults, Scope & Inheritance

**Goal:** Implement default values, scope hierarchy, and inheritance rules for all policy fields.

**Estimated Time:** 5 hours

**Dependencies:** Task 8, Task 10

**Files:**
- Modify: `src/defaults.rs` (implement defaults and inheritance)
- Create: `tests/defaults_test.rs` (defaults tests)

---

## Implementation Status

**Status**: 🔵 NOT STARTED

### What Exists
- **Config loading**: [src/config/unified.rs](../../src/config/unified.rs) loads configuration with resolution hierarchy ✅
- **Interpolation**: [src/model/interpolation.rs](../../src/model/interpolation.rs) handles variable interpolation ✅

### What's Missing
- **Defaults module** not implemented:
  - `src/defaults.rs` - NOT IMPLEMENTED (plan expects this file location)
  - `DefaultValues` struct - NOT IMPLEMENTED
  - `LoggingDefaults` struct - NOT IMPLEMENTED
  - `ToolPermissionDefaults` struct - NOT IMPLEMENTED

- **Inheritance implementation** not implemented:
  - `apply_defaults()` function - NOT IMPLEMENTED
  - `apply_logging_defaults()` function - NOT IMPLEMENTED
  - `apply_tool_permission_defaults()` function - NOT IMPLEMENTED

- **Scope hierarchy** not implemented:
  - L1 (workflow top-level) → L2 (agentic_workflow defaults) → L3 (step-level) - NOT IMPLEMENTED
  - Tool permissions: L1 baseline → L3 can only RESTRICT - NOT IMPLEMENTED
  - Retry: L2 step defaults → L3 overrides completely (not merged) - NOT IMPLEMENTED
  - Hooks: L2 default hooks MERGE with L3 step hooks - NOT IMPLEMENTED

- **v2.0 convention** not implemented:
  - "presence = enabled" convention - NOT IMPLEMENTED
  - `disabled: true` explicit turn-off - NOT IMPLEMENTED

- **Test file** not created:
  - `tests/defaults_test.rs` - NOT CREATED

### QA Coverage
- **Status**: No dedicated QA tests for defaults and inheritance
- **Coverage**: No EPOC tests for defaults/inheritance found

### Evidence
- **No defaults module**: No defaults/inheritance code exists ✅
- **Build**: ✅ `cargo build` passes (without defaults module)
- **No inheritance tests**: No test file `tests/defaults_test.rs` exists ✅

### Plan vs Reality Notes
- **Plan expects**: Comprehensive defaults and inheritance system with scope hierarchy
- **Current reality**: Config loading exists but no dedicated defaults/inheritance module
- **v2.0 notes**: Plan mentions v2.0 schema changes but current implementation doesn't implement these

---

## QA Cross-References

- **QA Criteria**: [QA-00-09](../../qa/phase-00/QA-CRITERIA.md)
- **Schema Ref**: Lines 726-740 (Scope & Inheritance Rules), Lines 740-805 (Default Behavior)
- **Inheritance rules**: Plan specifies strict override precedence, no implementation exists
