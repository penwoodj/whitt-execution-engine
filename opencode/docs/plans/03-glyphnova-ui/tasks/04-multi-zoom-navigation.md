# Task 04: Multi-Zoom Navigation

**Estimated Time:** 4 days
**Priority:** High - Enables drill-down into execution details

**Goal:** Implement abstraction level navigation (workflow → step → agent → tool) with zoom controls, state machine, and keyboard shortcuts.

**ADR Compliance:**
- Navigation reflects actual execution structure
- Zoom levels map to runtime hierarchy
- No artificial levels or views

**Files:**
- Create: `agentsdk/glyphnova/src-frontend/components/navigation/ZoomControls.tsx`
- Create: `agentsdk/glyphnova/src-frontend/components/navigation/NavigationStateMachine.tsx`
- Create: `agentsdk/glyphnova/src-frontend/hooks/useNavigation.tsx`
- Modify: `agentsdk/glyphnova/src-frontend/stores/appStore.ts`

---

## Step 1: Define Zoom Levels

- [ ] **Step 1.1: Create zoom level types**

```typescript
// Add to agentsdk/glyphnova/src-frontend/types/api.ts

export type ZoomLevel = 'workflow' | 'step' | 'agent' | 'tool'

export interface ZoomState {
  level: ZoomLevel
  id?: string
  name?: string
  parent?: ZoomState
}

export interface NavigationState {
  current: ZoomState | null
  history: ZoomState[]
  canGoBack: boolean
  canGoForward: boolean
}
```

---

## Step 2: Create Navigation Hook

- [ ] **Step 2.1: Create useNavigation hook**

```typescript
// agentsdk/glyphnova/src-frontend/hooks/useNavigation.tsx
import { useState, useCallback } from 'react'
import type { ZoomLevel, ZoomState, NavigationState } from '../types/api'

export function useNavigation() {
  const [navigation, setNavigation] = useState<NavigationState>({
    current: null,
    history: [],
    canGoBack: false,
    canGoForward: false,
  })

  const [historyIndex, setHistoryIndex] = useState(-1)
  const [history, setHistory] = useState<ZoomState[]>([])

  const navigateTo = useCallback((level: ZoomLevel, id: string, name?: string) => {
    const newState: ZoomState = { level, id, name }

    // Add to history if navigating to new location
    const newHistory = history.slice(0, historyIndex + 1)
    newHistory.push(newState)

    setHistory(newHistory)
    setHistoryIndex(newHistory.length - 1)

    setNavigation({
      current: newState,
      history: newHistory,
      canGoBack: historyIndex > 0,
      canGoForward: false,
    })
  }, [history, historyIndex])

  const goBack = useCallback(() => {
    if (historyIndex > 0) {
      const newIndex = historyIndex - 1
      setHistoryIndex(newIndex)
      setNavigation({
        current: history[newIndex],
        history: history.slice(0, newIndex + 1),
        canGoBack: newIndex > 0,
        canGoForward: newIndex < history.length - 1,
      })
    }
  }, [history, historyIndex])

  const goForward = useCallback(() => {
    if (historyIndex < history.length - 1) {
      const newIndex = historyIndex + 1
      setHistoryIndex(newIndex)
      setNavigation({
        current: history[newIndex],
        history: history.slice(0, newIndex + 1),
        canGoBack: newIndex > 0,
        canGoForward: newIndex < history.length - 1,
      })
    }
  }, [history, historyIndex])

  const navigateToParent = useCallback(() => {
    const current = navigation.current
    if (current && current.parent) {
      navigateTo(current.parent.level, current.parent.id, current.parent.name)
    }
  }, [navigation.current, navigateTo])

  const clearHistory = useCallback(() => {
    const current = navigation.current
    setHistory(current ? [current] : [])
    setHistoryIndex(current ? 0 : -1)
    setNavigation({
      current,
      history: current ? [current] : [],
      canGoBack: false,
      canGoForward: false,
    })
  }, [navigation.current])

  return {
    navigation,
    navigateTo,
    goBack,
    goForward,
    navigateToParent,
    clearHistory,
  }
}
```

