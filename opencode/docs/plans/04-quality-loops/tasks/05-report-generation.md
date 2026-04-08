# Task 5: Report Generation

**Files:**
- Create: `crates/quality/reports/src/lib.rs`
- Create: `crates/quality/reports/src/types.rs`
- Create: `crates/quality/reports/src/generators/html.rs`
- Create: `crates/quality/reports/src/generators/pdf.rs`
- Create: `crates/quality/reports/src/generators/json.rs`
- Create: `crates/quality/reports/src/generators/markdown.rs`
- Create: `crates/quality/reports/src/insights.rs`
- Create: `crates/quality/reports/Cargo.toml`
- Test: `crates/quality/reports/tests/report_tests.rs`

**Duration:** 1.5 weeks

## Overview

Implement multi-format report generation with actionable insights. Reports include quality trends, benchmark comparisons, and improvement recommendations.

## Architecture

The report system consists of:

1. **Report Types** — Quality, Benchmark, Improvement, Summary reports
2. **Generators** — HTML, PDF, JSON, Markdown output
3. **Insights Engine** — Analyze data to generate actionable recommendations
4. **Templates** — Handlebars templates for HTML/Markdown

---

## Implementation Steps

### Step 1: Crate Setup

- [ ] **Create reports crate with Cargo.toml**
  ```toml
  [package]
  name = "agentsdk-reports"
  version = "0.1.0"
  edition = "2021"

  [dependencies]
  agentsdk-types = { path = "../../types" }
  agentsdk-benchmark = { path = "../benchmark" }
  serde = { version = "1.0", features = ["derive"] }
  serde_json = "1.0"
  thiserror = "1.0"
  tokio = { version = "1.0", features = ["full"] }
  chrono = { version = "0.4", features = ["serde"] }
  handlebars = "5.0"
  printpdf = "0.6"
  plotters = "0.3"
  uuid = { version = "1.0", features = ["v4", "serde"] }
  ```

### Step 2: Define Types

- [ ] **Create types.rs with Report types**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub id: ReportId,
    pub report_type: ReportType,
    pub title: String,
    pub summary: String,
    pub sections: Vec<ReportSection>,
    pub insights: Vec<Insight>,
    pub metadata: ReportMetadata,
    pub generated_at: DateTime<Utc>,
}

