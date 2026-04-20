# Task 01: Git Experiment Framework

**Component:** Git experiment management with branch isolation and worktree support

**Dependencies:** None (foundational task)

**Estimated Time:** 8-10 days

**Goal:** Build a framework for running git experiments in isolated branches/worktrees with configurable merge policies, complete cleanup, and strict adherence to ADR-0007 branch isolation requirements.

---

## Overview

The git experiment framework enables safe parallel experimentation by:

- Creating isolated git branches for each experiment
- Managing git worktrees for parallel experiments
- Enforcing merge policies (auto-merge, require-approval, block)
- Preventing experiments on main branch
- Providing complete cleanup procedures
- Tracking experiment metadata

**ADR-0007 Compliance:**
- Experiments MUST run in isolated branches (never main)
- Worktree management ensures complete isolation
- Merge proposals are outputs only (auto-commits forbidden)
- Failed experiments are fully cleaned up

---

## File Structure

**New Files:**
- `automation/experiment/mod.rs` - Module exports
- `automation/experiment/manifest.rs` - Experiment manifest types
- `automation/experiment/git_operations.rs` - Git worktree management
- `automation/experiment/merge_policy.rs` - Merge policy types and logic
- `automation/experiment/isolation.rs` - Branch/worktree isolation enforcement

---

## Implementation Steps

### Step 1: Add git2 dependency

**Files:** Modify `Cargo.toml`

```toml
[dependencies]
# Existing dependencies...

# Git operations
git2 = "0.18"
tempfile = "3.8"
```

---

### Step 2: Create experiment module structure

**Files:** Create `automation/experiment/mod.rs`

```rust
pub mod manifest;
pub mod git_operations;
pub mod merge_policy;
pub mod isolation;

pub use manifest::{
    ExperimentManifest, ExperimentConfig, ExperimentStatus,
    ExperimentMetadata, ExperimentResult,
};
pub use git_operations::{
    GitExperimentManager, WorktreeInfo, GitExperimentError,
    create_temp_repo,
};
pub use merge_policy::{MergePolicy, MergePolicyType, MergeDecision};
pub use isolation::{IsolationEnforcer, IsolationViolation};

use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Main experiment manager instance
pub type ExperimentManager = Arc<GitExperimentManager>;

/// Experiment configuration
#[derive(Debug, Clone)]
pub struct ExperimentConfig {
    pub base_branch: String,
    pub experiment_branch: String,
    pub merge_policy: MergePolicyType,
    pub cleanup_on_failure: bool,
    pub keep_artifacts: bool,
}

impl Default for ExperimentConfig {
    fn default() -> Self {
        Self {
            base_branch: "main".to_string(),
            experiment_branch: "experiment".to_string(),
            merge_policy: MergePolicyType::RequireApproval,
            cleanup_on_failure: true,
            keep_artifacts: false,
        }
    }
}
```

---

### Step 3: Write failing tests for experiment manifest

