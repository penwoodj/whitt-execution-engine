# Mock Strategies for Testing

This document defines mock strategies for testing the Glyphnova UI without requiring a running AgentSDK runtime.

---

## Overview

Testing Glyphnova UI requires mocking:
1. Tauri backend commands
2. WebSocket server
3. Runtime API endpoints
4. Scheduler state

These mocks enable:
- Unit testing of components
- Integration testing of hooks
- E2E testing of workflows
- Development without full runtime

---

## Mock Tauri Backend

### Strategy: MSW (Mock Service Worker)

Use MSW to mock Tauri commands during testing.

#### Setup

```typescript
// tests/mocks/handlers/tauri.ts
import { rest } from 'msw'
import type { TauriApiResponse, TauriApiRequest } from '../../src-frontend/types/api'

export const tauriHandlers = [
  // Mock api_request command
  rest.post('/tauri/api_request', async (req, res, ctx) => {
    const body: TauriApiRequest = await req.json()

    // Handle different API endpoints
    if (body.path === '/api/queue') {
      return res(
        ctx.status(200),
        ctx.json({
          status: 200,
          body: JSON.stringify(mockQueueState),
          headers: {},
        } as TauriApiResponse)
      )
    }

    if (body.path.startsWith('/api/workflows/')) {
      const workflowId = body.path.split('/').pop()
      const workflow = mockWorkflows.find((w) => w.id === workflowId)
      return res(
        ctx.status(200),
        ctx.json({
          status: 200,
          body: JSON.stringify(workflow),
          headers: {},
        } as TauriApiResponse)
      )
    }

    // Default response
    return res(
      ctx.status(404),
      ctx.json({
        status: 404,
        body: JSON.stringify({ error: 'Not found' }),
        headers: {},
      } as TauriApiResponse)
    )
  }),

  // Mock get_runtime_status command
  rest.get('/tauri/get_runtime_status', (req, res, ctx) => {
    return res(
      ctx.status(200),
      ctx.json({ status: 'connected' })
    )
  }),

  // Mock get_websocket_url command
  rest.get('/tauri/get_websocket_url', (req, res, ctx) => {
    return res(
      ctx.status(200),
      ctx.json('ws://localhost:8080/ws')
    )
  }),
]
```

#### Usage in Tests

```typescript
// tests/unit/QueueList.test.tsx
import { render, screen } from '@testing-library/react'
import { setupServer } from 'msw/node'
import { tauriHandlers } from '../mocks/handlers/tauri'
import { QueueList } from '../../src-frontend/components/queue/QueueList'

const server = setupServer(...tauriHandlers)

beforeAll(() => server.listen())
afterEach(() => server.resetHandlers())
afterAll(() => server.close())

describe('QueueList', () => {
  it('displays queue items', () => {
    render(<QueueList />)
    expect(screen.getByText('Queue')).toBeInTheDocument()
  })
})
```

---

## Mock WebSocket Server

### Strategy: Fake WebSocket Class

Create a fake WebSocket class for testing WebSocket updates.

#### Setup

```typescript
// tests/mocks/fake-websocket.ts
export class FakeWebSocket {
  static CONNECTING = 0
  static OPEN = 1
  static CLOSING = 2
  static CLOSED = 3

  readyState = FakeWebSocket.OPEN
  onopen: ((event: Event) => void) | null = null
  onmessage: ((event: MessageEvent) => void) | null = null
  onclose: ((event: CloseEvent) => void) | null = null
  onerror: ((event: Event) => void) | null = null

  private messageQueue: string[] = []

  constructor(url: string) {
    // Simulate connection delay
    setTimeout(() => {
      this.onopen?.(new Event('open'))
    }, 100)
  }

  send(data: string) {
    // Queue messages for testing
    this.messageQueue.push(data)
  }

  close() {
    this.readyState = FakeWebSocket.CLOSED
    this.onclose?.(new CloseEvent('close'))
  }

  // Test helper: Simulate receiving a message
  simulateMessage(message: string) {
    this.onmessage?.(new MessageEvent('message', { data: message }))
  }

  // Test helper: Get sent messages
  getSentMessages(): string[] {
    return [...this.messageQueue]
  }
}

// Install fake WebSocket in global scope
global.WebSocket = FakeWebSocket as any
```

#### Usage in Tests

