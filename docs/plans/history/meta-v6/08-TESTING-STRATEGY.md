# META-v6 Testing Strategy

> **Purpose:** Define live validation protocol and unit test strategy

## Testing Philosophy

**Two-Phase Approach:**
1. Phase 1: Live validation on real system
2. Phase 2: Unit tests (ONLY after Phase 1 passes)

**Rationale:**
- Live validation finds architectural issues faster
- Unit tests only verify confirmed working behavior
- Avoids wasted effort on unproven designs

**Reference:** AGENTS.md lines 17-313 (QA Mode framework)

## Phase 1: Live Validation

### Objective

Verify that all 5 SWs run end-to-end on the live Docker system without panics and produce valid outputs.

### Prerequisites

**Infrastructure:**
- [ ] Docker container `whitt-llama-server` running
- [ ] Port 8080 responding
- [ ] Model loaded: Qwen3-5-9B-Q4_K_M.gguf
- [ ] Engine built: `target/release/whitt`
- [ ] Run scripts executable: `scripts/meta-v6/run-sw*.sh`

**Data:**
- [ ] Baseline prompts collected: `docs/plans/meta-v6/baseline/prompts/*.md`
- [ ] Single-shot baselines generated: `docs/plans/meta-v6/baseline/single-shot/*.yml`

**Documentation:**
- [ ] Plan documents reviewed
- [ ] Execution checklist ready
- [ ] Quality rubric defined

### Validation Steps

#### Step 1: Infrastructure Health Check

**Command:**
```bash
# Check Docker running
docker ps | grep whitt-llama-server

# Check port available
lsof -i :8080

# Check server responding
curl http://localhost:8080/health

# Check model loaded
curl http://localhost:8080/props | grep Qwen

# Check engine built
ls -la target/release/whitt

# Check scripts executable
ls -la scripts/meta-v6/run-sw*.sh
```

**Expected Output:**
- Container running
- Port 8080 available
- Server responds with 200 OK
- Model props show Qwen3-5-9B
- Engine binary exists
- Scripts have execute permission

**Pass Criteria:** All checks pass

#### Step 2: SW1 Execution (Task Analysis)

**Command:**
```bash
./scripts/meta-v6/run-sw1.sh --input docs/plans/meta-v6/baseline/prompts/prompt-001.md
```

**Wait:** Up to 30 minutes (load timeout)

**Verify:**
```bash
# Check output directory created
ls -la docs/benchmarks/outputs/meta-workflow/meta-*-sw1-*/

# Check log file present
ls -la docs/benchmarks/outputs/meta-workflow/meta-*-sw1-*/run.log

# Check output.md created
ls -la docs/benchmarks/outputs/meta-workflow/meta-*-sw1-*/output.md

# Check no panics
grep -i "panic\|crash\|fatal" docs/benchmarks/outputs/meta-workflow/meta-*-sw1-*/run.log

# Check workflow completed
grep "workflow:end" docs/benchmarks/outputs/meta-workflow/meta-*-sw1-*/run.log
```

**Pass Criteria:**
- Output directory created
- Log file present
- output.md present
- No panics or crashes
- Workflow end event present

#### Step 3: SW2 Execution (Output Structure)

**Command:**
```bash
./scripts/meta-v6/run-sw2.sh
```

**Verify:**
```bash
# Check output directory created
ls -la docs/benchmarks/outputs/meta-workflow/meta-*-sw2-*/

# Check categories.md created
ls -la docs/benchmarks/outputs/meta-workflow/meta-*-sw2-*/output.md

# Check no panics
grep -i "panic\|crash\|fatal" docs/benchmarks/outputs/meta-workflow/meta-*-sw2-*/run.log

# Check workflow completed
grep "workflow:end" docs/benchmarks/outputs/meta-workflow/meta-*-sw2-*/run.log
```

**Pass Criteria:** Same as SW1

#### Step 4: SW3 Execution (Category Mapping)

**Command:**
```bash
./scripts/meta-v6/run-sw3.sh
```

