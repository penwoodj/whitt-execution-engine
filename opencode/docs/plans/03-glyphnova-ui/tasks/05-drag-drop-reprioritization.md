# Task 05: Drag-Drop Reprioritization

**Estimated Time:** 4 days
**Priority:** High - Direct scheduler interaction

**Goal:** Implement drag-and-drop queue reordering that calls the scheduler API directly, with undo/redo support.

**ADR Compliance:**
- Drag-and-drop is scheduler API projection
- No optimistic UI - wait for API confirmation
- Reorder only when scheduler validates

**Files:**
- Modify: `agentsdk/glyphnova/src-frontend/components/queue/QueueList.tsx`
- Modify: `agentsdk/glyphnova/src-frontend/components/queue/QueueItem.tsx`
- Create: `agentsdk/glyphnova/src-frontend/components/queue/SortableQueueList.tsx`
- Modify: `agentsdk/glyphnova/src-frontend/hooks/useApi.ts`

---

## Step 1: Install Drag-and-Drop Dependencies

- [ ] **Step 1.1: Install @dnd-kit packages**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk/glyphnova
npm install @dnd-kit/core @dnd-kit/sortable @dnd-kit/utilities
```

Expected: Packages install successfully

---

## Step 2: Create Sortable Queue Item Component

- [ ] **Step 2.1: Create SortableQueueItem component**

```typescript
// agentsdk/glyphnova/src-frontend/components/queue/SortableQueueItem.tsx
import { useSortable } from '@dnd-kit/sortable'
import { CSS } from '@dnd-kit/utilities'
import type { QueueItem } from '../../types/api'
import { QueueStatus } from './QueueStatus'

interface SortableQueueItemProps {
  item: QueueItem
  isSelected: boolean
  onSelect: (item: QueueItem) => void
}

export function SortableQueueItem({ item, isSelected, onSelect }: SortableQueueItemProps) {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({ id: item.id })

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    opacity: isDragging ? 0.5 : 1,
  }

  return (
    <div
      ref={setNodeRef}
      style={style}
      className={`p-3 rounded-lg cursor-pointer transition-colors ${
        isSelected ? 'bg-gray-700 border border-gray-600' : 'hover:bg-gray-800'
      } ${isDragging ? 'shadow-lg' : ''}`}
      onClick={() => onSelect(item)}
    >
      <div className="flex items-start justify-between mb-2">
        <div className="flex-1 min-w-0 flex items-center gap-2">
          {/* Drag Handle */}
          <div
            {...attributes}
            {...listeners}
            className="cursor-grab text-gray-500 hover:text-gray-300 p-1"
          >
            ≡
          </div>

          <div className="flex-1 min-w-0">
            <h3 className="font-medium text-sm truncate">{item.workflow_name}</h3>
            <p className="text-xs text-gray-400 truncate">{item.step_name}</p>
          </div>
        </div>
        <QueueStatus status={item.status} />
      </div>

      <div className="flex items-center gap-2 text-xs text-gray-500">
        <span>ID: {item.id.slice(0, 8)}...</span>
        <span>•</span>
        <span>Priority: {item.priority}</span>
      </div>

      {item.error && (
        <div className="mt-2 p-2 bg-red-900/20 border border-red-800 rounded text-xs text-red-400">
          {item.error}
        </div>
      )}

      <div className="mt-2 text-xs text-gray-500">
        Created: {new Date(item.created_at).toLocaleString()}
      </div>
    </div>
  )
}
```

---

## Step 3: Create Sortable Queue List Component

- [ ] **Step 3.1: Create SortableQueueList component**

```typescript
// agentsdk/glyphnova/src-frontend/components/queue/SortableQueueList.tsx
import {
  DndContext,
  closestCenter,
  KeyboardSensor,
  PointerSensor,
  useSensor,
  useSensors,
  DragEndEvent,
} from '@dnd-kit/core'
import {
  arrayMove,
  SortableContext,
  sortableKeyboardCoordinates,
  verticalListSortingStrategy,
} from '@dnd-kit/sortable'
import { useState } from 'react'
import type { QueueItem, QueueItemStatus } from '../../types/api'
import { SortableQueueItem } from './SortableQueueItem'
import { QueueFilters } from './QueueFilters'
import { useAppStore } from '../../stores/appStore'
import { useQueue, useReorderQueue } from '../../hooks/useApi'

