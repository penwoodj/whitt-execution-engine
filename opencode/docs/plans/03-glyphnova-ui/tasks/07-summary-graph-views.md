# Task 07: Summary Graph Views

**Estimated Time:** 4 days
**Priority:** High - Visualizes workflow structure

**Goal:** Build workflow DAG (Directed Acyclic Graph) visualization using React Flow with zoom, pan, and node selection.

**ADR Compliance:**
- Graph reflects actual workflow structure
- No artificial layouts - use real dependencies
- Selection navigates to actual execution data

**Files:**
- Create: `agentsdk/glyphnova/src-frontend/components/visualization/WorkflowGraph.tsx`
- Create: `agentsdk/glyphnova/src-frontend/components/visualization/GraphControls.tsx`
- Create: `agentsdk/glyphnova/src-frontend/components/visualization/GraphNode.tsx`
- Create: `agentsdk/glyphnova/src-frontend/components/visualization/GraphEdge.tsx`

---

## Step 1: Install React Flow

- [ ] **Step 1.1: Install reactflow**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk/glyphnova
npm install reactflow
```

Expected: Package installs successfully

---

## Step 2: Create Graph Node Component

- [ ] **Step 2.1: Create GraphNode component**

```typescript
// agentsdk/glyphnova/src-frontend/components/visualization/GraphNode.tsx
import { memo } from 'react'
import { Handle, Position, NodeProps } from 'reactflow'
import type { WorkflowStep } from '../../types/api'

interface GraphNodeData {
  step: WorkflowStep
  status?: 'pending' | 'running' | 'completed' | 'failed'
}

export const GraphNode = memo(({ data, selected }: NodeProps<GraphNodeData>) => {
  const { step, status } = data

  const statusColors = {
    pending: 'bg-gray-600',
    running: 'bg-blue-600 animate-pulse',
    completed: 'bg-green-600',
    failed: 'bg-red-600',
  }

  const statusIcon = {
    pending: '○',
    running: '●',
    completed: '✓',
    failed: '✕',
  }

  return (
    <div
      className={`px-4 py-3 rounded-lg border-2 transition-all ${
        selected
          ? 'border-blue-500 bg-gray-700 shadow-lg'
          : 'border-gray-600 bg-gray-800 hover:border-gray-500'
      }`}
    >
      {/* Input Handle */}
      {step.dependencies && step.dependencies.length > 0 && (
        <Handle
          type="target"
          position={Position.Top}
          className="!bg-gray-400 !border-gray-500"
        />
      )}

      {/* Node Content */}
      <div className="flex items-start gap-3">
        {/* Status Icon */}
        {status && (
          <div
            className={`w-6 h-6 rounded-full flex items-center justify-center text-xs ${statusColors[status]}`}
          >
            {statusIcon[status]}
          </div>
        )}

        {/* Step Info */}
        <div className="flex-1 min-w-0">
          <h3 className="font-medium text-sm truncate">{step.name}</h3>
          <p className="text-xs text-gray-400">{step.agent}</p>
          {step.tools.length > 0 && (
            <div className="flex gap-1 mt-1">
              {step.tools.slice(0, 3).map((tool) => (
                <span key={tool} className="text-xs bg-gray-700 px-1.5 py-0.5 rounded">
                  {tool}
                </span>
              ))}
              {step.tools.length > 3 && (
                <span className="text-xs text-gray-500">+{step.tools.length - 3}</span>
              )}
            </div>
          )}
        </div>
      </div>

      {/* Output Handle */}
      <Handle
        type="source"
        position={Position.Bottom}
        className="!bg-gray-400 !border-gray-500"
      />
    </div>
  )
})

GraphNode.displayName = 'GraphNode'
```

---

## Step 3: Create Graph Edge Component

- [ ] **Step 3.1: Create GraphEdge component**

```typescript
// agentsdk/glyphnova/src-frontend/components/visualization/GraphEdge.tsx
import { memo } from 'react'
import {
  EdgeProps,
  getBezierPath,
  EdgeLabelRenderer,
  getMarkerEnd,
} from 'reactflow'
import type { WorkflowStepExecution } from '../../types/api'

interface GraphEdgeData {
  status?: 'pending' | 'running' | 'completed' | 'failed'
  execution?: WorkflowStepExecution
}