---

## Step 3: Create Zoom Controls Component

- [ ] **Step 3.1: Create ZoomControls component**

```typescript
// agentsdk/glyphnova/src-frontend/components/navigation/ZoomControls.tsx
import type { ZoomLevel } from '../../types/api'

interface ZoomControlsProps {
  currentLevel: ZoomLevel | null
  onLevelChange: (level: ZoomLevel) => void
}

const zoomLevels: Array<{ level: ZoomLevel; label: string; description: string }> = [
  { level: 'workflow', label: 'Workflow', description: 'View entire workflow structure' },
  { level: 'step', label: 'Step', description: 'Focus on workflow steps' },
  { level: 'agent', label: 'Agent', description: 'Inspect agent executions' },
  { level: 'tool', label: 'Tool', description: 'View tool-level details' },
]

export function ZoomControls({ currentLevel, onLevelChange }: ZoomControlsProps) {
  return (
    <div className="flex items-center gap-2">
      <span className="text-sm text-gray-400">Zoom:</span>
      {zoomLevels.map(({ level, label, description }) => (
        <button
          key={level}
          onClick={() => onLevelChange(level)}
          className={`px-3 py-1 text-sm rounded-lg transition-colors ${
            currentLevel === level
              ? 'bg-blue-600 text-white'
              : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
          }`}
          title={description}
        >
          {label}
        </button>
      ))}
    </div>
  )
}
```

---

## Step 4: Create Navigation State Machine Component

- [ ] **Step 4.1: Create NavigationStateMachine component**

```typescript
// agentsdk/glyphnova/src-frontend/components/navigation/NavigationStateMachine.tsx
import type { NavigationState } from '../../types/api'

interface NavigationStateMachineProps {
  navigation: NavigationState
  onBack: () => void
  onForward: () => void
  onUp: () => void
}

export function NavigationStateMachine({
  navigation,
  onBack,
  onForward,
  onUp,
}: NavigationStateMachineProps) {
  return (
    <div className="flex items-center gap-2">
      {/* Back Button */}
      <button
        onClick={onBack}
        disabled={!navigation.canGoBack}
        className={`p-2 rounded-lg transition-colors ${
          navigation.canGoBack
            ? 'bg-gray-700 hover:bg-gray-600 text-white'
            : 'bg-gray-800 text-gray-600 cursor-not-allowed'
        }`}
        title="Go back (Ctrl+Left Arrow)"
      >
        ←
      </button>

      {/* Forward Button */}
      <button
        onClick={onForward}
        disabled={!navigation.canGoForward}
        className={`p-2 rounded-lg transition-colors ${
          navigation.canGoForward
            ? 'bg-gray-700 hover:bg-gray-600 text-white'
            : 'bg-gray-800 text-gray-600 cursor-not-allowed'
        }`}
        title="Go forward (Ctrl+Right Arrow)"
      >
        →
      </button>

      {/* Up Button */}
      <button
        onClick={onUp}
        disabled={!navigation.current?.parent}
        className={`p-2 rounded-lg transition-colors ${
          navigation.current?.parent
            ? 'bg-gray-700 hover:bg-gray-600 text-white'
            : 'bg-gray-800 text-gray-600 cursor-not-allowed'
        }`}
        title="Go up (Ctrl+Up Arrow)"
      >
        ↑
      </button>

      {/* Current Level Indicator */}
      {navigation.current && (
        <div className="ml-4 px-3 py-1 bg-blue-600/20 border border-blue-600/50 rounded-lg">
          <span className="text-sm font-medium text-blue-400">
            {navigation.current.level}
          </span>
          {navigation.current.name && (
            <span className="text-sm text-gray-400 ml-2">
              {navigation.current.name}
            </span>
          )}
        </div>
      )}
    </div>
  )
}
```

