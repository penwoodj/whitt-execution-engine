# Task 04: Experiment Result Tracking

**Component:** Result storage, comparison logic, visualization, ranking, export

**Dependencies:** Task 01 (Git Experiment Framework)

**Estimated Time:** 6-8 days

**Goal:** Build a comprehensive experiment result tracking system that stores results, compares experiments, visualizes outcomes, ranks experiments, and exports data for analysis.

---

## Overview

The experiment result tracking system:

- Stores experiment results with metadata
- Compares results across experiments
- Visualizes experiment outcomes
- Ranks experiments by success metrics
- Exports data for analysis (JSON, CSV)

---

## File Structure

**New Files:**
- `automation/tracking/mod.rs` - Module exports
- `automation/tracking/storage.rs` - Result storage
- `automation/tracking/comparison.rs` - Comparison logic
- `automation/tracking/visualization.rs` - Visualization helpers
- `automation/tracking/export.rs` - Export functionality

---

## Implementation Steps

### Step 1: Create tracking module structure

**Files:** Create `automation/tracking/mod.rs`

```rust
pub mod storage;
pub mod comparison;
pub mod visualization;
pub mod export;

pub use storage::{ResultStorage, ExperimentResultData};
pub use comparison::{ResultComparator, ComparisonResult, ComparisonMetric};
pub use visualization::{ResultVisualizer, VisualizationFormat};
pub use export::{ResultExporter, ExportFormat};

use std::path::PathBuf;

/// Result artifact directory
pub const RESULT_ARTIFACT_DIR: &str = ".glyphnova/experiment-results";
```

---

### Step 2: Implement result storage

**Files:** Create `automation/tracking/storage.rs`

```rust
use crate::automation::experiment::ExperimentResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentResultData {
    pub id: String,
    pub experiment_id: String,
    pub result: ExperimentResult,
    pub captured_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
}

pub struct ResultStorage {
    storage_dir: PathBuf,
}

impl ResultStorage {
    pub fn new(repo_path: &Path) -> Result<Self, std::io::Error> {
        let storage_dir = repo_path.join(super::RESULT_ARTIFACT_DIR);
        fs::create_dir_all(&storage_dir)?;

        Ok(Self { storage_dir })
    }

    pub fn store_result(&self, data: &ExperimentResultData) -> Result<(), std::io::Error> {
        let result_path = self.storage_dir.join(format!("{}.json", data.id));
        let result_json = serde_json::to_string_pretty(data)?;
        fs::write(result_path, result_json)?;
        Ok(())
    }

    pub fn get_result(&self, id: &str) -> Result<Option<ExperimentResultData>, std::io::Error> {
        let result_path = self.storage_dir.join(format!("{}.json", id));
        if !result_path.exists() {
            return Ok(None);
        }

        let result_str = fs::read_to_string(result_path)?;
        let data: ExperimentResultData = serde_json::from_str(&result_str)?;
        Ok(Some(data))
    }

    pub fn get_results_for_experiment(&self, experiment_id: &str) -> Result<Vec<ExperimentResultData>, std::io::Error> {
        let mut results = Vec::new();

        for entry in fs::read_dir(&self.storage_dir)? {
            let entry = entry?;
            let content = fs::read_to_string(entry.path())?;
            if let Ok(data) = serde_json::from_str::<ExperimentResultData>(&content) {
                if data.experiment_id == experiment_id {
                    results.push(data);
                }
            }
        }

        results.sort_by(|a, b| b.captured_at.cmp(&a.captured_at));
        Ok(results)
    }

    pub fn list_all_results(&self) -> Result<Vec<ExperimentResultData>, std::io::Error> {
        let mut results = Vec::new();

        for entry in fs::read_dir(&self.storage_dir)? {
            let entry = entry?;
            let content = fs::read_to_string(entry.path())?;
            if let Ok(data) = serde_json::from_str::<ExperimentResultData>(&content) {
                results.push(data);
            }
        }

        results.sort_by(|a, b| b.captured_at.cmp(&a.captured_at));
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_store_and_retrieve_result() {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage = ResultStorage::new(temp_dir.path()).unwrap();

        let data = ExperimentResultData {
            id: Uuid::new_v4().to_string(),
            experiment_id: "test-experiment".to_string(),
            result: ExperimentResult {
                exit_code: 0,
                output: "Success".to_string(),
                error: None,
                duration_ms: 1000,
                metrics: HashMap::new(),
            },
            captured_at: Utc::now(),
            tags: vec!["test".to_string()],
            metadata: HashMap::new(),
        };

        storage.store_result(&data).unwrap();

        let retrieved = storage.get_result(&data.id).unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().experiment_id, "test-experiment");
    }
}
```

---

### Step 3: Implement result comparator

**Files:** Create `automation/tracking/comparison.rs`

