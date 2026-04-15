# Task 01: Shared Backend API

**Estimated Time:** 5 days
**Priority:** Critical - Enables all UI data fetching

**Goal:** Connect the Tauri frontend to the AgentSDK runtime API via REST and WebSocket, implementing the same endpoints used by the CLI.

**ADR Compliance:**
- UI uses the SAME backend API as CLI
- No separate API or database for UI
- All state comes from runtime

**Files:**
- Modify: `agentsdk/glyphnova/src/api/mod.rs` (implement API calls)
- Create: `agentsdk/glyphnova/src-frontend/types/api.ts` (TypeScript types)
- Create: `agentsdk/glyphnova/src-frontend/utils/api-client.ts` (API client)
- Create: `agentsdk/glyphnova/src-frontend/utils/websocket.ts` (WebSocket client)
- Create: `agentsdk/glyphnova/src-frontend/hooks/useApi.ts` (React Query hooks)
- Create: `agentsdk/glyphnova/src-frontend/hooks/useWebSocket.ts` (WebSocket hook)

---

## Step 1: Define TypeScript API Types

- [ ] **Step 1.1: Create base API types**

```typescript
// agentsdk/glyphnova/src-frontend/types/api.ts

// ============================================================================
// Base Types
// ============================================================================

export interface ApiResponse<T> {
  data: T
  status: 'success' | 'error'
  message?: string
}

export interface ApiError {
  code: string
  message: string
  details?: Record<string, unknown>
}

// ============================================================================
// Queue Types
// ============================================================================

export type QueueItemStatus = 'pending' | 'running' | 'completed' | 'failed' | 'cancelled'

export interface QueueItem {
  id: string
  workflow_id: string
  workflow_name: string
  step_name: string
  status: QueueItemStatus
  priority: number
  created_at: string
  started_at?: string
  completed_at?: string
  error?: string
  metadata?: Record<string, unknown>
}

export interface QueueState {
  items: QueueItem[]
  total: number
  pending: number
  running: number
  completed: number
  failed: number
}

// ============================================================================
// Workflow Types
// ============================================================================

export interface WorkflowStep {
  id: string
  name: string
  agent: string
  tools: string[]
  inputs?: Record<string, unknown>
  outputs?: Record<string, unknown>
  dependencies?: string[]
}

export interface Workflow {
  id: string
  name: string
  description?: string
  steps: WorkflowStep[]
  created_at: string
  updated_at: string
  metadata?: Record<string, unknown>
}

export interface WorkflowExecution {
  id: string
  workflow_id: string
  status: 'pending' | 'running' | 'completed' | 'failed'
  created_at: string
  started_at?: string
  completed_at?: string
  steps: WorkflowStepExecution[]
  error?: string
}

export interface WorkflowStepExecution {
  step_id: string
  status: 'pending' | 'running' | 'completed' | 'failed'
  started_at?: string
  completed_at?: string
  error?: string
}

// ============================================================================
// Agent Types
// ============================================================================

export interface Agent {
  id: string
  name: string
  description?: string
  tools: string[]
  capabilities: string[]
  metadata?: Record<string, unknown>
}

export interface AgentExecution {
  id: string
  agent_id: string
  task: string
  status: 'pending' | 'running' | 'completed' | 'failed'
  created_at: string
  started_at?: string
  completed_at?: string
  tools_used: ToolExecution[]
  result?: unknown
  error?: string
}

// ============================================================================
// Tool Types
// ============================================================================

export interface Tool {
  id: string
  name: string
  description?: string
  parameters: Record<string, ParameterDefinition>
  returns: ParameterDefinition
}

export interface ParameterDefinition {
  type: 'string' | 'number' | 'boolean' | 'object' | 'array'
  description?: string
  required?: boolean
  default?: unknown
}

export interface ToolExecution {
  id: string
  tool_id: string
  inputs: Record<string, unknown>
  status: 'pending' | 'running' | 'completed' | 'failed'
  started_at?: string
  completed_at?: string
  outputs?: Record<string, unknown>
  error?: string
}

// ============================================================================
// Artifact Types
// ============================================================================

export interface Artifact {
  id: string
  name: string
  type: string
  size: number
  created_at: string
  workflow_execution_id?: string
  step_execution_id?: string
  metadata?: Record<string, unknown>
  content?: string // For text artifacts
}

export interface ArtifactPreview {
  id: string
  name: string
  preview: string
  size: number
  type: string
}

// ============================================================================
// Metrics Types
// ============================================================================

export interface MetricsSummary {
  total_workflows: number
  successful_workflows: number
  failed_workflows: number
  average_duration_ms: number
  total_artifacts: number
  queue_size: number
}

export interface QualityMetrics {
  timestamp: string
  success_rate: number
  average_duration_ms: number
  error_rate: number
  throughput: number
}

export interface MetricTrend {
  timestamp: string
  value: number
}

// ============================================================================
// WebSocket Events
// ============================================================================

export type WebSocketEvent =
  | QueueUpdateEvent
  | AgentProgressEvent
  | ToolCompleteEvent
  | WorkflowCompleteEvent

export interface QueueUpdateEvent {
  type: 'queue:update'
  data: QueueState
}

export interface AgentProgressEvent {
  type: 'agent:progress'
  data: {
    agent_execution_id: string
    progress: number
    message?: string
  }
}

export interface ToolCompleteEvent {
  type: 'tool:complete'
  data: ToolExecution
}

export interface WorkflowCompleteEvent {
  type: 'workflow:complete'
  data: WorkflowExecution
}

// ============================================================================
// Scope Types (for navigation)
// ============================================================================

export type ScopeLevel = 'workspace' | 'project' | 'workflow' | 'step' | 'agent' | 'tool'

export interface Scope {
  level: ScopeLevel
  id?: string
  name?: string
  parent?: Scope
}

export interface WorkspaceScope extends Scope {
  level: 'workspace'
  id: string
  name: string
}

export interface ProjectScope extends Scope {
  level: 'project'
  id: string
  name: string
  parent: WorkspaceScope
}

export interface WorkflowScope extends Scope {
  level: 'workflow'
  id: string
  name: string
  parent: ProjectScope
}

export interface StepScope extends Scope {
  level: 'step'
  id: string
  name: string
  parent: WorkflowScope
}

export interface AgentScope extends Scope {
  level: 'agent'
  id: string
  name: string
  parent: StepScope
}

export interface ToolScope extends Scope {
  level: 'tool'
  id: string
  name: string
  parent: AgentScope
}
```