---

## Step 5: Create Zoom View Components

- [ ] **Step 5.1: Create WorkflowView component**

```typescript
// agentsdk/glyphnova/src-frontend/components/visualization/WorkflowView.tsx
import { useWorkflow } from '../../hooks/useApi'

interface WorkflowViewProps {
  workflowId: string
}

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
    <div className="p-8">
      <h2 className="text-2xl font-bold mb-4">{workflow.name}</h2>
      {workflow.description && (
        <p className="text-gray-400 mb-6">{workflow.description}</p>
      )}

      <div className="space-y-4">
        <h3 className="text-lg font-semibold">Steps ({workflow.steps.length})</h3>
        {workflow.steps.map((step, index) => (
          <div
            key={step.id}
            className="p-4 bg-gray-800 border border-gray-700 rounded-lg"
          >
            <div className="flex items-start justify-between mb-2">
              <div>
                <h4 className="font-medium">
                  {index + 1}. {step.name}
                </h4>
                <p className="text-sm text-gray-400">Agent: {step.agent}</p>
              </div>
              <span className="text-xs text-gray-500">{step.id.slice(0, 8)}...</span>
            </div>

            {step.tools.length > 0 && (
              <div className="mt-2">
                <span className="text-xs text-gray-500">Tools: </span>
                {step.tools.map((tool, i) => (
                  <span key={tool} className="text-xs text-blue-400">
                    {tool}{i < step.tools.length - 1 && ', '}
                  </span>
                ))}
              </div>
            )}

            {step.dependencies && step.dependencies.length > 0 && (
              <div className="mt-2">
                <span className="text-xs text-gray-500">Dependencies: </span>
                {step.dependencies.map((dep, i) => (
                  <span key={dep} className="text-xs text-yellow-400">
                    {dep}{i < step.dependencies.length - 1 && ', '}
                  </span>
                ))}
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  )
}
```

- [ ] **Step 5.2: Create StepView component**

```typescript
// agentsdk/glyphnova/src-frontend/components/visualization/StepView.tsx
import { useWorkflowExecution } from '../../hooks/useApi'

interface StepViewProps {
  workflowExecutionId: string
  stepId: string
}

export function StepView({ workflowExecutionId, stepId }: StepViewProps) {
  const { data: execution, isLoading, error } = useWorkflowExecution(workflowExecutionId)

  if (isLoading) {
    return <div className="p-8 text-gray-400">Loading step...</div>
  }

  if (error) {
    return <div className="p-8 text-red-400">Error loading step: {error.message}</div>
  }

  const step = execution?.steps.find((s) => s.step_id === stepId)

  if (!step) {
    return <div className="p-8 text-gray-400">Step not found</div>
  }

  return (
    <div className="p-8">
      <h2 className="text-2xl font-bold mb-4">{step.step_id}</h2>

      <div className="space-y-6">
        <div>
          <h3 className="text-sm font-semibold text-gray-400 mb-2">Status</h3>
          <div className="text-lg font-medium">{step.status}</div>
        </div>

        {step.started_at && (
          <div>
            <h3 className="text-sm font-semibold text-gray-400 mb-2">Started</h3>
            <div className="text-gray-300">
              {new Date(step.started_at).toLocaleString()}
            </div>
          </div>
        )}

        {step.completed_at && (
          <div>
            <h3 className="text-sm font-semibold text-gray-400 mb-2">Completed</h3>
            <div className="text-gray-300">
              {new Date(step.completed_at).toLocaleString()}
            </div>
          </div>
        )}

        {step.error && (
          <div>
            <h3 className="text-sm font-semibold text-gray-400 mb-2">Error</h3>
            <div className="p-3 bg-red-900/20 border border-red-800 rounded text-red-400">
              {step.error}
            </div>
          </div>
        )}

        <div>
          <h3 className="text-sm font-semibold text-gray-400 mb-2">Actions</h3>
          <button className="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-lg text-sm">
            View Agent Execution
          </button>
        </div>
      </div>
    </div>
  )
}
```

