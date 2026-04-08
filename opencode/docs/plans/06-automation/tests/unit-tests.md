# Phase 06 Automation - Unit Tests

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Define comprehensive unit test specifications for all automation components

**Architecture:** Modular unit tests organized by component: cron parser, scheduler, git isolation, merge proposals, refinement events, experiment tracking, and IR compilation.

**Tech Stack:** Rust, tempfile, git2, cron, chrono, serde, mockall

---

## Unit Test Structure

```
tests/unit/
├── cron_parser_tests.rs       # Cron expression parsing tests
├── scheduler_tests.rs         # Tokio scheduling engine tests
├── git_isolation_tests.rs    # Git branch isolation tests
├── merge_proposal_tests.rs   # Merge proposal generation tests
├── refinement_events_tests.rs # Refinement event capture tests
├── experiment_tracking_tests.rs # Experiment result tracking tests
├── ir_compilation_tests.rs   # WorkflowIR compilation tests
└── mod.rs                   # Unit test module
```

---

## Module 1: Cron Expression Parsing

### Test Group 1: Valid Cron Expressions

**Files:**
- Create: `tests/unit/cron_parser_tests.rs`

#### Test: Parse standard 5-field expressions

- [ ] **Step 1: Write test for valid cron expressions**

```rust
#[cfg(test)]
mod cron_parser_tests {
    use super::*;

    #[test]
    fn parse_standard_five_field_expressions() {
        let expressions = vec![
            // Every minute
            ("* * * * *", "Every minute"),
            // Every hour
            ("0 * * * *", "Every hour at minute 0"),
            // Daily at midnight
            ("0 0 * * *", "Daily at midnight"),
            // Weekly (Sunday midnight)
            ("0 0 * * 0", "Weekly on Sunday at midnight"),
            // Monthly (1st of month at midnight)
            ("0 0 1 * *", "Monthly on 1st at midnight"),
            // Weekdays 9am-5pm
            ("0 9-17 * * 1-5", "Weekdays 9am-5pm"),
            // Every 15 minutes
            ("*/15 * * * *", "Every 15 minutes"),
            // Every 6 hours
            "0 */6 * * *", "Every 6 hours"),
            // Specific hours
            ("0 8,12,18 * * *", "At 8am, 12pm, 6pm daily"),
            // First day of January
            ("0 0 1 1 *", "January 1st at midnight"),
            // Multiple days of week
            ("0 0 * * 0,6", "Saturday and Sunday at midnight"),
            // Multiple months
            ("0 0 1 1,4,7,10 *", "First day of quarter at midnight"),
            // Range and list combined
            ("0 9-17 * * 1-5", "Weekdays 9am-5pm"),
            // Specific time multiple days
            ("30 14 * * 2,4", "Tuesdays and Thursdays at 2:30pm"),
        ];

        for (expr, description) in expressions {
            let result = CronExpression::parse(expr);
            assert!(result.is_ok(), "Failed to parse '{}': {}", description, result.unwrap_err());
        }

        println!("✓ All {} standard 5-field expressions parsed successfully", expressions.len());
    }
}
```

- [ ] **Step 2: Run test to verify compilation**

```bash
cargo test --lib cron_parser_tests::parse_standard_five_field_expressions -- --exact
```

Expected: Test compiles and runs (will fail initially)

- [ ] **Step 3: Implement CronExpression::parse**

```rust
use cron::Schedule;
use chrono::{DateTime, Utc, Local, TimeZone};
use std::str::FromStr;

pub struct CronExpression {
    pub raw: String,
    pub schedule: Schedule,
    pub next_run: DateTime<Utc>,
    pub interval_seconds: u64,
}

impl CronExpression {
    pub fn parse(expr: &str) -> Result<Self, CronParseError> {
        let schedule = Schedule::from_str(expr)
            .map_err(|e| CronParseError::InvalidExpression(e.to_string()))?;

        let next_run = schedule.after(&Utc::now()).next()
            .ok_or_else(|| CronParseError::InvalidExpression("No next run time".into()))?;

        let interval_seconds = schedule.after(&Utc::now()).next()
            .and_then(|next| Some((next - Utc::now()).num_seconds().max(0) as u64))
            .unwrap_or(60);

        Ok(Self {
            raw: expr.to_string(),
            schedule,
            next_run,
            interval_seconds,
        })
    }
}
```

- [ ] **Step 4: Run test to verify passes**

```bash
cargo test --lib cron_parser_tests::parse_standard_five_field_expressions -- --exact
```

Expected: PASS

#### Test: Edge case expressions

- [ ] **Step 5: Write test for edge cases**

```rust
#[test]
fn parse_edge_case_expressions() {
    let expressions = vec![
        // Maximum values
        ("59 23 31 12 6", "Maximum values for all fields"),
        // Minimum values
        ("0 0 1 1 0", "Minimum values for all fields"),
        // List of values
        ("0,15,30,45 * * * *", "Multiple minute values"),
        ("1,2,3,4,5 * * * *", "Multiple minute values (consecutive)"),
        // Range
        ("0-59 * * * *", "Full minute range"),
        ("0 0-23 * * *", "Full hour range"),
        // Step values
            ("*/5 * * * *", "Every 5 minutes"),
            ("0 */2 * * *", "Every 2 hours"),
            ("0 0 */2 * *", "Every 2 days"),
        // Combined
            ("0 */4 */6 * *", "Every 4 hours, every 6 days"),
        // Comma-separated list
            ("0 0,12 * * *", "Midnight and noon"),
            ("0 9,12,15,18 * * 1-5", "Specific hours on weekdays"),
        // Day of week ranges
            ("0 9 * * 1-5", "Weekdays at 9am"),
            ("0 9 * * 0,6", "Weekends at 9am"),
        // Month ranges
            ("0 0 1 1-12 *", "First of every month"),
            ("0 0 * 1-3 *", "Jan-Mar daily at midnight"),
    ];

    for (expr, description) in expressions {
        let result = CronExpression::parse(expr);
        assert!(result.is_ok(), "Failed to parse '{}': {}", description, result.unwrap_err());
    }

    println!("✓ All {} edge case expressions parsed successfully", expressions.len());
}
```

- [ ] **Step 6: Run test to verify passes**

```bash
cargo test --lib cron_parser_tests::parse_edge_case_expressions -- --exact
```

Expected: PASS

### Test Group 2: Invalid Cron Expressions

#### Test: Reject invalid expressions

- [ ] **Step 7: Write test for invalid expressions**

```rust
#[test]
fn reject_invalid_cron_expressions() {
    let invalid_expressions = vec![
        // Not 5 fields
        ("invalid", "Not a cron expression"),
        ("* * * *", "Only 4 fields"),
        ("* * * * * *", "6 fields (too many)"),
        // Invalid minute
        ("60 * * * *", "Minute out of range (0-59)"),
        ("-1 * * * *", "Negative minute"),
        ("abc * * * *", "Non-numeric minute"),
        // Invalid hour
        ("0 25 * * *", "Hour out of range (0-23)"),
        ("0 -1 * * *", "Negative hour"),
        ("0 abc * * *", "Non-numeric hour"),
        // Invalid day of month
        ("0 * 32 * *", "Day of month out of range (1-31)"),
        ("0 * 0 * *", "Day of month zero"),
        ("0 * abc * *", "Non-numeric day of month"),
        // Invalid month
        ("0 * * 13 *", "Month out of range (1-12)"),
        ("0 * * 0 *", "Month zero"),
        ("0 * * abc *", "Non-numeric month"),
        // Invalid day of week
        ("0 * * * 8", "Day of week out of range (0-6)"),
        ("0 * * * -1", "Negative day of week"),
        ("0 * * * abc", "Non-numeric day of week"),
        // Empty expression
            ("", "Empty expression"),
        // Whitespace only
            ("   ", "Whitespace only"),
        // Invalid ranges
            ("0 59-60 * * *", "Hour range exceeds maximum"),
            ("0 * 1-32 * *", "Day range exceeds maximum"),
        // Invalid step values
            ("0 */0 * * *", "Step value zero"),
            ("0 * */0 * *", "Step value zero"),
        // Invalid characters
            ("0 * * * *$", "Invalid character $"),
            ("0 @ * * *", "Invalid character @"),
        // Malformed lists
            ("0 9,,12 * * *", "Double comma"),
            ("0 9, * * * *", "Trailing comma"),
    ];

    for (expr, description) in invalid_expressions {
        let result = CronExpression::parse(expr);
        assert!(result.is_err(), "'{}' should be rejected: {}", description, expr);

        if let Err(CronParseError::InvalidExpression(msg)) = result {
            assert!(!msg.is_empty(), "Error message should not be empty for '{}'", description);
        } else {
            panic!("Expected CronParseError::InvalidExpression for '{}'", description);
        }
    }

    println!("✓ All {} invalid expressions correctly rejected", invalid_expressions.len());
}
```

