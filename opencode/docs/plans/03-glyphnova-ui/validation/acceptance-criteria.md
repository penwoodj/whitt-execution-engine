# Acceptance Criteria for Phase 03: Glyphnova UI

**Phase Focus:** Desktop shell, queue visualization, scope indicators, navigation, drag-drop
**Owner Phase:** Phase 03
**Target Date:** 2026-04-13
**Completion Requirement:** All 7 verification layers pass + ADR-0004 compliance

---

## Document Purpose

This document defines the acceptance criteria for Phase 03 (Glyphnova UI). Implementation is **COMPLETE** when ALL items in this document are checked.

**Usage:**
- Each checkbox represents a required verification
- All checkboxes must be checked before phase exit
- Evidence must be collected and stored for each criterion

---

## ADR-0004 Compliance: UI is a Shell, Not a Separate System

### ADR Requirement: UI is Shell Over Runtime

- [ ] **Check:** No business logic in UI codebase
- **Verification:**
  - Search `src-frontend/` for algorithm implementations
  - Search `src-frontend/` for validation logic
  - Search `src-frontend/` for transformation logic
  - Expected: No business logic found
- **Test Commands:**
  ```bash
  # Search for business logic in UI
  grep -r "function.*algorithm" src-frontend/ || echo "No algorithms found"
  grep -r "class.*Validator" src-frontend/ || echo "No validators found"
  grep -r "function.*transform" src-frontend/ || echo "No transformers found"
  ```
- **Expected Output:** All searches return "No ... found"
- **Evidence:** Grep output logs

- [ ] **Check:** All state comes from API calls
- **Verification:**
  - `useQueue` hook fetches from `/api/queue`
  - `useWorkflow` hook fetches from `/api/workflows/{id}`
  - `useArtifacts` hook fetches from `/api/artifacts`
  - No hardcoded state values
  - No local state persistence beyond React Query cache
- **Test Commands:**
  ```bash
  # Verify API hooks
  grep -r "useQueue" src-frontend/hooks/useApi.ts | grep "/api/queue"
  grep -r "useWorkflow" src-frontend/hooks/useApi.ts | grep "/api/workflows"
  grep -r "useArtifacts" src-frontend/hooks/useApi.ts | grep "/api/artifacts"
  ```
- **Expected Output:** All hooks fetch from API endpoints
- **Evidence:** Grep output logs

- [ ] **Check:** No local database or storage in UI
- **Verification:**
  - No IndexedDB usage
  - No localStorage for state persistence
  - No database libraries in `package.json`
  - No file I/O for state storage
- **Test Commands:**
  ```bash
  # Check for databases
  grep -i "indexeddb" src-frontend/ || echo "No IndexedDB found"
  grep -i "localStorage.setItem.*queue" src-frontend/ || echo "No queue storage found"
  grep -E "dexie|nedb|sql" package.json || echo "No database deps found"
  ```
- **Expected Output:** All searches return "No ... found"
- **Evidence:** Grep output logs

- [ ] **Check:** UI breaks gracefully when backend is down
- **Verification:**
  - Start UI without runtime
  - Verify error messages display
  - Verify no local fallback data
- **Test Steps:**
  1. Stop runtime server: `pkill -f agentsdk-runtime`
  2. Start UI: `cargo run --bin glyphnova-ui`
  3. Verify connection error displays
  4. Verify queue doesn't show fake data
- **Expected Output:** Connection error message, no fake data
- **Evidence:** Screenshot of error message

- [ ] **Check:** No caching beyond React Query
- **Verification:**
  - Only React Query provides caching
  - No manual cache implementation
  - No custom cache stores
- **Test Commands:**
  ```bash
  # Check for manual caching
  grep -r "class.*Cache" src-frontend/ || echo "No cache classes found"
  grep -r "const cache = " src-frontend/ || echo "No cache objects found"
  grep -r "useState.*cache" src-frontend/ || echo "No cache state found"
  ```
- **Expected Output:** All searches return "No ... found"
- **Evidence:** Grep output logs

---

## ADR-0004 Compliance: Queue is Direct Projection of Scheduler State

### ADR Requirement: Queue Matches Scheduler Exactly

- [ ] **Check:** Queue order matches scheduler exactly
- **Verification:**
  - Get queue via CLI: `agentsdk queue list`
  - Compare with UI queue order
  - Order must be identical
