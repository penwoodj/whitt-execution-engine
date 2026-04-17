# Phase 03: Glyphnova UI - Validation Criteria

**Phase Focus:** Desktop shell, queue visualization, scope indicators, navigation, drag-drop
**Entry Criteria:** Phases 00, 01, and 02 complete
**Estimated Duration:** 3-4 weeks
**Blocking for:** Phases 04, 05, 06, 07, 08

---

## Phase Overview

Phase 03 implements the Glyphnova desktop UI. This phase provides a graphical interface for workflow management, queue visualization, and real-time monitoring. The UI displays scope indicators, enables safe navigation, and supports drag-drop operations for workflow composition.

**Critical Success Factors:**
1. Desktop shell starts without crashing
2. Queue visualization matches scheduler state
3. Scope indicators display correctly
4. Navigation is safe (no data loss)
5. Drag-drop operations call scheduler APIs correctly

---

## Phase Entry Criteria

**Status:** Must be verified before starting Phase 03

**Verification Commands:**
```bash
# Verify Phase 00 exit
cargo test --test phase_00_integration -- --test-threads=1

# Verify Phase 01 exit
cargo test --test phase_01_integration -- --test-threads=1

# Verify Phase 02 exit
cargo test --test phase_02_integration -- --test-threads=1

# Verify schema coverage
cargo run --bin schema_audit -- --phase 0 --output phase_00_coverage.md
cargo run --bin schema_audit -- --phase 1 --output phase_01_coverage.md
cargo run --bin schema_audit -- --phase 2 --output phase_02_coverage.md
# Expected: 100% coverage for all phases
```

**Prerequisites:**
- [ ] Phase 00 exit criteria verified (all 7 layers)
- [ ] Phase 01 exit criteria verified (all 7 layers)
- [ ] Phase 02 exit criteria verified (all 7 layers)
- [ ] Schema coverage 100% for all phases (00, 01, 02)
- [ ] ADR compliance verified for all phases
- [ ] Cross-phase regression clean (Phases 00+01+02)
- [ ] Desktop development environment ready (GUI framework dependencies)
- [ ] Scheduler APIs exposed for UI consumption

**Blocking Violations:**
- Unresolved Phase 00, 01, or 02 failures
- Schema coverage < 100% for any phase
- ADR compliance violations
- Cross-phase regression detected
- Scheduler APIs not exposed

---

## Desktop Shell

**Requirement:** Desktop shell starts without crashing

### Shell Features

1. **Window Management:**
   - Main window opens correctly
   - Window resizing works
   - Window minimization/maximization works
   - Window close exits cleanly

2. **Menu System:**
   - File menu (New, Open, Save, Exit)
   - Edit menu (Undo, Redo, Cut, Copy, Paste)
   - View menu (Queue, Workflows, Settings)
   - Help menu (Documentation, About)

3. **Status Bar:**
   - Queue status indicator
   - Scheduler status indicator
   - Connection status indicator
   - Error count display

### Verification Commands

```bash
# Test desktop shell startup
cargo test --lib ui::tests::shell_startup
cargo test --lib ui::tests::window_management
cargo test --lib ui::tests::menu_system
cargo test --lib ui::tests::status_bar

# Test clean exit
cargo test --lib ui::tests::clean_exit
```

### Pass Criteria

- [ ] Desktop shell starts without crashing
- [ ] Window management works correctly
- [ ] Menu system works correctly
- [ ] Status bar displays correctly
- [ ] Clean exit without data loss

### Evidence Required

- Shell startup test results
- Window management test logs
- Menu system test logs
- Clean exit test logs
- Screenshot of UI (manual verification)

---

## Queue Visualization

**Requirement:** Queue visualization matches scheduler state

### Visualization Components

1. **Queue List:**
   - Displays all queued workflows
   - Shows workflow name, status, priority
   - Updates in real-time as workflows enqueue/dequeue
   - Sortable by name, status, priority

2. **Workflow Details:**
   - Workflow ID
   - Pipeline count
   - Step count
   - Execution mode (serial/parallel/hybrid)
   - Current state (Queued, Running, Completed, Failed)

3. **Progress Indicators:**
   - Overall workflow progress (percentage)
   - Pipeline-level progress
   - Step-level progress
   - Time elapsed/estimated

### Verification Commands

```bash
# Test queue visualization
cargo test --lib ui::tests::queue_list_display
cargo test --lib ui::tests::workflow_details_display
cargo test --lib ui::tests::progress_indicators

# Test real-time updates
cargo test --lib ui::tests::real_time_queue_updates

# Test synchronization with scheduler
cargo test --lib ui::tests::scheduler_state_sync
```

### Pass Criteria

