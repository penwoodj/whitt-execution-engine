# 11 - Execution Checklist

## Executive Summary

This document provides a step-by-step checklist for executing the Qwen 3.5-9B meta-workflow generator project. The checklist is organized by phase (Setup, Development, Testing, Validation, Deployment) and includes verification gates that must be passed before proceeding to the next phase.

The checklist is designed to be:
- **Linear:** Follow the checklist in order, from top to bottom
- **Verifiable:** Each step has a clear verification criterion
- **Atomic:** Each step can be completed independently
- **Documented:** All steps reference other documents for detailed instructions

**Usage:**

1. Print this checklist or keep it open in a separate window
2. Complete each step in order
3. Check the box when the step is complete
4. Verify the verification criterion before checking the box
5. If a step fails, resolve the issue before proceeding

**Completion Criteria:**

- All 6 phases completed
- All verification gates passed
- All tests passing (unit, integration, live system)
- All documentation complete
- All quality metrics met

## Phase 0: Preparation

### Step 0.1: Review Project Documentation

**Task:** Read all project documentation to understand the project scope, architecture, and requirements.

**Action:**

1. Read `00-MASTER-PLAN.md` — Master plan with 5-phase breakdown
2. Read `01-OBJECTIVES-AND-SCOPE.md` — Objectives, scope, acceptance criteria
3. Read `02-ARCHITECTURE.md` — System architecture and data flow
4. Read `03-SUBWORKFLOW-SPECIFICATIONS.md` — I/O contracts for SW1-SW5
5. Read `04-ITERATION-PROTOCOL.md` — Baseline creation and iteration process
6. Read `05-QUALITY-BENCHMARK.md` — Scoring rubrics and evaluation criteria
7. Read `06-TEST-DATASET.md` — Test dataset specification
8. Read `07-HOOKS-STRATEGY.md` — Hook strategy for all sub-workflows
9. Read `08-CONFIG-AND-INFRASTRUCTURE.md` — Docker configuration and infrastructure
10. Read `09-LIVE-SYSTEM-TESTING.md` — Live testing protocol
11. Read `10-UNIT-TEST-STRATEGY.md` — Unit testing strategy

**Verification Criterion:**

- [ ] All 11 documents read
- [ ] Project scope, architecture, and requirements understood
- [ ] Verification gates and quality criteria identified

**Estimated Time:** 30 minutes

**Reference Documents:** `docs/plans/meta-workflow-qwen35/00-11-*.md`

### Step 0.2: Verify Development Environment

**Task:** Verify that the development environment has all required tools and dependencies.

**Action:**

1. Verify Rust installation:
   ```bash
   rustc --version
   # Expected output: rustc 1.75.0 or later
   ```

2. Verify Cargo installation:
   ```bash
   cargo --version
   # Expected output: cargo 1.75.0 or later
   ```

3. Verify Docker installation:
   ```bash
   docker --version
   # Expected output: Docker version 24.0.0 or later
   ```

4. Verify Docker Compose installation:
   ```bash
   docker compose version
   # Expected output: Docker Compose version v2.20.0 or later
   ```

5. Verify yq installation:
   ```bash
   yq --version
   # Expected output: yq (https://github.com/mikefarah/yq/) version v4.35.2 or later
   ```

6. Verify jq installation:
   ```bash
   jq --version
   # Expected output: jq-1.6 or later
   ```

7. Verify curl installation:
   ```bash
   curl --version
   # Expected output: curl 7.81.0 or later
   ```

**Verification Criterion:**

- [ ] All 7 tools installed with correct versions
- [ ] Development environment ready for development

**Estimated Time:** 5 minutes

**Reference:** Section 1.1 in 08-CONFIG-AND-INFRASTRUCTURE.md

### Step 0.3: Clone or Verify Repository

**Task:** Clone the repository or verify that the repository is up to date.

**Action:**

1. If repository does not exist, clone it:
   ```bash
   cd /home/jon/code
   git clone https://github.com/whitt-execution-engine/whitt-execution-engine.git
   cd whitt-execution-engine
   ```

2. If repository exists, verify it's up to date:
   ```bash
   cd /home/jon/code/whitt-execution-engine
   git fetch origin
   git status
   # Expected output: "Your branch is up to date with 'origin/main'."
   ```

**Verification Criterion:**

- [ ] Repository cloned or verified
- [ ] Repository is up to date with origin/main

**Estimated Time:** 2 minutes

### Step 0.4: Create Working Directory

**Task:** Create a working directory for the meta-workflow generator project.

**Action:**

1. Create directory:
   ```bash
   mkdir -p docs/plans/meta-workflow-qwen35
   cd docs/plans/meta-workflow-qwen35
   ```

**Verification Criterion:**

- [ ] Directory created at `docs/plans/meta-workflow-qwen35/`
- [ ] Working directory verified

**Estimated Time:** 1 minute

**Reference:** Section 1 in 00-MASTER-PLAN.md

## Phase 1: Infrastructure Setup

### Step 1.1: Download Model File

