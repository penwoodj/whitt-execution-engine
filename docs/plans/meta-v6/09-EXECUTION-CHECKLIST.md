# META-v6 Execution Checklist

> **Purpose:** Step-by-step execution checklist with gates

## Prerequisites

### Phase 0: Documentation Review

- [ ] Read `00-MASTER-PLAN.md`
- [ ] Read `01-OBJECTIVES-AND-SCOPE.md`
- [ ] Read `02-ARCHITECTURE.md`
- [ ] Read `03-SUBWORKFLOW-SPECIFICATIONS.md`
- [ ] Read `04-ITERATION-PROTOCOL.md`
- [ ] Read `05-QUALITY-BENCHMARK.md`
- [ ] Read `06-HOOKS-STRATEGY.md`
- [ ] Read `07-CONFIG-AND-INFRASTRUCTURE.md`
- [ ] Read `08-TESTING-STRATEGY.md`

**Gate 0: Documentation Reviewed**
- Pass: All 9 documents read
- Fail: Re-read missing documents

### Phase 1: Infrastructure Setup

- [ ] Docker installed and running
  - Command: `docker ps`
  - Expected: List running containers
- [ ] Docker Compose installed
  - Command: `docker compose version`
  - Expected: Version output
- [ ] Base docker-compose.yml exists
  - Command: `ls -la docker/docker-compose.yml`
  - Expected: File exists
- [ ] Entrypoint script exists
  - Command: `ls -la docker/entrypoint.sh`
  - Expected: File exists
- [ ] Model file exists
  - Command: `ls -la /models/Qwen3-5-9B-Q4_K_M.gguf`
  - Expected: File exists, size ~6.53 GB

**Gate 1: Infrastructure Ready**
- Pass: All checks pass
- Fail: Fix missing components

### Phase 2: Server Startup

- [ ] Start Docker container
  - Command: `docker compose -f docker/docker-compose.yml up -d`
  - Expected: Container started
- [ ] Verify container running
  - Command: `docker ps | grep whitt-llama-server`
  - Expected: Container in list
- [ ] Check port 8080 available
  - Command: `lsof -i :8080`
  - Expected: Port in use by docker-proxy
- [ ] Check server responding
  - Command: `curl http://localhost:8080/health`
  - Expected: 200 OK response
- [ ] Check model loaded
  - Command: `curl http://localhost:8080/props | grep Qwen`
  - Expected: Model props returned
- [ ] Check container logs
  - Command: `docker logs whitt-llama-server | tail -20`
  - Expected: No errors, server listening

**Gate 2: Server Running**
- Pass: Container running, server responding, model loaded
- Fail: Restart container or fix configuration

### Phase 3: Engine Build

- [ ] Check engine binary exists
  - Command: `ls -la target/release/whitt`
  - Expected: File exists
- [ ] If not exists, build engine
  - Command: `cargo build --release`
  - Expected: Build completes
- [ ] Verify engine version
  - Command: `./target/release/whitt --version`
  - Expected: Version output

**Gate 3: Engine Built**
- Pass: Engine binary exists and runs
- Fail: Build engine or fix compilation errors

### Phase 4: Script Preparation

- [ ] Check scripts exist
  - Command: `ls -la scripts/meta-v6/run-sw*.sh`
  - Expected: 5 scripts exist
- [ ] Check scripts executable
  - Command: `ls -la scripts/meta-v6/run-sw*.sh | grep -v "^-"`
  - Expected: No non-executable files
- [ ] Make scripts executable
  - Command: `chmod +x scripts/meta-v6/run-sw*.sh`
  - Expected: No output
- [ ] Verify scripts executable
  - Command: `ls -la scripts/meta-v6/run-sw*.sh | grep "^-rwx"`
  - Expected: 5 executable files

**Gate 4: Scripts Ready**
- Pass: All 5 scripts exist and are executable
- Fail: Create missing scripts or fix permissions

### Phase 5: Baseline Data Collection