- [ ] Queue list displays all workflows correctly
- [ ] Workflow details display accurately
- [ ] Progress indicators update correctly
- [ ] Real-time updates work without lag
- [ ] Visualization matches scheduler state exactly

### Evidence Required

- Queue visualization test results
- Real-time update test logs
- Scheduler synchronization test logs
- Screenshot of queue display (manual verification)

---

## Scope Indicators

**Requirement:** Scope indicators work correctly

### Scope Indicator Types

1. **Workflow Scope:**
   - Workflow name
   - Workflow ID
   - Workflow version

2. **Pipeline Scope:**
   - Pipeline name
   - Pipeline ID
   - Execution mode

3. **Step Scope:**
   - Step name
   - Step ID
   - Step type

4. **Log Scope:**
   - Current log scope (pipeline, step, model, tool, etc.)
   - Log level filter (debug, info, warn, error)

5. **Variable Scope:**
   - Available variables in current scope
   - Variable values

### Verification Commands

```bash
# Test scope indicators
cargo test --lib ui::tests::workflow_scope_indicator
cargo test --lib ui::tests::pipeline_scope_indicator
cargo test --lib ui::tests::step_scope_indicator
cargo test --lib ui::tests::log_scope_indicator
cargo test --lib ui::tests::variable_scope_indicator

# Test scope navigation
cargo test --lib ui::tests::scope_navigation
```

### Pass Criteria

- [ ] All scope indicators display correctly
- [ ] Scope indicators update when navigating
- [ ] Scope indicators show accurate information
- [ ] Scope navigation works without data loss

### Evidence Required

- Scope indicator test results
- Scope navigation test logs
- Screenshot of scope indicators (manual verification)

---

## Navigation

**Requirement:** Navigation is safe (no data loss)

### Navigation Features

1. **Workflow Navigation:**
   - Navigate between workflows
   - View workflow details
   - No data loss when switching

2. **Pipeline Navigation:**
   - Navigate between pipelines
   - View pipeline details
   - No data loss when switching

3. **Step Navigation:**
   - Navigate between steps
   - View step details
   - No data loss when switching

4. **Back/Forward:**
   - Back button returns to previous view
   - Forward button returns to next view
   - History preserved correctly

5. **Breadcrumb Navigation:**
   - Breadcrumbs show current location
   - Clicking breadcrumb jumps to location
   - Breadcrumbs update correctly

### Verification Commands

```bash
# Test navigation
cargo test --lib ui::tests::workflow_navigation
cargo test --lib ui::tests::pipeline_navigation
cargo test --lib ui::tests::step_navigation
cargo test --lib ui::tests::back_forward_navigation
cargo test --lib ui::tests::breadcrumb_navigation

# Test data loss prevention
cargo test --lib ui::tests::no_data_loss_on_navigation
```

### Pass Criteria

- [ ] All navigation types work correctly
- [ ] No data loss when navigating
- [ ] Back/Forward preserves history
- [ ] Breadcrumbs show correct path
- [ ] Navigation is fast and responsive

### Evidence Required

- Navigation test results
- Data loss prevention test logs
- Screenshot of navigation (manual verification)

---

## Drag-Drop Operations

**Requirement:** Drag-drop calls scheduler APIs correctly

### Drag-Drop Scenarios

1. **Workflow Reordering:**
   - Drag workflow to reorder in queue
   - Calls scheduler `requeue` API
   - Priority updated correctly

2. **Step Reordering:**
   - Drag step to reorder in pipeline
   - Calls scheduler `reorder_steps` API
   - Step order updated correctly

3. **Workflow to Queue:**
   - Drag workflow file to queue
   - Calls scheduler `enqueue` API
   - Workflow added to queue

4. **Step to Pipeline:**
   - Drag step template to pipeline
   - Calls scheduler `add_step` API
   - Step added to pipeline

### Verification Commands

```bash
# Test drag-drop operations
cargo test --lib ui::tests::workflow_reordering
cargo test --lib ui::tests::step_reordering
cargo test --lib ui::tests::workflow_to_queue
cargo test --lib ui::tests::step_to_pipeline

# Test API calls
cargo test --lib ui::tests::drag_drop_scheduler_api_calls
```

### Pass Criteria

- [ ] All drag-drop operations work correctly
- [ ] Correct scheduler APIs called
- [ ] Priority/order updated correctly
- [ ] Visual feedback during drag-drop
- [ ] No data loss during drag-drop

### Evidence Required

- Drag-drop test results
- Scheduler API call logs
- Screenshot of drag-drop operation (manual verification)

---

## Phase Exit Criteria

### Layer 1: Unit Tests

**Commands:**
```bash
cargo test --lib -- --test-threads=1
```