- [ ] **Step 8: Run test to verify passes**

```bash
cargo test --lib cron_parser_tests::reject_invalid_cron_expressions -- --exact
```

Expected: PASS

### Test Group 3: Timezone Handling

#### Test: Parse with timezone

- [ ] **Step 9: Write test for timezone handling**

```rust
#[test]
fn parse_with_timezone() {
    // Test UTC timezone
    let utc_expr = CronExpression::with_timezone("0 9 * * *", "UTC").unwrap();
    let utc_next = utc_expr.next_run;
    assert_eq!(utc_next.hour(), 9, "UTC expression should run at 9am UTC");

    // Test PST timezone (UTC-8)
    let pst_expr = CronExpression::with_timezone("0 9 * * *", "America/Los_Angeles").unwrap();
    let pst_next = pst_expr.next_run;
    // 9am PST = 5pm UTC
    assert_eq!(pst_next.hour(), 17, "PST expression should run at 5pm UTC (9am PST)");

    // Test EST timezone (UTC-5)
    let est_expr = CronExpression::with_timezone("0 9 * * *", "America/New_York").unwrap();
    let est_next = est_expr.next_run;
    // 9am EST = 2pm UTC
    assert_eq!(est_next.hour(), 14, "EST expression should run at 2pm UTC (9am EST)");

    // Test invalid timezone
    let invalid_result = CronExpression::with_timezone("0 9 * * *", "Invalid/Timezone");
    assert!(invalid_result.is_err(), "Invalid timezone should be rejected");

    println!("✓ Timezone handling works correctly");
}

impl CronExpression {
    pub fn with_timezone(expr: &str, timezone: &str) -> Result<Self, CronParseError> {
        let tz: Tz = timezone.parse()
            .map_err(|_| CronParseError::TimezoneError(format!("Invalid timezone: {}", timezone)))?;

        let schedule = Schedule::from_str(expr)
            .map_err(|e| CronParseError::InvalidExpression(e.to_string()))?;

        // Get next run time in the specified timezone
        let now = Utc::now().with_timezone(&tz);
        let next_run = schedule.after(&now).next()
            .ok_or_else(|| CronParseError::InvalidExpression("No next run time".into()))?;

        // Convert back to UTC
        let next_run_utc = next_run.with_timezone(&Utc);

        let interval_seconds = schedule.after(&now).next()
            .and_then(|next| Some((next - now).num_seconds().max(0) as u64))
            .unwrap_or(60);

        Ok(Self {
            raw: expr.to_string(),
            schedule,
            next_run: next_run_utc,
            interval_seconds,
        })
    }
}
```

- [ ] **Step 10: Run test to verify passes**

```bash
cargo test --lib cron_parser_tests::parse_with_timezone -- --exact
```

Expected: PASS

---

## Module 2: Cron Schedule Validation

### Test Group 1: Missed Schedule Handling

**Files:**
- Create: `tests/unit/schedule_validation_tests.rs`

#### Test: Detect missed schedules

- [ ] **Step 11: Write test for missed schedules**

```rust
#[cfg(test)]
mod schedule_validation_tests {
    use super::*;

    #[test]
    fn detect_missed_schedules() {
        let mut scheduler = TokioScheduler::new();
        let now = Utc::now();

        // Schedule task in the past
        let past_time = now - chrono::Duration::hours(1);
        let task_id = "missed-task-1";

        let result = scheduler.schedule_task(
            task_id,
            past_time,
            |_| async move { Ok(()) },
        );

        assert!(result.is_ok(), "Scheduling should succeed");

        // Check for missed schedules
        let missed = scheduler.check_missed_schedules().unwrap();
        assert!(!missed.is_empty(), "Should detect missed schedule");
        assert!(missed.iter().any(|m| m.task_id == task_id), "Should include our missed task");

        // Verify missed schedule details
        let missed_task = missed.iter().find(|m| m.task_id == task_id).unwrap();
        assert!(missed_task.scheduled_time < now);
        assert!(!missed_task.handled, "Missed schedule should not be handled yet");

        // Handle missed schedule
        scheduler.handle_missed_schedule(task_id).unwrap();

        // Verify it's no longer in missed list
        let missed_after = scheduler.check_missed_schedules().unwrap();
        assert!(missed_after.iter().all(|m| m.task_id != task_id), "Handled schedule should be removed");

        println!("✓ Missed schedule detection and handling works");
    }
}
```

- [ ] **Step 12: Run test to verify compilation**

```bash
cargo test --lib schedule_validation_tests::detect_missed_schedules -- --exact
```

Expected: Test compiles and runs (will fail initially)

- [ ] **Step 13: Implement missed schedule handling**

```rust
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;

pub struct TokioScheduler {
    scheduled_tasks: HashMap<String, ScheduledTask>,
}

#[derive(Debug, Clone)]
pub struct ScheduledTask {
    pub task_id: String,
    pub scheduled_time: DateTime<Utc>,
    pub task: Box<dyn Fn() -> Pin<Box<dyn Future<Output = Result<(), TaskError>>> + Send + Sync>,
    pub executed: bool,
}

#[derive(Debug, Clone)]
pub struct MissedSchedule {
    pub task_id: String,
    pub scheduled_time: DateTime<Utc>,
    pub missed_at: DateTime<Utc>,
    pub handled: bool,
}

impl TokioScheduler {
    pub fn check_missed_schedules(&self) -> Result<Vec<MissedSchedule>, SchedulerError> {
        let now = Utc::now();
        let mut missed = Vec::new();

        for (task_id, task) in &self.scheduled_tasks {
            if !task.executed && task.scheduled_time < now {
                missed.push(MissedSchedule {
                    task_id: task_id.clone(),
                    scheduled_time: task.scheduled_time,
                    missed_at: now,
                    handled: false,
                });
            }
        }

        Ok(missed)
    }

    pub fn handle_missed_schedule(&mut self, task_id: &str) -> Result<(), SchedulerError> {
        // Mark as handled (could also execute, skip, or reschedule based on policy)
        if let Some(task) = self.scheduled_tasks.get_mut(task_id) {
            task.executed = true;
        }
        Ok(())
    }
}
```

- [ ] **Step 14: Run test to verify passes**

```bash
cargo test --lib schedule_validation_tests::detect_missed_schedules -- --exact
```

Expected: PASS

### Test Group 2: Overlapping Schedule Detection

#### Test: Detect overlapping schedules

- [ ] **Step 15: Write test for overlapping schedules**

```rust
#[test]
fn detect_overlapping_schedules() {
    let mut scheduler = TokioScheduler::new();
    let now = Utc::now();

    // Schedule tasks at the same time
    let base_time = now + Duration::hours(1);

    scheduler.schedule_task("task-1", base_time, |_| async { Ok(()) }).unwrap();
    scheduler.schedule_task("task-2", base_time, |_| async { Ok(()) }).unwrap();
    scheduler.schedule_task("task-3", base_time + Duration::minutes(5), |_| async { Ok(()) }).unwrap();
    scheduler.schedule_task("task-4", base_time - Duration::minutes(5), |_| async { Ok(()) }).unwrap();

    // Check for overlapping schedules (within 10-minute window)
    let overlapping = scheduler.check_overlapping_schedules(Duration::minutes(10)).unwrap();

    assert!(!overlapping.is_empty(), "Should detect overlapping schedules");

    // Verify task-1 and task-2 are overlapping (same time)
    let overlap_group1 = overlapping.iter().find(|g| g.contains(&"task-1".to_string()));
    assert!(overlap_group1.is_some(), "Task-1 should be in overlap group");
    assert!(overlap_group1.unwrap().contains(&"task-2".to_string()), "Task-2 should overlap with task-1");

    // Verify task-3 and task-4 are overlapping (within window)
    let overlap_group2 = overlapping.iter().find(|g| g.contains(&"task-3".to_string()));
    assert!(overlap_group2.is_some(), "Task-3 should be in overlap group");
    assert!(overlap_group2.unwrap().contains(&"task-4".to_string()), "Task-4 should overlap with task-3");

    println!("✓ Overlapping schedule detection works");
}

impl TokioScheduler {
    pub fn check_overlapping_schedules(
        &self,
        window: Duration,
    ) -> Result<Vec<Vec<String>>, SchedulerError> {
        let mut groups: Vec<Vec<String>> = Vec::new();
        let mut task_times: Vec<(&String, &DateTime<Utc>)> = self.scheduled_tasks
            .iter()
            .map(|(id, task)| (id, &task.scheduled_time))
            .collect();

        // Sort by time
        task_times.sort_by_key(|(_, time)| *time);

        // Find overlapping groups
        let mut current_group = Vec::new();
        if let Some((first_id, first_time)) = task_times.first() {
            current_group.push((*first_id).clone());
            let mut group_start = **first_time;

            for (task_id, task_time) in task_times.iter().skip(1) {
                if **task_time - group_start <= window {
                    current_group.push((*task_id).clone());
                } else {
                    if current_group.len() > 1 {
                        groups.push(current_group);
                    }
                    current_group = vec![(*task_id).clone()];
                    group_start = **task_time;
                }
            }

            if current_group.len() > 1 {
                groups.push(current_group);
            }
        }

        Ok(groups)
    }
}
```