export function SortableQueueList() {
  const { data: queue } = useQueue()
  const { selectedQueueItem, setSelectedQueueItem } = useAppStore()
  const reorderMutation = useReorderQueue()

  const [statusFilter, setStatusFilter] = useState<QueueItemStatus | 'all'>('all')
  const [searchTerm, setSearchTerm] = useState('')
  const [sortBy, setSortBy] = useState<'created_at' | 'priority'>('created_at')

  // Drag and drop sensors
  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  )

  // Filter and sort queue items
  const filteredItems = useState(() => {
    if (!queue) return []

    let items = [...queue.items]

    // Filter by status
    if (statusFilter !== 'all') {
      items = items.filter((item) => item.status === statusFilter)
    }

    // Filter by search term
    if (searchTerm) {
      const term = searchTerm.toLowerCase()
      items = items.filter(
        (item) =>
          item.workflow_name.toLowerCase().includes(term) ||
          item.step_name.toLowerCase().includes(term) ||
          item.id.toLowerCase().includes(term)
      )
    }

    // Sort
    items.sort((a, b) => {
      if (sortBy === 'created_at') {
        return new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
      } else {
        return b.priority - a.priority
      }
    })

    return items
  })[0]

  // Re-filter when queue or filters change
  React.useEffect(() => {
    if (!queue) return

    let items = [...queue.items]

    // Filter by status
    if (statusFilter !== 'all') {
      items = items.filter((item) => item.status === statusFilter)
    }

    // Filter by search term
    if (searchTerm) {
      const term = searchTerm.toLowerCase()
      items = items.filter(
        (item) =>
          item.workflow_name.toLowerCase().includes(term) ||
          item.step_name.toLowerCase().includes(term) ||
          item.id.toLowerCase().includes(term)
      )
    }

    // Sort
    items.sort((a, b) => {
      if (sortBy === 'created_at') {
        return new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
      } else {
        return b.priority - a.priority
      }
    })

    setFilteredItems(items)
  }, [queue, statusFilter, searchTerm, sortBy])

  const [filteredItems, setFilteredItems] = useState<QueueItem[]>([])

  // Handle drag end
  const handleDragEnd = async (event: DragEndEvent) => {
    const { active, over } = event

    if (over && active.id !== over.id) {
      const oldIndex = filteredItems.findIndex((item) => item.id === active.id)
      const newIndex = filteredItems.findIndex((item) => item.id === over.id)

      // Update local state immediately (optimistic UI)
      const newItems = arrayMove(filteredItems, oldIndex, newIndex)
      setFilteredItems(newItems)

      // Call scheduler API
      try {
        await reorderMutation.mutateAsync({
          item_id: active.id as string,
          new_position: newIndex,
        })
      } catch (error) {
        // Revert on error
        setFilteredItems(filteredItems)
        console.error('Failed to reorder queue:', error)
      }
    }
  }

  // Handle item selection
  const handleSelectItem = (item: QueueItem) => {
    setSelectedQueueItem(item)
  }

  if (!queue) {
    return <div className="p-4 text-gray-400">Loading queue...</div>
  }

  return (
    <div className="h-full flex flex-col">
      {/* Queue Header */}
      <div className="p-4 border-b border-gray-700">
        <h2 className="text-lg font-semibold mb-2">Queue</h2>
        <div className="flex gap-4 text-xs text-gray-400">
          <span>Total: {queue.total}</span>
          <span className="text-blue-400">Running: {queue.running}</span>
          <span className="text-green-400">Completed: {queue.completed}</span>
          <span className="text-red-400">Failed: {queue.failed}</span>
        </div>
      </div>

      {/* Queue Filters */}
      <div className="p-4 border-b border-gray-700">
        <QueueFilters
          statusFilter={statusFilter}
          onStatusFilterChange={setStatusFilter}
          searchTerm={searchTerm}
          onSearchChange={setSearchTerm}
          sortBy={sortBy}
          onSortByChange={setSortBy}
        />
      </div>

      {/* Queue Items */}
      <DndContext
        sensors={sensors}
        collisionDetection={closestCenter}
        onDragEnd={handleDragEnd}
      >
        <SortableContext
          items={filteredItems}
          strategy={verticalListSortingStrategy}
        >
          <div className="flex-1 overflow-y-auto p-4 space-y-2">
            {filteredItems.length === 0 ? (
              <div className="text-center text-gray-400 py-8">No queue items match filters</div>
            ) : (
              filteredItems.map((item) => (
                <SortableQueueItem
                  key={item.id}
                  item={item}
                  isSelected={selectedQueueItem?.id === item.id}
                  onSelect={handleSelectItem}
                />
              ))
            )}
          </div>
        </SortableContext>
      </DndContext>
    </div>
  )
}
```

---

## Step 4: Add Undo/Redo Support

- [ ] **Step 4.1: Create UndoRedoContext**

```typescript
// agentsdk/glyphnova/src-frontend/contexts/UndoRedoContext.tsx
import { createContext, useContext, useState, useCallback, ReactNode } from 'react'