**Task:** Download the Qwen 3.5-9B model file in GGUF format.

**Action:**

1. Create models directory:
   ```bash
   mkdir -p docker/models
   cd docker/models
   ```

2. Download model file:
   ```bash
   wget https://huggingface.co/Qwen/Qwen2.5-7B-Instruct-GGUF/resolve/main/Qwen2.5-7B-Instruct-Q4_K_XL.gguf \
     -O Qwen3.5-9B-UD-Q4_K_XL.gguf
   ```

3. Verify file size:
   ```bash
   ls -lh Qwen3.5-9B-UD-Q4_K_XL.gguf
   # Expected output: 4.8G - 6.0G (Q4_K_XL quantization for 9B model)
   ```

4. Verify file format:
   ```bash
   file Qwen3.5-9B-UD-Q4_K_XL.gguf
   # Expected output: Qwen3.5-9B-UD-Q4_K_XL.gguf: GGUF data
   ```

**Verification Criterion:**

- [ ] Model file downloaded to `docker/models/Qwen3.5-9B-UD-Q4_K_XL.gguf`
- [ ] File size is between 4GB and 6GB
- [ ] File format is GGUF (not corrupted)

**Estimated Time:** 15 minutes (download time depends on internet speed)

**Reference:** Section 3.1 in 08-CONFIG-AND-INFRASTRUCTURE.md

### Step 1.2: Create Docker Compose Configuration

**Task:** Create or verify the Docker Compose configuration.

**Action:**

1. Verify `docker/docker-compose.yml` exists:
   ```bash
   ls -lh docker/docker-compose.yml
   ```

2. If file does not exist, create it (see Section 1.1 in 08-CONFIG-AND-INFRASTRUCTURE.md for full content)

3. Verify configuration:
   ```bash
   docker compose config > /dev/null
   # Expected output: No output (exit code 0)
   ```

**Verification Criterion:**

- [ ] `docker/docker-compose.yml` exists
- [ ] Docker Compose configuration is valid

**Estimated Time:** 5 minutes

**Reference:** Section 1.1 in 08-CONFIG-AND-INFRASTRUCTURE.md

### Step 1.3: Create Entrypoint Scripts

**Task:** Create entrypoint scripts for llama.cpp server and whitt engine.

**Action:**

1. Create `docker/entrypoint.sh` for llama.cpp server (see Section 1.3 in 08-CONFIG-AND-INFRASTRUCTURE.md for full content)

2. Make script executable:
   ```bash
   chmod +x docker/entrypoint.sh
   ```

3. Verify script:
   ```bash
   cat docker/entrypoint.sh | head -20
   # Expected output: Script content (shebang, comments, etc.)
   ```

**Verification Criterion:**

- [ ] `docker/entrypoint.sh` created
- [ ] Script is executable
- [ ] Script content matches specification

**Estimated Time:** 5 minutes

**Reference:** Section 1.3 in 08-CONFIG-AND-INFRASTRUCTURE.md

### Step 1.4: Start Docker Compose

**Task:** Start the Docker Compose services.

**Action:**

1. Start services:
   ```bash
   cd docker
   docker compose up -d
   ```

2. Wait for services to start:
   ```bash
   sleep 30
   ```

3. Verify services are running:
   ```bash
   docker compose ps
   # Expected output:
   # NAME                STATUS
   # llama_cpp_vulkan    healthy
   # whitt_engine        healthy
   ```

**Verification Criterion:**

- [ ] Docker Compose services started
- [ ] Both services are healthy (not unhealthy, exited, or dead)

**Estimated Time:** 2 minutes

**Reference:** Section 4.1 in 08-CONFIG-AND-INFRASTRUCTURE.md

### Step 1.5: Verify Llama.cpp Health

**Task:** Verify that the llama.cpp server is healthy and responding to requests.

**Action:**

1. Test health endpoint:
   ```bash
   curl http://localhost:8080/health
   # Expected output: OK
   ```

2. Test completion endpoint:
   ```bash
   curl -X POST http://localhost:8080/completion \
     -H "Content-Type: application/json" \
     -d '{"prompt": "Hello, world!", "n_predict": 10}'
   # Expected output: JSON with "content" field
   ```

**Verification Criterion:**

- [ ] Health endpoint returns "OK"
- [ ] Completion endpoint returns valid JSON
- [ ] Llama.cpp server is healthy

**Estimated Time:** 2 minutes

**Reference:** Section 1.4 in 08-CONFIG-AND-INFRASTRUCTURE.md

## Phase 2: Sub-Workflow Development

### Step 2.1: Create SW1 Workflow YAML

**Task:** Create the SW1 (Task Deconstruction) workflow YAML.

**Action:**

1. Create directory:
   ```bash
   mkdir -p docs/plans/meta-workflow-qwen35/sub-workflows
   cd docs/plans/meta-workflow-qwen35/sub-workflows
   ```

2. Create `sw1.yml` (see Section 3 in 02-ARCHITECTURE.md for full specification)

3. Validate YAML syntax:
   ```bash
   yq eval '.' sw1.yml > /dev/null
   # Expected output: No output (exit code 0)
   ```

