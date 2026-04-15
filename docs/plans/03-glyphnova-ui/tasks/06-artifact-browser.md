# Task 06: Artifact Browser

**Estimated Time:** 4 days
**Priority:** High - Essential for reviewing workflow outputs

**Goal:** Build artifact browsing UI with listing, preview, search, filtering, versioning, and export functionality.

**ADR Compliance:**
- Artifacts stored in runtime storage system
- UI provides read-only access
- No artifact modification in UI

**Files:**
- Create: `agentsdk/glyphnova/src-frontend/components/artifacts/ArtifactBrowser.tsx`
- Create: `agentsdk/glyphnova/src-frontend/components/artifacts/ArtifactPreview.tsx`
- Create: `agentsdk/glyphnova/src-frontend/components/artifacts/ArtifactDetails.tsx`
- Create: `agentsdk/glyphnova/src-frontend/components/artifacts/ArtifactFilters.tsx`

---

## Step 1: Create Artifact List Item Component

- [ ] **Step 1.1: Create ArtifactListItem component**

```typescript
// agentsdk/glyphnova/src-frontend/components/artifacts/ArtifactListItem.tsx
import type { Artifact } from '../../types/api'

interface ArtifactListItemProps {
  artifact: Artifact
  isSelected: boolean
  onSelect: (artifact: Artifact) => void
}

export function ArtifactListItem({ artifact, isSelected, onSelect }: ArtifactListItemProps) {
  return (
    <div
      className={`p-3 rounded-lg cursor-pointer transition-colors ${
        isSelected ? 'bg-gray-700 border border-gray-600' : 'hover:bg-gray-800'
      }`}
      onClick={() => onSelect(artifact)}
    >
      <div className="flex items-start gap-3">
        {/* File Icon */}
        <div className="text-2xl">
          {getFileIcon(artifact.type)}
        </div>

        <div className="flex-1 min-w-0">
          <h3 className="font-medium text-sm truncate">{artifact.name}</h3>
          <div className="flex items-center gap-2 text-xs text-gray-500 mt-1">
            <span>{artifact.type}</span>
            <span>•</span>
            <span>{formatFileSize(artifact.size)}</span>
          </div>
          <div className="text-xs text-gray-600 mt-1">
            {new Date(artifact.created_at).toLocaleString()}
          </div>
        </div>
      </div>
    </div>
  )
}

function getFileIcon(type: string): string {
  const iconMap: Record<string, string> = {
    'text/plain': '📄',
    'application/json': '📋',
    'application/pdf': '📕',
    'image/png': '🖼️',
    'image/jpeg': '🖼️',
    'image/svg+xml': '🎨',
    'text/markdown': '📝',
    'text/csv': '📊',
    'application/zip': '📦',
  }

  return iconMap[type] || '📄'
}

function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 Bytes'
  const k = 1024
  const sizes = ['Bytes', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i]
}
```

---

## Step 2: Create Artifact Filters Component

- [ ] **Step 2.1: Create ArtifactFilters component**

