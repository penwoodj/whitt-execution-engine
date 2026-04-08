# Task 02: Queue Visualization

**Estimated Time:** 5 days
**Priority:** Critical - Main interface to scheduler state

**Goal:** Build the left sidebar queue display showing real-time scheduler state with filtering, sorting, and selection actions.

**ADR Compliance:**
- Queue is a direct projection of scheduler state
- No local queue state caching
- Updates come from WebSocket or polling

**Files:**
- Create: `agentsdk/glyphnova/src-frontend/components/queue/QueueList.tsx`
- Create: `agentsdk/glyphnova/src-frontend/components/queue/QueueItem.tsx`
- Create: `agentsdk/glyphnova/src-frontend/components/queue/QueueFilters.tsx`
- Create: `agentsdk/glyphnova/src-frontend/components/queue/QueueStatus.tsx`
- Modify: `agentsdk/glyphnova/src-frontend/components/layout/Sidebar.tsx`

---

## Step 1: Create Queue Status Indicator Component

- [ ] **Step 1.1: Create QueueStatus component**

```typescript
// agentsdk/glyphnova/src-frontend/components/queue/QueueStatus.tsx
import type { QueueItemStatus } from '../../types/api'

interface QueueStatusProps {
  status: QueueItemStatus
}

export function QueueStatus({ status }: QueueStatusProps) {
  const statusConfig: Record<
    QueueItemStatus,
    { color: string; icon: string; label: string }
  > = {
    pending: {
      color: 'bg-gray-500',
      icon: '○',
      label: 'Pending',
    },
    running: {
      color: 'bg-blue-500 animate-pulse',
      icon: '●',
      label: 'Running',
    },
    completed: {
      color: 'bg-green-500',
      icon: '✓',
      label: 'Completed',
    },
    failed: {
      color: 'bg-red-500',
      icon: '✕',
      label: 'Failed',
    },
    cancelled: {
      color: 'bg-yellow-500',
      icon: '–',
      label: 'Cancelled',
    },
  }

  const config = statusConfig[status]

  return (
    <div className="flex items-center gap-2">
      <span className={config.color}>{config.icon}</span>
      <span className="text-sm text-gray-400">{config.label}</span>
    </div>
  )
}
```

---

## Step 2: Create Queue Item Component

- [ ] **Step 2.1: Create QueueItem component**

```typescript
// agentsdk/glyphnova/src-frontend/components/queue/QueueItem.tsx
import type { QueueItem } from '../../types/api'
import { QueueStatus } from './QueueStatus'

interface QueueItemProps {
  item: QueueItem
  isSelected: boolean
  onSelect: (item: QueueItem) => void
}

export function QueueItem({ item, isSelected, onSelect }: QueueItemProps) {
  return (
    <div
      className={`p-3 rounded-lg cursor-pointer transition-colors ${
        isSelected ? 'bg-gray-700 border border-gray-600' : 'hover:bg-gray-800'
      }`}
      onClick={() => onSelect(item)}
    >
      <div className="flex items-start justify-between mb-2">
        <div className="flex-1 min-w-0">
          <h3 className="font-medium text-sm truncate">{item.workflow_name}</h3>
          <p className="text-xs text-gray-400 truncate">{item.step_name}</p>
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

## Step 3: Create Queue Filters Component

- [ ] **Step 3.1: Create QueueFilters component**

```typescript
// agentsdk/glyphnova/src-frontend/components/queue/QueueFilters.tsx
import type { QueueItemStatus } from '../../types/api'

interface QueueFiltersProps {
  statusFilter: QueueItemStatus | 'all'
  onStatusFilterChange: (status: QueueItemStatus | 'all') => void
  searchTerm: string
  onSearchChange: (term: string) => void
  sortBy: 'created_at' | 'priority'
  onSortByChange: (sortBy: 'created_at' | 'priority') => void
}

