# Initial Plans Index

**Version**: 1.0
**Created**: 2026-03-26

---

## Overview

This index provides navigation to all implementation plans in the `initial-plans/` directory. Each plan corresponds to a phase of the roadmap and follows a structured execution methodology.

---

## Plan Files

### Phase 0: Foundation
- **[plan-00-foundation.md](./phase-0-foundation/plan-00-foundation.md)**
  - **Description**: Foundation compiler contract and local-first system-of-record
  - **Related ADR**: ADR-0001
  - **Related Research**: research-plan-01-foundation
  - **Status**: Not Started
  - **Estimated Time**: 4-6 weeks

### Phase 1: MVP Queue & Scheduler
- **[plan-01-mvp-queue.md](./phase-1-mvp-queue/plan-01-mvp-queue.md)**
  - **Description**: MVP work container, queue, scheduler, and human-gated safety
  - **Related ADR**: ADR-0002
  - **Related Research**: research-plan-02-mvp-queue-cli
  - **Status**: Not Started
  - **Estimated Time**: 6-8 weeks

### Phase 2: CLI & Backends
- **[plan-02-cli-backends.md](./phase-2-cli-backends/plan-02-cli-backends.md)**
  - **Description**: CLI, backends, and networking boundary
  - **Related ADR**: ADR-0003
  - **Related Research**: research-plan-03-networking-ui-backends
  - **Status**: Not Started
  - **Estimated Time**: 6-8 weeks

### Phase 3: Glyphnova UI
- **[plan-03-glyphnova-ui.md](./phase-3-glyphnova-ui/plan-03-glyphnova-ui.md)**
  - **Description**: Glyphnova desktop UI and multi-zoom control plane
  - **Related ADR**: ADR-0004
  - **Related Research**: research-plan-03-networking-ui-backends
  - **Status**: Not Started
  - **Estimated Time**: 6-8 weeks

### Phase 4: Quality Loops & Benchmarks
- **[plan-03-quality-loops.md](./phase-4-quality-loops/plan-03-quality-loops.md)**
  - **Description**: Quality loops, artifact workflows, and benchmark-driven file-type expansion
  - **Related ADR**: ADR-0005
  - **Related Research**: research-plan-03-quality-memory-search
  - **Status**: Not Started
  - **Estimated Time**: 8-10 weeks

### Phase 5: Memory & Search
- **[plan-04-memory-search.md](./phase-5-memory-search/plan-04-memory-search.md)**
  - **Description**: Local memory retrieval, web search, and web scraping
  - **Related ADR**: ADR-0006
  - **Related Research**: research-plan-03-quality-memory-search
  - **Status**: Not Started
  - **Estimated Time**: 10-12 weeks

### Phase 6: Automation
- **[plan-05-automation.md](./phase-6-automation/plan-05-automation.md)**
  - **Description**: Cron execution, git-branch experimentation, and workflow refinement
  - **Related ADR**: ADR-0007
  - **Related Research**: research-plan-04-automation-autonomy-metrics
  - **Status**: Not Started
  - **Estimated Time**: 8-10 weeks

### Phase 7: Autonomy & Metrics
- **[plan-06-autonomy-metrics.md](./phase-7-autonomy-metrics/plan-06-autonomy-metrics.md)**
  - **Description**: Autonomous loops and metrics-driven FE and BE UX expansion
  - **Related ADR**: ADR-0008
  - **Related Research**: research-plan-04-automation-autonomy-metrics
  - **Status**: Not Started
  - **Estimated Time**: 10-12 weeks

---

## How to Run Each Plan

### Standard Execution Flow

Each plan file follows this execution methodology:

#### Step 1: Code Reading Review (30-60 minutes)
- Read the related ADR(s)
- Read the related requirements documents
- Read the related research plan outputs
- Identify key implementation decisions and constraints
- Document dependencies and integration points

**Command**: Review plan file's "Code Review" section

#### Step 2: Web Research (60-120 minutes)
- Research best practices for implementation
- Find similar implementations in open-source projects
- Identify potential pitfalls and anti-patterns
- Gather evidence for design decisions
- Update plan file with research findings

**Command**: Update plan file's "Research Findings" section

#### Step 3: Start with Tests (Critical First Step)
- Write unit tests first for all components
- Write integration tests for component interactions
- Write property-based tests for edge cases
- Write end-to-end tests for full workflows
- Write performance tests for benchmarks
- All tests must fail initially (red state)

**Command**: `./implement.sh --plan <plan-id> --test-first`

#### Step 4: Implement Features Incrementally
- Implement features to pass unit tests
- Update implementation to pass integration tests
- Refine implementation to pass property tests
- Verify end-to-end workflow execution
- Optimize performance to meet benchmarks
- Each increment is verified and committed