- **Test Steps:**
  1. Add items to queue via CLI:
     ```bash
     agentsdk queue add workflow1.yaml
     agentsdk queue add workflow2.yaml
     agentsdk queue add workflow3.yaml
     ```
  2. Check order in CLI: `agentsdk queue list`
  3. Check order in UI (screenshot)
  4. Verify order matches
- **Expected Output:** Identical order in both CLI and UI
- **Evidence:** CLI output + UI screenshot + comparison document

- [ ] **Check:** Status changes reflect immediately
- **Verification:**
  - Start a workflow via CLI
  - Watch status in UI
  - Status must update within 2 seconds
- **Test Steps:**
  1. Add workflow to queue: `agentsdk queue add test_workflow.yaml`
  2. Start workflow via CLI: `agentsdk queue start <id>`
  3. Observe UI status change: `pending` → `running` → `completed`
  4. Measure time for each change
- **Expected Output:** Each status change within 2 seconds
- **Evidence:** UI screenshot at each state + timing logs

- [ ] **Check:** No local queue array maintained
- **Verification:**
  - Search for `let queue = []` in UI code
  - Search for `const queueItems` local state
  - Queue state should only come from API
- **Test Commands:**
  ```bash
  # Check for local queue
  grep -r "let queue = \[\]" src-frontend/ || echo "No local queue arrays found"
  grep -r "const queueItems.*useState" src-frontend/ || echo "No queue state found"
  ```
- **Expected Output:** All searches return "No ... found"
- **Evidence:** Grep output logs

- [ ] **Check:** Updates come from WebSocket or polling
- **Verification:**
  - WebSocket connection established
  - `queue:update` events received
  - Or polling configured (if no WebSocket)
- **Test Steps:**
  1. Open browser DevTools → Network → WS
  2. Verify WebSocket connection to `ws://localhost:8080/ws`
  3. Observe `queue:update` messages
  4. Start workflow and watch for updates
- **Expected Output:** WebSocket connected, `queue:update` events received
- **Evidence:** DevTools screenshots + WebSocket message logs

- [ ] **Check:** No optimistic queue updates
- **Verification:**
  - Drag-and-drop waits for API confirmation
  - No local queue update before API response
  - Queue reverts if API call fails
- **Test Steps:**
  1. Drag queue item to new position
  2. Verify order doesn't change until API responds
  3. Stop runtime, try to reorder
  4. Verify order reverts to original
- **Expected Output:** Order doesn't change until API responds, reverts on error
- **Evidence:** UI screenshots + API logs

---

## ADR-0004 Compliance: Scope is Always Visible in Header

### ADR Requirement: Scope Always Visible, Changes Require Confirmation

- [ ] **Check:** Header always displays current scope
- **Verification:**
  - Header is always visible (fixed position)
  - Scope breadcrumbs always displayed
  - No hiding of scope information
- **Test Steps:**
  1. Open UI
  2. Navigate to different scopes
  3. Verify header always shows scope
  4. Scroll down, verify header remains visible
- **Expected Output:** Header fixed at top, scope always visible
- **Evidence:** UI screenshots at different scopes

- [ ] **Check:** Breadcrumbs display full scope path
- **Verification:**
  - Path: `workspace → project → workflow → step`
  - All levels visible
  - Breadcrumbs clickable for navigation
- **Test Steps:**
  1. Navigate to step scope
  2. Verify breadcrumbs show: `workspace / project / workflow / step`
  3. Click each breadcrumb
  4. Verify navigation works
- **Expected Output:** Full path displayed, navigation works
- **Evidence:** UI screenshots + navigation logs

- [ ] **Check:** Scope changes require confirmation
- **Verification:**
  - Clicking breadcrumb shows confirmation dialog
  - Dialog clearly states change
  - User must confirm to proceed
- **Test Steps:**
  1. Select a queue item
  2. Click different breadcrumb
  3. Verify confirmation dialog appears
  4. Cancel and verify no change
  5. Confirm and verify change
- **Expected Output:** Dialog appears, cancellation prevents change
- **Evidence:** UI screenshots + interaction logs

- [ ] **Check:** User always knows current context
- **Verification:**
  - Context indicators show scope level
  - Color coding distinguishes scope levels
  - Quick actions are context-specific