export const GraphEdge = memo(
  ({ id, sourceX, sourceY, targetX, targetY, sourcePosition, targetPosition, data, style }: EdgeProps<GraphEdgeData>) => {
    const [edgePath] = getBezierPath({
      sourceX,
      sourceY,
      sourcePosition,
      targetX,
      targetY,
      targetPosition,
    })

    const markerEnd = getMarkerEnd(style?.markerEnd || 'arrowclosed')

    const statusColors = {
      pending: '#6b7280',
      running: '#3b82f6',
      completed: '#22c55e',
      failed: '#ef4444',
    }

    return (
      <>
        <path
          id={id}
          d={edgePath}
          style={{
            stroke: data?.status ? statusColors[data.status] : '#4b5563',
            strokeWidth: 2,
            ...style,
          }}
          className="transition-all hover:stroke-gray-300"
          markerEnd={markerEnd}
        />

        {data?.status && (
          <EdgeLabelRenderer>
            <div
              style={{
                position: 'absolute',
                transform: `translate(-50%, -50%) translate(${(sourceX + targetX) / 2}px, ${(sourceY + targetY) / 2}px)`,
                pointerEvents: 'all',
              }}
              className="px-2 py-1 bg-gray-800 rounded text-xs"
            >
              {data.status}
            </div>
          </EdgeLabelRenderer>
        )}
      </>
    )
  }
)

GraphEdge.displayName = 'GraphEdge'
```

---

## Step 4: Create Graph Controls Component

- [ ] **Step 4.1: Create GraphControls component**

```typescript
// agentsdk/glyphnova/src-frontend/components/visualization/GraphControls.tsx
import { useCallback } from 'react'
import { useReactFlow } from 'reactflow'

export function GraphControls() {
  const { zoomIn, zoomOut, fitView } = useReactFlow()

  const handleZoomIn = useCallback(() => zoomIn({ duration: 300 }), [zoomIn])
  const handleZoomOut = useCallback(() => zoomOut({ duration: 300 }), [zoomOut])
  const handleFitView = useCallback(() => fitView({ duration: 300 }), [fitView])

  return (
    <div className="flex flex-col gap-2">
      <button
        onClick={handleZoomIn}
        className="p-2 bg-gray-800 hover:bg-gray-700 rounded-lg border border-gray-700 transition-colors"
        title="Zoom In (+)"
      >
        +
      </button>
      <button
        onClick={handleZoomOut}
        className="p-2 bg-gray-800 hover:bg-gray-700 rounded-lg border border-gray-700 transition-colors"
        title="Zoom Out (-)"
      >
        −
      </button>
      <button
        onClick={handleFitView}
        className="p-2 bg-gray-800 hover:bg-gray-700 rounded-lg border border-gray-700 transition-colors"
        title="Fit View (F)"
      >
        ⊙
      </button>
    </div>
  )
}
```

---

## Step 5: Create Workflow Graph Component

- [ ] **Step 5.1: Create WorkflowGraph component**

```typescript
// agentsdk/glyphnova/src-frontend/components/visualization/WorkflowGraph.tsx
import { useCallback, useMemo } from 'react'
import ReactFlow, {
  Background,
  Controls,
  MiniMap,
  Node,
  Edge,
  addEdge,
  Connection,
  useNodesState,
  useEdgesState,
  BackgroundVariant,
} from 'reactflow'
import 'reactflow/dist/style.css'
import { GraphNode } from './GraphNode'
import { GraphEdge } from './GraphEdge'
import { GraphControls } from './GraphControls'
import type { Workflow, WorkflowExecution } from '../../types/api'

interface WorkflowGraphProps {
  workflow: Workflow
  execution?: WorkflowExecution
  onNodeClick?: (stepId: string) => void
}

const nodeTypes = {
  workflowStep: GraphNode,
}

const edgeTypes = {
  workflowEdge: GraphEdge,
}

