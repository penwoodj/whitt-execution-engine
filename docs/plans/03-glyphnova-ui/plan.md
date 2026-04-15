# Phase 3: Glyphnova UI Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a native desktop application (Glyphnova) that provides a visual shell over the AgentSDK runtime, enabling real-time monitoring, queue management, and workflow visualization through a unified interface that uses the same backend API as the CLI.

**Architecture:** Glyphnova is a Tauri-based desktop application with a Rust backend that acts as a thin wrapper around the existing AgentSDK runtime. The frontend is a React/TypeScript application that communicates with the runtime via the same REST and WebSocket endpoints used by the CLI. The UI is purely a projection layer - no business logic, no separate state, just visualization of runtime state.

**Tech Stack:**
- **Desktop Framework:** Tauri 2.0 (Rust backend + WebView frontend)
- **Frontend:** React 18 + TypeScript + Vite
- **UI Library:** shadcn/ui (Radix UI + Tailwind CSS)
- **State Management:** Zustand (simple, lightweight)
- **Data Fetching:** React Query + WebSocket client
- **Graph Visualization:** React Flow (for DAG visualization)
- **Charts:** Recharts (for metrics)
- **Backend API:** Existing Phase 2 REST + WebSocket endpoints
- **Build Tool:** Tauri CLI (cross-platform builds for Linux/macOS/Windows)

---

## Table of Contents