- **Test Steps:**
  1. Navigate to different scopes
  2. Verify context indicator changes color
  3. Verify quick actions change per scope
- **Expected Output:** Context indicators visible, actions change
- **Evidence:** UI screenshots at different scopes

---

## ADR-0004 Compliance: Drag-and-Drop is Scheduler API Projection

### ADR Requirement: Drag-Drop Calls Scheduler API

- [ ] **Check:** Drag-and-drop calls scheduler API
- **Verification:**
  - API call to `/api/queue/reorder`
  - Request includes `item_id` and `new_position`
  - No direct queue modification
- **Test Steps:**
  1. Open DevTools → Network
  2. Drag queue item
  3. Verify POST to `/api/queue/reorder`
  4. Verify request body includes item_id and new_position
- **Expected Output:** POST to `/api/queue/reorder` with correct payload
- **Evidence:** DevTools screenshots + Network logs

- [ ] **Check:** No direct data modification
- **Verification:**
  - No direct array manipulation
  - No local queue state update before API
  - All changes go through API
- **Test Commands:**
  ```bash
  # Check for direct array manipulation
  grep -r "queue.splice" src-frontend/ || echo "No direct splicing found"
  grep -r "queue.push" src-frontend/ || echo "No direct pushing found"
  ```
- **Expected Output:** All searches return "No ... found"
- **Evidence:** Grep output logs

- [ ] **Check:** Reorder validates with scheduler
- **Verification:**
  - Invalid reorder rejected by scheduler
  - UI shows error on rejection
  - Queue doesn't update on rejection
- **Test Steps:**
  1. Try to reorder running item to top (if configured)
  2. Verify scheduler rejects (if configured)
  3. Verify error message displays
  4. Verify queue doesn't change
- **Expected Output:** Rejection with error message, no queue change
- **Evidence:** UI screenshots + API logs

- [ ] **Check:** Errors revert UI to original state
- **Verification:**
  - If API call fails, queue reverts
  - No partial updates
  - User sees error message
- **Test Steps:**
  1. Stop runtime server
  2. Drag queue item
  3. Verify error message displays
  4. Verify queue reverts to original order
- **Expected Output:** Error message, queue reverts
- **Evidence:** UI screenshots + error logs

- [ ] **Check:** No optimistic UI updates
- **Verification:**
  - Queue doesn't update until API responds
  - Visual feedback shows loading state
  - Success/failure only after API response
- **Test Steps:**
  1. Drag queue item
  2. Observe queue before API responds
  3. Verify no update yet
  4. Wait for API response
  5. Verify update only after response
- **Expected Output:** No update until API responds
- **Evidence:** UI screenshots + timing logs

---

## ADR-0004 Compliance: No Separate Backend

### ADR Requirement: Single Shared Backend

- [ ] **Check:** No separate UI backend service
- **Verification:**
  - Only one backend service: AgentSDK runtime
  - No `ui-backend` process
  - No separate API server for UI
- **Test Commands:**
  ```bash
  # Check for multiple backends
  ps aux | grep -E "(agentsdk-runtime|ui-backend)"
  ```
- **Expected Output:** Only `agentsdk-runtime` process running
- **Evidence:** Process list output

- [ ] **Check:** Uses same REST and WebSocket APIs as CLI
- **Verification:**
  - UI calls `/api/queue`
  - CLI calls `/api/queue`
  - Same endpoints, same response format
- **Test Steps:**
  1. Get queue via CLI: `agentsdk queue list`
  2. Get queue via UI (Network tab)
  3. Compare responses
  4. Verify identical format
- **Expected Output:** Identical response format
- **Evidence:** CLI output + Network logs + comparison document

- [ ] **Check:** Tauri backend is thin wrapper
- **Verification:**
  - Tauri commands just forward to runtime
  - No business logic in Tauri
  - No state management in Tauri
- **Test Commands:**
  ```bash
  # Check Tauri commands
  grep -r "invoke(" src-tauri/src/ | head -20
  ```
- **Expected Output:** Only forwarding commands, no logic
- **Evidence:** Grep output logs

- [ ] **Check:** No extra database or storage
- **Verification:**
  - No SQLite, PostgreSQL, etc. for UI
  - No file-based storage for UI state
  - Only runtime stores data