4. Validate schema compliance:
   ```bash
   whitt validate --schema ../../schema/unified-workflow-schema.yml --workflow sw1.yml
   # Expected output: "Workflow is valid" (exit code 0)
   ```

**Verification Criterion:**

- [ ] `sw1.yml` created
- [ ] YAML syntax valid
- [ ] Schema compliant with unified-workflow-schema.yml

**Estimated Time:** 10 minutes

**Reference:** Section 2.1 in 03-SUBWORKFLOW-SPECIFICATIONS.md

### Step 2.2: Create SW2 Workflow YAML

**Task:** Create the SW2 (Desired Output State) workflow YAML.

**Action:**

1. Create `sw2.yml` (see Section 3 in 02-ARCHITECTURE.md for full specification)

2. Validate YAML syntax:
   ```bash
   yq eval '.' sw2.yml > /dev/null
   ```

3. Validate schema compliance:
   ```bash
   whitt validate --schema ../../schema/unified-workflow-schema.yml --workflow sw2.yml
   ```

**Verification Criterion:**

- [ ] `sw2.yml` created
- [ ] YAML syntax valid
- [ ] Schema compliant

**Estimated Time:** 10 minutes

**Reference:** Section 3.1 in 03-SUBWORKFLOW-SPECIFICATIONS.md

### Step 2.3: Create SW3 Workflow YAML

**Task:** Create the SW3 (Agentic Categorization) workflow YAML.

**Action:**

1. Create `sw3.yml` (see Section 3 in 02-ARCHITECTURE.md for full specification)

2. Validate YAML syntax:
   ```bash
   yq eval '.' sw3.yml > /dev/null
   ```

3. Validate schema compliance:
   ```bash
   whitt validate --schema ../../schema/unified-workflow-schema.yml --workflow sw3.yml
   ```

**Verification Criterion:**

- [ ] `sw3.yml` created
- [ ] YAML syntax valid
- [ ] Schema compliant

**Estimated Time:** 10 minutes

**Reference:** Section 3.2 in 03-SUBWORKFLOW-SPECIFICATIONS.md

### Step 2.4: Create SW4 Workflow YAML

**Task:** Create the SW4 (YAML Substructure Translation) workflow YAML.

**Action:**

1. Create `sw4.yml` (see Section 3 in 02-ARCHITECTURE.md for full specification)

2. Validate YAML syntax:
   ```bash
   yq eval '.' sw4.yml > /dev/null
   ```

3. Validate schema compliance:
   ```bash
   whitt validate --schema ../../schema/unified-workflow-schema.yml --workflow sw4.yml
   ```

**Verification Criterion:**

- [ ] `sw4.yml` created
- [ ] YAML syntax valid
- [ ] Schema compliant

**Estimated Time:** 10 minutes

**Reference:** Section 3.3 in 03-SUBWORKFLOW-SPECIFICATIONS.md

### Step 2.5: Create SW5 Workflow YAML

**Task:** Create the SW5 (Final Workflow Assembly) workflow YAML.

**Action:**

1. Create `sw5.yml` (see Section 3 in 02-ARCHITECTURE.md for full specification)

2. Validate YAML syntax:
   ```bash
   yq eval '.' sw5.yml > /dev/null
   ```

3. Validate schema compliance:
   ```bash
   whitt validate --schema ../../schema/unified-workflow-schema.yml --workflow sw5.yml
   ```

**Verification Criterion:**

- [ ] `sw5.yml` created
- [ ] YAML syntax valid
- [ ] Schema compliant

**Estimated Time:** 10 minutes

**Reference:** Section 3.4 in 03-SUBWORKFLOW-SPECIFICATIONS.md

## Phase 3: Live System Testing

### Step 3.1: Pre-Test Validation

**Task:** Validate infrastructure, model, and configuration before running tests.

**Action:**

1. Verify infrastructure health:
   ```bash
   docker compose ps
   # Expected output: Both services healthy
   ```

2. Verify model availability:
   ```bash
   ls -lh docker/models/Qwen3.5-9B-UD-Q4_K_XL.gguf
   # Expected output: File exists, 4.8G-6.0G
   ```

3. Verify configuration:
   ```bash
   docker compose config | grep "CONTEXT_SIZE"
   # Expected output: CONTEXT_SIZE=262144
   ```

4. Validate workflow YAMLs:
   ```bash
   for workflow in docs/plans/meta-workflow-qwen35/sub-workflows/sw*.yml; do
     yq eval '.' "$workflow" > /dev/null
     whitt validate --schema docs/schema/unified-workflow-schema.yml --workflow "$workflow"
   done
   # Expected output: All YAMLs valid and schema compliant
   ```

**Verification Criterion:**

- [ ] Infrastructure healthy
- [ ] Model file available
- [ ] Configuration correct
- [ ] All workflow YAMLs valid and schema compliant

**Estimated Time:** 5 minutes

**Reference:** Section 1 in 09-LIVE-SYSTEM-TESTING.md

### Step 3.2: Run SW1 Test

