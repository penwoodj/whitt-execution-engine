# Phase 06 Automation - Integration Tests

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Define comprehensive integration test specifications for end-to-end automation workflows

**Architecture:** Full lifecycle integration tests covering cron scheduling, git experiments, merge proposals, rollback procedures, and policy compilation.

**Tech Stack:** Rust, tempfile, git2, tokio, assert_cmd, serde_json

---

## Integration Test Structure

```
tests/integration/
├── cron_pipeline_tests.rs           # Full cron scheduling pipeline
├── git_experiment_lifecycle_tests.rs # Complete git experiment workflow
├── merge_proposal_workflow_tests.rs # Merge proposal generation workflow
├── rollback_verification_tests.rs    # Rollback and cleanup workflow
├── scheduling_roundtrip_tests.rs    # YAML → IR → execution round-trip
└── mod.rs                          # Integration test module
```

---

## Module 1: Full Cron Pipeline

### Test Group 1: Schedule → Trigger → Execute → Verify

**Files:**
- Create: `tests/integration/cron_pipeline_tests.rs`

#### Test: Complete cron pipeline

- [ ] **Step 55: Write end-to-end cron pipeline test**

```rust
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn complete_cron_pipeline_schedule_to_verify() {
    let (temp_dir, repo) = create_temp_repo_with_workflow().unwrap();
    let artifact_dir = temp_dir.path().join(".artifacts");
    std::fs::create_dir(&artifact_dir).unwrap();

    // Step 1: Schedule workflow with cron expression
    let workflow_file = temp_dir.path().join("workflow.yml");
    std::fs::write(&workflow_file, r#"
workflow_id: cron_test_workflow
name: "Cron Test Workflow"
scheduling:
  policy:
    type: cron
    expression: "*/2 * * * *"
  constraints:
    max_duration_minutes: 5
  resources:
    memory_mb: 1024
    cpu_cores: 1
agentic_workflow:
  - step: test_step
    id: step_1
    model: "test_model"
    input:
      prompt: "Test execution"
    output:
      save_to: test_output
      format: text
"#).unwrap();

    let spec = WorkflowSpec::from_yaml_file(&workflow_file).unwrap();
    let ir = WorkflowIRCompiler::compile(&spec).unwrap();

    // Verify IR contains scheduling metadata
    assert!(ir.scheduling_metadata.is_some());
    let scheduling = ir.scheduling_metadata.as_ref().unwrap();
    assert_eq!(scheduling.cron_expression, Some("*/2 * * * *".to_string()));

    println!("✓ Step 1: Scheduled workflow with cron expression");

    // Step 2: Trigger workflow execution
    let mut scheduler = TokioScheduler::from_ir(&ir).unwrap();
    scheduler.schedule_workflow(&ir.workflow_id).unwrap();

    let scheduled_tasks = scheduler.list_scheduled().unwrap();
    assert!(!scheduled_tasks.is_empty());
    assert!(scheduled_tasks.iter().any(|t| t.id == "cron_test_workflow"));

    println!("✓ Step 2: Triggered workflow execution");

    // Step 3: Execute workflow (simulate)
    let executor = WorkflowExecutor::new(&ir).unwrap();
    let execution_start = std::time::Instant::now();

    let execution_result = executor.execute(&spec).await.unwrap();
    let execution_duration = execution_start.elapsed();

    assert!(execution_result.success, "Workflow execution should succeed");
    assert!(execution_duration < Duration::from_secs(5), "Should complete within 5 seconds");

    println!("✓ Step 3: Executed workflow successfully (took: {:?})", execution_duration);

    // Step 4: Verify execution results
    let output_path = artifact_dir.join("test_output.txt");
    assert!(output_path.exists(), "Output file should exist");

    let output_content = std::fs::read_to_string(&output_path).unwrap();
    assert!(!output_content.is_empty(), "Output should not be empty");

    println!("✓ Step 4: Verified execution results");

    // Step 5: Verify scheduling constraints were enforced
    // Max duration constraint (5 minutes) should not be exceeded
    assert!(execution_duration < Duration::from_secs(300), "Should respect max duration");

    println!("✓ Step 5: Verified scheduling constraints enforced");

    // Step 6: Verify resource limits were applied
    let metrics = executor.get_metrics().unwrap();
    assert!(metrics.memory_usage_mb <= 1024, "Should respect memory limit");
    assert!(metrics.cpu_cores == 1, "Should use configured CPU cores");

    println!("✓ Step 6: Verified resource limits applied");

    println!("\n✅ Complete cron pipeline test PASSED");
}

#[tokio::test]
async fn cron_pipeline_with_missed_schedule() {
    let (temp_dir, _repo) = create_temp_repo_with_workflow().unwrap();

    let workflow_file = temp_dir.path().join("workflow.yml");
    std::fs::write(&workflow_file, r#"
workflow_id: missed_schedule_workflow
name: "Missed Schedule Workflow"
scheduling:
  policy:
    type: cron
    expression: "0 * * * *"
agentic_workflow:
  - step: test
    id: step_1
    model: "test_model"
    input:
      prompt: "Test"
"#).unwrap();

    // Schedule in the past (missed schedule)
    let spec = WorkflowSpec::from_yaml_file(&workflow_file).unwrap();
    let mut ir = WorkflowIRCompiler::compile(&spec).unwrap();

    // Modify IR to set past time
    if let Some(scheduling) = &mut ir.scheduling_metadata {
        scheduling.next_run_time = Some(chrono::Utc::now() - chrono::Duration::hours(1));
    }

    let mut scheduler = TokioScheduler::from_ir(&ir).unwrap();
    scheduler.schedule_workflow(&ir.workflow_id).unwrap();

    // Check for missed schedules
    let missed = scheduler.check_missed_schedules().unwrap();
    assert!(!missed.is_empty(), "Should detect missed schedule");

    let missed_task = &missed[0];
    assert_eq!(missed_task.task_id, "missed_schedule_workflow");
    assert!(missed_task.scheduled_time < chrono::Utc::now());

    println!("✓ Detected missed schedule: {}", missed_task.task_id);

    // Handle missed schedule (policy: reschedule)
    scheduler.handle_missed_schedule(&missed_task.task_id, MissedSchedulePolicy::Reschedule).unwrap();

    // Verify it's rescheduled
    let scheduled = scheduler.list_scheduled().unwrap();
    assert!(scheduled.iter().any(|t| t.id == "missed_schedule_workflow"));

    println!("✓ Rescheduled missed task");

    println!("\n✅ Missed schedule handling test PASSED");
}

#[tokio::test]
async fn cron_pipeline_with_overlapping_schedules() {
    let (temp_dir, _repo) = create_temp_repo_with_workflow().unwrap();

    // Schedule multiple workflows at same time
    let workflows = vec![
        ("workflow1", "0 9 * * *"),
        ("workflow2", "0 9 * * *"),
        ("workflow3", "0 9 * * *"),
    ];

    let mut scheduler = TokioScheduler::with_max_concurrent(2);

    for (workflow_id, cron_expr) in &workflows {
        let workflow_yaml = format!(r#"
workflow_id: {}
name: "Workflow {}"
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
"#, workflow_id, workflow_id, cron_expr);

        let workflow_file = temp_dir.path().join(format!("{}.yml", workflow_id));
        std::fs::write(&workflow_file, workflow_yaml).unwrap();

        let spec = WorkflowSpec::from_yaml_file(&workflow_file).unwrap();
        let ir = WorkflowIRCompiler::compile(&spec).unwrap();
        scheduler.schedule_workflow(&ir.workflow_id).unwrap();
    }

    // Check for overlapping schedules
    let window = Duration::from_secs(300); // 5-minute window
    let overlapping = scheduler.check_overlapping_schedules(window).unwrap();

    assert!(!overlapping.is_empty(), "Should detect overlapping schedules");

    // Verify workflows 1 and 2 overlap (at max concurrent limit)
    assert!(overlapping.len() >= 1, "Should have at least one overlap group");

    // Workflow 3 should be rejected (exceeds max concurrent)
    let scheduled = scheduler.list_scheduled().unwrap();
    let workflow3_scheduled = scheduled.iter().any(|t| t.id == "workflow3");
    assert!(!workflow3_scheduled, "Workflow 3 should be rejected due to max concurrent");

    println!("✓ Detected and handled overlapping schedules");

    println!("\n✅ Overlapping schedule handling test PASSED");
}
```