- **Test Commands:**
  ```bash
  # Check for database dependencies
  grep -E "sqlite|postgres|mysql" package.json || echo "No database deps found"
  grep -r "fs.writeFileSync.*state" src-frontend/ || echo "No state file I/O found"
  ```
- **Expected Output:** All searches return "No ... found"
- **Evidence:** Grep output logs

---

## Desktop Shell Startup and Functionality

### Desktop Shell Must Start Without Crashing

- [ ] **Check:** Desktop shell starts on Linux
- **Verification:**
  - Launch UI without crashes
  - Main window opens correctly
  - All menus accessible
- **Test Commands:**
  ```bash
  # Start UI
  cargo run --bin glyphnova-ui
  # Expected: Window opens, no errors
  ```
- **Expected Output:** Window opens, no crash logs
- **Evidence:** Screenshot of UI

- [ ] **Check:** Desktop shell starts on macOS
- **Verification:**
  - Launch UI without crashes
  - Main window opens correctly
  - All menus accessible
- **Test Commands:**
  ```bash
  # Start UI (on macOS)
  cargo run --bin glyphnova-ui
  # Expected: Window opens, no errors
  ```
- **Expected Output:** Window opens, no crash logs
- **Evidence:** Screenshot of UI

- [ ] **Check:** Desktop shell starts on Windows (optional)
- **Verification:**
  - Launch UI without crashes
  - Main window opens correctly
  - All menus accessible
- **Test Commands:**
  ```bash
  # Start UI (on Windows)
  cargo run --bin glyphnova-ui
  # Expected: Window opens, no errors
  ```
- **Expected Output:** Window opens, no crash logs
- **Evidence:** Screenshot of UI

- [ ] **Check:** Clean exit without data loss
- **Verification:**
  - Close window gracefully
  - No data loss on exit
  - No crash on exit
- **Test Steps:**
  1. Load queue with workflows
  2. Navigate to various scopes
  3. Close window
  4. Reopen UI
  5. Verify state preserved (or intentionally lost)
- **Expected Output:** Clean exit, no errors
- **Evidence:** Exit logs + re-open screenshot

---

## WebSocket Real-Time Updates

### WebSocket Connection and Updates Working

- [ ] **Check:** WebSocket connection established
- **Verification:**
  - WebSocket connects to `ws://localhost:8080/ws`
  - Connection stays alive
  - Auto-reconnect on failure
- **Test Steps:**
  1. Open UI
  2. Open DevTools → Network → WS
  3. Verify WebSocket connection
  4. Stop runtime, wait for reconnect
- **Expected Output:** WebSocket connected, reconnects on failure
- **Evidence:** DevTools screenshots + connection logs

- [ ] **Check:** Queue updates via WebSocket
- **Verification:**
  - `queue:update` events received
  - Queue UI updates on events
  - Updates within 2 seconds
- **Test Steps:**
  1. Open UI with DevTools
  2. Add workflow to queue via CLI
  3. Observe WebSocket `queue:update` event
  4. Observe UI update
- **Expected Output:** Event received, UI updates
- **Evidence:** DevTools screenshots + event logs

- [ ] **Check:** Workflow status updates via WebSocket
- **Verification:**
  - `workflow:status` events received
  - Status indicators update on events
  - Updates within 2 seconds
- **Test Steps:**
  1. Open UI with DevTools
  2. Start workflow via CLI
  3. Observe WebSocket `workflow:status` events
  4. Observe status indicator updates
- **Expected Output:** Events received, status updates
- **Evidence:** DevTools screenshots + event logs

- [ ] **Check:** Artifact updates via WebSocket
- **Verification:**
  - `artifact:created` events received
  - Artifact browser updates on events
  - Updates within 2 seconds
- **Test Steps:**
  1. Open UI with DevTools
  2. Run workflow that creates artifacts
  3. Observe WebSocket `artifact:created` events
  4. Observe artifact browser updates
- **Expected Output:** Events received, browser updates
- **Evidence:** DevTools screenshots + event logs

---

## Multi-Zoom Navigation

### Navigation Cannot Lead to Invalid States

- [ ] **Check:** Zoom level buttons work
- **Verification:**
  - Workflow zoom level button active
  - Step zoom level button active
  - Agent zoom level button active
  - Tool zoom level button active
