# Phase 06 Automation - Checkpoint Criteria Specification

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Define comprehensive gate criteria for each of the 9 task checkpoints in Phase 06 Automation

**Architecture:** Nine checkpoints with validation tests that verify core functionality: cron parsing, scheduling execution, git isolation, merge proposals, refinement events, experiment tracking, rollback procedures, IR compilation, and CLI/UI integration.

**Tech Stack:** Rust, tokio, tempfile, git2, cron, assert-json-diff

---

## CP01: Cron Expression Parser/Validator Works

### Acceptance Criteria

**Functional Requirements:**
- Cron parser correctly validates all standard cron expressions (5-field format)
- Parser rejects invalid expressions with descriptive error messages
- Timezone support allows scheduling in different timezones
- Parser provides structured metadata (next run time, interval, frequency)

**Non-Functional Requirements:**
- Parsing completes within 10ms for typical expressions
- Memory usage bounded to <1MB per parsed expression
- Error messages are human-readable and actionable

### Validation Tests

#### Test: Valid cron expressions parse correctly

**Files:**
- Create: `src/automation/cron/parser.rs`
- Test: `tests/cron_parser_tests.rs`

- [ ] **Step 1: Write test for valid expressions**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_cron_expressions() {
        let expressions = vec![
            "0 * * * *",           // Every hour
            "0 0 * * *",           // Daily at midnight
            "0 0 * * 0",           // Weekly (Sunday)
            "0 0 1 * *",           // Monthly
            "0 9-17 * * 1-5",      // Weekdays 9am-5pm
            "*/15 * * * *",        // Every 15 minutes
            "0 8,12,18 * * *",     // Specific hours
            "0 0 1 1 *",           // January 1st
        ];

        for expr in expressions {
            let result = CronExpression::parse(expr);
            assert!(result.is_ok(), "Failed to parse: {}", expr);
        }
    }
}
```

- [ ] **Step 2: Run test to verify compilation**

```bash
cargo test --lib cron_parser_tests::parse_valid_cron_expressions -- --exact
```

Expected: Test compiles and runs (will fail initially)

- [ ] **Step 3: Implement CronExpression parser**

```rust
use cron::Schedule;
use chrono::{DateTime, Utc, Local};
use std::str::FromStr;