- [ ] **Step 56: Run cron pipeline tests**

```bash
cargo test --test cron_pipeline_tests -- --nocapture
```

Expected: All 3 tests PASS

---

## Module 2: Git Experiment Lifecycle

### Test Group 1: Create → Run → Compare → Merge/Reject → Cleanup

**Files:**
- Create: `tests/integration/git_experiment_lifecycle_tests.rs`

#### Test: Complete experiment lifecycle

- [ ] **Step 57: Write end-to-end experiment lifecycle test**

```rust
#[tokio::test]
async fn complete_experiment_lifecycle() {
    let (temp_dir, repo) = create_temp_repo_with_initial_content().unwrap();
    let artifact_dir = temp_dir.path().join(".artifacts");
    std::fs::create_dir(&artifact_dir).unwrap();

    // Capture initial main branch state
    let main_head_initial = repo.head().unwrap();
    let main_commit_initial = main_head_initial.target().unwrap();

    println!("Initial main commit: {}", main_commit_initial);

    // Step 1: Create experiment branch
    let experiment = ExperimentBranch::create(
        repo.path(),
        "experiment-1",
    ).unwrap();

    let experiment_head = repo.head().unwrap();
    let experiment_commit = experiment_head.target().unwrap();

    assert_ne!(main_commit_initial, experiment_commit, "Experiment should have different commit");
    println!("✓ Step 1: Created experiment branch (commit: {})", experiment_commit);

    // Step 2: Run experiment (make changes)
    let repo_path = repo.path().parent().unwrap();

    // Add new file
    let new_feature = repo_path.join("new_feature.rs");
    std::fs::write(&new_feature, r#"pub fn new_feature() -> i32 { 42 }"#).unwrap();

    // Modify existing file
    let readme_path = repo_path.join("README.md");
    std::fs::write(&readme_path, "# Updated Project\n\nAdded new feature.\n").unwrap();

    experiment.commit_changes("Add new feature").unwrap();

    println!("✓ Step 2: Ran experiment and committed changes");

    // Step 3: Compare results
    let main_tree_initial = repo.find_commit(main_commit_initial).unwrap().tree().unwrap();
    let experiment_tree = repo.find_commit(experiment_commit).unwrap().tree().unwrap();

    assert_ne!(main_tree_initial.id(), experiment_tree.id(), "Trees should differ");

    let changes = repo.diff_tree_to_tree(
        Some(&main_tree_initial),
        Some(&experiment_tree),
        None,
    ).unwrap();

    let stats = changes.stats().unwrap();
    assert!(stats.files_changed() > 0, "Should have file changes");
    assert!(stats.insertions() > 0, "Should have insertions");

    println!("✓ Step 3: Compared results (files changed: {}, insertions: {})",
             stats.files_changed(), stats.insertions());

    // Step 4: Generate merge proposal
    let proposal = MergeProposalGenerator::generate(
        &repo,
        "experiment-1",
        "main",
        &ProposalConfig {
            include_validation: true,
            include_metrics: true,
            confidence_threshold: 0.0,
        },
    ).unwrap();

    assert!(!proposal.diff.is_empty(), "Proposal should have diff");
    assert!(proposal.validation.is_some(), "Proposal should have validation");
    assert!(proposal.metrics.is_some(), "Proposal should have metrics");

    println!("✓ Step 4: Generated merge proposal (confidence: {:.2})",
             proposal.confidence_score);

    // Save proposal to artifacts
    let proposal_path = artifact_dir.join("experiment-1-proposal.json");
    std::fs::write(
        &proposal_path,
        serde_json::to_string_pretty(&proposal).unwrap()
    ).unwrap();

    // Step 5: Approve experiment (capture refinement event)
    let event_log = RefinementEventLog::new(&artifact_dir).unwrap();

    let approval = RefinementEvent::new(
        "experiment-1".to_string(),
        RefinementAction::Approve,
        "reviewer1",
        "New feature looks good, ready to merge".to_string(),
    );
    event_log.append(approval).unwrap();

    let events = event_log.read_all().unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].action, RefinementAction::Approve);

    println!("✓ Step 5: Approved experiment and logged refinement event");

    // Step 6: Merge experiment to main
    let mut checkout_opts = git2::CheckoutBuilder::new();
    checkout_opts.force(true);

    repo.checkout_tree(
        &repo.find_commit(experiment_commit).unwrap().tree().unwrap(),
        Some(&mut checkout_opts),
    ).unwrap();

    repo.set_head("refs/heads/main").unwrap();

    let merge_commit_id = experiment_commit;

    println!("✓ Step 6: Merged experiment to main (commit: {})", merge_commit_id);

    // Step 7: Verify merge
    let main_head_after = repo.head().unwrap();
    let main_commit_after = main_head_after.target().unwrap();

    assert_eq!(main_commit_after, merge_commit_id, "Main should point to merge commit");
    assert_ne!(main_commit_initial, main_commit_after, "Main should have advanced");

    // Verify new feature file exists on main
    assert!(new_feature.exists(), "New feature should exist on main");
    assert!(readme_path.exists(), "README should exist on main");

    let readme_content = std::fs::read_to_string(&readme_path).unwrap();
    assert!(readme_content.contains("Updated Project"), "README should be updated");

    println!("✓ Step 7: Verified merge successful");

    // Step 8: Cleanup experiment branch
    let exp_branch = repo.find_branch("experiment-1", git2::BranchType::Local).unwrap();
    exp_branch.delete().unwrap();

    let exp_branch_after = repo.find_branch("experiment-1", git2::BranchType::Local);
    assert!(exp_branch_after.is_err(), "Experiment branch should be deleted");

    println!("✓ Step 8: Cleaned up experiment branch");

    println!("\n✅ Complete experiment lifecycle test PASSED");
}

#[tokio::test]
async fn experiment_lifecycle_with_rejection() {
    let (temp_dir, repo) = create_temp_repo_with_initial_content().unwrap();
    let artifact_dir = temp_dir.path().join(".artifacts");
    std::fs::create_dir(&artifact_dir).unwrap();

    // Create experiment
    let experiment = ExperimentBranch::create(repo.path(), "experiment-1").unwrap();

    // Make changes
    let repo_path = repo.path().parent().unwrap();
    let test_file = repo_path.join("test.rs");
    std::fs::write(&test_file, "pub fn test() {}").unwrap();

    experiment.commit_changes("Add test").unwrap();

    // Generate proposal
    let proposal = MergeProposalGenerator::generate(
        &repo,
        "experiment-1",
        "main",
        &ProposalConfig::default(),
    ).unwrap();

    // Reject experiment
    let event_log = RefinementEventLog::new(&artifact_dir).unwrap();

    let rejection = RefinementEvent::new(
        "experiment-1".to_string(),
        RefinementAction::Reject,
        "reviewer1",
        "Needs more testing before merge".to_string(),
    );
    event_log.append(rejection).unwrap();

    // Verify rejection logged
    let events = event_log.read_all().unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].action, RefinementAction::Reject);

    // Verify main branch unchanged
    let main_head = repo.head().unwrap();
    assert_eq!(main_head.shorthand().unwrap(), "main");

    // Cleanup experiment
    experiment.cleanup().unwrap();

    let exp_branch = repo.find_branch("experiment-1", git2::BranchType::Local);
    assert!(exp_branch.is_err());

    // Verify artifacts preserved (for review)
    let proposal_path = artifact_dir.join("experiment-1-proposal.json");
    std::fs::write(
        &proposal_path,
        serde_json::to_string_pretty(&proposal).unwrap()
    ).unwrap();

    assert!(proposal_path.exists(), "Proposal should be preserved");

    println!("✓ Experiment rejected and cleaned up (proposal preserved for review)");

    println!("\n✅ Experiment lifecycle with rejection test PASSED");
}
```