**Task:** Run SW1 with test prompt and verify output.

**Action:**

1. Run SW1:
   ```bash
   docker compose exec whitt_engine /whitt/benchmark \
     --workflow /docs/plans/meta-workflow-qwen35/sub-workflows/sw1.yml \
     --input-prompt "Implement user authentication with JWT tokens"
   ```

2. Verify output file exists:
   ```bash
   docker compose exec whitt_engine ls -lh /outputs/sw1/task_list.json
   # Expected output: File exists, 1K-10K
   ```

3. Validate JSON output:
   ```bash
   docker compose exec whitt_engine cat /outputs/sw1/task_list.json | jq empty
   # Expected output: No output (exit code 0)
   ```

4. Validate task count:
   ```bash
   docker compose exec whitt_engine cat /outputs/sw1/task_list.json | jq 'length > 0'
   # Expected output: true
   ```

**Verification Criterion:**

- [ ] SW1 executed successfully
- [ ] Output file exists
- [ ] JSON valid
- [ ] Task count >= 1

**Estimated Time:** 5 minutes

**Reference:** Section 2.2 in 09-LIVE-SYSTEM-TESTING.md

### Step 3.3: Run SW2 Test

**Task:** Run SW2 with SW1 output and verify output.

**Action:**

1. Run SW2:
   ```bash
   docker compose exec whitt_engine /whitt/benchmark \
     --workflow /docs/plans/meta-workflow-qwen35/sub-workflows/sw2.yml \
     --input-json /outputs/sw1/task_list.json
   ```

2. Verify output file exists:
   ```bash
   docker compose exec whitt_engine ls -lh /outputs/sw2/output_states.json
   # Expected output: File exists, 1K-10K
   ```

3. Validate JSON output:
   ```bash
   docker compose exec whitt_engine cat /outputs/sw2/output_states.json | jq empty
   # Expected output: No output (exit code 0)
   ```

4. Validate output count matches task count:
   ```bash
   sw1_count=$(docker compose exec whitt_engine cat /outputs/sw1/task_list.json | jq 'length')
   sw2_count=$(docker compose exec whitt_engine cat /outputs/sw2/output_states.json | jq 'length')
   [ "$sw1_count" -eq "$sw2_count" ]
   # Expected output: Exit code 0 (counts match)
   ```

**Verification Criterion:**

- [ ] SW2 executed successfully
- [ ] Output file exists
- [ ] JSON valid
- [ ] Output count == task count

**Estimated Time:** 5 minutes

**Reference:** Section 2.3 in 09-LIVE-SYSTEM-TESTING.md

### Step 3.4: Run SW3 Test

**Task:** Run SW3 with SW1 and SW2 outputs and verify output.

**Action:**

1. Run SW3:
   ```bash
   docker compose exec whitt_engine /whitt/benchmark \
     --workflow /docs/plans/meta-workflow-qwen35/sub-workflows/sw3.yml \
     --input-json /outputs/sw1/task_list.json \
     --input-json /outputs/sw2/output_states.json
   ```

2. Verify output file exists:
   ```bash
   docker compose exec whitt_engine ls -lh /outputs/sw3/categorized_tasks.json
   # Expected output: File exists, 1K-10K
   ```

3. Validate JSON output:
   ```bash
   docker compose exec whitt_engine cat /outputs/sw3/categorized_tasks.json | jq empty
   # Expected output: No output (exit code 0)
   ```

4. Validate task count matches SW1 count:
   ```bash
   sw1_count=$(docker compose exec whitt_engine cat /outputs/sw1/task_list.json | jq 'length')
   sw3_count=$(docker compose exec whitt_engine cat /outputs/sw3/categorized_tasks.json | jq 'length')
   [ "$sw1_count" -eq "$sw3_count" ]
   # Expected output: Exit code 0 (counts match)
   ```

5. Validate canonical categories:
   ```bash
   docker compose exec whitt_engine cat /outputs/sw3/categorized_tasks.json | jq -r '.[].canonical_category' | \
     grep -v -E '^(file-read|transform-llm|validate-gate|loop-iterate|branch-decision|shell-execute|tool-call|checkpoint-state|notify-external|sub-workflow-ref)$' || \
     echo "All categories valid"
   # Expected output: "All categories valid"
   ```

**Verification Criterion:**

- [ ] SW3 executed successfully
- [ ] Output file exists
- [ ] JSON valid
- [ ] Task count == SW1 count
- [ ] All categories valid

**Estimated Time:** 5 minutes

**Reference:** Section 2.4 in 09-LIVE-SYSTEM-TESTING.md

### Step 3.5: Run SW4 Test

**Task:** Run SW4 with SW3 output and verify output.

**Action:**

1. Run SW4:
   ```bash
   docker compose exec whitt_engine /whitt/benchmark \
     --workflow /docs/plans/meta-workflow-qwen35/sub-workflows/sw4.yml \
     --input-json /outputs/sw3/categorized_tasks.json
   ```