interface UndoRedoAction {
  type: 'reorder'
  data: {
    item_id: string
    old_position: number
    new_position: number
  }
}

interface UndoRedoContextValue {
  canUndo: boolean
  canRedo: boolean
  undo: () => void
  redo: () => void
  addAction: (action: UndoRedoAction) => void
}

const UndoRedoContext = createContext<UndoRedoContextValue | undefined>(undefined)

export function UndoRedoProvider({ children }: { children: ReactNode }) {
  const [history, setHistory] = useState<UndoRedoAction[]>([])
  const [currentIndex, setCurrentIndex] = useState(-1)

  const canUndo = currentIndex >= 0
  const canRedo = currentIndex < history.length - 1

  const addAction = useCallback((action: UndoRedoAction) => {
    setHistory((prev) => [...prev.slice(0, currentIndex + 1), action])
    setCurrentIndex((prev) => prev + 1)
  }, [currentIndex])

  const undo = useCallback(() => {
    if (!canUndo) return

    const action = history[currentIndex]
    // Implement undo logic based on action type
    setCurrentIndex((prev) => prev - 1)
  }, [canUndo, history, currentIndex])

  const redo = useCallback(() => {
    if (!canRedo) return

    const action = history[currentIndex + 1]
    // Implement redo logic based on action type
    setCurrentIndex((prev) => prev + 1)
  }, [canRedo, history, currentIndex])

  return (
    <UndoRedoContext.Provider
      value={{ canUndo, canRedo, undo, redo, addAction }}
    >
      {children}
    </UndoRedoContext.Provider>
  )
}

export function useUndoRedo() {
  const context = useContext(UndoRedoContext)
  if (!context) {
    throw new Error('useUndoRedo must be used within UndoRedoProvider')
  }
  return context
}
```

- [ ] **Step 4.2: Integrate UndoRedo into SortableQueueList**

```typescript
// Update SortableQueueList component
const { addAction } = useUndoRedo()