**Verify:**
```bash
# Check output directory created
ls -la docs/benchmarks/outputs/meta-workflow/meta-*-sw3-*/

# Check structs.md created
ls -la docs/benchmarks/outputs/meta-workflow/meta-*-sw3-*/output.md

# Check no panics
grep -i "panic\|crash\|fatal" docs/benchmarks/outputs/meta-workflow/meta-*-sw3-*/run.log

# Check workflow completed
grep "workflow:end" docs/benchmarks/outputs/meta-workflow/meta-*-sw3-*/run.log
```

**Pass Criteria:** Same as SW1

#### Step 5: SW4 Execution (Struct Generation)

**Command:**
```bash
./scripts/meta-v6/run-sw4.sh
```

**Verify:**
```bash
# Check output directory created
ls -la docs/benchmarks/outputs/meta-workflow/meta-*-sw4-*/

# Check workflow.yml skeleton created
ls -la docs/benchmarks/outputs/meta-workflow/meta-*-sw4-*/workflow.yml

# Check no panics
grep -i "panic\|crash\|fatal" docs/benchmarks/outputs/meta-workflow/meta-*-sw4-*/run.log

# Check workflow completed
grep "workflow:end" docs/benchmarks/outputs/meta-workflow/meta-*-sw4-*/run.log
```

**Pass Criteria:** Same as SW1, plus workflow.yml skeleton present

#### Step 6: SW5 Execution (Workflow Assembly)

**Command:**
```bash
./scripts/meta-v6/run-sw5.sh
```

**Verify:**
```bash
# Check output directory created
ls -la docs/benchmarks/outputs/meta-workflow/meta-*-sw5-*/

# Check final workflow.yml created
ls -la docs/benchmarks/outputs/meta-workflow/meta-*-sw5-*/workflow.yml

# Check no panics
grep -i "panic\|crash\|fatal" docs/benchmarks/outputs/meta-workflow/meta-*-sw5-*/run.log

# Check workflow completed
grep "workflow:end" docs/benchmarks/outputs/meta-workflow/meta-*-sw5-*/run.log

# Check schema validation passed
grep "schema_valid=true" docs/benchmarks/outputs/meta-workflow/meta-*-sw5-*/run.log
```

**Pass Criteria:** Same as SW1, plus schema valid

#### Step 7: End-to-End Validation (All SWs)

**Command:**
```bash
# Run all 5 SWs sequentially on single prompt
./scripts/meta-v6/run-sw1.sh --input docs/plans/meta-v6/baseline/prompts/prompt-001.md
./scripts/meta-v6/run-sw2.sh
./scripts/meta-v6/run-sw3.sh
./scripts/meta-v6/run-sw4.sh
./scripts/meta-v6/run-sw5.sh
```

**Verify:**
```bash
# Check all 5 output directories exist
ls -la docs/benchmarks/outputs/meta-workflow/meta-*-sw[1-5]-*/

# Check all 5 workflow completed
grep "workflow:end" docs/benchmarks/outputs/meta-workflow/meta-*-sw[1-5]-*/run.log

# Check SW5 schema valid
grep "schema_valid=true" docs/benchmarks/outputs/meta-workflow/meta-*-sw5-*/run.log

# Check no panics in any SW
grep -i "panic\|crash\|fatal" docs/benchmarks/outputs/meta-workflow/meta-*-sw[1-5]-*/run.log
```

**Pass Criteria:** All checks pass

#### Step 8: Dataset Validation (All Baseline Prompts)

**Command:**
```bash
# Run all 5 SWs on all baseline prompts
for prompt in docs/plans/meta-v6/baseline/prompts/*.md; do
  echo "Processing $prompt..."
  ./scripts/meta-v6/run-sw1.sh --input "$prompt"
  ./scripts/meta-v6/run-sw2.sh
  ./scripts/meta-v6/run-sw3.sh
  ./scripts/meta-v6/run-sw4.sh
  ./scripts/meta-v6/run-sw5.sh
done
```

