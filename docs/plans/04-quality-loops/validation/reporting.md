# Report Generation Validation Criteria

## ADR-0005 Compliance (CRITICAL)

### Report Generation Requirements

- [ ] Reports generated automatically on quality loop completion
- [ ] Reports stored in /workspace/reports/ directory (per ADR-0005)
- [ ] Report generation is NOT optional for quality loops
- [ ] Report types: quality, benchmark, improvement, summary (all required)
- [ ] Report metadata includes workflow version and policy snapshot
- [ ] Report provenance tracked (who generated, when, from what data)
- [ ] Reports include actionable insights (not just data dumps)
- [ ] Reports validate against schema before storage
- [ ] Reports are versioned for reproducibility

### Integration with Quality Loops

- [ ] Quality loops emit report generation events
- [ ] Report generation triggered on loop convergence or failure
- [ ] Report includes loop state and iteration history
- [ ] Report includes verification results and repair attempts
- [ ] Report includes convergence metrics and quality scores
- [ ] Report includes token usage and cost estimates
- [ ] Report includes scheduler job metadata

## Report Accuracy

### Report Structure

- [ ] Report.id is unique UUID
- [ ] Report.report_type correctly identifies report type
- [ ] Report.title is non-empty
- [ ] Report.summary is non-empty
- [ ] Report.sections ordered by order field
- [ ] Report.generated_at is recent timestamp
- [ ] Report.workflow_version is git commit hash or tag
- [ ] Report.policy_snapshot is valid JSON
- [ ] Report.metadata includes provenance fields

### Content Accuracy

- [ ] SectionContent.Text contains correct text
- [ ] SectionContent.Table has correct headers and rows
- [ ] SectionContent.Chart has correct data points
- [ ] SectionContent.Metrics has correct names and values
- [ ] All sections referenced in report exist
- [ ] Section order matches specification
- [ ] No duplicate sections
- [ ] All required sections present per report type

### Insights Quality

- [ ] Insights include insight_type
- [ ] Insights include priority (Low/Medium/High/Critical)
- [ ] Insights include descriptive text
- [ ] Insights include actionable action when applicable
- [ ] Insights are relevant to report data
- [ ] Insights are data-driven (not speculative)
- [ ] Insights include confidence score
- [ ] Insights include timestamp when data point observed
- [ ] Insights avoid vague language ("might be", "could be")
- [ ] Insights prioritize by impact and feasibility

## Report Types

### Quality Report

- [ ] Includes quality score trends
- [ ] Includes verification pass rates
- [ ] Includes convergence rate
- [ ] Includes iteration counts
- [ ] Includes error and warning summaries
- [ ] Includes top failing verifiers
- [ ] Includes repair success rate
- [ ] Includes benchmark metadata (workflow version, policy snapshot)
- [ ] Includes backend information (model, version, parameters)
- [ ] Includes file type breakdown

### Benchmark Report

- [ ] Includes benchmark suite metadata
- [ ] Includes individual benchmark results
- [ ] Includes statistical analysis (mean, median, std dev, p95, p99)
- [ ] Includes comparison to baseline
- [ ] Includes regression detection
- [ ] Includes performance trends over time
- [ ] Includes benchmark provenance (who ran, when, on what hardware)
- [ ] Includes significance test results
- [ ] Includes outlier detection
- [ ] Includes benchmark curation notes

### Improvement Report

- [ ] Includes gap analysis (baseline vs target)
- [ ] Includes priority recommendations
- [ ] Includes estimated impact
- [ ] Includes implementation effort estimates
- [ ] Includes dependencies and blockers
- [ ] Includes risk assessment
- [ ] Includes cost-benefit analysis
- [ ] Includes timeline estimates
- [ ] Includes success criteria
- [ ] Includes success metrics

### Summary Report

- [ ] Aggregates all report types
- [ ] Includes executive summary
- [ ] Includes key metrics across all dimensions
- [ ] Includes top insights (max 5, highest priority)
- [ ] Includes recommendations summary
- [ ] Includes next steps
- [ ] Includes overall health score
- [ ] Includes trend indicators (up/down/stable)
- [ ] Includes drill-down links to detailed reports
- [ ] Includes glossary of terms

## Format Validation

### HTML Generator

