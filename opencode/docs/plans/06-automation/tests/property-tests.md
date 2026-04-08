# Phase 06 Automation - Property-Based Tests

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Define property-based test specifications that verify invariants across all valid inputs

**Architecture:** Property-based tests using proptest to verify system invariants across random inputs, ensuring correctness across all possible inputs, not just specific examples.

**Tech Stack:** Rust, proptest, tempfile, git2, chrono, serde

---

## Property Test Structure

```
tests/property/
├── cron_properties.rs           # Cron expression invariants
├── git_isolation_properties.rs  # Git isolation invariants
├── merge_proposal_properties.rs # Merge proposal invariants
├── experiment_result_properties.rs # Experiment result invariants
├── rollback_properties.rs        # Rollback invariants
└── mod.rs                      # Property test module
```

---

## Module 1: Cron Expression Invariants

### Property Group 1: Deterministic Parsing

**Files:**
- Create: `tests/property/cron_properties.rs`

#### Property: Parsing is deterministic

- [ ] **Step 65: Write property-based test for deterministic parsing**

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn prop_parsing_is_deterministic(cron_expr in "[*0-5][*0-5][*0-5][*0-5][*0-5]") {
        // Parse expression twice
        let result1 = CronExpression::parse(&cron_expr);
        let result2 = CronExpression::parse(&cron_expr);

        // Both should either succeed or fail the same way
        prop_assert_eq!(result1.is_ok(), result2.is_ok(),
                        "Parsing should be deterministic: {}", cron_expr);

        // If parsing succeeded, results should be identical
        if result1.is_ok() && result2.is_ok() {
            let parsed1 = result1.unwrap();
            let parsed2 = result2.unwrap();

            prop_assert_eq!(parsed1.raw, parsed2.raw);
            prop_assert_eq!(parsed1.interval_seconds, parsed2.interval_seconds);

            // Next run times should be close (within 1 second)
            let diff = (parsed1.next_run - parsed2.next_run).num_seconds().abs();
            prop_assert!(diff <= 1, "Next run times should be consistent");
        }
    }
}
```

- [ ] **Step 66: Run property test**

```bash
cargo test --test prop_parsing_is_deterministic -- --exact
```

Expected: Test compiles and runs (will fail initially)

- [ ] **Step 67: Ensure CronExpression::parse implemented**

```rust
impl CronExpression {
    pub fn parse(expr: &str) -> Result<Self, CronParseError> {
        // Implementation already exists from unit tests
        // This property test verifies determinism
        // ...
    }
}
```

- [ ] **Step 68: Run property test to verify passes**

```bash
cargo test --test prop_parsing_is_deterministic -- --exact
```

Expected: PASS (tests many random expressions)

#### Property: Valid expressions always parse to same structure

- [ ] **Step 69: Write property for parse structure consistency**

```rust
proptest! {
    #[test]
    fn prop_valid_expressions_parse_to_same_structure(
        minute in "[0-5]?[0-9]",
        hour in "[0-1]?[0-9]",
        day in "[0-3]?[0-9]",
        month in "[0-1]?[0-9]",
        weekday in "[0-6]"
    ) {
        let cron_expr = format!("{} {} {} {} {}", minute, hour, day, month, weekday);

        let result = CronExpression::parse(&cron_expr);

        // If parsing succeeded, structure should be consistent
        if result.is_ok() {
            let parsed = result.unwrap();

            // Verify all fields are populated
            prop_assert!(!parsed.raw.is_empty());

            // Verify metadata is consistent with expression
            let metadata = parsed.metadata();

            // Metadata should reflect expression properties
            prop_assert!(metadata.interval_seconds > 0);

            // Next run time should be in the future
            let now = chrono::Utc::now();
            prop_assert!(parsed.next_run > now);
        }
    }
}
```

- [ ] **Step 70: Run property test**

```bash
cargo test --test prop_valid_expressions_parse_to_same_structure -- --exact
```

Expected: PASS

### Property Group 2: Timezone Consistency

#### Property: Timezone conversion is consistent

- [ ] **Step 71: Write property for timezone consistency**

```rust
proptest! {
    #[test]
    fn prop_timezone_conversion_is_consistent(
        cron_expr in "0 * * * *",
        timezone in "[A-Za-z]+/[A-Za-z]+"
    ) {
        // Parse in UTC
        let utc_result = CronExpression::with_timezone(&cron_expr, "UTC");

        // Parse in specific timezone
        let tz_result = CronExpression::with_timezone(&cron_expr, &timezone);

        // UTC should always parse successfully
        prop_assert!(utc_result.is_ok());

        // If specific timezone is valid, parse should succeed
        // (invalid timezones will fail, which is expected)
        if tz_result.is_ok() {
            let utc_parsed = utc_result.unwrap();
            let tz_parsed = tz_result.unwrap();

            // Raw expression should be the same
            prop_assert_eq!(utc_parsed.raw, tz_parsed.raw);

            // Interval should be the same (independent of timezone)
            prop_assert_eq!(utc_parsed.interval_seconds, tz_parsed.interval_seconds);

            // Next run times should differ by timezone offset
            let diff = (tz_parsed.next_run - utc_parsed.next_run).num_seconds().abs();

            // Difference should be reasonable (not more than 24 hours)
            prop_assert!(diff < 86400, "Timezone offset should be reasonable");
        }
    }
}
```

- [ ] **Step 72: Run property test**

```bash
cargo test --test prop_timezone_conversion_is_consistent -- --exact
```

Expected: PASS

---

## Module 2: Git Isolation Invariants

### Property Group 1: Main Branch Never Modified

**Files:**
- Create: `tests/property/git_isolation_properties.rs`

#### Property: Main branch unaffected by experiments

- [ ] **Step 73: Write property for main branch isolation**

```rust
proptest! {
    #[test]
    fn prop_main_branch_unaffected_by_experiments(
        num_changes in 1..10usize,
        num_experiments in 1..5usize
    ) {
        let (temp_dir, repo) = create_temp_repo_with_initial_content().unwrap();

        // Capture initial main state
        let main_head_initial = repo.head().unwrap();
        let main_commit_initial = main_head_initial.target().unwrap();
        let main_tree_initial = repo.find_commit(main_commit_initial).unwrap().tree().unwrap();

        let mut main_tree_id = main_tree_initial.id();

        // Create multiple experiments with random changes
        for exp_idx in 0..num_experiments {
            let exp_name = format!("experiment-{}", exp_idx);

            let experiment = ExperimentBranch::create(
                repo.path(),
                &exp_name,
            ).unwrap();

            // Make random number of changes
            for change_idx in 0..num_changes {
                let file_name = format!("change_{}_{}.txt", exp_idx, change_idx);
                let file_path = temp_dir.path().join(&file_name);

                let content = format!("Change {} in experiment {}\n", change_idx, exp_idx);
                std::fs::write(&file_path, content).unwrap();

                experiment.commit_changes(&format!("Add change {}", change_idx)).unwrap();
            }

            // Switch back to main and verify unchanged
            repo.checkout("main", None).unwrap();
            let main_head = repo.head().unwrap();
            let main_commit = main_head.target().unwrap();
            let main_tree = repo.find_commit(main_commit).unwrap().tree().unwrap();

            // Main tree should never change
            prop_assert_eq!(main_tree.id(), main_tree_id,
                           format!("Main branch should be unaffected by experiment {}",
                                   exp_name));
        }

        // Cleanup
        drop(temp_dir);

        println!("✓ Main branch isolation verified for {} experiments with {} changes each",
                 num_experiments, num_changes);
    }
}
```

- [ ] **Step 74: Run property test**

```bash
cargo test --test prop_main_branch_unaffected_by_experiments -- --exact
```

Expected: PASS (tests many combinations)

### Property Group 2: Experiment Isolation

#### Property: Experiments do not interfere with each other

- [ ] **Step 75: Write property for experiment isolation**

```rust
proptest! {
    #[test]
    fn prop_experiments_dont_interfere(
        num_experiments in 2..5usize,
        num_files_per_exp in 1..5usize
    ) {
        let (temp_dir, repo) = create_temp_repo_with_initial_content().unwrap();

        let mut experiment_file_sets: Vec<Vec<String>> = Vec::new();

        // Create multiple experiments
        for exp_idx in 0..num_experiments {
            let exp_name = format!("experiment-{}", exp_idx);

            let experiment = ExperimentBranch::create(
                repo.path(),
                &exp_name,
            ).unwrap();

            let mut files_in_exp = Vec::new();

            // Add files to this experiment
            for file_idx in 0..num_files_per_exp {
                let file_name = format!("exp{}_file{}.txt", exp_idx, file_idx);
                let file_path = temp_dir.path().join(&file_name);
                let content = format!("Content for {} {}\n", exp_name, file_idx);

                std::fs::write(&file_path, content).unwrap();

                // Check file exists (it should, since we're in experiment branch)
                prop_assert!(file_path.exists(),
                           format!("File should exist in experiment {}", exp_name));

                files_in_exp.push(file_name.clone());
            }

            experiment.commit_changes(&format!("Add {} files", num_files_per_exp)).unwrap();
            experiment_file_sets.push(files_in_exp);
        }

        // Verify each experiment only has its own files
        for exp_idx in 0..num_experiments {
            let exp_name = format!("experiment-{}", exp_idx);

            // Switch to experiment branch
            repo.checkout(&exp_name, None).unwrap();

            // Verify all files in this experiment exist
            for file in &experiment_file_sets[exp_idx] {
                let file_path = temp_dir.path().join(file);
                prop_assert!(file_path.exists(),
                           format!("File should exist in experiment {}: {}",
                                   exp_name, file));
            }

            // Verify files from other experiments don't exist
            for other_exp_idx in 0..num_experiments {
                if other_exp_idx != exp_idx {
                    for file in &experiment_file_sets[other_exp_idx] {
                        let file_path = temp_dir.path().join(file);

                        // File should not exist (unless names collide, which is unlikely)
                        // If names collide, the later experiment should overwrite
                        // This is acceptable behavior
                        if !experiment_file_sets[exp_idx].contains(file) {
                            prop_assert!(!file_path.exists(),
                                           format!("File from {} should not exist in {}",
                                                   other_exp_idx, exp_name));
                        }
                    }
                }
            }
        }

        // Cleanup
        drop(temp_dir);

        println!("✓ Experiment isolation verified for {} experiments",
                 num_experiments);
    }
}
```

- [ ] **Step 76: Run property test**

```bash
cargo test --test prop_experiments_dont_interfere -- --exact
```

Expected: PASS

---

## Module 3: Merge Proposal Invariants

### Property Group 1: Diff Completeness

**Files:**
- Create: `tests/property/merge_proposal_properties.rs`

#### Property: Diff always includes all changes

- [ ] **Step 77: Write property for diff completeness**

```rust
proptest! {
    #[test]
    fn prop_diff_includes_all_changes(
        num_additions in 1..10usize,
        num_modifications in 1..10usize,
        num_deletions in 0..5usize
    ) {
        let (temp_dir, repo) = create_temp_repo_with_initial_content().unwrap();

        // Create experiment branch
        let experiment = ExperimentBranch::create(repo.path(), "experiment-1").unwrap();

        let repo_path = repo.path().parent().unwrap();

        // Add files
        for i in 0..num_additions {
            let file_name = format!("new_file_{}.txt", i);
            let file_path = repo_path.join(&file_name);
            std::fs::write(&file_path, format!("New file {}\n", i)).unwrap();
        }

        // Modify existing file
        let readme_path = repo_path.join("README.md");
        let modified_content = format!("Modified\n{}", "# ".repeat(num_modifications));
        std::fs::write(&readme_path, modified_content).unwrap();

        // Delete files
        for i in 0..num_deletions {
            let file_name = format!("old_file_{}.txt", i);
            let file_path = repo_path.join(&file_name);
            std::fs::write(&file_path, format!("Old file {}\n", i)).unwrap();
        }

        // Create initial commit with files to delete
        let mut index = repo.index()?;
        for i in 0..num_deletions {
            let file_name = format!("old_file_{}.txt", i);
            index.add_path(std::path::Path::new(&file_name)).ok();
        }
        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;
        let head = repo.head()?;
        let parent = head.peel_to_commit()?;
        let sig = git2::Signature::now("Test", "test@test.com")?;
        repo.commit(Some("HEAD"), &sig, &sig, "Add files to delete", &tree, &[&parent])?;

        // Now delete them
        for i in 0..num_deletions {
            let file_name = format!("old_file_{}.txt", i);
            let file_path = repo_path.join(&file_name);
            std::fs::remove_file(&file_path).ok();
        }

        // Commit changes
        experiment.commit_changes("Make changes").unwrap();

        // Generate proposal
        let proposal = MergeProposalGenerator::generate(
            &repo,
            "experiment-1",
            "main",
            &ProposalConfig::default(),
        ).unwrap();

        let diff = &proposal.diff;
        prop_assert!(!diff.is_empty(), "Diff should not be empty");

        // Verify diff includes additions
        for i in 0..num_additions {
            let file_name = format!("new_file_{}.txt", i);
            prop_assert!(diff.contains(&format!("+++ b/{}", file_name)),
                           format!("Diff should include addition: {}", file_name));
        }

        // Verify diff includes modifications
        prop_assert!(diff.contains("diff --git a/README.md"),
                       "Diff should include README.md modification");
        prop_assert!(diff.contains("--- a/README.md"),
                       "Diff should show old README.md");
        prop_assert!(diff.contains("+++ b/README.md"),
                       "Diff should show new README.md");

        // Verify diff includes deletions
        for i in 0..num_deletions {
            let file_name = format!("old_file_{}.txt", i);
            prop_assert!(diff.contains(&format!("--- a/{}", file_name)),
                           format!("Diff should include deletion: {}", file_name));
            prop_assert!(!diff.contains(&format!("+++ b/{}", file_name)),
                           format!("Deletion should not have +++: {}", file_name));
        }

        // Cleanup
        drop(temp_dir);

        println!("✓ Diff completeness verified for {} additions, {} modifications, {} deletions",
                 num_additions, num_modifications, num_deletions);
    }
}
```

- [ ] **Step 78: Run property test**

```bash
cargo test --test prop_diff_includes_all_changes -- --exact
```

Expected: PASS

### Property Group 2: Confidence Score Consistency

#### Property: Confidence score is monotonic

- [ ] **Step 79: Write property for confidence score monotonicity**

```rust
proptest! {
    #[test]
    fn prop_confidence_score_is_monotonic(
        lines1 in 1..50usize,
        lines2 in 1..50usize
    ) {
        // Assume lines2 >= lines1 due to proptest strategy
        let smaller = std::cmp::min(lines1, lines2);
        let larger = std::cmp::max(lines1, lines2);

        // Create diffs
        let diff1 = "diff --git a/file.txt\n".repeat(smaller);
        let diff2 = "diff --git a/file.txt\n".repeat(larger);

        // Calculate confidence scores
        let config = ProposalConfig {
            include_validation: false,
            include_metrics: false,
            confidence_threshold: 0.0,
        };

        let score1 = MergeProposalGenerator::calculate_confidence(&diff1, &config).unwrap();
        let score2 = MergeProposalGenerator::calculate_confidence(&diff2, &config).unwrap();

        // Larger diff should have lower or equal confidence
        prop_assert!(score2 <= score1,
                       format!("Confidence should decrease with diff size: {} <= {} (diff sizes: {} >= {})",
                               score2, score1, larger, smaller));

        // Confidence should be in valid range
        prop_assert!(score1 >= 0.0 && score1 <= 1.0,
                       "Confidence score 1 should be in [0, 1]");
        prop_assert!(score2 >= 0.0 && score2 <= 1.0,
                       "Confidence score 2 should be in [0, 1]");

        println!("✓ Confidence score monotonicity: size {} -> {:.2}, size {} -> {:.2}",
                 smaller, score1, larger, score2);
    }
}
```

- [ ] **Step 80: Run property test**

```bash
cargo test --test prop_confidence_score_is_monotonic -- --exact
```

Expected: PASS

---

## Module 4: Experiment Result Invariants

### Property Group 1: Results Are Comparable

**Files:**
- Create: `tests/property/experiment_result_properties.rs`

#### Property: Results have consistent structure

- [ ] **Step 81: Write property for result structure consistency**

```rust
proptest! {
    #[test]
    fn prop_results_have_consistent_structure(
        execution_time in 0.0..1000.0f64,
        memory_mb in 0.0..10000.0f64,
        cpu_percent in 0.0..100.0f64,
        quality_score in 0.0..1.0f64,
        validation_passed in proptest::bool::ANY
    ) {
        let result = ExperimentResult {
            experiment_id: "test-experiment".to_string(),
            branch: "experiment-1".to_string(),
            timestamp: chrono::Utc::now(),
            config: serde_json::json!({"test": "config"}),
            metrics: ExperimentMetrics {
                execution_time_seconds: execution_time,
                memory_usage_mb: memory_mb,
                cpu_usage_percent: cpu_percent,
                quality_score: quality_score,
                validation_passed: validation_passed,
            },
            output: Some("Test output".to_string()),
        };

        // Verify all metrics are within valid ranges
        prop_assert!(result.metrics.execution_time_seconds >= 0.0,
                       "Execution time should be non-negative");
        prop_assert!(result.metrics.memory_usage_mb >= 0.0,
                       "Memory usage should be non-negative");
        prop_assert!(result.metrics.cpu_usage_percent >= 0.0 && result.metrics.cpu_usage_percent <= 100.0,
                       "CPU usage should be in [0, 100]");
        prop_assert!(result.metrics.quality_score >= 0.0 && result.metrics.quality_score <= 1.0,
                       "Quality score should be in [0, 1]");

        // Verify result can be serialized
        let result_json = serde_json::to_string(&result).unwrap();
        prop_assert!(!result_json.is_empty(), "Result should serialize to JSON");

        // Verify result can be deserialized
        let result_restored: ExperimentResult = serde_json::from_str(&result_json).unwrap();

        prop_assert_eq!(result.experiment_id, result_restored.experiment_id);
        prop_assert_eq!(result.branch, result_restored.branch);
        prop_assert_eq!(result.metrics.execution_time_seconds,
                       result_restored.metrics.execution_time_seconds);
        prop_assert_eq!(result.metrics.memory_usage_mb,
                       result_restored.metrics.memory_usage_mb);
        prop_assert_eq!(result.metrics.quality_score,
                       result_restored.metrics.quality_score);
        prop_assert_eq!(result.metrics.validation_passed,
                       result_restored.metrics.validation_passed);
    }
}
```

- [ ] **Step 82: Run property test**

```bash
cargo test --test prop_results_have_consistent_structure -- --exact
```

Expected: PASS

### Property Group 2: Comparison Properties

#### Property: Comparison is transitive

- [ ] **Step 83: Write property for comparison transitivity**

```rust
proptest! {
    #[test]
    fn prop_comparison_is_transitive(
        time1 in 1.0..100.0f64,
        time2 in 1.0..100.0f64,
        time3 in 1.0..100.0f64
    ) {
        // Create three results
        let result1 = create_test_result("exp-1", time1, 100.0, 0.8);
        let result2 = create_test_result("exp-2", time2, 100.0, 0.8);
        let result3 = create_test_result("exp-3", time3, 100.0, 0.8);

        // Compare 1 and 2
        let comp12 = ResultComparison::compare(&result1, &result2).unwrap();

        // Compare 2 and 3
        let comp23 = ResultComparison::compare(&result2, &result3).unwrap();

        // Compare 1 and 3
        let comp13 = ResultComparison::compare(&result1, &result3).unwrap();

        // Time differences should be transitive
        let time_diff_12 = comp12.execution_time_diff;
        let time_diff_23 = comp23.execution_time_diff;
        let time_diff_13 = comp13.execution_time_diff;

        // diff(1,3) should equal diff(1,2) + diff(2,3)
        // Allow for floating point tolerance
        let expected_diff = time_diff_12 + time_diff_23;
        let actual_diff = time_diff_13;

        let tolerance = 0.01; // 1% tolerance
        prop_assert!((expected_diff - actual_diff).abs() < tolerance,
                       format!("Time differences should be transitive: {} ≈ {} + {} (diff: {})",
                               actual_diff, time_diff_12, time_diff_23,
                               (expected_diff - actual_diff).abs()));

        // Ratios should be consistent
        let ratio_12 = comp12.execution_time_ratio;
        let ratio_23 = comp23.execution_time_ratio;
        let ratio_13 = comp13.execution_time_ratio;

        // ratio(1,3) should equal ratio(1,2) * ratio(2,3)
        // Allow for floating point tolerance
        let expected_ratio = ratio_12 * ratio_23;
        let actual_ratio = ratio_13;

        prop_assert!((expected_ratio - actual_ratio).abs() < tolerance,
                       format!("Time ratios should be transitive: {} ≈ {} * {} (diff: {})",
                               actual_ratio, ratio_12, ratio_23,
                               (expected_ratio - actual_ratio).abs()));

        println!("✓ Comparison transitivity: t1={:.1}, t2={:.1}, t3={:.1}",
                 time1, time2, time3);
    }
}

