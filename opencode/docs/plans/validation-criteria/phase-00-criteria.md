# Phase 00: Foundation - Validation Criteria

**Phase Focus:** Parsing, validation, and intermediate representation
**Entry Criteria:** None (first phase)
**Estimated Duration:** 2-3 weeks
**Blocking for:** All subsequent phases

---

## Phase Overview

Phase 00 establishes the foundation for the entire AgentSDK Execution Engine. This phase focuses on parsing workflow specifications, validating them against the schema, and generating an intermediate representation (WorkflowIR). Without a solid foundation, all later phases cannot proceed.

**Critical Success Factors:**
1. Robust YAML parsing that handles all edge cases
2. Accurate variable interpolation across scopes
3. Detecting structural errors (cycles, invalid thresholds)
4. Preserving all metadata needed for execution

---

## Phase Entry Criteria

**Status:** Not applicable (first phase)

**Prerequisites:**
- [ ] Development environment set up with Rust toolchain
- [ ] Code repository cloned and buildable
- [ ] Test infrastructure scaffolding in place
- [ ] Documentation templates created

---

## Schema Domain Coverage

**Requirement:** WorkflowSpec must compile for all 19 schema domains

### Domain List

1. **WorkflowSchema**: Top-level workflow metadata
2. **PipelineSchema**: Pipeline-level configuration
3. **StepSchema**: Individual step definitions
4. **RetrySchema**: Retry logic configuration
5. **LoopSchema**: Loop iteration configuration
6. **BranchSchema**: Conditional branching logic
7. **ThresholdSchema**: Threshold-based gating
8. **ScopeSchema**: Variable scoping rules
9. **EnvironmentSchema**: Environment variable definitions
10. **VariableSchema**: Variable declarations and references
11. **BackendSchema**: Model backend connections
12. **ModelSchema**: LLM model configurations
13. **ToolSchema**: Tool invocation specifications
14. **LoggingSchema**: Logging configuration
15. **MetricsSchema**: Metrics collection configuration
16. **HumanGatingSchema**: Human-in-the-loop gating
17. **ErrorHandlingSchema**: Error handling strategies
18. **OptimizationSchema**: Performance optimization hints
19. **MetadataSchema**: Custom metadata fields

### Verification Commands

```bash
# Test schema compilation for all domains
cargo test --test schema_compilation -- --test-threads=1

# Verify all domains compile without errors
for domain in workflow pipeline step retry loop branch threshold scope environment variable backend model tool logging metrics human_gating error_handling optimization metadata; do
  echo "Testing $domain domain..."
  cargo test --lib schemas::${domain}::tests::compiles || exit 1
done
```

### Pass Criteria

- [ ] All 19 schema domains compile without errors
- [ ] Serde serialization/deserialization works bidirectionally
- [ ] Optional fields correctly handle presence/absence
- [ ] Default values applied when fields missing
- [ ] Enum variants parse correctly from YAML

### Evidence Required

- Compilation log showing all domains compile
- Serde round-trip test results (1000 iterations)
- Unit test coverage >= 95% for schema code

---

## WorkflowIR Generation

**Requirement:** WorkflowIR generates correctly from valid specs

### IR Structure Requirements

WorkflowIR must include:
- Complete node graph (workflow → pipeline → step)
- Variable resolution table
- DAG structure with dependencies
- Scope hierarchy
- Metadata preserved from source YAML

### Verification Commands

```bash
# Generate IR for example workflows
cargo run --bin ir_generator -- examples/workflows/simple.yaml

# Verify IR completeness
cargo test --lib ir::tests::completeness_check

# Test IR round-trip (YAML → IR → YAML')
cargo test --lib ir::tests::round_trip
```

### Test Cases

**Valid Workflows:**
- Minimal workflow (single step)
- Multi-pipeline workflow
- Nested pipelines
- Workflow with all schema domains present

**IR Verification:**
- Node graph contains all workflow elements
- Variable resolution table includes all ${...} references
- DAG has correct dependency edges
- Scope hierarchy matches YAML nesting
- Metadata is preserved 1:1

### Pass Criteria

- [ ] IR generation succeeds for all valid YAML specs
- [ ] IR contains all required information
- [ ] No information loss in IR generation
- [ ] IR round-trip preserves semantic equivalence

### Evidence Required

- IR generation logs for example workflows
- IR structure validation test results
- Round-trip test results showing no semantic drift

---

## Parser Robustness

**Requirement:** Parser handles all 53 example workflows without crash

### Example Workflow Categories

1. **Basic Workflows** (15 files):
   - Simple linear workflows
   - Single-pipeline workflows
   - Multi-step workflows

2. **Complex Control Flow** (12 files):
   - Nested loops
   - Conditional branches
   - Retry logic
   - Threshold gating