```typescript
// agentsdk/glyphnova/src-frontend/components/artifacts/ArtifactFilters.tsx
import { useState } from 'react'

interface ArtifactFiltersProps {
  onSearchChange: (term: string) => void
  onTypeFilterChange: (types: string[]) => void
  onDateFilterChange: (dateRange: { start: Date; end: Date } | null) => void
}

export function ArtifactFilters({
  onSearchChange,
  onTypeFilterChange,
  onDateFilterChange,
}: ArtifactFiltersProps) {
  const [searchTerm, setSearchTerm] = useState('')
  const [selectedTypes, setSelectedTypes] = useState<string[]>([])
  const [dateRange, setDateRange] = useState<{ start: Date; end: Date } | null>(null)

  const commonTypes = [
    'text/plain',
    'application/json',
    'text/markdown',
    'application/pdf',
    'image/png',
    'image/jpeg',
    'text/csv',
  ]

  const handleTypeToggle = (type: string) => {
    const newTypes = selectedTypes.includes(type)
      ? selectedTypes.filter((t) => t !== type)
      : [...selectedTypes, type]

    setSelectedTypes(newTypes)
    onTypeFilterChange(newTypes)
  }

  const handleSearch = (e: React.ChangeEvent<HTMLInputElement>) => {
    const term = e.target.value
    setSearchTerm(term)
    onSearchChange(term)
  }

  return (
    <div className="space-y-3 mb-4">
      {/* Search */}
      <input
        type="text"
        placeholder="Search artifacts..."
        value={searchTerm}
        onChange={handleSearch}
        className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded-lg text-sm focus:outline-none focus:border-blue-500"
      />

      {/* Type Filter */}
      <div>
        <label className="text-xs text-gray-400 mb-2 block">Type</label>
        <div className="flex flex-wrap gap-2">
          {commonTypes.map((type) => (
            <button
              key={type}
              onClick={() => handleTypeToggle(type)}
              className={`px-2 py-1 text-xs rounded-lg transition-colors ${
                selectedTypes.includes(type)
                  ? 'bg-blue-600 text-white'
                  : 'bg-gray-800 text-gray-400 hover:bg-gray-700'
              }`}
            >
              {type}
            </button>
          ))}
        </div>
      </div>

      {/* Date Filter */}
      <div>
        <label className="text-xs text-gray-400 mb-2 block">Date Range</label>
        <div className="flex gap-2">
          <input
            type="date"
            className="flex-1 px-3 py-2 bg-gray-800 border border-gray-700 rounded-lg text-sm"
            onChange={(e) => {
              const start = e.target.value ? new Date(e.target.value) : null
              setDateRange(start && dateRange ? { ...dateRange, start } : null)
              if (start && dateRange?.end) onDateFilterChange({ start, end: dateRange.end })
            }}
          />
          <input
            type="date"
            className="flex-1 px-3 py-2 bg-gray-800 border border-gray-700 rounded-lg text-sm"
            onChange={(e) => {
              const end = e.target.value ? new Date(e.target.value) : null
              setDateRange(end && dateRange ? { ...dateRange, end } : null)
              if (end && dateRange?.start) onDateFilterChange({ start: dateRange.start, end })
            }}
          />
        </div>
      </div>
    </div>
  )
}
```

---

## Step 3: Create Artifact Preview Component

- [ ] **Step 3.1: Create ArtifactPreview component**

```typescript
// agentsdk/glyphnova/src-frontend/components/artifacts/ArtifactPreview.tsx
import type { Artifact } from '../../types/api'
import { useArtifactPreview } from '../../hooks/useApi'

interface ArtifactPreviewProps {
  artifact: Artifact
}

export function ArtifactPreview({ artifact }: ArtifactPreviewProps) {
  const { data: preview, isLoading, error } = useArtifactPreview(artifact.id)

  if (isLoading) {
    return <div className="p-8 text-gray-400">Loading preview...</div>
  }

  if (error) {
    return <div className="p-8 text-red-400">Error loading preview: {error.message}</div>
  }

  if (!preview) {
    return <div className="p-8 text-gray-400">Preview not available</div>
  }

  return (
    <div className="p-4">
      {renderPreview(artifact, preview)}
    </div>
  )
}

function renderPreview(artifact: Artifact, preview: unknown) {
  switch (artifact.type) {
    case 'text/plain':
    case 'application/json':
    case 'text/markdown':
      return (
        <pre className="p-4 bg-gray-900 rounded-lg overflow-auto max-h-96 text-sm">
          {typeof preview === 'string' ? preview : JSON.stringify(preview, null, 2)}
        </pre>
      )

    case 'text/csv':
      return <div className="text-gray-400">CSV preview coming soon</div>

    case 'image/png':
    case 'image/jpeg':
    case 'image/svg+xml':
      return (
        <div className="flex justify-center">
          <img
            src={`data:${artifact.type};base64,${preview}`}
            alt={artifact.name}
            className="max-w-full max-h-96 object-contain"
          />
        </div>
      )

    case 'application/pdf':
      return (
        <div className="flex flex-col items-center justify-center h-64 text-gray-400">
          <span className="text-4xl mb-2">📕</span>
          <p>PDF preview not available</p>
          <button className="mt-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-lg text-sm">
            Download
          </button>
        </div>
      )

    default:
      return (
        <div className="flex flex-col items-center justify-center h-64 text-gray-400">
          <span className="text-4xl mb-2">📄</span>
          <p>Preview not available for this type</p>
          <button className="mt-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-lg text-sm">
            Download
          </button>
        </div>
      )
  }
}
```

