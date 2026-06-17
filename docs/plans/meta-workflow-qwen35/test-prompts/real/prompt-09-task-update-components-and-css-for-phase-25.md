<!--
Source Session: ses_233416c8fffeCNcbsDFNmBTWoB
Part ID: prt_dccbe9375002TKy5dX3Ncl6ju2
Character Count: 13245
Extracted: 2026-06-17T07:50:39Z
-->

## TASK: Update components and CSS for Phase 2.5

You are working on Human File Cartographer. Phase 2.5 adds elastic layout, focal planes, markdown rendering, dynamic card heights.

## CRITICAL REACT FLOW V12 RULES (NON-NEGOTIABLE)
1. FileNodeData MUST have `[key: string]: unknown` index signature
2. Use `NodeProps<Node<FileNodeData>>` NOT `NodeProps<FileNodeData>`
3. Use `type FileNodeType = Node<FileNodeData>` then `NodeProps<FileNodeType>`
4. Import `Position` from `@xyflow/react` and use as enum (Position.Top, NOT 'top')
5. Wrap `zoomIn`/`zoomOut` in lambdas: `onClick={() => { zoomIn() }}`
6. Edges need unique `id` fields
7. Access node.data props via casts: `(node.data as Record<string, unknown>).expanded`
8. Handles MUST have `isConnectable={false}` — edge creation disabled

## FILE 1: MODIFY `/home/jon/code/human-file-cartographer/src/components/FileNode.tsx`

Current file (47 lines) shows plain text preview. Update to:
- Remove fixed height (let card auto-size)
- Render markdown using react-markdown + remark-gfm
- Keep skeleton/loading/failed states
- Add `isConnectable={false}` to all Handle components

New content:
```typescript
import { Handle, Position, type NodeProps, type Node } from '@xyflow/react'
import type { FileNodeData } from '../types'
import { memo } from 'react'
import { useFlowStore } from '../store'
import ReactMarkdown from 'react-markdown'
import remarkGfm from 'remark-gfm'

type FileNodeType = Node<FileNodeData>

const MarkdownComponents = {
  h1: ({ children, ...props }: React.HTMLAttributes<HTMLHeadingElement> & { children?: React.ReactNode }) => (
    <h3 style={{ fontSize: '0.95rem', fontWeight: 600, margin: '4px 0 2px', color: '#e0e0e0' }} {...props}>{children}</h3>
  ),
  h2: ({ children, ...props }: React.HTMLAttributes<HTMLHeadingElement> & { children?: React.ReactNode }) => (
    <h4 style={{ fontSize: '0.85rem', fontWeight: 600, margin: '4px 0 2px', color: '#c0c0d0' }} {...props}>{children}</h4>
  ),
  h3: ({ children, ...props }: React.HTMLAttributes<HTMLHeadingElement> & { children?: React.ReactNode }) => (
    <h5 style={{ fontSize: '0.8rem', fontWeight: 500, margin: '2px 0', color: '#b0b0c0' }} {...props}>{children}</h5>
  ),
}

function FileNode({ data, id }: NodeProps<FileNodeType>) {
  const nodeData = data as FileNodeData
  const summarizeNode = useFlowStore((s) => s.summarizeNode)
  const state = nodeData.summaryState ?? 'idle'

  const handleRetry = () => {
    summarizeNode(id)
  }

  return (
    <div className={`file-node state-${state}`}>
      <Handle type="target" position={Position.Top} isConnectable={false} />
      <div className="file-header">
        <div className="file-icon"></div>
        <div className="file-name">{nodeData.fileName}</div>
        <span className={`status-badge ${state}`} />
      </div>
      {(state === 'pending' || state === 'generating') && (
        <div className="preview-skeleton">
          <div className="skeleton-line long" />
          <div className="skeleton-line medium" />
          <div className="skeleton-line short" />
          {state === 'generating' && <div className="generating-spinner" />}
        </div>
      )}
      {state === 'failed' && (
        <div className="preview-failed">
          <span className="error-text">{nodeData.error ?? 'Summary failed'}</span>
          <button type="button" className="retry-button" onClick={handleRetry}>Retry</button>
        </div>
      )}
      {(state === 'idle' || state === 'complete' || state === 'cached') && nodeData.preview && (
        <div className="markdown-content">
          <ReactMarkdown remarkPlugins={[remarkGfm]} components={MarkdownComponents}>
            {nodeData.preview}
          </ReactMarkdown>
        </div>
      )}
      {(state === 'idle' || state === 'complete' || state === 'cached') && !nodeData.preview && (
        <div className="file-preview-empty">No summary available</div>
      )}
      <Handle type="source" position={Position.Bottom} isConnectable={false} />
    </div>
  )
}

export default memo(FileNode)
```

## FILE 2: MODIFY `/home/jon/code/human-file-cartographer/src/components/FolderNode.tsx`

Current file (33 lines). Update to:
- Show folder summary if available (markdown rendered)
- Click navigates into focal plane (calls navigateToFocalPlane)
- Add expand/collapse toggle as separate button
- Add `isConnectable={false}` to all Handles