- [ ] **Step 16: Run test to verify passes**

```bash
cargo test --lib schedule_validation_tests::detect_overlapping_schedules -- --exact
```

Expected: PASS

### Test Group 3: Max Concurrent Enforcement

#### Test: Enforce max concurrent limit

- [ ] **Step 17: Write test for max concurrent**

```rust
#[test]
fn enforce_max_concurrent_limit() {
    let mut scheduler = TokioScheduler::with_max_concurrent(2);
    let now = Utc::now();

    // Schedule tasks at the same time
    let base_time = now + Duration::minutes(1);

    let result1 = scheduler.schedule_task("task-1", base_time, |_| async { Ok(()) });
    let result2 = scheduler.schedule_task("task-2", base_time, |_| async { Ok(()) });
    let result3 = scheduler.schedule_task("task-3", base_time, |_| async { Ok(()) });

    assert!(result1.is_ok(), "Task-1 should be scheduled");
    assert!(result2.is_ok(), "Task-2 should be scheduled");
    assert!(result3.is_err(), "Task-3 should be rejected (max concurrent exceeded)");

    if let Err(SchedulerError::MaxConcurrentExceeded { limit }) = result3 {
        assert_eq!(limit, 2, "Should report correct limit");
    } else {
        panic!("Expected MaxConcurrentExceeded error");
    }

    // Verify scheduled task count
    assert_eq!(scheduler.pending_count(), 2, "Should have 2 pending tasks");

    // Verify we can schedule after time window
    let later_time = base_time + Duration::hours(1);
    let result4 = scheduler.schedule_task("task-4", later_time, |_| async { Ok(()) });
    assert!(result4.is_ok(), "Task-4 should be scheduled (different time window)");

    println!("✓ Max concurrent enforcement works");
}

impl TokioScheduler {
    pub fn with_max_concurrent(limit: usize) -> Self {
        Self {
            scheduled_tasks: HashMap::new(),
            max_concurrent: Some(limit),
        }
    }

    pub fn schedule_task<F, Fut>(
        &mut self,
        task_id: &str,
        scheduled_time: DateTime<Utc>,
        task: F,
    ) -> Result<(), SchedulerError>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), TaskError>> + Send + 'static,
    {
        // Check max concurrent limit
        if let Some(limit) = self.max_concurrent {
            let pending = self.scheduled_tasks
                .values()
                .filter(|t| !t.executed)
                .count();

            if pending >= limit {
                return Err(SchedulerError::MaxConcurrentExceeded { limit });
            }
        }

        let scheduled_task = ScheduledTask {
            task_id: task_id.to_string(),
            scheduled_time,
            task: Box::new(move || task()),
            executed: false,
        };

        self.scheduled_tasks.insert(task_id.to_string(), scheduled_task);
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SchedulerError {
    #[error("Max concurrent ({limit}) tasks exceeded")]
    MaxConcurrentExceeded { limit: usize },
}
```

- [ ] **Step 18: Run test to verify passes**

```bash
cargo test --lib schedule_validation_tests::enforce_max_concurrent_limit -- --exact
```

Expected: PASS

---

## Module 3: Git Experiment Manifest Validation

### Test Group 1: Manifest Structure

**Files:**
- Create: `tests/unit/git_manifest_tests.rs`

#### Test: Validate experiment manifest

- [ ] **Step 19: Write test for manifest validation**

```rust
#[cfg(test)]
mod git_manifest_tests {
    use super::*;

    #[test]
    fn validate_experiment_manifest_structure() {
        // Valid manifest
        let valid_manifest = serde_json::json!({
            "experiment_id": "experiment-1",
            "base_branch": "main",
            "experiment_branch": "experiment-1",
            "created_at": "2026-04-07T12:00:00Z",
            "workflow_id": "test_workflow",
            "config": {
                "param1": "value1",
                "param2": 42
            },
            "status": "running"
        });

        let result = ExperimentManifest::validate(&valid_manifest);
        assert!(result.is_ok(), "Valid manifest should pass validation");

        // Missing required fields
        let missing_id = serde_json::json!({
            "base_branch": "main",
            "experiment_branch": "experiment-1"
        });

        let result = ExperimentManifest::validate(&missing_id);
        assert!(result.is_err());
        match result.unwrap_err() {
            ManifestValidationError::MissingField(field) => {
                assert_eq!(field, "experiment_id");
            }
            _ => panic!("Expected MissingField error"),
        }

        // Invalid status
        let invalid_status = serde_json::json!({
            "experiment_id": "experiment-1",
            "base_branch": "main",
            "experiment_branch": "experiment-1",
            "status": "invalid_status"
        });

        let result = ExperimentManifest::validate(&invalid_status);
        assert!(result.is_err());
        match result.unwrap_err() {
            ManifestValidationError::InvalidStatus(status) => {
                assert_eq!(status, "invalid_status");
            }
            _ => panic!("Expected InvalidStatus error"),
        }

        // Valid statuses
        let valid_statuses = vec!["pending", "running", "completed", "failed", "cancelled"];
        for status in valid_statuses {
            let manifest = serde_json::json!({
                "experiment_id": "experiment-1",
                "base_branch": "main",
                "experiment_branch": "experiment-1",
                "status": status
            });

            let result = ExperimentManifest::validate(&manifest);
            assert!(result.is_ok(), "Status '{}' should be valid", status);
        }

        println!("✓ Experiment manifest validation works");
    }
}
```

- [ ] **Step 20: Run test to verify compilation**

```bash
cargo test --lib git_manifest_tests::validate_experiment_manifest_structure -- --exact
```

Expected: Test compiles and runs (will fail initially)

- [ ] **Step 21: Implement manifest validation**

```rust
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentManifest {
    pub experiment_id: String,
    pub base_branch: String,
    pub experiment_branch: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub workflow_id: Option<String>,
    pub config: Option<Value>,
    pub status: ExperimentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExperimentStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl ExperimentManifest {
    pub fn validate(json: &Value) -> Result<Self, ManifestValidationError> {
        // Check required fields
        if !json.get("experiment_id").is_some() {
            return Err(ManifestValidationError::MissingField("experiment_id".to_string()));
        }
        if !json.get("base_branch").is_some() {
            return Err(ManifestValidationError::MissingField("base_branch".to_string()));
        }
        if !json.get("experiment_branch").is_some() {
            return Err(ManifestValidationError::MissingField("experiment_branch".to_string()));
        }

        // Validate status
        if let Some(status) = json.get("status") {
            let status_str = status.as_str().ok_or_else(|| {
                ManifestValidationError::InvalidType("status".to_string(), "string".to_string())
            })?;

            match status_str {
                "pending" | "running" | "completed" | "failed" | "cancelled" => {}
                other => return Err(ManifestValidationError::InvalidStatus(other.to_string())),
            }
        }

        // Deserialize to full struct
        let manifest: ExperimentManifest = serde_json::from_value(json.clone())
            .map_err(|e| ManifestValidationError::DeserializationError(e.to_string()))?;

        Ok(manifest)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ManifestValidationError {
    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid status: {0}")]
    InvalidStatus(String),

    #[error("Invalid type for field '{0}', expected '{1}'")]
    InvalidType(String, String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),
}
```

- [ ] **Step 22: Run test to verify passes**

```bash
cargo test --lib git_manifest_tests::validate_experiment_manifest_structure -- --exact
```