---

## Step 4: Create Artifact Details Component

- [ ] **Step 4.1: Create ArtifactDetails component**

```typescript
// agentsdk/glyphnova/src-frontend/components/artifacts/ArtifactDetails.tsx
import type { Artifact } from '../../types/api'

interface ArtifactDetailsProps {
  artifact: Artifact
  onDownload: () => void
  onExport: () => void
}

export function ArtifactDetails({ artifact, onDownload, onExport }: ArtifactDetailsProps) {
  return (
    <div className="p-6 space-y-6">
      {/* Header */}
      <div className="flex items-start justify-between">
        <div>
          <h2 className="text-2xl font-bold">{artifact.name}</h2>
          <p className="text-sm text-gray-400 mt-1">{artifact.type}</p>
        </div>
        <div className="flex gap-2">
          <button
            onClick={onDownload}
            className="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-lg text-sm transition-colors"
          >
            Download
          </button>
          <button
            onClick={onExport}
            className="px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded-lg text-sm transition-colors"
          >
            Export
          </button>
        </div>
      </div>

      {/* Metadata */}
      <div className="grid grid-cols-2 gap-4">
        <div>
          <h3 className="text-sm font-semibold text-gray-400 mb-1">Size</h3>
          <p className="text-gray-300">{formatFileSize(artifact.size)}</p>
        </div>
        <div>
          <h3 className="text-sm font-semibold text-gray-400 mb-1">Created</h3>
          <p className="text-gray-300">
            {new Date(artifact.created_at).toLocaleString()}
          </p>
        </div>
        <div>
          <h3 className="text-sm font-semibold text-gray-400 mb-1">ID</h3>
          <p className="text-gray-300 text-sm font-mono">{artifact.id}</p>
        </div>
        {artifact.workflow_execution_id && (
          <div>
            <h3 className="text-sm font-semibold text-gray-400 mb-1">Workflow</h3>
            <p className="text-gray-300 text-sm font-mono">
              {artifact.workflow_execution_id.slice(0, 8)}...
            </p>
          </div>
        )}
      </div>

      {/* Additional Metadata */}
      {artifact.metadata && Object.keys(artifact.metadata).length > 0 && (
        <div>
          <h3 className="text-sm font-semibold text-gray-400 mb-2">Metadata</h3>
          <div className="bg-gray-800 rounded-lg p-4">
            <pre className="text-sm text-gray-300">
              {JSON.stringify(artifact.metadata, null, 2)}
            </pre>
          </div>
        </div>
      )}
    </div>
  )
}

function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 Bytes'
  const k = 1024
  const sizes = ['Bytes', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i]
}
```

---

## Step 5: Create Artifact Browser Component

- [ ] **Step 5.1: Create ArtifactBrowser component**