- **Test Steps:**
  1. Click workflow zoom button
  2. Verify workflow view displays
  3. Click step zoom button
  4. Verify step view displays
- **Expected Output:** All zoom buttons work
- **Evidence:** UI screenshots at each zoom level

- [ ] **Check:** Back/Forward/Up buttons work
- **Verification:**
  - Back button returns to previous view
  - Forward button returns to next view
  - Up button navigates to parent scope
- **Test Steps:**
  1. Navigate: workflow → step → agent
  2. Click Up, verify returns to step
  3. Click Back, verify returns to agent
  4. Click Forward, verify returns to step
- **Expected Output:** All navigation buttons work correctly
- **Evidence:** UI screenshots + navigation logs

- [ ] **Check:** Navigation state machine prevents invalid states
- **Verification:**
  - Cannot navigate to non-existent step
  - Cannot navigate to non-existent agent
  - Invalid navigation blocked
- **Test Steps:**
  1. Try to navigate to invalid step ID
  2. Verify error or no navigation
  3. Try to navigate to invalid agent ID
  4. Verify error or no navigation
- **Expected Output:** Invalid navigation blocked
- **Evidence:** UI screenshots + error logs

- [ ] **Check:** Zoom views display correct content
- **Verification:**
  - Workflow view shows workflow DAG
  - Step view shows step execution details
  - Agent view shows agent execution details
  - Tool view shows tool execution details
- **Test Steps:**
  1. Navigate to workflow zoom level
  2. Verify DAG displayed
  3. Navigate to step zoom level
  4. Verify step details displayed
- **Expected Output:** Correct content at each level
- **Evidence:** UI screenshots at each level

- [ ] **Check:** Navigation preserves context
- **Verification:**
  - Context indicators update correctly
  - Breadcrumbs show current path
  - Quick actions update for current scope
- **Test Steps:**
  1. Navigate to different zoom levels
  2. Verify breadcrumbs update
  3. Verify context indicators update
  4. Verify quick actions update
- **Expected Output:** All indicators update correctly
- **Evidence:** UI screenshots at each level

---

## Artifact Browser

### Artifact Browser with Search and Versioning

- [ ] **Check:** Artifact list displays
- **Verification:**
  - All artifacts listed
  - File type icons displayed
  - Metadata shown (size, type, date)
- **Test Steps:**
  1. Run workflow that creates artifacts
  2. Open artifact browser
  3. Verify all artifacts listed
- **Expected Output:** All artifacts with icons and metadata
- **Evidence:** UI screenshot of artifact browser

- [ ] **Check:** Artifact search works
- **Verification:**
  - Search by artifact name
  - Search results update instantly
  - No results message when appropriate
- **Test Steps:**
  1. Enter search term
  2. Verify results filtered
  3. Enter non-matching term
  4. Verify "No results" message
- **Expected Output:** Search filters correctly
- **Evidence:** UI screenshots of search results

- [ ] **Check:** Artifact type filtering works
- **Verification:**
  - Filter by file type (YAML, Rust, Markdown, JSON, Text)
  - Only matching artifacts shown
  - Filter selection preserved
- **Test Steps:**
  1. Select "Rust" filter
  2. Verify only .rs files shown
  3. Select "Markdown" filter
  4. Verify only .md files shown
- **Expected Output:** Filtering works correctly
- **Evidence:** UI screenshots of filtered results

- [ ] **Check:** Artifact preview works
- **Verification:**
  - Text artifacts show content
  - Images show thumbnail
  - PDFs show download button
- **Test Steps:**
  1. Click text artifact
  2. Verify content preview displays
  3. Click image artifact
  4. Verify thumbnail displays
- **Expected Output:** Preview displays correctly
- **Evidence:** UI screenshots of previews

- [ ] **Check:** Artifact versioning works
- **Verification:**
  - Version history accessible
  - Version comparison available
  - Version rollback possible
- **Test Steps:**
  1. Create multiple versions of artifact
  2. Open artifact details
  3. Click "Version History"
  4. Verify all versions listed
- **Expected Output:** Version history accessible
- **Evidence:** UI screenshot of version history

---

## Verification Layer 1: Unit Tests

### Unit Tests Pass