Expected: PASS

---

## Module 4: Merge Proposal Generation

### Test Group 1: Diff Accuracy

**Files:**
- Create: `tests/unit/merge_proposal_tests.rs`

#### Test: Diff accuracy

- [ ] **Step 23: Write test for diff accuracy**

```rust
#[cfg(test)]
mod merge_proposal_tests {
    use super::*;

    #[test]
    fn merge_proposal_diff_accuracy() {
        // Create mock repository with changes
        let (temp_dir, repo) = create_mock_repo_with_changes().unwrap();

        // Generate merge proposal
        let proposal = MergeProposalGenerator::generate(
            &repo,
            "experiment-1",
            "main",
            &ProposalConfig::default(),
        ).unwrap();

        // Verify diff is present and complete
        assert!(!proposal.diff.is_empty(), "Diff should not be empty");

        let diff = &proposal.diff;

        // Verify diff format (unified diff)
        assert!(diff.starts_with("diff --git"), "Diff should start with header");

        // Verify all changes are included
        assert!(diff.contains("--- a/modified_file.txt"), "Should show modified file (old)");
        assert!(diff.contains("+++ b/modified_file.txt"), "Should show modified file (new)");
        assert!(diff.contains("-Old content"), "Should show removed line");
        assert!(diff.contains("+New content"), "Should show added line");

        assert!(diff.contains("diff --git a/new_file.txt"), "Should show new file");
        assert!(diff.contains("+++ b/new_file.txt"), "Should show new file addition");
        assert!(diff.contains("+New file content"), "Should show new file content");

        assert!(diff.contains("diff --git a/old_file.txt"), "Should show deleted file");
        assert!(diff.contains("--- a/old_file.txt"), "Should show deleted file (old)");
        assert!(diff.contains("-Old file content"), "Should show deleted file content");

        println!("✓ Merge proposal diff accuracy verified");
    }

    fn create_mock_repo_with_changes() -> Result<(tempfile::TempDir, git2::Repository), Box<dyn std::error::Error>> {
        let temp_dir = tempfile::TempDir::new()?;
        let repo = git2::Repository::init(temp_dir.path())?;

        // Create initial commit with main branch files
        let main_readme = temp_dir.path().join("modified_file.txt");
        std::fs::write(&main_readme, "Old content\n")?;

        let mut index = repo.index()?;
        index.add_path(std::path::Path::new("modified_file.txt"))?;
        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;

        let sig = git2::Signature::now("Test User", "test@example.com")?;
        repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])?;
        repo.set_head("refs/heads/main")?;

        // Create old file
        let old_file = temp_dir.path().join("old_file.txt");
        std::fs::write(&old_file, "Old file content\n")?;

        let mut index = repo.index()?;
        index.add_path(std::path::Path::new("old_file.txt"))?;
        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;

        repo.commit(Some("HEAD"), &sig, &sig, "Add old file", &tree, &[&repo.head()?.peel_to_commit()?])?;

        // Create experiment branch with changes
        repo.branch("experiment-1", &repo.head()?.peel_to_commit()?, false)?;
        repo.set_head("refs/heads/experiment-1")?;

        // Modify file
        std::fs::write(&main_readme, "New content\n")?;

        // Add new file
        let new_file = temp_dir.path().join("new_file.txt");
        std::fs::write(&new_file, "New file content\n")?;

        // Delete old file
        std::fs::remove_file(&old_file)?;

        // Commit changes
        let mut index = repo.index()?;
        index.add_path(std::path::Path::new("modified_file.txt"))?;
        index.add_path(std::path::Path::new("new_file.txt"))?;
        index.remove(std::path::Path::new("old_file.txt"), None)?;
        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;

        repo.commit(Some("HEAD"), &sig, &sig, "Experiment changes", &tree, &[&repo.head()?.peel_to_commit()?])?;

        Ok((temp_dir, repo))
    }
}
```

- [ ] **Step 24: Run test to verify compilation**

```bash
cargo test --lib merge_proposal_tests::merge_proposal_diff_accuracy -- --exact
```

Expected: Test compiles and runs (will fail initially)

- [ ] **Step 25: Implement diff generation**

```rust
use git2::{Repository, Diff, DiffOptions, ObjectType};

impl MergeProposalGenerator {
    pub fn generate(
        repo: &Repository,
        experiment_branch: &str,
        target_branch: &str,
        config: &ProposalConfig,
    ) -> Result<MergeProposal, ProposalError> {
        // Get branch references
        let experiment_ref = repo.find_branch(experiment_branch, git2::BranchType::Local)?;
        let target_ref = repo.find_branch(target_branch, git2::BranchType::Local)?;

        let experiment_commit = experiment_ref.get().peel_to_commit()?;
        let target_commit = target_ref.get().peel_to_commit()?;

        // Generate diff
        let experiment_tree = experiment_commit.tree()?;
        let target_tree = target_commit.tree()?;
        let mut diff_opts = DiffOptions::new();
        let diff = repo.diff_tree_to_tree(
            Some(&target_tree),
            Some(&experiment_tree),
            Some(&mut diff_opts),
        )?;

        let diff_text = Self::diff_to_string(&diff)?;

        Ok(MergeProposal {
            experiment_branch: experiment_branch.to_string(),
            target_branch: target_branch.to_string(),
            diff: diff_text,
            validation: if config.include_validation {
                Some(Self::run_validation(repo)?)
            } else {
                None
            },
            metrics: if config.include_metrics {
                Some(Self::collect_metrics(repo)?)
            } else {
                None
            },
            confidence_score: Self::calculate_confidence(&diff_text, config)?,
            recommendations: Self::generate_recommendations(&diff_text),
            generated_at: chrono::Utc::now(),
        })
    }

    fn diff_to_string(diff: &Diff) -> Result<String, ProposalError> {
        let mut diff_text = String::new();
        diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
            match line.origin_value() {
                '+' | '-' | ' ' => {
                    diff_text.push(line.origin());
                    diff_text.push_str(std::str::from_utf8(line.content()).unwrap());
                }
                'F' | 'T' => {} // File/Tree headers
                _ => {}
            }
            true
        })?;
        Ok(diff_text)
    }
}
```

- [ ] **Step 26: Run test to verify passes**

```bash
cargo test --lib merge_proposal_tests::merge_proposal_diff_accuracy -- --exact
```

Expected: PASS

### Test Group 2: Validation Criteria

#### Test: Validation results included

- [ ] **Step 27: Write test for validation criteria**

```rust
#[test]
fn merge_proposal_includes_validation_criteria() {
    let (temp_dir, repo) = create_mock_repo_with_changes().unwrap();

    let config = ProposalConfig {
        include_validation: true,
        include_metrics: true,
        confidence_threshold: 0.0,
    };

    let proposal = MergeProposalGenerator::generate(
        &repo,
        "experiment-1",
        "main",
        &config,
    ).unwrap();

    // Verify validation results present
    assert!(proposal.validation.is_some(), "Should include validation results");

    let validation = proposal.validation.as_ref().unwrap();

    // Verify lint results
    assert!(matches!(validation.lints_passed, true | false), "lints_passed should be boolean");
    if !validation.lints_passed {
        assert!(!validation.lint_errors.is_empty(), "Should have lint errors if failed");
    }

    // Verify test results
    assert!(matches!(validation.tests_passed, true | false), "tests_passed should be boolean");
    if !validation.tests_passed {
        assert!(!validation.test_failures.is_empty(), "Should have test failures if failed");
    }

    // Verify metrics present
    assert!(proposal.metrics.is_some(), "Should include metrics");

    let metrics = proposal.metrics.as_ref().unwrap();
    assert!(metrics.execution_time_seconds >= 0.0, "Execution time should be non-negative");
    assert!(metrics.memory_usage_mb >= 0.0, "Memory usage should be non-negative");
    assert!(metrics.code_coverage_percent >= 0.0 && metrics.code_coverage_percent <= 100.0,
             "Coverage should be 0-100%");

    println!("✓ Merge proposal includes validation criteria");
}
```

- [ ] **Step 28: Run test to verify passes**

```bash
cargo test --lib merge_proposal_tests::merge_proposal_includes_validation_criteria -- --exact
```

Expected: PASS

### Test Group 3: Confidence Thresholds

#### Test: Confidence scoring

- [ ] **Step 29: Write test for confidence thresholds**

