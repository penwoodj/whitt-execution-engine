# ADR-0004 Compliance Checks

This document provides detailed compliance checks for ADR-0004 (Glyphnova UI Design Decisions). Each section maps to an ADR requirement with specific verification steps.

---

## ADR Requirement 1: UI is a Shell, Not a Separate System

### Description

The UI does NOT maintain its own state or business logic. All state comes from the runtime via API calls.

### Compliance Checks

#### No Business Logic in UI

- [ ] **Check:** Review `src-frontend/` directory for business logic
- **Verification:**
  - No algorithm implementations in UI code
  - No validation logic in UI code (should be in runtime)
  - No transformation logic in UI code (should be in runtime)
- **Files to Check:**
  - `src-frontend/components/`
  - `src-frontend/hooks/`
  - `src-frontend/utils/`

#### State from API

- [ ] **Check:** All state comes from API calls
- **Verification:**
  - `useQueue` hook fetches from `/api/queue`
  - `useWorkflow` hook fetches from `/api/workflows/{id}`
  - `useArtifacts` hook fetches from `/api/artifacts`
  - No hardcoded state values
  - No local state persistence beyond React Query cache
- **Files to Check:**
  - `src-frontend/hooks/useApi.ts`

#### No Local Database

- [ ] **Check:** No local database or storage in UI
- **Verification:**
  - No IndexedDB usage
  - No localStorage for state persistence
  - No database libraries in `package.json`
  - No file I/O for state storage
- **Files to Check:**
  - `package.json`
  - `src-frontend/` (grep for "IndexedDB", "localStorage", "sessionStorage")

#### Breaks on Backend Down

- [ ] **Check:** UI breaks gracefully when backend is down
- **Verification:**
  - Start UI without runtime
  - Verify error messages display
  - Verify no local fallback data
- **Test Steps:**
  1. Stop runtime server
  2. Start UI
  3. Verify connection error displays
  4. Verify queue doesn't show fake data

#### No Caching Beyond React Query

- [ ] **Check:** No manual caching in UI
- **Verification:**
  - Only React Query provides caching
  - No manual cache implementation
  - No custom cache stores
- **Files to Check:**
  - `src-frontend/hooks/useApi.ts` (verify only React Query used)

### Evidence

- [ ] Code review shows no business logic in UI
- [ ] API hooks fetch all state from runtime
- [ ] No local database or storage used
- [ ] Error handling displays when backend down
- [ ] Only React Query provides caching

---

## ADR Requirement 2: Queue is Direct Projection of Scheduler State

### Description

Queue order, status, and metadata come directly from scheduler. Queue is a 1:1 projection.

### Compliance Checks

#### Order Matches Scheduler

- [ ] **Check:** Queue order matches scheduler exactly
- **Verification:**
  - Get queue via CLI: `agentsdk queue list`
  - Compare with UI queue order
  - Order must be identical
- **Test Steps:**
  1. Add items to queue via CLI
  2. Check order in CLI
  3. Check order in UI
  4. Verify order matches

#### Status Updates Reflect

- [ ] **Check:** Status changes reflect immediately
- **Verification:**
  - Start a workflow via CLI
  - Watch status in UI
  - Status must update within 2 seconds
- **Test Steps:**
  1. Add workflow to queue
  2. Start workflow via CLI
  3. Observe UI status change: `pending` → `running` → `completed`
  4. Verify each change within 2 seconds

#### No Local Queue

- [ ] **Check:** No local queue array maintained
- **Verification:**
  - Search for `let queue = []` in UI code
  - Search for `const queueItems` local state
  - Queue state should only come from API
- **Files to Check:**
  - `src-frontend/components/queue/`
  - `src-frontend/hooks/useApi.ts`

#### WebSocket/Polling for Updates

- [ ] **Check:** Updates come from WebSocket or polling
- **Verification:**
  - WebSocket connection established
  - `queue:update` events received
  - Or polling configured (if no WebSocket)
- **Test Steps:**
  1. Open browser DevTools → Network → WS
  2. Verify WebSocket connection to `ws://localhost:8080/ws`
  3. Observe `queue:update` messages

#### No Optimistic Queue

- [ ] **Check:** No optimistic UI for queue state
- **Verification:**
  - Drag-and-drop waits for API confirmation
  - No local queue update before API response
  - Queue reverts if API call fails
- **Test Steps:**
  1. Drag queue item to new position
  2. Verify order doesn't change until API responds
  3. Stop runtime, try to reorder
  4. Verify order reverts to original

### Evidence

- [ ] CLI queue order matches UI queue order
- [ ] Status updates reflect within 2 seconds
- [ ] No local queue array in code
- [ ] WebSocket receives `queue:update` events
- [ ] Drag-and-drop waits for API confirmation

---

## ADR Requirement 3: Scope is Always Visible in Header

### Description

User must always know which context they're operating in. Scope never hidden.

### Compliance Checks

#### Header Always Shows Scope

- [ ] **Check:** Header displays current scope
- **Verification:**
  - Header is always visible (fixed position)
  - Scope breadcrumbs always displayed
  - No hiding of scope information
- **Test Steps:**
  1. Open UI
  2. Navigate to different scopes
  3. Verify header always shows scope

#### Breadcrumbs Display Full Path