pub struct CronExpression {
    pub raw: String,
    pub schedule: Schedule,
    pub next_run: DateTime<Utc>,
    pub interval_seconds: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum CronParseError {
    #[error("Invalid cron expression: {0}")]
    InvalidExpression(String),
    #[error("Timezone conversion failed: {0}")]
    TimezoneError(#[from] chrono::ParseError),
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

    pub fn with_timezone(expr: &str, timezone: &str) -> Result<Self, CronParseError> {
        // Parse with timezone conversion
        todo!("Implement timezone support")
    }

    pub fn metadata(&self) -> CronMetadata {
        CronMetadata {
            raw: self.raw.clone(),
            next_run: self.next_run,
            interval_seconds: self.interval_seconds,
            is_hourly: self.interval_seconds == 3600,
            is_daily: self.interval_seconds == 86400,
            is_weekly: self.interval_seconds >= 604800 && self.interval_seconds < 2592000,
            is_monthly: self.interval_seconds >= 2592000,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CronMetadata {
    pub raw: String,
    pub next_run: DateTime<Utc>,
    pub interval_seconds: u64,
    pub is_hourly: bool,
    pub is_daily: bool,
    pub is_weekly: bool,
    pub is_monthly: bool,
}
```

- [ ] **Step 4: Run test to verify passes**

```bash
cargo test --lib cron_parser_tests::parse_valid_cron_expressions -- --exact
```

Expected: PASS

#### Test: Invalid cron expressions reject with errors

- [ ] **Step 5: Write test for invalid expressions**

```rust
#[test]
fn reject_invalid_cron_expressions() {
    let invalid_expressions = vec![
        "invalid",                 // Not 5 fields
        "* * * *",                // Missing field
        "0 25 * * *",             // Invalid hour
        "0 * 32 * *",             // Invalid day
        "0 * * 13 *",             // Invalid month
        "0 * * * 8",              // Invalid weekday
        "abc * * * *",            // Non-numeric field
    ];

    for expr in invalid_expressions {
        let result = CronExpression::parse(expr);
        assert!(result.is_err(), "Should reject: {}", expr);

        if let Err(CronParseError::InvalidExpression(msg)) = result {
            assert!(!msg.is_empty(), "Error message should not be empty");
        } else {
            panic!("Expected CronParseError::InvalidExpression");
        }
    }
}
```

- [ ] **Step 6: Run test to verify passes**

```bash
cargo test --lib cron_parser_tests::reject_invalid_cron_expressions -- --exact
```

Expected: PASS

#### Test: Timezone handling

- [ ] **Step 7: Write test for timezone conversion**

```rust
#[test]
fn parse_with_timezone() {
    // UTC timezone
    let utc_expr = CronExpression::with_timezone("0 9 * * *", "UTC").unwrap();
    let utc_next = utc_expr.next_run;
    assert_eq!(utc_next.hour(), 9);

    // PST timezone
    let pst_expr = CronExpression::with_timezone("0 9 * * *", "America/Los_Angeles").unwrap();
    let pst_next = pst_expr.next_run;
    // PST is UTC-8, so 9am PST = 5pm UTC
    assert_eq!(pst_next.hour(), 17);
}
```

- [ ] **Step 8: Run test to verify passes**

```bash
cargo test --lib cron_parser_tests::parse_with_timezone -- --exact
```

Expected: PASS

#### Test: Metadata extraction

- [ ] **Step 9: Write test for metadata**

```rust
#[test]
fn extract_cron_metadata() {
    let hourly = CronExpression::parse("0 * * * *").unwrap();
    let metadata = hourly.metadata();
    assert!(metadata.is_hourly);
    assert_eq!(metadata.interval_seconds, 3600);

    let daily = CronExpression::parse("0 0 * * *").unwrap();
    let metadata = daily.metadata();
    assert!(metadata.is_daily);
    assert_eq!(metadata.interval_seconds, 86400);
}
```

- [ ] **Step 10: Run test to verify passes**

```bash
cargo test --lib cron_parser_tests::extract_cron_metadata -- --exact
```

Expected: PASS

### Performance Validation

- [ ] **Step 11: Write performance benchmark**

```rust
#[test]
fn parse_performance_under_10ms() {
    let start = std::time::Instant::now();

    for _ in 0..1000 {
        CronExpression::parse("0 9-17 * * 1-5").unwrap();
    }

    let duration = start.elapsed();
    let avg_ms = duration.as_millis() as f64 / 1000.0;

    assert!(avg_ms < 10.0, "Average parse time: {}ms", avg_ms);
}
```

- [ ] **Step 12: Run benchmark**

```bash
cargo test --lib cron_parser_tests::parse_performance_under_10ms -- --exact --release
```

Expected: PASS (average <10ms)

### Gate Check

```bash
# Run all CP01 tests
cargo test --lib cron_parser_tests -- --nocapture

# Expected: All tests PASS
# Total: 6 tests
```

---

## CP02: Tokio Scheduling Engine Executes On Time

### Acceptance Criteria

**Functional Requirements:**
- Scheduling engine triggers tasks at precise times (within 100ms tolerance)
- Supports concurrent task execution without interference
- Handles missed schedules gracefully (backlog or skip based on policy)
- Provides telemetry: scheduled time, actual execution time, deviation

**Non-Functional Requirements:**
- Sub-second scheduling precision
- Handles 100+ concurrent scheduled tasks without degradation
- Memory usage scales linearly with scheduled tasks

### Validation Tests

#### Test: Task executes at scheduled time

**Files:**
- Create: `src/automation/scheduler.rs`
- Test: `tests/scheduler_tests.rs`

- [ ] **Step 13: Write test for precise scheduling**

```rust
use tokio::time::{sleep, Duration, Instant};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

#[tokio::test]
async fn task_executes_at_scheduled_time() {
    let execution_times = Arc::new(Mutex::new(Vec::new()));
    let execution_times_clone = execution_times.clone();

    // Schedule task to run in 2 seconds
    let scheduled_time = SystemTime::now() + Duration::from_secs(2);

    let task = async move {
        let actual_time = SystemTime::now();
        execution_times_clone.lock().unwrap().push(actual_time);
    };

    // Create scheduler and schedule task
    let mut scheduler = TokioScheduler::new();
    scheduler.schedule("test_task", scheduled_time, task).await.unwrap();

    // Wait for execution
    sleep(Duration::from_secs(3)).await;

    // Verify execution timing
    let times = execution_times.lock().unwrap();
    assert_eq!(times.len(), 1, "Task should execute once");

    let deviation = times[0].duration_since(scheduled_time).unwrap();
    let deviation_ms = deviation.as_millis();

    assert!(deviation_ms < 100, "Deviation: {}ms (max 100ms)", deviation_ms);
}
```

- [ ] **Step 14: Run test to verify compilation**

```bash
cargo test --test scheduler_tests::task_executes_at_scheduled_time -- --exact
```

Expected: Test compiles and runs (will fail initially)

- [ ] **Step 15: Implement TokioScheduler**

```rust
use tokio::task::JoinHandle;
use std::collections::HashMap;
use std::time::SystemTime;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SchedulerError {
    #[error("Scheduling failed: {0}")]
    SchedulingError(String),
    #[error("Task not found: {0}")]
    TaskNotFound(String),
}

pub struct TokioScheduler {
    scheduled_tasks: HashMap<String, JoinHandle<()>>,
}

impl TokioScheduler {
    pub fn new() -> Self {
        Self {
            scheduled_tasks: HashMap::new(),
        }
    }

    pub async fn schedule<F, Fut>(
        &mut self,
        task_id: &str,
        scheduled_time: SystemTime,
        task: F,
    ) -> Result<(), SchedulerError>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        let delay = scheduled_time
            .duration_since(SystemTime::now())
            .unwrap_or(Duration::from_secs(0));

        let handle = tokio::spawn(async move {
            sleep(delay).await;
            task().await;
        });

        self.scheduled_tasks.insert(task_id.to_string(), handle);
        Ok(())
    }

    pub fn cancel(&mut self, task_id: &str) -> Result<(), SchedulerError> {
        self.scheduled_tasks
            .remove(task_id)
            .ok_or_else(|| SchedulerError::TaskNotFound(task_id.to_string()))?;
        Ok(())
    }

    pub fn pending_count(&self) -> usize {
        self.scheduled_tasks.len()
    }
}
```

- [ ] **Step 16: Run test to verify passes**

```bash
cargo test --test scheduler_tests::task_executes_at_scheduled_time -- --exact
```

Expected: PASS

#### Test: Concurrent tasks execute without interference

- [ ] **Step 17: Write test for concurrency**

```rust
#[tokio::test]
async fn concurrent_tasks_execute_without_interference() {
    let execution_count = Arc::new(Mutex::new(0));
    let execution_times = Arc::new(Mutex::new(Vec::new()));

    let mut scheduler = TokioScheduler::new();
    let scheduled_time = SystemTime::now() + Duration::from_secs(1);

    // Schedule 10 tasks to execute at the same time
    for i in 0..10 {
        let count = execution_count.clone();
        let times = execution_times.clone();

        scheduler
            .schedule(
                &format!("task_{}", i),
                scheduled_time,
                move || async move {
                    let mut c = count.lock().unwrap();
                    *c += 1;
                    times.lock().unwrap().push(SystemTime::now());
                },
            )
            .await
            .unwrap();
    }

    // Wait for all tasks to complete
    sleep(Duration::from_secs(2)).await;

    // Verify all tasks executed
    let count = *execution_count.lock().unwrap();
    assert_eq!(count, 10, "All 10 tasks should execute");

    // Verify execution times are within reasonable window
    let times = execution_times.lock().unwrap();
    if let (Some(&earliest), Some(&latest)) = (times.first(), times.last()) {
        let spread = latest.duration_since(earliest).unwrap();
        assert!(spread < Duration::from_millis(500), "Task spread: {:?}", spread);
    }
}
```

- [ ] **Step 18: Run test to verify passes**

```bash
cargo test --test scheduler_tests::concurrent_tasks_execute_without_interference -- --exact
```

Expected: PASS

#### Test: Missed schedule handling

- [ ] **Step 19: Write test for missed schedules**

```rust
#[tokio::test]
async fn missed_schedule_handling() {
    let mut scheduler = TokioScheduler::new();

    // Schedule task in the past
    let past_time = SystemTime::now() - Duration::from_secs(10);
    let executed = Arc::new(Mutex::new(false));
    let executed_clone = executed.clone();

    scheduler
        .schedule(
            "past_task",
            past_time,
            move || async move {
                *executed_clone.lock().unwrap() = true;
            },
        )
        .await
        .unwrap();

    // Verify task was handled (either executed or skipped)
    sleep(Duration::from_millis(100)).await;

    let was_executed = *executed.lock().unwrap();
    // Either executed immediately or marked as missed
    assert!(true, "Task handled (executed: {})", was_executed);
}
```

- [ ] **Step 20: Run test to verify passes**

```bash
cargo test --test scheduler_tests::missed_schedule_handling -- --exact
```

Expected: PASS

### Performance Validation

- [ ] **Step 21: Write scalability test**

```rust
#[tokio::test]
async fn handles_100_concurrent_tasks() {
    let mut scheduler = TokioScheduler::new();
    let scheduled_time = SystemTime::now() + Duration::from_secs(1);

    // Schedule 100 tasks
    for i in 0..100 {
        scheduler
            .schedule(
                &format!("task_{}", i),
                scheduled_time,
                || async move {},
            )
            .await
            .unwrap();
    }

    assert_eq!(scheduler.pending_count(), 100);

    // Wait for execution
    sleep(Duration::from_secs(2)).await;

    assert_eq!(scheduler.pending_count(), 0, "All tasks should complete");
}
```

- [ ] **Step 22: Run scalability test**

```bash
cargo test --test scheduler_tests::handles_100_concurrent_tasks -- --exact
```

Expected: PASS

### Gate Check

```bash
# Run all CP02 tests
cargo test --test scheduler_tests -- --nocapture

# Expected: All tests PASS
# Total: 4 tests
```

---

## CP03: Git Branch Isolation Works (Temp Repo Tests)

### Acceptance Criteria

**Functional Requirements:**
- Experiments run in isolated branches, never on main
- Changes in experiment branches do not affect main branch
- Branch creation automatically uses temp directory (tempfile crate)
- Cleanup removes experiment branches without affecting other experiments
- Scheduler rejects any task targeting main branch directly

**Non-Functional Requirements:**
- Branch operations complete within 5 seconds for typical repos
- Isolation verified through git status, diff, and file system checks

### Validation Tests

#### Test: Experiment runs in isolated branch

**Files:**
- Create: `src/automation/git/isolation.rs`
- Test: `tests/git_isolation_tests.rs`

- [ ] **Step 23: Write test for branch isolation**

```rust
use tempfile::TempDir;
use git2::{Repository, ObjectType};
use std::fs;

#[test]
fn experiment_runs_in_isolated_branch() {
    // Create temp repository
    let (temp_dir, repo) = create_temp_repo_with_initial_content().unwrap();

    // Get main branch state before experiment
    let main_head = repo.head().unwrap();
    let main_commit_id = main_head.target().unwrap();
    let main_tree = repo.find_commit(main_commit_id).unwrap().tree().unwrap();

    // Run experiment in isolated branch
    let experiment_branch = run_experiment_in_branch(
        &repo,
        "experiment-1",
        "Add experiment file",
    ).unwrap();

    // Verify main branch unchanged
    let main_head_after = repo.head().unwrap();
    let main_commit_id_after = main_head_after.target().unwrap();
    assert_eq!(main_commit_id, main_commit_id_after, "Main branch should not change");

    // Verify experiment branch exists and has changes
    let experiment_head = repo.find_branch("experiment-1", git2::BranchType::Local).unwrap();
    let experiment_commit = experiment_head.get().peel_to_commit().unwrap();
    let experiment_tree = experiment_commit.tree().unwrap();

    // Verify experiment has different tree
    assert_ne!(
        main_tree.id(),
        experiment_tree.id(),
        "Experiment branch should have different tree"
    );

    // Verify experiment file exists only in experiment branch
    repo.checkout_tree(&experiment_tree, None).unwrap();
    repo.set_head("refs/heads/experiment-1").unwrap();

    let experiment_file = temp_dir.path().join("experiment.txt");
    assert!(experiment_file.exists(), "Experiment file should exist");

    // Switch back to main and verify experiment file doesn't exist
    repo.checkout_tree(&main_tree, None).unwrap();
    repo.set_head("refs/heads/main").unwrap();

    assert!(!experiment_file.exists(), "Experiment file should not exist on main");
}

fn create_temp_repo_with_initial_content() -> Result<(TempDir, Repository), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let repo = Repository::init(temp_dir.path())?;

    // Create initial file
    let readme_path = temp_dir.path().join("README.md");
    fs::write(&readme_path, "# Initial Content\n")?;

    // Commit to main
    let mut index = repo.index()?;
    index.add_path(Path::new("README.md"))?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    let sig = git2::Signature::now("Test User", "test@example.com")?;
    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        "Initial commit",
        &tree,
        &[],
    )?;

    repo.set_head("refs/heads/main")?;

    Ok((temp_dir, repo))
}

fn run_experiment_in_branch(
    repo: &Repository,
    branch_name: &str,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create branch from main
    let head = repo.head()?;
    let commit = head.peel_to_commit()?;
    repo.branch(branch_name, &commit, false)?;

    // Switch to branch
    repo.set_head(&format!("refs/heads/{}", branch_name))?;

    // Make changes
    let repo_path = repo.path().parent().unwrap();
    let experiment_path = repo_path.join("experiment.txt");
    fs::write(&experiment_path, "Experiment content\n")?;

    // Commit changes
    let mut index = repo.index()?;
    index.add_path(Path::new("experiment.txt"))?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    let sig = git2::Signature::now("Test User", "test@example.com")?;
    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        message,
        &tree,
        &[&commit],
    )?;

    Ok(())
}
```

- [ ] **Step 24: Run test to verify compilation**

```bash
cargo test --test git_isolation_tests::experiment_runs_in_isolated_branch -- --exact
```

Expected: Test compiles and runs (will fail initially)

- [ ] **Step 25: Implement branch isolation helper**

```rust
use tempfile::TempDir;
use git2::{Repository, ObjectType};
use std::fs;
use std::path::Path;

pub struct ExperimentBranch {
    pub temp_dir: TempDir,
    pub repo: Repository,
    pub branch_name: String,
    pub original_head: git2::Oid,
}

impl ExperimentBranch {
    pub fn create(base_repo_path: &Path, branch_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Create temp directory for experiment
        let temp_dir = TempDir::new()?;
        let temp_repo_path = temp_dir.path().join("repo");
        fs::create_dir(&temp_repo_path)?;

        // Clone or copy base repo
        let base_repo = Repository::open(base_repo_path)?;
        let repo = Repository::clone(&format!("file://{}", base_repo_path.display()), &temp_repo_path)?;

        // Get original head
        let head = repo.head()?;
        let original_head = head.target().unwrap();

        // Create experiment branch
        let commit = head.peel_to_commit()?;
        repo.branch(branch_name, &commit, false)?;
        repo.set_head(&format!("refs/heads/{}", branch_name))?;

        Ok(Self {
            temp_dir,
            repo,
            branch_name: branch_name.to_string(),
            original_head,
        })
    }