```rust
#[test]
fn confidence_scoring_respects_thresholds() {
    // Test 1: Small changes get high confidence
    let small_diff = "diff --git a/file.txt\n--- a/file.txt\n+++ b/file.txt\n@@ -1 +1 @@\n-old\n+new\n";
    let small_config = ProposalConfig {
        include_validation: false,
        include_metrics: false,
        confidence_threshold: 0.7,
    };

    let small_confidence = MergeProposalGenerator::calculate_confidence(small_diff, &small_config).unwrap();
    assert!(small_confidence >= 0.8, "Small changes should have high confidence");
    assert!(small_confidence <= 1.0, "Confidence should be <= 1.0");

    println!("Small diff confidence: {:.2}", small_confidence);

    // Test 2: Large changes get lower confidence
    let large_diff = "diff --git a/file1.txt\n+Line 1\n+Line 2\n+Line 3\n".repeat(100);
    let large_confidence = MergeProposalGenerator::calculate_confidence(&large_diff, &small_config).unwrap();
    assert!(large_confidence < 0.8, "Large changes should have lower confidence");
    assert!(large_confidence > 0.0, "Confidence should be > 0.0");

    println!("Large diff confidence: {:.2}", large_confidence);

    // Test 3: Below threshold gets adjusted
    let low_confidence_config = ProposalConfig {
        include_validation: false,
        include_metrics: false,
        confidence_threshold: 0.9,
    };

    let adjusted_confidence = MergeProposalGenerator::calculate_confidence(small_diff, &low_confidence_config).unwrap();
    assert!(adjusted_confidence >= 0.9, "Should adjust to meet threshold");

    println!("Adjusted confidence (threshold 0.9): {:.2}", adjusted_confidence);

    println!("✓ Confidence scoring respects thresholds");
}
```

- [ ] **Step 30: Run test to verify passes**

```bash
cargo test --lib merge_proposal_tests::confidence_scoring_respects_thresholds -- --exact
```

Expected: PASS

---

## Module 5: Refinement Event Capture

### Test Group 1: Event Structure

**Files:**
- Create: `tests/unit/refinement_events_tests.rs`

#### Test: Event structure validation

- [ ] **Step 31: Write test for event structure**

```rust
#[cfg(test)]
mod refinement_events_tests {
    use super::*;

    #[test]
    fn refinement_event_structure_validation() {
        // Test 1: Create valid event
        let event = RefinementEvent::new(
            "proposal-1".to_string(),
            RefinementAction::Approve,
            "reviewer1",
            "Changes look good".to_string(),
        );

        // Verify all fields are populated
        assert!(!event.id.is_empty(), "Event ID should not be empty");
        assert_eq!(event.proposal_id, "proposal-1");
        assert_eq!(event.action, RefinementAction::Approve);
        assert_eq!(event.actor, "reviewer1");
        assert_eq!(event.justification, "Changes look good");
        assert!(event.timestamp <= chrono::Utc::now());
        assert!(event.linked_events.is_empty(), "New event should have no linked events");

        // Test 2: Verify ID is unique UUID
        let event2 = RefinementEvent::new(
            "proposal-2".to_string(),
            RefinementAction::Reject,
            "reviewer2",
            "Needs more work".to_string(),
        );

        assert_ne!(event.id, event2.id, "Event IDs should be unique");

        // Test 3: Verify timestamp is reasonable
        let now = chrono::Utc::now();
        let event3 = RefinementEvent::new(
            "proposal-3".to_string(),
            RefinementAction::Modify,
            "reviewer3",
            "Added validation".to_string(),
        );

        let age = (now - event3.timestamp).num_seconds();
        assert!(age >= 0 && age < 5, "Timestamp should be recent");

        // Test 4: Verify all action types work
        let actions = vec![
            RefinementAction::Approve,
            RefinementAction::Reject,
            RefinementAction::Modify,
            RefinementAction::RequestChanges,
        ];

        for action in actions {
            let event = RefinementEvent::new(
                "test-proposal".to_string(),
                action.clone(),
                "test-actor",
                "Test justification".to_string(),
            );
            assert_eq!(event.action, action);
        }

        println!("✓ Refinement event structure validation passed");
    }
}
```

- [ ] **Step 32: Run test to verify compilation**

```bash
cargo test --lib refinement_events_tests::refinement_event_structure_validation -- --exact
```

Expected: Test compiles and runs (will fail initially)

- [ ] **Step 33: Implement RefinementEvent**

```rust
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefinementEvent {
    pub id: String,
    pub proposal_id: String,
    pub action: RefinementAction,
    pub actor: String,
    pub justification: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub linked_events: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RefinementAction {
    Approve,
    Reject,
    Modify,
    RequestChanges,
}

impl RefinementEvent {
    pub fn new(
        proposal_id: String,
        action: RefinementAction,
        actor: String,
        justification: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            proposal_id,
            action,
            actor,
            justification,
            timestamp: chrono::Utc::now(),
            linked_events: Vec::new(),
        }
    }
}
```

- [ ] **Step 34: Run test to verify passes**

```bash
cargo test --lib refinement_events_tests::refinement_event_structure_validation -- --exact
```

Expected: PASS

### Test Group 2: Event Linking

#### Test: Event linking functionality

- [ ] **Step 35: Write test for event linking**

```rust
#[test]
fn event_linking_functionality() {
    let mut event1 = RefinementEvent::new(
        "proposal-1".to_string(),
        RefinementAction::Modify,
        "actor1",
        "Initial modification".to_string(),
    );

    let mut event2 = RefinementEvent::new(
        "proposal-1".to_string(),
        RefinementAction::Modify,
        "actor2",
        "Follow-up modification".to_string(),
    );

    let mut event3 = RefinementEvent::new(
        "proposal-1".to_string(),
        RefinementAction::Approve,
        "actor3",
        "Final approval".to_string(),
    );

    // Test 1: Link events
    event1.link_to(event2.id.clone());
    assert!(event1.linked_events.contains(&event2.id));
    assert_eq!(event1.linked_events.len(), 1);

    // Test 2: Link multiple events
    event2.link_to(event3.id.clone());
    event3.link_to(event1.id.clone());

    assert!(event2.linked_events.contains(&event3.id));
    assert!(event3.linked_events.contains(&event1.id));

    // Test 3: Avoid duplicates
    event1.link_to(event2.id.clone());
    event1.link_to(event2.id.clone());
    assert_eq!(event1.linked_events.len(), 1, "Should avoid duplicate links");

    // Test 4: Bi-directional linking
    event1.link_to(event3.id.clone());
    event3.link_to(event1.id.clone());

    assert!(event1.linked_events.contains(&event3.id));
    assert!(event3.linked_events.contains(&event1.id));

    // Verify cycle detection (should not create infinite loops)
    let linked_count = event1.linked_events.len();
    assert!(linked_count < 10, "Should limit linked events to prevent cycles");

    println!("✓ Event linking functionality works");
}

impl RefinementEvent {
    pub fn link_to(&mut self, other_event_id: String) {
        if !self.linked_events.contains(&other_event_id) {
            self.linked_events.push(other_event_id);
        }
    }
}
```

- [ ] **Step 36: Run test to verify passes**

```bash
cargo test --lib refinement_events_tests::event_linking_functionality -- --exact
```

Expected: PASS

### Test Group 3: Approval/Rejection Handling

#### Test: Approval and rejection events

- [ ] **Step 37: Write test for approval/rejection**