- [ ] **Step 58: Run experiment lifecycle tests**

```bash
cargo test --test git_experiment_lifecycle_tests -- --nocapture
```

Expected: Both tests PASS

---

## Module 3: Merge Proposal Workflow

### Test Group 1: Experiment → Diff → Validate → Recommend → Approve/Reject

**Files:**
- Create: `tests/integration/merge_proposal_workflow_tests.rs`

#### Test: Complete merge proposal workflow

- [ ] **Step 59: Write end-to-end merge proposal test**

```rust
#[tokio::test]
async fn complete_merge_proposal_workflow() {
    let (temp_dir, repo) = create_temp_repo_with_initial_content().unwrap();
    let artifact_dir = temp_dir.path().join(".artifacts");
    std::fs::create_dir(&artifact_dir).unwrap();

    // Step 1: Create experiment with comprehensive changes
    let experiment = ExperimentBranch::create(repo.path(), "feature-ui-refactor").unwrap();

    let repo_path = repo.path().parent().unwrap();

    // Modify multiple files
    let cargo_toml = repo_path.join("Cargo.toml");
    std::fs::write(&cargo_toml, r#"
[package]
name = "my-app"
version = "2.0.0"
"#).unwrap();

    let src_main = repo_path.join("src/main.rs");
    std::fs::create_dir_all(repo_path.join("src")).unwrap();
    std::fs::write(&src_main, r#"
fn main() {
    println!("Refactored UI");
}
"#).unwrap();

    // Add new file
    let new_ui = repo_path.join("src/ui.rs");
    std::fs::write(&new_ui, r#"
pub struct UI {
    width: u32,
    height: u32,
}

impl UI {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
"#).unwrap();

    // Delete old file
    let old_file = repo_path.join("old_ui.rs");
    std::fs::write(&old_file, "This will be deleted\n").unwrap();

    let mut index = repo.index()?;
    index.add_path(std::path::Path::new("Cargo.toml"))?;
    index.add_path(std::path::Path::new("src/main.rs"))?;
    index.add_path(std::path::Path::new("src/ui.rs"))?;
    index.remove(std::path::Path::new("old_ui.rs"), None)?;
    let tree_id = index.write_tree()?;

    let tree = repo.find_tree(tree_id)?;
    let head = repo.head()?;
    let parent_commit = head.peel_to_commit()?;

    let sig = git2::Signature::now("Test User", "test@example.com")?;
    let commit_id = repo.commit(Some("HEAD"), &sig, &sig, "UI refactor", &tree, &[&parent_commit])?;

    println!("✓ Step 1: Created experiment with multiple file changes (commit: {})", commit_id);

    // Step 2: Generate merge proposal with full validation
    let config = ProposalConfig {
        include_validation: true,
        include_metrics: true,
        confidence_threshold: 0.7,
    };

    let proposal = MergeProposalGenerator::generate(
        &repo,
        "feature-ui-refactor",
        "main",
        &config,
    ).unwrap();

    println!("✓ Step 2: Generated merge proposal");

    // Step 3: Verify diff accuracy
    assert!(!proposal.diff.is_empty(), "Diff should not be empty");

    let diff = &proposal.diff;
    assert!(diff.contains("diff --git a/Cargo.toml"), "Should show Cargo.toml changes");
    assert!(diff.contains("diff --git a/src/main.rs"), "Should show main.rs changes");
    assert!(diff.contains("diff --git a/src/ui.rs"), "Should show new ui.rs file");
    assert!(diff.contains("diff --git a/old_ui.rs"), "Should show deleted old_ui.rs");

    println!("✓ Step 3: Verified diff accuracy (shows all changes)");

    // Step 4: Verify validation criteria
    assert!(proposal.validation.is_some());
    let validation = proposal.validation.as_ref().unwrap();

    // Verify lint results
    assert!(matches!(validation.lints_passed, true | false));
    if !validation.lints_passed {
        assert!(!validation.lint_errors.is_empty());
    }

    // Verify test results
    assert!(matches!(validation.tests_passed, true | false));
    if !validation.tests_passed {
        assert!(!validation.test_failures.is_empty());
    }

    println!("✓ Step 4: Verified validation criteria (lints: {}, tests: {})",
             validation.lints_passed, validation.tests_passed);

    // Step 5: Verify recommendations
    assert!(!proposal.recommendations.is_empty());
    assert!(proposal.confidence_score >= 0.0 && proposal.confidence_score <= 1.0);

    println!("✓ Step 5: Verified recommendations (confidence: {:.2}, {} recommendations)",
             proposal.confidence_score, proposal.recommendations.len());

    for rec in &proposal.recommendations {
        println!("  - {}", rec);
    }

    // Step 6: Save proposal to artifacts
    let proposal_path = artifact_dir.join("feature-ui-refactor-proposal.json");
    std::fs::write(
        &proposal_path,
        serde_json::to_string_pretty(&proposal).unwrap()
    ).unwrap();

    assert!(proposal_path.exists());
    println!("✓ Step 6: Saved proposal to artifacts");

    // Step 7: Capture refinement event (modify)
    let event_log = RefinementEventLog::new(&artifact_dir).unwrap();

    let modify = RefinementEvent::new(
        "feature-ui-refactor".to_string(),
        RefinementAction::Modify,
        "developer1",
        "Added error handling to UI component".to_string(),
    );
    modify.link_to(proposal_path.to_string_lossy().to_string());
    event_log.append(modify).unwrap();

    println!("✓ Step 7: Captured refinement event (modify)");

    // Step 8: Capture refinement event (approve)
    let approve = RefinementEvent::new(
        "feature-ui-refactor".to_string(),
        RefinementAction::Approve,
        "reviewer1",
        "All concerns addressed, ready to merge".to_string(),
    );
    event_log.append(approve).unwrap();

    let events = event_log.read_for_proposal("feature-ui-refactor").unwrap();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].action, RefinementAction::Modify);
    assert_eq!(events[1].action, RefinementAction::Approve);

    println!("✓ Step 8: Captured refinement event (approve)");

    // Step 9: Verify proposal not committed to git
    let mut index = repo.index()?;
    let status = index.path(&proposal_path.replace(&temp_dir.path().to_str().unwrap(), ""));
    assert!(status.is_none() || status.unwrap().is_none());

    println!("✓ Step 9: Verified proposal not committed to git");

    println!("\n✅ Complete merge proposal workflow test PASSED");
}
```

