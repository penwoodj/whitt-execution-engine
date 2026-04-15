# Task 03: Scope Indicators

**Estimated Time:** 3 days
**Priority:** High - Always-visible context navigation

**Goal:** Build the header with breadcrumb navigation showing current scope (workspace → project → workflow → step) and context switch detection.

**ADR Compliance:**
- Scope is ALWAYS visible in header
- No actions that change scope without explicit confirmation
- Breadcrumbs reflect actual navigation state

**Files:**
- Create: `agentsdk/glyphnova/src-frontend/components/layout/Header.tsx`
- Create: `agentsdk/glyphnova/src-frontend/components/navigation/BreadcrumbNav.tsx`
- Create: `agentsdk/glyphnova/src-frontend/components/navigation/ScopeConfirmDialog.tsx`
- Modify: `agentsdk/glyphnova/src-frontend/stores/appStore.ts`

---

## Step 1: Enhance App Store with Scope Management

- [ ] **Step 1.1: Update appStore to include scope history**

```typescript
// Update agentsdk/glyphnova/src-frontend/stores/appStore.ts

interface AppState {
  // Current scope
  currentScope: Scope | null
  setCurrentScope: (scope: Scope) => void

  // Scope history for breadcrumbs
  scopeHistory: Scope[]
  setScopeHistory: (history: Scope[]) => void

  // Pending scope change (for confirmation)
  pendingScopeChange: { from: Scope; to: Scope } | null
  setPendingScopeChange: (change: { from: Scope; to: Scope } | null) => void

  // Scope change log
  scopeChangeLog: Array<{ timestamp: string; from: Scope; to: Scope }>
  addScopeChangeLog: (from: Scope, to: Scope) => void

  // ... existing fields ...
}

export const useAppStore = create<AppState>((set, get) => ({
  // ... existing initialization ...

  scopeHistory: [],
  setScopeHistory: (history) => set({ scopeHistory: history }),

  pendingScopeChange: null,
  setPendingScopeChange: (change) => set({ pendingScopeChange: change }),

  scopeChangeLog: [],
  addScopeChangeLog: (from, to) =>
    set((state) => ({
      scopeChangeLog: [
        ...state.scopeChangeLog,
        { timestamp: new Date().toISOString(), from, to },
      ],
    })),
}))
```

---

## Step 2: Create Breadcrumb Navigation Component

- [ ] **Step 2.1: Create BreadcrumbNav component**

```typescript
// agentsdk/glyphnova/src-frontend/components/navigation/BreadcrumbNav.tsx
import type { Scope } from '../../types/api'

interface BreadcrumbNavProps {
  currentScope: Scope | null
  onNavigate: (scope: Scope) => void
}

export function BreadcrumbNav({ currentScope, onNavigate }: BreadcrumbNavProps) {
  if (!currentScope) {
    return <div className="text-gray-400">Select a scope</div>
  }

  // Build breadcrumb path from current scope
  const breadcrumbs: Scope[] = []
  let scope: Scope | undefined = currentScope

  while (scope) {
    breadcrumbs.unshift(scope)
    scope = scope.parent
  }

  return (
    <nav className="flex items-center gap-2 text-sm">
      {breadcrumbs.map((scope, index) => (
        <React.Fragment key={`${scope.level}-${scope.id}`}>
          {index > 0 && <span className="text-gray-600">/</span>}
          <button
            onClick={() => onNavigate(scope)}
            className={`hover:text-blue-400 transition-colors ${
              index === breadcrumbs.length - 1
                ? 'font-semibold text-white'
                : 'text-gray-400'
            }`}
          >
            {scope.name || scope.level}
          </button>
        </React.Fragment>
      ))}
    </nav>
  )
}
```

---

## Step 3: Create Scope Confirmation Dialog

- [ ] **Step 3.1: Create ScopeConfirmDialog component**