2. Verify output file exists:
   ```bash
   docker compose exec whitt_engine ls -lh /outputs/sw4/workflow.yml
   # Expected output: File exists, 1K-10K
   ```

3. Validate YAML syntax:
   ```bash
   docker compose exec whitt_engine yq eval '.' /outputs/sw4/workflow.yml > /dev/null
   # Expected output: No output (exit code 0)
   ```

4. Validate step count:
   ```bash
   docker compose exec whitt_engine yq eval '.steps | length' /outputs/sw4/workflow.yml
   # Expected output: Number >= task count
   ```

5. Validate hook triggers:
   ```bash
   docker compose exec whitt_engine yq eval '.steps[].hooks | keys' /outputs/sw4/workflow.yml | jq -r '.[]' | \
     grep -v -E '^(before_step_starts|after_step_starts|after_step_fails|after_all_retries_exhausted|after_step_succeeds|on_requires_failed|after_loop_iteration_fails)$' || \
     echo "All triggers valid"
   # Expected output: "All triggers valid"
   ```

6. Validate hook actions:
   ```bash
   docker compose exec whitt_engine yq eval '.steps[].hooks[][] | keys' /outputs/sw4/workflow.yml | jq -r '.[]' | \
     grep -v -E '^(log|append_to|save_to|route_to|bookmark|notify|fail|shell|skip_step|skip_remaining|gwt|iterate_values)$' || \
     echo "All actions valid"
   # Expected output: "All actions valid"
   ```

**Verification Criterion:**

- [ ] SW4 executed successfully
- [ ] Output file exists
- [ ] YAML valid
- [ ] Step count >= task count
- [ ] All triggers valid
- [ ] All actions valid

**Estimated Time:** 5 minutes

**Reference:** Section 2.5 in 09-LIVE-SYSTEM-TESTING.md

### Step 3.6: Run SW5 Test

**Task:** Run SW5 with SW4 output and verify output.

**Action:**

1. Run SW5:
   ```bash
   docker compose exec whitt_engine /whitt/benchmark \
     --workflow /docs/plans/meta-workflow-qwen35/sub-workflows/sw5.yml \
     --input-yaml /outputs/sw4/workflow.yml
   ```

2. Verify output file exists:
   ```bash
   docker compose exec whitt_engine ls -lh /outputs/sw5/workflow.yml
   # Expected output: File exists, 1K-10K
   ```

3. Validate YAML syntax:
   ```bash
   docker compose exec whitt_engine yq eval '.' /outputs/sw5/workflow.yml > /dev/null
   # Expected output: No output (exit code 0)
   ```

4. Validate schema compliance:
   ```bash
   docker compose exec whitt_engine /whitt/validate \
     --schema /docs/schema/unified-workflow-schema.yml \
     --workflow /outputs/sw5/workflow.yml
   # Expected output: "Workflow is valid" (exit code 0)
   ```

5. Validate provider configuration:
   ```bash
   docker compose exec whitt_engine yq eval '.providers[0].key' /outputs/sw5/workflow.yml
   # Expected output: llama_cpp_with_vulkan
   ```

6. Validate model configuration:
   ```bash
   docker compose exec whitt_engine yq eval '.models[0].name' /outputs/sw5/workflow.yml
   # Expected output: Qwen3.5-9B-UD-Q4_K_XL.gguf
   ```

**Verification Criterion:**

- [ ] SW5 executed successfully
- [ ] Output file exists
- [ ] YAML valid
- [ ] Schema compliant
- [ ] Provider valid
- [ ] Model valid

**Estimated Time:** 5 minutes

**Reference:** Section 2.6 in 09-LIVE-SYSTEM-TESTING.md

## Phase 4: Post-Test Validation

### Step 4.1: Generate Validation Report

**Task:** Generate a validation report with all test results.

**Action:**

1. Create output directory:
   ```bash
   mkdir -p docs/benchmarks/outputs/meta-workflow-qwen35
   ```

2. Generate validation report (see Section 4.1 in 09-LIVE-SYSTEM-TESTING.md for template)

3. Verify validation report:
   ```bash
   cat docs/benchmarks/outputs/meta-workflow-qwen35/validation-report.md | grep "✅"
   # Expected output: Multiple "✅" marks (all tests passed)
   ```

**Verification Criterion:**

- [ ] Validation report created
- [ ] All tests marked as "✅ PASS"
- [ ] No tests marked as "❌ FAIL"

**Estimated Time:** 10 minutes

**Reference:** Section 4.1 in 09-LIVE-SYSTEM-TESTING.md

### Step 4.2: Calculate Quality Metrics

**Task:** Calculate quality metrics for each sub-workflow.

**Action:**

1. Calculate SW1 quality metrics (see Section 4.2 in 09-LIVE-SYSTEM-TESTING.md for commands)

2. Calculate SW2 quality metrics

3. Calculate SW3 quality metrics

4. Calculate SW4 quality metrics

5. Calculate SW5 quality metrics

6. Verify metrics meet targets (see Section 5 in 05-QUALITY-BENCHMARK.md for targets)

**Verification Criterion:**