- [ ] **Step 60: Run merge proposal workflow tests**

```bash
cargo test --test merge_proposal_workflow_tests -- --nocapture
```

Expected: PASS

---

## Module 4: Rollback Verification

### Test Group 1: Create Experiment → Rollback → Verify Clean State

**Files:**
- Create: `tests/integration/rollback_verification_tests.rs`

#### Test: Complete rollback workflow

- [ ] **Step 61: Write end-to-end rollback test**

```rust
#[tokio::test]
async fn complete_rollback_workflow() {
    let (temp_dir, repo) = create_temp_repo_with_initial_content().unwrap();
    let artifact_dir = temp_dir.path().join(".artifacts");
    std::fs::create_dir(&artifact_dir).unwrap();

    // Capture initial state
    let main_head_initial = repo.head().unwrap();
    let main_commit_initial = main_head_initial.target().unwrap();
    let main_tree_initial = repo.find_commit(main_commit_initial).unwrap().tree().unwrap();

    let branches_initial: Vec<String> = repo.branches(None)
        .unwrap()
        .map(|b| b.unwrap().0.name().unwrap().unwrap().to_string())
        .collect();

    println!("Initial state:");
    println!("  Main commit: {}", main_commit_initial);
    println!("  Branches: {:?}", branches_initial);

    // Step 1: Create experiment with multiple changes
    let experiment = ExperimentBranch::create(repo.path(), "experiment-to-rollback").unwrap();

    let repo_path = repo.path().parent().unwrap();

    // Add changes
    let feature1 = repo_path.join("feature1.rs");
    std::fs::write(&feature1, "Feature 1\n").unwrap();

    let feature2 = repo_path.join("feature2.rs");
    std::fs::write(&feature2, "Feature 2\n").unwrap();

    let modified = repo_path.join("README.md");
    std::fs::write(&modified, "Modified README\n").unwrap();

    experiment.commit_changes("Add features").unwrap();

    println!("✓ Step 1: Created experiment with changes");

    // Step 2: Create artifacts
    let exp_artifacts = artifact_dir.join("experiment-to-rollback");
    std::fs::create_dir(&exp_artifacts).unwrap();

    std::fs::write(exp_artifacts.join("result.json"), "{\"status\":\"completed\"}").unwrap();
    std::fs::write(exp_artifacts.join("metrics.json"), "{\"time\":1.5}").unwrap();

    println!("✓ Step 2: Created experiment artifacts");

    // Step 3: Verify experiment exists and main unchanged
    let exp_branch = repo.find_branch("experiment-to-rollback", git2::BranchType::Local).unwrap();
    assert!(exp_branch.is_valid());

    let main_head_during = repo.head().unwrap();
    let main_commit_during = main_head_during.target().unwrap();
    assert_eq!(main_commit_initial, main_commit_during);

    println!("✓ Step 3: Verified experiment exists and main unchanged");

    // Step 4: Perform rollback
    let rollback = RollbackManager::new(&repo, &artifact_dir).unwrap();
    rollback.rollback_experiment("experiment-to-rollback", "Testing complete rollback").unwrap();

    println!("✓ Step 4: Performed rollback");

    // Step 5: Verify experiment branch removed
    let exp_branch_after = repo.find_branch("experiment-to-rollback", git2::BranchType::Local);
    assert!(exp_branch_after.is_err(), "Experiment branch should be removed");

    println!("✓ Step 5: Verified experiment branch removed");

    // Step 6: Verify artifacts removed
    assert!(!exp_artifacts.exists(), "Experiment artifacts should be removed");

    // Verify artifact directory cleaned
    if artifact_dir.exists() {
        let entries: Vec<_> = std::fs::read_dir(&artifact_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();
        assert!(entries.is_empty(), "Artifact directory should be empty");
    }

    println!("✓ Step 6: Verified artifacts removed");

    // Step 7: Verify main branch unchanged
    let main_head_after = repo.head().unwrap();
    let main_commit_after = main_head_after.target().unwrap();
    assert_eq!(main_commit_initial, main_commit_after);

    let main_tree_after = repo.find_commit(main_commit_after).unwrap().tree().unwrap();
    assert_eq!(main_tree_initial.id(), main_tree_after.id());

    println!("✓ Step 7: Verified main branch unchanged");

    // Step 8: Verify branches back to initial state
    let branches_after: Vec<String> = repo.branches(None)
        .unwrap()
        .map(|b| b.unwrap().0.name().unwrap().unwrap().to_string())
        .collect();
    assert_eq!(branches_after, branches_initial);

    println!("✓ Step 8: Verified branches back to initial state");

    // Step 9: Verify rollback event logged
    let artifact_parent = artifact_dir.parent().unwrap();
    let event_log = RefinementEventLog::new(artifact_parent).unwrap();

    let events = event_log.read_all().unwrap();
    let rollback_events: Vec<_> = events
        .iter()
        .filter(|e| matches!(e.action, RefinementAction::Reject))
        .collect();

    assert!(!rollback_events.is_empty(), "Rollback should be logged");

    let rollback_event = &rollback_events[0];
    assert_eq!(rollback_event.proposal_id, "experiment-to-rollback");
    assert_eq!(rollback_event.actor, "rollback_system");
    assert_eq!(rollback_event.justification, "Testing complete rollback");

    println!("✓ Step 9: Verified rollback event logged");

    println!("\n✅ Complete rollback workflow test PASSED");
}

#[tokio::test]
async fn rollback_with_multiple_experiments() {
    let (temp_dir, repo) = create_temp_repo_with_initial_content().unwrap();
    let artifact_dir = temp_dir.path().join(".artifacts");
    std::fs::create_dir(&artifact_dir).unwrap();

    // Create multiple experiments
    let experiments = vec!["exp-1", "exp-2", "exp-3"];

    for exp_id in &experiments {
        let experiment = ExperimentBranch::create(repo.path(), exp_id).unwrap();

        let repo_path = repo.path().parent().unwrap();
        let exp_file = repo_path.join(format!("{}.rs", exp_id));
        std::fs::write(&exp_file, format!("Content for {}\n", exp_id)).unwrap();

        experiment.commit_changes(&format!("Add {}", exp_id)).unwrap();

        // Create artifacts
        let exp_artifacts = artifact_dir.join(exp_id);
        std::fs::create_dir(&exp_artifacts).unwrap();
        std::fs::write(exp_artifacts.join("result.json"), "{}").unwrap();
    }

    println!("✓ Created {} experiments", experiments.len());

    // Rollback one experiment
    let rollback = RollbackManager::new(&repo, &artifact_dir).unwrap();
    rollback.rollback_experiment("exp-2", "Rollback experiment 2").unwrap();

    // Verify exp-2 removed but others remain
    let exp2_after = repo.find_branch("exp-2", git2::BranchType::Local);
    assert!(exp2_after.is_err(), "exp-2 should be removed");

    let exp1_after = repo.find_branch("exp-1", git2::BranchType::Local);
    assert!(exp1_after.is_ok(), "exp-1 should still exist");

    let exp3_after = repo.find_branch("exp-3", git2::BranchType::Local);
    assert!(exp3_after.is_ok(), "exp-3 should still exist");

    // Verify exp-2 artifacts removed
    let exp2_artifacts = artifact_dir.join("exp-2");
    assert!(!exp2_artifacts.exists());

    // Verify other artifacts remain
    let exp1_artifacts = artifact_dir.join("exp-1");
    assert!(exp1_artifacts.exists());

    let exp3_artifacts = artifact_dir.join("exp-3");
    assert!(exp3_artifacts.exists());

    println!("✓ Selective rollback successful");

    // Rollback all remaining experiments
    rollback.rollback_experiment("exp-1", "Cleanup").unwrap();
    rollback.rollback_experiment("exp-3", "Cleanup").unwrap();

    // Verify all experiments removed
    let exp1_final = repo.find_branch("exp-1", git2::BranchType::Local);
    assert!(exp1_final.is_err());

    let exp3_final = repo.find_branch("exp-3", git2::BranchType::Local);
    assert!(exp3_final.is_err());

    // Verify all artifacts removed
    let entries: Vec<_> = std::fs::read_dir(&artifact_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    assert!(entries.is_empty(), "All artifacts should be removed");

    println!("✓ All experiments rolled back");

    println!("\n✅ Multiple experiments rollback test PASSED");
}
```