- [ ] **Step 1.2: Create API request/response wrapper types**

```typescript
// Add to agentsdk/glyphnova/src-frontend/types/api.ts

export interface ApiRequest {
  method: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH'
  path: string
  body?: unknown
  headers?: Record<string, string>
}

export interface TauriApiRequest {
  method: string
  path: string
  body?: string
}
```

---

## Step 2: Implement Rust API Command Handlers

- [ ] **Step 2.1: Update Rust API module to make real HTTP requests**

```rust
// agentsdk/glyphnova/src/api/mod.rs
use serde::{Deserialize, Serialize};
use tauri::State;
use std::collections::HashMap;

const RUNTIME_API_BASE: &str = "http://localhost:8080/api";

#[derive(Debug, Serialize, Deserialize)]
pub struct TauriApiRequest {
    pub method: String,
    pub path: String,
    pub body: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TauriApiResponse {
    pub status: u16,
    pub body: String,
    pub headers: HashMap<String, String>,
}

/// Make an HTTP request to the runtime API
#[tauri::command]
pub async fn api_request(req: TauriApiRequest) -> Result<TauriApiResponse, String> {
    let client = reqwest::Client::new();
    let url = format!("{}{}", RUNTIME_API_BASE, req.path);

    let mut request_builder = match req.method.as_str() {
        "GET" => client.get(&url),
        "POST" => client.post(&url),
        "PUT" => client.put(&url),
        "DELETE" => client.delete(&url),
        "PATCH" => client.patch(&url),
        _ => return Err(format!("Unsupported HTTP method: {}", req.method)),
    };

    // Add body if present
    if let Some(body) = req.body {
        request_builder = request_builder.header("Content-Type", "application/json");
        request_builder = request_builder.body(body);
    }

    // Execute request
    let response = request_builder
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    let status = response.status().as_u16();
    let body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    let mut headers = HashMap::new();
    for (name, value) in response.headers() {
        if let Ok(value_str) = value.to_str() {
            headers.insert(name.to_string(), value_str.to_string());
        }
    }

    Ok(TauriApiResponse {
        status,
        body,
        headers,
    })
}

/// Get runtime connection status
#[tauri::command]
pub async fn get_runtime_status() -> Result<String, String> {
    let client = reqwest::Client::new();

    match client.get(format!("{}/health", RUNTIME_API_BASE)).send().await {
        Ok(response) => {
            if response.status().is_success() {
                Ok(r#"{"status": "connected"}"#.to_string())
            } else {
                Ok(r#"{"status": "error"}"#.to_string())
            }
        }
        Err(_) => Ok(r#"{"status": "disconnected"}"#.to_string()),
    }
}

/// Get WebSocket server URL
#[tauri::command]
pub fn get_websocket_url() -> Result<String, String> {
    Ok("ws://localhost:8080/ws".to_string())
}
```