1. [ADR-0004 Compliance](#adr-0004-compliance)
2. [Dependencies](#dependencies)
3. [Architecture Overview](#architecture-overview)
4. [File Structure](#file-structure)
5. [Task Breakdown](#task-breakdown)
6. [Timeline](#timeline)
7. [Validation Criteria](#validation-criteria)

---

## ADR-0004 Compliance

This implementation plan strictly follows ADR-0004 (Glyphnova UI Design Decisions). Key constraints:

### 1. UI is a Shell, Not a Separate System
- **Constraint:** The UI does NOT maintain its own state or business logic
- **Implementation:** All state comes from runtime API calls
- **Example:** Queue display = GET /api/queue, NOT a local array
- **Proof:** UI will break if backend is down (expected behavior)

### 2. Queue is a Direct Projection of Scheduler State
- **Constraint:** Queue order, status, and metadata come directly from scheduler
- **Implementation:** Poll GET /api/queue every 1-2 seconds or use WebSocket updates
- **Example:** If scheduler marks task as "running", UI shows "running" icon
- **No:** Do NOT cache queue state locally beyond what React Query does

### 3. Scope is Always Visible in Header
- **Constraint:** User must always know which context they're operating in
- **Implementation:** Fixed header with breadcrumbs showing workspace → project → workflow → step hierarchy
- **Example:** Header: "agentsdk-workspace / my-project / process-docs / extract-text"
- **No:** Do NOT allow actions that change scope without explicit confirmation

### 4. Drag-and-Drop is Scheduler API Projection
- **Constraint:** Dragging a queue item calls scheduler API, doesn't directly modify data
- **Implementation:** dnd-kit or @dnd-kit/core library → on drop → POST /api/queue/reorder
- **Example:** Drag task A above task B → Backend validates and reorders
- **No:** Do NOT implement optimistic UI that assumes success

### 5. No Separate Backend
- **Constraint:** Glyphnova does NOT have its own database or API
- **Implementation:** Tauri backend is a thin wrapper that forwards to AgentSDK runtime
- **Example:** Frontend calls `invoke('api_request', { method: 'GET', path: '/api/queue' })`
- **No:** Do NOT create a separate "ui-backend" service

---

## Dependencies

### Required Prerequisites (Must be Complete)

1. **Phase 0: Foundation** ✅
   - Project structure and tooling
   - Cargo workspace configuration
   - Basic Rust/C++ bridge setup

2. **Phase 1: Core Runtime** ✅
   - Agent execution engine
   - Tool system (Python tool bridge)
   - Workflow engine
   - Scheduler system
   - Queue management
   - Storage system

3. **Phase 2: CLI Framework** ✅
   - CLI command structure
   - Shared backend API (REST + WebSocket)
   - Queue/Scheduler API endpoints
   - Workflow management endpoints
   - Storage access endpoints
   - Metrics and logging endpoints
   - OpenAPI specification

### Can Start in Parallel With

- **Phase 4: Agent Library Development** - After Phase 2 completes, UI and agent library can be developed in parallel

### External Dependencies

- Node.js 18+ (for frontend dev)
- Tauri CLI (installed via cargo)
- WebKitGTK (Linux) or Safari (macOS) or WebView2 (Windows)

---

## Architecture Overview

### System Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Glyphnova Desktop                      │
│  ┌───────────────────────────────────────────────────┐  │
│  │              React/TypeScript Frontend              │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────────┐    │  │
│  │  │ Queue    │  │ DAG      │  │ Metrics      │    │  │
│  │  │ View     │  │ Graph    │  │ Dashboard    │    │  │
│  │  └──────────┘  └──────────┘  └──────────────┘    │  │
│  └───────────────────────────────────────────────────┘  │
│                      ↓ (IPC)                             │
│  ┌───────────────────────────────────────────────────┐  │
│  │              Tauri Backend (Rust)                 │  │
│  │         (Thin wrapper, no business logic)         │  │
│  └───────────────────────────────────────────────────┘  │
│                      ↓                                  │
│  ┌───────────────────────────────────────────────────┐  │
│  │         AgentSDK Runtime (Existing)                 │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────────┐    │  │
│  │  │ REST     │  │ WebSocket│  │ Scheduler    │    │  │
│  │  │ API      │  │ Server   │  │ Engine       │    │  │
│  │  └──────────┘  └──────────┘  └──────────────┘    │  │
│  └───────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

### Data Flow

1. **REST API Calls** (State queries)
   - Frontend → Tauri invoke → Runtime REST API → Response
   - Used for: Initial data load, actions, queries
   - Examples: GET /api/queue, POST /api/workflows/{id}/execute

2. **WebSocket Updates** (Real-time)
   - Frontend → WebSocket connection → Runtime WS server → Push updates
   - Used for: Queue state changes, agent progress, tool execution
   - Examples: `queue:update`, `agent:progress`, `tool:complete`

3. **Drag-and-Drop** (Scheduler projection)
   - User drags queue item → Frontend calculates new order → POST /api/queue/reorder → Success/Failure → Update UI

### Component Architecture

```
src/
├── main.rs                      # Tauri entry point
├── lib.rs                       # Tauri command handlers
└── api/
    └── mod.rs                   # API invocation wrappers

src-frontend/
├── App.tsx                      # Root component
├── main.tsx                     # React entry point
├── components/
│   ├── layout/
│   │   ├── Header.tsx           # Breadcrumbs, scope
│   │   ├── Sidebar.tsx          # Queue view
│   │   └── MainContent.tsx      # Dynamic content
│   ├── queue/
│   │   ├── QueueList.tsx        # Queue item list
│   │   ├── QueueItem.tsx        # Single queue item
│   │   └── QueueFilters.tsx     # Filter/sort controls
│   ├── navigation/
│   │   ├── ZoomControls.tsx     # Zoom level selector
│   │   └── BreadcrumbNav.tsx    # Navigation breadcrumbs
│   ├── visualization/
│   │   ├── WorkflowGraph.tsx    # React Flow DAG
│   │   └── MetricsChart.tsx     # Recharts visualization
│   └── artifacts/
│       ├── ArtifactBrowser.tsx  # Artifact list
│       └── ArtifactPreview.tsx  # Artifact preview panel
├── hooks/
│   ├── useQueue.ts              # Queue state management
│   ├── useWebSocket.ts          # WebSocket connection
│   └── useApi.ts                # API query hooks
├── stores/
│   └── appStore.ts              # Global app state
├── types/
│   └── api.ts                   # TypeScript API types
└── utils/
    ├── api-client.ts            # API invocation helpers
    └── websocket.ts             # WebSocket client
```

---

## File Structure

### New Files Created by This Plan

```
agentsdk/
├── glyphnova/                    # NEW: Tauri desktop app
│   ├── src/
│   │   ├── main.rs              # Tauri entry point
│   │   ├── lib.rs               # Command handlers
│   │   └── api/
│   │       └── mod.rs          # API wrappers
│   ├── src-frontend/            # React frontend
│   │   ├── App.tsx
│   │   ├── main.tsx
│   │   ├── components/          # UI components
│   │   ├── hooks/               # React hooks
│   │   ├── stores/              # Zustand stores
│   │   ├── types/               # TypeScript types
│   │   └── utils/               # Utility functions
│   ├── src-tauri/               # Tauri configuration
│   │   ├── tauri.conf.json
│   │   └── Cargo.toml
│   ├── package.json
│   ├── tsconfig.json
│   ├── vite.config.ts
│   └── tailwind.config.js
```

### Modified Files

- `agentsdk/Cargo.toml` - Add `glyphnova` workspace member
- `agentsdk/Makefile` - Add UI build targets
- `agentsdk/README.md` - Add UI documentation

---

## Task Breakdown

This plan consists of 9 major tasks, each broken down into bite-sized steps (2-5 minutes each).

### Task Overview

1. **Desktop Shell Setup** - Initialize Tauri project with React/TypeScript
2. **Shared Backend API** - Connect frontend to runtime API (REST + WS)
3. **Queue Visualization** - Left panel queue display with live updates
4. **Scope Indicators** - Header with breadcrumbs and context
5. **Multi-Zoom Navigation** - Abstraction level navigation (workflow→step→agent→tool)
6. **Drag-Drop Reprioritization** - Queue reordering via scheduler API
7. **Artifact Browser** - Artifact listing, preview, search, export
8. **Summary Graph Views** - Workflow DAG visualization
9. **Quality Metrics Dashboard** - Metrics trends and alerts

### Task Files

Each task has a dedicated file in the `tasks/` directory with detailed implementation steps:

- `tasks/00-desktop-shell-setup.md` - Tauri initialization
- `tasks/01-shared-backend-api.md` - API integration
- `tasks/02-queue-visualization.md` - Queue component
- `tasks/03-scope-indicators.md` - Header/navigation
- `tasks/04-multi-zoom-navigation.md` - Zoom levels
- `tasks/05-drag-drop-reprioritization.md` - Drag-and-drop
- `tasks/06-artifact-browser.md` - Artifacts
- `tasks/07-summary-graph-views.md` - DAG graphs
- `tasks/08-quality-metrics-dashboard.md` - Metrics

---

## Timeline

**Estimated Duration:** 10-12 weeks (assuming 1 full-time developer)

### Week-by-Week Breakdown

**Week 1-2: Desktop Shell Setup**
- Task 00: Desktop Shell Setup (5 days)
- Milestone: Tauri app builds and runs with empty UI

**Week 3-4: Shared Backend API**
- Task 01: Shared Backend API (5 days)
- Milestone: Frontend can query queue state via API

**Week 5-6: Queue Visualization**
- Task 02: Queue Visualization (5 days)
- Milestone: Queue displays in left panel with live updates

**Week 7: Scope Indicators**
- Task 03: Scope Indicators (3 days)
- Milestone: Header shows breadcrumbs and context

**Week 8: Multi-Zoom Navigation**
- Task 04: Multi-Zoom Navigation (4 days)
- Milestone: User can navigate between abstraction levels

**Week 9: Drag-Drop Reprioritization**
- Task 05: Drag-Drop Reprioritization (4 days)
- Milestone: Queue items can be reordered via drag-and-drop

**Week 10: Artifact Browser**
- Task 06: Artifact Browser (4 days)
- Milestone: Artifacts can be browsed, previewed, and exported

**Week 11: Summary Graph Views**
- Task 07: Summary Graph Views (4 days)
- Milestone: DAG graph displays workflows

**Week 12: Quality Metrics Dashboard**
- Task 08: Quality Metrics Dashboard (3 days)
- Milestone: Metrics dashboard with trends and alerts
- Integration and testing (2 days)

### Parallel Development Opportunities

After Task 02 (Queue Visualization) is complete, Tasks 03-08 can potentially be developed in parallel by multiple developers, as they operate on different UI components.

---

## Validation Criteria

This implementation plan must satisfy the following validation criteria (detailed in `validation/criteria.md`):

### Functional Requirements

- [ ] UI displays real-time queue state from scheduler
- [ ] Header always shows current scope (workspace/project/workflow/step)
- [ ] User can navigate between abstraction levels (workflow→step→agent→tool)
- [ ] Drag-and-drop reordering calls scheduler API
- [ ] Artifacts can be browsed, previewed, and exported
- [ ] DAG graph visualizes workflow structure
- [ ] Metrics dashboard shows quality trends

### ADR-0004 Compliance

- [ ] UI is a projection of runtime state (no local state duplication)
- [ ] Queue order matches scheduler state exactly
- [ ] Drag-and-drop actions call scheduler API directly
- [ ] No separate backend or database for UI
- [ ] Scope is always visible in header

### Technical Requirements

- [ ] Tauri app builds for Linux/macOS/Windows
- [ ] Frontend uses shared REST and WebSocket APIs
- [ ] WebSocket connection handles real-time updates
- [ ] Error handling displays user-friendly messages
- [ ] Hot reload works in development mode

### Performance Requirements

- [ ] Queue updates reflect within 2 seconds of scheduler state change
- [ ] DAG graphs render workflows with 100+ nodes smoothly
- [ ] Metrics dashboard updates without UI lag
- [ ] WebSocket connection auto-reconnects on failure

### Testing Requirements

- [ ] Unit tests for all UI components
- [ ] Integration tests with mock API
- [ ] E2E tests for critical workflows
- [ ] Mock WebSocket server for testing

See `validation/criteria.md` for complete validation criteria.

---

## Implementation Guidelines

### Code Style

- **Rust:** Follow `rustfmt` and `clippy` recommendations
- **TypeScript:** Follow `eslint` and `prettier` configuration
- **React:** Use functional components with hooks
- **CSS:** Use Tailwind CSS utility classes

### Testing Strategy

- **Unit Tests:** Test individual components in isolation
- **Integration Tests:** Test API calls with mock server
- **E2E Tests:** Test user workflows with Playwright
- **Mock Strategies:** See `tests/mock-strategies.md`

### Error Handling

- Display user-friendly error messages
- Log errors to runtime error system
- Auto-reconnect WebSocket on failure
- Show loading states for async operations

### Accessibility

- Follow WCAG 2.1 AA guidelines
- Keyboard navigation support
- Screen reader support
- High contrast mode support

---

## Next Steps

After reviewing this plan:

1. **Review Task Files:** Read through each task file in `tasks/` directory
2. **Review Validation Criteria:** Check `validation/criteria.md` and `validation/adr-compliance.md`
3. **Choose Execution Approach:**
   - **Subagent-Driven (recommended):** Fresh subagent per task with review checkpoints
   - **Inline Execution:** Execute in current session with batch checkpoints

4. **Begin Implementation:** Start with Task 0 (Desktop Shell Setup)

---

## References

- **ADR-0004:** Glyphnova UI Design Decisions
- **Phase 2 Plan:** CLI Framework and Shared Backend API
- **Tauri Documentation:** https://tauri.app/
- **shadcn/ui:** https://ui.shadcn.com/
- **React Flow:** https://reactflow.dev/
- **Recharts:** https://recharts.org/
- **@dnd-kit/core:** https://dndkit.com/

---

**Document Version:** 1.0
**Last Updated:** 2025-04-06
**Plan Location:** `/home/jon/code/yaml-to-rust-agentsdk/opencode/docs/plans/03-glyphnova-ui/plan.md`