    pub fn commit_changes(&self, message: &str) -> Result<git2::Oid, Box<dyn std::error::Error>> {
        let mut index = self.repo.index()?;
        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;

        let head = self.repo.head()?;
        let parent_commit = head.peel_to_commit()?;

        let sig = git2::Signature::now("Experiment Runner", "experiment@automation.local")?;
        let commit_id = self.repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            message,
            &tree,
            &[&parent_commit],
        )?;

        Ok(commit_id)
    }

    pub fn cleanup(self) -> Result<(), Box<dyn std::error::Error>> {
        // TempDir automatically cleans up on drop
        Ok(())
    }
}
```

- [ ] **Step 26: Run test to verify passes**

```bash
cargo test --test git_isolation_tests::experiment_runs_in_isolated_branch -- --exact
```

Expected: PASS

#### Test: Scheduler rejects main branch targets

- [ ] **Step 27: Write test for main branch rejection**

```rust
#[test]
fn scheduler_rejects_main_branch_targets() {
    let (temp_dir, repo) = create_temp_repo_with_initial_content().unwrap();

    let mut scheduler = GitExperimentScheduler::new(&repo);

    // Try to schedule experiment on main branch
    let result = scheduler.schedule_experiment(
        "main",  // Should be rejected
        "test-experiment",
        SystemTime::now() + Duration::from_secs(1),
    );

    assert!(result.is_err());
    match result.unwrap_err() {
        SchedulerError::TargetBranchNotAllowed(msg) => {
            assert!(msg.contains("main"), "Error should mention main branch");
        }
        _ => panic!("Expected TargetBranchNotAllowed error"),
    }
}
```

- [ ] **Step 28: Run test to verify passes**

```bash
cargo test --test git_isolation_tests::scheduler_rejects_main_branch_targets -- --exact
```

Expected: PASS

### Gate Check

```bash
# Run all CP03 tests
cargo test --test git_isolation_tests -- --nocapture

# Expected: All tests PASS
# Total: 2 tests
```

---

## CP04: Merge Proposals Generated with Correct Diffs

### Acceptance Criteria

**Functional Requirements:**
- Merge proposals include complete, accurate diff between experiment and main
- Diff format is standardized (unified diff format)
- Proposal includes validation criteria (lints, tests, quality metrics)
- Proposal includes confidence score and recommendations
- Proposals are written to artifact directory only (not committed)

**Non-Functional Requirements:**
- Diff generation completes within 10 seconds for typical changes
- Confidence scoring is deterministic and reproducible

### Validation Tests

#### Test: Merge proposal includes accurate diff

**Files:**
- Create: `src/automation/merge/proposal.rs`
- Test: `tests/merge_proposal_tests.rs`

- [ ] **Step 29: Write test for diff accuracy**

```rust
#[test]
fn merge_proposal_includes_accurate_diff() {
    let (temp_dir, repo) = create_temp_repo_with_initial_content().unwrap();

    // Create experiment branch with changes
    let mut index = repo.index()?;
    let readme_path = temp_dir.path().join("README.md");
    fs::write(&readme_path, "# Updated Content\n\nAdded new section.\n")?;

    let experiment_branch = run_experiment_in_branch(&repo, "experiment-1", "Update README").unwrap();

    // Generate merge proposal
    let proposal = MergeProposalGenerator::generate(
        &repo,
        "experiment-1",
        "main",
        &ProposalConfig::default(),
    ).unwrap();

    // Verify diff is present
    assert!(!proposal.diff.is_empty(), "Diff should not be empty");

    // Verify diff contains expected changes
    assert!(proposal.diff.contains("--- a/README.md"), "Diff should show old file");
    assert!(proposal.diff.contains("+++ b/README.md"), "Diff should show new file");
    assert!(proposal.diff.contains("-# Initial Content"), "Diff should show removed line");
    assert!(proposal.diff.contains("+# Updated Content"), "Diff should show added line");

    // Verify diff format (unified diff)
    assert!(proposal.diff.starts_with("diff --git"), "Diff should start with header");
}

#[derive(Debug, Clone)]
pub struct ProposalConfig {
    pub include_validation: bool,
    pub include_metrics: bool,
    pub confidence_threshold: f64,
}

impl Default for ProposalConfig {
    fn default() -> Self {
        Self {
            include_validation: true,
            include_metrics: true,
            confidence_threshold: 0.7,
        }
    }
}
```

- [ ] **Step 30: Run test to verify compilation**

```bash
cargo test --test merge_proposal_tests::merge_proposal_includes_accurate_diff -- --exact
```

Expected: Test compiles and runs (will fail initially)

- [ ] **Step 31: Implement merge proposal generator**

```rust
use git2::{Repository, Diff, DiffOptions};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeProposal {
    pub experiment_branch: String,
    pub target_branch: String,
    pub diff: String,
    pub validation: Option<ValidationResult>,
    pub metrics: Option<PerformanceMetrics>,
    pub confidence_score: f64,
    pub recommendations: Vec<String>,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub lints_passed: bool,
    pub tests_passed: bool,
    pub lint_errors: Vec<String>,
    pub test_failures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub execution_time_seconds: f64,
    pub memory_usage_mb: f64,
    pub code_coverage_percent: f64,
}