export function WorkflowGraph({ workflow, execution, onNodeClick }: WorkflowGraphProps) {
  // Convert workflow steps to nodes
  const initialNodes: Node[] = useMemo(() => {
    return workflow.steps.map((step, index) => {
      const stepExecution = execution?.steps.find((s) => s.step_id === step.id)

      return {
        id: step.id,
        type: 'workflowStep',
        position: { x: index * 300, y: 100 },
        data: {
          step,
          status: stepExecution?.status,
        },
      }
    })
  }, [workflow.steps, execution])

  // Convert dependencies to edges
  const initialEdges: Edge[] = useMemo(() => {
    const edges: Edge[] = []

    workflow.steps.forEach((step) => {
      if (step.dependencies) {
        step.dependencies.forEach((depId) => {
          const depStep = workflow.steps.find((s) => s.id === depId)
          if (depStep) {
            const depExecution = execution?.steps.find((s) => s.step_id === depId)
            const stepExecution = execution?.steps.find((s) => s.step_id === step.id)

            edges.push({
              id: `${depId}-${step.id}`,
              source: depId,
              target: step.id,
              type: 'workflowEdge',
              data: {
                status: stepExecution?.status,
                execution: stepExecution,
              },
              animated: depExecution?.status === 'running',
            })
          }
        })
      }
    })

    return edges
  }, [workflow.steps, execution])

  const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes)
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges)

  const onConnect = useCallback(
    (params: Connection) => setEdges((eds) => addEdge(params, eds)),
    [setEdges]
  )

  const onNodeClickHandler = useCallback(
    (_event: React.MouseEvent, node: Node) => {
      if (onNodeClick) {
        onNodeClick(node.id)
      }
    },
    [onNodeClick]
  )

  // Keyboard shortcuts
  React.useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === '+') {
        // Zoom in
      } else if (e.key === '-') {
        // Zoom out
      } else if (e.key === 'f' || e.key === 'F') {
        // Fit view
      }
    }

    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [])

  return (
    <div className="w-full h-full">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
        onNodeClick={onNodeClickHandler}
        nodeTypes={nodeTypes}
        edgeTypes={edgeTypes}
        fitView
        className="bg-gray-900"
      >
        <Background variant={BackgroundVariant.Dots} gap={12} size={1} />
        <Controls />
        <MiniMap
          nodeColor={(node) => {
            const status = node.data.status
            switch (status) {
              case 'running':
                return '#3b82f6'
              case 'completed':
                return '#22c55e'
              case 'failed':
                return '#ef4444'
              default:
                return '#6b7280'
            }
          }}
          className="!bg-gray-800 !border-gray-700"
        />
        <GraphControls />
      </ReactFlow>
    </div>
  )
}
```

---

## Step 6: Create Graph Layout Algorithm

- [ ] **Step 6.1: Create layout utilities**

```typescript
// agentsdk/glyphnova/src-frontend/utils/graph-layout.ts
import type { Node, Edge } from 'reactflow'
import type { Workflow } from '../types/api'

export interface LayoutNode {
  id: string
  level: number
  index: number
  dependencies: string[]
}

export function calculateLayout(workflow: Workflow): {
  nodes: Node[]
  edges: Edge[]
} {
  // Calculate node levels (topological sort)
  const levels = calculateNodeLevels(workflow.steps)

  // Position nodes based on levels
  const nodes: Node[] = workflow.steps.map((step, index) => {
    const level = levels.get(step.id) || 0
    const nodesAtLevel = Array.from(levels.values()).filter((l) => l === level).length

    return {
      id: step.id,
      type: 'workflowStep',
      position: {
        x: level * 300,
        y: (index % nodesAtLevel) * 150 + 100,
      },
      data: {
        step,
        status: 'pending',
      },
    }
  })

  // Create edges from dependencies
  const edges: Edge[] = []
  workflow.steps.forEach((step) => {
    if (step.dependencies) {
      step.dependencies.forEach((depId) => {
        edges.push({
          id: `${depId}-${step.id}`,
          source: depId,
          target: step.id,
          type: 'workflowEdge',
          data: { status: 'pending' },
        })
      })
    }
  })

  return { nodes, edges }
}

function calculateNodeLevels(steps: Array<{ id: string; dependencies?: string[] }>): Map<string, number> {
  const levels = new Map<string, number>()
  const visited = new Set<string>()

  function visit(stepId: string): number {
    if (visited.has(stepId)) {
      return levels.get(stepId) || 0
    }

    visited.add(stepId)

    const step = steps.find((s) => s.id === stepId)
    if (!step || !step.dependencies || step.dependencies.length === 0) {
      levels.set(stepId, 0)
      return 0
    }

    let maxDepLevel = 0
    step.dependencies.forEach((depId) => {
      const depLevel = visit(depId)
      maxDepLevel = Math.max(maxDepLevel, depLevel)
    })

    const level = maxDepLevel + 1
    levels.set(stepId, level)
    return level
  }

  steps.forEach((step) => visit(step.id))

  return levels
}
```

---

## Step 7: Add Graph to Main Content

- [ ] **Step 7.1: Update WorkflowView to include graph**

```typescript
// Update agentsdk/glyphnova/src-frontend/components/visualization/WorkflowView.tsx
import { WorkflowGraph } from './WorkflowGraph'