**Verify:**
```bash
# Count successful completions
completion_count=$(grep -c "workflow:end" docs/benchmarks/outputs/meta-workflow/meta-*-sw5-*/run.log)
echo "Successful completions: $completion_count"

# Count schema validations
schema_count=$(grep -c "schema_valid=true" docs/benchmarks/outputs/meta-workflow/meta-*-sw5-*/run.log)
echo "Schema validations: $schema_count"

# Check for panics
panic_count=$(grep -ci "panic\|crash\|fatal" docs/benchmarks/outputs/meta-workflow/meta-*-sw[1-5]-*/run.log)
echo "Panics: $panic_count"
```

**Pass Criteria:**
- `completion_count` = total prompts
- `schema_count` = total prompts
- `panic_count` = 0

### Live Validation Gates

**Gate 1: Infrastructure Ready**
- All health checks pass
- Server responding
- Model loaded

**Gate 2: SW1 Passes**
- Output created
- No panics
- Workflow completes

**Gate 3: SW2 Passes**
- Output created
- No panics
- Workflow completes

**Gate 4: SW3 Passes**
- Output created
- No panics
- Workflow completes

**Gate 5: SW4 Passes**
- Output created
- No panics
- Workflow completes
- YAML skeleton present

**Gate 6: SW5 Passes**
- Output created
- No panics
- Workflow completes
- Schema valid

**Gate 7: End-to-End Passes**
- All 5 SWs complete
- No panics
- Schema valid

**Gate 8: Dataset Passes**
- All prompts complete
- All schema valid
- No panics

### Live Validation Failure Modes

**F1: Server Not Responding**
- Symptom: `curl http://localhost:8080/health` fails
- Fix: Restart container, check port
- Gate: Gate 1

**F2: Model Load Timeout**
- Symptom: `--load-timeout 1800` exceeded
- Fix: Increase timeout, check memory
- Gate: Gate 1

**F3: SW Panics**
- Symptom: `panic` in logs
- Fix: Debug panic, fix code
- Gate: Any gate

**F4: Schema Validation Fails**
- Symptom: `schema_valid=false` in logs
- Fix: Fix YAML structure
- Gate: Gate 6

**F5: Output File Missing**
- Symptom: `output.md` or `workflow.yml` not created
- Fix: Check hooks, fix file paths
- Gate: Any gate

**F6: Workflow Not Complete**
- Symptom: No `workflow:end` event
- Fix: Check for errors, fix workflow
- Gate: Any gate

## Phase 2: Unit Testing

### Objective

**ONLY after Phase 1 passes:** Verify that execution engine paths are tested.

**Scope:**
- Hook execution paths
- GWT expression evaluation
- Template interpolation
- Schema validation
- Runner firing points

**NOT in Scope:**
- LLM backend (external dependency)
- Docker container (external dependency)
- Model behavior (external dependency)

### Test Locations

**New Test File:** `tests/meta_v6_integration.rs`

**Test Modules:**
- `tests/hooks_execution.rs` - Hook path tests
- `tests/gwt_evaluation.rs` - GWT evaluator tests
- `tests/template_interpolation.rs` - Template tests
- `tests/schema_validation.rs` - Schema validation tests

### Test Categories

#### Category 1: Hook Execution Paths

**Test Hook Action Execution:**
```rust
#[test]
fn test_log_action_execution() {
    // Given
    let engine = HookEngine::new();
    let action = HookAction::Log(LogAction {
        message: "Test message".to_string(),
        to_file_path: Some("/tmp/test.log".to_string()),
        level: LogLevel::Info,
    });

    // When
    let result = execute_action(&engine, &action, context);

    // Then
    assert!(result.is_continue());
    assert!(Path::new("/tmp/test.log").exists());
}
```