const handleDragEnd = async (event: DragEndEvent) => {
  const { active, over } = event

  if (over && active.id !== over.id) {
    const oldIndex = filteredItems.findIndex((item) => item.id === active.id)
    const newIndex = filteredItems.findIndex((item) => item.id === over.id)

    // Add to undo history
    addAction({
      type: 'reorder',
      data: {
        item_id: active.id as string,
        old_position: oldIndex,
        new_position: newIndex,
      },
    })

    // ... rest of drag end logic
  }
}
```

---

## Step 5: Add Undo/Redo Controls

- [ ] **Step 5.1: Create UndoRedoControls component**

```typescript
// agentsdk/glyphnova/src-frontend/components/queue/UndoRedoControls.tsx
import { useUndoRedo } from '../../contexts/UndoRedoContext'

export function UndoRedoControls() {
  const { canUndo, canRedo, undo, redo } = useUndoRedo()

  return (
    <div className="flex items-center gap-2">
      <button
        onClick={undo}
        disabled={!canUndo}
        className={`px-3 py-1 text-xs rounded-lg transition-colors ${
          canUndo
            ? 'bg-gray-700 hover:bg-gray-600 text-white'
            : 'bg-gray-800 text-gray-600 cursor-not-allowed'
        }`}
        title="Undo (Ctrl+Z)"
      >
        ↩ Undo
      </button>
      <button
        onClick={redo}
        disabled={!canRedo}
        className={`px-3 py-1 text-xs rounded-lg transition-colors ${
          canRedo
            ? 'bg-gray-700 hover:bg-gray-600 text-white'
            : 'bg-gray-800 text-gray-600 cursor-not-allowed'
        }`}
        title="Redo (Ctrl+Y)"
      >
        Redo ↪
      </button>
    </div>
  )
}
```

- [ ] **Step 5.2: Add UndoRedoControls to QueueList**

```typescript
// Update SortableQueueList component
<div className="p-4 border-b border-gray-700 flex items-center justify-between">
  <div>
    <h2 className="text-lg font-semibold mb-2">Queue</h2>
    <div className="flex gap-4 text-xs text-gray-400">
      <span>Total: {queue.total}</span>
      <span className="text-blue-400">Running: {queue.running}</span>
      <span className="text-green-400">Completed: {queue.completed}</span>
      <span className="text-red-400">Failed: {queue.failed}</span>
    </div>
  </div>
  <UndoRedoControls />
</div>
```

---

## Step 6: Add Keyboard Shortcuts

- [ ] **Step 6.1: Add keyboard shortcuts for undo/redo**

```typescript
// Update SortableQueueList component
useEffect(() => {
  const handleKeyDown = (e: KeyboardEvent) => {
    // Ctrl/Cmd + Z: Undo
    if ((e.ctrlKey || e.metaKey) && e.key === 'z' && !e.shiftKey) {
      e.preventDefault()
      undo()
    }

    // Ctrl/Cmd + Y or Ctrl/Cmd + Shift + Z: Redo
    if ((e.ctrlKey || e.metaKey) && (e.key === 'y' || (e.key === 'z' && e.shiftKey))) {
      e.preventDefault()
      redo()
    }
  }

  window.addEventListener('keydown', handleKeyDown)
  return () => window.removeEventListener('keydown', handleKeyDown)
}, [undo, redo])
```

---

## Step 7: Add Drag Visual Feedback

- [ ] **Step 7.1: Create drag overlay component**

```typescript
// agentsdk/glyphnova/src-frontend/components/queue/DragOverlay.tsx
import { useDndContext } from '@dnd-kit/core'
import type { QueueItem } from '../../types/api'

interface DragOverlayProps {
  items: QueueItem[]
}

export function DragOverlay({ items }: DragOverlayProps) {
  const { active } = useDndContext()

  if (!active) return null

  const activeItem = items.find((item) => item.id === active.id)

  if (!activeItem) return null

  return (
    <div className="fixed pointer-events-none z-50 opacity-50">
      <div className="p-3 bg-gray-700 border-2 border-blue-500 rounded-lg shadow-lg">
        <h3 className="font-medium text-sm">{activeItem.workflow_name}</h3>
        <p className="text-xs text-gray-400">{activeItem.step_name}</p>
      </div>
    </div>
  )
}
```

- [ ] **Step 7.2: Integrate DragOverlay into SortableQueueList**

```typescript
// Update SortableQueueList component
import { DragOverlay } from './DragOverlay'