export function WorkflowView({ workflowId }: WorkflowViewProps) {
  const { data: workflow, isLoading, error } = useWorkflow(workflowId)

  if (isLoading) {
    return <div className="p-8 text-gray-400">Loading workflow...</div>
  }

  if (error) {
    return <div className="p-8 text-red-400">Error loading workflow: {error.message}</div>
  }

  if (!workflow) {
    return <div className="p-8 text-gray-400">Workflow not found</div>
  }

  return (
    <div className="h-full flex flex-col">
      {/* Header */}
      <div className="p-4 border-b border-gray-700">
        <h2 className="text-2xl font-bold">{workflow.name}</h2>
        {workflow.description && (
          <p className="text-sm text-gray-400 mt-1">{workflow.description}</p>
        )}
      </div>

      {/* Graph */}
      <div className="flex-1">
        <WorkflowGraph
          workflow={workflow}
          onNodeClick={(stepId) => {
            // Navigate to step view
            console.log('Navigate to step:', stepId)
          }}
        />
      </div>
    </div>
  )
}
```

---

## Step 8: Add Graph Metrics Panel

- [ ] **Step 8.1: Create GraphMetrics component**

```typescript
// agentsdk/glyphnova/src-frontend/components/visualization/GraphMetrics.tsx
import type { WorkflowExecution } from '../../types/api'

interface GraphMetricsProps {
  execution?: WorkflowExecution
}

export function GraphMetrics({ execution }: GraphMetricsProps) {
  if (!execution) {
    return null
  }

  const steps = execution.steps
  const total = steps.length
  const completed = steps.filter((s) => s.status === 'completed').length
  const running = steps.filter((s) => s.status === 'running').length
  const failed = steps.filter((s) => s.status === 'failed').length
  const pending = steps.filter((s) => s.status === 'pending').length

  return (
    <div className="p-4 bg-gray-800 border-t border-gray-700">
      <div className="flex gap-6 text-sm">
        <div>
          <span className="text-gray-400">Total:</span>
          <span className="ml-2 font-medium">{total}</span>
        </div>
        <div>
          <span className="text-green-400">Completed:</span>
          <span className="ml-2 font-medium">{completed}</span>
        </div>
        <div>
          <span className="text-blue-400">Running:</span>
          <span className="ml-2 font-medium">{running}</span>
        </div>
        <div>
          <span className="text-red-400">Failed:</span>
          <span className="ml-2 font-medium">{failed}</span>
        </div>
        <div>
          <span className="text-gray-500">Pending:</span>
          <span className="ml-2 font-medium">{pending}</span>
        </div>
      </div>
    </div>
  )
}
```

- [ ] **Step 8.2: Integrate into WorkflowView**

```typescript
// Update WorkflowView component
<div className="flex-1 flex flex-col">
  <WorkflowGraph workflow={workflow} />
  <GraphMetrics execution={execution} />
</div>
```

---

## Step 9: Test Workflow Graph

- [ ] **Step 9.1: Run Tauri dev server**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk/glyphnova
npm run tauri dev
```

- [ ] **Step 9.2: Verify graph displays**

Expected:
- Workflow DAG displays correctly
- Nodes show step names and agents
- Edges show dependencies

- [ ] **Step 9.3: Test graph controls**

Expected:
- Zoom in/out buttons work
- Fit view button works
- Keyboard shortcuts work (+, -, F)
- Mini map updates correctly

- [ ] **Step 9.4: Test node selection**

Expected:
- Clicking node highlights it
- Node navigation triggers
- Status colors display correctly

- [ ] **Step 9.5: Test animated edges**

Expected:
- Running steps have animated edges
- Edge colors match status
- Edge labels show status

---

## Step 10: Commit

- [ ] **Step 10.1: Stage and commit changes**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk
git add glyphnova/src-frontend/components/visualization/ glyphnova/src-frontend/utils/graph-layout.ts
git commit -m "feat(ui): implement workflow DAG visualization with React Flow

- Create workflow graph with nodes and edges
- Build custom GraphNode component with status indicators
- Implement GraphEdge with status colors and animations
- Add graph controls (zoom, fit view)
- Create graph layout algorithm for automatic positioning
- Add minimap and background grid
- Display graph metrics summary

ADR Compliance:
- Graph reflects actual workflow structure
- No artificial layouts - use real dependencies
- Selection navigates to actual execution data

Task: 07-summary-graph-views
Part of: Phase 3 - Glyphnova UI"
```

---

## Success Criteria

- [ ] Workflow DAG displays correctly
- [ ] Nodes show step information
- [ ] Edges show dependencies
- [ ] Zoom and pan work smoothly
- [ ] Node selection triggers navigation
- [ ] Status colors are accurate
- [ ] Animated edges for running steps
- [ ] Minimap updates correctly

---

## Next Steps

After completing this task, proceed to **Task 08: Quality Metrics Dashboard** to build metrics visualization.