- [ ] All quality metrics calculated
- [ ] All metrics meet targets
- [ ] No metric regression (> 5% degradation from baseline)

**Estimated Time:** 5 minutes

**Reference:** Section 4.2 in 09-LIVE-SYSTEM-TESTING.md

### Step 4.3: Validate Schema Compliance

**Task:** Validate final workflow YAML against schema.

**Action:**

1. Validate schema compliance:
   ```bash
   docker compose exec whitt_engine /whitt/validate \
     --schema /docs/schema/unified-workflow-schema.yml \
     --workflow /outputs/sw5/workflow.yml
   # Expected output: "Workflow is valid" (exit code 0)
   ```

2. Verify no unknown keys:
   ```bash
   docker compose exec whitt_engine yq eval 'keys | . - ["workflows", "version", "providers", "models", "steps", "hooks"]' /outputs/sw5/workflow.yml
   # Expected output: [] (no unknown keys)
   ```

**Verification Criterion:**

- [ ] Schema compliant
- [ ] No unknown keys
- [ ] No invalid values

**Estimated Time:** 2 minutes

**Reference:** Section 3.3 in 09-LIVE-SYSTEM-TESTING.md

## Phase 5: Unit Testing

### Step 5.1: Run Unit Tests

**Task:** Run all unit tests and verify they pass.

**Action:**

1. Run unit tests:
   ```bash
   cargo test --lib
   # Expected output: test result: ok. X passed; 0 failed
   ```

2. Verify test count:
   ```bash
   cargo test --lib 2>&1 | grep "test result"
   # Expected output: test result: ok. X passed; 0 failed (X >= 100)
   ```

**Verification Criterion:**

- [ ] All unit tests pass
- [ ] No test failures
- [ ] Test count >= 100

**Estimated Time:** 2 minutes

**Reference:** Section 6.1 in 10-UNIT-TEST-STRATEGY.md

### Step 5.2: Generate Coverage Report

**Task:** Generate test coverage report and verify coverage targets.

**Action:**

1. Generate coverage report:
   ```bash
   cargo tarpaulin --out Html --output-dir coverage
   # Expected output: Coverage report generated
   ```

2. Verify overall coverage:
   ```bash
   open coverage/index.html  # macOS
   xdg-open coverage/index.html  # Linux
   # Expected output: Coverage >= 80%
   ```

3. Verify critical module coverage:
   # Expected output: Hook actions >= 95%, GWT evaluator >= 95%

**Verification Criterion:**

- [ ] Overall coverage >= 80%
- [ ] Critical modules >= 95%
- [ ] No module below 60%

**Estimated Time:** 3 minutes

**Reference:** Section 6.3 in 10-UNIT-TEST-STRATEGY.md

### Step 5.3: Run Integration Tests

**Task:** Run integration tests and verify they pass.

**Action:**

1. Run integration tests:
   ```bash
   cargo test --test hooks_integration
   # Expected output: test result: ok. X passed; 0 failed
   ```

2. Run meta-workflow integration tests:
   ```bash
   cargo test --test meta_workflow_integration
   # Expected output: test result: ok. X passed; 0 failed
   ```

**Verification Criterion:**

- [ ] All integration tests pass
- [ ] Hook integration tests pass
- [ ] Meta-workflow integration tests pass

**Estimated Time:** 5 minutes

**Reference:** Section 2.4 in 10-UNIT-TEST-STRATEGY.md

## Phase 6: Documentation and Deployment

### Step 6.1: Review All Documentation

**Task:** Review all documentation for completeness and accuracy.

**Action:**

1. Review 00-MASTER-PLAN.md:
   ```bash
   cat docs/plans/meta-workflow-qwen35/00-MASTER-PLAN.md | grep -i "objective\|success\|phase"
   # Expected output: Objectives, success criteria, phases defined
   ```

2. Review 01-OBJECTIVES-AND-SCOPE.md:
   ```bash
   cat docs/plans/meta-workflow-qwen35/01-OBJECTIVES-AND-SCOPE.md | grep -i "in-scope\|out-of-scope\|acceptance"
   # Expected output: In-scope, out-of-scope, acceptance criteria defined
   ```

3. Review 02-ARCHITECTURE.md:
   ```bash
   cat docs/plans/meta-workflow-qwen35/02-ARCHITECTURE.md | grep -i "orchestrator\|pipeline\|data flow"
   # Expected output: Orchestrator, pipeline, data flow defined
   ```

4. Review 03-SUBWORKFLOW-SPECIFICATIONS.md:
   ```bash
   cat docs/plans/meta-workflow-qwen35/03-SUBWORKFLOW-SPECIFICATIONS.md | grep -i "SW1\|SW2\|SW3\|SW4\|SW5"
   # Expected output: All 5 sub-workflows specified
   ```

5. Review 04-ITERATION-PROTOCOL.md:
   ```bash
   cat docs/plans/meta-workflow-qwen35/04-ITERATION-PROTOCOL.md | grep -i "baseline\|comparison\|quality gate"
   # Expected output: Baseline, comparison, quality gates defined
   ```