- [ ] **Check:** All Phase 03 unit tests pass
- **Test Commands:**
  ```bash
  cargo test --lib ui::tests -- --test-threads=1
  ```
- **Expected Output:** All tests pass
- **Evidence:** Test output log

- [ ] **Check:** Test coverage >= 90%
- **Test Commands:**
  ```bash
  cargo tarpaulin --out Html --output-dir coverage/
  ```
- **Expected Output:** Coverage >= 90%
- **Evidence:** Coverage report

- [ ] **Check:** No clippy warnings
- **Test Commands:**
  ```bash
  cargo clippy -- -D warnings
  ```
- **Expected Output:** No warnings
- **Evidence:** Clippy output

---

## Verification Layer 2: Integration Tests

### Integration Tests Pass

- [ ] **Check:** All Phase 03 integration tests pass
- **Test Commands:**
  ```bash
  cargo test --test phase_03_integration -- --test-threads=1
  ```
- **Expected Output:** All tests pass
- **Evidence:** Test output log

- [ ] **Check:** UI components integrate with scheduler
- **Test Commands:**
  ```bash
  cargo test --lib ui::tests::scheduler_integration
  ```
- **Expected Output:** Integration works
- **Evidence:** Test output log

---

## Verification Layer 3: Property Tests

### Property Tests Pass

- [ ] **Check:** Navigation state invariants hold
- **Test Commands:**
  ```bash
  PROPTEST_NUMBER_OF_TESTS=100 cargo test --lib ui::tests::navigation_invariants
  ```
- **Expected Output:** All invariants hold
- **Evidence:** Test output log

- [ ] **Check:** Drag-drop ordering invariants hold
- **Test Commands:**
  ```bash
  PROPTEST_NUMBER_OF_TESTS=100 cargo test --lib ui::tests::ordering_invariants
  ```
- **Expected Output:** All invariants hold
- **Evidence:** Test output log

---

## Verification Layer 4: E2E Tests

### End-to-End Tests Pass

- [ ] **Check:** Desktop shell startup E2E
- **Test Steps:**
  1. Launch UI
  2. Verify window opens
  3. Verify no crashes
  4. Verify menus work
- **Expected Output:** Shell works end-to-end
- **Evidence:** E2E test log + screenshots

- [ ] **Check:** Queue operations E2E
- **Test Steps:**
  1. Add workflow via CLI
  2. Verify appears in UI
  3. Start workflow via UI
  4. Verify status updates
- **Expected Output:** Queue operations work
- **Evidence:** E2E test log + screenshots

- [ ] **Check:** Navigation E2E
- **Test Steps:**
  1. Navigate through all zoom levels
  2. Verify breadcrumbs update
  3. Verify context indicators update
  4. Verify back/forward works
- **Expected Output:** Navigation works
- **Evidence:** E2E test log + screenshots

- [ ] **Check:** Drag-drop E2E
- **Test Steps:**
  1. Drag queue item
  2. Verify API call made
  3. Verify queue updates
  4. Verify no optimistic updates
- **Expected Output:** Drag-drop works
- **Evidence:** E2E test log + screenshots

---

## Verification Layer 5: System Log Validation

### System Logs Emitted Correctly

- [ ] **Check:** UI scope logs emitted
- **Test Commands:**
  ```bash
  cargo run --bin glyphnova-ui -- 2>&1 | jq -e 'select(.scope == "ui")'
  ```
- **Expected Output:** UI logs present
- **Evidence:** Log samples

- [ ] **Check:** Logs include timestamp, scope, event, message
- **Test Commands:**
  ```bash
  cargo run --bin glyphnova-ui -- 2>&1 | jq -e 'select(.scope == "ui") | keys == ["timestamp", "scope", "event", "message"]'
  ```
- **Expected Output:** All fields present
- **Evidence:** Log samples

---

## Verification Layer 6: Live CLI Verification

### CLI Commands Work

- [ ] **Check:** UI launch command works
- **Test Commands:**
  ```bash
  cargo run --bin agentsdk -- ui
  ```
- **Expected Output:** UI launches
- **Evidence:** Command output

- [ ] **Check:** Queue operations via UI work
- **Test Steps:**
  1. Open UI
  2. Perform queue operations
  3. Verify operations complete
- **Expected Output:** Operations work
- **Evidence:** UI screenshots

---