- [ ] Create baseline prompts directory
  - Command: `mkdir -p docs/plans/meta-v6/baseline/prompts`
  - Expected: Directory created
- [ ] Export prompts from opencode DB
  - Method: SQL query on session messages
  - Count: 50-100 prompts
- [ ] Save prompts to baseline directory
  - Format: `prompt-001.md`, `prompt-002.md`, etc.
- [ ] Create single-shot baseline directory
  - Command: `mkdir -p docs/plans/meta-v6/baseline/single-shot`
  - Expected: Directory created
- [ ] Generate single-shot baselines
  - Method: Run SW1-SW5 on each prompt
  - Storage: `baseline-prompt-<id>-workflow.yml`

**Gate 5: Baseline Data Collected**
- Pass: 50-100 prompts exported, single-shot baselines generated
- Fail: Export more prompts or fix generation issues

### Phase 6: Baseline Quality Assessment

- [ ] Assess each baseline using rubric
  - Method: Manual assessment per `05-QUALITY-BENCHMARK.md`
  - Dimensions: 7 (schema, coverage, granularity, hooks, GWT, variety, template)
- [ ] Compute baseline average score
  - Method: Average all baseline scores
  - Target: 65/100 (example)
- [ ] Document baseline quality report
  - File: `docs/plans/meta-v6/baseline/baseline-quality-report.md`
  - Contents: Summary, per-prompt scores, quality distribution, common issues

**Gate 6: Baseline Quality Assessed**
- Pass: Baseline report complete, average score computed
- Fail: Complete assessment or fix rubric issues

## Phase 7: SW1 Execution (Task Analysis)

### Step 1: Run SW1 on Single Prompt

- [ ] Run SW1 on first baseline prompt
  - Command: `./scripts/meta-v6/run-sw1.sh --input docs/plans/meta-v6/baseline/prompts/prompt-001.md`
  - Expected: Execution completes
- [ ] Check output directory created
  - Command: `ls -la docs/benchmarks/outputs/meta-workflow/meta-*-sw1-*/ | head -20`
  - Expected: Directory exists
- [ ] Check log file present
  - Command: `ls -la docs/benchmarks/outputs/meta-workflow/meta-*-sw1-*/run.log`
  - Expected: File exists
- [ ] Check output.md present
  - Command: `ls -la docs/benchmarks/outputs/meta-workflow/meta-*-sw1-*/output.md`
  - Expected: File exists
- [ ] Check no panics
  - Command: `grep -i "panic\|crash\|fatal" docs/benchmarks/outputs/meta-workflow/meta-*-sw1-*/run.log`
  - Expected: No output
- [ ] Check workflow completed
  - Command: `grep "workflow:end" docs/benchmarks/outputs/meta-workflow/meta-*-sw1-*/run.log`
  - Expected: One match

**Gate 7a: SW1 Single Prompt Passes**
- Pass: Output created, no panics, workflow completes
- Fail: Debug and fix issues

### Step 2: Run SW1 on All Baseline Prompts

- [ ] Run SW1 on all prompts
  - Command: `for p in docs/plans/meta-v6/baseline/prompts/*.md; do ./scripts/meta-v6/run-sw1.sh --input "$p"; done`
  - Expected: All executions complete
- [ ] Check all output directories created
  - Command: `ls -la docs/benchmarks/outputs/meta-workflow/meta-*-sw1-*/ | wc -l`
  - Expected: Count = number of prompts
- [ ] Check all log files present
  - Command: `ls docs/benchmarks/outputs/meta-workflow/meta-*-sw1-*/run.log | wc -l`
  - Expected: Count = number of prompts
- [ ] Check all output.md files present
  - Command: `ls docs/benchmarks/outputs/meta-workflow/meta-*-sw1-*/output.md | wc -l`
  - Expected: Count = number of prompts
- [ ] Check no panics in any execution
  - Command: `grep -ci "panic\|crash\|fatal" docs/benchmarks/outputs/meta-workflow/meta-*-sw1-*/run.log`
  - Expected: 0