pub type ReportId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportType {
    Quality { period: ReportPeriod },
    Benchmark { suite_ids: Vec<SuiteId> },
    Improvement { workflow_ids: Vec<String> },
    Summary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportPeriod {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSection {
    pub id: String,
    pub title: String,
    pub content: SectionContent,
    pub order: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SectionContent {
    Text(String),
    Table(Table),
    Chart(Chart),
    Metrics(Vec<Metric>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chart {
    pub chart_type: ChartType,
    pub data: ChartData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChartType {
    Line,
    Bar,
    Pie,
    Scatter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartData {
    pub labels: Vec<String>,
    pub datasets: Vec<DataSet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSet {
    pub label: String,
    pub data: Vec<f64>,
    pub color: String,
}
```

- [ ] **Write and run tests**
- [ ] **Commit**

### Step 3: Implement HTML Generator

- [ ] **Create generators/html.rs**

```rust
use handlebars::Handlebars;
use super::types::*;

pub struct HtmlGenerator {
    template_engine: Handlebars<'static>,
}

impl HtmlGenerator {
    pub fn new() -> Result<Self, ReportError> {
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("report", include_str!("templates/report.html.hbs"))?;
        handlebars.register_template_string("section", include_str!("templates/section.html.hbs"))?;

        Ok(Self { template_engine: handlebars })
    }

    pub fn generate(&self, report: &Report) -> Result<String, ReportError> {
        let rendered = self.template_engine.render("report", report)?;
        Ok(rendered)
    }
}

impl Default for HtmlGenerator {
    fn default() -> Self {
        Self::new().expect("Failed to create HTML generator")
    }
}
```

- [ ] **Create templates/report.html.hbs**
  ```html
  <!DOCTYPE html>
  <html>
  <head>
      <title>{{title}}</title>
      <style>
          body { font-family: Arial, sans-serif; max-width: 1200px; margin: 0 auto; padding: 20px; }
          h1, h2, h3 { color: #333; }
          .summary { background: #f5f5f5; padding: 15px; border-radius: 5px; }
          .insight { background: #e8f4f8; padding: 10px; margin: 10px 0; border-left: 4px solid #0066cc; }
      </style>
  </head>
  <body>
      <h1>{{title}}</h1>
      <div class="summary">{{summary}}</div>
      {{#each sections}}
          {{> section}}
      {{/each}}
      <h2>Insights</h2>
      {{#each insights}}
          <div class="insight">
              <strong>{{type}}:</strong> {{description}}
              {{#if action}}
                  <br><strong>Action:</strong> {{action}}
              {{/if}}
          </div>
      {{/each}}
  </body>
  </html>
  ```

- [ ] **Write and run tests**
- [ ] **Commit**

### Step 4: Implement PDF Generator

- [ ] **Create generators/pdf.rs**

```rust
use printpdf::*;
use super::types::*;

pub struct PdfGenerator;

impl PdfGenerator {
    pub fn generate(&self, report: &Report) -> Result<Vec<u8>, ReportError> {
        let (doc, page1, layer1) = PdfDocument::new(report.title(), Mm(210.0), Mm(297.0), "Layer 1");
        let font = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;

        // Add title
        let mut title = Text::new(&report.title(), 10.0, Mm(200.0), &font);
        title.render(&mut doc.get_page(page1).get_layer(layer1));

        // Add summary
        let mut summary = Text::new(&report.summary, 10.0, Mm(190.0), &font);
        summary.render(&mut doc.get_page(page1).get_layer(layer1));

        // Add sections (simplified)
        // Full implementation would iterate sections and render properly

        let bytes = doc.save_to_bytes()?;
        Ok(bytes)
    }
}
```

- [ ] **Write and run tests**
- [ ] **Commit**

### Step 5: Implement JSON Generator

- [ ] **Create generators/json.rs**

```rust
use super::types::*;

pub struct JsonGenerator;

impl JsonGenerator {
    pub fn generate(&self, report: &Report) -> Result<String, ReportError> {
        let json = serde_json::to_string_pretty(report)?;
        Ok(json)
    }
}
```

- [ ] **Write and run tests**
- [ ] **Commit**

### Step 6: Implement Markdown Generator

- [ ] **Create generators/markdown.rs**

```rust
use super::types::*;

pub struct MarkdownGenerator;

impl MarkdownGenerator {
    pub fn generate(&self, report: &Report) -> Result<String, ReportError> {
        let mut md = String::new();

        md.push_str(&format!("# {}\n\n", report.title));
        md.push_str(&format!("{}\n\n", report.summary));

        for section in &report.sections {
            md.push_str(&format!("## {}\n\n", section.title));
            md.push_str(&self.render_content(&section.content));
            md.push_str("\n\n");
        }

        md.push_str("## Insights\n\n");
        for insight in &report.insights {
            md.push_str(&format!("- **{}:** {}\n", insight.insight_type, insight.description));
            if let Some(action) = &insight.action {
                md.push_str(&format!("  - Action: {}\n", action));
            }
        }

        Ok(md)
    }

    fn render_content(&self, content: &SectionContent) -> String {
        match content {
            SectionContent::Text(text) => text.clone(),
            SectionContent::Table(table) => self.render_table(table),
            SectionContent::Metrics(metrics) => self.render_metrics(metrics),
            SectionContent::Chart(_) => "[Chart visualization]".to_string(),
        }
    }

    fn render_table(&self, table: &Table) -> String {
        let mut md = String::new();

        md.push_str(&format!("| {} |\n", table.headers.join(" | ")));
        md.push_str(&format!("| {} |\n", table.headers.iter().map(|_| "---").collect::<Vec<_>>().join(" | ")));

        for row in &table.rows {
            md.push_str(&format!("| {} |\n", row.join(" | ")));
        }

        md
    }

    fn render_metrics(&self, metrics: &[Metric]) -> String {
        let mut md = String::new();
        for metric in metrics {
            md.push_str(&format!("- **{}:** {}\n", metric.name, metric.value));
        }
        md
    }
}
```

- [ ] **Write and run tests**
- [ ] **Commit**

### Step 7: Implement Insights Engine

- [ ] **Create insights.rs**

```rust
use super::types::*;

pub struct InsightsEngine;

impl InsightsEngine {
    pub fn generate_insights(&self, data: &ReportData) -> Vec<Insight> {
        let mut insights = Vec::new();

        // Quality trend insights
        if let Some(quality_trend) = &data.quality_trend {
            if quality_trend.is_declining() {
                insights.push(Insight {
                    insight_type: InsightType::Warning,
                    priority: Priority::High,
                    description: "Quality scores are declining over time".to_string(),
                    action: Some("Review recent workflow changes and verifier configurations".to_string()),
                });
            }
        }

        // Cost insights
        if let Some(cost_data) = &data.cost_data {
            if cost_data.avg_cost > cost_data.budget * 1.1 {
                insights.push(Insight {
                    insight_type: InsightType::Alert,
                    priority: Priority::Critical,
                    description: format!("Cost exceeds budget by {:.1}%", (cost_data.avg_cost / cost_data.budget - 1.0) * 100.0),
                    action: Some("Consider switching to more cost-effective LLM backends".to_string()),
                });
            }
        }

        // Convergence insights
        if let Some(convergence) = &data.convergence_data {
            if convergence.rate < 0.8 {
                insights.push(Insight {
                    insight_type: InsightType::Info,
                    priority: Priority::Medium,
                    description: format!("Convergence rate is {:.1}%, below target", convergence.rate * 100.0),
                    action: Some("Review repair strategies and adjust convergence criteria".to_string()),
                });
            }
        }

        insights
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportData {
    pub quality_trend: Option<QualityTrend>,
    pub cost_data: Option<CostData>,
    pub convergence_data: Option<ConvergenceData>,
}

#[derive(Debug, Clone)]
pub struct QualityTrend {
    pub scores: Vec<f64>,
}

impl QualityTrend {
    fn is_declining(&self) -> bool {
        if self.scores.len() < 2 {
            return false;
        }

        let recent = &self.scores[self.scores.len() - 3..];
        let avg_recent: f64 = recent.iter().sum::<f64>() / recent.len() as f64;

        let historical = &self.scores[..self.scores.len() - 3];
        let avg_historical: f64 = historical.iter().sum::<f64>() / historical.len() as f64;

        avg_recent < avg_historical * 0.95
    }
}

#[derive(Debug, Clone)]
pub struct CostData {
    pub avg_cost: f64,
    pub budget: f64,
}

#[derive(Debug, Clone)]
pub struct ConvergenceData {
    pub rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insight {
    pub insight_type: InsightType,
    pub priority: Priority,
    pub description: String,
    pub action: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum InsightType {
    Info,
    Warning,
    Alert,
    Success,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}
```

- [ ] **Write and run tests**
- [ ] **Commit**

### Step 8: Documentation

- [ ] **Create README.md**

## Completion Criteria

Task 5 is complete when:

- ✅ Report types defined (Quality, Benchmark, Improvement, Summary)
- ✅ HTML generator with templates
- ✅ PDF generator
- ✅ JSON generator
- ✅ Markdown generator
- ✅ Insights engine with actionable recommendations
- ✅ All tests passing
- ✅ Documentation complete
