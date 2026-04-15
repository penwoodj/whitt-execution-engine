# Task 08: Quality Metrics Dashboard

**Estimated Time:** 3 days
**Priority:** High - Monitor workflow quality over time

**Goal:** Build metrics dashboard showing quality trends, regression detection, benchmark results, and alerts using Recharts.

**ADR Compliance:**
- Metrics come from runtime metrics system
- No separate metrics storage in UI
- Real-time updates via WebSocket/polling

**Files:**
- Create: `agentsdk/glyphnova/src-frontend/components/metrics/QualityMetricsDashboard.tsx`
- Create: `agentsdk/glyphnova/src-frontend/components/metrics/MetricsChart.tsx`
- Create: `agentsdk/glyphnova/src-frontend/components/metrics/AlertPanel.tsx`
- Create: `agentsdk/glyphnova/src-frontend/components/metrics/BenchmarkResults.tsx`

---

## Step 1: Install Recharts

- [ ] **Step 1.1: Install recharts**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk/glyphnova
npm install recharts date-fns
```

Expected: Packages install successfully

---

## Step 2: Create Metrics Chart Component

- [ ] **Step 2.1: Create MetricsChart component**

```typescript
// agentsdk/glyphnova/src-frontend/components/metrics/MetricsChart.tsx
import {
  LineChart,
  Line,
  AreaChart,
  Area,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from 'recharts'
import type { QualityMetrics, MetricTrend } from '../../types/api'

interface MetricsChartProps {
  data: MetricTrend[]
  metric: keyof QualityMetrics
  title: string
  color: string
}

export function MetricsChart({ data, metric, title, color }: MetricsChartProps) {
  return (
    <div className="h-64">
      <h3 className="text-sm font-semibold text-gray-400 mb-2">{title}</h3>
      <ResponsiveContainer width="100%" height="100%">
        <AreaChart data={data}>
          <defs>
            <linearGradient id={`gradient-${color}`} x1="0" y1="0" x2="0" y2="1">
              <stop offset="5%" stopColor={color} stopOpacity={0.3} />
              <stop offset="95%" stopColor={color} stopOpacity={0} />
            </linearGradient>
          </defs>
          <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
          <XAxis
            dataKey="timestamp"
            tickFormatter={(value) => new Date(value).toLocaleTimeString()}
            stroke="#9ca3af"
          />
          <YAxis stroke="#9ca3af" />
          <Tooltip
            contentStyle={{
              backgroundColor: '#1f2937',
              border: '1px solid #374151',
              borderRadius: '0.5rem',
            }}
            labelFormatter={(value) => new Date(value).toLocaleString()}
          />
          <Area
            type="monotone"
            dataKey="value"
            stroke={color}
            fillOpacity={1}
            fill={`url(#gradient-${color})`}
          />
        </AreaChart>
      </ResponsiveContainer>
    </div>
  )
}
```

---

## Step 3: Create Success Rate Chart

- [ ] **Step 3.1: Create SuccessRateChart component**

```typescript
// agentsdk/glyphnova/src-frontend/components/metrics/SuccessRateChart.tsx
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from 'recharts'
import type { QualityMetrics } from '../../types/api'

interface SuccessRateChartProps {
  data: QualityMetrics[]
}

export function SuccessRateChart({ data }: SuccessRateChartProps) {
  const chartData = data.map((metric) => ({
    timestamp: metric.timestamp,
    successRate: metric.success_rate * 100,
  }))

  return (
    <div className="h-64">
      <h3 className="text-sm font-semibold text-gray-400 mb-2">Success Rate (%)</h3>
      <ResponsiveContainer width="100%" height="100%">
        <LineChart data={chartData}>
          <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
          <XAxis
            dataKey="timestamp"
            tickFormatter={(value) => new Date(value).toLocaleTimeString()}
            stroke="#9ca3af"
          />
          <YAxis
            domain={[0, 100]}
            stroke="#9ca3af"
            tickFormatter={(value) => `${value}%`}
          />
          <Tooltip
            contentStyle={{
              backgroundColor: '#1f2937',
              border: '1px solid #374151',
              borderRadius: '0.5rem',
            }}
            labelFormatter={(value) => new Date(value).toLocaleString()}
            formatter={(value) => [`${Number(value).toFixed(2)}%`, 'Success Rate']}
          />
          <Legend />
          <Line
            type="monotone"
            dataKey="successRate"
            stroke="#22c55e"
            strokeWidth={2}
            dot={false}
          />
        </LineChart>
      </ResponsiveContainer>
    </div>
  )
}
```

---

## Step 4: Create Duration Chart

- [ ] **Step 4.1: Create DurationChart component**

```typescript
// agentsdk/glyphnova/src-frontend/components/metrics/DurationChart.tsx
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from 'recharts'
import type { QualityMetrics } from '../../types/api'

interface DurationChartProps {
  data: QualityMetrics[]
}

export function DurationChart({ data }: DurationChartProps) {
  const chartData = data.map((metric) => ({
    timestamp: metric.timestamp,
    duration: metric.average_duration_ms / 1000, // Convert to seconds
  }))

  return (
    <div className="h-64">
      <h3 className="text-sm font-semibold text-gray-400 mb-2">Average Duration (s)</h3>
      <ResponsiveContainer width="100%" height="100%">
        <LineChart data={chartData}>
          <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
          <XAxis
            dataKey="timestamp"
            tickFormatter={(value) => new Date(value).toLocaleTimeString()}
            stroke="#9ca3af"
          />
          <YAxis stroke="#9ca3af" tickFormatter={(value) => `${value}s`} />
          <Tooltip
            contentStyle={{
              backgroundColor: '#1f2937',
              border: '1px solid #374151',
              borderRadius: '0.5rem',
            }}
            labelFormatter={(value) => new Date(value).toLocaleString()}
            formatter={(value) => [`${Number(value).toFixed(2)}s`, 'Duration']}
          />
          <Legend />
          <Line
            type="monotone"
            dataKey="duration"
            stroke="#3b82f6"
            strokeWidth={2}
            dot={false}
          />
        </LineChart>
      </ResponsiveContainer>
    </div>
  )
}
```

---

## Step 5: Create Error Rate Chart

- [ ] **Step 5.1: Create ErrorRateChart component**

```typescript
// agentsdk/glyphnova/src-frontend/components/metrics/ErrorRateChart.tsx
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from 'recharts'
import type { QualityMetrics } from '../../types/api'

interface ErrorRateChartProps {
  data: QualityMetrics[]
}

export function ErrorRateChart({ data }: ErrorRateChartProps) {
  const chartData = data.map((metric) => ({
    timestamp: metric.timestamp,
    errorRate: metric.error_rate * 100,
  }))

  return (
    <div className="h-64">
      <h3 className="text-sm font-semibold text-gray-400 mb-2">Error Rate (%)</h3>
      <ResponsiveContainer width="100%" height="100%">
        <LineChart data={chartData}>
          <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
          <XAxis
            dataKey="timestamp"
            tickFormatter={(value) => new Date(value).toLocaleTimeString()}
            stroke="#9ca3af"
          />
          <YAxis
            domain={[0, 100]}
            stroke="#9ca3af"
            tickFormatter={(value) => `${value}%`}
          />
          <Tooltip
            contentStyle={{
              backgroundColor: '#1f2937',
              border: '1px solid #374151',
              borderRadius: '0.5rem',
            }}
            labelFormatter={(value) => new Date(value).toLocaleString()}
            formatter={(value) => [`${Number(value).toFixed(2)}%`, 'Error Rate']}
          />
          <Legend />
          <Line
            type="monotone"
            dataKey="errorRate"
            stroke="#ef4444"
            strokeWidth={2}
            dot={false}
          />
        </LineChart>
      </ResponsiveContainer>
    </div>
  )
}
```

---

## Step 6: Create Throughput Chart

- [ ] **Step 6.1: Create ThroughputChart component**

```typescript
// agentsdk/glyphnova/src-frontend/components/metrics/ThroughputChart.tsx
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from 'recharts'
import type { QualityMetrics } from '../../types/api'

interface ThroughputChartProps {
  data: QualityMetrics[]
}

export function ThroughputChart({ data }: ThroughputChartProps) {
  const chartData = data.map((metric) => ({
    timestamp: metric.timestamp,
    throughput: metric.throughput,
  }))

  return (
    <div className="h-64">
      <h3 className="text-sm font-semibold text-gray-400 mb-2">Throughput (workflows/hour)</h3>
      <ResponsiveContainer width="100%" height="100%">
        <BarChart data={chartData}>
          <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
          <XAxis
            dataKey="timestamp"
            tickFormatter={(value) => new Date(value).toLocaleTimeString()}
            stroke="#9ca3af"
          />
          <YAxis stroke="#9ca3af" />
          <Tooltip
            contentStyle={{
              backgroundColor: '#1f2937',
              border: '1px solid #374151',
              borderRadius: '0.5rem',
            }}
            labelFormatter={(value) => new Date(value).toLocaleString()}
            formatter={(value) => [value, 'Throughput']}
          />
          <Legend />
          <Bar dataKey="throughput" fill="#8b5cf6" />
        </BarChart>
      </ResponsiveContainer>
    </div>
  )
}
```

---

## Step 7: Create Alert Panel Component

- [ ] **Step 7.1: Create AlertPanel component**

```typescript
// agentsdk/glyphnova/src-frontend/components/metrics/AlertPanel.tsx
import { useState } from 'react'

interface Alert {
  id: string
  type: 'warning' | 'error' | 'info'
  message: string
  timestamp: string
  resolved?: boolean
}

interface AlertPanelProps {
  alerts: Alert[]
  onDismiss: (alertId: string) => void
}

export function AlertPanel({ alerts, onDismiss }: AlertPanelProps) {
  const [filter, setFilter] = useState<'all' | 'unresolved'>('all')

  const filteredAlerts = alerts.filter((alert) =>
    filter === 'unresolved' ? !alert.resolved : true
  )

  const typeColors = {
    warning: 'border-yellow-500 bg-yellow-900/20',
    error: 'border-red-500 bg-red-900/20',
    info: 'border-blue-500 bg-blue-900/20',
  }

  const typeIcons = {
    warning: '⚠️',
    error: '🔴',
    info: 'ℹ️',
  }

  return (
    <div className="h-full flex flex-col">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-semibold">Alerts</h3>
        <div className="flex gap-2">
          <button
            onClick={() => setFilter('all')}
            className={`px-3 py-1 text-xs rounded-lg ${
              filter === 'all'
                ? 'bg-blue-600 text-white'
                : 'bg-gray-800 text-gray-400 hover:bg-gray-700'
            }`}
          >
            All ({alerts.length})
          </button>
          <button
            onClick={() => setFilter('unresolved')}
            className={`px-3 py-1 text-xs rounded-lg ${
              filter === 'unresolved'
                ? 'bg-blue-600 text-white'
                : 'bg-gray-800 text-gray-400 hover:bg-gray-700'
            }`}
          >
            Unresolved ({alerts.filter((a) => !a.resolved).length})
          </button>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto space-y-2">
        {filteredAlerts.length === 0 ? (
          <div className="text-center text-gray-400 py-8">No alerts</div>
        ) : (
          filteredAlerts.map((alert) => (
            <div
              key={alert.id}
              className={`p-3 border-l-4 rounded-r-lg ${
                alert.resolved
                  ? 'opacity-50 bg-gray-800'
                  : typeColors[alert.type]
              }`}
            >
              <div className="flex items-start justify-between">
                <div className="flex-1">
                  <div className="flex items-center gap-2 mb-1">
                    <span>{typeIcons[alert.type]}</span>
                    <span className="text-sm font-medium">
                      {alert.type.charAt(0).toUpperCase() + alert.type.slice(1)}
                    </span>
                  </div>
                  <p className="text-sm text-gray-300">{alert.message}</p>
                  <p className="text-xs text-gray-500 mt-1">
                    {new Date(alert.timestamp).toLocaleString()}
                  </p>
                </div>
                {!alert.resolved && (
                  <button
                    onClick={() => onDismiss(alert.id)}
                    className="text-gray-400 hover:text-white transition-colors"
                  >
                    ×
                  </button>
                )}
              </div>
            </div>
          ))
        )}
      </div>
    </div>
  )
}
```

---

## Step 8: Create Benchmark Results Component

- [ ] **Step 8.1: Create BenchmarkResults component**

```typescript
// agentsdk/glyphnova/src-frontend/components/metrics/BenchmarkResults.tsx

interface BenchmarkResult {
  id: string
  name: string
  baseline: number
  current: number
  unit: string
  threshold: number
  status: 'pass' | 'fail' | 'warning'
}

interface BenchmarkResultsProps {
  results: BenchmarkResult[]
}

export function BenchmarkResults({ results }: BenchmarkResultsProps) {
  return (
    <div className="h-full flex flex-col">
      <h3 className="text-lg font-semibold mb-4">Benchmark Results</h3>

      <div className="flex-1 overflow-y-auto space-y-3">
        {results.map((result) => {
          const percentage = (result.current / result.baseline) * 100
          const statusColors = {
            pass: 'text-green-400',
            fail: 'text-red-400',
            warning: 'text-yellow-400',
          }

          return (
            <div key={result.id} className="bg-gray-800 rounded-lg p-4">
              <div className="flex items-center justify-between mb-2">
                <h4 className="font-medium">{result.name}</h4>
                <span className={`text-sm ${statusColors[result.status]}`}>
                  {result.status.toUpperCase()}
                </span>
              </div>

              <div className="space-y-2">
                <div className="flex justify-between text-sm">
                  <span className="text-gray-400">Baseline:</span>
                  <span>{result.baseline} {result.unit}</span>
                </div>
                <div className="flex justify-between text-sm">
                  <span className="text-gray-400">Current:</span>
                  <span>{result.current} {result.unit}</span>
                </div>
                <div className="flex justify-between text-sm">
                  <span className="text-gray-400">Change:</span>
                  <span className={percentage > 100 ? 'text-red-400' : 'text-green-400'}>
                    {percentage.toFixed(1)}%
                  </span>
                </div>
              </div>

              {/* Progress bar */}
              <div className="mt-3 h-2 bg-gray-700 rounded-full overflow-hidden">
                <div
                  className={`h-full transition-all ${
                    percentage > result.threshold
                      ? 'bg-red-500'
                      : percentage > result.threshold * 0.9
                      ? 'bg-yellow-500'
                      : 'bg-green-500'
                  }`}
                  style={{ width: `${Math.min(percentage, 100)}%` }}
                />
              </div>
            </div>
          )
        })}
      </div>
    </div>
  )
}
```

---

## Step 9: Create Quality Metrics Dashboard Component

- [ ] **Step 9.1: Create QualityMetricsDashboard component**

```typescript
// agentsdk/glyphnova/src-frontend/components/metrics/QualityMetricsDashboard.tsx
import { useState } from 'react'
import { useQualityMetrics, useMetricsSummary } from '../../hooks/useApi'
import { SuccessRateChart } from './SuccessRateChart'
import { DurationChart } from './DurationChart'
import { ErrorRateChart } from './ErrorRateChart'
import { ThroughputChart } from './ThroughputChart'
import { AlertPanel } from './AlertPanel'
import { BenchmarkResults } from './BenchmarkResults'
import type { Alert, BenchmarkResult } from './AlertPanel'

export function QualityMetricsDashboard() {
  const [timeRange, setTimeRange] = useState<'1h' | '6h' | '24h' | '7d'>('1h')
  const { data: metrics, isLoading, error } = useQualityMetrics(timeRange)
  const { data: summary } = useMetricsSummary()

  // Mock alerts and benchmarks (replace with real data)
  const [alerts, setAlerts] = useState<Alert[]>([
    {
      id: '1',
      type: 'error',
      message: 'Success rate dropped below 90% threshold',
      timestamp: new Date().toISOString(),
    },
    {
      id: '2',
      type: 'warning',
      message: 'Average execution time increased by 20%',
      timestamp: new Date(Date.now() - 3600000).toISOString(),
    },
  ])

  const [benchmarks] = useState<BenchmarkResult[]>([
    {
      id: '1',
      name: 'Workflow Duration',
      baseline: 30,
      current: 32,
      unit: 's',
      threshold: 110,
      status: 'warning',
    },
    {
      id: '2',
      name: 'Success Rate',
      baseline: 95,
      current: 97,
      unit: '%',
      threshold: 90,
      status: 'pass',
    },
    {
      id: '3',
      name: 'Memory Usage',
      baseline: 512,
      current: 480,
      unit: 'MB',
      threshold: 110,
      status: 'pass',
    },
  ])

  const handleDismissAlert = (alertId: string) => {
    setAlerts(alerts.map((alert) =>
      alert.id === alertId ? { ...alert, resolved: true } : alert
    ))
  }

  if (isLoading) {
    return <div className="p-8 text-gray-400">Loading metrics...</div>
  }

  if (error) {
    return <div className="p-8 text-red-400">Error loading metrics: {error.message}</div>
  }

  return (
    <div className="h-full flex flex-col">
      {/* Header */}
      <div className="p-4 border-b border-gray-700 flex items-center justify-between">
        <div>
          <h2 className="text-xl font-bold">Quality Metrics Dashboard</h2>
          <p className="text-sm text-gray-400">Monitor workflow quality and performance</p>
        </div>
        <div className="flex gap-2">
          {(['1h', '6h', '24h', '7d'] as const).map((range) => (
            <button
              key={range}
              onClick={() => setTimeRange(range)}
              className={`px-3 py-1 text-sm rounded-lg transition-colors ${
                timeRange === range
                  ? 'bg-blue-600 text-white'
                  : 'bg-gray-800 text-gray-400 hover:bg-gray-700'
              }`}
            >
              {range}
            </button>
          ))}
        </div>
      </div>

      {/* Summary Cards */}
      {summary && (
        <div className="p-4 grid grid-cols-4 gap-4">
          <div className="bg-gray-800 rounded-lg p-4">
            <p className="text-sm text-gray-400 mb-1">Total Workflows</p>
            <p className="text-2xl font-bold">{summary.total_workflows}</p>
          </div>
          <div className="bg-gray-800 rounded-lg p-4">
            <p className="text-sm text-gray-400 mb-1">Success Rate</p>
            <p className="text-2xl font-bold text-green-400">
              {((summary.successful_workflows / summary.total_workflows) * 100).toFixed(1)}%
            </p>
          </div>
          <div className="bg-gray-800 rounded-lg p-4">
            <p className="text-sm text-gray-400 mb-1">Avg Duration</p>
            <p className="text-2xl font-bold">
              {(summary.average_duration_ms / 1000).toFixed(1)}s
            </p>
          </div>
          <div className="bg-gray-800 rounded-lg p-4">
            <p className="text-sm text-gray-400 mb-1">Total Artifacts</p>
            <p className="text-2xl font-bold">{summary.total_artifacts}</p>
          </div>
        </div>
      )}

      {/* Metrics Grid */}
      <div className="flex-1 overflow-y-auto p-4">
        <div className="grid grid-cols-2 gap-4 mb-4">
          {metrics && (
            <>
              <SuccessRateChart data={metrics} />
              <DurationChart data={metrics} />
              <ErrorRateChart data={metrics} />
              <ThroughputChart data={metrics} />
            </>
          )}
        </div>

        <div className="grid grid-cols-2 gap-4">
          <AlertPanel alerts={alerts} onDismiss={handleDismissAlert} />
          <BenchmarkResults results={benchmarks} />
        </div>
      </div>
    </div>
  )
}
```

---

## Step 10: Add Dashboard to Main Content

- [ ] **Step 10.1: Update MainContent to include dashboard**

```typescript
// Add to agentsdk/glyphnova/src-frontend/components/layout/MainContent.tsx
import { QualityMetricsDashboard } from '../metrics/QualityMetricsDashboard'

// Add to renderContent function
case 'metrics':
  return <QualityMetricsDashboard />
```

- [ ] **Step 10.2: Add metrics button to navigation**

```typescript
// Add metrics zoom level to ZoomControls
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
```

---

## Step 11: Test Metrics Dashboard

- [ ] **Step 11.1: Run Tauri dev server**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk/glyphnova
npm run tauri dev
```

- [ ] **Step 11.2: Verify dashboard displays**

Expected:
- Summary cards show key metrics
- Charts render correctly
- Time range filter works

- [ ] **Step 11.3: Test charts**

Expected:
- Success rate chart displays
- Duration chart displays
- Error rate chart displays
- Throughput chart displays
- Charts update when time range changes

- [ ] **Step 11.4: Test alerts**

Expected:
- Alert panel shows alerts
- Filter works (all/unresolved)
- Dismiss alerts works

- [ ] **Step 11.5: Test benchmarks**

Expected:
- Benchmark results display
- Progress bars show status
- Color coding is correct

---

## Step 12: Commit

- [ ] **Step 12.1: Stage and commit changes**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk
git add glyphnova/src-frontend/components/metrics/
git commit -m "feat(ui): implement quality metrics dashboard with charts

- Create metrics charts (success rate, duration, error rate, throughput)
- Build alert panel with filtering and dismissal
- Implement benchmark results display
- Add time range selector (1h, 6h, 24h, 7d)
- Display summary cards with key metrics
- Use Recharts for visualizations

ADR Compliance:
- Metrics come from runtime metrics system
- No separate metrics storage in UI
- Real-time updates via WebSocket/polling

Task: 08-quality-metrics-dashboard
Part of: Phase 3 - Glyphnova UI"
```

---

## Success Criteria

- [ ] Dashboard displays with all charts
- [ ] Time range filter works
- [ ] Charts render correctly with data
- [ ] Alerts display and can be dismissed
- [ ] Benchmark results show correctly
- [ ] Summary cards display key metrics
- [ ] Charts update on time range change
- [ ] UI is responsive and performant

---

## Next Steps

All tasks for Phase 3 are complete! Proceed to validation and testing.

See:
- `validation/criteria.md` - Validation criteria
- `validation/adr-compliance.md` - ADR-0004 compliance checks
- `tests/mock-strategies.md` - Mock strategies for testing
