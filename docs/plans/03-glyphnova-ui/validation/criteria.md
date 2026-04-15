# Validation Criteria for Phase 3: Glyphnova UI

This document defines the validation criteria for the Glyphnova UI implementation. All criteria must be satisfied for the implementation to be considered complete.

---

## Functional Requirements

### Queue Visualization

- [ ] **Queue Display**: Queue displays in left sidebar with all items visible
- [ ] **Real-time Updates**: Queue updates reflect within 2 seconds of scheduler state change
- [ **Item Selection**: Queue items can be selected by clicking
- [ ] **Status Indicators**: Queue items display correct status (pending/running/completed/failed/cancelled)
- [ ] **Filtering**: Queue can be filtered by status (all/pending/running/completed/failed/cancelled)
- [ ] **Search**: Queue can be searched by workflow name, step name, or ID
- [ ] **Sorting**: Queue can be sorted by creation time or priority
- [ ] **Statistics**: Queue summary displays total, running, completed, and failed counts
- [ ] **Empty State**: Empty state displays when queue has no items or no items match filters
- [ ] **Error Display**: Failed items show error messages
- [ ] **Keyboard Navigation**: Arrow keys navigate queue items, Escape clears selection

### Scope Indicators

- [ ] **Header Display**: Header always displays current scope breadcrumbs
- [ ] **Breadcrumb Navigation**: Breadcrumbs show full scope path (workspace → project → workflow → step)
- [ ] **Scope Change Confirmation**: Changing scope shows confirmation dialog
- [ ] **Change Logging**: All scope changes are logged with timestamps
- [ ] **History View**: Scope change history is accessible
- [ ] **Context Indicators**: Current scope level is visually indicated (color/icon)
- [ ] **Connection Status**: Runtime connection status displays in header
- [ ] **Quick Actions**: Context-specific quick actions appear for current scope
- [ ] **Keyboard Shortcuts**: Ctrl+Shift+B navigates back in history, Escape clears selection

### Multi-Zoom Navigation

- [ ] **Zoom Controls**: Zoom level buttons display and work (workflow/step/agent/tool)
- [ ] **Navigation State Machine**: Back/Forward/Up buttons function correctly
- [ ] **State History**: Navigation history is maintained and can be traversed
- [ ] **Zoom Views**: Each zoom level displays appropriate content
  - [ ] Workflow view: Shows workflow DAG
  - [ ] Step view: Shows step execution details
  - [ ] Agent view: Shows agent execution details
  - [ ] Tool view: Shows tool execution details
- [ ] **Keyboard Shortcuts**: Ctrl+Arrows for navigation, 1-4 for zoom levels
- [ ] **Zoom Transitions**: Smooth animations when changing zoom levels
- [ ] **Parent Navigation**: Up button navigates to parent scope
- [ ] **Current Level Display**: Current zoom level is clearly indicated

### Drag-Drop Reprioritization

- [ ] **Drag Handles**: Queue items have visible drag handles
- [ ] **Drop Zones**: Drop zones appear when dragging items
- [ ] **Reordering**: Items can be reordered by dragging
- [ ] **API Calls**: Drag-and-drop calls scheduler API to reorder queue
- [ ] **Validation**: Reorder only succeeds when scheduler validates
- [ ] **Error Handling**: If API call fails, UI reverts to original order
- [ ] **Undo/Redo**: Queue reordering can be undone and redone
- [ ] **Undo Controls**: Undo/redo buttons work
- [ ] **Undo Shortcuts**: Ctrl+Z and Ctrl+Y work for undo/redo
- [ ] **Drag Visual Feedback**: Drag overlay provides visual feedback during drag

### Artifact Browser

- [ ] **Artifact List**: Artifacts display in list view
- [ ] **File Icons**: Icons display based on artifact type
- [ ] **Metadata**: Artifact size, type, and creation date display
- [ ] **Selection**: Artifacts can be selected by clicking
- [ ] **Search**: Artifacts can be searched by name
- [ ] **Type Filter**: Artifacts can be filtered by type
- [ ] **Date Filter**: Artifacts can be filtered by date range
- [ ] **Preview**: Artifact preview displays for supported types
  - [ ] Text artifacts: Show content
  - [ ] Images: Show thumbnail
  - [ ] PDF: Show download button
- [ ] **Details Panel**: Selected artifact shows full details
- [ ] **Download**: Download button initiates artifact download
- [ ] **Export**: Export options work (JSON/CSV/TXT)
- [ ] **Version History**: Artifact versions can be navigated

### Summary Graph Views

- [ ] **DAG Display**: Workflow DAG displays correctly with nodes and edges
- [ ] **Node Information**: Nodes show step name, agent, and tools
- [ ] **Edge Display**: Edges show dependencies between steps
- [ ] **Status Colors**: Node and edge colors reflect execution status
- [ ] **Animated Edges**: Running steps have animated edges
- [ ] **Zoom/Pan**: Graph can be zoomed and panned
- [ ] **Zoom Controls**: Zoom in/out and fit view buttons work
- [ ] **Keyboard Shortcuts**: + and - for zoom, F for fit view
- [ ] **Node Selection**: Clicking node highlights it
- [ ] **Node Navigation**: Clicking node navigates to step view
- [ ] **Minimap**: Minimap displays and updates correctly
- [ ] **Background Grid**: Background grid provides visual reference
- [ ] **Graph Metrics**: Metrics summary displays below graph

### Quality Metrics Dashboard

- [ ] **Dashboard Display**: Dashboard displays with all components
- [ ] **Time Range Filter**: Time range selector works (1h/6h/24h/7d)
- [ ] **Summary Cards**: Key metrics display in summary cards
  - [ ] Total Workflows
  - [ ] Success Rate
  - [ ] Average Duration
  - [ ] Total Artifacts
- [ ] **Charts**: All charts render correctly
  - [ ] Success Rate Chart
  - [ ] Duration Chart
  - [ ] Error Rate Chart
  - [ ] Throughput Chart
- [ ] **Chart Updates**: Charts update when time range changes
- [ ] **Alert Panel**: Alerts display with type and message
- [ ] **Alert Filtering**: Alerts can be filtered (all/unresolved)
- [ ] **Alert Dismissal**: Alerts can be dismissed
- [ ] **Benchmark Results**: Benchmarks display with status
- [ ] **Benchmark Progress**: Progress bars show benchmark status
- [ ] **Benchmark Status**: Color coding indicates pass/warning/fail

---

## ADR-0004 Compliance

### UI is a Shell, Not a Separate System

- [ ] **No Business Logic in UI**: UI does not contain business logic
- [ ] **State from API**: All state comes from runtime API calls
- [ ] **No Local Database**: UI does not have its own database
- [ ] **Breaks on Backend Down**: UI breaks if backend is down (expected behavior)
- [ ] **No Caching Beyond React Query**: No local caching beyond what React Query does

### Queue is Direct Projection of Scheduler State

- [ ] **Order Matches Scheduler**: Queue order matches scheduler state exactly
- [ ] **Status Updates Reflect**: Status changes reflect immediately
- [ ] **No Local Queue**: No local queue array maintained
- [ ] **WebSocket/Polling**: Updates come from WebSocket or polling
- [ ] **No Optimistic Queue**: No optimistic UI for queue state

### Scope is Always Visible in Header

- [ ] **Header Always Shows**: Header always shows current scope
- [ ] **Breadcrumbs Display**: Breadcrumbs display full scope path
- [ ] **No Hidden Actions**: No actions change scope without confirmation
- [ ] **Context Clear**: User always knows which context they're in

### Drag-and-Drop is Scheduler API Projection

- [ ] **Calls Scheduler API**: Dragging calls scheduler API
- [ ] **No Direct Modification**: Dragging doesn't directly modify data
- [ ] **Validates with Scheduler**: Reorder only succeeds when scheduler validates
- [ ] **Error Reverts**: Errors revert UI to original state
- [ ] **No Optimistic Updates**: No optimistic UI for drag-and-drop

### No Separate Backend

- [ ] **Single Backend**: No separate UI backend service
- [ ] **Uses Runtime API**: Uses same REST and WebSocket APIs as CLI
- [ ] **Thin Tauri Wrapper**: Tauri backend is a thin wrapper
- [ ] **No Extra Database**: No extra database or storage

---

## Technical Requirements

### Build and Deployment

- [ ] **Tauri Builds for All Platforms**: Tauri builds for Linux, macOS, and Windows
- [ ] **Production Build**: Production build creates executable
- [ ] **Hot Reload**: Hot reload works in development mode
- [ ] **Linting Passes**: ESLint passes without errors
- [ ] **TypeScript Compiles**: TypeScript compiles without errors

### API Integration

- [ ] **REST API Calls Work**: All REST API calls work correctly
- [ ] **WebSocket Connected**: WebSocket connection established
- [ ] **WebSocket Updates**: WebSocket receives real-time updates
- [ ] **React Query Works**: React Query hooks fetch and cache data
- [ ] **Error Handling**: User-friendly error messages display
- [ ] **Loading States**: Loading states show during async operations

### Performance

- [ ] **Queue Updates in 2s**: Queue updates reflect within 2 seconds
- [ ] **DAG Renders Smoothly**: DAG graphs render smoothly (100+ nodes)
- [ ] **Dashboard No Lag**: Metrics dashboard updates without UI lag
- [ ] **WebSocket Reconnects**: WebSocket auto-reconnects on failure
- [ ] **Large Queues Handle**: UI handles queues with 100+ items

### Code Quality

- [ ] **Components Are Reusable**: Components are small and focused
- [ ] **Types Are Defined**: TypeScript types defined for all data
- [ ] **No Console Errors**: No console errors during normal operation
- [ ] **Memory Leaks Absent**: No memory leaks (check with React DevTools)
- [ ] **Accessibility**: Follows WCAG 2.1 AA guidelines

---

## Testing Requirements

### Unit Tests

- [ ] **Component Tests**: All components have unit tests
- [ ] **Hook Tests**: All custom hooks have tests
- [ ] **Utility Tests**: All utility functions have tests
- [ ] **Coverage**: Test coverage > 80%

### Integration Tests

- [ ] **API Integration**: API calls tested with mock server
- [ ] **WebSocket Tests**: WebSocket connection tested
- [ ] **State Management**: Zustand store tested

### E2E Tests

- [ ] **Critical Workflows**: Critical user workflows have E2E tests
- [ ] **Queue Operations**: Queue operations tested end-to-end
- [ ] **Navigation**: Navigation tested end-to-end
- [ ] **Drag-and-Drop**: Drag-and-drop tested end-to-end

### Mock Strategies

- [ ] **Mock WebSocket**: Mock WebSocket server for testing
- [ ] **Mock Tauri Backend**: Mock Tauri commands for testing
- [ ] **Mock Scheduler State**: Mock scheduler state for testing
- [ ] **Test Data**: Comprehensive test data sets available

See `tests/mock-strategies.md` for detailed mock strategies.

---

## User Experience Requirements

### Usability

- [ ] **Intuitive Interface**: Interface is intuitive for new users
- [ ] **Consistent Design**: Design is consistent across all components
- [ ] **Clear Feedback**: User actions provide clear feedback
- [ ] **Error Messages**: Error messages are user-friendly and actionable
- [ ] **Loading Indicators**: Loading indicators show during async operations

### Accessibility

- [ ] **Keyboard Navigation**: All features accessible via keyboard
- [ ] **Screen Reader Support**: Screen reader support works
- [ ] **Color Contrast**: Color contrast meets WCAG AA standards
- [ ] **Focus Indicators**: Focus indicators are visible
- [ ] **Text Alternatives**: Images have text alternatives

### Responsiveness

- [ ] **Window Resizing**: UI works at different window sizes
- [ ] **Small Screens**: UI is usable on small screens
- [ ] **Large Screens**: UI takes advantage of large screens
- [ ] **Touch Support**: Touch gestures work where appropriate

---

## Documentation Requirements

- [ ] **README Complete**: README is complete and up-to-date
- [ ] **API Documentation**: API endpoints documented
- [ ] **Component Documentation**: Components have JSDoc comments
- [ ] **Architecture Document**: Architecture documented
- [ ] **User Guide**: User guide for common tasks

---

## Security Requirements

- [ ] **Input Validation**: All user inputs are validated
- [ ] **XSS Prevention**: XSS vulnerabilities prevented
- [ ] **CSRF Protection**: CSRF protection in place
- [ ] **Secure Storage**: Sensitive data stored securely
- [ ] **No Secrets in Code**: No hardcoded secrets in code

---

## Validation Checklist

Use this checklist to validate the implementation:

### Phase 1: Setup
- [ ] Tauri project initialized
- [ ] React + TypeScript configured
- [ ] Hot reload working
- [ ] Production build successful

### Phase 2: API Integration
- [ ] REST API calls work
- [ ] WebSocket connected
- [ ] Real-time updates work
- [ ] Error handling in place

### Phase 3: Queue
- [ ] Queue displays
- [ ] Real-time updates work
- [ ] Filters work
- [ ] Drag-and-drop works

### Phase 4: Navigation
- [ ] Scope indicators display
- [ ] Multi-zoom works
- [ ] Keyboard shortcuts work
- [ ] Navigation history works

### Phase 5: Artifacts
- [ ] Artifact browser works
- [ ] Preview works
- [ ] Download works
- [ ] Export works

### Phase 6: Visualization
- [ ] DAG graph displays
- [ ] Graph controls work
- [ ] Metrics dashboard works
- [ ] Charts render correctly

### Phase 7: Testing
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] E2E tests pass
- [ ] Coverage > 80%

### Phase 8: Documentation
- [ ] README complete
- [ ] API documented
- [ ] Components documented
- [ ] User guide complete

---

## Exit Criteria

Implementation is complete when:

1. **All functional requirements** are satisfied
2. **All ADR-0004 compliance** items are met
3. **All technical requirements** are met
4. **All testing requirements** are satisfied
5. **All UX requirements** are met
6. **All documentation requirements** are satisfied
7. **All security requirements** are met
8. **Validation checklist** is complete

---

**Document Version:** 1.0
**Last Updated:** 2025-04-06
**Plan Location:** `/home/jon/code/yaml-to-rust-agentsdk/opencode/docs/plans/03-glyphnova-ui/validation/criteria.md`