- [ ] **Check:** Breadcrumbs show full scope path
- **Verification:**
  - Path: `workspace → project → workflow → step`
  - All levels visible
  - Breadcrumbs clickable for navigation
- **Test Steps:**
  1. Navigate to step scope
  2. Verify breadcrumbs show: workspace / project / workflow / step
  3. Click each breadcrumb
  4. Verify navigation works

#### No Actions Without Confirmation

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

#### Context Clear

- [ ] **Check:** User always knows current context
- **Verification:**
  - Context indicators show scope level
  - Color coding distinguishes scope levels
  - Quick actions are context-specific
- **Test Steps:**
  1. Navigate to different scopes
  2. Verify context indicator changes color
  3. Verify quick actions change per scope

### Evidence

- [ ] Header fixed at top, always visible
- [ ] Breadcrumbs show full path at all times
- [ ] Scope changes show confirmation dialog
- [ ] Context indicators clearly show current level

---

## ADR Requirement 4: Drag-and-Drop is Scheduler API Projection

### Description

Dragging a queue item calls scheduler API directly. No direct data modification.

### Compliance Checks

#### Calls Scheduler API

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

#### No Direct Modification

- [ ] **Check:** Dragging doesn't directly modify data
- **Verification:**
  - No direct array manipulation
  - No local queue state update before API
  - All changes go through API
- **Files to Check:**
  - `src-frontend/components/queue/SortableQueueList.tsx`

#### Validates with Scheduler

- [ ] **Check:** Reorder only succeeds when scheduler validates
- **Verification:**
  - Invalid reorder rejected by scheduler
  - UI shows error on rejection
  - Queue doesn't update on rejection
- **Test Steps:**
  1. Try to reorder running item to top
  2. Verify scheduler rejects (if configured)
  3. Verify error message displays
  4. Verify queue doesn't change

#### Error Reverts

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

#### No Optimistic Updates

- [ ] **Check:** No optimistic UI for drag-and-drop
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

### Evidence

- [ ] Network tab shows POST to `/api/queue/reorder`
- [ ] Code review shows no direct array manipulation
- [ ] Invalid reorders rejected by scheduler
- [ ] Errors revert queue to original state
- [ ] Queue doesn't update until API responds

---

## ADR Requirement 5: No Separate Backend

### Description

Glyphnova does not have its own database or API. Uses same backend as CLI.

### Compliance Checks

#### Single Backend

- [ ] **Check:** No separate UI backend service
- **Verification:**
  - Only one backend service: AgentSDK runtime
  - No `ui-backend` process
  - No separate API server for UI
- **Test Steps:**
  1. List running processes
  2. Verify only `agentsdk-runtime` process
  3. No `ui-backend` or similar

#### Uses Runtime API

- [ ] **Check:** Uses same REST and WebSocket APIs as CLI
- **Verification:**
  - UI calls `/api/queue`
  - CLI calls `/api/queue`
  - Same endpoints, same response format
- **Test Steps:**
  1. Get queue via CLI: `agentsdk queue list`
  2. Get queue via UI
  3. Compare responses
  4. Verify identical format

#### Thin Tauri Wrapper

- [ ] **Check:** Tauri backend is a thin wrapper
- **Verification:**
  - Tauri commands just forward to runtime
  - No business logic in Tauri
  - No state management in Tauri
- **Files to Check:**
  - `src/lib.rs`
  - `src/api/mod.rs`

#### No Extra Database

- [ ] **Check:** No extra database or storage
- **Verification:**
  - No SQLite, PostgreSQL, etc. for UI
  - No file-based storage for UI state
  - Only runtime stores data
- **Files to Check:**
  - `src-tauri/Cargo.toml` (check for DB dependencies)
  - `package.json` (check for DB dependencies)

### Evidence

- [ ] Only one backend process: `agentsdk-runtime`
- [ ] CLI and UI call same API endpoints
- [ ] Tauri commands just forward to runtime
- [ ] No database dependencies in UI code

---

## ADR-0004 Compliance Summary

### Compliance Matrix

| ADR Requirement | Status | Evidence |
|----------------|--------|----------|
| 1. UI is a Shell, Not a Separate System | ⬜ | Pending |
| 2. Queue is Direct Projection of Scheduler State | ⬜ | Pending |
| 3. Scope is Always Visible in Header | ⬜ | Pending |
| 4. Drag-and-Drop is Scheduler API Projection | ⬜ | Pending |
| 5. No Separate Backend | ⬜ | Pending |

### Overall Compliance

- [ ] **Fully Compliant**: All 5 requirements met
- [ ] **Partially Compliant**: Some requirements met, others need work
- [ ] **Non-Compliant**: Major violations present

### Next Steps

Based on compliance results:

1. **If Fully Compliant**: Proceed to validation testing
2. **If Partially Compliant**: Fix non-compliant items
3. **If Non-Compliant**: Review architecture and restructure

### Approval

- [ ] Compliance verified by: _________________
- [ ] Date: _________________
- [ ] Approved for production: ⬜ Yes ⬜ No

---

**Document Version:** 1.0
**Last Updated:** 2025-04-06
**Plan Location:** `/home/jon/code/yaml-to-rust-agentsdk/opencode/docs/plans/03-glyphnova-ui/validation/adr-compliance.md`