3. **Variable Interpolation** (8 files):
   - Simple variable references
   - Nested variable references
   - Scope inheritance
   - Default values

4. **Error Scenarios** (10 files):
   - Missing required fields
   - Invalid types
   - Out-of-range values
   - Circular dependencies

5. **Edge Cases** (8 files):
   - Empty workflows
   - Large workflows (100+ steps)
   - Unicode content
   - Comments and special characters

### Verification Commands

```bash
# Parse all 53 example workflows
find examples/workflows -name "*.yaml" -o -name "*.yml" | while read file; do
  echo "Parsing $file..."
  cargo run --bin parser -- "$file" || exit 1
done

# Count parsed files
find examples/workflows -name "*.yaml" -o -name "*.yml" | wc -l
# Expected output: 53
```

### Pass Criteria

- [ ] All 53 example workflows parse without crash
- [ ] Parser returns appropriate error for invalid workflows
- [ ] No memory leaks during parsing (valgrind clean)
- [ ] Parsing time < 1 second per workflow on average

### Evidence Required

- Parse log showing all 53 files processed
- Error log for invalid workflows (expected errors)
- Valgrind report showing no memory leaks
- Performance metrics (parse time distribution)

---

## Variable Interpolation

**Requirement:** Variable interpolation correctly resolves ${...} patterns

### Interpolation Scenarios

**Simple References:**
- `${workflow.name}` → workflow-level variable
- `${pipeline.input}` → pipeline-level variable
- `${step.output}` → step-level variable

**Nested References:**
- `${workflow.env.${step.runtime_mode}}` → dynamic variable name
- `${pipeline.steps.${iteration_index}.output}` → array indexing

**Scope Inheritance:**
- Variables cascade from workflow → pipeline → step
- Lower scope can override higher scope variables
- Variable shadowing rules applied correctly

**Default Values:**
- `${variable:default_value}` → use default if undefined
- Nested defaults in scope hierarchy

### Verification Commands

```bash
# Test variable interpolation
cargo test --lib interpolation::tests::simple_references
cargo test --lib interpolation::tests::nested_references
cargo test --lib interpolation::tests::scope_inheritance
cargo test --lib interpolation::tests::default_values

# Property-based testing for interpolation invariants
PROPTEST_NUMBER_OF_TESTS=1000 cargo test --lib interpolation::tests::invariants
```

### Invariants to Test

1. **Identity Invariant:** `interpolate("${x}") == interpolate("${x}")` (deterministic)
2. **Scope Invariant:** Child scope can access parent scope variables
3. **Default Invariant:** Undefined variable with default returns default
4. **Cycle Invariant:** Circular references detected and rejected

### Pass Criteria

- [ ] All interpolation test cases pass
- [ ] Property tests hold for 1000 iterations
- [ ] No false positives (valid interpolation rejected)
- [ ] No false negatives (invalid interpolation accepted)

### Evidence Required

- Unit test results for all interpolation scenarios
- Property test results showing 1000 successful iterations
- Test cases covering edge cases (empty variables, deeply nested, etc.)

---

## Default Value Application

**Requirement:** Defaults applied correctly when fields missing

### Default Value Categories

**Schema-Level Defaults:**
- Retry max_attempts: 3
- Retry backoff_ms: 1000
- Loop max_iterations: 10

**Backend-Level Defaults:**
- Timeout: 30 seconds
- Max_tokens: 4096
- Temperature: 0.7

**Tool-Level Defaults:**
- Timeout: 10 seconds
- Retry on failure: true

### Verification Commands

```bash
# Test default value application
cargo test --lib defaults::tests::schema_defaults
cargo test --lib defaults::tests::backend_defaults
cargo test --lib defaults::tests::tool_defaults

# Test missing field handling
cargo test --lib defaults::tests::missing_fields
```

### Test Cases

**Minimal YAML (all optional fields missing):**
```yaml
workflow:
  name: minimal_workflow
  pipelines:
    - name: pipeline_1
      steps:
        - name: step_1
```

Expected behavior:
- All schema defaults applied
- Workflow parses successfully
- No missing field errors

### Pass Criteria

- [ ] All default values applied correctly
- [ ] Missing fields trigger appropriate defaults
- [ ] Explicit values override defaults
- [ ] Defaults are type-consistent with schema

### Evidence Required

- Test results showing default application
- YAML → IR comparison showing defaults inserted
- No test failures for missing fields

---

## Scope Inheritance Validation

**Requirement:** Scope inheritance validated correctly

### Scope Hierarchy

```
Workflow Scope
├── Environment Variables
├── Pipeline Scope
│   ├── Environment Variables
│   └── Step Scope
│       ├── Environment Variables
│       └── Loop Scope
│           └── Iteration Variables
```

### Validation Rules