- [ ] Generated HTML is well-formed
- [ ] Includes DOCTYPE declaration
- [ ] Includes CSS for styling
- [ ] Includes report title
- [ ] Includes summary section
- [ ] Includes all report sections
- [ ] Includes insights section
- [ ] All placeholders replaced
- [ ] No template errors
- [ ] Responsive design (mobile-friendly)
- [ ] Accessible (ARIA labels, semantic HTML)
- [ ] Print-friendly styles
- [ ] Interactive elements (expandable sections, sortable tables)
- [ ] Chart.js or D3.js for visualizations
- [ ] Navigation table of contents

### PDF Generator

- [ ] Generated PDF is valid
- [ ] Includes report title
- [ ] Includes summary
- [ ] Includes sections (simplified)
- [ ] PDF dimensions correct (A4)
- [ ] Font readable (minimum 10pt)
- [ ] No truncation of content
- [ ] Page numbers included
- [ ] Table of contents included
- [ ] Hyperlinks preserved (where applicable)
- [ ] Images embedded correctly
- [ ] Color scheme consistent
- [ ] Headers and footers included
- [ ] Margins and padding appropriate

### JSON Generator

- [ ] Generated JSON is valid
- [ ] Includes all report fields
- [ ] Properly nested
- [ ] No data loss in serialization
- [ ] Can be deserialized back to Report
- [ ] Arrays sorted consistently
- [ ] Null fields explicit (not missing)
- [ ] Numbers use appropriate precision
- [ ] Dates in ISO 8601 format
- [ ] UUIDs in standard format
- [ ] Unicode properly encoded
- [ ] No trailing commas

### Markdown Generator

- [ ] Generated Markdown is valid
- [ ] Headers use proper # syntax
- [ ] Tables use proper | syntax
- [ ] Lists use proper - syntax
- [ ] Bold uses proper ** syntax
- [ ] Code blocks use proper ``` syntax
- [ ] Links use proper []() syntax
- [ ] Images use proper ![]() syntax
- [ ] Line breaks appropriate
- [ ] Can be rendered by standard Markdown renderers
- [ ] GFM-compatible (GitHub Flavored Markdown)
- [ ] Math equations use $ or $$ syntax
- [ ] HTML tags escaped where necessary

## Data Collection Validation

### Metric Store Integration

- [ ] Queries metric store for historical data
- [ ] Queries metric store for current run data
- [ ] Aggregates data across multiple runs
- [ ] Handles missing metric data gracefully
- [ ] Validates metric names and units
- [ ] Handles data type conversions
- [ ] Caches frequently accessed metrics

### Log Aggregation

- [ ] Parses structured log files
- [ ] Extracts quality loop logs
- [ ] Extracts benchmark execution logs
- [ ] Extracts scheduler job logs
- [ ] Extracts LLM call logs
- [ ] Aggregates logs by time window
- [ ] Handles log rotation and compression
- [ ] Filters logs by severity level

### Benchmark Data

- [ ] Retrieves benchmark results from storage
- [ ] Includes provenance metadata
- [ ] Includes execution context
- [ ] Includes statistical analysis results
- [ ] Handles partial benchmark sets
- [ ] Validates benchmark schema
- [ ] Includes outlier detection results

## Query Interface Validation

### Search Capabilities

- [ ] Reports searchable by report_id
- [ ] Reports searchable by report_type
- [ ] Reports searchable by generated_at range
- [ ] Reports searchable by workflow_version
- [ ] Reports searchable by keywords (full-text)
- [ ] Reports filterable by backend_type
- [ ] Reports filterable by model
- [ ] Reports filterable by file_type

### Filter Capabilities

- [ ] Filter by quality score range
- [ ] Filter by convergence rate
- [ ] Filter by error count
- [ ] Filter by iteration count
- [ ] Filter by cost range
- [ ] Filter by priority of insights
- [ ] Filter by insight type
- [ ] Combine filters with AND/OR logic

### Sort Capabilities

- [ ] Sort by generated_at (asc/desc)
- [ ] Sort by quality score (asc/desc)
- [ ] Sort by convergence rate (asc/desc)
- [ ] Sort by iteration count (asc/desc)
- [ ] Sort by cost (asc/desc)
- [ ] Sort by relevance (full-text search)
- [ ] Multi-column sort supported

## Visualization Verification

### Chart Types

- [ ] Line charts for trends over time
- [ ] Bar charts for categorical comparisons
- [ ] Scatter plots for correlation analysis
- [ ] Heatmaps for matrix data
- [ ] Pie charts for distribution
- [ ] Histograms for frequency distribution
- [ ] Box plots for statistical distribution

### Chart Accuracy

- [ ] Data points plotted correctly
- [ ] Axes labeled with units
- [ ] Legends present and accurate
- [ ] Colors distinguishable (colorblind-friendly)
- [ ] Tooltips show exact values on hover
- [ ] Grid lines aid readability
- [ ] Scales appropriate (linear, logarithmic)
- [ ] Error bars included where applicable
- [ ] Annotations highlight key points
- [ ] Export to PNG/SVG supported

### Dashboard Widgets

- [ ] KPI cards (key performance indicators)
- [ ] Sparklines for trend indicators
- [ ] Progress bars for targets
- [ ] Status badges (green/yellow/red)
- [ ] Gauges for percentage metrics
- [ ] Tables with sorting and pagination
- [ ] Filters and search inputs
- [ ] Date range selectors

## Storage and Archiving

### Storage Validation

- [ ] Reports stored in /workspace/reports/
- [ ] Filenames follow convention: {report_type}_{timestamp}.json
- [ ] Reports indexed by metadata
- [ ] Storage atomic (no partial writes)
- [ ] Storage handles concurrent writes
- [ ] Storage disk usage monitored
- [ ] Storage quota enforced (optional)
- [ ] Storage backup scheduled

### Archiving Validation

- [ ] Old reports archived after retention period (default: 90 days)
- [ ] Archive format: compressed tarball
- [ ] Archive includes all report formats (HTML, PDF, JSON, Markdown)
- [ ] Archive index maintained
- [ ] Archived reports searchable (via index)
- [ ] Archived reports restorable on demand
- [ ] Archive integrity validated
- [ ] Archive deduplicated (by content hash)

### Retrieval Validation

- [ ] Report retrieval by ID fast (< 100ms)
- [ ] Report retrieval by filter efficient
- [ ] Report streaming for large reports
- [ ] Report metadata retrieval separate from content
- [ ] Report thumbnails for preview (PDF/HTML)
- [ ] Report version history accessible

## Mock Strategies

### Mock Metric Store

```rust
struct MockMetricStore {
    data: HashMap<String, Vec<Metric>>,
    latency_ms: u64,
    fail_after: Option<usize>,
}