**Evidence:**
- All Phase 03 unit tests pass
- Test coverage >= 90%
- No clippy warnings

### Layer 2: Integration Tests

**Commands:**
```bash
cargo test --test '*' -- --test-threads=1
```

**Evidence:**
- All Phase 03 integration tests pass
- UI components integrate correctly with scheduler

### Layer 3: Property Tests

**Commands:**
```bash
PROPTEST_NUMBER_OF_TESTS=100 cargo test --lib property_based
```

**Evidence:**
- Navigation state invariants hold for 100 iterations
- Drag-drop ordering invariants hold

### Layer 4: E2E Tests

**Commands:**
```bash
# Test UI workflow (manual)
# 1. Start desktop shell
# 2. Navigate queue
# 3. View workflow details
# 4. Drag-drop to reorder
# 5. Navigate back
# 6. Exit cleanly
```

**Evidence:**
- Manual test checklist completed
- Screenshot evidence collected
- No crashes or data loss

### Layer 5: System Log Validation

**Commands:**
```bash
# Verify UI scope logs
cargo run --bin whitt -- 2>&1 | jq -e 'select(.scope == "ui")'
```

**Evidence:**
- UI scope logs emitted
- Logs include timestamp, scope, event, message

### Layer 6: Live CLI Verification

**Commands:**
```bash
# Not applicable for UI phase
# Manual verification required
```

**Evidence:**
- Manual test checklist completed
- UI verified to work correctly

### Layer 7: Benchmark Performance

**Commands:**
```bash
cargo bench --bench phase_03_benchmarks
```

**Evidence:**
- UI rendering performance acceptable
- Drag-drop operations responsive
- No performance regression > 10%

---

## Cross-Phase Regression Tests

**Commands:**
```bash
# Run all prior phase tests
cargo test --test phase_00_integration -- --test-threads=1
cargo test --test phase_01_integration -- --test-threads=1
cargo test --test phase_02_integration -- --test-threads=1

# Verify schema coverage
cargo run --bin schema_audit -- --phase 0 --output phase_00_regression.md
cargo run --bin schema_audit -- --phase 1 --output phase_01_regression.md
cargo run --bin schema_audit -- --phase 2 --output phase_02_regression.md
```

**Pass Criteria:**
- [ ] All Phase 00 tests still pass
- [ ] All Phase 01 tests still pass
- [ ] All Phase 02 tests still pass
- [ ] Schema coverage 100% for all phases

---

## Schema Coverage Audit

**Requirement:** 100% of Phase 03-owned fields implemented

### Phase 03-Owned Fields

**UISchema:**
- window_title
- window_width
- window_height
- theme
- layout

**QueueVisualizationSchema:**
- show_queue
- show_workflow_details
- show_progress
- auto_refresh

**NavigationSchema:**
- enable_back_forward
- enable_breadcrumbs
- preserve_history

### Verification Commands

```bash
cargo run --bin schema_audit -- --phase 3 --output coverage_report.md
```

**Expected Output:**
- 100% coverage for Phase 03-owned fields
- No unimplemented fields

---

## ADR Compliance

**Relevant ADRs:**
- ADR-007: UI Architecture

### Verification Commands

```bash
cargo run --bin adr_compliance -- --phase 3
```

**Expected Output:**
- All Phase 03-relevant ADR constraints satisfied
- No outstanding ADR TODOs

---

## Anti-Goal-Drift Checklist

**Run after phase completion:**

1. **Requirements Drift:**
   - [ ] Implementation matches Phase 03 requirements
   - [ ] No features from later phases added

2. **Architecture Drift:**
   - [ ] UI matches ADR-007 architecture
   - [ ] UI integrates correctly with scheduler

3. **Scope Creep:**
   - [ ] Only UI features implemented
   - [ ] No automation or autonomy features added

---

## Evidence Storage

**Location:** `results/phase_03/`

**Contents:**
- `unit_test_results.json`
- `integration_test_output.log`
- `property_test_results.json`
- `e2e_manual_test_checklist.md` (manual UI tests)
- `screenshots/` (UI screenshots)
- `system_log_samples.json` (ui scope)
- `benchmarks/` (UI performance)
- `schema_coverage_report.md`
- `adr_compliance_report.md`
- `anti_goal_drift_checklist.md`
- `phase_00_regression/`
- `phase_01_regression/`
- `phase_02_regression/`

---

## Blocking Issues

**Cannot exit Phase 03 if:**
- Any verification layer fails
- Desktop shell crashes on startup
- Queue visualization doesn't match scheduler state
- Scope indicators don't work
- Navigation causes data loss
- Drag-drop doesn't call scheduler APIs
- Any prior phase regression detected

---

**End of Phase 03 Criteria**