```typescript
// agentsdk/glyphnova/src-frontend/components/artifacts/ArtifactBrowser.tsx
import { useState } from 'react'
import { useArtifacts } from '../../hooks/useApi'
import type { Artifact } from '../../types/api'
import { ArtifactListItem } from './ArtifactListItem'
import { ArtifactFilters } from './ArtifactFilters'
import { ArtifactPreview } from './ArtifactPreview'
import { ArtifactDetails } from './ArtifactDetails'

export function ArtifactBrowser() {
  const { data: artifacts, isLoading, error } = useArtifacts()
  const [selectedArtifact, setSelectedArtifact] = useState<Artifact | null>(null)
  const [searchTerm, setSearchTerm] = useState('')
  const [selectedTypes, setSelectedTypes] = useState<string[]>([])
  const [dateRange, setDateRange] = useState<{ start: Date; end: Date } | null>(null)

  // Filter artifacts
  const filteredArtifacts = artifacts?.filter((artifact) => {
    // Search filter
    if (searchTerm) {
      const term = searchTerm.toLowerCase()
      if (!artifact.name.toLowerCase().includes(term)) {
        return false
      }
    }

    // Type filter
    if (selectedTypes.length > 0 && !selectedTypes.includes(artifact.type)) {
      return false
    }

    // Date filter
    if (dateRange) {
      const artifactDate = new Date(artifact.created_at)
      if (artifactDate < dateRange.start || artifactDate > dateRange.end) {
        return false
      }
    }

    return true
  }) || []

  // Handle artifact selection
  const handleSelectArtifact = (artifact: Artifact) => {
    setSelectedArtifact(artifact)
  }

  // Handle download
  const handleDownload = () => {
    if (!selectedArtifact) return
    // Implement download logic
    console.log('Downloading artifact:', selectedArtifact.id)
  }

  // Handle export
  const handleExport = () => {
    if (!selectedArtifact) return
    // Implement export logic
    console.log('Exporting artifact:', selectedArtifact.id)
  }

  if (isLoading) {
    return <div className="p-8 text-gray-400">Loading artifacts...</div>
  }

  if (error) {
    return <div className="p-8 text-red-400">Error loading artifacts: {error.message}</div>
  }

  return (
    <div className="h-full flex">
      {/* Left: Artifact List */}
      <div className="w-80 border-r border-gray-700 flex flex-col">
        <div className="p-4 border-b border-gray-700">
          <h2 className="text-lg font-semibold mb-2">Artifacts</h2>
          <p className="text-xs text-gray-400">
            {artifacts?.length || 0} artifacts found
          </p>
        </div>

        <div className="p-4 border-b border-gray-700">
          <ArtifactFilters
            onSearchChange={setSearchTerm}
            onTypeFilterChange={setSelectedTypes}
            onDateFilterChange={setDateRange}
          />
        </div>

        <div className="flex-1 overflow-y-auto p-4 space-y-2">
          {filteredArtifacts.length === 0 ? (
            <div className="text-center text-gray-400 py-8">
              No artifacts match filters
            </div>
          ) : (
            filteredArtifacts.map((artifact) => (
              <ArtifactListItem
                key={artifact.id}
                artifact={artifact}
                isSelected={selectedArtifact?.id === artifact.id}
                onSelect={handleSelectArtifact}
              />
            ))
          )}
        </div>
      </div>

      {/* Right: Artifact Details */}
      <div className="flex-1 overflow-y-auto">
        {selectedArtifact ? (
          <>
            <ArtifactDetails
              artifact={selectedArtifact}
              onDownload={handleDownload}
              onExport={handleExport}
            />
            <div className="border-t border-gray-700">
              <div className="p-4 border-b border-gray-700">
                <h3 className="text-lg font-semibold">Preview</h3>
              </div>
              <ArtifactPreview artifact={selectedArtifact} />
            </div>
          </>
        ) : (
          <div className="flex flex-col items-center justify-center h-full text-gray-400">
            <div className="text-6xl mb-4">📦</div>
            <p className="text-lg font-medium mb-2">Select an artifact to view details</p>
            <p className="text-sm">Choose an artifact from the list on the left</p>
          </div>
        )}
      </div>
    </div>
  )
}
```

---

## Step 6: Add Export Functionality

- [ ] **Step 6.1: Create export options**