export function QueueFilters({
  statusFilter,
  onStatusFilterChange,
  searchTerm,
  onSearchChange,
  sortBy,
  onSortByChange,
}: QueueFiltersProps) {
  return (
    <div className="space-y-3 mb-4">
      {/* Search */}
      <input
        type="text"
        placeholder="Search queue..."
        value={searchTerm}
        onChange={(e) => onSearchChange(e.target.value)}
        className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded-lg text-sm focus:outline-none focus:border-blue-500"
      />

      {/* Status Filter */}
      <div className="flex gap-2">
        {(['all', 'pending', 'running', 'completed', 'failed', 'cancelled'] as const).map(
          (status) => (
            <button
              key={status}
              onClick={() => onStatusFilterChange(status)}
              className={`px-3 py-1 text-xs rounded-lg transition-colors ${
                statusFilter === status
                  ? 'bg-blue-600 text-white'
                  : 'bg-gray-800 text-gray-400 hover:bg-gray-700'
              }`}
            >
              {status === 'all' ? 'All' : status.charAt(0).toUpperCase() + status.slice(1)}
            </button>
          )
        )}
      </div>

      {/* Sort */}
      <div className="flex items-center gap-2">
        <span className="text-xs text-gray-400">Sort by:</span>
        <button
          onClick={() => onSortByChange('created_at')}
          className={`px-2 py-1 text-xs rounded ${
            sortBy === 'created_at' ? 'bg-blue-600' : 'bg-gray-800'
          }`}
        >
          Created
        </button>
        <button
          onClick={() => onSortByChange('priority')}
          className={`px-2 py-1 text-xs rounded ${
            sortBy === 'priority' ? 'bg-blue-600' : 'bg-gray-800'
          }`}
        >
          Priority
        </button>
      </div>
    </div>
  )
}
```

---

## Step 4: Create Queue List Component

- [ ] **Step 4.1: Create QueueList component**

```typescript
// agentsdk/glyphnova/src-frontend/components/queue/QueueList.tsx
import { useState, useMemo } from 'react'
import type { QueueItem, QueueItemStatus } from '../../types/api'
import { QueueItem } from './QueueItem'
import { QueueFilters } from './QueueFilters'
import { useAppStore } from '../../stores/appStore'
import { useQueue } from '../../hooks/useApi'

export function QueueList() {
  const { data: queue } = useQueue()
  const { selectedQueueItem, setSelectedQueueItem } = useAppStore()

  const [statusFilter, setStatusFilter] = useState<QueueItemStatus | 'all'>('all')
  const [searchTerm, setSearchTerm] = useState('')
  const [sortBy, setSortBy] = useState<'created_at' | 'priority'>('created_at')

  // Filter and sort queue items
  const filteredItems = useMemo(() => {
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
  }, [queue, statusFilter, searchTerm, sortBy])

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
      <div className="flex-1 overflow-y-auto p-4 space-y-2">
        {filteredItems.length === 0 ? (
          <div className="text-center text-gray-400 py-8">No queue items match filters</div>
        ) : (
          filteredItems.map((item) => (
            <QueueItem
              key={item.id}
              item={item}
              isSelected={selectedQueueItem?.id === item.id}
              onSelect={handleSelectItem}
            />
          ))
        )}
      </div>
    </div>
  )
}
```

---

## Step 5: Update Sidebar Component

- [ ] **Step 5.1: Update Sidebar to use QueueList**

```typescript
// agentsdk/glyphnova/src-frontend/components/layout/Sidebar.tsx
import { QueueList } from '../queue/QueueList'

export function Sidebar() {
  return (
    <aside className="bg-gray-850 w-72 border-r border-gray-700 flex flex-col">
      <QueueList />
    </aside>
  )
}
```

---

## Step 6: Add Queue Actions Menu

- [ ] **Step 6.1: Create QueueActions component**

```typescript
// agentsdk/glyphnova/src-frontend/components/queue/QueueActions.tsx
import { useState } from 'react'
import type { QueueItem } from '../../types/api'

interface QueueActionsProps {
  item: QueueItem
  onAction: (action: 'cancel' | 'retry' | 'view') => void
}

export function QueueActions({ item, onAction }: QueueActionsProps) {
  const [isOpen, setIsOpen] = useState(false)

  return (
    <div className="relative">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="p-1 hover:bg-gray-700 rounded"
      >
        <span>⋮</span>
      </button>

      {isOpen && (
        <>
          <div className="absolute right-0 top-full mt-1 bg-gray-800 border border-gray-700 rounded-lg shadow-lg z-10 min-w-32">
            <button
              onClick={() => {
                onAction('view')
                setIsOpen(false)
              }}
              className="w-full px-4 py-2 text-left text-sm hover:bg-gray-700 rounded-t-lg"
            >
              View Details
            </button>
            {item.status === 'running' && (
              <button
                onClick={() => {
                  onAction('cancel')
                  setIsOpen(false)
                }}
                className="w-full px-4 py-2 text-left text-sm text-yellow-400 hover:bg-gray-700"
              >
                Cancel
              </button>
            )}
            {item.status === 'failed' && (
              <button
                onClick={() => {
                  onAction('retry')
                  setIsOpen(false)
                }}
                className="w-full px-4 py-2 text-left text-sm text-blue-400 hover:bg-gray-700"
              >
                Retry
              </button>
            )}
          </div>
          <div
            className="fixed inset-0"
            onClick={() => setIsOpen(false)}
          />
        </>
      )}
    </div>
  )
}
```

- [ ] **Step 6.2: Integrate QueueActions into QueueItem**

```typescript
// Update agentsdk/glyphnova/src-frontend/components/queue/QueueItem.tsx
import { QueueActions } from './QueueActions'

