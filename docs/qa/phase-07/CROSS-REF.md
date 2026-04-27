# Cross-References — Phase 07: Final Validation

**Phase**: 07 - Final Validation
**Schema Version**: 2.0.0
**Last Updated**: 2026-04-26

---

## Schema References

| QA Area | Schema Section | Line Range | Key Fields |
|----------|---------------|--------------|-------------|
| System Log Verification | Lines 14-805 (full unified workflow schema) | Schema validation, logging infrastructure |
| Unit Test Verification | N/A (code quality) | Coverage > 90%, no warnings |
| Integration Test Verification | N/A (cross-module) | API contracts, state machine |
| CLI Live Testing | N/A (CLI integration) | Commands, error handling |
| Workflow Execution Tests | Lines 196-497 (agentic_workflow) | Steps, variables, output |
| Benchmark 5 Model | N/A (performance) | Baseline metrics |
| Benchmark 20 Model | N/A (performance) | Linear scaling metrics |
| Benchmark 50 Model | N/A (performance) | Stress test metrics |
| Benchmark 120 Model | N/A (performance) | Max stress test metrics |
| Workflow Generation Test | Lines 14-805 (full unified workflow schema) | Meta-validation key test |
| Cross-Phase Regression | Lines 14-805 (full unified workflow schema) | All phases 0-6 regression tests |
| Final Signoff | Lines 14-805 (full unified workflow schema) | Final acceptance criteria |

---

## Plan References

| QA Area | Plan File | Line Range | Key Tasks |
|----------|-----------|--------------|-------------|
| System Log Verification | [00-system-log-verification.md](../plans/07-final-validation/00-system-log-verification.md) | N/A | Validate logging infrastructure |
| Unit Test Verification | [01-unit-test-verification.md](../plans/07-final-validation/01-unit-test-verification.md) | N/A | Run all unit tests, verify coverage |
| Integration Test Verification | [02-integration-test-verification.md](../plans/07-final-validation/02-integration-test-verification.md) | N/A | Run all integration tests |
| CLI Live Testing | [03-cli-live-testing.md](../plans/07-final-validation/03-cli-live-testing.md) | N/A | Test all CLI commands |
| Workflow Execution Tests | [04-workflow-execution-tests.md](../plans/07-final-validation/04-workflow-execution-tests.md) | N/A | Execute all 50 workflows |
| Benchmark 5 Model | [05-benchmark-5-model.md](../plans/07-final-validation/05-benchmark-5-model.md) | N/A | Baseline performance test |
| Benchmark 20 Model | [06-benchmark-20-model.md](../plans/07-final-validation/06-benchmark-20-model.md) | N/A | Linear scaling test |
| Benchmark 50 Model | [07-benchmark-50-model.md](../plans/07-final-validation/07-benchmark-50-model.md) | N/A | Stress test |
| Benchmark 120 Model | [08-benchmark-120-model.md](../plans/07-final-validation/08-benchmark-120-model.md) | N/A | Max stress test (CRITICAL) |
| Workflow Generation Test | [09-workflow-generation-test.md](../plans/07-final-validation/09-workflow-generation-test.md) | N/A | Meta-validation key test |
| Cross-Phase Regression | [10-cross-phase-regression.md](../plans/07-final-validation/10-cross-phase-regression.md) | N/A | Regression tests for phases 0-6 |
| Final Signoff | [11-final-signoff.md](../plans/07-final-validation/11-final-signoff.md) | N/A | Final acceptance criteria and signoff |

---

## Related QA Areas

| Related Phase | Related QA Area | Relationship |
|---------------|------------------|-------------|
| Phase 00 | N/A | Foundation phase validates project setup |
| Phase 01 | N/A | Core execution engine tested |
| Phase 02 | N/A | CLI & backend tested |
| Phase 03 | N/A | Quality loops tested |
| Phase 04 | N/A | Memory & search tested |
| Phase 05 | N/A | Automation tested |
| Phase 06 | N/A | Autonomy & metrics tested |

---

## ADR References

| ADR | Title | Relevance |
|------|-------|-----------|
| All ADRs | Complete ADR Set | All architectural decisions validated in final phase |

---

## External Documentation

| Document | Path | Purpose |
|----------|--------|---------|
| Unified Workflow Schema | docs/schema/unified-workflow-schema.yml | Complete schema reference (805 lines) |
| Validation Framework | docs/plans/validation-criteria/framework.md | 7-layer validation system |
| Extended POC QA | docs/qa/extended-poc/QA-AREAS-EXTENDED-POC.md | QA format reference |
| AGENTS.md | AGENTS.md | QA area format, verification protocol |
| Example Workflows | docs/reports/requirements/example-workflows/requirements-oriented-auto/ | 50 workflows for testing |

---

## Task File References