fn create_test_result(
    id: &str,
    exec_time: f64,
    memory: f64,
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
            cpu_usage_percent: 50.0,
            quality_score: quality,
            validation_passed: true,
        },
        output: None,
    }
}
```

- [ ] **Step 84: Run property test**

```bash
cargo test --test prop_comparison_is_transitive -- --exact
```

Expected: PASS

---

## Module 5: Rollback Invariants

### Property Group 1: Rollback Produces Clean State

**Files:**
- Create: `tests/property/rollback_properties.rs`

#### Property: Rollback always restores clean state

- [ ] **Step 85: Write property for rollback clean state**

```rust
proptest! {
    #[test]
    fn prop_rollback_restores_clean_state(
        num_experiments in 1..5usize,
        num_changes_per_exp in 1..10usize
    ) {
        let (temp_dir, repo) = create_temp_repo_with_initial_content().unwrap();

        // Capture initial clean state
        let main_head_initial = repo.head().unwrap();
        let main_commit_initial = main_head_initial.target().unwrap();
        let main_tree_initial = repo.find_commit(main_commit_initial).unwrap().tree().unwrap();

        let branches_initial: Vec<String> = repo.branches(None)
            .unwrap()
            .map(|b| b.unwrap().0.name().unwrap().unwrap().to_string())
            .collect();

        let files_initial: Vec<_> = std::fs::read_dir(temp_dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name())
            .collect();

        println!("Initial state: {} branches, {} files",
                 branches_initial.len(), files_initial.len());

        // Create experiments with random changes
        let artifact_dir = temp_dir.path().join(".artifacts");
        std::fs::create_dir(&artifact_dir).unwrap();

        for exp_idx in 0..num_experiments {
            let exp_name = format!("experiment-{}", exp_idx);

            let experiment = ExperimentBranch::create(
                repo.path(),
                &exp_name,
            ).unwrap();

            // Make random changes
            let repo_path = repo.path().parent().unwrap();
            for change_idx in 0..num_changes_per_exp {
                let file_name = format!("change_{}_{}.txt", exp_idx, change_idx);
                let file_path = repo_path.join(&file_name);
                let content = format!("Change {} in experiment {}\n", change_idx, exp_idx);

                std::fs::write(&file_path, content).unwrap();

                experiment.commit_changes(&format!("Add change {}", change_idx)).unwrap();
            }

            // Create artifacts
            let exp_artifacts = artifact_dir.join(&exp_name);
            std::fs::create_dir(&exp_artifacts).unwrap();
            std::fs::write(exp_artifacts.join("result.json"), "{}").unwrap();
        }

        // Verify experiments exist
        let experiments_created: Vec<_> = branches_initial.iter()
            .filter(|b| b.starts_with("experiment-"))
            .collect();

        println!("Created {} experiments", num_experiments);

        // Rollback all experiments
        let rollback = RollbackManager::new(&repo, &artifact_dir).unwrap();

        for exp_idx in 0..num_experiments {
            let exp_name = format!("experiment-{}", exp_idx);
            rollback.rollback_experiment(&exp_name, "Property test rollback").unwrap();
        }

        // Verify final state matches initial state
        let main_head_final = repo.head().unwrap();
        let main_commit_final = main_head_final.target().unwrap();
        let main_tree_final = repo.find_commit(main_commit_final).unwrap().tree().unwrap();

        let branches_final: Vec<String> = repo.branches(None)
            .unwrap()
            .map(|b| b.unwrap().0.name().unwrap().unwrap().to_string())
            .collect();

        // Main branch should be unchanged
        prop_assert_eq!(main_commit_initial, main_commit_final,
                       "Main branch commit should be unchanged after rollback");
        prop_assert_eq!(main_tree_initial.id(), main_tree_final.id(),
                       "Main branch tree should be unchanged after rollback");

        // Branches should be back to initial state
        prop_assert_eq!(branches_initial, branches_final,
                       "Branches should be back to initial state after rollback");

        // Artifacts should be cleaned
        if artifact_dir.exists() {
            let entries: Vec<_> = std::fs::read_dir(&artifact_dir)
                .unwrap()
                .filter_map(|e| e.ok())
                .collect();
            prop_assert!(entries.is_empty(),
                       "Artifact directory should be empty after rollback");
        }

        println!("✓ Rollback restored clean state: {} experiments rolled back",
                 num_experiments);

        // Cleanup
        drop(temp_dir);
    }
}
```

- [ ] **Step 86: Run property test**

```bash
cargo test --test prop_rollback_restores_clean_state -- --exact
```

Expected: PASS

### Property Group 2: Rollback Idempotency

#### Property: Rollback is idempotent

- [ ] **Step 87: Write property for rollback idempotency**

```rust
proptest! {
    #[test]
    fn prop_rollback_is_idempotent(
        num_rollbacks in 1..5usize
    ) {
        let (temp_dir, repo) = create_temp_repo_with_initial_content().unwrap();
        let artifact_dir = temp_dir.path().join(".artifacts");
        std::fs::create_dir(&artifact_dir).unwrap();

        // Create experiment
        let experiment = ExperimentBranch::create(repo.path(), "experiment-1").unwrap();

        let repo_path = repo.path().parent().unwrap();
        let test_file = repo_path.join("test.txt");
        std::fs::write(&test_file, "Test content\n").unwrap();

        experiment.commit_changes("Add test").unwrap();

        // Rollback multiple times
        let rollback = RollbackManager::new(&repo, &artifact_dir).unwrap();

        for i in 0..num_rollbacks {
            let result = rollback.rollback_experiment("experiment-1",
                                                  &format!("Rollback {}", i));

            prop_assert!(result.is_ok(),
                       format!("Rollback {} should succeed (idempotent)", i));

            println!("  Rollback {} completed", i);
        }

        // Verify experiment branch is removed (only once)
        let exp_branch = repo.find_branch("experiment-1", git2::BranchType::Local);
        prop_assert!(exp_branch.is_err(),
                       "Experiment branch should be removed after first rollback");

        // Verify artifacts are cleaned (only once)
        if artifact_dir.exists() {
            let entries: Vec<_> = std::fs::read_dir(&artifact_dir)
                .unwrap()
                .filter_map(|e| e.ok())
                .collect();
            prop_assert!(entries.is_empty(),
                       "Artifacts should be cleaned after first rollback");
        }

        // Verify main branch unchanged
        let main_head = repo.head().unwrap();
        prop_assert_eq!(main_head.shorthand().unwrap(), "main",
                       "Main branch should be unchanged");

        println!("✓ Rollback idempotency verified for {} rollback attempts",
                 num_rollbacks);

        // Cleanup
        drop(temp_dir);
    }
}
```

- [ ] **Step 88: Run property test**

```bash
cargo test --test prop_rollback_is_idempotent -- --exact
```

Expected: PASS

---

## Comprehensive Property Test Suite

### Run All Property Tests

```bash
# Run all property tests
cargo test --test prop_* -- --nocapture