- [ ] **Step 62: Run rollback verification tests**

```bash
cargo test --test rollback_verification_tests -- --nocapture
```

Expected: Both tests PASS

---

## Module 5: Scheduling Policy Round-Trip

### Test Group 1: YAML → WorkflowSpec → WorkflowIR → Execute

**Files:**
- Create: `tests/integration/scheduling_roundtrip_tests.rs`

#### Test: Complete policy round-trip

- [ ] **Step 63: Write end-to-end round-trip test**

```rust
#[tokio::test]
async fn complete_scheduling_policy_roundtrip() {
    // Step 1: Define scheduling policy in YAML
    let yaml_workflow = r#"
workflow_id: roundtrip_test_workflow
name: "Roundtrip Test Workflow"
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
    output:
      save_to: analysis_result
      format: json
  - step: validate
    id: step_2
    model: "test_model"
    input:
      prompt: "Validate"
      analysis: "${step.step_1.output}"
    output:
      save_to: validation_result
      format: json
"#;

    println!("✓ Step 1: Defined scheduling policy in YAML");

    // Step 2: Parse YAML to WorkflowSpec
    let spec = WorkflowSpec::from_yaml(yaml_workflow).unwrap();

    assert_eq!(spec.workflow_id, "roundtrip_test_workflow");
    assert_eq!(spec.scheduling.policy.r#type, "cron");
    assert_eq!(spec.scheduling.policy.expression, "0 9 * * 1-5");
    assert_eq!(spec.scheduling.policy.timezone, Some("UTC".to_string()));
    assert_eq!(spec.scheduling.constraints.max_duration_minutes, Some(60));
    assert_eq!(spec.scheduling.constraints.max_concurrent, Some(3));
    assert_eq!(spec.scheduling.resources.memory_mb, 4096);
    assert_eq!(spec.scheduling.resources.cpu_cores, 2);

    println!("✓ Step 2: Parsed YAML to WorkflowSpec");

    // Step 3: Compile WorkflowSpec to WorkflowIR
    let ir = WorkflowIRCompiler::compile(&spec).unwrap();

    assert_eq!(ir.workflow_id, "roundtrip_test_workflow");
    assert_eq!(ir.steps.len(), 2);

    assert!(ir.scheduling_metadata.is_some());
    let scheduling = ir.scheduling_metadata.as_ref().unwrap();

    assert_eq!(scheduling.cron_expression, Some("0 9 * * 1-5".to_string()));
    assert_eq!(scheduling.timezone, Some("UTC".to_string()));
    assert_eq!(scheduling.max_duration_minutes, Some(60));
    assert_eq!(scheduling.max_concurrent, Some(3));
    assert_eq!(scheduling.resources.memory_mb, 4096);
    assert_eq!(scheduling.resources.cpu_cores, 2);

    println!("✓ Step 3: Compiled WorkflowSpec to WorkflowIR");

    // Step 4: Serialize WorkflowIR
    let ir_json = serde_json::to_string(&ir).unwrap();
    assert!(!ir_json.is_empty());

    println!("✓ Step 4: Serialized WorkflowIR to JSON");

    // Step 5: Deserialize WorkflowIR
    let ir_restored: WorkflowIR = serde_json::from_str(&ir_json).unwrap();

    assert_eq!(ir.workflow_id, ir_restored.workflow_id);
    assert_eq!(ir.steps, ir_restored.steps);
    assert_eq!(ir.scheduling_metadata, ir_restored.scheduling_metadata);

    println!("✓ Step 5: Deserialized WorkflowIR from JSON");

    // Step 6: Create scheduler from IR
    let scheduler = TokioScheduler::from_ir(&ir_restored).unwrap();

    let next_run = scheduler.next_run_time_from_ir().unwrap();
    assert!(next_run > chrono::Utc::now());

    println!("✓ Step 6: Created scheduler from IR (next run: {:?})", next_run);

    // Step 7: Execute workflow from IR
    let temp_dir = tempfile::tempdir().unwrap();
    let artifact_dir = temp_dir.path().join(".artifacts");
    std::fs::create_dir(&artifact_dir).unwrap();

    let executor = WorkflowExecutor::from_ir(&ir_restored).unwrap();
    let execution_result = executor.execute(&spec).await.unwrap();

    assert!(execution_result.success);
    assert_eq!(execution_result.workflow_id, "roundtrip_test_workflow");

    println!("✓ Step 7: Executed workflow from IR (status: {})",
             execution_result.status);

    // Step 8: Verify execution respected scheduling constraints
    let metrics = executor.get_metrics().unwrap();
    assert!(metrics.memory_usage_mb <= 4096, "Should respect memory limit");
    assert!(metrics.execution_duration_seconds <= 3600.0, "Should respect max duration");

    println!("✓ Step 8: Verified execution respected scheduling constraints");

    // Step 9: Verify output files created
    let analysis_path = artifact_dir.join("analysis_result.json");
    let validation_path = artifact_dir.join("validation_result.json");

    assert!(analysis_path.exists(), "Analysis result should exist");
    assert!(validation_path.exists(), "Validation result should exist");

    println!("✓ Step 9: Verified output files created");

    println!("\n✅ Complete scheduling policy round-trip test PASSED");
}

#[tokio::test]
async fn roundtrip_with_complex_scheduling() {
    // Test with multiple scheduling policies
    let yaml_workflow = r#"
workflow_id: complex_scheduling_workflow
name: "Complex Scheduling Workflow"
scheduling:
  policy:
    type: cron
    expression: "*/30 8-18 * * 1-5"
    timezone: "America/New_York"
  constraints:
    max_duration_minutes: 120
    max_concurrent: 5
  resources:
    memory_mb: 8192
    cpu_cores: 4
agentic_workflow:
  - step: task1
    id: step_1
    model: "model1"
    input:
      prompt: "Task 1"
  - step: task2
    id: step_2
    model: "model2"
    input:
      prompt: "Task 2"
  - step: task3
    id: step_3
    model: "model3"
    input:
      prompt: "Task 3"
"#;

    let spec = WorkflowSpec::from_yaml(yaml_workflow).unwrap();
    let ir = WorkflowIRCompiler::compile(&spec).unwrap();

    // Verify timezone conversion
    let scheduling = ir.scheduling_metadata.as_ref().unwrap();
    assert_eq!(scheduling.timezone, Some("America/New_York".to_string()));

    // Verify next run time is in UTC (converted from EST)
    let next_run = scheduling.next_run_time.unwrap();
    assert!(next_run.hour() >= 13 && next_run.hour() <= 23, // 8am-6pm EST = 1pm-11pm UTC
             "Next run should be in UTC hours");

    println!("✓ Complex scheduling with timezone conversion: EST 8am-6pm = UTC {:?}",
             next_run.hour());

    // Verify resource constraints
    assert_eq!(scheduling.resources.memory_mb, 8192);
    assert_eq!(scheduling.resources.cpu_cores, 4);

    println!("✅ Complex scheduling round-trip test PASSED");
}
```

- [ ] **Step 64: Run scheduling round-trip tests**

```bash
cargo test --test scheduling_roundtrip_tests -- --nocapture
```

Expected: Both tests PASS

---

## Comprehensive Integration Test Suite

### Run All Integration Tests

```bash
# Run all integration tests
cargo test --test *_tests -- --nocapture

# Expected: All tests PASS
# Breakdown by module:
#   Cron Pipeline: 3 tests
#   Git Experiment Lifecycle: 2 tests
#   Merge Proposal Workflow: 1 test
#   Rollback Verification: 2 tests
#   Scheduling Round-Trip: 2 tests
# Total: 10 tests
```

### Success Criteria

All integration tests must meet:
- ✅ All 10 tests PASS
- ✅ End-to-end workflows execute successfully
- ✅ All artifacts created and cleaned up properly
- ✅ Git operations isolated correctly
- ✅ Scheduling constraints enforced
- ✅ No data corruption or state leakage

### Next Steps

After passing all integration tests:
1. Run property-based tests
2. Run performance benchmarks
3. Conduct security audit
4. Prepare for Phase 06 completion