// Test: report generation with mock metric store
#[test]
fn test_generate_report_with_mock_store() {
    let mock = MockMetricStore {
        data: vec![
            ("quality_score".to_string(), vec![Metric::new(0.85), Metric::new(0.90)]),
        ].into_iter().collect(),
        latency_ms: 50,
        fail_after: None,
    };

    let report = generate_quality_report(&mock).unwrap();
    assert_eq!(report.sections.len(), 5);
}
```

### Mock Report Templates

```rust
struct MockTemplateStore {
    templates: HashMap<String, Template>,
    latency_ms: u64,
    fail_after: Option<usize>,
}

// Test: HTML generation with mock templates
#[test]
fn test_html_generation_with_mock_templates() {
    let mock = MockTemplateStore {
        templates: vec![
            ("quality_report".to_string(), Template::from_str("{{ title }}").unwrap()),
        ].into_iter().collect(),
        latency_ms: 10,
        fail_after: None,
    };

    let html = generate_html(&mock, &report).unwrap();
    assert!(html.contains("<h1>Quality Report</h1>"));
}
```

### Mock Chart Generator

```rust
struct MockChartGenerator {
    charts: HashMap<String, String>,
    latency_ms: u64,
}

// Test: chart generation with mock generator
#[test]
fn test_chart_generation_with_mock() {
    let mock = MockChartGenerator {
        charts: vec![
            ("quality_trend".to_string(), "<canvas></canvas>".to_string()),
        ].into_iter().collect(),
        latency_ms: 20,
    };

    let charts = generate_charts(&mock, &report).unwrap();
    assert!(charts.len() > 0);
}
```

## Cargo Test Commands

### Unit Tests

```bash
# Run all report generation unit tests
cargo test --lib reporting::tests::unit -- --nocapture

# Test report creation
cargo test --lib reporting::report::tests::create_report -- --exact