New content:
```typescript
import { Handle, Position, type NodeProps, type Node } from '@xyflow/react'
import type { FileNodeData } from '../types'
import { useFlowStore } from '../store'
import { memo } from 'react'
import ReactMarkdown from 'react-markdown'
import remarkGfm from 'remark-gfm'

type FolderNodeType = Node<FileNodeData>

function FolderNode({ data, id }: NodeProps<FolderNodeType>) {
  const toggleFolder = useFlowStore((state) => state.toggleFolder)
  const navigateToFocalPlane = useFlowStore((state) => state.navigateToFocalPlane)
  const nodeData = data as FileNodeData

  const handleToggle = (e: React.MouseEvent) => {
    e.stopPropagation()
    toggleFolder(id)
  }

  const handleNavigate = (e: React.MouseEvent) => {
    e.stopPropagation()
    navigateToFocalPlane(id, nodeData.fileName)
  }

  const hasSummary = nodeData.preview && nodeData.summaryState === 'complete' || nodeData.summaryState === 'cached'

  return (
    <div className="folder-node" onClick={handleNavigate} style={{ cursor: 'pointer' }}>
      <Handle type="target" position={Position.Top} isConnectable={false} />
      <div className="folder-header">
        <div className="folder-icon"></div>
        <div className="folder-name">{nodeData.fileName}</div>
        <button type="button" className="toggle-button" aria-label="Toggle folder" onClick={handleToggle}>
          {nodeData.expanded ? '▼' : '▶'}
        </button>
      </div>
      <div className="child-count">{nodeData.childIds.length} items</div>
      {hasSummary && (
        <div className="folder-summary">
          <ReactMarkdown remarkPlugins={[remarkGfm]}>
            {nodeData.preview}
          </ReactMarkdown>
        </div>
      )}
      <Handle type="source" position={Position.Bottom} isConnectable={false} />
    </div>
  )
}

export default memo(FolderNode)
```

## FILE 3: MODIFY `/home/jon/code/human-file-cartographer/src/components/Toolbar.tsx`

Current file (115 lines). Update to:
- Add layout mode toggle button (Tree/Graph)
- Remove "Summarize All" button, replace with "Summarize Layer"
- Remove onConnect reference

Read the file first. Then make these changes:

1. Add to destructured store: `layoutMode, setLayoutMode, summarizeCurrentLayer`
2. Remove `summarizeAllFiles` from destructuring (replace with `summarizeCurrentLayer`)
3. Change the handleSummarize function:
```typescript
const handleSummarize = async () => {
  await summarizeCurrentLayer()
}
```
4. Change the Summarize button text from "✨ Summarize All" to "✨ Summarize Layer"
5. Add a layout toggle button after the existing direction toggle:
```typescript
<button type="button" onClick={() => setLayoutMode(layoutMode === 'tree' ? 'graph' : 'tree')}>
  Layout: {layoutMode === 'tree' ? 'Tree' : 'Graph'}
</button>
```
6. Remove `onConnect` from any ReactFlow props if present

## FILE 4: CREATE `/home/jon/code/human-file-cartographer/src/components/Breadcrumbs.tsx`

```typescript
import { useFlowStore } from '../store'
import type { FocalPlane } from '../types'

function focalPlaneToArray(plane: FocalPlane | null): FocalPlane[] {
  const planes: FocalPlane[] = []
  let current: FocalPlane | null = plane
  while (current) {
    planes.unshift(current)
    current = current.parentFocalPlane
  }
  planes.unshift({ folderId: '__root__', folderName: 'Root', parentFocalPlane: null })
  return planes
}

export function Breadcrumbs() {
  const currentFocalPlane = useFlowStore((s) => s.currentFocalPlane)
  const navigateToFocalPlaneIndex = useFlowStore((s) => s.navigateToFocalPlaneIndex)
  const navigateToRoot = useFlowStore((s) => s.navigateToRoot)

  if (!currentFocalPlane) return null

  const planes = focalPlaneToArray(currentFocalPlane)

  const handleClick = (index: number) => {
    if (index === 0) {
      navigateToRoot()
    } else {
      navigateToFocalPlaneIndex(index)
    }
  }

  return (
    <div className="breadcrumbs">
      {planes.map((plane, i) => (
        <span key={`${plane.folderId}-${i}`} className="breadcrumb-item">
          {i > 0 && <span className="breadcrumb-sep">›</span>}
          <button
            type="button"
            className={`breadcrumb ${i === planes.length - 1 ? 'active' : ''}`}
            onClick={() => handleClick(i)}
          >
            {plane.folderName}
          </button>
        </span>
      ))}
    </div>
  )
}
```

## FILE 5: MODIFY `/home/jon/code/human-file-cartographer/src/styles/index.css`

Read the existing file first (452 lines). Then make these changes:

**A. Change .file-node to remove fixed height:**
Find `.file-node` and change:
```css
.file-node {
  width: 320px;
  /* REMOVED: height: 120px; — now auto-sizes */
  min-height: 60px;
  background: linear-gradient(135deg, #2a2a4a 0%, #1a1a3a 100%);
  /* ... rest stays the same ... */
}
```