```typescript
// Add to ArtifactDetails component
interface ExportOptionsProps {
  onExport: (format: string) => void
}

export function ExportOptions({ onExport }: ExportOptionsProps) {
  const exportFormats = [
    { format: 'json', label: 'JSON' },
    { format: 'csv', label: 'CSV' },
    { format: 'txt', label: 'Text' },
  ]

  return (
    <div className="mt-4">
      <h3 className="text-sm font-semibold text-gray-400 mb-2">Export As</h3>
      <div className="flex gap-2">
        {exportFormats.map(({ format, label }) => (
          <button
            key={format}
            onClick={() => onExport(format)}
            className="px-3 py-1 bg-gray-800 hover:bg-gray-700 rounded text-sm transition-colors"
          >
            {label}
          </button>
        ))}
      </div>
    </div>
  )
}
```

---

## Step 7: Test Artifact Browser

- [ ] **Step 7.1: Run Tauri dev server**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk/glyphnova
npm run tauri dev
```

- [ ] **Step 7.2: Verify artifact list displays**

Expected:
- Artifacts list on left side
- File icons based on type
- File size and date display

- [ ] **Step 7.3: Test filters**

Expected:
- Search filters artifacts by name
- Type filter shows only selected types
- Date filter filters by date range

- [ ] **Step 7.4: Test preview**

Expected:
- Clicking artifact shows preview
- Text artifacts show content
- Images show thumbnails
- PDF shows download button

- [ ] **Step 7.5: Test download and export**

Expected:
- Download button initiates download
- Export options work
- Data exports in selected format

---

## Step 8: Add Version History

- [ ] **Step 8.1: Create VersionHistory component**

```typescript
// agentsdk/glyphnova/src-frontend/components/artifacts/VersionHistory.tsx
import type { Artifact } from '../../types/api'

interface VersionHistoryProps {
  currentArtifact: Artifact
  versions: Artifact[]
  onSelectVersion: (version: Artifact) => void
}

export function VersionHistory({ currentArtifact, versions, onSelectVersion }: VersionHistoryProps) {
  return (
    <div className="mt-4">
      <h3 className="text-sm font-semibold text-gray-400 mb-2">Version History</h3>
      <div className="space-y-2">
        {versions.map((version, index) => (
          <button
            key={version.id}
            onClick={() => onSelectVersion(version)}
            className={`w-full p-2 rounded text-left transition-colors ${
              version.id === currentArtifact.id
                ? 'bg-blue-600/20 border border-blue-600/50'
                : 'bg-gray-800 hover:bg-gray-700'
            }`}
          >
            <div className="flex items-center justify-between">
              <span className="text-sm">Version {versions.length - index}</span>
              <span className="text-xs text-gray-500">
                {new Date(version.created_at).toLocaleString()}
              </span>
            </div>
          </button>
        ))}
      </div>
    </div>
  )
}
```

---

## Step 9: Commit

- [ ] **Step 9.1: Stage and commit changes**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk
git add glyphnova/src-frontend/components/artifacts/
git commit -m "feat(ui): implement artifact browser with preview and export

- Create artifact list with file icons and metadata
- Build artifact filters (search, type, date)
- Implement preview for text, images, and other formats
- Add artifact details panel with download/export
- Support version history navigation
- Filter artifacts by multiple criteria

ADR Compliance:
- Artifacts stored in runtime storage system
- UI provides read-only access
- No artifact modification in UI

Task: 06-artifact-browser
Part of: Phase 3 - Glyphnova UI"
```

---

## Success Criteria

- [ ] Artifact list displays correctly
- [ ] Filters work (search, type, date)
- [ ] Preview renders for supported types
- [ ] Download initiates artifact download
- [ ] Export works for JSON/CSV/TXT
- [ ] Version history displays
- [ ] Selected artifact shows details

---

## Next Steps

After completing this task, proceed to **Task 07: Summary Graph Views** to build DAG visualization.