# Test HTML generator
cargo test --lib reporting::generator::html::tests::generate_html -- --exact

# Test PDF generator
cargo test --lib reporting::generator::pdf::tests::generate_pdf -- --exact

# Test JSON generator
cargo test --lib reporting::generator::json::tests::generate_json -- --exact

# Test Markdown generator
cargo test --lib reporting::generator::markdown::tests::generate_markdown -- --exact

# Test insights engine
cargo test --lib reporting::insights::tests::generate_insights -- --exact

# Expected output:
# test reporting::report::tests::create_report ... ok
# test reporting::generator::html::tests::generate_html ... ok
# test reporting::generator::pdf::tests::generate_pdf ... ok
# test reporting::generator::json::tests::generate_json ... ok
# test reporting::generator::markdown::tests::generate_markdown ... ok
# test reporting::insights::tests::generate_insights ... ok
```

### Integration Tests

```bash
# Generate quality report from benchmark data
cargo test --test reporting_integration test_quality_report_generation -- --exact --nocapture

# Generate benchmark report from suite data
cargo test --test reporting_integration test_benchmark_report_generation -- --exact

# Generate improvement report from gap analysis
cargo test --test reporting_integration test_improvement_report_generation -- --exact

# Generate summary report from all data
cargo test --test reporting_integration test_summary_report_generation -- --exact

# Test all formats produce equivalent information
cargo test --test reporting_integration test_format_equivalence -- --exact

# Expected output:
# test reporting_integration::test_quality_report_generation ... ok
# test reporting_integration::test_benchmark_report_generation ... ok
# test reporting_integration::test_improvement_report_generation ... ok
# test reporting_integration::test_summary_report_generation ... ok
# test reporting_integration::test_format_equivalence ... ok
```

### Mock Strategy Tests

```bash
# Test with mock metric store
cargo test --test mock_strategies test_report_with_mock_metric_store -- --exact

# Test with mock templates
cargo test --test mock_strategies test_report_with_mock_templates -- --exact

# Test with mock chart generator
cargo test --test mock_strategies test_report_with_mock_charts -- --exact

# Expected output:
# test mock_strategies::test_report_with_mock_metric_store ... ok
# test mock_strategies::test_report_with_mock_templates ... ok
# test mock_strategies::test_report_with_mock_charts ... ok
```

### Query Interface Tests

```bash
# Test search functionality
cargo test --test query_interface test_search_reports -- --exact --nocapture

# Test filter functionality
cargo test --test query_interface test_filter_reports -- --exact

# Test sort functionality
cargo test --test query_interface test_sort_reports -- --exact

# Expected output:
# test query_interface::test_search_reports ... ok
# test query_interface::test_filter_reports ... ok
# test query_interface::test_sort_reports ... ok
```

### Storage Tests

```bash
# Test report storage
cargo test --test storage test_store_report -- --exact

# Test report retrieval
cargo test --test storage test_retrieve_report -- --exact

# Test report archiving
cargo test --test storage test_archive_report -- --exact