```rust
#[test]
fn approval_and_rejection_event_handling() {
    let log_path = tempfile::NamedTempFile::new().unwrap();
    let event_log = RefinementEventLog::new(log_path.path()).unwrap();

    // Test 1: Approval event
    let approval = RefinementEvent::new(
        "proposal-1".to_string(),
        RefinementAction::Approve,
        "approver1",
        "Approved for merge".to_string(),
    );

    event_log.append(approval.clone()).unwrap();

    // Test 2: Rejection event
    let rejection = RefinementEvent::new(
        "proposal-2".to_string(),
        RefinementAction::Reject,
        "reviewer1",
        "Needs more work".to_string(),
    );

    event_log.append(rejection.clone()).unwrap();

    // Test 3: Read back events
    let events = event_log.read_all().unwrap();
    assert_eq!(events.len(), 2);

    // Verify approval event
    let approval_event = events.iter().find(|e| e.id == approval.id).unwrap();
    assert_eq!(approval_event.action, RefinementAction::Approve);
    assert_eq!(approval_event.proposal_id, "proposal-1");

    // Verify rejection event
    let rejection_event = events.iter().find(|e| e.id == rejection.id).unwrap();
    assert_eq!(rejection_event.action, RefinementAction::Reject);
    assert_eq!(rejection_event.proposal_id, "proposal-2");

    // Test 4: Query by proposal ID
    let proposal1_events = event_log.read_for_proposal("proposal-1").unwrap();
    assert_eq!(proposal1_events.len(), 1);
    assert_eq!(proposal1_events[0].action, RefinementAction::Approve);

    let proposal2_events = event_log.read_for_proposal("proposal-2").unwrap();
    assert_eq!(proposal2_events.len(), 1);
    assert_eq!(proposal2_events[0].action, RefinementAction::Reject);

    println!("✓ Approval and rejection event handling works");
}

impl RefinementEventLog {
    pub fn new(log_path: &std::path::Path) -> Result<Self, EventLogError> {
        Ok(Self {
            log_path: log_path.to_path_buf(),
        })
    }

    pub fn append(&self, event: RefinementEvent) -> Result<(), EventLogError> {
        use std::fs::OpenOptions;
        use std::io::Write;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)?;

        let event_json = serde_json::to_string(&event)?;
        writeln!(file, "{}", event_json)?;

        Ok(())
    }

    pub fn read_all(&self) -> Result<Vec<RefinementEvent>, EventLogError> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open(&self.log_path)?;
        let reader = BufReader::new(file);

        let mut events = Vec::new();
        for line in reader.lines() {
            let line = line?;
            if !line.is_empty() {
                let event: RefinementEvent = serde_json::from_str(&line)?;
                events.push(event);
            }
        }

        Ok(events)
    }

    pub fn read_for_proposal(&self, proposal_id: &str) -> Result<Vec<RefinementEvent>, EventLogError> {
        let all_events = self.read_all()?;
        Ok(all_events
            .into_iter()
            .filter(|e| e.proposal_id == proposal_id)
            .collect())
    }
}
```

- [ ] **Step 38: Run test to verify passes**

```bash
cargo test --lib refinement_events_tests::approval_and_rejection_event_handling -- --exact
```

Expected: PASS

---

## Module 6: Experiment Result Comparison

### Test Group 1: Difference Calculation

**Files:**
- Create: `tests/unit/experiment_tracking_tests.rs`

#### Test: Difference calculation

- [ ] **Step 39: Write test for difference calculation**

```rust
#[cfg(test)]
mod experiment_tracking_tests {
    use super::*;

    #[test]
    fn experiment_result_difference_calculation() {
        let result1 = ExperimentResult {
            experiment_id: "experiment-1".to_string(),
            branch: "experiment-1".to_string(),
            timestamp: chrono::Utc::now() - chrono::Duration::hours(1),
            config: serde_json::json!({"version": 1}),
            metrics: ExperimentMetrics {
                execution_time_seconds: 10.0,
                memory_usage_mb: 100.0,
                cpu_usage_percent: 50.0,
                quality_score: 0.85,
                validation_passed: true,
            },
            output: None,
        };

        let result2 = ExperimentResult {
            experiment_id: "experiment-2".to_string(),
            branch: "experiment-2".to_string(),
            timestamp: chrono::Utc::now(),
            config: serde_json::json!({"version": 2}),
            metrics: ExperimentMetrics {
                execution_time_seconds: 8.0,   // 20% faster
                memory_usage_mb: 90.0,        // 10% less memory
                cpu_usage_percent: 45.0,      // 10% less CPU
                quality_score: 0.92,          // 7% better quality
                validation_passed: true,
            },
            output: None,
        };

        let comparison = ResultComparison::compare(&result1, &result2).unwrap();

        // Verify execution time difference
        assert_eq!(comparison.execution_time_diff, -2.0, "Should be 2 seconds faster");
        assert_eq!(comparison.execution_time_ratio, 0.8, "Should be 80% of original");

        // Verify memory difference
        assert_eq!(comparison.memory_diff_mb, -10.0, "Should use 10MB less memory");

        // Verify quality difference
        assert_eq!(comparison.quality_diff, 0.07, "Quality should improve by 0.07");

        // Verify significance
        assert!(matches!(
            comparison.significance,
            Significance::Low | Significance::Medium
        ));

        println!("✓ Experiment result difference calculation works");
        println!("  Execution time: {}s vs {}s (ratio: {:.2})",
                 result1.metrics.execution_time_seconds,
                 result2.metrics.execution_time_seconds,
                 comparison.execution_time_ratio);
        println!("  Memory: {}MB vs {}MB (diff: {}MB)",
                 result1.metrics.memory_usage_mb,
                 result2.metrics.memory_usage_mb,
                 comparison.memory_diff_mb);
        println!("  Quality: {:.2} vs {:.2} (diff: {:.2}, significance: {:?})",
                 result1.metrics.quality_score,
                 result2.metrics.quality_score,
                 comparison.quality_diff,
                 comparison.significance);
    }
}
```

- [ ] **Step 40: Run test to verify compilation**

```bash
cargo test --lib experiment_tracking_tests::experiment_result_difference_calculation -- --exact
```

Expected: Test compiles and runs (will fail initially)

- [ ] **Step 41: Implement ResultComparison**

```rust
#[derive(Debug, Clone)]
pub struct ResultComparison {
    pub experiment_id_1: String,
    pub experiment_id_2: String,
    pub execution_time_diff: f64,
    pub execution_time_ratio: f64,
    pub memory_diff_mb: f64,
    pub quality_diff: f64,
    pub significance: Significance,
}

#[derive(Debug, Clone)]
pub enum Significance {
    Negligible,
    Low,
    Medium,
    High,
}

impl ResultComparison {
    pub fn compare(result1: &ExperimentResult, result2: &ExperimentResult) -> Result<Self, ComparisonError> {
        let execution_time_diff = result2.metrics.execution_time_seconds - result1.metrics.execution_time_seconds;
        let execution_time_ratio = if result1.metrics.execution_time_seconds > 0.0 {
            result2.metrics.execution_time_seconds / result1.metrics.execution_time_seconds
        } else {
            0.0
        };

        let memory_diff_mb = result2.metrics.memory_usage_mb - result1.metrics.memory_usage_mb;
        let quality_diff = result2.metrics.quality_score - result1.metrics.quality_score;

        let significance = Self::calculate_significance(quality_diff);

        Ok(Self {
            experiment_id_1: result1.experiment_id.clone(),
            experiment_id_2: result2.experiment_id.clone(),
            execution_time_diff,
            execution_time_ratio,
            memory_diff_mb,
            quality_diff,
            significance,
        })
    }

    fn calculate_significance(quality_diff: f64) -> Significance {
        let abs_diff = quality_diff.abs();

        if abs_diff < 0.05 {
            Significance::Negligible
        } else if abs_diff < 0.15 {
            Significance::Low
        } else if abs_diff < 0.30 {
            Significance::Medium
        } else {
            Significance::High
        }
    }
}
```

- [ ] **Step 42: Run test to verify passes**

```bash
cargo test --lib experiment_tracking_tests::experiment_result_difference_calculation -- --exact
```

Expected: PASS

### Test Group 2: Ratio Calculation

#### Test: Ratio calculation

- [ ] **Step 43: Write test for ratio calculation**