**Test Hook Trigger Firing:**
```rust
#[test]
fn test_before_step_starts_trigger_fires() {
    // Given
    let workflow = WorkflowFile::from_str(workflow_yaml_with_hooks).unwrap();
    let runner = WorkflowRunner::new(workflow);

    // When
    runner.execute_step("test_step").await;

    // Then
    assert_eq!(trigger_count("before_step_starts"), 1);
}
```

**Test Hook Result Merging:**
```rust
#[test]
fn test_hook_result_merge_priority() {
    // Given
    let result1 = HookResult::Continue;
    let result2 = HookResult::Fail("Error".to_string());
    let result3 = HookResult::RouteTo(vec!["step2".to_string()]);

    // When
    let merged = result1.merge(result2).merge(result3);

    // Then
    assert_eq!(merged, HookResult::Fail("Error".to_string()));
}
```

#### Category 2: GWT Expression Evaluation

**Test GWT Parsing:**
```rust
#[test]
fn test_gwt_parse_simple_expression() {
    // Given
    let expression = "field > 0";

    // When
    let parsed = GwtExpression::parse(expression);

    // Then
    assert!(parsed.is_ok());
}
```

**Test GWT Evaluation:**
```rust
#[test]
fn test_gwt_evaluate_numeric_comparison() {
    // Given
    let expression = "value > 0";
    let context = json!({"value": 10});

    // When
    let result = evaluate_gwt(expression, &context);

    // Then
    assert_eq!(result, true);
}
```

**Test GWT First-Match Semantics:**
```rust
#[test]
fn test_gwt_first_match_semantics() {
    // Given
    let clauses = vec![
        GwtClause {
            given: "type".to_string(),
            when: "== 'json'".to_string(),
            then: vec![HookAction::Log(...)],
            else: None,
        },
        GwtClause {
            given: "type".to_string(),
            when: "== 'yaml'".to_string(),
            then: vec![HookAction::Log(...)],
            else: None,
        },
    ];
    let context = json!({"type": "json"});

    // When
    let result = evaluate_gwt_clauses(&clauses, &context);

    // Then
    assert_eq!(result.len(), 1); // Only first clause matched
}
```

#### Category 3: Template Interpolation

**Test Simple Interpolation:**
```rust
#[test]
fn test_template_simple_interpolation() {
    // Given
    let template = "Hello {{name}}";
    let variables = json!({"name": "World"});

    // When
    let result = interpolate_template(template, &variables);

    // Then
    assert_eq!(result, "Hello World");
}
```

**Test Nested Interpolation:**
```rust
#[test]
fn test_template_nested_interpolation() {
    // Given
    let template = "{{user.name}} is {{user.age}} years old";
    let variables = json!({"user": {"name": "Alice", "age": 30}});

    // When
    let result = interpolate_template(template, &variables);

    // Then
    assert_eq!(result, "Alice is 30 years old");
}
```

**Test Array Interpolation:**
```rust
#[test]
fn test_template_array_interpolation() {
    // Given
    let template = "{{items[0].name}}";
    let variables = json!({"items": [{"name": "Item1"}, {"name": "Item2"}]});

    // When
    let result = interpolate_template(template, &variables);

    // Then
    assert_eq!(result, "Item1");
}
```

#### Category 4: Schema Validation

**Test Schema Valid:**
```rust
#[test]
fn test_schema_valid_workflow() {
    // Given
    let workflow_yaml = valid_workflow_yaml();

    // When
    let result = WorkflowFile::validate(&workflow_yaml);

    // Then
    assert!(result.is_valid());
}
```

**Test Schema Invalid:**
```rust
#[test]
fn test_schema_invalid_workflow() {
    // Given
    let workflow_yaml = invalid_workflow_yaml();

    // When
    let result = WorkflowFile::validate(&workflow_yaml);

    // Then
    assert!(!result.is_valid());
}
```

**Test Provider Key Required:**
```rust
#[test]
fn test_provider_key_required() {
    // Given
    let workflow_yaml = workflow_without_provider();

    // When
    let result = WorkflowFile::validate(&workflow_yaml);

    // Then
    assert!(!result.is_valid());
    assert!(result.errors().iter().any(|e| e.contains("provider")));
}
```