```typescript
// agentsdk/glyphnova/src-frontend/components/navigation/ScopeConfirmDialog.tsx
import type { Scope } from '../../types/api'

interface ScopeConfirmDialogProps {
  from: Scope
  to: Scope
  onConfirm: () => void
  onCancel: () => void
}

export function ScopeConfirmDialog({ from, to, onConfirm, onCancel }: ScopeConfirmDialogProps) {
  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-gray-800 border border-gray-700 rounded-lg p-6 max-w-md w-full mx-4">
        <h2 className="text-xl font-bold mb-4 text-yellow-400">⚠️ Scope Change Warning</h2>

        <div className="space-y-4 text-gray-300">
          <p>
            You are about to change the scope of operation. This will affect all
            actions you perform.
          </p>

          <div className="bg-gray-900 rounded-lg p-4 space-y-2">
            <div className="flex items-center gap-2">
              <span className="text-gray-500">From:</span>
              <span className="font-medium">{from.name || from.level}</span>
              <span className="text-gray-500">({from.level})</span>
            </div>
            <div className="flex items-center gap-2">
              <span className="text-gray-500">To:</span>
              <span className="font-medium text-blue-400">{to.name || to.level}</span>
              <span className="text-gray-500">({to.level})</span>
            </div>
          </div>

          <p className="text-sm text-yellow-400">
            Are you sure you want to proceed?
          </p>
        </div>

        <div className="flex gap-3 mt-6">
          <button
            onClick={onCancel}
            className="flex-1 px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded-lg transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={onConfirm}
            className="flex-1 px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors"
          >
            Confirm
          </button>
        </div>
      </div>
    </div>
  )
}
```

---

## Step 4: Create Header Component with Scope Indicators

- [ ] **Step 4.1: Create enhanced Header component**

```typescript
// agentsdk/glyphnova/src-frontend/components/layout/Header.tsx
import { useState } from 'react'
import { BreadcrumbNav } from '../navigation/BreadcrumbNav'
import { ScopeConfirmDialog } from '../navigation/ScopeConfirmDialog'
import { useAppStore } from '../../stores/appStore'
import type { Scope } from '../../types/api'

export function Header() {
  const {
    currentScope,
    setCurrentScope,
    setScopeHistory,
    setPendingScopeChange,
    addScopeChangeLog,
    runtimeConnected,
  } = useAppStore()

  const [showConfirmDialog, setShowConfirmDialog] = useState(false)
  const [pendingScope, setPendingScope] = useState<Scope | null>(null)

  const handleNavigate = (scope: Scope) => {
    if (scope === currentScope) return

    // Store the pending scope change for confirmation
    setPendingScope(scope)
    setPendingScopeChange({
      from: currentScope!,
      to: scope,
    })
    setShowConfirmDialog(true)
  }

  const handleConfirmScopeChange = () => {
    if (!pendingScope || !currentScope) return

    // Log the change
    addScopeChangeLog(currentScope, pendingScope)

    // Update scope history
    setScopeHistory([...useAppStore.getState().scopeHistory, currentScope])

    // Set new scope
    setCurrentScope(pendingScope)

    // Close dialog
    setShowConfirmDialog(false)
    setPendingScope(null)
    setPendingScopeChange(null)
  }

  const handleCancelScopeChange = () => {
    setShowConfirmDialog(false)
    setPendingScope(null)
    setPendingScopeChange(null)
  }

  return (
    <>
      <header className="bg-gray-800 border-b border-gray-700 h-16 flex items-center justify-between px-6">
        {/* Left: Logo and Breadcrumbs */}
        <div className="flex items-center gap-6 flex-1">
          <h1 className="text-xl font-bold text-white">Glyphnova</h1>
          <div className="h-6 w-px bg-gray-700" />
          <BreadcrumbNav currentScope={currentScope} onNavigate={handleNavigate} />
        </div>

        {/* Center: Connection Status */}
        <div className="flex items-center gap-2">
          <div
            className={`w-2 h-2 rounded-full ${
              runtimeConnected ? 'bg-green-500' : 'bg-red-500'
            }`}
          />
          <span className="text-sm text-gray-400">
            {runtimeConnected ? 'Connected' : 'Disconnected'}
          </span>
        </div>

        {/* Right: Actions */}
        <div className="flex items-center gap-4">
          <button className="text-gray-400 hover:text-white transition-colors">
            Settings
          </button>
          <button className="text-gray-400 hover:text-white transition-colors">
            Help
          </button>
        </div>
      </header>

      {/* Confirmation Dialog */}
      {showConfirmDialog && currentScope && pendingScope && (
        <ScopeConfirmDialog
          from={currentScope}
          to={pendingScope}
          onConfirm={handleConfirmScopeChange}
          onCancel={handleCancelScopeChange}
        />
      )}
    </>
  )
}
```