**Command**: `./implement.sh --plan <plan-id> --continue`

#### Step 5: Incremental Batched Verification
- Run all 5 test layers after each feature increment
- Update plan file with verification results
- Document any issues found and resolutions
- Only proceed when all tests pass
- Batch verification checkpoints are documented

**Command**: `./implement.sh --plan <plan-id> --verify`

#### Step 6: Update Plan with Verified Code
- Document implementation details in plan file
- Include code snippets and examples
- Update verification status
- Mark plan as complete when all features verified
- Update SoT.md with completion status

**Command**: `./implement.sh --plan <plan-id> --complete`

---

## Using the Implementation Script

The `implement.sh` script automates the execution flow:

### Starting a New Plan
```bash
# Start plan-00 with research phase
./implement.sh --plan plan-00 --start

# This will:
# 1. Create phase directory structure
# 2. Run code review (document findings)
# 3. Run web research (document findings)
# 4. Generate test skeleton files
# 5. Create verification checkpoint structure
```

### Continuing Implementation
```bash
# Continue from last checkpoint
./implement.sh --plan plan-00 --continue

# This will:
# 1. Load last checkpoint status
# 2. Run pending tests
# 3. Implement next feature increment
# 4. Verify all test layers
# 5. Update plan file with results
```

### Running Verification Only
```bash
# Verify current implementation
./implement.sh --plan plan-00 --verify

# This will:
# 1. Run all 5 test layers
# 2. Generate verification report
# 3. Update plan file with verification status
```

### Marking Plan as Complete
```bash
# Mark plan as complete
./implement.sh --plan plan-00 --complete

# This will:
# 1. Run final full verification
# 2. Generate completion report
# 3. Update SoT.md with completion status
# 4. Generate summary documentation
```

### Checking Plan Status
```bash
# Check current status of a plan
./implement.sh --plan plan-00 --status

# This will:
# 1. Show current checkpoint
# 2. Show test pass/fail rates
# 3. Show overall completion percentage
# 4. List pending tasks
```

---

## Verification Layers

Each plan includes 4 verification layers:

### Layer 1: Unit Tests
**Purpose**: Test individual components in isolation
**Coverage**: Public APIs, data structures, utility functions
**Tools**: `cargo test --lib`
**Pass Criteria**: 100% of unit tests pass

### Layer 2: Integration Tests
**Purpose**: Test component interactions
**Coverage**: Multi-component workflows, data flow, state transitions
**Tools**: `cargo test --test '*'`
**Pass Criteria**: 100% of integration tests pass

### Layer 3: Property-Based Tests
**Purpose**: Test edge cases and invariants
**Coverage**: Input validation, boundary conditions, error handling
**Tools**: `proptest` crate with 1000 iterations each
**Pass Criteria**: 100% of property tests pass

### Layer 4: End-to-End Tests
**Purpose**: Test full workflow execution
**Coverage**: Complete workflows from YAML to output
**Tools**: `cargo test --test '*e2e*'`
**Pass Criteria**: 100% of e2e tests pass

---

## Dependency Management

### Sequential Plans
These plans must execute in order:
- plan-00 → plan-01 → plan-02

### Parallelizable Plans
These plans can execute in parallel after dependencies:
- plan-03 (UI) can start with plan-04 (Quality Loops)
- plan-05 (Memory) can start after plan-04
- plan-07 (Autonomy) can start after plan-06

### Blocking Dependencies
| Plan | Blocked By | Unblocks |
|-------|-------------|----------|
| plan-01 | plan-00 | plan-02 |
| plan-02 | plan-00, plan-01 | plan-03, plan-04 |
| plan-03 | plan-00, plan-01, plan-02 | plan-04 |
| plan-04 | plan-00, plan-01, plan-02 | plan-05 |
| plan-05 | plan-04 | plan-06 |
| plan-06 | plan-01, plan-02, plan-03, plan-04, plan-05 | plan-07 |
| plan-07 | plan-06 | (final phase) |

---

## Progress Tracking

All progress is tracked in:
1. **[SoT.md](./SoT.md)** - Master System of Record
2. **Individual plan files** - Plan-specific progress
3. **Verification checkpoints** - Batched verification results
4. **Implementation script** - Automated progress tracking

### Reading Progress

To check overall progress:
```bash
# View master status
cat SoT.md

# View individual plan status
cat ./phase-X-<name>/plan-XX-<name>.md

# Get JSON-formatted progress
./implement.sh --status --json
```

---

## Critical Review Process

Before any plan is marked complete, a critical review must pass:

### Review Criteria
1. **Upstream Factor 1**: ADR Compliance
   - Does implementation align with ADR decisions?
   - Are all ADR requirements satisfied?

2. **Upstream Factor 2**: Requirements Satisfaction
   - Are all requirements mapped to this plan satisfied?
   - Is coverage 100%?

3. **Upstream Factor 3**: Test Coverage
   - Do all 5 test layers pass?
   - Is code coverage adequate (>80%)?

4. **Upstream Factor 4**: Documentation
   - Is plan file updated with implementation details?
   - Are code examples and API docs complete?

5. **Upstream Factor 5**: Integration
   - Does implementation integrate cleanly with previous plans?
   - Are all dependencies satisfied?

### Running Critical Review
```bash
# Run critical review for a plan
./implement.sh --plan plan-00 --review

# This will:
# 1. Check ADR compliance
# 2. Verify requirements satisfaction
# 3. Run full test suite
# 4. Check documentation completeness
# 5. Verify integration points
# 6. Generate review report
```

### Review Output
Review produces:
- **Review Report**: `plan-XX-review.md`
- **Checklist**: Pass/fail for each upstream factor
- **Issues List**: Any issues found requiring resolution
- **Approval**: Only approved plans can proceed

---

## Integration with Opencode Tools

All plans are designed to integrate with:
- **OpenCode skill system** - Skills are loaded as needed
- **OpenCode verification** - Automated test running
- **OpenCode documentation** - API docs generation
- **OpenCode debugging** - Step-by-step execution inspection

### Opencode Integration Points
1. **Testing**: `test-driven-development` skill
2. **Verification**: `verification-before-completion` skill
3. **Code Review**: `code-review` skill
4. **Documentation**: `api-doc-generator` skill
5. **Debugging**: `systematic-debugging` skill

### Using Skills in Implementation
When implementing a plan, relevant skills are automatically loaded:
```bash
# Skills are loaded based on plan context
./implement.sh --plan plan-00 --skills test-driven-development,code-review
```

---

## Meta Instructions

### Best Practices for Plan Execution
1. **Never skip research phase** - Always do code review and web research
2. **Always write tests first** - Red-Green-Refactor cycle
3. **Verify incrementally** - After each feature, run all test layers
4. **Document everything** - Update plan file with all findings
5. **Respect dependencies** - Don't start a plan until dependencies complete
6. **Use the script** - Let the implementation script manage checkpoints
7. **Critical review required** - No plan completes without passing review

### Common Pitfalls to Avoid
1. **Implementing without tests** - Leads to unverified code
2. **Skipping verification layers** - Incomplete testing coverage
3. **Ignoring dependencies** - Causes integration failures
4. **Forgetting to update plan** - Progress is lost
5. **Skipping critical review** - Quality issues slip through
6. **Starting parallel plans early** - Causes merge conflicts
7. **Not using the script** - Manual tracking is error-prone

---

## Next Actions

### To Begin Implementation

1. **Read this index** - Understand the plan structure
2. **Read SoT.md** - Understand overall tracking
3. **Start with plan-00** - First plan in critical path
4. **Use implement.sh** - Automate the execution flow
5. **Track progress** - Update SoT.md after each checkpoint

### Example Starting Sequence
```bash
# 1. Navigate to plans directory
cd transpiler/opencode/docs/reports/initial-plans

# 2. Start plan-00
./implement.sh --plan plan-00 --start

# 3. Follow script prompts
# Script will guide through each step

# 4. Check progress
./implement.sh --plan plan-00 --status

# 5. Continue when ready
./implement.sh --plan plan-00 --continue
```

---

## Critical Review & Substructure Analysis

Two analysis documents have been produced to evaluate plan readiness:

- **[critical-review.md](./critical-review.md)** — Critical review of all 8 plan files with 5+ upstream factors each, rated for OpenCode single-plan execution success (PASS/NEEDS-WORK/FAIL)
- **[substructure-alignment.md](./substructure-alignment.md)** — Analysis of how each plan aligns with the agentic_workflow L1-L5 hierarchy from the reference YAML files

---

## Questions?

For questions about:
- **Plan structure**: See individual plan files
- **Execution flow**: See "How to Run Each Plan" section
- **Verification**: See "Verification Layers" section
- **Dependencies**: See "Dependency Management" section
- **Progress**: See SoT.md

---

## Summary

- **8 Plans** covering all roadmap phases
- **5 Verification Layers** per plan for comprehensive testing
- **Automated Script** for execution and tracking
- **Critical Review** with 5 upstream factors
- **Opencode Integration** for seamless tool support
- **Estimated Total Time**: 50-68 weeks (12-16 months)

**Status**: Ready for execution