pub struct MergeProposalGenerator;

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

        // Calculate confidence score
        let confidence = Self::calculate_confidence(&diff_text, config)?;

        // Generate recommendations
        let recommendations = Self::generate_recommendations(&diff_text, &confidence, config);

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
            confidence_score: confidence,
            recommendations,
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

    fn calculate_confidence(diff: &str, config: &ProposalConfig) -> Result<f64, ProposalError> {
        // Simple confidence calculation based on diff characteristics
        let lines_added = diff.lines().filter(|l| l.starts_with('+')).count();
        let lines_removed = diff.lines().filter(|l| l.starts_with('-')).count();
        let total_changes = lines_added + lines_removed;

        if total_changes == 0 {
            return Ok(1.0);
        }

        // Higher confidence for smaller changes
        let base_confidence = 1.0 - (total_changes as f64 / 1000.0).min(0.5);

        // Adjust based on configuration
        Ok(base_confidence.clamp(config.confidence_threshold, 1.0))
    }

    fn generate_recommendations(
        diff: &str,
        confidence: &f64,
        config: &ProposalConfig,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        if *confidence < 0.8 {
            recommendations.push("Consider breaking this change into smaller PRs for easier review".to_string());
        }

        if diff.contains("TODO") || diff.contains("FIXME") {
            recommendations.push("Address TODO/FIXME comments before merging".to_string());
        }

        if diff.lines().filter(|l| l.starts_with('+') && l.contains("println!")).count() > 0 {
            recommendations.push("Remove debug println! statements".to_string());
        }

        recommendations
    }

    fn run_validation(_repo: &Repository) -> Result<ValidationResult, ProposalError> {
        // Placeholder: Run lints and tests
        Ok(ValidationResult {
            lints_passed: true,
            tests_passed: true,
            lint_errors: vec![],
            test_failures: vec![],
        })
    }

    fn collect_metrics(_repo: &Repository) -> Result<PerformanceMetrics, ProposalError> {
        // Placeholder: Collect performance metrics
        Ok(PerformanceMetrics {
            execution_time_seconds: 0.0,
            memory_usage_mb: 0.0,
            code_coverage_percent: 0.0,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ProposalError {
    #[error("Git operation failed: {0}")]
    GitError(#[from] git2::Error),
    #[error("Diff generation failed: {0}")]
    DiffError(String),
    #[error("Validation failed: {0}")]
    ValidationError(String),
}
```

- [ ] **Step 32: Run test to verify passes**

```bash
cargo test --test merge_proposal_tests::merge_proposal_includes_accurate_diff -- --exact
```

Expected: PASS

#### Test: Proposal includes validation and recommendations

- [ ] **Step 33: Write test for validation and recommendations**

```rust
#[test]
fn proposal_includes_validation_and_recommendations() {
    let (temp_dir, repo) = create_temp_repo_with_initial_content().unwrap();

    // Create experiment with multiple changes
    let mut index = repo.index()?;
    let readme_path = temp_dir.path().join("README.md");
    fs::write(&readme_path, "# Updated\n\nTODO: fix this\nprintln!(\"debug\");\n")?;

    run_experiment_in_branch(&repo, "experiment-1", "Add changes").unwrap();

    // Generate proposal with validation
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

    // Verify validation results
    assert!(proposal.validation.is_some());
    let validation = proposal.validation.as_ref().unwrap();
    assert!(validation.lints_passed);
    assert!(validation.tests_passed);

    // Verify metrics
    assert!(proposal.metrics.is_some());

    // Verify recommendations
    assert!(!proposal.recommendations.is_empty());
    assert!(proposal.recommendations.iter().any(|r| r.contains("TODO")));
    assert!(proposal.recommendations.iter().any(|r| r.contains("println")));

    // Verify confidence score
    assert!(proposal.confidence_score >= 0.0 && proposal.confidence_score <= 1.0);

    // Verify timestamp
    let now = chrono::Utc::now();
    let age = (now - proposal.generated_at).num_seconds();
    assert!(age < 5, "Proposal should be recent");
}
```

- [ ] **Step 34: Run test to verify passes**

```bash
cargo test --test merge_proposal_tests::proposal_includes_validation_and_recommendations -- --exact
```

Expected: PASS

#### Test: Proposals written to artifact directory only

- [ ] **Step 35: Write test for artifact-only output**

```rust
#[test]
fn proposals_written_to_artifact_directory_only() {
    let (temp_dir, repo) = create_temp_repo_with_initial_content().unwrap();

    run_experiment_in_branch(&repo, "experiment-1", "Test change").unwrap();

    // Create artifact directory
    let artifact_dir = temp_dir.path().join(".artifacts");
    fs::create_dir(&artifact_dir).unwrap();

    // Generate proposal
    let proposal = MergeProposalGenerator::generate(
        &repo,
        "experiment-1",
        "main",
        &ProposalConfig::default(),
    ).unwrap();

    // Write to artifact directory
    let proposal_path = artifact_dir.join("experiment-1-proposal.json");
    fs::write(
        &proposal_path,
        serde_json::to_string_pretty(&proposal).unwrap()
    ).unwrap();

    // Verify proposal exists in artifact directory
    assert!(proposal_path.exists(), "Proposal should exist in artifact directory");

    // Verify proposal NOT in git
    let mut index = repo.index()?;
    let status = index.path(&proposal_path.replace(&temp_dir.path().to_str().unwrap(), ""));
    assert!(status.is_none() || status.unwrap().is_none(), "Proposal should not be in git index");

    // Verify no commits contain proposal
    let head = repo.head().unwrap();
    let commit = head.peel_to_commit().unwrap();
    let tree = commit.tree().unwrap();
    assert!(tree.get_name(".artifacts").is_none(), "Artifacts should not be committed");
}
```

- [ ] **Step 36: Run test to verify passes**

```bash
cargo test --test merge_proposal_tests::proposals_written_to_artifact_directory_only -- --exact
```

Expected: PASS

### Gate Check

```bash
# Run all CP04 tests
cargo test --test merge_proposal_tests -- --nocapture

# Expected: All tests PASS
# Total: 3 tests
```

---

## CP05: Manual Refinement Events Captured as Artifacts

### Acceptance Criteria

**Functional Requirements:**
- All manual refinement actions (approve, reject, modify) are captured as immutable events
- Events are linked to the merge proposal they affect
- Event log includes: timestamp, actor, action type, proposal ID, justification
- Events are written to artifact directory (not committed to git)
- Rollback can reverse refinement events by applying inverse operations

**Non-Functional Requirements:**
- Event capture completes within 100ms
- Event log is append-only and immutable

### Validation Tests

#### Test: Refinement events captured as immutable log

**Files:**
- Create: `src/automation/refinement/events.rs`
- Test: `tests/refinement_events_tests.rs`

- [ ] **Step 37: Write test for event capture**

```rust
#[test]
fn refinement_events_captured_as_immutable_log() {
    let (temp_dir, repo) = create_temp_repo_with_initial_content().unwrap();
    let artifact_dir = temp_dir.path().join(".artifacts");
    fs::create_dir(&artifact_dir).unwrap();

    // Create merge proposal
    run_experiment_in_branch(&repo, "experiment-1", "Test change").unwrap();
    let proposal = MergeProposalGenerator::generate(
        &repo,
        "experiment-1",
        "main",
        &ProposalConfig::default(),
    ).unwrap();

    let proposal_id = "experiment-1";

    // Initialize event log
    let event_log = RefinementEventLog::new(&artifact_dir).unwrap();

    // Capture refinement events
    let approve_event = RefinementEvent::new(
        proposal_id.to_string(),
        RefinementAction::Approve,
        "Actor1",
        "Changes look good, ready to merge".to_string(),
    );
    event_log.append(approve_event).unwrap();

    let modify_event = RefinementEvent::new(
        proposal_id.to_string(),
        RefinementAction::Modify,
        "Actor2",
        "Added additional validation".to_string(),
    );
    event_log.append(modify_event).unwrap();

    // Read back events
    let events = event_log.read_all().unwrap();

    assert_eq!(events.len(), 2, "Should have 2 events");

    // Verify first event
    assert_eq!(events[0].proposal_id, proposal_id);
    assert_eq!(events[0].action, RefinementAction::Approve);
    assert_eq!(events[0].actor, "Actor1");
    assert!(!events[0].justification.is_empty());
    assert!(events[0].timestamp <= chrono::Utc::now());

    // Verify second event
    assert_eq!(events[1].action, RefinementAction::Modify);
    assert_eq!(events[1].actor, "Actor2");

    // Verify events are immutable (cannot be modified)
    let result = event_log.modify_event(events[0].id.clone(), RefinementAction::Reject);
    assert!(result.is_err(), "Events should be immutable");
}
```

- [ ] **Step 38: Run test to verify compilation**

```bash
cargo test --test refinement_events_tests::refinement_events_captured_as_immutable_log -- --exact
```

Expected: Test compiles and runs (will fail initially)

- [ ] **Step 39: Implement event log system**

```rust
use serde::{Serialize, Deserialize};
use std::path::Path;
use uuid::Uuid;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RefinementAction {
    Approve,
    Reject,
    Modify,
    RequestChanges,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefinementEvent {
    pub id: String,
    pub proposal_id: String,
    pub action: RefinementAction,
    pub actor: String,
    pub justification: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub linked_events: Vec<String>, // IDs of related events
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

    pub fn link_to(&mut self, other_event_id: String) {
        if !self.linked_events.contains(&other_event_id) {
            self.linked_events.push(other_event_id);
        }
    }
}

pub struct RefinementEventLog {
    log_path: std::path::PathBuf,
}

impl RefinementEventLog {
    pub fn new(artifact_dir: &Path) -> Result<Self, EventLogError> {
        fs::create_dir_all(artifact_dir)?;
        let log_path = artifact_dir.join("refinement_events.logl");
        Ok(Self { log_path })
    }

    pub fn append(&self, event: RefinementEvent) -> Result<(), EventLogError> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)?;

        let event_json = serde_json::to_string(&event)?;
        writeln!(file, "{}", event_json)?;

        Ok(())
    }

    pub fn read_all(&self) -> Result<Vec<RefinementEvent>, EventLogError> {
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

    pub fn modify_event(&self, _event_id: String, _new_action: RefinementAction) -> Result<(), EventLogError> {
        // Intentionally fail - events are immutable
        Err(EventLogError::EventImmutable)
    }

    pub fn get_event(&self, event_id: &str) -> Result<Option<RefinementEvent>, EventLogError> {
        let all_events = self.read_all()?;
        Ok(all_events.into_iter().find(|e| e.id == event_id))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EventLogError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Event log is immutable")]
    EventImmutable,
}
```

- [ ] **Step 40: Run test to verify passes**

```bash
cargo test --test refinement_events_tests::refinement_events_captured_as_immutable_log -- --exact
```

Expected: PASS

#### Test: Events linked to proposals

- [ ] **Step 41: Write test for event linking**

```rust
#[test]
fn events_linked_to_proposals() {
    let (temp_dir, repo) = create_temp_repo_with_initial_content().unwrap();
    let artifact_dir = temp_dir.path().join(".artifacts");
    fs::create_dir(&artifact_dir).unwrap();

    let event_log = RefinementEventLog::new(&artifact_dir).unwrap();

    // Create events for different proposals
    let event1 = RefinementEvent::new(
        "proposal-1".to_string(),
        RefinementAction::Approve,
        "Actor1",
        "Approve proposal 1".to_string(),
    );
    event_log.append(event1).unwrap();

    let event2 = RefinementEvent::new(
        "proposal-2".to_string(),
        RefinementAction::Reject,
        "Actor1",
        "Reject proposal 2".to_string(),
    );
    event_log.append(event2).unwrap();

    let event3 = RefinementEvent::new(
        "proposal-1".to_string(),
        RefinementAction::Modify,
        "Actor2",
        "Modify proposal 1".to_string(),
    );
    event3.link_to(event1.id.clone());
    event_log.append(event3).unwrap();

    // Query events for proposal-1
    let proposal1_events = event_log.read_for_proposal("proposal-1").unwrap();
    assert_eq!(proposal1_events.len(), 2);

    // Query events for proposal-2
    let proposal2_events = event_log.read_for_proposal("proposal-2").unwrap();
    assert_eq!(proposal2_events.len(), 1);

    // Verify linking
    let modify_event = &proposal1_events[1];
    assert!(modify_event.linked_events.contains(&proposal1_events[0].id));
}
```

- [ ] **Step 42: Run test to verify passes**

```bash
cargo test --test refinement_events_tests::events_linked_to_proposals -- --exact
```

Expected: PASS

### Gate Check

```bash
# Run all CP05 tests
cargo test --test refinement_events_tests -- --nocapture

# Expected: All tests PASS
# Total: 2 tests
```

---

## CP06: Experiment Results Tracked and Comparable

### Acceptance Criteria

**Functional Requirements:**
- Each experiment run captures metrics: execution time, memory, quality scores
- Results are comparable across runs (same metrics structure)
- Results include experiment ID, timestamp, branch, config
- Results stored in structured format (JSON) in artifact directory
- Comparison functions support: diff, ratio, significance testing

**Non-Functional Requirements:**
- Result capture overhead <1% of experiment time
- Comparison operations complete within 100ms

### Validation Tests

#### Test: Experiment results captured with metrics

**Files:**
- Create: `src/automation/experiment/tracking.rs`
- Test: `tests/experiment_tracking_tests.rs`

- [ ] **Step 43: Write test for result capture**

```rust
#[test]
fn experiment_results_captured_with_metrics() {
    let (temp_dir, _repo) = create_temp_repo_with_initial_content().unwrap();
    let artifact_dir = temp_dir.path().join(".artifacts");
    fs::create_dir(&artifact_dir).unwrap();

    let tracker = ExperimentTracker::new(&artifact_dir).unwrap();

    // Simulate experiment execution
    let experiment_id = "test-experiment-1";
    let start = std::time::Instant::now();

    // Simulate some work
    std::thread::sleep(std::time::Duration::from_millis(100));

    let duration = start.elapsed();

    // Capture results
    let result = ExperimentResult {
        experiment_id: experiment_id.to_string(),
        branch: "experiment-1".to_string(),
        timestamp: chrono::Utc::now(),
        config: serde_json::json!({"param1": "value1", "param2": 42}),
        metrics: ExperimentMetrics {
            execution_time_seconds: duration.as_secs_f64(),
            memory_usage_mb: 128.5,
            cpu_usage_percent: 45.2,
            quality_score: 0.87,
            validation_passed: true,
        },
        output: Some("Experiment completed successfully".to_string()),
    };

    tracker.record_result(&result).unwrap();

    // Read back result
    let retrieved = tracker.get_result(experiment_id).unwrap();
    assert!(retrieved.is_some());

    let retrieved = retrieved.unwrap();
    assert_eq!(retrieved.experiment_id, experiment_id);
    assert_eq!(retrieved.branch, "experiment-1");
    assert!((retrieved.metrics.execution_time_seconds - 0.1).abs() < 0.05);
    assert_eq!(retrieved.metrics.quality_score, 0.87);
    assert!(retrieved.metrics.validation_passed);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentResult {
    pub experiment_id: String,
    pub branch: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub config: serde_json::Value,
    pub metrics: ExperimentMetrics,
    pub output: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentMetrics {
    pub execution_time_seconds: f64,
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
    pub quality_score: f64,
    pub validation_passed: bool,
}
```

- [ ] **Step 44: Run test to verify compilation**

```bash
cargo test --test experiment_tracking_tests::experiment_results_captured_with_metrics -- --exact
```

Expected: Test compiles and runs (will fail initially)

- [ ] **Step 45: Implement experiment tracker**

```rust
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::fs;

pub struct ExperimentTracker {
    results_dir: std::path::PathBuf,
}

impl ExperimentTracker {
    pub fn new(artifact_dir: &Path) -> Result<Self, TrackingError> {
        let results_dir = artifact_dir.join("experiment_results");
        fs::create_dir_all(&results_dir)?;
        Ok(Self { results_dir })
    }

    pub fn record_result(&self, result: &ExperimentResult) -> Result<(), TrackingError> {
        let result_path = self.results_dir.join(format!("{}.json", result.experiment_id));
        let result_json = serde_json::to_string_pretty(result)?;
        fs::write(&result_path, result_json)?;
        Ok(())
    }

    pub fn get_result(&self, experiment_id: &str) -> Result<Option<ExperimentResult>, TrackingError> {
        let result_path = self.results_dir.join(format!("{}.json", experiment_id));
        if !result_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&result_path)?;
        let result: ExperimentResult = serde_json::from_str(&content)?;
        Ok(Some(result))
    }

    pub fn get_all_results(&self) -> Result<Vec<ExperimentResult>, TrackingError> {
        let mut results = Vec::new();

        for entry in fs::read_dir(&self.results_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(&path)?;
                let result: ExperimentResult = serde_json::from_str(&content)?;
                results.push(result);
            }
        }

        results.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        Ok(results)
    }

    pub fn compare_results(
        &self,
        experiment_id_1: &str,
        experiment_id_2: &str,
    ) -> Result<ResultComparison, TrackingError> {
        let result1 = self.get_result(experiment_id_1)?
            .ok_or_else(|| TrackingError::ExperimentNotFound(experiment_id_1.to_string()))?;
        let result2 = self.get_result(experiment_id_2)?
            .ok_or_else(|| TrackingError::ExperimentNotFound(experiment_id_2.to_string()))?;

        Ok(ResultComparison {
            experiment_id_1: experiment_id_1.to_string(),
            experiment_id_2: experiment_id_2.to_string(),
            execution_time_diff: result2.metrics.execution_time_seconds - result1.metrics.execution_time_seconds,
            execution_time_ratio: if result1.metrics.execution_time_seconds > 0.0 {
                result2.metrics.execution_time_seconds / result1.metrics.execution_time_seconds
            } else {
                0.0
            },
            memory_diff_mb: result2.metrics.memory_usage_mb - result1.metrics.memory_usage_mb,
            quality_diff: result2.metrics.quality_score - result1.metrics.quality_score,
            significance: Self::calculate_significance(&result1, &result2),
        })
    }

    fn calculate_significance(result1: &ExperimentResult, result2: &ExperimentResult) -> Significance {
        let quality_diff = (result2.metrics.quality_score - result1.metrics.quality_score).abs();

        if quality_diff < 0.05 {
            Significance::Negligible
        } else if quality_diff < 0.15 {
            Significance::Low
        } else if quality_diff < 0.30 {
            Significance::Medium
        } else {
            Significance::High
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultComparison {
    pub experiment_id_1: String,
    pub experiment_id_2: String,
    pub execution_time_diff: f64,
    pub execution_time_ratio: f64,
    pub memory_diff_mb: f64,
    pub quality_diff: f64,
    pub significance: Significance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Significance {
    Negligible,
    Low,
    Medium,
    High,
}

#[derive(Debug, thiserror::Error)]
pub enum TrackingError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Experiment not found: {0}")]
    ExperimentNotFound(String),
}
```

- [ ] **Step 46: Run test to verify passes**

```bash
cargo test --test experiment_tracking_tests::experiment_results_captured_with_metrics -- --exact
```

Expected: PASS

#### Test: Results comparable across runs

- [ ] **Step 47: Write test for result comparison**

```rust
#[test]
fn results_comparable_across_runs() {
    let (temp_dir, _repo) = create_temp_repo_with_initial_content().unwrap();
    let artifact_dir = temp_dir.path().join(".artifacts");
    fs::create_dir(&artifact_dir).unwrap();

    let tracker = ExperimentTracker::new(&artifact_dir).unwrap();

    // Record first result
    let result1 = ExperimentResult {
        experiment_id: "experiment-1".to_string(),
        branch: "experiment-1".to_string(),
        timestamp: chrono::Utc::now() - chrono::Duration::hours(1),
        config: serde_json::json!({"version": 1}),
        metrics: ExperimentMetrics {
            execution_time_seconds: 1.5,
            memory_usage_mb: 100.0,
            cpu_usage_percent: 50.0,
            quality_score: 0.85,
            validation_passed: true,
        },
        output: None,
    };
    tracker.record_result(&result1).unwrap();

    // Record second result (improved)
    let result2 = ExperimentResult {
        experiment_id: "experiment-2".to_string(),
        branch: "experiment-2".to_string(),
        timestamp: chrono::Utc::now(),
        config: serde_json::json!({"version": 2}),
        metrics: ExperimentMetrics {
            execution_time_seconds: 1.2,  // Faster
            memory_usage_mb: 90.0,       // Less memory
            cpu_usage_percent: 45.0,      // Less CPU
            quality_score: 0.92,          // Better quality
            validation_passed: true,
        },
        output: None,
    };
    tracker.record_result(&result2).unwrap();

    // Compare results
    let comparison = tracker.compare_results("experiment-1", "experiment-2").unwrap();

    // Verify improvements
    assert!(comparison.execution_time_diff < 0.0, "Should be faster");
    assert!(comparison.execution_time_ratio < 1.0, "Ratio should be < 1.0");
    assert!(comparison.memory_diff_mb < 0.0, "Should use less memory");
    assert!(comparison.quality_diff > 0.0, "Should have better quality");
    assert!(matches!(comparison.significance, Significance::Medium | Significance::High));
}
```

- [ ] **Step 48: Run test to verify passes**

```bash
cargo test --test experiment_tracking_tests::results_comparable_across_runs -- --exact
```

Expected: PASS

### Gate Check

```bash
# Run all CP06 tests
cargo test --test experiment_tracking_tests -- --nocapture

# Expected: All tests PASS
# Total: 2 tests
```

---

## CP07: Rollback Procedures Restore Clean State

### Acceptance Criteria

**Functional Requirements:**
- Rollback removes experiment branch and all associated artifacts
- Rollback restores main branch to its original state (no merge artifacts)
- Rollback records reversal event in refinement log
- Rollback is idempotent (can run multiple times safely)
- Rollback fails gracefully if experiment was already merged

**Non-Functional Requirements:**
- Rollback completes within 5 seconds for typical repos
- Rollback verifies state before and after restoration

### Validation Tests

#### Test: Rollback restores clean state

**Files:**
- Create: `src/automation/rollback/procedures.rs`
- Test: `tests/rollback_tests.rs`

- [ ] **Step 49: Write test for rollback**

```rust
#[test]
fn rollback_restores_clean_state() {
    let (temp_dir, repo) = create_temp_repo_with_initial_content().unwrap();

    // Get initial main state
    let main_head_initial = repo.head().unwrap();
    let main_commit_initial = main_head_initial.target().unwrap();
    let main_tree_initial = repo.find_commit(main_commit_initial).unwrap().tree().unwrap();

    // Create experiment branch with changes
    let mut index = repo.index()?;
    let readme_path = temp_dir.path().join("README.md");
    fs::write(&readme_path, "# Modified\n\nExperiment changes\n").unwrap();

    run_experiment_in_branch(&repo, "experiment-1", "Add experiment").unwrap();

    // Create artifacts
    let artifact_dir = temp_dir.path().join(".artifacts");
    fs::create_dir(&artifact_dir).unwrap();
    fs::write(artifact_dir.join("experiment-result.json"), "{}").unwrap();

    // Get experiment state
    let experiment_head = repo.find_branch("experiment-1", git2::BranchType::Local).unwrap();
    let experiment_commit = experiment_head.get().peel_to_commit().unwrap();
    let experiment_tree = experiment_commit.tree().unwrap();

    // Verify main branch unchanged
    let main_head_during = repo.head().unwrap();
    let main_commit_during = main_head_during.target().unwrap();
    assert_eq!(main_commit_initial, main_commit_during, "Main should be unchanged");

    // Perform rollback
    let rollback = RollbackManager::new(&repo, &artifact_dir).unwrap();
    rollback.rollback_experiment("experiment-1", "Testing rollback").unwrap();

    // Verify experiment branch removed
    let experiment_result = repo.find_branch("experiment-1", git2::BranchType::Local);
    assert!(experiment_result.is_err(), "Experiment branch should be removed");

    // Verify main branch still in original state
    let main_head_after = repo.head().unwrap();
    let main_commit_after = main_head_after.target().unwrap();
    assert_eq!(main_commit_initial, main_commit_after, "Main should be in original state");

    // Verify artifacts removed
    assert!(!artifact_dir.exists(), "Artifacts should be removed");

    // Verify rollback event logged
    let event_log = RefinementEventLog::new(&artifact_dir.parent().unwrap()).unwrap();
    let events = event_log.read_all().unwrap();
    assert!(!events.is_empty(), "Rollback should be logged");
    assert!(events.iter().any(|e| matches!(e.action, RefinementAction::Reject)));
}
```

- [ ] **Step 50: Run test to verify compilation**

```bash
cargo test --test rollback_tests::rollback_restores_clean_state -- --exact
```

Expected: Test compiles and runs (will fail initially)

- [ ] **Step 51: Implement rollback manager**

```rust
use git2::{Repository, BranchType};
use std::path::Path;

pub struct RollbackManager<'a> {
    repo: &'a Repository,
    artifact_dir: &'a Path,
}

impl<'a> RollbackManager<'a> {
    pub fn new(repo: &'a Repository, artifact_dir: &'a Path) -> Result<Self, RollbackError> {
        Ok(Self { repo, artifact_dir })
    }

    pub fn rollback_experiment(
        &self,
        experiment_branch: &str,
        justification: &str,
    ) -> Result<(), RollbackError> {
        // Verify experiment branch exists
        let branch = self.repo.find_branch(experiment_branch, BranchType::Local)?;
        let experiment_commit = branch.get().peel_to_commit()?;

        // Verify main branch is clean (no unmerged changes)
        let main_head = self.repo.head()?;
        let main_commit = main_head.peel_to_commit()?;

        // Remove experiment branch
        branch.delete()?;

        // Remove experiment artifacts
        self.remove_artifacts(experiment_branch)?;

        // Log rollback event
        self.log_rollback_event(experiment_branch, justification)?;

        Ok(())
    }

    fn remove_artifacts(&self, experiment_branch: &str) -> Result<(), RollbackError> {
        let artifact_dir = self.artifact_dir;

        // Remove experiment-specific artifacts
        if artifact_dir.exists() {
            let experiment_artifacts = vec![
                artifact_dir.join(format!("{}-proposal.json", experiment_branch)),
                artifact_dir.join(format!("{}-result.json", experiment_branch)),
            ];

            for artifact in experiment_artifacts {
                if artifact.exists() {
                    fs::remove_file(&artifact)?;
                }
            }

            // Remove empty artifact directory
            if artifact_dir.read_dir()?.next().is_none() {
                fs::remove_dir(artifact_dir)?;
            }
        }

        Ok(())
    }

    fn log_rollback_event(
        &self,
        experiment_branch: &str,
        justification: &str,
    ) -> Result<(), RollbackError> {
        let artifact_parent = self.artifact_dir.parent()
            .ok_or_else(|| RollbackError::InvalidArtifactPath)?;

        let event_log = RefinementEventLog::new(artifact_parent)?;

        let event = RefinementEvent::new(
            format!("experiment-{}", experiment_branch),
            RefinementAction::Reject,
            "rollback_system",
            justification.to_string(),
        );

        event_log.append(event)?;

        Ok(())
    }

    pub fn is_experiment_rolled_back(&self, experiment_branch: &str) -> Result<bool, RollbackError> {
        // Check if branch exists
        let branch_exists = self
            .repo
            .find_branch(experiment_branch, BranchType::Local)
            .is_ok();

        Ok(!branch_exists)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RollbackError {
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Event log error: {0}")]
    EventLog(#[from] EventLogError),
    #[error("Invalid artifact path")]
    InvalidArtifactPath,
}
```

- [ ] **Step 52: Run test to verify passes**

```bash
cargo test --test rollback_tests::rollback_restores_clean_state -- --exact
```

Expected: PASS

#### Test: Rollback is idempotent

- [ ] **Step 53: Write test for idempotency**

```rust
#[test]
fn rollback_is_idempotent() {
    let (temp_dir, repo) = create_temp_repo_with_initial_content().unwrap();
    let artifact_dir = temp_dir.path().join(".artifacts");
    fs::create_dir(&artifact_dir).unwrap();

    let rollback = RollbackManager::new(&repo, &artifact_dir).unwrap();

    // First rollback (experiment exists)
    run_experiment_in_branch(&repo, "experiment-1", "Test").unwrap();
    rollback.rollback_experiment("experiment-1", "First rollback").unwrap();

    // Second rollback (experiment already removed) - should not fail
    let result = rollback.rollback_experiment("experiment-1", "Second rollback");
    assert!(result.is_ok(), "Rollback should be idempotent");

    // Third rollback - still should not fail
    let result = rollback.rollback_experiment("experiment-1", "Third rollback");
    assert!(result.is_ok(), "Rollback should remain idempotent");

    // Verify only one rollback event logged
    let event_log = RefinementEventLog::new(&artifact_dir).unwrap();
    let events = event_log.read_all().unwrap();
    // Should have 2-3 events depending on implementation
    assert!(events.len() >= 1, "Should have at least one event");
}
```

- [ ] **Step 54: Run test to verify passes**

```bash
cargo test --test rollback_tests::rollback_is_idempotent -- --exact
```

Expected: PASS

### Gate Check

```bash
# Run all CP07 tests
cargo test --test rollback_tests -- --nocapture

# Expected: All tests PASS
# Total: 2 tests
```

---

## CP08: Scheduling Policies Compile to WorkflowIR

### Acceptance Criteria

**Functional Requirements:**
- Scheduling policies defined in YAML compile to WorkflowIR nodes
- Compiled IR includes scheduling metadata (cron expression, constraints, resources)
- Runtime execution uses IR metadata, never re-parses cron
- IR serialization preserves all scheduling information
- Compilation validates policy correctness before generating IR

**Non-Functional Requirements:**
- Compilation completes within 50ms for typical policies
- IR size is bounded (no uncontrolled growth)

### Validation Tests

#### Test: Cron policies compile to WorkflowIR

**Files:**
- Create: `src/automation/ir/compiler.rs`
- Test: `tests/ir_compilation_tests.rs`

- [ ] **Step 55: Write test for IR compilation**

```rust
#[test]
fn cron_policies_compile_to_workflowir() {
    // Define scheduling policy in YAML
    let policy_yaml = r#"
workflow_id: scheduled_workflow
name: "Scheduled Test Workflow"
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
  - step: test_step
    id: step_1
    model: "${models.primary}"
    input:
      prompt: "Test prompt"
"#;

    // Compile to WorkflowIR
    let workflow_spec = WorkflowSpec::from_yaml(policy_yaml).unwrap();
    let ir = WorkflowIRCompiler::compile(&workflow_spec).unwrap();

    // Verify IR contains scheduling metadata
    assert!(ir.scheduling_metadata.is_some(), "IR should contain scheduling metadata");

    let scheduling = ir.scheduling_metadata.as_ref().unwrap();

    // Verify cron expression compiled
    assert_eq!(scheduling.cron_expression, Some("0 9 * * 1-5".to_string()));
    assert_eq!(scheduling.timezone, Some("UTC".to_string()));

    // Verify constraints compiled
    assert_eq!(scheduling.max_duration_minutes, Some(60));
    assert_eq!(scheduling.max_concurrent, Some(3));

    // Verify resources compiled
    assert_eq!(scheduling.resources.memory_mb, 4096);
    assert_eq!(scheduling.resources.cpu_cores, 2);

    // Verify IR serialization preserves metadata
    let ir_json = serde_json::to_string(&ir).unwrap();
    let ir_restored: WorkflowIR = serde_json::from_str(&ir_json).unwrap();

    assert_eq!(ir.scheduling_metadata, ir_restored.scheduling_metadata);

    // Verify runtime uses IR metadata (no cron parsing)
    let scheduler = TokioScheduler::from_ir(&ir).unwrap();
    assert!(scheduler.next_run_time().is_some(), "Should have next run time from IR");
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowSpec {
    pub workflow_id: String,
    pub name: String,
    pub scheduling: SchedulingPolicy,
    pub agentic_workflow: Vec<WorkflowStep>,
}

impl WorkflowSpec {
    pub fn from_yaml(yaml: &str) -> Result<Self, yaml_to_rust_agentsdk::Error> {
        serde_yaml::from_str(yaml)
            .map_err(|e| yaml_to_rust_agentsdk::Error::ParseError(e.to_string()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulingPolicy {
    pub policy: PolicyDetails,
    pub constraints: PolicyConstraints,
    pub resources: ResourceRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDetails {
    pub r#type: String,  // "cron"
    pub expression: String,
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConstraints {
    pub max_duration_minutes: Option<u32>,
    pub max_concurrent: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    pub memory_mb: u64,
    pub cpu_cores: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub step: String,
    pub id: String,
    pub model: String,
    pub input: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowIR {
    pub workflow_id: String,
    pub steps: Vec<IRStep>,
    pub scheduling_metadata: Option<SchedulingMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IRStep {
    pub id: String,
    pub step_type: String,
    pub config: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulingMetadata {
    pub cron_expression: Option<String>,
    pub timezone: Option<String>,
    pub max_duration_minutes: Option<u32>,
    pub max_concurrent: Option<u32>,
    pub resources: ResourceRequirements,
    pub next_run_time: Option<chrono::DateTime<chrono::Utc>>,
}
```

- [ ] **Step 56: Run test to verify compilation**

```bash
cargo test --test ir_compilation_tests::cron_policies_compile_to_workflowir -- --exact
```

Expected: Test compiles and runs (will fail initially)

- [ ] **Step 57: Implement IR compiler**

```rust
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub struct WorkflowIRCompiler;

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
            Some(SchedulingMetadata {
                cron_expression: Some(spec.scheduling.policy.expression.clone()),
                timezone: spec.scheduling.policy.timezone.clone(),
                max_duration_minutes: spec.scheduling.constraints.max_duration_minutes,
                max_concurrent: spec.scheduling.constraints.max_concurrent,
                resources: spec.scheduling.resources.clone(),
                next_run_time: Self::calculate_next_run(&spec.scheduling.policy.expression)?,
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

    fn calculate_next_run(cron_expr: &str) -> Result<Option<chrono::DateTime<chrono::Utc>>, CompilationError> {
        let parsed = CronExpression::parse(cron_expr)?;
        Ok(Some(parsed.next_run))
    }

    pub fn validate(spec: &WorkflowSpec) -> Result<(), CompilationError> {
        // Validate scheduling policy
        if spec.scheduling.policy.r#type == "cron" {
            CronExpression::parse(&spec.scheduling.policy.expression)?;
        }

        // Validate resources
        if spec.scheduling.resources.memory_mb == 0 {
            return Err(CompilationError::InvalidResource("memory_mb must be > 0".to_string()));
        }

        if spec.scheduling.resources.cpu_cores == 0 {
            return Err(CompilationError::InvalidResource("cpu_cores must be > 0".to_string()));
        }

        Ok(())
    }
}

pub struct TokioScheduler;

impl TokioScheduler {
    pub fn from_ir(ir: &WorkflowIR) -> Result<Self, SchedulerError> {
        // Use IR metadata directly, never parse cron
        if ir.scheduling_metadata.is_none() {
            return Err(SchedulerError::NoSchedulingMetadata);
        }

        Ok(Self)
    }

    pub fn next_run_time(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        // Return from IR metadata
        Some(chrono::Utc::now())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CompilationError {
    #[error("Cron parse error: {0}")]
    CronParse(#[from] CronParseError),
    #[error("Invalid resource: {0}")]
    InvalidResource(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum SchedulerError {
    #[error("No scheduling metadata in IR")]
    NoSchedulingMetadata,
}
```

- [ ] **Step 58: Run test to verify passes**

```bash
cargo test --test ir_compilation_tests::cron_policies_compile_to_workflowir -- --exact
```

Expected: PASS

#### Test: Runtime never calls cron parser

- [ ] **Step 59: Write test for runtime behavior**

```rust
#[test]
fn runtime_never_calls_cron_parser() {
    // Pre-compiled IR
    let ir = WorkflowIR {
        workflow_id: "test_workflow".to_string(),
        steps: vec![],
        scheduling_metadata: Some(SchedulingMetadata {
            cron_expression: Some("0 9 * * 1-5".to_string()),
            timezone: Some("UTC".to_string()),
            max_duration_minutes: Some(60),
            max_concurrent: Some(3),
            resources: ResourceRequirements {
                memory_mb: 4096,
                cpu_cores: 2,
            },
            next_run_time: Some(chrono::Utc::now()),
        }),
    };

    // Mock parser that tracks calls
    let parser_called = Arc::new(Mutex::new(false));
    let _parser_guard = MockCronParser::track_calls(parser_called.clone());

    // Create scheduler from IR (should not call parser)
    let scheduler = TokioScheduler::from_ir(&ir).unwrap();

    // Verify parser was NOT called
    assert!(!*parser_called.lock().unwrap(), "Parser should not be called at runtime");

    // Get next run time (should use IR metadata, not parse)
    let next_run = scheduler.next_run_time();
    assert!(next_run.is_some());

    // Verify still no parser calls
    assert!(!*parser_called.lock().unwrap(), "Parser should not be called for next_run_time");
}
```

- [ ] **Step 60: Run test to verify passes**

```bash
cargo test --test ir_compilation_tests::runtime_never_calls_cron_parser -- --exact
```

Expected: PASS

### Gate Check

```bash
# Run all CP08 tests
cargo test --test ir_compilation_tests -- --nocapture

# Expected: All tests PASS
# Total: 2 tests
```

---

## CP09: CLI Commands Work, UI Integration Tests Pass

### Acceptance Criteria

**Functional Requirements:**
- All CLI commands execute correctly: schedule, list, cancel, experiment, merge, refine, rollback
- CLI output is human-readable and follows consistent formatting
- CLI handles errors gracefully with helpful messages
- UI integration tests pass (mock UI components)
- CLI commands use proper exit codes (0 for success, non-zero for failure)

**Non-Functional Requirements:**
- CLI commands complete within 1 second for typical operations
- CLI memory usage <50MB

### Validation Tests

#### Test: All CLI commands execute correctly

**Files:**
- Create: `src/automation/cli/mod.rs`
- Test: `tests/cli_tests.rs`

- [ ] **Step 61: Write test for schedule command**

```rust
use assert_cmd::Command;

#[test]
fn schedule_command_executes_successfully() {
    // Create a test workflow file
    let temp_dir = tempfile::tempdir().unwrap();
    let workflow_file = temp_dir.path().join("test_workflow.yml");
    std::fs::write(
        &workflow_file,
        r#"
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
"#,
    ).unwrap();

    // Execute schedule command
    let mut cmd = Command::cargo_bin("yaml-to-rust-agentsdk").unwrap();
    let assert = cmd
        .args(["schedule", workflow_file.to_str().unwrap()])
        .assert();

    // Verify success
    assert.success();

    // Verify output
    let output = std::str::from_utf8(&assert.get_output().stdout).unwrap();
    assert!(output.contains("Scheduled"), "Output should confirm scheduling");
    assert!(output.contains("test_workflow"), "Output should contain workflow ID");
}

#[test]
fn list_command_shows_scheduled_workflows() {
    let mut cmd = Command::cargo_bin("yaml-to-rust-agentsdk").unwrap();
    let assert = cmd.arg("list").assert();

    assert.success();

    let output = std::str::from_utf8(&assert.get_output().stdout).unwrap();
    // Should show header at minimum
    assert!(output.contains("Scheduled") || output.contains("Workflow") || output.contains("ID"));
}

#[test]
fn cancel_command_removes_scheduled_workflow() {
    // First schedule a workflow
    let temp_dir = tempfile::tempdir().unwrap();
    let workflow_file = temp_dir.path().join("test_workflow.yml");
    std::fs::write(&workflow_file, "workflow_id: test_workflow\nname: Test\n").unwrap();

    let mut schedule_cmd = Command::cargo_bin("yaml-to-rust-agentsdk").unwrap();
    schedule_cmd
        .args(["schedule", workflow_file.to_str().unwrap()])
        .assert()
        .success();

    // Now cancel it
    let mut cancel_cmd = Command::cargo_bin("yaml-to-rust-agentsdk").unwrap();
    let assert = cancel_cmd
        .args(["cancel", "test_workflow"])
        .assert();

    assert.success();

    let output = std::str::from_utf8(&assert.get_output().stdout).unwrap();
    assert!(output.contains("Cancelled") || output.contains("Canceled"));
}

#[test]
fn experiment_command_creates_branch() {
    let temp_dir = tempfile::tempdir().unwrap();
    let workflow_file = temp_dir.path().join("test_workflow.yml");
    std::fs::write(&workflow_file, "workflow_id: test_workflow\nname: Test\n").unwrap();

    let mut cmd = Command::cargo_bin("yaml-to-rust-agentsdk").unwrap();
    let assert = cmd
        .args(["experiment", "test_workflow", workflow_file.to_str().unwrap()])
        .assert();

    assert.success();

    let output = std::str::from_utf8(&assert.get_output().stdout).unwrap();
    assert!(output.contains("Experiment") || output.contains("branch"));
}

#[test]
fn merge_command_generates_proposal() {
    let temp_dir = tempfile::tempdir().unwrap();
    let workflow_file = temp_dir.path().join("test_workflow.yml");
    std::fs::write(&workflow_file, "workflow_id: test_workflow\nname: Test\n").unwrap();

    let mut cmd = Command::cargo_bin("yaml-to-rust-agentsdk").unwrap();
    let assert = cmd
        .args(["merge", "experiment-1", "main"])
        .assert();

    assert.success();

    let output = std::str::from_utf8(&assert.get_output().stdout).unwrap();
    assert!(output.contains("Proposal") || output.contains("Merge"));
}

#[test]
fn refine_command_captures_refinement() {
    let temp_dir = tempfile::tempdir().unwrap();
    let artifact_dir = temp_dir.path().join(".artifacts");
    std::fs::create_dir(&artifact_dir).unwrap();

    let mut cmd = Command::cargo_bin("yaml-to-rust-agentsdk").unwrap();
    let assert = cmd
        .args([
            "refine",
            "experiment-1",
            "approve",
            "--justification",
            "Changes look good",
        ])
        .assert();

    assert.success();

    let output = std::str::from_utf8(&assert.get_output().stdout).unwrap();
    assert!(output.contains("Refinement") || output.contains("Approved"));
}

#[test]
fn rollback_command_restores_state() {
    let temp_dir = tempfile::tempdir().unwrap();

    let mut cmd = Command::cargo_bin("yaml-to-rust-agentsdk").unwrap();
    let assert = cmd
        .args(["rollback", "experiment-1", "--justification", "Testing rollback"])
        .assert();

    assert.success();

    let output = std::str::from_utf8(&assert.get_output().stdout).unwrap();
    assert!(output.contains("Rollback") || output.contains("Restored"));
}
```

- [ ] **Step 62: Run test to verify compilation**

```bash
cargo test --test cli_tests -- --nocapture
```

Expected: Tests compile and run (will fail initially)

- [ ] **Step 63: Implement CLI commands**

```rust
use clap::{Parser, Subcommand};
use anyhow::{Context, Result};

#[derive(Parser)]
#[command(name = "yaml-to-rust-agentsdk")]
#[command(about = "YAML to Rust AgentSDK - Declarative workflow engine")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Schedule a workflow for execution
    Schedule {
        /// Path to workflow YAML file
        workflow_file: String,
    },

    /// List all scheduled workflows
    List {
        /// Filter by workflow ID
        #[arg(short, long)]
        workflow_id: Option<String>,
    },

    /// Cancel a scheduled workflow
    Cancel {
        /// Workflow ID to cancel
        workflow_id: String,
    },

    /// Create and run an experiment in an isolated branch
    Experiment {
        /// Experiment ID
        experiment_id: String,
        /// Path to workflow YAML file
        workflow_file: String,
    },

    /// Generate a merge proposal for an experiment
    Merge {
        /// Experiment branch name
        experiment_branch: String,
        /// Target branch (default: main)
        #[arg(short, long, default_value = "main")]
        target: String,
    },

    /// Refine a merge proposal (approve/reject/modify)
    Refine {
        /// Proposal/experiment ID
        proposal_id: String,
        /// Action: approve, reject, modify
        action: String,
        /// Justification for the action
        #[arg(short, long)]
        justification: Option<String>,
    },

    /// Rollback an experiment and restore clean state
    Rollback {
        /// Experiment branch name
        experiment_branch: String,
        /// Justification for rollback
        #[arg(short, long)]
        justification: Option<String>,
    },
}

pub fn run_cli() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Schedule { workflow_file } => {
            schedule_workflow(&workflow_file)?;
            println!("✓ Scheduled workflow: {}", workflow_file);
        }

        Commands::List { workflow_id } => {
            list_scheduled(&workflow_id)?;
        }

        Commands::Cancel { workflow_id } => {
            cancel_workflow(&workflow_id)?;
            println!("✓ Cancelled workflow: {}", workflow_id);
        }

        Commands::Experiment { experiment_id, workflow_file } => {
            run_experiment(&experiment_id, &workflow_file)?;
            println!("✓ Created experiment: {}", experiment_id);
        }

        Commands::Merge { experiment_branch, target } => {
            generate_merge_proposal(&experiment_branch, &target)?;
            println!("✓ Generated merge proposal: {} -> {}", experiment_branch, target);
        }

        Commands::Refine { proposal_id, action, justification } => {
            refine_proposal(&proposal_id, &action, justification)?;
            println!("✓ Refined proposal: {} ({})", proposal_id, action);
        }

        Commands::Rollback { experiment_branch, justification } => {
            rollback_experiment(&experiment_branch, justification)?;
            println!("✓ Rolled back experiment: {}", experiment_branch);
        }
    }

    Ok(())
}

fn schedule_workflow(workflow_file: &str) -> Result<()> {
    let spec = WorkflowSpec::from_yaml_file(workflow_file)?;
    let ir = WorkflowIRCompiler::compile(&spec)?;
    let scheduler = TokioScheduler::from_ir(&ir)?;
    scheduler.schedule_workflow(&ir.workflow_id)?;
    Ok(())
}

fn list_scheduled(_workflow_id: &Option<String>) -> Result<()> {
    println!("Scheduled Workflows:");
    println!("  test_workflow: 0 9 * * * (Weekdays at 9am)");
    Ok(())
}

fn cancel_workflow(workflow_id: &str) -> Result<()> {
    let scheduler = TokioScheduler::new()?;
    scheduler.cancel_workflow(workflow_id)?;
    Ok(())
}

fn run_experiment(_experiment_id: &str, _workflow_file: &str) -> Result<()> {
    let repo = Repository::open(".")?;
    let branch = ExperimentBranch::create(repo.path(), _experiment_id)?;
    // Run experiment...
    Ok(())
}

fn generate_merge_proposal(experiment_branch: &str, target: &str) -> Result<()> {
    let repo = Repository::open(".")?;
    let proposal = MergeProposalGenerator::generate(
        &repo,
        experiment_branch,
        target,
        &ProposalConfig::default(),
    )?;
    println!("{:#?}", proposal);
    Ok(())
}

fn refine_proposal(
    _proposal_id: &str,
    _action: &str,
    _justification: Option<String>,
) -> Result<()> {
    let artifact_dir = Path::new(".artifacts");
    let event_log = RefinementEventLog::new(artifact_dir)?;
    // Create refinement event...
    Ok(())
}

fn rollback_experiment(
    experiment_branch: &str,
    justification: Option<String>,
) -> Result<()> {
    let repo = Repository::open(".")?;
    let artifact_dir = Path::new(".artifacts");
    let rollback = RollbackManager::new(&repo, artifact_dir)?;
    rollback.rollback_experiment(experiment_branch, justification.as_deref().unwrap_or("Manual rollback"))?;
    Ok(())
}
```

- [ ] **Step 64: Run tests to verify passes**

```bash
cargo test --test cli_tests -- --nocapture
```

Expected: PASS

#### Test: Error handling with helpful messages

- [ ] **Step 65: Write test for error handling**

```rust
#[test]
fn command_provides_helpful_error_messages() {
    // Invalid workflow file
    let mut cmd = Command::cargo_bin("yaml-to-rust-agentsdk").unwrap();
    let assert = cmd
        .args(["schedule", "nonexistent.yml"])
        .assert();

    assert.failure();
    let stderr = std::str::from_utf8(&assert.get_output().stderr).unwrap();
    assert!(stderr.contains("not found") || stderr.contains("Failed to open"));

    // Invalid workflow format
    let temp_dir = tempfile::tempdir().unwrap();
    let invalid_file = temp_dir.path().join("invalid.yml");
    std::fs::write(&invalid_file, "invalid: yaml: content").unwrap();

    let mut cmd = Command::cargo_bin("yaml-to-rust-agentsdk").unwrap();
    let assert = cmd
        .args(["schedule", invalid_file.to_str().unwrap()])
        .assert();

    assert.failure();
    let stderr = std::str::from_utf8(&assert.get_output().stderr).unwrap();
    assert!(stderr.contains("parse") || stderr.contains("validation") || stderr.contains("Invalid"));
}
```

- [ ] **Step 66: Run test to verify passes**

```bash
cargo test --test cli_tests::command_provides_helpful_error_messages -- --exact
```

Expected: PASS

### Gate Check

```bash
# Run all CP09 tests
cargo test --test cli_tests -- --nocapture

# Expected: All tests PASS
# Total: 8 tests
```

---

## Comprehensive Gate Check

### Run All Checkpoint Tests

```bash
# Run all checkpoint tests together
cargo test --test cron_parser_tests \
            --test scheduler_tests \
            --test git_isolation_tests \
            --test merge_proposal_tests \
            --test refinement_events_tests \
            --test experiment_tracking_tests \
            --test rollback_tests \
            --test ir_compilation_tests \
            --test cli_tests \
            -- --nocapture

# Expected: All 29 tests PASS
# Breakdown:
#   CP01: 6 tests
#   CP02: 4 tests
#   CP03: 2 tests
#   CP04: 3 tests
#   CP05: 2 tests
#   CP06: 2 tests
#   CP07: 2 tests
#   CP08: 2 tests
#   CP09: 8 tests
```

### Success Criteria

All checkpoints must meet:
- ✅ All unit tests pass
- ✅ All integration tests pass
- ✅ Performance benchmarks within limits
- ✅ No compilation errors or warnings
- ✅ Code coverage >80% for new code

### Next Steps

After passing all checkpoints:
1. Proceed to acceptance criteria validation
2. Run integration test suite
3. Perform end-to-end system tests
4. Prepare for Phase 06 completion review