- [ ] Check all workflows completed
  - Command: `grep -c "workflow:end" docs/benchmarks/outputs/meta-workflow/meta-*-sw1-*/run.log`
  - Expected: Count = number of prompts

**Gate 7b: SW1 All Prompts Pass**
- Pass: All outputs created, no panics, all workflows complete
- Fail: Debug and fix issues, rerun

### Step 3: Assess SW1 Quality

- [ ] Assess SW1 outputs using rubric
  - Method: Manual assessment per `05-QUALITY-BENCHMARK.md`
  - Focus: Input coverage, task granularity
- [ ] Compare to baseline
  - Method: Compute score delta (current - baseline)
  - Target: +5 points minimum
- [ ] Document iteration report
  - File: `docs/plans/meta-v6/iterations/iter-01/iteration-report.md`
  - Contents: Metrics, gaps, improvements, next steps

**Gate 7c: SW1 Quality Assessed**
- Pass: Quality report complete, improvement documented
- Fail: Complete assessment or fix rubric issues

### Step 4: Iterate SW1 (If Needed)

- [ ] Identify quality gaps
  - Method: Compare current vs baseline per dimension
  - Focus: Where current < baseline
- [ ] Plan improvements
  - Method: For each gap, plan YAML change
- [ ] Apply improvements to SW1 YAML
  - File: `docs/benchmarks/workflows/sw1-task-analysis.yml`
  - Changes: Improve prompt, add hooks, fix structure
- [ ] Rerun SW1 on all prompts
  - Command: Same as Step 2
  - Expected: All executions complete
- [ ] Reassess quality
  - Method: Same as Step 3
  - Target: Improvement vs previous iteration
- [ ] Compare scores
  - Method: Compute delta (current - previous)
  - Target: +5 points minimum

**Gate 7d: SW1 Iteration Complete**
- Pass: Quality > baseline or max 5 iterations reached
- Fail: Continue iterating or escalate

**SW1 Pass Condition:**
- All prompts complete without panics
- Quality score > baseline
- Or max 5 iterations reached with improvement documented

## Phase 8: SW2-SW5 Execution

Repeat Phase 7 steps for SW2, SW3, SW4, SW5:

### SW2 Execution (Output Structure)

- [ ] Run SW2 on single SW1 output
- [ ] Verify output created, no panics
- [ ] Run SW2 on all SW1 outputs
- [ ] Verify all outputs created
- [ ] Assess SW2 quality
- [ ] Compare to baseline
- [ ] Iterate if needed (max 5 iterations)

**Gate 8: SW2 Passes**
- Pass: All outputs created, quality > baseline
- Fail: Debug and fix issues

### SW3 Execution (Category Mapping)

- [ ] Run SW3 on single SW2 output
- [ ] Verify output created, no panics
- [ ] Run SW3 on all SW2 outputs
- [ ] Verify all outputs created
- [ ] Assess SW3 quality
- [ ] Compare to baseline
- [ ] Iterate if needed (max 5 iterations)

**Gate 9: SW3 Passes**
- Pass: All outputs created, quality > baseline
- Fail: Debug and fix issues

### SW4 Execution (Struct Generation)

- [ ] Run SW4 on single SW3 output
- [ ] Verify output created, no panics
- [ ] Run SW4 on all SW3 outputs
- [ ] Verify all outputs created
- [ ] Assess SW4 quality
- [ ] Compare to baseline
- [ ] Iterate if needed (max 5 iterations)

**Gate 10: SW4 Passes**
- Pass: All outputs created, quality > baseline
- Fail: Debug and fix issues

### SW5 Execution (Workflow Assembly)

- [ ] Run SW5 on single SW4 output
- [ ] Verify output created, no panics
- [ ] Check schema validation passed
  - Command: `grep "schema_valid=true" docs/benchmarks/outputs/meta-workflow/meta-*-sw5-*/run.log`
  - Expected: One match
