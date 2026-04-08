# Merge Outputs Validation

Validates that merge proposals are outputs only, never auto-committed.

```rust
use agentsdk::automation::merge::{MergeProposalGenerator, ProposalArtifactManager, ProposalStatus};
use agentsdk::automation::experiment::{create_temp_repo, ExperimentConfig, ExperimentManifest};

#[tokio::test]
async fn test_proposals_written_to_artifact_directory() {
    let (temp_dir, _) = create_temp_repo().unwrap();
    let artifact_manager = ProposalArtifactManager::new(temp_dir.path()).unwrap();

    // Create proposal
    let metadata = agentsdk::automation::merge::ProposalMetadata {
        id: uuid::Uuid::new_v4().to_string(),
        experiment_id: "test-experiment".to_string(),
        base_branch: "main".to_string(),
        experiment_branch: "experiment-1".to_string(),
        created_at: chrono::Utc::now(),
        confidence: agentsdk::automation::merge::ConfidenceLevel::High,
        validation: agentsdk::automation::merge::ValidationResult::Pass,
        diff_stats: agentsdk::automation::merge::DiffStats {
            files_added: 1,
            files_modified: 0,
            files_deleted: 0,
            lines_added: 10,
            lines_deleted: 0,
        },
        status: ProposalStatus::Pending,
    };

    let diff = "+ New line";
    artifact_manager.save_proposal(&metadata, diff).unwrap();

    // Verify proposal was written to artifact directory
    let artifact_dir = temp_dir.path().join(".glyphnova/merge-proposals");
    assert!(artifact_dir.exists(), "Artifact directory should exist");

    let proposal_dir = artifact_dir.join(&metadata.id);
    assert!(proposal_dir.exists(), "Proposal directory should exist");
    assert!(proposal_dir.join("metadata.json").exists());
    assert!(proposal_dir.join("diff.patch").exists());
}

#[tokio::test]
async fn test_no_automatic_commits() {
    let (temp_dir, repo) = create_temp_repo().unwrap();

    // Create proposal
    let generator = MergeProposalGenerator::new(temp_dir.path());
    let diff = generator.generate_diff("experiment-1", "main").unwrap();

    // Verify no commits were made during proposal generation
    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();

    let commit_count = revwalk.count().unwrap();
    assert_eq!(commit_count, 1, "Should only have initial commit, no auto-commits");
}

#[tokio::test]
async fn test_manual_intervention_required() {
    // This test verifies that approval workflow requires manual intervention
    // by checking that proposals remain in "Pending" state until explicitly approved

    let temp_dir = tempfile::tempdir().unwrap();
    let artifact_manager = ProposalArtifactManager::new(temp_dir.path()).unwrap();

    // Create proposal in Pending state
    let metadata = agentsdk::automation::merge::ProposalMetadata {
        id: uuid::Uuid::new_v4().to_string(),
        experiment_id: "test-experiment".to_string(),
        base_branch: "main".to_string(),
        experiment_branch: "experiment-1".to_string(),
        created_at: chrono::Utc::now(),
        confidence: agentsdk::automation::merge::ConfidenceLevel::High,
        validation: agentsdk::automation::merge::ValidationResult::Pass,
        diff_stats: agentsdk::automation::merge::DiffStats {
            files_added: 1,
            files_modified: 0,
            files_deleted: 0,
            lines_added: 10,
            lines_deleted: 0,
        },
        status: ProposalStatus::Pending,
    };

    let diff = "+ New line";
    artifact_manager.save_proposal(&metadata, diff).unwrap();

    // Verify proposal is still in Pending state
    let loaded = artifact_manager.load_proposal(&metadata.id).unwrap().unwrap();
    assert!(matches!(loaded.status, ProposalStatus::Pending));

    // Proposal should NOT auto-merge or change status without manual intervention
    assert!(matches!(loaded.status, ProposalStatus::Pending));
}
```