```rust
use super::storage::ExperimentResultData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonMetric {
    Duration,
    ExitCode,
    SuccessRate,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonResult {
    pub experiment_a_id: String,
    pub experiment_b_id: String,
    pub metric: ComparisonMetric,
    pub difference: f64,
    pub improvement: bool,  // true if experiment_a is better
}

pub struct ResultComparator;

impl ResultComparator {
    pub fn compare(
        a: &ExperimentResultData,
        b: &ExperimentResultData,
        metric: ComparisonMetric,
    ) -> Option<ComparisonResult> {
        match metric {
            ComparisonMetric::Duration => {
                let diff = (a.result.duration_ms as f64) - (b.result.duration_ms as f64);
                Some(ComparisonResult {
                    experiment_a_id: a.experiment_id.clone(),
                    experiment_b_id: b.experiment_id.clone(),
                    metric,
                    difference: diff,
                    improvement: diff < 0.0,  // Lower duration is better
                })
            }
            ComparisonMetric::ExitCode => {
                let diff = (a.result.exit_code as f64) - (b.result.exit_code as f64);
                Some(ComparisonResult {
                    experiment_a_id: a.experiment_id.clone(),
                    experiment_b_id: b.experiment_id.clone(),
                    metric,
                    difference: diff,
                    improvement: a.result.exit_code < b.result.exit_code,
                })
            }
            ComparisonMetric::SuccessRate => {
                let rate_a = a.result.metrics.get("success_rate").copied().unwrap_or(0.0);
                let rate_b = b.result.metrics.get("success_rate").copied().unwrap_or(0.0);
                let diff = rate_a - rate_b;
                Some(ComparisonResult {
                    experiment_a_id: a.experiment_id.clone(),
                    experiment_b_id: b.experiment_id.clone(),
                    metric,
                    difference: diff,
                    improvement: diff > 0.0,  // Higher success rate is better
                })
            }
            ComparisonMetric::Custom(_) => None,
        }
    }

    pub fn rank_experiments(results: &[ExperimentResultData]) -> Vec<&ExperimentResultData> {
        let mut ranked = results.to_vec();
        ranked.sort_by(|a, b| {
            let score_a = Self::calculate_score(a);
            let score_b = Self::calculate_score(b);
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        ranked.iter().collect()
    }

    fn calculate_score(result: &ExperimentResultData) -> f64 {
        let mut score = 0.0;

        // Exit code (lower is better)
        score -= result.result.exit_code as f64 * 10.0;

        // Duration (lower is better)
        score -= (result.result.duration_ms as f64) / 1000.0;

        // Success rate (higher is better)
        if let Some(&rate) = result.result.metrics.get("success_rate") {
            score += rate * 100.0;
        }

        score
    }
}
```

---

### Step 4: Implement visualization helpers

**Files:** Create `automation/tracking/visualization.rs`

```rust
use super::storage::ExperimentResultData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualizationFormat {
    Table,
    Chart,
    Summary,
}

pub struct ResultVisualizer;

impl ResultVisualizer {
    pub fn format_results_table(results: &[ExperimentResultData]) -> String {
        let mut table = String::new();
        table.push_str("+----------------------+----------------+--------+----------+\n");
        table.push_str("| Experiment ID       | Exit Code      | Duration | Success  |\n");
        table.push_str("+----------------------+----------------+--------+----------+\n");

        for result in results {
            let success_rate = result
                .result
                .metrics
                .get("success_rate")
                .map(|r| format!("{:.1}%", r * 100.0))
                .unwrap_or("N/A".to_string());

            table.push_str(&format!(
                "| {:<20} | {:<14} | {:>6}ms | {:>8} |\n",
                result.experiment_id,
                result.result.exit_code,
                result.result.duration_ms,
                success_rate
            ));
        }

        table.push_str("+----------------------+----------------+--------+----------+\n");
        table
    }

    pub fn format_summary(results: &[ExperimentResultData]) -> String {
        let count = results.len();
        let avg_duration = if count > 0 {
            results.iter().map(|r| r.result.duration_ms).sum::<u64>() as f64 / count as f64
        } else {
            0.0
        };

        let success_count = results.iter().filter(|r| r.result.exit_code == 0).count();
        let success_rate = if count > 0 {
            success_count as f64 / count as f64
        } else {
            0.0
        };

        format!(
            "Summary:\n  Total experiments: {}\n  Average duration: {:.2}ms\n  Success rate: {:.1}%\n",
            count, avg_duration, success_rate * 100.0
        )
    }
}
```

---

### Step 5: Implement result exporter

**Files:** Create `automation/tracking/export.rs`

```rust
use super::storage::ExperimentResultData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportFormat {
    Json,
    Csv,
}

pub struct ResultExporter;

impl ResultExporter {
    pub fn export_json(results: &[ExperimentResultData]) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(results)
    }

    pub fn export_csv(results: &[ExperimentResultData]) -> Result<String, std::fmt::Error> {
        let mut csv = String::new();
        csv.push_str("experiment_id,exit_code,duration_ms,success_rate\n");

        for result in results {
            let success_rate = result
                .result
                .metrics
                .get("success_rate")
                .map(|r| r.to_string())
                .unwrap_or("N/A".to_string());

            csv.push_str(&format!(
                "{},{},{},{}\n",
                result.experiment_id, result.result.exit_code, result.result.duration_ms, success_rate
            ));
        }

        Ok(csv)
    }
}
```

---

### Step 6: Update automation module and commit

**Files:** Modify `automation/mod.rs` and commit

```bash
git add automation/tracking
git commit -m "feat(automation): implement experiment result tracking (Task 04)

- Add result storage with metadata
- Implement result comparison logic
- Add visualization helpers (table, summary)
- Implement export functionality (JSON, CSV)
- Add comprehensive unit tests

Refs: Phase 6, Task 04"
```

---

## Summary

Task 04 implements experiment result tracking with:

✅ Result storage with metadata
✅ Comparison logic across experiments
✅ Visualization helpers
✅ Export functionality (JSON, CSV)
✅ Unit tests

**Next Steps:** Task 05 (Rollback & Cleanup) or Task 07 (Automation CLI)