**B. Change .file-preview to allow dynamic height:**
Find `.file-preview` and change:
```css
.file-preview {
  flex: 1;
  font-size: 12px;
  color: #a0a0b0;
  line-height: 1.4;
  overflow: hidden;
  display: -webkit-box;
  -webkit-line-clamp: 8;  /* was 4, now shows more */
  -webkit-box-orient: vertical;
}
```

**C. Add markdown content styles** at the end of the file (before the @media query):
```css
/* Markdown content in file nodes */
.markdown-content {
  overflow-wrap: break-word;
  word-wrap: break-word;
  line-height: 1.5;
  max-height: 400px;
  overflow-y: auto;
  font-size: 12px;
  color: #c0c0d0;
}

.markdown-content h1,
.markdown-content h2,
.markdown-content h3,
.markdown-content h4,
.markdown-content h5 {
  color: #e0e0e0;
  margin: 4px 0 2px;
}

.markdown-content h1 {
  font-size: 0.95rem;
  font-weight: 600;
}

.markdown-content h2 {
  font-size: 0.85rem;
  font-weight: 600;
}

.markdown-content h3 {
  font-size: 0.8rem;
  font-weight: 500;
}

.markdown-content p {
  margin: 2px 0;
}

.markdown-content ul, .markdown-content ol {
  margin: 2px 0 2px 16px;
  padding: 0;
}

.markdown-content li {
  margin: 1px 0;
}

.markdown-content code {
  font-size: 0.7rem;
  background: #0a0a1a;
  padding: 1px 4px;
  border-radius: 3px;
  color: #a0d0a0;
}

.markdown-content pre {
  white-space: pre-wrap;
  background: #0a0a1a;
  padding: 8px;
  border-radius: 4px;
  font-size: 0.7rem;
  overflow-x: auto;
}

.markdown-content pre code {
  background: none;
  padding: 0;
}

.markdown-content table {
  table-layout: fixed;
  width: 100%;
  font-size: 0.7rem;
  border-collapse: collapse;
  margin: 4px 0;
}

.markdown-content th,
.markdown-content td {
  border: 1px solid #3a3a5a;
  padding: 2px 6px;
  text-align: left;
}

.markdown-content th {
  background: #2a2a4a;
  font-weight: 600;
}

.markdown-content strong {
  color: #e0e0f0;
}

.markdown-content a {
  color: #818cf8;
  text-decoration: underline;
}

.markdown-content blockquote {
  border-left: 3px solid #4a4a6a;
  margin: 4px 0;
  padding: 2px 8px;
  color: #9090a0;
}

.file-preview-empty {
  font-size: 11px;
  color: #606070;
  font-style: italic;
}

/* Folder summary */
.folder-summary {
  margin-top: 4px;
  font-size: 11px;
  color: #c0c0d0;
  line-height: 1.4;
  max-height: 120px;
  overflow-y: auto;
}

.folder-summary h1, .folder-summary h2, .folder-summary h3,
.folder-summary h4, .folder-summary h5 {
  font-size: 0.75rem;
  color: #e0e0e0;
  margin: 2px 0 1px;
}

.folder-summary ul, .folder-summary ol {
  margin: 1px 0 1px 12px;
}

.folder-summary code {
  font-size: 0.65rem;
  background: #0a0a1a;
  padding: 1px 3px;
  border-radius: 2px;
}

/* Breadcrumbs */
.breadcrumbs {
  position: fixed;
  top: 80px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  align-items: center;
  gap: 4px;
  background: rgba(26, 26, 46, 0.9);
  border: 1px solid #3a3a5a;
  border-radius: 8px;
  padding: 6px 14px;
  z-index: 999;
  font-size: 13px;
  backdrop-filter: blur(8px);
}

.breadcrumb-item {
  display: flex;
  align-items: center;
  gap: 4px;
}

.breadcrumb {
  background: none;
  border: none;
  color: #a0a0b0;
  cursor: pointer;
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 13px;
  transition: all 0.15s ease;
}

.breadcrumb:hover {
  color: #e0e0e0;
  background: rgba(255, 255, 255, 0.05);
}

.breadcrumb.active {
  color: #818cf8;
  font-weight: 600;
}

.breadcrumb-sep {
  color: #4a4a6a;
  font-size: 14px;
  user-select: none;
}
```

## MUST DO
- Read all files before editing
- Follow ALL React Flow v12 gotchas listed above
- Use `isConnectable={false}` on ALL Handle components
- No `as any` or `@ts-ignore`
- Cards auto-size (no fixed height)
- H1 in markdown ≤ card title font size

## MUST NOT DO
- Do NOT modify store.ts or treeToFlow.ts (another agent handles those)
- Do NOT install packages (already done)
- Do NOT use string literals for Position
- Do NOT pass zoomIn/zoomOut directly as onClick handlers
- Do NOT remove existing CSS rules unless specified
<!-- OMO_INTERNAL_INITIATOR -->