6. Review 05-QUALITY-BENCHMARK.md:
   ```bash
   cat docs/plans/meta-workflow-qwen35/05-QUALITY-BENCHMARK.md | grep -i "scoring rubric\|evaluation\|criteria"
   # Expected output: Scoring rubric, evaluation, criteria defined
   ```

7. Review 06-TEST-DATASET.md:
   ```bash
   cat docs/plans/meta-workflow-qwen35/06-TEST-DATASET.md | grep -i "session\|prompt\|coverage"
   # Expected output: Sessions, prompts, coverage defined
   ```

8. Review 07-HOOKS-STRATEGY.md:
   ```bash
   cat docs/plans/meta-workflow-qwen35/07-HOOKS-STRATEGY.md | grep -i "trigger\|action\|hook"
   # Expected output: Triggers, actions, hooks defined
   ```

9. Review 08-CONFIG-AND-INFRASTRUCTURE.md:
   ```bash
   cat docs/plans/meta-workflow-qwen35/08-CONFIG-AND-INFRASTRUCTURE.md | grep -i "docker\|llama.cpp\|loadparams"
   # Expected output: Docker, llama.cpp, LoadParams defined
   ```

10. Review 09-LIVE-SYSTEM-TESTING.md:
    ```bash
    cat docs/plans/meta-workflow-qwen35/09-LIVE-SYSTEM-TESTING.md | grep -i "pre-test\|test execution\|post-test"
    # Expected output: Pre-test, test execution, post-test defined
    ```

11. Review 10-UNIT-TEST-STRATEGY.md:
    ```bash
    cat docs/plans/meta-workflow-qwen35/10-UNIT-TEST-STRATEGY.md | grep -i "unit test\|coverage\|mock"
    # Expected output: Unit tests, coverage, mocking defined
    ```

**Verification Criterion:**

- [ ] All 11 documents reviewed
- [ ] All documents complete
- [ ] All documents accurate

**Estimated Time:** 15 minutes

**Reference:** All documents in `docs/plans/meta-workflow-qwen35/`

### Step 6.2: Archive Test Artifacts

**Task:** Archive all test outputs for future reference.

**Action:**

1. Create archive directory:
   ```bash
   mkdir -p docs/benchmarks/outputs/meta-workflow-qwen35/archive/$(date +%Y-%m-%d)
   ```

2. Copy all outputs to archive:
   ```bash
   cp -r docs/benchmarks/outputs/meta-workflow-qwen35/sw* docs/benchmarks/outputs/meta-workflow-qwen35/archive/$(date +%Y-%m-%d)/
   cp docs/benchmarks/outputs/meta-workflow-qwen35/validation-report.md docs/benchmarks/outputs/meta-workflow-qwen35/archive/$(date +%Y-%m-%d)/
   cp docs/benchmarks/outputs/meta-workflow-qwen35/test-report.md docs/benchmarks/outputs/meta-workflow-qwen35/archive/$(date +%Y-%m-%d)/
   ```

3. Compress archive:
   ```bash
   cd docs/benchmarks/outputs/meta-workflow-qwen35/archive
   tar -czf $(date +%Y-%m-%d)-test-run.tar.gz $(date +%Y-%m-%d)/
   ```

4. Verify archive:
   ```bash
   ls -lh $(date +%Y-%m-%d)-test-run.tar.gz
   # Expected output: Archive file, 10K-50K
   ```

**Verification Criterion:**

- [ ] Archive directory created
- [ ] All outputs copied to archive
- [ ] Archive compressed
- [ ] Archive file verified

**Estimated Time:** 2 minutes

**Reference:** Section 4.4 in 09-LIVE-SYSTEM-TESTING.md

### Step 6.3: Stop Docker Compose

**Task:** Stop Docker Compose services (optional, for cleanup).

**Action:**

1. Stop services:
   ```bash
   cd docker
   docker compose down
   # Expected output: Containers stopped and removed
   ```

2. Verify services stopped:
   ```bash
   docker compose ps
   # Expected output: No containers running
   ```

**Verification Criterion:**

- [ ] Docker Compose services stopped
- [ ] No containers running

**Estimated Time:** 1 minute

**Reference:** Section 4.2 in 08-CONFIG-AND-INFRASTRUCTURE.md

## Completion Checklist

### Overall Completion Criteria

Verify that all completion criteria are met before marking the project as complete.

**Phase 0: Preparation**

- [ ] All 11 project documents read
- [ ] Development environment verified
- [ ] Repository cloned or verified
- [ ] Working directory created

**Phase 1: Infrastructure Setup**

- [ ] Model file downloaded (Qwen3.5-9B-UD-Q4_K_XL.gguf)
- [ ] Docker Compose configuration created
- [ ] Entrypoint scripts created
- [ ] Docker Compose services started
- [ ] Llama.cpp health verified

**Phase 2: Sub-Workflow Development**

- [ ] SW1 workflow YAML created and validated
- [ ] SW2 workflow YAML created and validated
- [ ] SW3 workflow YAML created and validated
- [ ] SW4 workflow YAML created and validated
- [ ] SW5 workflow YAML created and validated