```typescript
// tests/unit/useWebSocket.test.tsx
import { renderHook, act } from '@testing-library/react'
import { useWebSocket } from '../../src-frontend/hooks/useWebSocket'
import { FakeWebSocket } from '../mocks/fake-websocket'

describe('useWebSocket', () => {
  it('receives queue update messages', () => {
    const { result } = renderHook(() => useWebSocket())

    act(() => {
      // Simulate WebSocket message
      const ws = result.current as any
      ws.simulateMessage(JSON.stringify({
        type: 'queue:update',
        data: mockQueueState,
      }))
    })

    // Verify state updated
    expect(result.current.event).toEqual({
      type: 'queue:update',
      data: mockQueueState,
    })
  })
})
```

---

## Mock Scheduler State

### Strategy: Mock Data Objects

Create mock data objects that represent realistic scheduler state.

#### Queue State

```typescript
// tests/mocks/data/queue.ts
import type { QueueState, QueueItem } from '../../src-frontend/types/api'

export const mockQueueItems: QueueItem[] = [
  {
    id: 'queue-001',
    workflow_id: 'workflow-001',
    workflow_name: 'Process Document',
    step_name: 'Extract Text',
    status: 'running',
    priority: 1,
    created_at: '2025-04-06T10:00:00Z',
    started_at: '2025-04-06T10:01:00Z',
  },
  {
    id: 'queue-002',
    workflow_id: 'workflow-001',
    workflow_name: 'Process Document',
    step_name: 'Summarize Content',
    status: 'pending',
    priority: 2,
    created_at: '2025-04-06T10:02:00Z',
  },
  {
    id: 'queue-003',
    workflow_id: 'workflow-002',
    workflow_name: 'Generate Report',
    step_name: 'Create Charts',
    status: 'completed',
    priority: 1,
    created_at: '2025-04-06T09:00:00Z',
    started_at: '2025-04-06T09:01:00Z',
    completed_at: '2025-04-06T09:05:00Z',
  },
]

export const mockQueueState: QueueState = {
  items: mockQueueItems,
  total: mockQueueItems.length,
  pending: mockQueueItems.filter((i) => i.status === 'pending').length,
  running: mockQueueItems.filter((i) => i.status === 'running').length,
  completed: mockQueueItems.filter((i) => i.status === 'completed').length,
  failed: mockQueueItems.filter((i) => i.status === 'failed').length,
}
```

#### Workflow Data

```typescript
// tests/mocks/data/workflows.ts
import type { Workflow, WorkflowExecution, WorkflowStep } from '../../src-frontend/types/api'

export const mockWorkflowSteps: WorkflowStep[] = [
  {
    id: 'step-001',
    name: 'Extract Text',
    agent: 'text-extractor',
    tools: ['pdf-parser', 'ocr-engine'],
    inputs: { document_path: '/data/doc.pdf' },
    dependencies: [],
  },
  {
    id: 'step-002',
    name: 'Summarize Content',
    agent: 'summarizer',
    tools: ['nlp-processor'],
    inputs: {},
    dependencies: ['step-001'],
  },
  {
    id: 'step-003',
    name: 'Generate Report',
    agent: 'report-generator',
    tools: ['doc-writer', 'chart-builder'],
    inputs: {},
    dependencies: ['step-002'],
  },
]

export const mockWorkflow: Workflow = {
  id: 'workflow-001',
  name: 'Process Document',
  description: 'Extract text from PDF and generate summary report',
  steps: mockWorkflowSteps,
  created_at: '2025-04-06T08:00:00Z',
  updated_at: '2025-04-06T08:00:00Z',
}

export const mockWorkflowExecution: WorkflowExecution = {
  id: 'exec-001',
  workflow_id: 'workflow-001',
  status: 'running',
  created_at: '2025-04-06T10:00:00Z',
  started_at: '2025-04-06T10:01:00Z',
  steps: [
    {
      step_id: 'step-001',
      status: 'completed',
      started_at: '2025-04-06T10:01:00Z',
      completed_at: '2025-04-06T10:02:00Z',
    },
    {
      step_id: 'step-002',
      status: 'running',
      started_at: '2025-04-06T10:02:00Z',
    },
    {
      step_id: 'step-003',
      status: 'pending',
    },
  ],
}
```

#### Artifact Data

```typescript
// tests/mocks/data/artifacts.ts
import type { Artifact } from '../../src-frontend/types/api'

export const mockArtifacts: Artifact[] = [
  {
    id: 'artifact-001',
    name: 'extracted-text.txt',
    type: 'text/plain',
    size: 1024,
    created_at: '2025-04-06T10:02:00Z',
    workflow_execution_id: 'exec-001',
    step_execution_id: 'step-001',
    content: 'This is the extracted text content...',
  },
  {
    id: 'artifact-002',
    name: 'summary.md',
    type: 'text/markdown',
    size: 512,
    created_at: '2025-04-06T10:03:00Z',
    workflow_execution_id: 'exec-001',
    step_execution_id: 'step-002',
    content: '# Summary\n\nThis is the summary...',
  },
  {
    id: 'artifact-003',
    name: 'report.pdf',
    type: 'application/pdf',
    size: 2048,
    created_at: '2025-04-06T10:04:00Z',
    workflow_execution_id: 'exec-001',
    step_execution_id: 'step-003',
  },
]
```

