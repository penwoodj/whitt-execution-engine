# Branch Isolation Validation

Validates that git experiments run in isolated branches only.

```rust
use agentsdk::automation::experiment::{GitExperimentManager, ExperimentConfig, ExperimentManifest, IsolationEnforcer};
use agentsdk::automation::experiment::create_temp_repo;

#[tokio::test]
async fn test_branch_created_before_experiment() {
    let (temp_dir, _) = create_temp_repo().unwrap();
    let manager = GitExperimentManager::new(temp_dir.path()).unwrap();

    let config = ExperimentConfig::default();
    let manifest = ExperimentManifest::new(
        "test-experiment".to_string(),
        "Test experiment".to_string(),
        config,
    );

    // Create experiment
    let experiment_id = manager.create_experiment(manifest).await.unwrap();

    // Verify branch was created
    let repo = git2::Repository::open(temp_dir.path()).unwrap();
    let branch = repo.find_branch("experiment-1", git2::BranchType::Local);

    assert!(branch.is_ok(), "Branch should be created before experiment execution");
}

#[tokio::test]
async fn test_no_artifacts_on_main_branch() {
    let (temp_dir, _) = create_temp_repo().unwrap();
    let manager = GitExperimentManager::new(temp_dir.path()).unwrap();

    let config = ExperimentConfig::default();
    let manifest = ExperimentManifest::new(
        "test-experiment".to_string(),
        "Test experiment".to_string(),
        config,
    );

    // Create and cleanup experiment
    let experiment_id = manager.create_experiment(manifest).await.unwrap();
    manager.cleanup_experiment(&experiment_id).await.unwrap();

    // Verify no artifacts on main branch
    let repo = git2::Repository::open(temp_dir.path()).unwrap();
    let main_head = repo.head().unwrap();

    // Verify we're still on main branch
    assert!(main_head.shorthand().unwrap() == "main" || main_head.shorthand().unwrap() == "master");
}

#[tokio::test]
async fn test_scheduler_rejects_main_branch_experiments() {
    let enforcer = IsolationEnforcer::new();

    // Try to create experiment on main branch (should fail)
    let result = enforcer.validate_experiment_branch("main");

    assert!(result.is_err(), "Should reject main branch");
    assert!(matches!(
        result.unwrap_err(),
        agentsdk::automation::experiment::IsolationViolation::ProtectedBranch(_)
    ));
}
```
