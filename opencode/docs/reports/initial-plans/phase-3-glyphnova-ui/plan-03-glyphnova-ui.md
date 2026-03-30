# Plan-03: Glyphnova Desktop UI and Multi-Zoom Control Plane

**Plan ID**: plan-03
**Phase**: Phase 3 - Glyphnova UI
**Status**: Not Started
**Created**: 2026-03-27
**Related ADR**: ADR-0004 (Glyphnova desktop UI and multi-zoom control plane)
**Related Research Plan**: research-plan-03-networking-ui-backends.yml
**Estimated Time**: 8-10 weeks
**Dependencies**: plan-00 (Foundation), plan-01 (MVP Queue & Scheduler), plan-02 (CLI & Backends)

---

## Overview

This plan implements the Glyphnova desktop UI as a shell over the existing runtime and queue control plane. The UI exposes the same runtime abstractions without creating a separate system, maintaining consistency with CLI behavior.

**Key Features**:
- **Desktop Shell Architecture**: Rust backend + web frontend
- **Queue Visualization**: Reflects persistent scheduler states (not separate UI model)
- **Scope Visibility**: Always present, explicit context switch confirmations
- **Multi-Zoom Navigation**: Multiple abstraction levels for workflows
- **Drag-and-Drop Reprioritization**: UI projection of scheduler priority APIs
- **Artifact Browsers**: Browse and search workflow artifacts
- **Summary and Graph Navigation**: Visual representations of workflow structure
- **Shared Backend API Boundary**: Single API for both CLI and UI

**Key Deliverables**:
- Desktop shell with Rust backend and web frontend
- Queue visualization with live execution status
- Scope indicators and context switch safety UI
- Multi-zoom navigation (workflow → step → agent → tool)
- Drag-and-drop queue reprioritization
- Artifact browser with search and filtering
- Summary and graph views for workflows
- Shared backend API for CLI and UI

---

## Code Review

### ADR-0004 Summary

**Decision**: Build Glyphnova UI as a shell over existing runtime and queue control plane.

**Key Requirements**:
1. **Desktop Shell Architecture**: Rust backend + web frontend
2. **Queue Visualization**: Reflects persistent scheduler states (not separate model)
3. **Scope Visibility**: Always present, context switches require explicit user acknowledgment
4. **Multi-Zoom Navigation**: Supports multiple abstraction levels
5. **Drag-and-Drop Reprioritization**: UI projection of scheduler priority APIs
6. **Shared Backend API Boundary**: Single API for CLI and UI

**Scope**:
- **Included**: Desktop shell architecture, queue views and live execution meaning, scope indicators and context switch safety, artifact browsers and search interfaces, summary and graph navigation, drag-and-drop reprioritization as scheduler control, shared backend API boundary
- **Excluded**: Changes to core scheduler semantics, first implementation of web scraping or memory, deep autonomy logic

**Positive Consequences**:
- No divergence between CLI and UI
- Future observability and graph features become natural
- UI remains honest to runtime state

**Negative Consequences**:
- UI delivery waits on backend readiness
- Graph navigation increases data-model complexity

### Requirements Review

From `requirements.mdc` and `schema-consolidated-report.md`:

**R05**: CLI/TUI-first path then richer desktop UI
**R08**: Left-side chat/work queue with titles and live execution meaning
**R10**: Multi-zoom navigation
**R11**: Visible scope and explicit context-switch confirmation

**Validation Focus for v0.1.0**:
- Desktop shell exposes runtime queue state correctly
- Queue visualization reflects scheduler states not a separate model
- Scope indicators prevent context confusion
- Drag-and-drop operations call the same scheduler priority APIs
- Navigation provides multiple abstraction levels for workflows
- Shared backend API boundary prevents duplication

**Validation Focus for v1.0 and Later**:
- Graph and summary views scale for complex workflows
- Artifact browsers support workflow inspection and version comparison
- Search and filtering capabilities enable workflow discovery
- Drag-and-drop improves operator efficiency without breaking scheduler logic

### Research Plan Review

**research-plan-03-networking-ui-backends.yml** is **COMPLETED** with 5 research domains (plan-03 uses domain 4):

4. **Desktop Shell Architecture**: How should a desktop shell expose runtime state without duplicating logic?

**Key Findings**:
- Report outputs: `backends-ui-research-report.md`
- Quality gates met: UI recommendations preserve shared runtime abstractions

---

## Web Research