- [ ] Run SW5 on all SW4 outputs
- [ ] Verify all outputs created
- [ ] Check all schema validations passed
  - Command: `grep -c "schema_valid=true" docs/benchmarks/outputs/meta-workflow/meta-*-sw5-*/run.log`
  - Expected: Count = number of prompts
- [ ] Assess SW5 quality
- [ ] Compare to baseline
- [ ] Iterate if needed (max 5 iterations)

**Gate 11: SW5 Passes**
- Pass: All outputs created, all schema valid, quality > baseline
- Fail: Debug and fix issues

## Phase 9: End-to-End Validation

- [ ] Run all 5 SWs sequentially on single prompt
  - Command: `./scripts/meta-v6/run-sw1.sh --input <prompt>; ./scripts/meta-v6/run-sw2.sh; ./scripts/meta-v6/run-sw3.sh; ./scripts/meta-v6/run-sw4.sh; ./scripts/meta-v6/run-sw5.sh`
  - Expected: All complete
- [ ] Verify all 5 outputs created
  - Command: `ls -la docs/benchmarks/outputs/meta-workflow/meta-*-sw[1-5]-*/`
  - Expected: 5 directories
- [ ] Verify all 5 workflows completed
  - Command: `grep -c "workflow:end" docs/benchmarks/outputs/meta-workflow/meta-*-sw[1-5]-*/run.log`
  - Expected: 5 matches
- [ ] Verify no panics
  - Command: `grep -ci "panic\|crash\|fatal" docs/benchmarks/outputs/meta-workflow/meta-*-sw[1-5]-*/run.log`
  - Expected: 0
- [ ] Verify SW5 schema valid
  - Command: `grep "schema_valid=true" docs/benchmarks/outputs/meta-workflow/meta-*-sw5-*/run.log`
  - Expected: One match

**Gate 12: End-to-End Passes**
- Pass: All 5 SWs complete, no panics, schema valid
- Fail: Debug and fix issues

## Phase 10: Final Quality Assessment

- [ ] Assess final outputs using rubric
  - Method: Manual assessment per `05-QUALITY-BENCHMARK.md`
  - Scope: All final workflow.yml files
- [ ] Compute final average score
  - Method: Average all final scores
- [ ] Compare to baseline
  - Method: Compute delta (final - baseline)
  - Target: +10 points minimum
- [ ] Document final report
  - File: `docs/plans/meta-v6/iterations/FINAL-REPORT.md`
  - Contents: Summary, metrics, comparison, lessons learned

**Gate 13: Final Quality Assessed**
- Pass: Final report complete, improvement documented
- Fail: Complete assessment or fix rubric issues

## Phase 11: Unit Testing (After Live Validation Passes)

**ONLY proceed if Gate 13 passes**

- [ ] Create test file
  - File: `tests/meta_v6_integration.rs`
  - Expected: File created
- [ ] Write hook execution tests
  - Scope: Log, SaveTo, Bookmark, Gwt, RouteTo
  - Count: 5-10 tests
- [ ] Write GWT evaluation tests
  - Scope: Parsing, evaluation, first-match
  - Count: 5-10 tests
- [ ] Write template interpolation tests
  - Scope: Simple, nested, array
  - Count: 3-5 tests
- [ ] Write schema validation tests
  - Scope: Valid, invalid, provider key
  - Count: 3-5 tests
- [ ] Run all tests
  - Command: `cargo test --package whitt_execution_engine --test meta_v6_integration`
  - Expected: All pass
- [ ] Document test results
  - File: `docs/plans/meta-v6/tests/TEST-RESULTS.md`
  - Contents: Pass/fail counts, coverage, failures

**Gate 14: Unit Tests Pass**
- Pass: All tests pass, coverage ≥80%
- Fail: Fix test failures, improve coverage

## Phase 12: Documentation Finalization

- [ ] Update master plan with results
  - File: `docs/plans/meta-v6/00-MASTER-PLAN.md`
  - Changes: Add results section, update status