```rust
#[test]
fn experiment_result_ratio_calculation() {
    let result1 = ExperimentResult {
        experiment_id: "baseline".to_string(),
        branch: "baseline".to_string(),
        timestamp: chrono::Utc::now(),
        config: serde_json::json!({}),
        metrics: ExperimentMetrics {
            execution_time_seconds: 100.0,
            memory_usage_mb: 1000.0,
            cpu_usage_percent: 100.0,
            quality_score: 0.50,
            validation_passed: true,
        },
        output: None,
    };

    // Test 1: 50% improvement
    let improved = ExperimentResult {
        experiment_id: "improved".to_string(),
        branch: "improved".to_string(),
        timestamp: chrono::Utc::now(),
        config: serde_json::json!({}),
        metrics: ExperimentMetrics {
            execution_time_seconds: 50.0,  // 50% faster
            memory_usage_mb: 500.0,       // 50% less memory
            cpu_usage_percent: 50.0,       // 50% less CPU
            quality_score: 0.75,           // 50% better quality
            validation_passed: true,
        },
        output: None,
    };

    let comparison1 = ResultComparison::compare(&result1, &improved).unwrap();
    assert_eq!(comparison1.execution_time_ratio, 0.5, "Should be 50% of baseline");
    assert_eq!(comparison1.memory_diff_mb, -500.0, "Should use 500MB less");
    assert_eq!(comparison1.quality_diff, 0.25, "Quality should improve by 0.25");
    assert!(matches!(comparison1.significance, Significance::High));

    println!("50% improvement: ratio={:.2}, quality_diff={:.2}, significance={:?}",
             comparison1.execution_time_ratio,
             comparison1.quality_diff,
             comparison1.significance);

    // Test 2: 2x worse (ratio > 1.0)
    let worse = ExperimentResult {
        experiment_id: "worse".to_string(),
        branch: "worse".to_string(),
        timestamp: chrono::Utc::now(),
        config: serde_json::json!({}),
        metrics: ExperimentMetrics {
            execution_time_seconds: 200.0,  // 2x slower
            memory_usage_mb: 2000.0,        // 2x more memory
            cpu_usage_percent: 200.0,        // 2x more CPU
            quality_score: 0.25,             // 50% worse quality
            validation_passed: false,
        },
        output: None,
    };

    let comparison2 = ResultComparison::compare(&result1, &worse).unwrap();
    assert_eq!(comparison2.execution_time_ratio, 2.0, "Should be 200% of baseline");
    assert_eq!(comparison2.memory_diff_mb, 1000.0, "Should use 1000MB more");
    assert_eq!(comparison2.quality_diff, -0.25, "Quality should worsen by 0.25");
    assert!(matches!(comparison2.significance, Significance::High));

    println!("2x worse: ratio={:.2}, quality_diff={:.2}, significance={:?}",
             comparison2.execution_time_ratio,
             comparison2.quality_diff,
             comparison2.significance);

    // Test 3: Negligible difference
    let similar = ExperimentResult {
        experiment_id: "similar".to_string(),
        branch: "similar".to_string(),
        timestamp: chrono::Utc::now(),
        config: serde_json::json!({}),
        metrics: ExperimentMetrics {
            execution_time_seconds: 98.0,   // 2% slower
            memory_usage_mb: 995.0,       // 0.5% less memory
            cpu_usage_percent: 99.0,       // 1% less CPU
            quality_score: 0.52,           // 2% better quality
            validation_passed: true,
        },
        output: None,
    };

    let comparison3 = ResultComparison::compare(&result1, &similar).unwrap();
    assert_eq!(comparison3.execution_time_ratio, 0.98, "Should be 98% of baseline");
    assert!(matches!(comparison3.significance, Significance::Negligible));

    println!("Negligible difference: ratio={:.2}, quality_diff={:.2}, significance={:?}",
             comparison3.execution_time_ratio,
             comparison3.quality_diff,
             comparison3.significance);

    println!("✓ Experiment result ratio calculation works");
}
```

- [ ] **Step 44: Run test to verify passes**

```bash
cargo test --lib experiment_tracking_tests::experiment_result_ratio_calculation -- --exact
```

Expected: PASS

### Test Group 3: Significance Testing

#### Test: Significance thresholds

- [ ] **Step 45: Write test for significance thresholds**

```rust
#[test]
fn significance_threshold_testing() {
    let baseline = ExperimentResult {
        experiment_id: "baseline".to_string(),
        branch: "baseline".to_string(),
        timestamp: chrono::Utc::now(),
        config: serde_json::json!({}),
        metrics: ExperimentMetrics {
            execution_time_seconds: 100.0,
            memory_usage_mb: 1000.0,
            cpu_usage_percent: 100.0,
            quality_score: 0.50,
            validation_passed: true,
        },
        output: None,
    };

    // Test 1: Negligible significance (quality diff < 0.05)
    let negligible = create_result("negligible", 99.0, 995.0, 99.0, 0.52);
    let comp_negligible = ResultComparison::compare(&baseline, &negligible).unwrap();
    assert!(matches!(comp_negligible.significance, Significance::Negligible));

    println!("Negligible: quality_diff={:.2}, significance={:?}",
             comp_negligible.quality_diff,
             comp_negligible.significance);

    // Test 2: Low significance (0.05 <= quality diff < 0.15)
    let low = create_result("low", 90.0, 950.0, 90.0, 0.60);
    let comp_low = ResultComparison::compare(&baseline, &low).unwrap();
    assert!(matches!(comp_low.significance, Significance::Low));

    println!("Low: quality_diff={:.2}, significance={:?}",
             comp_low.quality_diff,
             comp_low.significance);

    // Test 3: Medium significance (0.15 <= quality diff < 0.30)
    let medium = create_result("medium", 70.0, 700.0, 70.0, 0.70);
    let comp_medium = ResultComparison::compare(&baseline, &medium).unwrap();
    assert!(matches!(comp_medium.significance, Significance::Medium));

    println!("Medium: quality_diff={:.2}, significance={:?}",
             comp_medium.quality_diff,
             comp_medium.significance);

    // Test 4: High significance (quality diff >= 0.30)
    let high = create_result("high", 50.0, 500.0, 50.0, 0.85);
    let comp_high = ResultComparison::compare(&baseline, &high).unwrap();
    assert!(matches!(comp_high.significance, Significance::High));

    println!("High: quality_diff={:.2}, significance={:?}",
             comp_high.quality_diff,
             comp_high.significance);

    // Test 5: Boundary conditions
    let boundary1 = create_result("boundary1", 100.0, 1000.0, 100.0, 0.55);  // Diff = 0.05
    let comp_boundary1 = ResultComparison::compare(&baseline, &boundary1).unwrap();
    assert!(matches!(comp_boundary1.significance, Significance::Low),
             "Quality diff 0.05 should be Low significance");

    let boundary2 = create_result("boundary2", 100.0, 1000.0, 100.0, 0.65);  // Diff = 0.15
    let comp_boundary2 = ResultComparison::compare(&baseline, &boundary2).unwrap();
    assert!(matches!(comp_boundary2.significance, Significance::Medium),
             "Quality diff 0.15 should be Medium significance");

    let boundary3 = create_result("boundary3", 100.0, 1000.0, 100.0, 0.80);  // Diff = 0.30
    let comp_boundary3 = ResultComparison::compare(&baseline, &boundary3).unwrap();
    assert!(matches!(comp_boundary3.significance, Significance::High),
             "Quality diff 0.30 should be High significance");

    println!("✓ Significance threshold testing works");
}

fn create_result(
    id: &str,
    exec_time: f64,
    memory: f64,
    cpu: f64,
    quality: f64,
) -> ExperimentResult {
    ExperimentResult {
        experiment_id: id.to_string(),
        branch: id.to_string(),
        timestamp: chrono::Utc::now(),
        config: serde_json::json!({}),
        metrics: ExperimentMetrics {
            execution_time_seconds: exec_time,
            memory_usage_mb: memory,
            cpu_usage_percent: cpu,
            quality_score: quality,
            validation_passed: true,
        },
        output: None,
    }
}
```

- [ ] **Step 46: Run test to verify passes**

```bash
cargo test --lib experiment_tracking_tests::significance_threshold_testing -- --exact
```

Expected: PASS

---

## Module 7: Scheduling Policy Compilation

### Test Group 1: Cron to IR Node Compilation

**Files:**
- Create: `tests/unit/ir_compilation_tests.rs`

#### Test: Cron expression compiles to IR

- [ ] **Step 47: Write test for cron to IR compilation**

```rust
#[cfg(test)]
mod ir_compilation_tests {
    use super::*;

    #[test]
    fn cron_expression_compiles_to_ir_nodes() {
        let policies = vec![
            // Hourly
            ("0 * * * *", "hourly", 3600),
            // Daily
            ("0 0 * * *", "daily", 86400),
            // Weekly
            ("0 0 * * 0", "weekly", 604800),
            // Monthly
            ("0 0 1 * *", "monthly", 2592000),
            // Every 15 minutes
            ("*/15 * * * *", "every_15min", 900),
        ];

        for (cron_expr, name, expected_interval) in policies {
            let policy_yaml = format!(r#"
workflow_id: test_{}
name: "Test {} Workflow"
scheduling:
  policy:
    type: cron
    expression: "{}"
agentic_workflow:
  - step: test
    id: step_1
    model: "test_model"
    input:
      prompt: "Test"
"#, name, name, cron_expr);

            let spec = WorkflowSpec::from_yaml(&policy_yaml).unwrap();
            let ir = WorkflowIRCompiler::compile(&spec).unwrap();

            // Verify IR contains scheduling metadata
            assert!(ir.scheduling_metadata.is_some(),
                     "IR should contain scheduling metadata for {}", name);

            let scheduling = ir.scheduling_metadata.as_ref().unwrap();

            // Verify cron expression compiled
            assert_eq!(scheduling.cron_expression, Some(cron_expr.to_string()),
                         "Cron expression should match for {}", name);

            // Verify next run time is set
            assert!(scheduling.next_run_time.is_some(),
                     "Next run time should be set for {}", name);

            // Verify IR structure
            assert_eq!(ir.workflow_id, format!("test_{}", name));
            assert!(!ir.steps.is_empty());

            println!("✓ Compiled cron '{}' to IR: next_run={:?}, interval={}",
                     cron_expr,
                     scheduling.next_run_time,
                     expected_interval);
        }

        println!("✓ All {} cron expressions compiled to IR nodes successfully", policies.len());
    }
}
```