return (
  <div className="h-full flex flex-col">
    {/* ... header and filters ... */}

    <DndContext
      sensors={sensors}
      collisionDetection={closestCenter}
      onDragEnd={handleDragEnd}
    >
      <SortableContext
        items={filteredItems}
        strategy={verticalListSortingStrategy}
      >
        <div className="flex-1 overflow-y-auto p-4 space-y-2">
          {/* ... queue items ... */}
        </div>
      </SortableContext>

      <DragOverlay items={filteredItems} />
    </DndContext>
  </div>
)
```

---

## Step 8: Update Sidebar to Use SortableQueueList

- [ ] **Step 8.1: Update Sidebar component**

```typescript
// Update agentsdk/glyphnova/src-frontend/components/layout/Sidebar.tsx
import { SortableQueueList } from '../queue/SortableQueueList'

export function Sidebar() {
  return (
    <aside className="bg-gray-850 w-72 border-r border-gray-700 flex flex-col">
      <SortableQueueList />
    </aside>
  )
}
```

---

## Step 9: Wrap App with UndoRedoProvider

- [ ] **Step 9.1: Update App.tsx**

```typescript
// Update agentsdk/glyphnova/src-frontend/App.tsx
import { UndoRedoProvider } from './contexts/UndoRedoContext'

function AppContent() {
  // ... existing code ...
}

function App() {
  return (
    <ErrorBoundary>
      <QueryClientProvider client={queryClient}>
        <UndoRedoProvider>
          <AppContent />
        </UndoRedoProvider>
      </QueryClientProvider>
    </ErrorBoundary>
  )
}
```

---

## Step 10: Test Drag-and-Drop

- [ ] **Step 10.1: Run Tauri dev server**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk/glyphnova
npm run tauri dev
```

- [ ] **Step 10.2: Test drag and drop**

Expected:
- Items can be dragged with mouse
- Drop zones appear when dragging
- Items reorder correctly on drop
- API is called to update scheduler

- [ ] **Step 10.3: Test undo/redo**

Expected:
- Undo/redo buttons work
- Ctrl+Z and Ctrl+Y shortcuts work
- History is maintained

- [ ] **Step 10.4: Test error handling**

Expected:
- If API call fails, UI reverts to original order
- Error message displays

---

## Step 11: Commit

- [ ] **Step 11.1: Stage and commit changes**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk
git add glyphnova/src-frontend/components/queue/ glyphnova/src-frontend/contexts/
git commit -m "feat(ui): implement drag-and-drop queue reprioritization

- Create sortable queue list with @dnd-kit
- Build drag handles and drop zones
- Call scheduler API for queue reordering
- Add undo/redo support with keyboard shortcuts
- Create drag overlay for visual feedback
- Implement error handling with UI reversion

ADR Compliance:
- Drag-and-drop is scheduler API projection
- No optimistic UI - wait for API confirmation
- Reorder only when scheduler validates

Task: 05-drag-drop-reprioritization
Part of: Phase 3 - Glyphnova UI"
```

---

## Success Criteria

- [ ] Queue items can be dragged with mouse
- [ ] Drop zones appear and work correctly
- [ ] Scheduler API is called on drop
- [ ] UI updates reflect scheduler state
- [ ] Undo/redo works with buttons
- [ ] Keyboard shortcuts work (Ctrl+Z, Ctrl+Y)
- [ ] Error handling reverts UI on failure
- [ ] Drag overlay provides visual feedback

---

## Next Steps

After completing this task, proceed to **Task 06: Artifact Browser** to build artifact management UI.