- [ ] **Step 2.2: Update lib.rs to include new commands**

```rust
// agentsdk/glyphnova/src/lib.rs
mod api;

use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            api::api_request,
            api::get_runtime_status,
            api::get_websocket_url,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 2.3: Add reqwest dependency**

```toml
# Add to agentsdk/glyphnova/src-tauri/Cargo.toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
```

- [ ] **Step 2.4: Test Rust compilation**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk/glyphnova
cargo build --manifest-path=src-tauri/Cargo.toml
```

Expected: Build succeeds without errors

---

## Step 3: Create TypeScript API Client

- [ ] **Step 3.1: Create API client utility**

```typescript
// agentsdk/glyphnova/src-frontend/utils/api-client.ts
import { invoke } from '@tauri-apps/api/core'
import type {
  ApiResponse,
  ApiError,
  TauriApiRequest,
} from '../types/api'

/**
 * Generic API client that communicates with Tauri backend
 * Tauri forwards requests to runtime REST API
 */
export class ApiClient {
  /**
   * Make an API request
   */
  async request<T>(
    method: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH',
    path: string,
    body?: unknown
  ): Promise<T> {
    const tauriReq: TauriApiRequest = {
      method,
      path,
      body: body ? JSON.stringify(body) : undefined,
    }

    try {
      const response = await invoke<TauriApiResponse>('api_request', {
        req: tauriReq,
      })

      if (response.status >= 400) {
        const error: ApiError = JSON.parse(response.body)
        throw new Error(error.message || 'API request failed')
      }

      const result: ApiResponse<T> = JSON.parse(response.body)
      return result.data
    } catch (error) {
      if (error instanceof Error) {
        throw error
      }
      throw new Error('Unknown error occurred')
    }
  }

  /**
   * GET request
   */
  async get<T>(path: string): Promise<T> {
    return this.request<T>('GET', path)
  }

  /**
   * POST request
   */
  async post<T>(path: string, body: unknown): Promise<T> {
    return this.request<T>('POST', path, body)
  }

  /**
   * PUT request
   */
  async put<T>(path: string, body: unknown): Promise<T> {
    return this.request<T>('PUT', path, body)
  }

  /**
   * DELETE request
   */
  async delete<T>(path: string): Promise<T> {
    return this.request<T>('DELETE', path)
  }

  /**
   * PATCH request
   */
  async patch<T>(path: string, body: unknown): Promise<T> {
    return this.request<T>('PATCH', path, body)
  }
}

// Singleton instance
export const apiClient = new ApiClient()
```

- [ ] **Step 3.2: Add runtime status check**

```typescript
// Add to agentsdk/glyphnova/src-frontend/utils/api-client.ts

/**
 * Check runtime connection status
 */
export async function getRuntimeStatus(): Promise<'connected' | 'disconnected' | 'error'> {
  try {
    const response = await invoke<string>('get_runtime_status')
    const status = JSON.parse(response)
    return status.status
  } catch {
    return 'error'
  }
}
```

---

## Step 4: Create WebSocket Client

- [ ] **Step 4.1: Create WebSocket client utility**