### Test Execution

**Run Tests:**
```bash
cargo test --package whitt_execution_engine --test meta_v6_integration
```

**Expected Output:**
```
running 20 tests
test hooks_execution::test_log_action_execution ... ok
test hooks_execution::test_hook_result_merge_priority ... ok
test gwt_evaluation::test_gwt_parse_simple_expression ... ok
test gwt_evaluation::test_gwt_evaluate_numeric_comparison ... ok
test template_interpolation::test_template_simple_interpolation ... ok
test schema_validation::test_schema_valid_workflow ... ok
...

test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured
```

**Run with Coverage:**
```bash
cargo test --package whitt_execution_engine --test meta_v6_integration -- --nocapture
```

## Testing Timeline

### Phase 1: Live Validation

**Week 1:**
- Day 1-2: Infrastructure setup and health checks
- Day 3-4: SW1-SW5 individual validation
- Day 5: End-to-end validation on single prompt

**Week 2:**
- Day 1-3: Dataset validation on all baseline prompts
- Day 4: Quality assessment
- Day 5: Document findings

### Phase 2: Unit Testing (After Phase 1 Passes)

**Week 3:**
- Day 1-2: Write hook execution tests
- Day 3-4: Write GWT evaluation tests
- Day 5: Write template interpolation tests

**Week 4:**
- Day 1-2: Write schema validation tests
- Day 3-4: Run all tests, fix failures
- Day 5: Document test coverage

## Test Documentation

### Test Plan

**File:** `docs/plans/meta-v6/tests/TEST-PLAN.md`

**Contents:**
- Test objectives
- Test scope
- Test cases
- Expected results

### Test Results

**File:** `docs/plans/meta-v6/tests/TEST-RESULTS.md`

**Contents:**
- Live validation results
- Unit test results
- Coverage metrics
- Failures and fixes

### Test Coverage

**File:** `docs/plans/meta-v6/tests/TEST-COVERAGE.md`

**Contents:**
- Hook coverage (7/10 triggers)
- Action coverage (12/12 actions)
- GWT expression coverage
- Template coverage
- Schema coverage

## QA Framework Alignment

**Reference:** AGENTS.md lines 17-313

**Mode:** QA Mode
**Evidence Required:**
- Unit test output
- Live validation logs
- Schema validation results
- Manual verification

**Severity Levels:**
- HIGH: Blocks functionality (immediate fix)
- MEDIUM: Partial functionality (fix in current cycle)
- LOW: Code quality (document, fix opportunistically)

**Failure Handling:**
- Max 3 fix attempts per issue
- After 3 failures, escalate

## Test Metrics

### Live Validation Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Infrastructure Health | 100% | Health check pass rate |
| SW1 Completion | 100% | Output files present |
| SW2 Completion | 100% | Output files present |
| SW3 Completion | 100% | Output files present |
| SW4 Completion | 100% | Output files present |
| SW5 Completion | 100% | Output files present |
| Schema Validity | 100% | schema_valid=true count |
| Panics | 0 | grep panic count |

### Unit Test Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Test Pass Rate | 100% | cargo test output |
| Code Coverage | ≥80% | cargo tarpaulin |
| Hook Coverage | 7/10 triggers | Manual count |
| Action Coverage | 12/12 actions | Manual count |
| GWT Coverage | ≥90% patterns | Manual count |

## Testing Tools

### Required Tools

- `cargo` (Rust package manager)
- `docker` (Container runtime)
- `docker-compose` (Container orchestration)
- `curl` (HTTP client)
- `grep` (Text search)
- `ls` (File listing)
- `cat` (File reading)

### Optional Tools

- `cargo-tarpaulin` (Coverage)
- `cargo-nextest` (Parallel test runner)
- `watchexec` (Watch mode)

---

**Document Status:** Draft
**Last Updated:** 2026-06-14
**Author:** META-v6 Planning Session
**Review Status:** Pending