---

## Step 6: Create Main Content Component with Zoom Support

- [ ] **Step 6.1: Update MainContent component**

```typescript
// agentsdk/glyphnova/src-frontend/components/layout/MainContent.tsx
import { useNavigation } from '../../hooks/useNavigation'
import { ZoomControls } from '../navigation/ZoomControls'
import { NavigationStateMachine } from '../navigation/NavigationStateMachine'
import { WorkflowView } from '../visualization/WorkflowView'
import { StepView } from '../visualization/StepView'
import type { ZoomLevel } from '../../types/api'

export function MainContent() {
  const { navigation, navigateTo, goBack, goForward, navigateToParent } = useNavigation()
  const { selectedQueueItem } = useAppStore()

  const handleZoomLevelChange = (level: ZoomLevel) => {
    if (selectedQueueItem) {
      navigateTo(level, selectedQueueItem.id, selectedQueueItem.step_name)
    }
  }

  // Render content based on zoom level
  const renderContent = () => {
    if (!navigation.current) {
      return (
        <div className="flex flex-col items-center justify-center h-full text-gray-400">
          <div className="text-6xl mb-4">🔍</div>
          <p className="text-lg font-medium mb-2">Select an item to view details</p>
          <p className="text-sm">Choose a queue item or use zoom controls</p>
        </div>
      )
    }

    switch (navigation.current.level) {
      case 'workflow':
        return selectedQueueItem ? (
          <WorkflowView workflowId={selectedQueueItem.workflow_id} />
        ) : null
      case 'step':
        return selectedQueueItem ? (
          <StepView
            workflowExecutionId={selectedQueueItem.id}
            stepId={selectedQueueItem.step_name}
          />
        ) : null
      case 'agent':
        return <div className="p-8 text-gray-400">Agent view (TODO)</div>
      case 'tool':
        return <div className="p-8 text-gray-400">Tool view (TODO)</div>
      default:
        return null
    }
  }

  return (
    <main className="flex flex-col h-full">
      {/* Navigation Toolbar */}
      <div className="flex items-center justify-between p-4 border-b border-gray-700">
        <NavigationStateMachine
          navigation={navigation}
          onBack={goBack}
          onForward={goForward}
          onUp={navigateToParent}
        />
        <ZoomControls
          currentLevel={navigation.current?.level || null}
          onLevelChange={handleZoomLevelChange}
        />
      </div>

      {/* Content Area */}
      <div className="flex-1 overflow-y-auto">
        {renderContent()}
      </div>
    </main>
  )
}
```

---

## Step 7: Add Keyboard Shortcuts

- [ ] **Step 7.1: Add keyboard shortcuts to MainContent**

```typescript
// Update MainContent component
useEffect(() => {
  const handleKeyDown = (e: KeyboardEvent) => {
    // Ctrl/Cmd + Left: Go back
    if ((e.ctrlKey || e.metaKey) && e.key === 'ArrowLeft' && navigation.canGoBack) {
      e.preventDefault()
      goBack()
    }

    // Ctrl/Cmd + Right: Go forward
    if ((e.ctrlKey || e.metaKey) && e.key === 'ArrowRight' && navigation.canGoForward) {
      e.preventDefault()
      goForward()
    }

    // Ctrl/Cmd + Up: Go up to parent
    if ((e.ctrlKey || e.metaKey) && e.key === 'ArrowUp' && navigation.current?.parent) {
      e.preventDefault()
      navigateToParent()
    }

    // 1-4: Change zoom level
    if (e.key === '1') handleZoomLevelChange('workflow')
    if (e.key === '2') handleZoomLevelChange('step')
    if (e.key === '3') handleZoomLevelChange('agent')
    if (e.key === '4') handleZoomLevelChange('tool')
  }

  window.addEventListener('keydown', handleKeyDown)
  return () => window.removeEventListener('keydown', handleKeyDown)
}, [navigation, goBack, goForward, navigateToParent, handleZoomLevelChange])
```