# Expected output:
# test storage::test_store_report ... ok
# test storage::test_retrieve_report ... ok
# test storage::test_archive_report ... ok
```

## Log Verification Patterns

### Report Generation Log Pattern

```json
{
  "level": "info",
  "target": "reporting::generator",
  "event": "report_generation_start",
  "report_id": "<uuid>",
  "report_type": "quality",
  "workflow_version": "abc123",
  "generated_at": "2026-04-07T12:00:00Z"
}
```

**Verification**: `grep '"event":"report_generation_start"' logs/reporting.log | jq '.report_type == "quality"' | grep true`

### HTML Generation Log Pattern

```json
{
  "level": "debug",
  "target": "reporting::generator::html",
  "event": "html_generation_complete",
  "report_id": "<uuid>",
  "output_path": "/workspace/reports/quality_report_20260407_120000.html",
  "duration_ms": 150,
  "file_size_bytes": 524288
}
```

**Verification**: `grep '"event":"html_generation_complete"' logs/reporting.log | jq '.file_size_bytes | tonumber > 0' | grep true`

### PDF Generation Log Pattern

```json
{
  "level": "debug",
  "target": "reporting::generator::pdf",
  "event": "pdf_generation_complete",
  "report_id": "<uuid>",
  "output_path": "/workspace/reports/quality_report_20260407_120000.pdf",
  "duration_ms": 500,
  "page_count": 15
}
```

**Verification**: `grep '"event":"pdf_generation_complete"' logs/reporting.log | jq '.page_count | tonumber >= 1' | grep true`

### Insights Generation Log Pattern

```json
{
  "level": "info",
  "target": "reporting::insights",
  "event": "insights_generated",
  "report_id": "<uuid>",
  "insight_count": 7,
  "priorities": {
    "Critical": 1,
    "High": 2,
    "Medium": 3,
    "Low": 1
  }
}
```

**Verification**: `grep '"event":"insights_generated"' logs/reporting.log | jq '.insight_count | tonumber > 0' | grep true`

### Chart Generation Log Pattern

```json
{
  "level": "debug",
  "target": "reporting::visualization",
  "event": "chart_generated",
  "report_id": "<uuid>",
  "chart_type": "line",
  "chart_title": "Quality Score Trend",
  "data_points": 30,
  "duration_ms": 75
}
```

**Verification**: `grep '"event":"chart_generated"' logs/reporting.log | jq '.chart_type == "line"' | grep true`

### Storage Log Pattern

```json
{
  "level": "info",
  "target": "reporting::storage",
  "event": "report_stored",
  "report_id": "<uuid>",
  "formats": ["json", "html", "pdf", "markdown"],
  "storage_path": "/workspace/reports/",
  "duration_ms": 25
}
```

**Verification**: `grep '"event":"report_stored"' logs/reporting.log | jq '.formats | length == 4' | grep true`

## Testing Criteria

### Unit Tests

- [ ] Report creation with all fields
- [ ] HTML generator produces valid HTML
- [ ] PDF generator produces valid PDF
- [ ] JSON generator produces valid JSON
- [ ] Markdown generator produces valid Markdown
- [ ] Insights engine generates correct insights
- [ ] Trend detection accurate
- [ ] Thresholds triggered correctly
- [ ] Chart generator produces valid SVG/canvas
- [ ] Template rendering correct

### Integration Tests

- [ ] Generate quality report from benchmark data
- [ ] Generate benchmark report from suite data
- [ ] Generate improvement report from gap analysis
- [ ] Generate summary report from all data
- [ ] All formats produce equivalent information
- [ ] Report generation triggered on quality loop completion
- [ ] Report provenance tracked correctly
- [ ] Storage and retrieval of reports
- [ ] Query interface returns correct reports
- [ ] Visualization renders correctly in browser

### Edge Cases

- [ ] Empty benchmark set
- [ ] Single benchmark
- [ ] Very large benchmark set (1000+)
- [ ] No quality trends (flat line)
- [ ] No insights generated
- [ ] All insights same priority
- [ ] Missing template files
- [ ] Invalid data types in metric store
- [ ] Disk full during report storage
- [ ] Concurrent report generation

## Performance Criteria

- [ ] HTML generation < 100ms (typical report)
- [ ] PDF generation < 500ms (typical report)
- [ ] JSON generation < 50ms (typical report)
- [ ] Markdown generation < 50ms (typical report)
- [ ] Insights generation < 100ms (1000 benchmarks)
- [ ] Report generation memory bounded
- [ ] Chart generation < 50ms per chart
- [ ] Report storage < 50ms
- [ ] Report retrieval < 100ms (indexed)
- [ ] Query search < 200ms (full-text search)

## Error Handling

- [ ] ReportError covers all failure modes
- [ ] TransformError covers format-specific errors
- [ ] Missing template returns specific error
- [ ] Invalid template returns specific error
- [ ] Serialization errors handled gracefully
- [ ] Error messages actionable
- [ ] Errors logged with context
- [ ] Partial report generation handled (report what's available)
- [ ] Data validation errors specific to field
- [ ] Storage errors include path and operation

## ADR-0005 Constraint Compliance Summary

| Constraint | Validation | Test Coverage |
|------------|-------------|---------------|
| Automatic report generation | Quality loop triggers report | integration::test_report_on_loop_completion |
| /workspace/reports/ storage | Report path validation | storage::tests::report_path |
| Provenance tracking | Report includes provenance fields | report::tests::provenance_fields |
| Actionable insights | Insights include action field | insights::tests::actionable_insights |
| All report types available | Report type enum validated | report::tests::report_types |