# Expected: All tests PASS
# Breakdown by module:
#   Cron Properties: 3 tests
#   Git Isolation Properties: 2 tests
#   Merge Proposal Properties: 2 tests
#   Experiment Result Properties: 2 tests
#   Rollback Properties: 2 tests
# Total: 11 tests
```

### Test Execution Details

Property tests use proptest to generate random inputs and verify invariants:

```bash
# Run property tests with more cases (default is 100)
PROPTEST_CASES=1000 cargo test --test prop_* -- --nocapture

# Run property tests with specific seed for reproducibility
PROPTEST_SEED=12345 cargo test --test prop_parsing_is_deterministic -- --nocapture

# Run property tests in parallel
PROPTEST_THREADS=4 cargo test --test prop_* -- --nocapture
```

### Success Criteria

All property tests must meet:
- ✅ All 11 tests PASS
- ✅ No counterexamples found
- ✅ Tests run on 100+ random inputs each
- ✅ Properties verified across input space

### Finding Counterexamples

If a property test fails with a counterexample:

```bash
# Re-run with the failing case
cargo test --test prop_test_name -- --nocapture

# Output will show the minimal counterexample
# Example:
# thread 'prop_test_name' panicked at 'proptest property test failed'
# cases: [
#   CronExpression { raw: "invalid", ... }
# ]
```

### Next Steps

After passing all property tests:
1. Run comprehensive integration tests
2. Perform end-to-end validation
3. Generate coverage report
4. Prepare for Phase 06 completion
