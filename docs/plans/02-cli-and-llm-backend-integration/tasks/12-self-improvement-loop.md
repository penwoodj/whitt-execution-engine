# Task 12: Self-Improvement Loop

**Files:**
- Create: `src/self_improvement/mod.rs`
- Create: `src/self_improvement/executor.rs`
- Create: `src/self_improvement/analyzer.rs`
- Create: `src/self_improvement/diff.rs`
- Modify: `src/lib.rs` (add self_improvement module)
- Test: `tests/self_improvement/loop_test.rs`

---

## Overview

Implement self-improvement loop infrastructure with execution log analysis, improvement suggestions, workflow diff generation, and automated updates for continuous workflow optimization.

---

## Implementation Steps

### Step 1: Create executor

- [ ] **Step 1.1: Write executor**

```rust
// src/self_improvement/executor.rs
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionLog {
    pub id: String,
    pub workflow_id: String,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub status: ExecutionStatus,
    pub steps: Vec<StepExecution>,
    pub metrics: ExecutionMetrics,
    pub errors: Vec<ExecutionError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionStatus {
    Running,
    Completed,
    Failed,
    Partial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepExecution {
    pub step_id: String,
    pub name: String,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub status: ExecutionStatus,
    pub input: serde_json::Value,
    pub output: Option<serde_json::Value>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMetrics {
    pub total_duration_ms: Option<u64>,
    pub total_tokens: Option<u32>,
    pub tool_calls: u32,
    pub cache_hits: u32,
    pub cache_misses: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionError {
    pub step_id: String,
    pub error_type: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

pub struct ExecutionLogger {
    log_dir: PathBuf,
}

impl ExecutionLogger {
    pub fn new(log_dir: PathBuf) -> Self {
        Self { log_dir }
    }

    pub async fn log_execution(&self, execution_log: &ExecutionLog) -> Result<()> {
        // Create log directory if needed
        std::fs::create_dir_all(&self.log_dir)
            .context("Failed to create log directory")?;

        // Write log file
        let log_file = self.log_dir.join(format!("{}.json", execution_log.id));
        let json = serde_json::to_string_pretty(execution_log)
            .context("Failed to serialize execution log")?;

        std::fs::write(log_file, json)
            .context("Failed to write execution log")?;

        Ok(())
    }

    pub async fn load_execution(&self, execution_id: &str) -> Result<ExecutionLog> {
        let log_file = self.log_dir.join(format!("{}.json", execution_id));
        let content = std::fs::read_to_string(&log_file)
            .context("Failed to read execution log")?;

        serde_json::from_str(&content)
            .context("Failed to deserialize execution log")
    }

    pub async fn list_executions(
        &self,
        workflow_id: Option<&str>,
    ) -> Result<Vec<ExecutionLog>> {
        let mut executions = Vec::new();

        if !self.log_dir.exists() {
            return Ok(executions);
        }

        for entry in std::fs::read_dir(&self.log_dir)
            .context("Failed to read log directory")?
        {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(log) = self.load_execution(
                    path.file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("")
                ).await {
                    if workflow_id.is_none() || log.workflow_id == workflow_id.unwrap() {
                        executions.push(log);
                    }
                }
            }
        }

        executions.sort_by(|a, b| b.started_at.cmp(&a.started_at));

        Ok(executions)
    }
}
```

- [ ] **Step 1.2: Commit**

```bash
git add src/self_improvement/executor.rs
git commit -m "feat(self_improvement): add execution logger"
```

---

### Step 2: Create analyzer

- [ ] **Step 2.1: Write analyzer**