- [ ] Update iteration reports
  - Directory: `docs/plans/meta-v6/iterations/`
  - Changes: Ensure all reports complete
- [ ] Update quality reports
  - File: `docs/plans/meta-v6/baseline/baseline-quality-report.md`
  - Changes: Add final scores
- [ ] Create execution summary
  - File: `docs/plans/meta-v6/EXECUTION-SUMMARY.md`
  - Contents: Timeline, results, issues, lessons

**Gate 15: Documentation Complete**
- Pass: All reports updated, summary created
- Fail: Complete missing documentation

## Phase 13: Cleanup

- [ ] Stop Docker container
  - Command: `docker compose -f docker/docker-compose.yml down`
  - Expected: Container stopped
- [ ] Archive intermediate outputs
  - Method: Move to `archive/` directory
  - Reason: Clean up workspace
- [ ] Keep final outputs
  - Method: Keep in `docs/benchmarks/outputs/meta-workflow/`
  - Reason: Reference for future work

**Gate 16: Cleanup Complete**
- Pass: Container stopped, workspace cleaned
- Fail: Complete cleanup steps

## Execution Timeline

**Week 1:**
- Day 1: Phase 0-2 (Documentation review, infrastructure setup, server startup)
- Day 2: Phase 3-4 (Engine build, script preparation)
- Day 3-4: Phase 5-6 (Baseline collection and assessment)
- Day 5: Phase 7 (SW1 execution)

**Week 2:**
- Day 1-2: Phase 8 (SW2-SW5 execution)
- Day 3: Phase 9 (End-to-end validation)
- Day 4: Phase 10 (Final quality assessment)
- Day 5: Phase 11-12 (Unit testing, documentation)

**Week 3:**
- Day 1: Phase 13 (Cleanup)
- Day 2-5: Contingency (re-run failed steps, iterate more)

## Failure Handling

### Infrastructure Failures

**F1: Docker not running**
- Symptom: `docker ps` fails
- Fix: Install Docker, start service
- Gate: Gate 1

**F2: Container won't start**
- Symptom: `docker compose up -d` fails
- Fix: Check docker-compose.yml, fix config
- Gate: Gate 2

**F3: Server not responding**
- Symptom: `curl http://localhost:8080/health` fails
- Fix: Restart container, check port
- Gate: Gate 2

### Execution Failures

**F4: Engine won't build**
- Symptom: `cargo build --release` fails
- Fix: Fix compilation errors
- Gate: Gate 3

**F5: Scripts not executable**
- Symptom: Permission denied
- Fix: `chmod +x scripts/meta-v6/run-sw*.sh`
- Gate: Gate 4

**F6: SW panics**
- Symptom: `panic` in logs
- Fix: Debug panic, fix code
- Gate: Any gate

**F7: Schema validation fails**
- Symptom: `schema_valid=false` in logs
- Fix: Fix YAML structure
- Gate: Gate 11

### Quality Failures

**F8: Quality score < baseline**
- Symptom: Improvement not achieved
- Fix: Iterate more, up to 5 iterations
- Gate: Gate 7d, 8, 9, 10, 11

**F9: No improvement after 5 iterations**
- Symptom: Stalled iteration
- Fix: Reassess baseline, escalate
- Gate: Any iteration gate

### Testing Failures

**F10: Unit tests fail**
- Symptom: Test failures
- Fix: Fix tests or implementation
- Gate: Gate 14

## Success Criteria

**Overall Success:**
- All 5 SWs complete end-to-end without panics
- All schema validations pass
- Quality score > baseline (+10 points minimum)
- All unit tests pass
- All documentation complete

**Partial Success:**
- All 5 SWs complete end-to-end
- Some schema validations fail
- Quality score = baseline
- Unit tests not run

**Failure:**
- Any SW fails to complete
- Server or infrastructure issues
- Cannot achieve quality > baseline after 5 iterations

---

**Document Status:** Draft
**Last Updated:** 2026-06-14
**Author:** META-v6 Planning Session
**Review Status:** Pending