1. **Upward Visibility:** Child scope can access parent scope variables
2. **Downward Isolation:** Parent scope cannot access child scope variables
3. **Sibling Isolation:** Sibling scopes cannot access each other's variables
4. **Override Rules:** Child scope can override parent scope variables
5. **Shadowing Detection:** Shadowed variables logged as warnings

### Verification Commands

```bash
# Test scope inheritance
cargo test --lib scope::tests::upward_visibility
cargo test --lib scope::tests::downward_isolation
cargo test --lib scope::tests::sibling_isolation
cargo test --lib scope::tests::override_rules
cargo test --lib scope::tests::shadowing_detection
```

### Pass Criteria

- [ ] All scope inheritance rules enforced
- [ ] Upward visibility works correctly
- [ ] Downward isolation prevents access violations
- [ ] Shadowing detected and logged
- [ ] Variable resolution respects scope boundaries

### Evidence Required

- Scope inheritance test results
- Variable resolution logs showing scope hierarchy
- Shadowing warnings in logs

---

## Threshold Validation

**Requirement:** Threshold values validated for correctness

### Threshold Types

1. **Numeric Thresholds:**
   - min/max values
   - Range validation
   - Unit validation (ms, seconds, bytes)

2. **Count Thresholds:**
   - Positive integers
   - Maximum limits

3. **Percentage Thresholds:**
   - 0-100 range
   - Fractional values allowed

### Validation Rules

```yaml
# Valid thresholds
threshold:
  max_attempts: 10        # Positive integer
  min_confidence: 0.5     # 0.0-1.0 range
  max_duration_ms: 60000 # Positive integer
  error_rate_threshold: 0.1 # 0.0-1.0 range

# Invalid thresholds
threshold:
  max_attempts: -1       # Negative (invalid)
  min_confidence: 1.5     # Out of range (invalid)
  max_duration_ms: "10s"  # Wrong type (invalid)
```

### Verification Commands

```bash
# Test threshold validation
cargo test --lib threshold::tests::numeric_thresholds
cargo test --lib threshold::tests::count_thresholds
cargo test --lib threshold::tests::percentage_thresholds
cargo test --lib threshold::tests::invalid_thresholds
```

### Pass Criteria

- [ ] Valid thresholds accepted
- [ ] Invalid thresholds rejected with clear error messages
- [ ] Error messages reference schema documentation
- [ ] Threshold types validated correctly

### Evidence Required

- Threshold validation test results
- Error messages for invalid thresholds
- Schema documentation references in errors

---

## DAG Validation

**Requirement:** DAG validation detects cycles and invalid dependencies

### DAG Structure

Workflow DAG is built from:
- Pipeline dependencies (p1 → p2)
- Step dependencies (s1 → s2)
- Loop iterations (iteration i → i+1)
- Branch dependencies (condition → true_branch, condition → false_branch)

### Validation Rules

1. **Cycle Detection:** No cycles allowed in DAG
2. **Transitive Dependency:** Dependency edges form transitive closure
3. **Topological Sort:** DAG must be topologically sortable
4. **Orphan Detection:** All nodes reachable from root

### Verification Commands

```bash
# Test cycle detection
cargo test --lib dag::tests::cycle_detection

# Test topological sort
cargo test --lib dag::tests::topological_sort

# Test orphan detection
cargo test --lib dag::tests::orphan_detection
```

### Test Cases

**Valid DAG:**
```
workflow → pipeline1 → step1 → step2 → step3
                        ↓
                     step4 → step5
```
Expected: No cycles, topologically sortable

**Invalid DAG (Cycle):**
```
workflow → pipeline1 → step1 → step2 → step1 (cycle)
```
Expected: Cycle detected, error raised

**Invalid DAG (Orphan):**
```
workflow → pipeline1 → step1
            orphan_step (not reachable from workflow)
```
Expected: Orphan detected, error raised

### Pass Criteria

- [ ] All cycles detected and reported
- [ ] All orphans detected and reported
- [ ] Valid DAGs successfully topologically sorted
- [ ] Error messages include cycle path or orphan node

### Evidence Required

- Cycle detection test results
- Topological sort test results
- Orphan detection test results
- Error messages showing cycle paths or orphan nodes

---

## .glyphnova/ Storage

**Requirement:** .glyphnova/ storage structure works correctly

### Storage Layout

```
.glyphnova/
├── cache/              # Cached artifacts
├── state/              # Workflow execution state
├── logs/               # Structured logs
├── metrics/            # Metrics storage
├── tmp/                # Temporary files
└── config/             # Configuration files
```

### Verification Commands

```bash
# Test .glyphnova/ directory creation
cargo test --lib storage::tests::directory_creation

# Test file storage in .glyphnova/
cargo test --lib storage::tests::file_storage

# Test state persistence
cargo test --lib storage::tests::state_persistence
```