```rust
// src/self_improvement/analyzer.rs
use super::executor::{ExecutionLog, StepExecution, ExecutionStatus};
use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub execution_id: String,
    pub workflow_id: String,
    pub overall_assessment: Assessment,
    pub step_analyses: Vec<StepAnalysis>,
    pub suggestions: Vec<ImprovementSuggestion>,
}

#[derive(Debug, Clone)]
pub struct Assessment {
    pub status: OverallStatus,
    pub score: f32,
    pub issues: Vec<Issue>,
}

#[derive(Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum OverallStatus {
    Excellent,
    Good,
    Fair,
    Poor,
}

#[derive(Debug, Clone)]
pub struct Issue {
    pub severity: IssueSeverity,
    pub category: IssueCategory,
    pub description: String,
    pub affected_steps: Vec<String>,
}

#[derive(Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum IssueSeverity {
    Critical,
    Warning,
    Info,
}

#[derive(Debug, Clone)]
pub enum IssueCategory {
    Performance,
    Reliability,
    Correctness,
    ResourceUsage,
}

#[derive(Debug, Clone)]
pub struct StepAnalysis {
    pub step_id: String,
    pub name: String,
    pub status: ExecutionStatus,
    pub duration_ms: Option<u64>,
    pub assessment: String,
    pub issues: Vec<Issue>,
}

#[derive(Debug, Clone)]
pub struct ImprovementSuggestion {
    pub category: SuggestionCategory,
    pub priority: SuggestionPriority,
    pub title: String,
    pub description: String,
    pub affected_steps: Vec<String>,
    pub estimated_impact: String,
}

#[derive(Debug, Clone)]
pub enum SuggestionCategory {
    Optimization,
    Refactoring,
    Caching,
    ErrorHandling,
    Configuration,
}

#[derive(Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SuggestionPriority {
    High,
    Medium,
    Low,
}

pub struct ExecutionAnalyzer;

impl ExecutionAnalyzer {
    pub fn analyze(execution_log: &ExecutionLog) -> Result<AnalysisResult> {
        let step_analyses = Self::analyze_steps(&execution_log.steps)?;
        let suggestions = Self::generate_suggestions(execution_log, &step_analyses)?;
        let overall_assessment = Self::assess_overall(execution_log, &step_analyses);

        Ok(AnalysisResult {
            execution_id: execution_log.id.clone(),
            workflow_id: execution_log.workflow_id.clone(),
            overall_assessment,
            step_analyses,
            suggestions,
        })
    }

    fn analyze_steps(steps: &[StepExecution]) -> Result<Vec<StepAnalysis>> {
        let mut analyses = Vec::new();

        for step in steps {
            let duration = step.started_at
                .signed_duration_since(step.ended_at.unwrap_or(step.started_at))
                .num_milliseconds() as u64;

            let assessment = Self::assess_step(step, duration);
            let issues = Self::identify_step_issues(step, duration);

            analyses.push(StepAnalysis {
                step_id: step.step_id.clone(),
                name: step.name.clone(),
                status: step.status.clone(),
                duration_ms: if step.ended_at.is_some() { Some(duration) } else { None },
                assessment,
                issues,
            });
        }

        Ok(analyses)
    }

    fn assess_step(step: &StepExecution, duration_ms: u64) -> String {
        match step.status {
            ExecutionStatus::Completed => {
                if duration_ms < 1000 {
                    "Excellent performance".to_string()
                } else if duration_ms < 5000 {
                    "Good performance".to_string()
                } else {
                    "Consider optimization".to_string()
                }
            }
            ExecutionStatus::Failed => "Step failed - investigate errors".to_string(),
            ExecutionStatus::Partial => "Partial completion - review output".to_string(),
            ExecutionStatus::Running => "Step still running".to_string(),
        }
    }

    fn identify_step_issues(step: &StepExecution, duration_ms: u64) -> Vec<Issue> {
        let mut issues = Vec::new();

        // Check for failures
        if let ExecutionStatus::Failed = step.status {
            issues.push(Issue {
                severity: IssueSeverity::Critical,
                category: IssueCategory::Reliability,
                description: "Step execution failed".to_string(),
                affected_steps: vec![step.step_id.clone()],
            });
        }

        // Check for long duration
        if duration_ms > 10000 {
            issues.push(Issue {
                severity: IssueSeverity::Warning,
                category: IssueCategory::Performance,
                description: format!("Step took {}ms, consider optimization", duration_ms),
                affected_steps: vec![step.step_id.clone()],
            });
        }

        // Check for errors
        if let Some(error) = &step.error {
            issues.push(Issue {
                severity: IssueSeverity::Warning,
                category: IssueCategory::Correctness,
                description: error.clone(),
                affected_steps: vec![step.step_id.clone()],
            });
        }

        issues
    }

    fn generate_suggestions(
        execution_log: &ExecutionLog,
        step_analyses: &[StepAnalysis],
    ) -> Result<Vec<ImprovementSuggestion>> {
        let mut suggestions = Vec::new();

        // Check for caching opportunities
        if execution_log.metrics.cache_hits > 0 {
            let cache_hit_rate = execution_log.metrics.cache_hits as f32 /
                (execution_log.metrics.cache_hits + execution_log.metrics.cache_misses) as f32;

            if cache_hit_rate < 0.5 {
                suggestions.push(ImprovementSuggestion {
                    category: SuggestionCategory::Caching,
                    priority: SuggestionPriority::Medium,
                    title: "Improve caching strategy".to_string(),
                    description: format!(
                        "Cache hit rate is {:.1}%, consider increasing cache duration or scope",
                        cache_hit_rate * 100.0
                    ),
                    affected_steps: vec![],
                    estimated_impact: "Moderate performance improvement".to_string(),
                });
            }
        }

        // Check for failed steps
        let failed_steps: Vec<_> = step_analyses
            .iter()
            .filter(|a| matches!(a.status, ExecutionStatus::Failed))
            .collect();

        if !failed_steps.is_empty() {
            suggestions.push(ImprovementSuggestion {
                category: SuggestionCategory::ErrorHandling,
                priority: SuggestionPriority::High,
                title: "Add error handling for failed steps".to_string(),
                description: format!(
                    "{} steps failed, consider adding retry logic or fallback mechanisms",
                    failed_steps.len()
                ),
                affected_steps: failed_steps.iter().map(|s| s.step_id.clone()).collect(),
                estimated_impact: "Improved reliability".to_string(),
            });
        }

        // Check for slow steps
        let slow_steps: Vec<_> = step_analyses
            .iter()
            .filter(|a| a.duration_ms.map_or(false, |d| d > 5000))
            .collect();

        if !slow_steps.is_empty() {
            suggestions.push(ImprovementSuggestion {
                category: SuggestionCategory::Optimization,
                priority: SuggestionPriority::Medium,
                title: "Optimize slow steps".to_string(),
                description: format!(
                    "{} steps took >5s, consider parallelization or batching",
                    slow_steps.len()
                ),
                affected_steps: slow_steps.iter().map(|s| s.step_id.clone()).collect(),
                estimated_impact: "Significant performance improvement".to_string(),
            });
        }

        Ok(suggestions)
    }

    fn assess_overall(
        execution_log: &ExecutionLog,
        step_analyses: &[StepAnalysis],
    ) -> Assessment {
        let completed_steps = step_analyses
            .iter()
            .filter(|a| matches!(a.status, ExecutionStatus::Completed))
            .count();

        let failed_steps = step_analyses
            .iter()
            .filter(|a| matches!(a.status, ExecutionStatus::Failed))
            .count();

        let total_steps = step_analyses.len();

        let success_rate = if total_steps > 0 {
            completed_steps as f32 / total_steps as f32
        } else {
            0.0
        };

        let status = if success_rate == 1.0 {
            OverallStatus::Excellent
        } else if success_rate >= 0.8 {
            OverallStatus::Good
        } else if success_rate >= 0.5 {
            OverallStatus::Fair
        } else {
            OverallStatus::Poor
        };

        let score = success_rate * 100.0;

        let issues = step_analyses
            .iter()
            .flat_map(|a| a.issues.clone())
            .collect();

        Assessment {
            status,
            score,
            issues,
        }
    }
}
```