```typescript
// agentsdk/glyphnova/src-frontend/utils/websocket.ts
import type { WebSocketEvent } from '../types/api'

export type WebSocketMessageHandler = (event: WebSocketEvent) => void
export type WebSocketCloseHandler = () => void
export type WebSocketErrorHandler = (error: Event) => void

export class WebSocketClient {
  private ws: WebSocket | null = null
  private reconnectAttempts = 0
  private maxReconnectAttempts = 5
  private reconnectDelay = 1000
  private reconnectTimeout: ReturnType<typeof setTimeout> | null = null
  private messageHandlers: Set<WebSocketMessageHandler> = new Set()
  private closeHandlers: Set<WebSocketCloseHandler> = new Set()
  private errorHandlers: Set<WebSocketErrorHandler> = new Set()
  private url: string

  constructor(url: string) {
    this.url = url
  }

  /**
   * Connect to WebSocket server
   */
  connect(): void {
    if (this.ws?.readyState === WebSocket.OPEN) {
      return
    }

    try {
      this.ws = new WebSocket(this.url)

      this.ws.onopen = () => {
        console.log('WebSocket connected')
        this.reconnectAttempts = 0
      }

      this.ws.onmessage = (event) => {
        try {
          const data: WebSocketEvent = JSON.parse(event.data)
          this.messageHandlers.forEach((handler) => handler(data))
        } catch (error) {
          console.error('Failed to parse WebSocket message:', error)
        }
      }

      this.ws.onclose = () => {
        console.log('WebSocket disconnected')
        this.closeHandlers.forEach((handler) => handler())
        this.scheduleReconnect()
      }

      this.ws.onerror = (error) => {
        console.error('WebSocket error:', error)
        this.errorHandlers.forEach((handler) => handler(error))
      }
    } catch (error) {
      console.error('Failed to create WebSocket:', error)
      this.scheduleReconnect()
    }
  }

  /**
   * Disconnect from WebSocket server
   */
  disconnect(): void {
    if (this.reconnectTimeout) {
      clearTimeout(this.reconnectTimeout)
      this.reconnectTimeout = null
    }

    if (this.ws) {
      this.ws.close()
      this.ws = null
    }
  }

  /**
   * Schedule reconnection attempt
   */
  private scheduleReconnect(): void {
    if (this.reconnectAttempts >= this.maxReconnectAttempts) {
      console.error('Max reconnection attempts reached')
      return
    }

    const delay = this.reconnectDelay * Math.pow(2, this.reconnectAttempts)

    this.reconnectTimeout = setTimeout(() => {
      this.reconnectAttempts++
      console.log(`Reconnecting... (attempt ${this.reconnectAttempts})`)
      this.connect()
    }, delay)
  }

  /**
   * Add message handler
   */
  onMessage(handler: WebSocketMessageHandler): () => void {
    this.messageHandlers.add(handler)
    return () => this.messageHandlers.delete(handler)
  }

  /**
   * Add close handler
   */
  onClose(handler: WebSocketCloseHandler): () => void {
    this.closeHandlers.add(handler)
    return () => this.closeHandlers.delete(handler)
  }

  /**
   * Add error handler
   */
  onError(handler: WebSocketErrorHandler): () => void {
    this.errorHandlers.add(handler)
    return () => this.errorHandlers.delete(handler)
  }

  /**
   * Send message to server
   */
  send(message: unknown): void {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(message))
    } else {
      console.error('WebSocket is not connected')
    }
  }
}

// Singleton instance (will be initialized with URL from Tauri)
let wsClient: WebSocketClient | null = null

export function getWebSocketClient(): Promise<WebSocketClient> {
  return new Promise((resolve, reject) => {
    if (wsClient) {
      resolve(wsClient)
      return
    }

    // Get WebSocket URL from Tauri
    invoke('get_websocket_url')
      .then((url: string) => {
        wsClient = new WebSocketClient(url)
        wsClient.connect()
        resolve(wsClient)
      })
      .catch(reject)
  })
}

export async function getWebsocketUrl(): Promise<string> {
  return invoke('get_websocket_url')
}
```

---

## Step 5: Create React Query Hooks

- [ ] **Step 5.1: Create API hooks**