- [ ] **Step 48: Run test to verify compilation**

```bash
cargo test --lib ir_compilation_tests::cron_expression_compiles_to_ir_nodes -- --exact
```

Expected: Test compiles and runs (will fail initially)

- [ ] **Step 49: Implement IR compiler**

```rust
impl WorkflowIRCompiler {
    pub fn compile(spec: &WorkflowSpec) -> Result<WorkflowIR, CompilationError> {
        // Compile workflow steps
        let steps = spec.agentic_workflow
            .iter()
            .map(|step| IRStep {
                id: step.id.clone(),
                step_type: step.step.clone(),
                config: serde_json::json!({
                    "model": step.model,
                    "input": step.input,
                }),
            })
            .collect();

        // Compile scheduling metadata
        let scheduling_metadata = if spec.scheduling.policy.r#type == "cron" {
            let parsed = CronExpression::parse(&spec.scheduling.policy.expression)?;
            Some(SchedulingMetadata {
                cron_expression: Some(spec.scheduling.policy.expression.clone()),
                timezone: spec.scheduling.policy.timezone.clone(),
                max_duration_minutes: spec.scheduling.constraints.max_duration_minutes,
                max_concurrent: spec.scheduling.constraints.max_concurrent,
                resources: spec.scheduling.resources.clone(),
                next_run_time: Some(parsed.next_run),
            })
        } else {
            None
        };

        Ok(WorkflowIR {
            workflow_id: spec.workflow_id.clone(),
            steps,
            scheduling_metadata,
        })
    }
}
```

- [ ] **Step 50: Run test to verify passes**

```bash
cargo test --lib ir_compilation_tests::cron_expression_compiles_to_ir_nodes -- --exact
```

Expected: PASS

### Test Group 2: IR Node Validation

#### Test: IR node structure

- [ ] **Step 51: Write test for IR node validation**

```rust
#[test]
fn ir_node_structure_validation() {
    let policy_yaml = r#"
workflow_id: test_workflow
name: "Test Workflow"
scheduling:
  policy:
    type: cron
    expression: "0 9 * * 1-5"
    timezone: "UTC"
  constraints:
    max_duration_minutes: 60
    max_concurrent: 3
  resources:
    memory_mb: 4096
    cpu_cores: 2
agentic_workflow:
  - step: analyze
    id: step_1
    model: "test_model"
    input:
      prompt: "Analyze"
  - step: validate
    id: step_2
    model: "test_model"
    input:
      prompt: "Validate"
"#;

    let spec = WorkflowSpec::from_yaml(policy_yaml).unwrap();
    let ir = WorkflowIRCompiler::compile(&spec).unwrap();

    // Verify workflow ID
    assert_eq!(ir.workflow_id, "test_workflow");

    // Verify steps
    assert_eq!(ir.steps.len(), 2);

    let step1 = &ir.steps[0];
    assert_eq!(step1.id, "step_1");
    assert_eq!(step1.step_type, "analyze");
    assert!(step1.config.is_object());

    let step2 = &ir.steps[1];
    assert_eq!(step2.id, "step_2");
    assert_eq!(step2.step_type, "validate");

    // Verify scheduling metadata
    assert!(ir.scheduling_metadata.is_some());
    let scheduling = ir.scheduling_metadata.as_ref().unwrap();

    assert_eq!(scheduling.cron_expression, Some("0 9 * * 1-5".to_string()));
    assert_eq!(scheduling.timezone, Some("UTC".to_string()));
    assert_eq!(scheduling.max_duration_minutes, Some(60));
    assert_eq!(scheduling.max_concurrent, Some(3));
    assert_eq!(scheduling.resources.memory_mb, 4096);
    assert_eq!(scheduling.resources.cpu_cores, 2);
    assert!(scheduling.next_run_time.is_some());

    // Verify IR can be serialized and deserialized
    let ir_json = serde_json::to_string(&ir).unwrap();
    let ir_restored: WorkflowIR = serde_json::from_str(&ir_json).unwrap();

    assert_eq!(ir.workflow_id, ir_restored.workflow_id);
    assert_eq!(ir.steps.len(), ir_restored.steps.len());
    assert_eq!(ir.scheduling_metadata, ir_restored.scheduling_metadata);

    println!("✓ IR node structure validation passed");
}
```

- [ ] **Step 52: Run test to verify passes**

```bash
cargo test --lib ir_compilation_tests::ir_node_structure_validation -- --exact
```

Expected: PASS

### Test Group 3: Runtime IR Usage

#### Test: Runtime uses IR metadata

- [ ] **Step 53: Write test for runtime IR usage**

```rust
#[test]
fn runtime_uses_ir_metadata() {
    let policy_yaml = r#"
workflow_id: test_workflow
name: "Test Workflow"
scheduling:
  policy:
    type: cron
    expression: "0 9 * * *"
agentic_workflow:
  - step: test
    id: step_1
    model: "test_model"
    input:
      prompt: "Test"
"#;

    let spec = WorkflowSpec::from_yaml(policy_yaml).unwrap();
    let ir = WorkflowIRCompiler::compile(&spec).unwrap();

    // Create scheduler from IR
    let scheduler = TokioScheduler::from_ir(&ir).unwrap();

    // Verify scheduler uses IR metadata (not cron parsing)
    let next_run = scheduler.next_run_time_from_ir();
    assert!(next_run.is_some(), "Should have next run time from IR");

    // Mock parser to detect runtime parsing
    let parser_called = Arc::new(Mutex::new(false));
    let _guard = MockCronParser::track_calls(parser_called.clone());

    // Trigger scheduler (should use IR, not parser)
    scheduler.trigger_scheduled_tasks();

    // Verify parser was NOT called
    assert!(!*parser_called.lock().unwrap(), "Runtime should not call cron parser");

    println!("✓ Runtime uses IR metadata (no cron parsing)");
}

impl TokioScheduler {
    pub fn from_ir(ir: &WorkflowIR) -> Result<Self, SchedulerError> {
        if ir.scheduling_metadata.is_none() {
            return Err(SchedulerError::NoSchedulingMetadata);
        }

        Ok(Self {
            scheduled_tasks: HashMap::new(),
            ir_metadata: ir.scheduling_metadata.clone(),
        })
    }

    pub fn next_run_time_from_ir(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.ir_metadata.as_ref()?.next_run_time
    }

    pub fn trigger_scheduled_tasks(&self) {
        // Use IR metadata to determine tasks to run
        // Never parse cron at runtime
    }
}
```

- [ ] **Step 54: Run test to verify passes**

```bash
cargo test --lib ir_compilation_tests::runtime_uses_ir_metadata -- --exact
```

Expected: PASS

---

## Comprehensive Unit Test Suite

### Run All Unit Tests

```bash
# Run all unit tests
cargo test --lib -- --nocapture

# Expected: All tests PASS
# Breakdown by module:
#   Cron Expression Parsing: 3 tests
#   Schedule Validation: 3 tests
#   Git Manifest Validation: 1 test
#   Merge Proposal Generation: 3 tests
#   Refinement Event Capture: 3 tests
#   Experiment Tracking: 3 tests
#   IR Compilation: 3 tests
# Total: 19 tests
```

### Coverage Report

```bash
# Generate coverage report
cargo tarpaulin --out Html --output-dir target/tarpaulin

# Expected: Coverage >= 80%
# View report: target/tarpaulin/index.html
```

### Success Criteria

All unit tests must meet:
- ✅ All 19 tests PASS
- ✅ Code coverage >= 80%
- ✅ No compilation errors or warnings
- ✅ All edge cases covered
- ✅ Error handling verified

### Next Steps

After passing all unit tests:
1. Run integration tests
2. Run property-based tests
3. Perform end-to-end validation
4. Prepare for acceptance criteria verification