#### Metrics Data

```typescript
// tests/mocks/data/metrics.ts
import type { QualityMetrics, MetricsSummary } from '../../src-frontend/types/api'

export const mockQualityMetrics: QualityMetrics[] = Array.from({ length: 24 }, (_, i) => ({
  timestamp: new Date(Date.now() - i * 3600000).toISOString(),
  success_rate: 0.92 + Math.random() * 0.08,
  average_duration_ms: 30000 + Math.random() * 10000,
  error_rate: 0.01 + Math.random() * 0.05,
  throughput: 10 + Math.random() * 5,
}))

export const mockMetricsSummary: MetricsSummary = {
  total_workflows: 100,
  successful_workflows: 92,
  failed_workflows: 8,
  average_duration_ms: 32000,
  total_artifacts: 250,
  queue_size: 5,
}
```

---

## Mock Runtime API

### Strategy: Mock Server with Express

Create a mock Express server that simulates the runtime API.

#### Setup

```typescript
// tests/mocks/server/runtime-api.ts
import express from 'express'
import { mockQueueState, mockWorkflows, mockArtifacts, mockMetricsSummary } from '../data'

const app = express()
app.use(express.json())

// Queue endpoints
app.get('/api/queue', (req, res) => {
  res.json({ data: mockQueueState, status: 'success' })
})

app.post('/api/queue/reorder', (req, res) => {
  const { item_id, new_position } = req.body
  // Simulate reordering
  const itemIndex = mockQueueState.items.findIndex((i) => i.id === item_id)
  if (itemIndex !== -1) {
    const [item] = mockQueueState.items.splice(itemIndex, 1)
    mockQueueState.items.splice(new_position, 0, item)
  }
  res.json({ data: { success: true }, status: 'success' })
})

// Workflow endpoints
app.get('/api/workflows/:id', (req, res) => {
  const workflow = mockWorkflows.find((w) => w.id === req.params.id)
  if (workflow) {
    res.json({ data: workflow, status: 'success' })
  } else {
    res.status(404).json({ error: 'Workflow not found' })
  }
})

app.get('/api/workflows', (req, res) => {
  res.json({ data: mockWorkflows, status: 'success' })
})

// Artifact endpoints
app.get('/api/artifacts', (req, res) => {
  res.json({ data: mockArtifacts, status: 'success' })
})

app.get('/api/artifacts/:id', (req, res) => {
  const artifact = mockArtifacts.find((a) => a.id === req.params.id)
  if (artifact) {
    res.json({ data: artifact, status: 'success' })
  } else {
    res.status(404).json({ error: 'Artifact not found' })
  }
})

// Metrics endpoints
app.get('/api/metrics/summary', (req, res) => {
  res.json({ data: mockMetricsSummary, status: 'success' })
})

app.get('/api/metrics/quality', (req, res) => {
  const range = req.query.range || '1h'
  // Return metrics based on range
  res.json({ data: mockQualityMetrics, status: 'success' })
})

// Health endpoint
app.get('/health', (req, res) => {
  res.json({ status: 'healthy' })
})

export function startMockServer(port = 8081) {
  return app.listen(port, () => {
    console.log(`Mock runtime API running on port ${port}`)
  })
}
```

#### Usage in E2E Tests

```typescript
// tests/e2e/queue-flow.test.ts
import { startMockServer } from '../mocks/server/runtime-api'
import { test, expect } from '@playwright/test'

test.beforeAll(async () => {
  startMockServer(8081)
})

test.describe('Queue Flow', () => {
  test('displays queue items', async ({ page }) => {
    await page.goto('http://localhost:5173')
    await expect(page.getByText('Queue')).toBeVisible()
    await expect(page.getByText('Process Document')).toBeVisible()
  })

  test('filters queue items', async ({ page }) => {
    await page.goto('http://localhost:5173')
    await page.click('button:has-text("Running")')
    await expect(page.getByText('Extract Text')).toBeVisible()
    await expect(page.getByText('Summarize Content')).not.toBeVisible()
  })
})
```

---

## Mock WebSocket Events

### Strategy: Event Generator