```typescript
// agentsdk/glyphnova/src-frontend/hooks/useApi.ts
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { apiClient, getRuntimeStatus } from '../utils/api-client'
import type {
  QueueState,
  Workflow,
  WorkflowExecution,
  Artifact,
  MetricsSummary,
  QualityMetrics,
} from '../types/api'

/**
 * Get queue state
 */
export function useQueue() {
  return useQuery({
    queryKey: ['queue'],
    queryFn: () => apiClient.get<QueueState>('/queue'),
    refetchInterval: 2000, // Poll every 2 seconds
  })
}

/**
 * Get workflow by ID
 */
export function useWorkflow(id: string) {
  return useQuery({
    queryKey: ['workflow', id],
    queryFn: () => apiClient.get<Workflow>(`/workflows/${id}`),
    enabled: !!id,
  })
}

/**
 * Get all workflows
 */
export function useWorkflows() {
  return useQuery({
    queryKey: ['workflows'],
    queryFn: () => apiClient.get<Workflow[]>('/workflows'),
  })
}

/**
 * Get workflow execution by ID
 */
export function useWorkflowExecution(id: string) {
  return useQuery({
    queryKey: ['workflow-execution', id],
    queryFn: () => apiClient.get<WorkflowExecution>(`/workflow-executions/${id}`),
    enabled: !!id,
  })
}

/**
 * Execute workflow
 */
export function useExecuteWorkflow() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (workflowId: string) =>
      apiClient.post<WorkflowExecution>(`/workflows/${workflowId}/execute`, {}),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['queue'] })
      queryClient.invalidateQueries({ queryKey: ['workflows'] })
    },
  })
}

/**
 * Get artifacts
 */
export function useArtifacts(workflowExecutionId?: string) {
  return useQuery({
    queryKey: ['artifacts', workflowExecutionId],
    queryFn: () => {
      const params = workflowExecutionId
        ? `?workflow_execution_id=${workflowExecutionId}`
        : ''
      return apiClient.get<Artifact[]>(`/artifacts${params}`)
    },
  })
}

/**
 * Get artifact preview
 */
export function useArtifactPreview(id: string) {
  return useQuery({
    queryKey: ['artifact-preview', id],
    queryFn: () => apiClient.get(`/artifacts/${id}/preview`),
    enabled: !!id,
  })
}

/**
 * Get metrics summary
 */
export function useMetricsSummary() {
  return useQuery({
    queryKey: ['metrics-summary'],
    queryFn: () => apiClient.get<MetricsSummary>('/metrics/summary'),
    refetchInterval: 5000, // Poll every 5 seconds
  })
}

/**
 * Get quality metrics
 */
export function useQualityMetrics(timeRange: string = '1h') {
  return useQuery({
    queryKey: ['quality-metrics', timeRange],
    queryFn: () => apiClient.get<QualityMetrics[]>(`/metrics/quality?range=${timeRange}`),
    refetchInterval: 10000, // Poll every 10 seconds
  })
}

/**
 * Reorder queue items
 */
export function useReorderQueue() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (body: { item_id: string; new_position: number }) =>
      apiClient.post('/queue/reorder', body),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['queue'] })
    },
  })
}

/**
 * Get runtime status
 */
export function useRuntimeStatus() {
  return useQuery({
    queryKey: ['runtime-status'],
    queryFn: getRuntimeStatus,
    refetchInterval: 3000, // Poll every 3 seconds
  })
}
```

---

## Step 6: Create WebSocket Hook

- [ ] **Step 6.1: Create WebSocket hook**