- [ ] **Step 2.2: Commit**

```bash
git add src/self_improvement/analyzer.rs
git commit -m "feat(self_improvement): add execution analyzer"
```

---

### Step 3: Create diff generator

- [ ] **Step 3.1: Write diff generator**

```rust
// src/self_improvement/diff.rs
use crate::ir::WorkflowIR;
use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct WorkflowDiff {
    pub workflow_id: String,
    pub changes: Vec<WorkflowChange>,
    pub summary: String,
}

#[derive(Debug, Clone)]
pub enum WorkflowChange {
    StepAdded {
        step_id: String,
        step_name: String,
    },
    StepRemoved {
        step_id: String,
        step_name: String,
    },
    StepModified {
        step_id: String,
        step_name: String,
        field: String,
        old_value: String,
        new_value: String,
    },
    MetadataChanged {
        field: String,
        old_value: String,
        new_value: String,
    },
}

pub struct DiffGenerator;

impl DiffGenerator {
    pub fn generate_diff(
        original: &WorkflowIR,
        modified: &WorkflowIR,
    ) -> Result<WorkflowDiff> {
        let mut changes = Vec::new();

        // Compare metadata
        if original.name != modified.name {
            changes.push(WorkflowChange::MetadataChanged {
                field: "name".to_string(),
                old_value: original.name.clone(),
                new_value: modified.name.clone(),
            });
        }

        // Compare steps
        let original_step_ids: std::collections::HashSet<_> =
            original.steps.iter().map(|s| s.id.clone()).collect();
        let modified_step_ids: std::collections::HashSet<_> =
            modified.steps.iter().map(|s| s.id.clone()).collect();

        // Added steps
        for step_id in modified_step_ids.difference(&original_step_ids) {
            if let Some(step) = modified.steps.iter().find(|s| &s.id == step_id) {
                changes.push(WorkflowChange::StepAdded {
                    step_id: step.id.clone(),
                    step_name: step.name.clone(),
                });
            }
        }

        // Removed steps
        for step_id in original_step_ids.difference(&modified_step_ids) {
            if let Some(step) = original.steps.iter().find(|s| &s.id == step_id) {
                changes.push(WorkflowChange::StepRemoved {
                    step_id: step.id.clone(),
                    step_name: step.name.clone(),
                });
            }
        }

        // Modified steps
        for step_id in modified_step_ids.intersection(&original_step_ids) {
            let original_step = original.steps.iter().find(|s| &s.id == step_id).unwrap();
            let modified_step = modified.steps.iter().find(|s| &s.id == step_id).unwrap();

            if original_step.name != modified_step.name {
                changes.push(WorkflowChange::StepModified {
                    step_id: step_id.clone(),
                    step_name: modified_step.name.clone(),
                    field: "name".to_string(),
                    old_value: original_step.name.clone(),
                    new_value: modified_step.name.clone(),
                });
            }

            if original_step.prompt != modified_step.prompt {
                changes.push(WorkflowChange::StepModified {
                    step_id: step_id.clone(),
                    step_name: modified_step.name.clone(),
                    field: "prompt".to_string(),
                    old_value: original_step.prompt.clone(),
                    new_value: modified_step.prompt.clone(),
                });
            }
        }

        let summary = Self::generate_summary(&changes);

        Ok(WorkflowDiff {
            workflow_id: original.id.clone(),
            changes,
            summary,
        })
    }

    fn generate_summary(changes: &[WorkflowChange]) -> String {
        let added = changes.iter().filter(|c| matches!(c, WorkflowChange::StepAdded { .. })).count();
        let removed = changes.iter().filter(|c| matches!(c, WorkflowChange::StepRemoved { .. })).count();
        let modified = changes.iter().filter(|c| matches!(c, WorkflowChange::StepModified { .. })).count();

        format!(
            "{} steps added, {} steps removed, {} steps modified",
            added, removed, modified
        )
    }
}
```