Create functions that generate realistic WebSocket events.

#### Setup

```typescript
// tests/mocks/websocket/events.ts
import type { WebSocketEvent, QueueUpdateEvent, AgentProgressEvent } from '../../src-frontend/types/api'
import { mockQueueState } from '../data/queue'

export function createQueueUpdateEvent(overrides = {}): QueueUpdateEvent {
  return {
    type: 'queue:update',
    data: {
      ...mockQueueState,
      ...overrides,
    },
  }
}

export function createAgentProgressEvent(
  agentExecutionId: string,
  progress: number,
  message?: string
): AgentProgressEvent {
  return {
    type: 'agent:progress',
    data: {
      agent_execution_id,
      progress,
      message,
    },
  }
}

export function createToolCompleteEvent(toolExecutionId: string, success: boolean) {
  return {
    type: 'tool:complete',
    data: {
      id: toolExecutionId,
      tool_id: 'test-tool',
      inputs: {},
      status: success ? 'completed' : 'failed',
      completed_at: new Date().toISOString(),
      outputs: success ? { result: 'done' } : undefined,
      error: success ? undefined : 'Test error',
    },
  }
}

export function createWorkflowCompleteEvent(executionId: string) {
  return {
    type: 'workflow:complete',
    data: {
      id: executionId,
      workflow_id: 'workflow-001',
      status: 'completed',
      created_at: new Date().toISOString(),
      started_at: new Date(Date.now() - 60000).toISOString(),
      completed_at: new Date().toISOString(),
      steps: [],
    },
  }
}
```

#### Usage in Tests

```typescript
// tests/unit/QueueList.test.tsx
import { createQueueUpdateEvent } from '../mocks/websocket/events'

describe('QueueList', () => {
  it('updates queue on WebSocket event', async () => {
    const { result } = renderHook(() => useQueue())

    act(() => {
      // Simulate WebSocket event
      const event = createQueueUpdateEvent({
        total: mockQueueState.total + 1,
      })
      // Update mock data
      mockQueueState.total += 1
      // Trigger event
      // ...
    })

    // Verify update
    expect(result.current.data?.total).toBe(mockQueueState.total)
  })
})
```

---

## Test Scenarios

### Unit Test Scenarios

#### Component Tests

- [ ] **QueueItem renders correctly**
- [ ] **QueueFilters apply correctly**
- [ ] **BreadcrumbNav displays path**
- [ ] **ZoomControls switch levels**
- [ ] **ArtifactListItem displays metadata**

#### Hook Tests

- [ ] **useQueue fetches and caches data**
- [ ] **useWebSocket connects and receives events**
- [ ] **useNavigation maintains history**
- [ ] **useAppStore updates state**

### Integration Test Scenarios

- [ ] **Queue list fetches from API and displays**
- [ ] **WebSocket updates trigger queue refresh**
- [ ] **Drag-and-drop calls reorder API**
- [ ] **Scope changes navigate correctly**

### E2E Test Scenarios

- [ ] **Complete workflow execution flow**
  1. Add workflow to queue
  2. Monitor execution progress
  3. View artifacts
  4. Check metrics

- [ ] **Queue management flow**
  1. View queue
  2. Filter by status
  3. Reorder items
  4. Cancel execution

- [ ] **Navigation flow**
  1. Navigate between zoom levels
  2. Use breadcrumbs
  3. Use keyboard shortcuts

---

## Testing Best Practices

1. **Isolation**: Each test should be independent
2. **Clear Setup**: Use `beforeAll` and `beforeEach` for setup
3. **Tear Down**: Clean up mocks after tests
4. **Realistic Data**: Use mock data that resembles production
5. **Edge Cases**: Test error states and edge cases
6. **Assertions**: Use clear, specific assertions
7. **Test Names**: Use descriptive test names

---

## Continuous Integration

### GitHub Actions Example

```yaml
# .github/workflows/ui-tests.yml
name: UI Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: '18'

      - name: Install dependencies
        run: cd glyphnova && npm install

      - name: Run unit tests
        run: cd glyphnova && npm run test:unit

      - name: Run integration tests
        run: cd glyphnova && npm run test:integration

      - name: Run E2E tests
        run: cd glyphnova && npm run test:e2e

      - name: Upload coverage
        uses: codecov/codecov-action@v3
        with:
          files: ./coverage/lcov.info
```

---

**Document Version:** 1.0
**Last Updated:** 2025-04-06
**Plan Location:** `/home/jon/code/yaml-to-rust-agentsdk/opencode/docs/plans/03-glyphnova-ui/tests/mock-strategies.md`