## Verification Layer 7: Benchmark Performance

### Performance Acceptable

- [ ] **Check:** UI rendering performance acceptable
- **Test Commands:**
  ```bash
  cargo bench --bench ui_rendering
  ```
- **Expected Output:** Rendering < 100ms for 100+ items
- **Evidence:** Benchmark output

- [ ] **Check:** Drag-drop operations responsive
- **Test Commands:**
  ```bash
  cargo bench --bench drag_drop
  ```
- **Expected Output:** Drag-drop < 50ms
- **Evidence:** Benchmark output

- [ ] **Check:** No performance regression > 10%
- **Test Commands:**
  ```bash
  cargo bench --bench phase_03_benchmarks | grep "perf regression"
  ```
- **Expected Output:** No regression > 10%
- **Evidence:** Benchmark output

---

## Cross-Phase Regression Tests

### No Regression in Prior Phases

- [ ] **Check:** Phase 00 tests still pass
- **Test Commands:**
  ```bash
  cargo test --test phase_00_integration -- --test-threads=1
  ```
- **Expected Output:** All tests pass
- **Evidence:** Test output log

- [ ] **Check:** Phase 01 tests still pass
- **Test Commands:**
  ```bash
  cargo test --test phase_01_integration -- --test-threads=1
  ```
- **Expected Output:** All tests pass
- **Evidence:** Test output log

- [ ] **Check:** Phase 02 tests still pass
- **Test Commands:**
  ```bash
  cargo test --test phase_02_integration -- --test-threads=1
  ```
- **Expected Output:** All tests pass
- **Evidence:** Test output log

---

## Evidence Storage

### Location: `results/phase_03/acceptance_criteria/`

**Contents:**
- `adr_compliance/` (ADR-0004 compliance evidence)
  - `ui_shell_verification.log`
  - `queue_projection_verification.log`
  - `scope_visibility_verification.log`
  - `drag_drop_projection_verification.log`
  - `no_separate_backend_verification.log`
- `desktop_shell/` (Desktop shell evidence)
  - `startup_linux.log`
  - `startup_macos.log`
  - `startup_windows.log` (if applicable)
  - `screenshots/`
- `websocket/` (WebSocket evidence)
  - `connection_test.log`
  - `queue_updates.log`
  - `workflow_status_updates.log`
  - `artifact_updates.log`
- `navigation/` (Navigation evidence)
  - `zoom_level_test.log`
  - `navigation_buttons_test.log`
  - `state_machine_test.log`
  - `screenshots/`
- `artifact_browser/` (Artifact browser evidence)
  - `list_display.log`
  - `search_test.log`
  - `filter_test.log`
  - `preview_test.log`
  - `versioning_test.log`
  - `screenshots/`
- `verification_layers/` (7 verification layers)
  - `layer1_unit_tests.log`
  - `layer2_integration_tests.log`
  - `layer3_property_tests.log`
  - `layer4_e2e_tests.log`
  - `layer5_system_logs.log`
  - `layer6_cli_verification.log`
  - `layer7_benchmarks.log`
- `cross_phase_regression/` (Regression tests)
  - `phase_00_regression.log`
  - `phase_01_regression.log`
  - `phase_02_regression.log`

---

## Blocking Issues

**Cannot exit Phase 03 if:**
- Any ADR-0004 compliance item fails
- Desktop shell fails to start
- Queue visualization doesn't match scheduler state
- Scope indicators not always visible
- Navigation leads to invalid states
- Drag-drop doesn't call scheduler APIs
- Separate backend detected
- WebSocket connection fails
- Multi-zoom navigation broken
- Artifact browser missing features
- Any verification layer fails
- Any prior phase regression detected

---

## Approval

**Phase 03 Implementation Complete When:**
- All checkboxes in this document checked
- All evidence collected and stored
- All verification layers pass
- No blocking issues
- All prior phases still pass

**Sign-off:**
- [ ] Implementation reviewed by: _________________
- [ ] Date: _________________
- [ ] Approved for phase exit: ⬜ Yes ⬜ No

---

**Document Version:** 1.0
**Last Updated:** 2026-04-07
**Plan Location:** `/home/jon/code/yaml-to-rust-agentsdk/opencode/docs/plans/03-glyphnova-ui/validation/acceptance-criteria.md`