---

## Step 5: Add Scope Change Logging

- [ ] **Step 5.1: Create ScopeChangeLog component**

```typescript
// agentsdk/glyphnova/src-frontend/components/navigation/ScopeChangeLog.tsx
import { useAppStore } from '../../stores/appStore'

export function ScopeChangeLog() {
  const { scopeChangeLog } = useAppStore()

  if (scopeChangeLog.length === 0) {
    return (
      <div className="text-sm text-gray-400">No scope changes yet</div>
    )
  }

  return (
    <div className="space-y-2">
      {scopeChangeLog.map((entry, index) => (
        <div key={index} className="flex items-center gap-2 text-sm">
          <span className="text-gray-500">
            {new Date(entry.timestamp).toLocaleTimeString()}
          </span>
          <span className="text-gray-400">→</span>
          <span className="text-gray-300">
            {entry.from.name || entry.from.level} → {entry.to.name || entry.to.level}
          </span>
        </div>
      ))}
    </div>
  )
}
```

- [ ] **Step 5.2: Add scope log toggle to header**

```typescript
// Update Header component to include scope log toggle
const [showScopeLog, setShowScopeLog] = useState(false)

// Add button to header
<button
  onClick={() => setShowScopeLog(!showScopeLog)}
  className="text-gray-400 hover:text-white transition-colors"
>
  History
</button>

// Add dropdown
{showScopeLog && (
  <div className="absolute top-16 right-6 bg-gray-800 border border-gray-700 rounded-lg p-4 shadow-lg z-50 w-80">
    <h3 className="font-semibold mb-3">Scope Change History</h3>
    <ScopeChangeLog />
  </div>
)}
```

---

## Step 6: Add Scope Quick Actions

- [ ] **Step 6.1: Create ScopeQuickActions component**

```typescript
// agentsdk/glyphnova/src-frontend/components/navigation/ScopeQuickActions.tsx
import type { Scope } from '../../types/api'

interface ScopeQuickActionsProps {
  currentScope: Scope | null
}

export function ScopeQuickActions({ currentScope }: ScopeQuickActionsProps) {
  if (!currentScope) return null

  return (
    <div className="flex items-center gap-2">
      <button className="px-3 py-1 text-xs bg-gray-700 hover:bg-gray-600 rounded-lg transition-colors">
        View Details
      </button>
      <button className="px-3 py-1 text-xs bg-gray-700 hover:bg-gray-600 rounded-lg transition-colors">
        Edit
      </button>
      {currentScope.level !== 'workspace' && (
        <button className="px-3 py-1 text-xs bg-gray-700 hover:bg-gray-600 rounded-lg transition-colors">
          Go Up
        </button>
      )}
    </div>
  )
}
```

- [ ] **Step 6.2: Integrate into header**

```typescript
// Update Header to include quick actions
<Header>
  {/* ... existing header content ... */}
  <div className="flex items-center gap-4">
    <ScopeQuickActions currentScope={currentScope} />
  </div>
</Header>
```

---

## Step 7: Add Keyboard Shortcuts for Navigation

- [ ] **Step 7.1: Add keyboard navigation to Header**

```typescript
// Update Header component
useEffect(() => {
  const handleKeyDown = (e: KeyboardEvent) => {
    // Ctrl/Cmd + B: Go back in scope history
    if ((e.ctrlKey || e.metaKey) && e.key === 'b' && e.shiftKey) {
      e.preventDefault()
      const history = useAppStore.getState().scopeHistory
      if (history.length > 0) {
        const previousScope = history[history.length - 1]
        setCurrentScope(previousScope)
      }
    }

    // Escape: Clear scope selection
    if (e.key === 'Escape') {
      setCurrentScope(null)
    }
  }

  window.addEventListener('keydown', handleKeyDown)
  return () => window.removeEventListener('keydown', handleKeyDown)
}, [setCurrentScope])
```