### Research Area 1: Desktop Application Architecture

**Research Question**: What architecture patterns exist for building Rust backends with web frontends?

**Recommended Sources**:
- [tauri.app](https://tauri.app) - Rust backend + web frontend
- [Electron + Rust](https://electronjs.org) - Rust FFI with Electron
- [wry](https://docs.rs/wry) - Lightweight WebView wrapper
- [Neon](https://neon-bindings.com) - Rust bindings to Node.js
- [GitHub - desktop patterns](https://github.com) - Search for "rust desktop app" projects

**Expected Findings**:
- Desktop shell architecture patterns
- Rust-web communication mechanisms
- State synchronization between backend and frontend
- Cross-platform deployment strategies
- Performance considerations for desktop apps

**Status**: Not Started

---

### Research Area 2: Real-Time UI Updates

**Research Question**: What patterns exist for real-time updates from Rust backend to web frontend?

**Recommended Sources**:
- [tokio.rs](https://tokio.rs) - Async runtime patterns
- [warp](https://docs.rs/warp) - WebSocket server
- [actix-web](https://docs.rs/actix-web) - WebSocket support
- [axum](https://docs.rs/axum) - WebSocket examples
- [web sockets](https://websockets.spec.whatwg.org) - WebSocket protocol

**Expected Findings**:
- WebSocket server implementation patterns
- Event streaming from backend
- Frontend reactivity patterns (React, Vue, etc.)
- Connection lifecycle management
- Error handling and reconnection

**Status**: Not Started

---

### Research Area 3: Drag-and-Drop UI Patterns

**Research Question**: What are best practices for implementing drag-and-drop reprioritization in web UIs?

**Recommended Sources**:
- [react-dnd](https://react-dnd.github.io/react-dnd) - React drag-and-drop
- [dnd-kit](https://dndkit.com) - Modern drag-and-drop library
- [SortableJS](https://sortablejs.github.io/Sortable) - Sortable lists
- [HTML5 Drag and Drop API](https://developer.mozilla.org/en-US/docs/Web/API/HTML_Drag_and_Drop_API) - Native API
- [GitHub - drag-and-drop examples](https://github.com) - Search for "drag drop queue" projects

**Expected Findings**:
- Drag-and-drop library selection
- Queue reordering patterns
- Visual feedback during drag
- Drop zone handling
- Accessibility considerations

**Status**: Not Started

---

### Research Area 4: Graph Visualization for Workflows

**Research Question**: What libraries and patterns exist for visualizing workflow graphs in web UIs?

**Recommended Sources**:
- [D3.js](https://d3js.org) - Data visualization library
- [React Flow](https://reactflow.dev) - Node-based graph editor
- [Cytoscape.js](https://js.cytoscape.org) - Graph theory library
- [vis.js](https://visjs.github.io) - Network visualization
- [mermaid](https://mermaid.js.org) - Diagram generation from text

**Expected Findings**:
- Graph rendering library options
- Node and edge positioning algorithms
- Interactive features (zoom, pan, select)
- Performance optimization for large graphs
- Layout algorithms (dagre, elk, etc.)

**Status**: Not Started

---

### Research Area 5: Cross-Platform Desktop Deployment

**Research Question**: How to package and distribute Rust-backed desktop applications across platforms?

**Recommended Sources**:
- [tauri.app](https://tauri.app) - Tauri distribution
- [electron-builder](https://www.electron.build) - Electron packaging
- [GitHub Actions](https://github.com/features/actions) - CI/CD for releases
- [Homebrew](https://brew.sh) - macOS package management
- [Snapcraft](https://snapcraft.io) - Linux packaging

**Expected Findings**:
- Cross-platform build processes
- Code signing and notarization
- Package manager integration
- Update mechanisms
- Installation and uninstallation UX

**Status**: Not Started

---

## Implementation Plan

### Phase 1: Desktop Shell Setup

**Description**: Set up desktop application framework with Rust backend and web frontend

**Tasks**:
1. Initialize Tauri project structure
2. Set up Rust backend scaffolding
3. Initialize web frontend (React/TypeScript)
4. Configure build system for cross-platform
5. Set up development workflow (hot reload)
6. Implement window management
7. Add basic logging and error handling

**Related Requirements**: ADR-0004 (desktop shell architecture)
**Verification Layers**: 1, 2
**Status**: Not Started

---

### Phase 2: Shared Backend API

**Description**: Implement API boundary that both CLI and UI use

**Tasks**:
1. Define REST API endpoints (HTTP)
2. Define WebSocket endpoints (real-time)
3. Implement API authentication/authorization
4. Add rate limiting
5. Implement request/response serialization
6. Add API documentation (OpenAPI)
7. Create API client library for internal use
8. Implement API versioning

**Related Requirements**: Shared backend API boundary
**Verification Layers**: 1, 2, 3
**Status**: Not Started

---

### Phase 3: Queue Visualization

**Description**: Implement queue view that reflects scheduler state in real-time

**Tasks**:
1. Design queue view layout (left panel)
2. Implement queue item rendering (titles, status, progress)
3. Add live execution status updates
4. Implement WebSocket connection for real-time updates
5. Add queue filtering and sorting
6. Implement queue item selection
7. Add queue item actions (pause, resume, cancel)

**Related Requirements**: R08 (left-side chat/work queue)
**Verification Layers**: 1, 2, 4
**Status**: Not Started

---

### Phase 4: Scope Indicators and Context Switch Safety

**Description**: Implement always-visible scope and explicit context switch confirmation

**Tasks**:
1. Design scope indicator UI (header/breadcrumb)
2. Implement scope display for current context
3. Add context switch detection
4. Implement confirmation dialog for context switches
5. Add scope history (breadcrumb navigation)
6. Implement scope change logging
7. Add scope-based UI filtering

**Related Requirements**: R11 (visible scope, explicit confirmation)
**Verification Layers**: 1, 2, 4
**Status**: Not Started

---

### Phase 5: Multi-Zoom Navigation

**Description**: Implement navigation at multiple abstraction levels

**Tasks**:
1. Define abstraction levels (workflow → step → agent → tool)
2. Implement navigation state machine
3. Design navigation UI (sidebar/panel)
4. Implement level transitions
5. Add level-specific views and details
6. Implement navigation breadcrumbs
7. Add keyboard shortcuts for navigation
8. Implement navigation history (back/forward)

**Related Requirements**: R10 (multi-zoom navigation)
**Verification Layers**: 1, 2, 4
**Status**: Not Started

---

### Phase 6: Drag-and-Drop Reprioritization

**Description**: Implement drag-and-drop queue reprioritization using scheduler APIs

**Tasks**:
1. Integrate drag-and-drop library
2. Implement drag handle on queue items
3. Implement drop zones in queue
4. Add visual feedback during drag (ghost, drop indicators)
5. Call scheduler priority APIs on drop
6. Update queue state after reprioritization
7. Handle reprioritization errors
8. Add undo/redo for drag operations

**Related Requirements**: Drag-and-drop as scheduler control
**Verification Layers**: 1, 2, 3, 4
**Status**: Not Started

---

### Phase 7: Artifact Browser

**Description**: Implement browsing and searching of workflow artifacts

**Tasks**:
1. Design artifact browser layout
2. Implement artifact listing (runs, logs, outputs, metrics)
3. Add artifact preview (logs, outputs)
4. Implement search and filtering
5. Add artifact comparison (diffing)
6. Implement artifact versioning UI
7. Add artifact export functionality
8. Implement artifact deletion (with confirmation)

**Related Requirements**: Artifact browsers and search interfaces
**Verification Layers**: 1, 2, 4
**Status**: Not Started

---

### Phase 8: Summary and Graph Views

**Description**: Implement visual representations of workflow structure

**Tasks**:
1. Integrate graph visualization library
2. Implement workflow-to-graph transformation
3. Design graph layout (DAG representation)
4. Implement graph rendering
5. Add graph interaction (zoom, pan, select)
6. Implement graph navigation (click to zoom)
7. Add summary view (key metrics, status)
8. Implement graph and summary synchronization

**Related Requirements**: Summary and graph navigation
**Verification Layers**: 1, 2, 4
**Status**: Not Started

---

## Verification Strategy

### Layer 1: Unit Tests

**Scope**: Individual component testing

**Coverage Areas**:
- API endpoint handlers
- WebSocket connection management
- Queue state transformations
- Navigation state machine
- Drag-and-drop event handling
- Artifact filtering and search logic
- Graph data transformation
- Scope validation

**Test Framework**: `cargo test --lib` + Jest/Vitest for frontend

**Success Criteria**:
- 90%+ code coverage on backend modules
- 80%+ code coverage on frontend components
- All edge cases tested (empty states, invalid inputs, etc.)

---

### Layer 2: Integration Tests

**Scope**: Multi-component interaction testing

**Coverage Areas**:
- CLI and UI calling same backend API
- WebSocket real-time updates from scheduler
- Drag-and-drop triggering scheduler priority changes
- Navigation triggering correct context switches
- Artifact browser loading from storage
- Graph view rendering from workflow data
- Scope indicators reflecting actual scope

**Test Framework**: `cargo test --test '*'` + Playwright for frontend

**Success Criteria**:
- All API endpoints work correctly
- WebSocket connections are stable
- UI reflects backend state accurately
- Drag-and-drop operations update scheduler
- Navigation is safe and predictable
- Artifact browser displays correct data
- Graph views render correctly

---

### Layer 3: Property-Based Tests

**Scope**: Edge cases and invariants

**Coverage Areas**:
- Queue state transitions (always valid)
- Navigation state machine (no invalid transitions)
- Drag-and-drop (order preserved, no duplicates)
- Scope boundaries (never violated)
- Graph transformations (DAG properties preserved)
- Artifact filtering (correct subset)

**Test Framework**: `proptest` (backend) + property-based testing (frontend)

**Success Criteria**:
- No crashes on random inputs
- Invariants always hold (queue always valid, scope always respected)
- Properties verified with 1000+ iterations

---

### Layer 4: End-to-End Tests

**Scope**: Full user workflows

**Coverage Areas**:
- Launch desktop application successfully
- Connect to backend and load queue
- Drag-and-drop reorder queue items
- Navigate between abstraction levels
- Context switch with explicit confirmation
- Browse and search artifacts
- View workflow graph
- Pause/resume/cancel workflows
- All operations work across restart

**Test Framework**: `cargo test --test '*e2e*'` + Playwright E2E tests

**Success Criteria**:
- Complete user workflows execute successfully
- UI remains responsive during operations
- No state corruption across sessions
- Real-time updates work correctly
- Cross-platform behavior is consistent

---

## Verification Checkpoints

### Checkpoint 1: Desktop Shell Running

**Target Date**: Week 2
**Verification Layers**: 1, 2
**Sign-Off Criteria**:
- [ ] Tauri app launches on macOS, Windows, Linux
- [ ] Rust backend starts successfully
- [ ] Web frontend loads in window
- [ ] Development workflow works (hot reload)
- [ ] Logging captures errors correctly

**Status**: Not Started

---

### Checkpoint 2: Shared API Working

**Target Date**: Week 4
**Verification Layers**: 1, 2, 3
**Sign-Off Criteria**:
- [ ] All API endpoints are accessible
- [ ] WebSocket connections are stable
- [ ] API client library works
- [ ] CLI and UI use same API successfully
- [ ] Property tests pass on API contracts

**Status**: Not Started

---

### Checkpoint 3: Queue Visualization Live

**Target Date**: Week 5
**Verification Layers**: 1, 2, 4
**Sign-Off Criteria**:
- [ ] Queue view displays correctly
- [ ] Real-time updates work via WebSocket
- [ ] Queue items show correct status
- [ ] Filtering and sorting work
- [ ] Queue actions (pause/resume/cancel) work
- [ ] E2E test: queue updates in real-time

**Status**: Not Started

---

### Checkpoint 4: Scope and Navigation Safe

**Target Date**: Week 6
**Verification Layers**: 1, 2, 4
**Sign-Off Criteria**:
- [ ] Scope indicator always visible
- [ ] Context switches trigger confirmation
- [ ] Navigation works at all levels
- [ ] Navigation breadcrumbs display correctly
- [ ] Scope-based filtering works
- [ ] E2E test: safe context switch workflow

**Status**: Not Started

---

### Checkpoint 5: Drag-and-Drop Reprioritizes

**Target Date**: Week 7
**Verification Layers**: 1, 2, 3, 4
**Sign-Off Criteria**:
- [ ] Drag-and-drop works smoothly
- [ ] Visual feedback is clear
- [ ] Scheduler priority APIs called on drop
- [ ] Queue order updates correctly
- [ ] Undo/redo works for drag operations
- [ ] Property tests verify ordering invariants
- [ ] E2E test: full drag-and-drop workflow

**Status**: Not Started

---

### Checkpoint 6: Artifact Browser Functional

**Target Date**: Week 8
**Verification Layers**: 1, 2, 4
**Sign-Off Criteria**:
- [ ] Artifact browser displays all artifact types
- [ ] Search and filtering work correctly
- [ ] Artifact preview works
- [ ] Comparison/diff works
- [ ] Export functionality works
- [ ] E2E test: browse, search, export artifacts

**Status**: Not Started

---

### Checkpoint 7: Summary and Graph Views Working

**Target Date**: Week 9
**Verification Layers**: 1, 2, 4
**Sign-Off Criteria**:
- [ ] Graph visualization renders correctly
- [ ] Graph interaction (zoom/pan/select) works
- [ ] Navigation from graph to details works
- [ ] Summary view displays correct metrics
- [ ] Graph and summary are synchronized
- [ ] E2E test: view workflow graph and navigate

**Status**: Not Started

---

### Checkpoint 8: Cross-Platform Deployment

**Target Date**: Week 10
**Verification Layers**: 2, 4
**Sign-Off Criteria**:
- [ ] Builds successfully on macOS
- [ ] Builds successfully on Windows
- [ ] Builds successfully on Linux
- [ ] Installation works on all platforms
- [ ] E2E tests pass on all platforms
- [ ] Update mechanism works

**Status**: Not Started

---

## Progress Tracking

### Overall Progress

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Phases Complete | 8 | 0 | 0% |
| Verification Layers | 4 | 0 | 0% |
| Checkpoints Passed | 8 | 0 | 0% |
| Web Research Areas | 5 | 0 | 0% |

**Overall Status**: Not Started

---

### Phase Progress

| Phase | Description | Status | Completion |
|-------|-------------|--------|------------|
| 1 | Desktop Shell Setup | Not Started | 0% |
| 2 | Shared Backend API | Not Started | 0% |
| 3 | Queue Visualization | Not Started | 0% |
| 4 | Scope Indicators and Context Switch Safety | Not Started | 0% |
| 5 | Multi-Zoom Navigation | Not Started | 0% |
| 6 | Drag-and-Drop Reprioritization | Not Started | 0% |
| 7 | Artifact Browser | Not Started | 0% |
| 8 | Summary and Graph Views | Not Started | 0% |

---

### Verification Layer Progress

| Layer | Description | Tests Written | Tests Passing | Coverage |
|-------|-------------|----------------|---------------|----------|
| 1 | Unit Tests | 0 | 0 | 0% |
| 2 | Integration Tests | 0 | 0 | 0% |
| 3 | Property-Based Tests | 0 | 0 | 0% |
| 4 | End-to-End Tests | 0 | 0 | 0% |

---

### Research Progress

| Research Area | Status | Evidence Collected | Synthesized |
|--------------|--------|-------------------|-------------|
| Desktop Application Architecture | Not Started | 0 | No |
| Real-Time UI Updates | Not Started | 0 | No |
| Drag-and-Drop UI Patterns | Not Started | 0 | No |
| Graph Visualization for Workflows | Not Started | 0 | No |
| Cross-Platform Desktop Deployment | Not Started | 0 | No |

---

## Dependencies

### Blocks

- plan-00 (Foundation): Needed for WorkflowIR and storage
- plan-01 (MVP Queue): Needed for queue and scheduler state
- plan-02 (CLI & Backends): Needed for shared backend API

### Unblocks

- plan-04: Quality Loops (depends on UI visualization of metrics)
- plan-05: Memory & Search (depends on artifact browser)
- plan-06: Automation (depends on UI for monitoring)
- plan-07: Autonomy & Metrics (depends on all UI features)

### Integration Points

- **plan-00 (Foundation)**: Uses WorkflowIR, storage layer, validation
- **plan-01 (MVP Queue)**: Visualizes queue state, scheduler events
- **plan-02 (CLI & Backends)**: Shares backend API, calls same scheduler APIs

---

## Quality Gates

### ADR-0004 Quality Gates

1. **Queue State Reflection**: UI queue matches scheduler state exactly (not separate model)
2. **Scope Indicators Present**: Scope is always visible and context switches require confirmation
3. **Navigation Safety**: Navigation cannot lead to invalid or dangerous states
4. **Drag-and-Drop Correctness**: Drag operations call scheduler APIs, queue order updates correctly
5. **Shared API Boundary**: CLI and UI use identical backend API (no duplication)

### Critical Review Upstream Factors

1. **Real-Time Performance**: WebSocket updates deliver state changes within 100ms
2. **State Consistency**: UI never shows stale state compared to backend
3. **Cross-Platform Fidelity**: Behavior is identical across macOS, Windows, Linux
4. **Graph Scalability**: Graph views handle workflows with 100+ nodes without lag
5. **Drag-and-Drop Safety**: Undo/redo prevents loss of queue items
6. **Scope Boundary Enforcement**: No operation can bypass scope indicators
7. **Navigation Correctness**: Navigation breadcrumbs always reflect true state
8. **Artifact Browser Performance**: Search returns results within 1s for 10,000+ artifacts

---

## Next Steps

### Workflow for Execution

1. **Code Review**: Read ADR-0004 and related research plan findings
2. **Web Research**: Complete 5 research areas with evidence collection
3. **Update Plan**: Incorporate research findings into implementation plan
4. **Write Tests**: Start with unit tests for Phase 1 (Desktop Shell Setup)
5. **Implement Phase 1**: Build desktop shell with test-driven development
6. **Verify Checkpoint 1**: Run Layer 1 and 2 tests, update plan with results
7. **Continue Phases**: Repeat test-first approach for phases 2-8
8. **All Checkpoints**: Verify each checkpoint with all applicable layers
9. **Update Plan**: Document all working code and verification results
10. **Proceed**: Mark plan complete and move to plan-04

### Starting Point

Begin with Phase 1 (Desktop Shell Setup):
1. Initialize Tauri project
2. Set up Rust backend scaffolding
3. Initialize web frontend (React/TypeScript)
4. Write unit tests for window management
5. Implement basic window setup
6. Verify Checkpoint 1

---

## Execution Commands

### Verify All Tests

```bash
# Backend tests
cargo test --lib              # Layer 1: Unit tests
cargo test --test '*'           # Layer 2: Integration tests
cargo test --test '*proptest*'  # Layer 3: Property-based tests

# Frontend tests
npm test                       # Layer 1: Unit tests
npm run test:e2e               # Layer 4: E2E tests

# Combined
cargo test && npm test && npm run test:e2e
```

### Verify Single Phase

```bash
# Verify Phase 1 (Desktop Shell Setup)
cargo test --lib desktop
npm run test desktop

# Verify Phase 2 (Shared Backend API)
cargo test --lib api
cargo test --test api_integration
cargo test --test api_proptest
```

### Run Desktop Application

```bash
# Development mode (with hot reload)
npm run tauri dev

# Build for production
npm run tauri build

# Build for specific platform
npm run tauri build --target x86_64-apple-darwin
```

### Use implement.sh Script

```bash
# Start from beginning
./implement.sh start plan-03

# Resume from checkpoint
./implement.sh resume plan-03 cp4

# Check progress
./implement.sh status plan-03

# Generate progress report
./implement.sh report plan-03
```

---

## References

### Related Documents

- **ADR-0004**: [Glyphnova desktop UI and multi-zoom control plane](../roadmap/adr-0004-glyphnova-ui-control-plane.yml)
- **Research Plan 03**: [Networking boundary, backend providers, and Glyphnova UI research](../roadmap/research-plan-03-networking-ui-backends.yml)
- **Plan 00**: [Foundation Compiler Contract](phase-0-foundation/plan-00-foundation.md)
- **Plan 01**: [MVP Queue & Scheduler](phase-1-mvp-queue/plan-01-mvp-queue.md)
- **Plan 02**: [CLI & Backends](phase-2-cli-backends/plan-02-cli-backends.md)
- **Requirements**: [schema-consolidated-report.md](../requirements/schema-consolidated-report.md)

### Implementation References

- **Tauri**: [tauri.app](https://tauri.app) - Desktop app framework
- **React Flow**: [reactflow.dev](https://reactflow.dev) - Graph visualization
- **D3.js**: [d3js.org](https://d3js.org) - Data visualization
- **DND Kit**: [dndkit.com](https://dndkit.com) - Drag-and-drop
- **Playwright**: [playwright.dev](https://playwright.dev) - E2E testing
- **warp**: [docs.rs/warp](https://docs.rs/warp) - WebSocket server
- **actix-web**: [docs.rs/actix-web](https://docs.rs/actix-web) - HTTP server

### Tool and Plugin References

- **OpenCode Tools**: File operations, web browsing, shell execution
- **dev-browser**: Browser automation for UI testing
- **bash tool**: Command execution for builds
- **lsp_diagnostics**: Type checking for Rust and TypeScript

---

**Last Updated**: 2026-03-27
**Status**: Not Started