- [ ] **Step 3.2: Write module exports**

```rust
// src/self_improvement/mod.rs
pub mod executor;
pub mod analyzer;
pub mod diff;

pub use executor::{ExecutionLog, ExecutionLogger, ExecutionStatus, StepExecution, ExecutionMetrics, ExecutionError};
pub use analyzer::{ExecutionAnalyzer, AnalysisResult, Assessment, Issue, StepAnalysis, ImprovementSuggestion};
pub use diff::{WorkflowDiff, WorkflowChange, DiffGenerator};
```

- [ ] **Step 3.3: Update lib.rs**

```rust
// src/lib.rs
pub mod self_improvement;

pub use self_improvement::*;
```

- [ ] **Step 3.4: Commit**

```bash
git add src/self_improvement/diff.rs src/self_improvement/mod.rs src/lib.rs
git commit -m "feat(self_improvement): add workflow diff generator"
```

---

### Step 4: Write tests

- [ ] **Step 4.1: Write integration tests**

```rust
// tests/self_improvement/loop_test.rs
use whitt_execution_engine::self_improvement::{
    ExecutionLog, ExecutionLogger, ExecutionAnalyzer, DiffGenerator,
    StepExecution, ExecutionStatus, WorkflowIR, StepIR,
};
use tempfile::TempDir;

#[tokio::test]
async fn test_execution_logging() {
    let temp_dir = TempDir::new().unwrap();
    let logger = ExecutionLogger::new(temp_dir.path().to_path_buf());

    let execution_log = ExecutionLog {
        id: "test-exec".to_string(),
        workflow_id: "test-workflow".to_string(),
        started_at: chrono::Utc::now(),
        ended_at: None,
        status: ExecutionStatus::Completed,
        steps: vec![],
        metrics: Default::default(),
        errors: vec![],
    };

    logger.log_execution(&execution_log).await.unwrap();

    let loaded = logger.load_execution("test-exec").await.unwrap();
    assert_eq!(loaded.id, "test-exec");
}

#[tokio::test]
async fn test_execution_analysis() {
    let execution_log = ExecutionLog {
        id: "test-exec".to_string(),
        workflow_id: "test-workflow".to_string(),
        started_at: chrono::Utc::now(),
        ended_at: Some(chrono::Utc::now()),
        status: ExecutionStatus::Completed,
        steps: vec![
            StepExecution {
                step_id: "step-1".to_string(),
                name: "Step 1".to_string(),
                started_at: chrono::Utc::now(),
                ended_at: Some(chrono::Utc::now()),
                status: ExecutionStatus::Completed,
                input: serde_json::json!({}),
                output: None,
                error: None,
            },
        ],
        metrics: Default::default(),
        errors: vec![],
    };

    let analysis = ExecutionAnalyzer::analyze(&execution_log).unwrap();

    assert_eq!(analysis.workflow_id, "test-workflow");
    assert_eq!(analysis.step_analyses.len(), 1);
}

#[test]
fn test_workflow_diff() {
    let original = WorkflowIR {
        id: "test".to_string(),
        name: "Original".to_string(),
        steps: vec![
            StepIR {
                id: "step-1".to_string(),
                name: "Step 1".to_string(),
                prompt: "Old prompt".to_string(),
                subworkflow: None,
                context: Default::default(),
            },
        ],
    };

    let modified = WorkflowIR {
        id: "test".to_string(),
        name: "Modified".to_string(),
        steps: vec![
            StepIR {
                id: "step-1".to_string(),
                name: "Step 1".to_string(),
                prompt: "New prompt".to_string(),
                subworkflow: None,
                context: Default::default(),
            },
        ],
    };

    let diff = DiffGenerator::generate_diff(&original, &modified).unwrap();

    assert_eq!(diff.changes.len(), 2); // name change + prompt change
}
```

- [ ] **Step 4.2: Commit**

```bash
git add tests/self_improvement/loop_test.rs
git commit -m "test(self_improvement): add self-improvement loop tests"
```

---

## Summary

This task implements the self-improvement loop infrastructure including:

1. **Execution logging** with detailed step tracking and metrics
2. **Execution analysis** with automatic issue detection
3. **Improvement suggestions** with priority and impact estimates
4. **Workflow diff generation** for tracking changes
5. **Comprehensive tests** for all components

**Key Features:**
- Detailed execution logs with step-level metrics
- Automatic analysis of execution quality
- Issue detection (performance, reliability, correctness)
- Improvement suggestions with priorities
- Workflow diffing for change tracking
- Assessment scoring system
- Caching analysis

**Improvement Categories:**
- Optimization (performance)
- Refactoring (code quality)
- Caching (efficiency)
- Error handling (reliability)
- Configuration (setup)

**Next:** Validation and Test Specifications

---

## QA Cross-References

- **QA Criteria**: ['$qa_criteria']('$file')
- **Test Cases**: ['$test_case']('$file')
- **Schema Ref**: $schema_ref