**Phase 3: Live System Testing**

- [ ] Pre-test validation passed
- [ ] SW1 test passed
- [ ] SW2 test passed
- [ ] SW3 test passed
- [ ] SW4 test passed
- [ ] SW5 test passed

**Phase 4: Post-Test Validation**

- [ ] Validation report generated
- [ ] Quality metrics calculated (all targets met)
- [ ] Schema compliance validated

**Phase 5: Unit Testing**

- [ ] Unit tests passed (100+ tests)
- [ ] Coverage report generated (>= 80% overall, >= 95% critical modules)
- [ ] Integration tests passed

**Phase 6: Documentation and Deployment**

- [ ] All documentation reviewed
- [ ] Test artifacts archived
- [ ] Docker Compose services stopped (optional)

**Final Verification:**

- [ ] All 6 phases completed
- [ ] All verification gates passed
- [ ] All tests passing (unit, integration, live system)
- [ ] All documentation complete
- [ ] All quality metrics met

**Project Status:**

- [ ] ✅ COMPLETE — All tasks completed, all gates passed, all tests passing

## Troubleshooting

### Common Issues and Solutions

**Issue 1: Docker Compose fails to start**

**Symptoms:** `docker compose up -d` fails with error.

**Solutions:**

1. Check Docker is running: `docker ps`
2. Check Docker Compose version: `docker compose version` (must be v2.20.0 or later)
3. Check configuration syntax: `docker compose config`
4. Check for port conflicts: `lsof -i :8080` (kill conflicting process)
5. Rebuild containers: `docker compose up -d --build`

**Issue 2: Llama.cpp server unhealthy**

**Symptoms:** `docker compose ps` shows `llama_cpp_vulkan unhealthy`.

**Solutions:**

1. Check logs: `docker compose logs llama_cpp_server`
2. Verify model file exists: `ls -lh docker/models/Qwen3.5-9B-UD-Q4_K_XL.gguf`
3. Test health endpoint: `curl http://localhost:8080/health`
4. Restart service: `docker compose restart llama_cpp_server`
5. Rebuild service: `docker compose up -d --build llama_cpp_server`

**Issue 3: Workflow YAML validation fails**

**Symptoms:** `whitt validate --workflow sw1.yml` fails with error.

**Solutions:**

1. Check YAML syntax: `yq eval '.' sw1.yml`
2. Check schema compliance: `whitt validate --schema docs/schema/unified-workflow-schema.yml --workflow sw1.yml`
3. Fix YAML errors (indentation, missing colons, invalid keys)
4. Verify against schema: Compare with docs/schema/unified-workflow-schema.yml

**Issue 4: Unit tests fail**

**Symptoms:** `cargo test --lib` fails with errors.

**Solutions:**

1. Check Rust version: `rustc --version` (must be 1.75.0 or later)
2. Update dependencies: `cargo update`
3. Clean build: `cargo clean && cargo test --lib`
4. Check LSP diagnostics: Run `lsp_diagnostics` on changed files
5. Fix errors and re-run tests

**Issue 5: Live system tests fail**

**Symptoms:** SW1-SW5 tests fail with errors.

**Solutions:**

1. Check infrastructure health: `docker compose ps`
2. Check llama.cpp health: `curl http://localhost:8080/health`
3. Check workflow YAMLs: `whitt validate --schema docs/schema/unified-workflow-schema.yml --workflow sw1.yml`
4. Check output files: `ls -lh outputs/sw1/task_list.json`
5. Review logs: `docker compose logs whitt_engine | grep "SW1"`

## Conclusion

This execution checklist provides a comprehensive, step-by-step guide for executing the Qwen 3.5-9B meta-workflow generator project. The checklist is organized into 6 phases (Preparation, Infrastructure Setup, Sub-Workflow Development, Live System Testing, Unit Testing, Documentation and Deployment) with 30 total steps and 30 verification criteria.

**Completion Summary:**

- **Total Steps:** 30
- **Total Verification Criteria:** 30
- **Estimated Time:** 3-4 hours (excluding model download time)
- **Required Skills:** Rust, Docker, YAML, Linux shell

**Key Success Factors:**

1. **Follow checklist in order:** Do not skip steps or proceed out of order
2. **Verify each step:** Check the verification criterion before checking the box
3. **Resolve issues promptly:** If a step fails, resolve the issue before proceeding
4. **Document findings:** Keep notes on any issues encountered and resolutions
5. **Archive artifacts:** Archive test outputs for future reference and regression testing

**Next Steps After Completion:**

1. Iterate on sub-workflows if quality metrics not met (see 04-ITERATION-PROTOCOL.md)
2. Generate quality benchmark report (see 05-QUALITY-BENCHMARK.md)
3. If all quality gates pass, project is complete and ready for production
4. If quality gates not met, return to Phase 2 (Sub-Workflow Development) and iterate

**Project Status:**

- [ ] ✅ COMPLETE — All 30 steps completed, all 30 verification criteria passed