### Test Cases

1. **Directory Creation:**
   - All required directories created on first run
   - Existing directories not overwritten
   - Permissions set correctly (0700 for sensitive data)

2. **File Storage:**
   - Files written to correct subdirectories
   - File naming follows conventions
   - Large files handled correctly

3. **State Persistence:**
   - Workflow state saved to .glyphnova/state/
   - State restored correctly after interruption
   - Concurrent writes handled safely

### Pass Criteria

- [ ] All required directories created
- [ ] Files stored in correct locations
- [ ] State persistence works correctly
- [ ] No permission errors on file operations

### Evidence Required

- Directory structure verification
- File storage test results
- State persistence test results
- Permission verification

---

## Phase Exit Criteria

### Layer 1: Unit Tests

**Commands:**
```bash
cargo test --lib -- --test-threads=1
```

**Evidence:**
- All unit tests pass
- Test coverage >= 90%
- No clippy warnings

### Layer 2: Integration Tests

**Commands:**
```bash
cargo test --test '*' -- --test-threads=1
```

**Evidence:**
- All integration tests pass
- No deadlocks or panics
- Component communication works

### Layer 3: Property Tests

**Commands:**
```bash
PROPTEST_NUMBER_OF_TESTS=1000 cargo test --lib property_based
```

**Evidence:**
- All property tests pass for 1000 iterations
- No invariants violated
- No shrinking required

### Layer 4: E2E Tests

**Commands:**
```bash
for workflow in examples/workflows/*.yaml; do
  cargo run --bin parser -- "$workflow" || exit 1
done
```

**Evidence:**
- All 53 example workflows parse
- No crashes during parsing
- Parsing time < 1 second per workflow

### Layer 5: System Log Validation

**Commands:**
```bash
cargo run --bin parser -- examples/workflows/test.yaml 2>&1 | jq -e '.scope'
```

**Evidence:**
- System scope logs emitted
- Logs include timestamp and message
- No missing required fields

### Layer 6: Live CLI Verification

**Commands:**
```bash
cargo run --bin parser -- --help
cargo run --bin parser -- examples/workflows/simple.yaml
cargo run --bin parser -- nonexistent.yaml; echo "Exit code: $?"
```

**Evidence:**
- Help command works
- Valid input parses successfully
- Invalid input returns error (exit code != 0)

### Layer 7: Benchmark Performance

**Commands:**
```bash
cargo bench --bench phase_00_benchmarks
```

**Evidence:**
- Parsing performance within baseline (no regression > 10%)
- Memory usage linear with workflow size
- No memory leaks (valgrind clean)

---

## Cross-Phase Regression Tests

**Status:** Not applicable (first phase)

---

## Schema Coverage Audit

**Requirement:** 100% of Phase 00-owned fields implemented

### Phase 00-Owned Fields

**WorkflowSchema:**
- name
- description
- version
- metadata

**PipelineSchema:**
- name
- description
- steps

**StepSchema:**
- name
- description
- type
- configuration

### Verification Commands

```bash
cargo run --bin schema_audit -- --phase 0 --output coverage_report.md
```

**Expected Output:**
- 100% coverage for Phase 00-owned fields
- No unimplemented fields

---

## ADR Compliance

**Relevant ADRs:**
- ADR-001: Schema Definition
- ADR-002: WorkflowIR Structure
- ADR-003: Parsing Strategy

### Verification Commands

```bash
cargo run --bin adr_compliance -- --phase 0
```

**Expected Output:**
- All Phase 00-relevant ADR constraints satisfied
- No outstanding ADR TODOs

---

## Anti-Goal-Drift Checklist

**Run after phase completion:**

1. **Requirements Drift:**
   - [ ] Implementation matches Phase 00 requirements
   - [ ] No features from later phases added

2. **Architecture Drift:**
   - [ ] Parser matches ADR-001 structure
   - [ ] WorkflowIR matches ADR-002 structure

3. **Scope Creep:**
   - [ ] Only parsing/validation features implemented
   - [ ] No execution logic added

---

## Evidence Storage

**Location:** `results/phase_00/`

**Contents:**
- `unit_test_results.json`
- `integration_test_output.log`
- `property_test_results.json`
- `e2e_execution_logs/` (parse logs for 53 workflows)
- `system_log_samples.json`
- `cli_verification/` (terminal screenshots)
- `benchmarks/` (criterion reports)
- `schema_coverage_report.md`
- `adr_compliance_report.md`
- `anti_goal_drift_checklist.md`

---

## Blocking Issues

**Cannot exit Phase 00 if:**
- Any verification layer fails
- Schema coverage < 100%
- ADR compliance violations exist
- Performance regression > 10%
- 53 workflows don't all parse

---

**End of Phase 00 Criteria**