| Task ID | Task File | Description |
|----------|-----------|-------------|
| 00 | [00-system-log-verification.md](../plans/07-final-validation/00-system-log-verification.md) | Validate system logs |
| 01 | [01-unit-test-verification.md](../plans/07-final-validation/01-unit-test-verification.md) | Verify unit tests |
| 02 | [02-integration-test-verification.md](../plans/07-final-validation/02-integration-test-verification.md) | Verify integration tests |
| 03 | [03-cli-live-testing.md](../plans/07-final-validation/03-cli-live-testing.md) | Test CLI commands |
| 04 | [04-workflow-execution-tests.md](../plans/07-final-validation/04-workflow-execution-tests.md) | Execute 50 workflows |
| 05 | [05-benchmark-5-model.md](../plans/07-final-validation/05-benchmark-5-model.md) | Baseline benchmark |
| 06 | [06-benchmark-20-model.md](../plans/07-final-validation/06-benchmark-20-model.md) | Linear scaling benchmark |
| 07 | [07-benchmark-50-model.md](../plans/07-final-validation/07-benchmark-50-model.md) | Stress benchmark |
| 08 | [08-benchmark-120-model.md](../plans/07-final-validation/08-benchmark-120-model.md) | Max stress benchmark (CRITICAL) |
| 09 | [09-workflow-generation-test.md](../plans/07-final-validation/09-workflow-generation-test.md) | Meta-validation test |
| 10 | [10-cross-phase-regression.md](../plans/07-final-validation/10-cross-phase-regression.md) | Regression tests |
| 11 | [11-final-signoff.md](../plans/07-final-validation/11-final-signoff.md) | Final signoff |

---

## Test File References

| Test File | Purpose |
|-----------|---------|
| tests/system_log_test.rs | System log validation |
| tests/unit_test.rs | All unit tests |
| tests/integration_test.rs | All integration tests |
| tests/cli_test.rs | CLI command tests |
| tests/workflow_execution_test.rs | Execute all 50 workflows |
| tests/benchmark_5_model.rs | Baseline benchmark |
| tests/benchmark_20_model.rs | Linear scaling benchmark |
| tests/benchmark_50_model.rs | Stress benchmark |
| tests/benchmark_120_model.rs | Max stress benchmark (CRITICAL) |
| tests/workflow_generation_test.rs | Meta-validation test |
| tests/cross_phase_regression_test.rs | Regression tests for phases 0-6 |

---

## Validation File References

| Validation File | Purpose |
|----------------|---------|
| [validation/00-system-log-verification.md](../plans/07-final-validation/00-system-log-verification.md) | Validation criteria for task 00 |
| [validation/01-unit-test-verification.md](../plans/07-final-validation/01-unit-test-verification.md) | Validation criteria for task 01 |
| [validation/02-integration-test-verification.md](../plans/07-final-validation/02-integration-test-verification.md) | Validation criteria for task 02 |
| [validation/03-cli-live-testing.md](../plans/07-final-validation/03-cli-live-testing.md) | Validation criteria for task 03 |
| [validation/04-workflow-execution-tests.md](../plans/07-final-validation/04-workflow-execution-tests.md) | Validation criteria for task 04 |
| [validation/05-benchmark-5-model.md](../plans/07-final-validation/05-benchmark-5-model.md) | Validation criteria for task 05 |
| [validation/06-benchmark-20-model.md](../plans/07-final-validation/06-benchmark-20-model.md) | Validation criteria for task 06 |
| [validation/07-benchmark-50-model.md](../plans/07-final-validation/07-benchmark-50-model.md) | Validation criteria for task 07 |
| [validation/08-benchmark-120-model.md](../plans/07-final-validation/08-benchmark-120-model.md) | Validation criteria for task 08 |
| [validation/09-workflow-generation-test.md](../plans/07-final-validation/09-workflow-generation-test.md) | Validation criteria for task 09 |
| [validation/10-cross-phase-regression.md](../plans/07-final-validation/10-cross-phase-regression.md) | Validation criteria for task 10 |
| [validation/11-final-signoff.md](../plans/07-final-validation/11-final-signoff.md) | Validation criteria for task 11 |

---

## Implementation Checklist

- [ ] Task 00: System Log Verification implemented and passing tests
- [ ] Task 01: Unit Test Verification implemented and passing tests
- [ ] Task 02: Integration Test Verification implemented and passing tests
- [ ] Task 03: CLI Live Testing implemented and passing tests
- [ ] Task 04: Workflow Execution Tests implemented and passing tests
- [ ] Task 05: Benchmark 5 Model implemented and passing tests
- [ ] Task 06: Benchmark 20 Model implemented and passing tests
- [ ] Task 07: Benchmark 50 Model implemented and passing tests
- [ ] Task 08: Benchmark 120 Model implemented and passing tests
- [ ] Task 09: Workflow Generation Test implemented and passing tests
- [ ] Task 10: Cross-Phase Regression implemented and passing tests
- [ ] Task 11: Final Signoff implemented and passing tests
- [ ] All unit tests passing (cargo test --lib)
- [ ] All integration tests passing (cargo test --test)
- [ ] Performance report generated
- [ ] Documentation complete
- [ ] LSP diagnostics clean on all changed files
- [ ] Build passes (cargo build --release --all-features)

---

**End of Cross-References for Phase 07**