interface QueueItemProps {
  item: QueueItem
  isSelected: boolean
  onSelect: (item: QueueItem) => void
  onAction?: (item: QueueItem, action: 'cancel' | 'retry' | 'view') => void
}

// Add actions to the item display
<div className="flex items-start justify-between mb-2">
  <div className="flex-1 min-w-0">
    <h3 className="font-medium text-sm truncate">{item.workflow_name}</h3>
    <p className="text-xs text-gray-400 truncate">{item.step_name}</p>
  </div>
  <div className="flex items-center gap-2">
    <QueueStatus status={item.status} />
    {onAction && (
      <QueueActions
        item={item}
        onAction={(action) => onAction(item, action)}
      />
    )}
  </div>
</div>
```

---

## Step 7: Add Queue Empty State

- [ ] **Step 7.1: Create QueueEmpty component**

```typescript
// agentsdk/glyphnova/src-frontend/components/queue/QueueEmpty.tsx
export function QueueEmpty() {
  return (
    <div className="flex flex-col items-center justify-center h-full text-gray-400">
      <div className="text-6xl mb-4">📭</div>
      <p className="text-lg font-medium mb-2">Queue is empty</p>
      <p className="text-sm">
        Add workflows to the queue to start processing
      </p>
    </div>
  )
}
```

- [ ] **Step 7.2: Use QueueEmpty when queue is empty**

```typescript
// Update QueueList component
{filteredItems.length === 0 ? (
  <QueueEmpty />
) : (
  filteredItems.map((item) => (
    <QueueItem
      key={item.id}
      item={item}
      isSelected={selectedQueueItem?.id === item.id}
      onSelect={handleSelectItem}
    />
  ))
)}
```

---

## Step 8: Test Queue Visualization

- [ ] **Step 8.1: Run Tauri dev server**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk/glyphnova
npm run tauri dev
```

- [ ] **Step 8.2: Verify queue displays correctly**

Expected:
- Queue sidebar shows items
- Status indicators display correctly
- Filters work (status, search, sort)
- Items can be selected
- Actions menu appears on items

- [ ] **Step 8.3: Test real-time updates**

Add items to queue via CLI or API

Expected:
- Queue updates within 2 seconds
- Status changes reflect immediately
- WebSocket updates received

---

## Step 9: Add Keyboard Shortcuts

- [ ] **Step 9.1: Add keyboard navigation to QueueList**

```typescript
// Update agentsdk/glyphnova/src-frontend/components/queue/QueueList.tsx
import { useEffect } from 'react'

export function QueueList() {
  // ... existing code ...

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'ArrowDown' && selectedQueueItem) {
        e.preventDefault()
        const index = filteredItems.findIndex((item) => item.id === selectedQueueItem.id)
        if (index < filteredItems.length - 1) {
          setSelectedQueueItem(filteredItems[index + 1])
        }
      } else if (e.key === 'ArrowUp' && selectedQueueItem) {
        e.preventDefault()
        const index = filteredItems.findIndex((item) => item.id === selectedQueueItem.id)
        if (index > 0) {
          setSelectedQueueItem(filteredItems[index - 1])
        }
      } else if (e.key === 'Escape') {
        setSelectedQueueItem(null)
      }
    }

    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [filteredItems, selectedQueueItem])
}
```

---

## Step 10: Commit

- [ ] **Step 10.1: Stage and commit changes**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk
git add glyphnova/src-frontend/components/queue/
git commit -m "feat(ui): implement queue visualization in sidebar

- Create QueueList component with filtering and sorting
- Build QueueItem component with status indicators
- Add QueueFilters for status/search/sort controls
- Implement queue actions (cancel, retry, view)
- Add keyboard navigation (Arrow keys, Escape)
- Display queue statistics (total, running, completed, failed)
- Support real-time updates via WebSocket

ADR Compliance:
- Queue is direct projection of scheduler state
- No local caching, updates from WebSocket/polling
- Order matches scheduler exactly

Task: 02-queue-visualization
Part of: Phase 3 - Glyphnova UI"
```

---

## Success Criteria

- [ ] Queue displays in left sidebar
- [ ] Real-time updates work (within 2 seconds)
- [ ] Filters work (status, search, sort)
- [ ] Items can be selected
- [ ] Actions menu works (cancel, retry, view)
- [ ] Keyboard navigation works
- [ ] Empty state displays correctly
- [ ] Queue statistics are accurate

---

## Next Steps

After completing this task, proceed to **Task 03: Scope Indicators** to add the header with breadcrumbs.