```typescript
// agentsdk/glyphnova/src-frontend/hooks/useWebSocket.ts
import { useEffect, useState } from 'react'
import { getWebSocketClient } from '../utils/websocket'
import type { WebSocketEvent, QueueUpdateEvent, AgentProgressEvent } from '../types/api'

export function useWebSocket() {
  const [connected, setConnected] = useState(false)
  const [event, setEvent] = useState<WebSocketEvent | null>(null)

  useEffect(() => {
    let client: Awaited<ReturnType<typeof getWebSocketClient>> | null = null

    getWebSocketClient()
      .then((wsClient) => {
        client = wsClient
        setConnected(true)

        const unsubscribe = client.onMessage((message) => {
          setEvent(message)

          // Auto-handle queue updates
          if (message.type === 'queue:update') {
            // Queue updates trigger refetch
          }
        })

        client.onClose(() => {
          setConnected(false)
        })

        return () => {
          unsubscribe()
        }
      })
      .catch((error) => {
        console.error('Failed to get WebSocket client:', error)
        setConnected(false)
      })

    return () => {
      client?.disconnect()
    }
  }, [])

  return { connected, event }
}

/**
 * Hook that subscribes to queue updates
 */
export function useQueueUpdates() {
  const [queueState, setQueueState] = useState<QueueUpdateEvent['data'] | null>(null)

  useEffect(() => {
    let client: Awaited<ReturnType<typeof getWebSocketClient>> | null = null

    getWebSocketClient()
      .then((wsClient) => {
        client = wsClient

        const unsubscribe = client.onMessage((message) => {
          if (message.type === 'queue:update') {
            setQueueState(message.data)
          }
        })

        return unsubscribe
      })
      .catch(console.error)

    return () => {
      client?.disconnect()
    }
  }, [])

  return queueState
}

/**
 * Hook that subscribes to agent progress
 */
export function useAgentProgress(agentExecutionId: string) {
  const [progress, setProgress] = useState<AgentProgressEvent['data'] | null>(null)

  useEffect(() => {
    let client: Awaited<ReturnType<typeof getWebSocketClient>> | null = null

    getWebSocketClient()
      .then((wsClient) => {
        client = wsClient

        const unsubscribe = client.onMessage((message) => {
          if (
            message.type === 'agent:progress' &&
            message.data.agent_execution_id === agentExecutionId
          ) {
            setProgress(message.data)
          }
        })

        return unsubscribe
      })
      .catch(console.error)

    return () => {
      client?.disconnect()
    }
  }, [agentExecutionId])

  return progress
}
```

---

## Step 7: Create API Store with Zustand

- [ ] **Step 7.1: Create global app store**

```typescript
// agentsdk/glyphnova/src-frontend/stores/appStore.ts
import { create } from 'zustand'
import type { Scope, QueueState, QueueItem } from '../types/api'

interface AppState {
  // Current scope
  currentScope: Scope | null
  setCurrentScope: (scope: Scope) => void

  // Queue state (from WebSocket)
  queueState: QueueState | null
  setQueueState: (state: QueueState) => void

  // Selected queue item
  selectedQueueItem: QueueItem | null
  setSelectedQueueItem: (item: QueueItem | null) => void

  // UI state
  sidebarOpen: boolean
  setSidebarOpen: (open: boolean) => void

  // Connection status
  runtimeConnected: boolean
  setRuntimeConnected: (connected: boolean) => void
}

export const useAppStore = create<AppState>((set) => ({
  currentScope: null,
  setCurrentScope: (scope) => set({ currentScope: scope }),

  queueState: null,
  setQueueState: (state) => set({ queueState: state }),

  selectedQueueItem: null,
  setSelectedQueueItem: (item) => set({ selectedQueueItem: item }),

  sidebarOpen: true,
  setSidebarOpen: (open) => set({ sidebarOpen: open }),

  runtimeConnected: false,
  setRuntimeConnected: (connected) => set({ runtimeConnected: connected }),
}))
```

---

## Step 8: Update App with API Integration

- [ ] **Step 8.1: Add React Query provider to App.tsx**

```typescript
// agentsdk/glyphnova/src-frontend/App.tsx
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { Header, Sidebar, MainContent } from './components/layout'
import { useRuntimeStatus, useQueue, useQueueUpdates } from './hooks/useApi'
import { useAppStore } from './stores/appStore'
import { useWebSocket } from './hooks/useWebSocket'

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
      retry: 3,
    },
  },
})

function AppContent() {
  const { connected } = useWebSocket()
  const { data: runtimeStatus } = useRuntimeStatus()
  const { data: queue } = useQueue()
  const queueUpdate = useQueueUpdates()
  const setRuntimeConnected = useAppStore((state) => state.setRuntimeConnected)
  const setQueueState = useAppStore((state) => state.setQueueState)

  // Update connection status
  useEffect(() => {
    setRuntimeConnected(connected && runtimeStatus === 'connected')
  }, [connected, runtimeStatus])

  // Update queue state from WebSocket
  useEffect(() => {
    if (queueUpdate) {
      setQueueState(queueUpdate)
    } else if (queue) {
      setQueueState(queue)
    }
  }, [queueUpdate, queue])

  return (
    <div className="min-h-screen bg-gray-900 text-gray-100 flex flex-col">
      <Header />
      <div className="flex flex-1 overflow-hidden">
        <Sidebar />
        <MainContent />
      </div>
    </div>
  )
}

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <AppContent />
    </QueryClientProvider>
  )
}

export default App
```