**Files:** Create `automation/experiment/manifest.rs` (with tests)

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExperimentStatus {
    Created,
    Running,
    Completed,
    Failed(String),
    Merged,
    Rejected,
    RolledBack,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentManifest {
    pub id: String,
    pub name: String,
    pub description: String,
    pub config: ExperimentConfig,
    pub status: ExperimentStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub metadata: ExperimentMetadata,
    pub results: Option<ExperimentResult>,
    pub artifacts: HashMap<String, PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentConfig {
    pub base_branch: String,
    pub experiment_branch: String,
    pub merge_policy: MergePolicyType,
    pub cleanup_on_failure: bool,
    pub keep_artifacts: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentMetadata {
    pub author: String,
    pub tags: Vec<String>,
    pub environment: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentResult {
    pub exit_code: i32,
    pub output: String,
    pub error: Option<String>,
    pub duration_ms: u64,
    pub metrics: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergePolicyType {
    AutoMerge,
    RequireApproval,
    Block,
}

impl ExperimentManifest {
    pub fn new(name: String, description: String, config: ExperimentConfig) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            config,
            status: ExperimentStatus::Created,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            metadata: ExperimentMetadata {
                author: "system".to_string(),
                tags: Vec::new(),
                environment: HashMap::new(),
            },
            results: None,
            artifacts: HashMap::new(),
        }
    }

    pub fn start(&mut self) {
        self.status = ExperimentStatus::Running;
        self.started_at = Some(Utc::now());
    }

    pub fn complete(&mut self, result: ExperimentResult) {
        self.status = ExperimentStatus::Completed;
        self.completed_at = Some(Utc::now());
        self.results = Some(result);
    }

    pub fn fail(&mut self, error: String) {
        self.status = ExperimentStatus::Failed(error);
        self.completed_at = Some(Utc::now());
    }

    pub fn mark_merged(&mut self) {
        self.status = ExperimentStatus::Merged;
        self.completed_at = Some(self.completed_at.unwrap_or_else(Utc::now));
    }

    pub fn mark_rejected(&mut self) {
        self.status = ExperimentStatus::Rejected;
        self.completed_at = Some(self.completed_at.unwrap_or_else(Utc::now));
    }

    pub fn mark_rolled_back(&mut self) {
        self.status = ExperimentStatus::RolledBack;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_experiment_manifest() {
        let config = ExperimentConfig::default();
        let manifest = ExperimentManifest::new(
            "test-experiment".to_string(),
            "Test experiment".to_string(),
            config,
        );

        assert_eq!(manifest.name, "test-experiment");
        assert_eq!(manifest.description, "Test experiment");
        assert!(matches!(manifest.status, ExperimentStatus::Created));
        assert!(manifest.started_at.is_none());
        assert!(manifest.completed_at.is_none());
    }

    #[test]
    fn test_start_experiment() {
        let config = ExperimentConfig::default();
        let mut manifest = ExperimentManifest::new(
            "test-experiment".to_string(),
            "Test experiment".to_string(),
            config,
        );

        manifest.start();

        assert!(matches!(manifest.status, ExperimentStatus::Running));
        assert!(manifest.started_at.is_some());
    }

    #[test]
    fn test_complete_experiment() {
        let config = ExperimentConfig::default();
        let mut manifest = ExperimentManifest::new(
            "test-experiment".to_string(),
            "Test experiment".to_string(),
            config,
        );

        let result = ExperimentResult {
            exit_code: 0,
            output: "Success".to_string(),
            error: None,
            duration_ms: 1000,
            metrics: HashMap::new(),
        };

        manifest.start();
        manifest.complete(result);

        assert!(matches!(manifest.status, ExperimentStatus::Completed));
        assert!(manifest.completed_at.is_some());
        assert!(manifest.results.is_some());
    }

    #[test]
    fn test_fail_experiment() {
        let config = ExperimentConfig::default();
        let mut manifest = ExperimentManifest::new(
            "test-experiment".to_string(),
            "Test experiment".to_string(),
            config,
        );

        manifest.fail("Test error".to_string());

        assert!(matches!(manifest.status, ExperimentStatus::Failed(_)));
        assert!(manifest.completed_at.is_some());
    }
}
```

---

### Step 4: Run manifest tests to verify they pass

**Files:** Run tests

```bash
cargo test automation::experiment::manifest --lib
```

**Expected:** PASS all tests

---

### Step 5: Write failing tests for merge policy

**Files:** Create `automation/experiment/merge_policy.rs` (with tests)

```rust
use super::manifest::{ExperimentManifest, ExperimentResult, MergePolicyType};
use crate::automation::experiment::git_operations::GitExperimentError;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeDecision {
    AutoMerge,
    RequireApproval,
    Block,
}

#[derive(Debug, Clone)]
pub struct MergePolicy {
    pub policy_type: MergePolicyType,
    pub min_success_rate: f64,
    pub max_failure_rate: f64,
    pub require_test_passage: bool,
}

impl Default for MergePolicy {
    fn default() -> Self {
        Self {
            policy_type: MergePolicyType::RequireApproval,
            min_success_rate: 0.95,
            max_failure_rate: 0.05,
            require_test_passage: true,
        }
    }
}

impl MergePolicy {
    pub fn evaluate(&self, manifest: &ExperimentManifest) -> Result<MergeDecision, MergePolicyError> {
        // Implementation will be added in Step 7
        todo!()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MergePolicyError {
    #[error("Experiment has not completed")]
    NotCompleted,
    #[error("Experiment failed: {0}")]
    ExperimentFailed(String),
    #[error("Success rate {0} below threshold {1}")]
    SuccessRateTooLow(f64, f64),
    #[error("Failure rate {0} above threshold {1}")]
    FailureRateTooHigh(f64, f64),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_completed_manifest(success: bool) -> ExperimentManifest {
        let config = ExperimentConfig::default();
        let mut manifest = ExperimentManifest::new(
            "test-experiment".to_string(),
            "Test experiment".to_string(),
            config,
        );

        let result = if success {
            ExperimentResult {
                exit_code: 0,
                output: "Success".to_string(),
                error: None,
                duration_ms: 1000,
                metrics: HashMap::new(),
            }
        } else {
            ExperimentResult {
                exit_code: 1,
                output: "Error".to_string(),
                error: Some("Failed".to_string()),
                duration_ms: 1000,
                metrics: HashMap::new(),
            }
        };

        manifest.start();
        manifest.complete(result);
        manifest
    }

    #[test]
    fn test_auto_merge_policy() {
        let policy = MergePolicy {
            policy_type: MergePolicyType::AutoMerge,
            ..Default::default()
        };

        let manifest = create_completed_manifest(true);
        let decision = policy.evaluate(&manifest).unwrap();

        assert!(matches!(decision, MergeDecision::AutoMerge));
    }

    #[test]
    fn test_require_approval_policy() {
        let policy = MergePolicy {
            policy_type: MergePolicyType::RequireApproval,
            ..Default::default()
        };

        let manifest = create_completed_manifest(true);
        let decision = policy.evaluate(&manifest).unwrap();

        assert!(matches!(decision, MergeDecision::RequireApproval));
    }

    #[test]
    fn test_block_policy() {
        let policy = MergePolicy {
            policy_type: MergePolicyType::Block,
            ..Default::default()
        };

        let manifest = create_completed_manifest(true);
        let decision = policy.evaluate(&manifest).unwrap();

        assert!(matches!(decision, MergeDecision::Block));
    }

    #[test]
    fn test_evaluate_not_completed() {
        let policy = MergePolicy::default();
        let config = ExperimentConfig::default();
        let manifest = ExperimentManifest::new(
            "test-experiment".to_string(),
            "Test experiment".to_string(),
            config,
        );

        let result = policy.evaluate(&manifest);
        assert!(result.is_err());
        assert!(matches!(result, Err(MergePolicyError::NotCompleted)));
    }
}
```

---

### Step 6: Run merge policy tests to verify they fail

**Files:** Run tests

```bash
cargo test automation::experiment::merge_policy --lib
```

**Expected:** FAIL with "todo!()" errors

---

### Step 7: Implement merge policy

**Files:** Modify `automation/experiment/merge_policy.rs`

```rust
use super::manifest::{ExperimentManifest, ExperimentResult, MergePolicyType};
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeDecision {
    AutoMerge,
    RequireApproval,
    Block,
}

#[derive(Debug, Clone)]
pub struct MergePolicy {
    pub policy_type: MergePolicyType,
    pub min_success_rate: f64,
    pub max_failure_rate: f64,
    pub require_test_passage: bool,
}

impl Default for MergePolicy {
    fn default() -> Self {
        Self {
            policy_type: MergePolicyType::RequireApproval,
            min_success_rate: 0.95,
            max_failure_rate: 0.05,
            require_test_passage: true,
        }
    }
}

impl MergePolicy {
    pub fn evaluate(&self, manifest: &ExperimentManifest) -> Result<MergeDecision, MergePolicyError> {
        // Check if experiment completed
        if !matches!(
            manifest.status,
            ExperimentStatus::Completed | ExperimentStatus::Failed(_)
        ) {
            return Err(MergePolicyError::NotCompleted);
        }

        // Check if experiment failed
        if let ExperimentStatus::Failed(error) = &manifest.status {
            return Err(MergePolicyError::ExperimentFailed(error.clone()));
        }

        // Get results
        let results = manifest
            .results
            .as_ref()
            .ok_or_else(|| MergePolicyError::NotCompleted)?;

        // Check exit code
        if results.exit_code != 0 {
            return Err(MergePolicyError::ExperimentFailed(
                format!("Exit code: {}", results.exit_code),
            ));
        }

        // Check success rate (if available)
        if let Some(&success_rate) = results.metrics.get("success_rate") {
            if success_rate < self.min_success_rate {
                return Err(MergePolicyError::SuccessRateTooLow(
                    success_rate,
                    self.min_success_rate,
                ));
            }
        }

        // Check failure rate (if available)
        if let Some(&failure_rate) = results.metrics.get("failure_rate") {
            if failure_rate > self.max_failure_rate {
                return Err(MergePolicyError::FailureRateTooHigh(
                    failure_rate,
                    self.max_failure_rate,
                ));
            }
        }

        // Return decision based on policy type
        match self.policy_type {
            MergePolicyType::AutoMerge => Ok(MergeDecision::AutoMerge),
            MergePolicyType::RequireApproval => Ok(MergeDecision::RequireApproval),
            MergePolicyType::Block => Ok(MergeDecision::Block),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MergePolicyError {
    #[error("Experiment has not completed")]
    NotCompleted,
    #[error("Experiment failed: {0}")]
    ExperimentFailed(String),
    #[error("Success rate {0} below threshold {1}")]
    SuccessRateTooLow(f64, f64),
    #[error("Failure rate {0} above threshold {1}")]
    FailureRateTooHigh(f64, f64),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_completed_manifest(success: bool) -> ExperimentManifest {
        let config = ExperimentConfig::default();
        let mut manifest = ExperimentManifest::new(
            "test-experiment".to_string(),
            "Test experiment".to_string(),
            config,
        );

        let result = if success {
            ExperimentResult {
                exit_code: 0,
                output: "Success".to_string(),
                error: None,
                duration_ms: 1000,
                metrics: {
                    let mut m = HashMap::new();
                    m.insert("success_rate".to_string(), 1.0);
                    m
                },
            }
        } else {
            ExperimentResult {
                exit_code: 1,
                output: "Error".to_string(),
                error: Some("Failed".to_string()),
                duration_ms: 1000,
                metrics: HashMap::new(),
            }
        };

        manifest.start();
        manifest.complete(result);
        manifest
    }

    #[test]
    fn test_auto_merge_policy() {
        let policy = MergePolicy {
            policy_type: MergePolicyType::AutoMerge,
            ..Default::default()
        };

        let manifest = create_completed_manifest(true);
        let decision = policy.evaluate(&manifest).unwrap();

        assert!(matches!(decision, MergeDecision::AutoMerge));
    }

    #[test]
    fn test_require_approval_policy() {
        let policy = MergePolicy {
            policy_type: MergePolicyType::RequireApproval,
            ..Default::default()
        };

        let manifest = create_completed_manifest(true);
        let decision = policy.evaluate(&manifest).unwrap();

        assert!(matches!(decision, MergeDecision::RequireApproval));
    }

    #[test]
    fn test_block_policy() {
        let policy = MergePolicy {
            policy_type: MergePolicyType::Block,
            ..Default::default()
        };

        let manifest = create_completed_manifest(true);
        let decision = policy.evaluate(&manifest).unwrap();

        assert!(matches!(decision, MergeDecision::Block));
    }

    #[test]
    fn test_evaluate_not_completed() {
        let policy = MergePolicy::default();
        let config = ExperimentConfig::default();
        let manifest = ExperimentManifest::new(
            "test-experiment".to_string(),
            "Test experiment".to_string(),
            config,
        );

        let result = policy.evaluate(&manifest);
        assert!(result.is_err());
        assert!(matches!(result, Err(MergePolicyError::NotCompleted)));
    }

    #[test]
    fn test_evaluate_failed_experiment() {
        let policy = MergePolicy::default();
        let manifest = create_completed_manifest(false);

        let result = policy.evaluate(&manifest);
        assert!(result.is_err());
        assert!(matches!(result, Err(MergePolicyError::ExperimentFailed(_))));
    }

    #[test]
    fn test_success_rate_below_threshold() {
        let policy = MergePolicy {
            min_success_rate: 0.95,
            ..Default::default()
        };

        let config = ExperimentConfig::default();
        let mut manifest = ExperimentManifest::new(
            "test-experiment".to_string(),
            "Test experiment".to_string(),
            config,
        );

        let result = ExperimentResult {
            exit_code: 0,
            output: "Success".to_string(),
            error: None,
            duration_ms: 1000,
            metrics: {
                let mut m = HashMap::new();
                m.insert("success_rate".to_string(), 0.90);
                m
            },
        };

        manifest.start();
        manifest.complete(result);

        let decision = policy.evaluate(&manifest);
        assert!(decision.is_err());
        assert!(matches!(decision, Err(MergePolicyError::SuccessRateTooLow(..))));
    }
}
```

---

### Step 8: Run merge policy tests to verify they pass

**Files:** Run tests

```bash
cargo test automation::experiment::merge_policy --lib
```

**Expected:** PASS all tests

---

### Step 9: Write failing tests for isolation enforcer

**Files:** Create `automation/experiment/isolation.rs` (with tests)

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IsolationViolation {
    #[error("Cannot create experiment on protected branch: {0}")]
    ProtectedBranch(String),
    #[error("Cannot modify main branch: {0}")]
    MainBranchModification(String),
    #[error("Worktree conflict: {0}")]
    WorktreeConflict(String),
    #[error("Isolation check failed: {0}")]
    CheckFailed(String),
}

pub struct IsolationEnforcer {
    protected_branches: Vec<String>,
}

impl IsolationEnforcer {
    pub fn new() -> Self {
        Self {
            protected_branches: vec!["main".to_string(), "master".to_string()],
        }
    }

    pub fn with_protected_branches(mut self, branches: Vec<String>) -> Self {
        self.protected_branches = branches;
        self
    }

    pub fn validate_experiment_branch(&self, branch: &str) -> Result<(), IsolationViolation> {
        // Implementation will be added in Step 11
        todo!()
    }

    pub fn validate_base_branch(&self, branch: &str) -> Result<(), IsolationViolation> {
        // Implementation will be added in Step 11
        todo!()
    }
}

impl Default for IsolationEnforcer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_experiment_branch_valid() {
        let enforcer = IsolationEnforcer::new();
        let result = enforcer.validate_experiment_branch("experiment-1");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_experiment_branch_main() {
        let enforcer = IsolationEnforcer::new();
        let result = enforcer.validate_experiment_branch("main");
        assert!(result.is_err());
        assert!(matches!(result, Err(IsolationViolation::ProtectedBranch(_))));
    }

    #[test]
    fn test_validate_experiment_branch_master() {
        let enforcer = IsolationEnforcer::new();
        let result = enforcer.validate_experiment_branch("master");
        assert!(result.is_err());
        assert!(matches!(result, Err(IsolationViolation::ProtectedBranch(_))));
    }

    #[test]
    fn test_validate_base_branch_main() {
        let enforcer = IsolationEnforcer::new();
        let result = enforcer.validate_base_branch("main");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_base_branch_experiment() {
        let enforcer = IsolationEnforcer::new();
        let result = enforcer.validate_base_branch("experiment-1");
        assert!(result.is_err());
        assert!(matches!(result, Err(IsolationViolation::MainBranchModification(_))));
    }
}
```

---

### Step 10: Run isolation tests to verify they fail

**Files:** Run tests

```bash
cargo test automation::experiment::isolation --lib
```

**Expected:** FAIL with "todo!()" errors

---

### Step 11: Implement isolation enforcer

**Files:** Modify `automation/experiment/isolation.rs`

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IsolationViolation {
    #[error("Cannot create experiment on protected branch: {0}")]
    ProtectedBranch(String),
    #[error("Cannot modify main branch: {0}")]
    MainBranchModification(String),
    #[error("Worktree conflict: {0}")]
    WorktreeConflict(String),
    #[error("Isolation check failed: {0}")]
    CheckFailed(String),
}

pub struct IsolationEnforcer {
    protected_branches: Vec<String>,
}

impl IsolationEnforcer {
    pub fn new() -> Self {
        Self {
            protected_branches: vec!["main".to_string(), "master".to_string()],
        }
    }

    pub fn with_protected_branches(mut self, branches: Vec<String>) -> Self {
        self.protected_branches = branches;
        self
    }

    pub fn validate_experiment_branch(&self, branch: &str) -> Result<(), IsolationViolation> {
        if self.protected_branches.contains(&branch.to_string()) {
            return Err(IsolationViolation::ProtectedBranch(branch.to_string()));
        }
        Ok(())
    }

    pub fn validate_base_branch(&self, branch: &str) -> Result<(), IsolationViolation> {
        // Base branch must be a protected branch (main/master)
        if !self.protected_branches.contains(&branch.to_string()) {
            return Err(IsolationViolation::MainBranchModification(branch.to_string()));
        }
        Ok(())
    }
}

impl Default for IsolationEnforcer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_experiment_branch_valid() {
        let enforcer = IsolationEnforcer::new();
        let result = enforcer.validate_experiment_branch("experiment-1");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_experiment_branch_main() {
        let enforcer = IsolationEnforcer::new();
        let result = enforcer.validate_experiment_branch("main");
        assert!(result.is_err());
        assert!(matches!(result, Err(IsolationViolation::ProtectedBranch(_))));
    }

    #[test]
    fn test_validate_experiment_branch_master() {
        let enforcer = IsolationEnforcer::new();
        let result = enforcer.validate_experiment_branch("master");
        assert!(result.is_err());
        assert!(matches!(result, Err(IsolationViolation::ProtectedBranch(_))));
    }

    #[test]
    fn test_validate_base_branch_main() {
        let enforcer = IsolationEnforcer::new();
        let result = enforcer.validate_base_branch("main");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_base_branch_experiment() {
        let enforcer = IsolationEnforcer::new();
        let result = enforcer.validate_base_branch("experiment-1");
        assert!(result.is_err());
        assert!(matches!(result, Err(IsolationViolation::MainBranchModification(_))));
    }
}
```

---

### Step 12: Run isolation tests to verify they pass

**Files:** Run tests

```bash
cargo test automation::experiment::isolation --lib
```

**Expected:** PASS all tests

---

### Step 13: Write failing tests for git operations

**Files:** Create `automation/experiment/git_operations.rs` (with tests)

```rust
use super::isolation::IsolationViolation;
use super::manifest::{ExperimentConfig, ExperimentManifest, ExperimentResult};
use crate::automation::experiment::isolation::IsolationEnforcer;
use git2::{Repository, Worktree, WorktreeAddOptions};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tempfile::TempDir;
use thiserror::Error;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct WorktreeInfo {
    pub id: String,
    pub path: PathBuf,
    pub branch: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Error)]
pub enum GitExperimentError {
    #[error("Git error: {0}")]
    GitError(#[from] git2::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Isolation violation: {0}")]
    IsolationViolation(#[from] IsolationViolation),
    #[error("Experiment not found: {0}")]
    NotFound(String),
    #[error("Invalid state: {0}")]
    InvalidState(String),
}

pub struct GitExperimentManager {
    repo_path: PathBuf,
    experiments: Arc<RwLock<HashMap<String, ExperimentManifest>>>,
    worktrees: Arc<RwLock<HashMap<String, WorktreeInfo>>>,
    isolation_enforcer: IsolationEnforcer,
}

impl GitExperimentManager {
    pub fn new(repo_path: &Path) -> Result<Self, GitExperimentError> {
        Ok(Self {
            repo_path: repo_path.to_path_buf(),
            experiments: Arc::new(RwLock::new(HashMap::new())),
            worktrees: Arc::new(RwLock::new(HashMap::new())),
            isolation_enforcer: IsolationEnforcer::new(),
        })
    }

    pub fn with_isolation_enforcer(mut self, enforcer: IsolationEnforcer) -> Self {
        self.isolation_enforcer = enforcer;
        self
    }

    pub async fn create_experiment(
        &self,
        manifest: ExperimentManifest,
    ) -> Result<String, GitExperimentError> {
        // Implementation will be added in Step 15
        todo!()
    }

    pub async fn create_worktree(
        &self,
        experiment_id: &str,
        branch: &str,
    ) -> Result<PathBuf, GitExperimentError> {
        // Implementation will be added in Step 15
        todo!()
    }

    pub async fn complete_experiment(
        &self,
        experiment_id: &str,
        result: ExperimentResult,
    ) -> Result<(), GitExperimentError> {
        // Implementation will be added in Step 15
        todo!()
    }

    pub async fn cleanup_experiment(&self, experiment_id: &str) -> Result<(), GitExperimentError> {
        // Implementation will be added in Step 15
        todo!()
    }

    pub async fn get_experiment(&self, id: &str) -> Option<ExperimentManifest> {
        let experiments = self.experiments.read().await;
        experiments.get(id).cloned()
    }

    pub async fn list_experiments(&self) -> Vec<ExperimentManifest> {
        let experiments = self.experiments.read().await;
        experiments.values().cloned().collect()
    }
}

/// Create a temporary git repository for testing
pub fn create_temp_repo() -> Result<(TempDir, Repository), GitExperimentError> {
    let temp_dir = TempDir::new()?;
    let repo = Repository::init(temp_dir.path())?;

    // Create initial commit
    let mut index = repo.index()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    let sig = repo.signature()?;
    let head_id = repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        "Initial commit",
        &tree,
        &[],
    )?;

    // Set HEAD to main branch
    repo.set_head("refs/heads/main")?;

    Ok((temp_dir, repo))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_experiment() {
        let (temp_dir, _) = create_temp_repo().unwrap();
        let manager = GitExperimentManager::new(temp_dir.path()).unwrap();

        let config = ExperimentConfig::default();
        let mut manifest = ExperimentManifest::new(
            "test-experiment".to_string(),
            "Test experiment".to_string(),
            config,
        );

        let experiment_id = manager.create_experiment(manifest).await.unwrap();
        assert!(!experiment_id.is_empty());

        // Verify experiment exists
        let retrieved = manager.get_experiment(&experiment_id).await;
        assert!(retrieved.is_some());
    }

    #[tokio::test]
    async fn test_create_worktree() {
        let (temp_dir, _) = create_temp_repo().unwrap();
        let manager = GitExperimentManager::new(temp_dir.path()).unwrap();

        let config = ExperimentConfig::default();
        let mut manifest = ExperimentManifest::new(
            "test-experiment".to_string(),
            "Test experiment".to_string(),
            config,
        );

        let experiment_id = manager.create_experiment(manifest).await.unwrap();
        let worktree_path = manager
            .create_worktree(&experiment_id, "experiment-1")
            .await
            .unwrap();

        assert!(worktree_path.exists());
    }

    #[tokio::test]
    async fn test_cleanup_experiment() {
        let (temp_dir, _) = create_temp_repo().unwrap();
        let manager = GitExperimentManager::new(temp_dir.path()).unwrap();

        let config = ExperimentConfig::default();
        let mut manifest = ExperimentManifest::new(
            "test-experiment".to_string(),
            "Test experiment".to_string(),
            config,
        );

        let experiment_id = manager.create_experiment(manifest).await.unwrap();
        manager
            .create_worktree(&experiment_id, "experiment-1")
            .await
            .unwrap();

        manager.cleanup_experiment(&experiment_id).await.unwrap();

        // Verify experiment is gone
        let retrieved = manager.get_experiment(&experiment_id).await;
        assert!(retrieved.is_none());
    }
}
```

---

### Step 14: Run git operations tests to verify they fail

**Files:** Run tests

```bash
cargo test automation::experiment::git_operations --lib
```

**Expected:** FAIL with "todo!()" errors

---

### Step 15: Implement git operations

**Files:** Modify `automation/experiment/git_operations.rs`

```rust
use super::isolation::IsolationViolation;
use super::manifest::{ExperimentConfig, ExperimentManifest, ExperimentResult};
use crate::automation::experiment::isolation::IsolationEnforcer;
use git2::{Repository, Worktree, WorktreeAddOptions};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tempfile::TempDir;
use thiserror::Error;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct WorktreeInfo {
    pub id: String,
    pub path: PathBuf,
    pub branch: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Error)]
pub enum GitExperimentError {
    #[error("Git error: {0}")]
    GitError(#[from] git2::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Isolation violation: {0}")]
    IsolationViolation(#[from] IsolationViolation),
    #[error("Experiment not found: {0}")]
    NotFound(String),
    #[error("Invalid state: {0}")]
    InvalidState(String),
}

pub struct GitExperimentManager {
    repo_path: PathBuf,
    experiments: Arc<RwLock<HashMap<String, ExperimentManifest>>>,
    worktrees: Arc<RwLock<HashMap<String, WorktreeInfo>>>,
    isolation_enforcer: IsolationEnforcer,
}

impl GitExperimentManager {
    pub fn new(repo_path: &Path) -> Result<Self, GitExperimentError> {
        Ok(Self {
            repo_path: repo_path.to_path_buf(),
            experiments: Arc::new(RwLock::new(HashMap::new())),
            worktrees: Arc::new(RwLock::new(HashMap::new())),
            isolation_enforcer: IsolationEnforcer::new(),
        })
    }

    pub fn with_isolation_enforcer(mut self, enforcer: IsolationEnforcer) -> Self {
        self.isolation_enforcer = enforcer;
        self
    }

    pub async fn create_experiment(
        &self,
        manifest: ExperimentManifest,
    ) -> Result<String, GitExperimentError> {
        // Validate branch isolation
        self.isolation_enforcer
            .validate_experiment_branch(&manifest.config.experiment_branch)?;
        self.isolation_enforcer
            .validate_base_branch(&manifest.config.base_branch)?;

        // Check for duplicate experiment
        let experiments = self.experiments.read().await;
        if experiments.contains_key(&manifest.id) {
            return Err(GitExperimentError::InvalidState(format!(
                "Experiment {} already exists",
                manifest.id
            )));
        }
        drop(experiments);

        // Create experiment branch
        let repo = Repository::open(&self.repo_path)?;
        let commit = repo.head()?.peel_to_commit()?;
        let branch_ref = repo.branch(
            &manifest.config.experiment_branch,
            &commit,
            false,
        )?;

        // Store experiment
        let mut experiments = self.experiments.write().await;
        experiments.insert(manifest.id.clone(), manifest);
        drop(experiments);

        Ok(manifest.id)
    }

    pub async fn create_worktree(
        &self,
        experiment_id: &str,
        branch: &str,
    ) -> Result<PathBuf, GitExperimentError> {
        // Validate experiment exists
        let experiments = self.experiments.read().await;
        let experiment = experiments
            .get(experiment_id)
            .ok_or_else(|| GitExperimentError::NotFound(experiment_id.to_string()))?;

        // Validate branch
        if experiment.config.experiment_branch != branch {
            return Err(GitExperimentError::InvalidState(format!(
                "Branch {} does not match experiment branch {}",
                branch, experiment.config.experiment_branch
            )));
        }
        drop(experiments);

        // Create worktree
        let repo = Repository::open(&self.repo_path)?;
        let worktree_path = self.repo_path.join(format!("worktree-{}", experiment_id));

        let mut opts = WorktreeAddOptions::new();
        opts.reference(Some(repo.find_branch(branch, git2::BranchType::Local)?.get()));

        let worktree = repo.worktree(&experiment_id, &worktree_path, Some(&mut opts))?;

        // Store worktree info
        let info = WorktreeInfo {
            id: experiment_id.to_string(),
            path: worktree_path.clone(),
            branch: branch.to_string(),
            created_at: chrono::Utc::now(),
        };

        let mut worktrees = self.worktrees.write().await;
        worktrees.insert(experiment_id.to_string(), info);
        drop(worktrees);

        Ok(worktree_path)
    }

    pub async fn complete_experiment(
        &self,
        experiment_id: &str,
        result: ExperimentResult,
    ) -> Result<(), GitExperimentError> {
        let mut experiments = self.experiments.write().await;
        if let Some(experiment) = experiments.get_mut(experiment_id) {
            experiment.complete(result);
            Ok(())
        } else {
            Err(GitExperimentError::NotFound(experiment_id.to_string()))
        }
    }

    pub async fn cleanup_experiment(&self, experiment_id: &str) -> Result<(), GitExperimentError> {
        // Remove worktree
        if let Some(worktree_info) = self.worktrees.read().await.get(experiment_id) {
            let repo = Repository::open(&self.repo_path)?;
            if let Ok(worktree) = repo.find_worktree(&worktree_info.id) {
                let _ = worktree.prune(git2::WorktreePruneOptions::new());
            }
        }

        // Remove experiment
        let mut experiments = self.experiments.write().await;
        experiments.remove(experiment_id);

        // Remove worktree info
        let mut worktrees = self.worktrees.write().await;
        worktrees.remove(experiment_id);

        Ok(())
    }

    pub async fn get_experiment(&self, id: &str) -> Option<ExperimentManifest> {
        let experiments = self.experiments.read().await;
        experiments.get(id).cloned()
    }

    pub async fn list_experiments(&self) -> Vec<ExperimentManifest> {
        let experiments = self.experiments.read().await;
        experiments.values().cloned().collect()
    }
}

/// Create a temporary git repository for testing
pub fn create_temp_repo() -> Result<(TempDir, Repository), GitExperimentError> {
    let temp_dir = TempDir::new()?;
    let repo = Repository::init(temp_dir.path())?;

    // Create initial commit
    let mut index = repo.index()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    let sig = repo.signature()?;
    let head_id = repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        "Initial commit",
        &tree,
        &[],
    )?;

    // Set HEAD to main branch
    repo.set_head("refs/heads/main")?;

    Ok((temp_dir, repo))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_experiment() {
        let (temp_dir, _) = create_temp_repo().unwrap();
        let manager = GitExperimentManager::new(temp_dir.path()).unwrap();

        let config = ExperimentConfig::default();
        let mut manifest = ExperimentManifest::new(
            "test-experiment".to_string(),
            "Test experiment".to_string(),
            config,
        );

        let experiment_id = manager.create_experiment(manifest).await.unwrap();
        assert!(!experiment_id.is_empty());

        // Verify experiment exists
        let retrieved = manager.get_experiment(&experiment_id).await;
        assert!(retrieved.is_some());
    }

    #[tokio::test]
    async fn test_create_worktree() {
        let (temp_dir, _) = create_temp_repo().unwrap();
        let manager = GitExperimentManager::new(temp_dir.path()).unwrap();

        let config = ExperimentConfig::default();
        let mut manifest = ExperimentManifest::new(
            "test-experiment".to_string(),
            "Test experiment".to_string(),
            config,
        );

        let experiment_id = manager.create_experiment(manifest).await.unwrap();
        let worktree_path = manager
            .create_worktree(&experiment_id, "experiment-1")
            .await
            .unwrap();

        assert!(worktree_path.exists());
    }

    #[tokio::test]
    async fn test_cleanup_experiment() {
        let (temp_dir, _) = create_temp_repo().unwrap();
        let manager = GitExperimentManager::new(temp_dir.path()).unwrap();

        let config = ExperimentConfig::default();
        let mut manifest = ExperimentManifest::new(
            "test-experiment".to_string(),
            "Test experiment".to_string(),
            config,
        );

        let experiment_id = manager.create_experiment(manifest).await.unwrap();
        manager
            .create_worktree(&experiment_id, "experiment-1")
            .await
            .unwrap();

        manager.cleanup_experiment(&experiment_id).await.unwrap();

        // Verify experiment is gone
        let retrieved = manager.get_experiment(&experiment_id).await;
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_isolation_enforcement() {
        let (temp_dir, _) = create_temp_repo().unwrap();
        let manager = GitExperimentManager::new(temp_dir.path()).unwrap();

        // Try to create experiment on main branch (should fail)
        let mut config = ExperimentConfig::default();
        config.experiment_branch = "main".to_string();

        let manifest = ExperimentManifest::new(
            "test-experiment".to_string(),
            "Test experiment".to_string(),
            config,
        );

        let result = manager.create_experiment(manifest).await;
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(GitExperimentError::IsolationViolation(
                IsolationViolation::ProtectedBranch(_)
            ))
        ));
    }
}
```

---

### Step 16: Run git operations tests to verify they pass

**Files:** Run tests

```bash
cargo test automation::experiment::git_operations --lib
```

**Expected:** PASS all tests

---

### Step 17: Create integration test for git experiment framework

**Files:** Create `tests/integration/git_experiment_test.rs`

```rust
use agentsdk::automation::experiment::{
    create_temp_repo, ExperimentConfig, ExperimentManifest, ExperimentResult,
    GitExperimentManager, IsolationEnforcer,
};
use std::collections::HashMap;

#[tokio::test]
async fn test_experiment_lifecycle() {
    let (temp_dir, _) = create_temp_repo().unwrap();
    let manager = GitExperimentManager::new(temp_dir.path()).unwrap();

    // Create experiment
    let config = ExperimentConfig::default();
    let mut manifest = ExperimentManifest::new(
        "test-experiment".to_string(),
        "Test experiment".to_string(),
        config,
    );

    let experiment_id = manager.create_experiment(manifest.clone()).await.unwrap();

    // Create worktree
    let worktree_path = manager
        .create_worktree(&experiment_id, "experiment-1")
        .await
        .unwrap();

    assert!(worktree_path.exists());

    // Complete experiment
    let result = ExperimentResult {
        exit_code: 0,
        output: "Success".to_string(),
        error: None,
        duration_ms: 1000,
        metrics: HashMap::new(),
    };

    manager
        .complete_experiment(&experiment_id, result)
        .await
        .unwrap();

    // Cleanup
    manager.cleanup_experiment(&experiment_id).await.unwrap();

    // Verify cleanup
    let retrieved = manager.get_experiment(&experiment_id).await;
    assert!(retrieved.is_none());
}

#[tokio::test]
async fn test_isolation_enforcement() {
    let (temp_dir, _) = create_temp_repo().unwrap();
    let enforcer = IsolationEnforcer::new();
    let manager = GitExperimentManager::new(temp_dir.path())
        .unwrap()
        .with_isolation_enforcer(enforcer);

    // Try to create experiment on main branch
    let mut config = ExperimentConfig::default();
    config.experiment_branch = "main".to_string();

    let manifest = ExperimentManifest::new(
        "test-experiment".to_string(),
        "Test experiment".to_string(),
        config,
    );

    let result = manager.create_experiment(manifest).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_parallel_experiments() {
    let (temp_dir, _) = create_temp_repo().unwrap();
    let manager = GitExperimentManager::new(temp_dir.path()).unwrap();

    // Create multiple experiments
    for i in 0..3 {
        let mut config = ExperimentConfig::default();
        config.experiment_branch = format!("experiment-{}", i);

        let manifest = ExperimentManifest::new(
            format!("experiment-{}", i),
            format!("Test experiment {}", i),
            config,
        );

        let experiment_id = manager.create_experiment(manifest).await.unwrap();
        let worktree_path = manager
            .create_worktree(&experiment_id, &format!("experiment-{}", i))
            .await
            .unwrap();

        assert!(worktree_path.exists());
    }

    // Verify all experiments exist
    let experiments = manager.list_experiments().await;
    assert_eq!(experiments.len(), 3);
}
```

---

### Step 18: Run integration tests

**Files:** Run tests

```bash
cargo test --test git_experiment_test
```

**Expected:** PASS all tests

---

### Step 19: Update automation module to include experiment

**Files:** Modify `automation/mod.rs`

```rust
pub mod cron;
pub mod experiment;

pub use cron::{
    CronScheduler, CronSchedulerInstance, CronExpression, CronParseError,
    ScheduleSafetyConfig, ScheduleValidator, ValidationError,
};
pub use experiment::{
    ExperimentConfig, ExperimentManager, ExperimentManifest, ExperimentResult,
    GitExperimentManager, IsolationEnforcer, MergePolicy, MergePolicyType,
};
```

---

### Step 20: Write documentation

**Files:** Create `automation/experiment/README.md`

```markdown
# Git Experiment Framework

The git experiment framework enables safe parallel experimentation with complete
branch isolation and worktree management.

## Features

- Isolated git branches for each experiment
- Git worktree management for parallel experiments
- Configurable merge policies (auto-merge, require-approval, block)
- Strict isolation enforcement (no experiments on main branch)
- Complete cleanup procedures

## Usage

```rust
use agentsdk::automation::experiment::{
    GitExperimentManager, ExperimentManifest, ExperimentConfig,
};

let manager = GitExperimentManager::new("/path/to/repo")?;

// Create experiment
let config = ExperimentConfig::default();
let manifest = ExperimentManifest::new(
    "test-experiment".to_string(),
    "Test experiment".to_string(),
    config,
);

let experiment_id = manager.create_experiment(manifest).await?;

// Create worktree
let worktree_path = manager
    .create_worktree(&experiment_id, "experiment-1")
    .await?;

// Run experiment in worktree...

// Complete experiment
let result = ExperimentResult { /* ... */ };
manager.complete_experiment(&experiment_id, result).await?;

// Cleanup
manager.cleanup_experiment(&experiment_id).await?;
```

## ADR-0007 Compliance

- Experiments MUST run in isolated branches (never main)
- Merge proposals are outputs only (auto-commits forbidden)
- Failed experiments are fully cleaned up

## Safety

- Branch isolation enforcement
- Worktree isolation
- Configurable merge policies
- Complete cleanup on failure
```

---

### Step 21: Commit all changes

**Files:** Commit

```bash
git add automation/experiment tests/integration/git_experiment_test.rs automation/mod.rs
git commit -m "feat(automation): implement git experiment framework (Task 01)

- Add experiment manifest and status tracking
- Implement merge policy evaluation (auto-merge/require-approval/block)
- Add isolation enforcer to prevent main branch experiments
- Implement git worktree management for parallel experiments
- Add comprehensive unit and integration tests
- Follow ADR-0007: branch isolation enforced, no auto-commits

Refs: Phase 6, Task 01"
```

---

## Summary

Task 01 implements git experiment framework with:

✅ Experiment manifest and status tracking
✅ Merge policy evaluation (auto-merge, require-approval, block)
✅ Isolation enforcer (prevents main branch experiments)
✅ Git worktree management (parallel experiments)
✅ Complete cleanup procedures
✅ Comprehensive unit and integration tests
✅ ADR-0007 compliance (branch isolation, no auto-commits)

**Next Steps:** Task 02 (Merge Proposal Generation) or Task 05 (Rollback & Cleanup)