---

## Step 8: Test Scope Indicators

- [ ] **Step 8.1: Run Tauri dev server**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk/glyphnova
npm run tauri dev
```

- [ ] **Step 8.2: Verify breadcrumb navigation**

Expected:
- Breadcrumbs display current scope path
- Clicking breadcrumbs shows confirmation dialog
- Confirmation prevents accidental scope changes

- [ ] **Step 8.3: Test scope change logging**

Expected:
- Scope changes are logged with timestamps
- History view shows all changes
- Log persists during session

- [ ] **Step 8.4: Test keyboard shortcuts**

Expected:
- Ctrl+Shift+B goes back in history
- Escape clears selection

---

## Step 9: Add Scope Context Indicator

- [ ] **Step 9.1: Create ScopeContextIndicator component**

```typescript
// agentsdk/glyphnova/src-frontend/components/navigation/ScopeContextIndicator.tsx
import type { Scope } from '../../types/api'

interface ScopeContextIndicatorProps {
  scope: Scope
}

export function ScopeContextIndicator({ scope }: ScopeContextIndicatorProps) {
  const levelColors: Record<string, string> = {
    workspace: 'bg-purple-600',
    project: 'bg-blue-600',
    workflow: 'bg-green-600',
    step: 'bg-yellow-600',
    agent: 'bg-orange-600',
    tool: 'bg-red-600',
  }

  return (
    <div className="flex items-center gap-2">
      <div className={`w-3 h-3 rounded-full ${levelColors[scope.level]}`} />
      <span className="text-xs font-medium uppercase tracking-wide">
        {scope.level}
      </span>
    </div>
  )
}
```

- [ ] **Step 9.2: Integrate into breadcrumbs**

```typescript
// Update BreadcrumbNav to include context indicator
{breadcrumbs.map((scope, index) => (
  <React.Fragment key={`${scope.level}-${scope.id}`}>
    {index > 0 && <span className="text-gray-600">/</span>}
    <button
      onClick={() => onNavigate(scope)}
      className={`hover:text-blue-400 transition-colors flex items-center gap-2 ${
        index === breadcrumbs.length - 1
          ? 'font-semibold text-white'
          : 'text-gray-400'
      }`}
    >
      <ScopeContextIndicator scope={scope} />
      {scope.name || scope.level}
    </button>
  </React.Fragment>
))}
```

---

## Step 10: Commit

- [ ] **Step 10.1: Stage and commit changes**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk
git add glyphnova/src-frontend/components/layout/ glyphnova/src-frontend/components/navigation/ glyphnova/src-frontend/stores/
git commit -m "feat(ui): implement scope indicators with breadcrumb navigation

- Create header with breadcrumb navigation
- Build scope confirmation dialog for context changes
- Add scope change history logging
- Implement context indicators for each scope level
- Add keyboard shortcuts (Ctrl+Shift+B, Escape)
- Display connection status in header
- Add quick actions for current scope

ADR Compliance:
- Scope is ALWAYS visible in header
- No actions change scope without explicit confirmation
- Breadcrumbs reflect actual navigation state

Task: 03-scope-indicators
Part of: Phase 3 - Glyphnova UI"
```

---

## Success Criteria

- [ ] Header displays breadcrumbs with current scope
- [ ] Clicking breadcrumbs shows confirmation dialog
- [ ] Scope changes are logged with timestamps
- [ ] Context indicators show scope level colors
- [ ] Connection status displays correctly
- [ ] Keyboard shortcuts work (Ctrl+Shift+B, Escape)
- [ ] Quick actions appear for current scope

---

## Next Steps

After completing this task, proceed to **Task 04: Multi-Zoom Navigation** to implement abstraction level navigation.