---

## Step 8: Add Zoom Animation Transitions

- [ ] **Step 8.1: Create ZoomTransition component**

```typescript
// agentsdk/glyphnova/src-frontend/components/visualization/ZoomTransition.tsx
import { ReactNode } from 'react'

interface ZoomTransitionProps {
  children: ReactNode
  direction: 'in' | 'out'
}

export function ZoomTransition({ children, direction }: ZoomTransitionProps) {
  return (
    <div
      className={`transition-all duration-300 ease-in-out ${
        direction === 'in'
          ? 'opacity-0 scale-95'
          : 'opacity-100 scale-100'
      }`}
    >
      {children}
    </div>
  )
}
```

- [ ] **Step 8.2: Add animation state to MainContent**

```typescript
// Update MainContent component
const [zoomDirection, setZoomDirection] = useState<'in' | 'out'>('in')

useEffect(() => {
  setZoomDirection('in')
  const timer = setTimeout(() => setZoomDirection('out'), 300)
  return () => clearTimeout(timer)
}, [navigation.current?.level])

return (
  <main className="flex flex-col h-full">
    {/* Navigation Toolbar */}
    <div className="flex items-center justify-between p-4 border-b border-gray-700">
      {/* ... toolbar content ... */}
    </div>

    {/* Content Area */}
    <div className="flex-1 overflow-y-auto">
      <ZoomTransition direction={zoomDirection}>
        {renderContent()}
      </ZoomTransition>
    </div>
  </main>
)
```

---

## Step 9: Test Multi-Zoom Navigation

- [ ] **Step 9.1: Run Tauri dev server**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk/glyphnova
npm run tauri dev
```

- [ ] **Step 9.2: Verify zoom controls work**

Expected:
- Zoom level buttons display in header
- Clicking zoom levels changes view
- Navigation state machine buttons work

- [ ] **Step 9.3: Test keyboard shortcuts**

Expected:
- Ctrl/Cmd + Left goes back
- Ctrl/Cmd + Right goes forward
- Ctrl/Cmd + Up goes to parent
- 1-4 keys change zoom levels

- [ ] **Step 9.4: Test zoom transitions**

Expected:
- Smooth animations when changing zoom levels
- Content fades and scales appropriately

---

## Step 10: Commit

- [ ] **Step 10.1: Stage and commit changes**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk
git add glyphnova/src-frontend/components/navigation/ glyphnova/src-frontend/components/visualization/ glyphnova/src-frontend/hooks/useNavigation.tsx
git commit -m "feat(ui): implement multi-zoom navigation with abstraction levels

- Create navigation state machine with back/forward/up buttons
- Build zoom controls for workflow/step/agent/tool levels
- Implement WorkflowView and StepView components
- Add keyboard shortcuts (Ctrl+Arrows, 1-4)
- Create smooth zoom transitions with animations
- Support navigation history management

ADR Compliance:
- Navigation reflects actual execution structure
- Zoom levels map to runtime hierarchy
- No artificial levels or views

Task: 04-multi-zoom-navigation
Part of: Phase 3 - Glyphnova UI"
```

---

## Success Criteria

- [ ] Zoom controls display and work
- [ ] Navigation state machine (back/forward/up) functions
- [ ] Keyboard shortcuts work correctly
- [ ] Zoom transitions are smooth
- [ ] Content renders at each zoom level
- [ ] History management works (back/forward)
- [ ] Parent navigation works (go up)

---

## Next Steps

After completing this task, proceed to **Task 05: Drag-Drop Reprioritization** to implement queue reordering.