---

## Step 9: Test API Integration

- [ ] **Step 9.1: Start runtime server (if not running)**

```bash
# In a separate terminal
cd /home/jon/code/yaml-to-rust-agentsdk
cargo run --bin agentsdk-runtime
```

Expected: Runtime server starts on port 8080

- [ ] **Step 9.2: Start Tauri dev server**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk/glyphnova
npm run tauri dev
```

Expected: Tauri window opens, connects to runtime

- [ ] **Step 9.3: Check browser console for WebSocket connection**

Open DevTools (F12) → Console tab

Expected: See "WebSocket connected" message

- [ ] **Step 9.4: Verify API calls work**

Check Network tab in DevTools

Expected: See successful requests to `/api/queue`, `/api/health`, etc.

---

## Step 10: Add Error Handling

- [ ] **Step 10.1: Create error boundary component**

```typescript
// agentsdk/glyphnova/src-frontend/components/ErrorBoundary.tsx
import { Component, ReactNode } from 'react'

interface Props {
  children: ReactNode
}

interface State {
  hasError: boolean
  error?: Error
}

export class ErrorBoundary extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = { hasError: false }
  }

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error }
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
    console.error('Error caught by boundary:', error, errorInfo)
  }

  render() {
    if (this.state.hasError) {
      return (
        <div className="min-h-screen bg-gray-900 text-gray-100 flex items-center justify-center">
          <div className="max-w-md p-6 bg-gray-800 rounded-lg">
            <h1 className="text-2xl font-bold mb-4 text-red-400">Something went wrong</h1>
            <p className="text-gray-400 mb-4">
              {this.state.error?.message || 'An unexpected error occurred'}
            </p>
            <button
              onClick={() => window.location.reload()}
              className="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-lg"
            >
              Reload
            </button>
          </div>
        </div>
      )
    }

    return this.props.children
  }
}
```

- [ ] **Step 10.2: Wrap App with ErrorBoundary**

```typescript
// Update agentsdk/glyphnova/src-frontend/App.tsx
import { ErrorBoundary } from './components/ErrorBoundary'

function App() {
  return (
    <ErrorBoundary>
      <QueryClientProvider client={queryClient}>
        <AppContent />
      </QueryClientProvider>
    </ErrorBoundary>
  )
}
```

---

## Step 11: Add Loading States

- [ ] **Step 11.1: Create loading component**

```typescript
// agentsdk/glyphnova/src-frontend/components/Loading.tsx
export function Loading({ message = 'Loading...' }: { message?: string }) {
  return (
    <div className="flex items-center justify-center h-full">
      <div className="text-center">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto mb-4"></div>
        <p className="text-gray-400">{message}</p>
      </div>
    </div>
  )
}
```

---

## Step 12: Commit

- [ ] **Step 12.1: Stage and commit changes**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk
git add glyphnova/src/ glyphnova/src-frontend/
git commit -m "feat(ui): implement shared backend API integration

- Connect Tauri frontend to runtime REST API
- Implement WebSocket client for real-time updates
- Create TypeScript types for all API endpoints
- Add React Query hooks for data fetching
- Create global app store with Zustand
- Add error boundary and loading states
- Support queue, workflow, artifact, and metrics APIs

ADR Compliance:
- UI uses same backend API as CLI
- No separate API or database
- All state comes from runtime

Task: 01-shared-backend-api
Part of: Phase 3 - Glyphnova UI"
```

---

## Success Criteria

- [ ] Frontend can query queue state via REST API
- [ ] WebSocket connection established and receiving updates
- [ ] React Query hooks work correctly
- [ ] Error handling displays user-friendly messages
- [ ] Loading states shown during async operations
- [ ] Runtime connection status displayed
- [ ] API types match OpenAPI specification from Phase 2

---

## Next Steps

After completing this task, proceed to **Task 02: Queue Visualization** to build the queue display component.
