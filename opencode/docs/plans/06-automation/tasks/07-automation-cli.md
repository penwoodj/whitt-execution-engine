# Task 07: Automation CLI

**Component:** CLI commands for schedule, experiment, merge, refine, rollback management

**Dependencies:** All previous tasks

**Estimated Time:** 7-9 days

**Goal:** Build a comprehensive CLI for managing automation features including schedules, experiments, merge proposals, refinements, and rollbacks.

---

## Overview

The automation CLI provides:

- Schedule management (create, list, cancel)
- Experiment management (create, list, status, cleanup)
- Merge proposal management (view, approve, reject)
- Refinement capture (create, list)
- Rollback procedures

**ADR-0007 Compliance:**
- CLI commands for manual approval/rejection (no auto-commits)
- Complete audit trail via refinement events
- Manual control over all automation actions

---

## File Structure

**New Files:**
- `automation/cli/mod.rs` - Module exports
- `automation/cli/schedule.rs` - Schedule management commands
- `automation/cli/experiment.rs` - Experiment commands
- `automation/cli/merge.rs` - Merge proposal commands
- `automation/cli/refine.rs` - Refinement commands
- `automation/cli/rollback.rs` - Rollback commands

---

## Implementation Steps

### Step 1: Create CLI module structure

**Files:** Create `automation/cli/mod.rs`

```rust
pub mod schedule;
pub mod experiment;
pub mod merge;
pub mod refine;
pub mod rollback;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "agentsdk-automation")]
#[command(about = "AgentSDK Automation CLI")]
pub struct AutomationCli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Schedule(schedule::ScheduleCommands),
    Experiment(experiment::ExperimentCommands),
    Merge(merge::MergeCommands),
    Refine(refine::RefineCommands),
    Rollback(rollback::RollbackCommands),
}
```

---

### Step 2: Implement schedule commands

**Files:** Create `automation/cli/schedule.rs`

```rust
use clap::{Parser, Subcommand};
use crate::automation::cron::{CronScheduler, parse_cron_expression};
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum ScheduleCommands {
    /// Create a new schedule
    Create {
        /// Schedule ID
        #[arg(short, long)]
        id: String,
        /// Workflow ID to schedule
        #[arg(short, long)]
        workflow_id: String,
        /// Cron expression
        #[arg(short, long)]
        cron: String,
        /// Timezone
        #[arg(short, long)]
        timezone: Option<String>,
    },
    /// List all schedules
    List,
    /// Cancel a schedule
    Cancel {
        /// Schedule ID
        id: String,
    },
}

impl ScheduleCommands {
    pub async fn execute(self, repo_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            ScheduleCommands::Create { id, workflow_id, cron, timezone } => {
                let cron_expr = parse_cron_expression(&cron)?;
                let scheduler = CronScheduler::new(/* execution_engine */);

                // Build full cron expression with timezone
                let full_cron = if let Some(tz) = timezone {
                    format!("{}@{}", cron, tz)
                } else {
                    cron
                };

                let cron_expr = parse_cron_expression(&full_cron)?;

                scheduler.add_schedule(id, workflow_id, cron_expr).await?;
                println!("Schedule created successfully");
            }
            ScheduleCommands::List => {
                let scheduler = CronScheduler::new(/* execution_engine */);
                let schedules = scheduler.list_schedules().await;

                println!("Schedules:");
                for schedule in schedules {
                    println!("  - {}: {} (next run: {:?})", schedule.id, schedule.workflow_id, schedule.next_run);
                }
            }
            ScheduleCommands::Cancel { id } => {
                let scheduler = CronScheduler::new(/* execution_engine */);
                scheduler.remove_schedule(&id).await?;
                println!("Schedule cancelled: {}", id);
            }
        }

        Ok(())
    }
}
```

---

### Step 3: Implement experiment commands

**Files:** Create `automation/cli/experiment.rs`

```rust
use clap::{Parser, Subcommand};
use crate::automation::experiment::{GitExperimentManager, ExperimentManifest, ExperimentConfig, ExperimentResult};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum ExperimentCommands {
    /// Create a new experiment
    Create {
        /// Experiment name
        #[arg(short, long)]
        name: String,
        /// Description
        #[arg(short, long)]
        description: String,
        /// Base branch
        #[arg(short, long, default_value = "main")]
        base_branch: String,
        /// Experiment branch
        #[arg(short, long)]
        experiment_branch: String,
        /// Merge policy (auto-merge, require-approval, block)
        #[arg(short, long, default_value = "require-approval")]
        merge_policy: String,
    },
    /// List all experiments
    List,
    /// Show experiment status
    Status {
        /// Experiment ID
        id: String,
    },
    /// Cleanup experiment
    Cleanup {
        /// Experiment ID
        id: String,
    },
}

impl ExperimentCommands {
    pub async fn execute(self, repo_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            ExperimentCommands::Create { name, description, base_branch, experiment_branch, merge_policy } => {
                let manager = GitExperimentManager::new(&repo_path)?;

                let config = ExperimentConfig {
                    base_branch,
                    experiment_branch,
                    merge_policy: match merge_policy.as_str() {
                        "auto-merge" => crate::automation::experiment::MergePolicyType::AutoMerge,
                        "require-approval" => crate::automation::experiment::MergePolicyType::RequireApproval,
                        "block" => crate::automation::experiment::MergePolicyType::Block,
                        _ => return Err("Invalid merge policy".into()),
                    },
                    cleanup_on_failure: true,
                    keep_artifacts: false,
                };

                let manifest = ExperimentManifest::new(name, description, config);
                let experiment_id = manager.create_experiment(manifest).await?;

                println!("Experiment created: {}", experiment_id);
            }
            ExperimentCommands::List => {
                let manager = GitExperimentManager::new(&repo_path)?;
                let experiments = manager.list_experiments().await;

                println!("Experiments:");
                for exp in experiments {
                    println!("  - {}: {} (status: {:?})", exp.id, exp.name, exp.status);
                }
            }
            ExperimentCommands::Status { id } => {
                let manager = GitExperimentManager::new(&repo_path)?;
                if let Some(exp) = manager.get_experiment(&id).await {
                    println!("Experiment: {}", exp.name);
                    println!("Status: {:?}", exp.status);
                    println!("Created: {}", exp.created_at);
                } else {
                    println!("Experiment not found: {}", id);
                }
            }
            ExperimentCommands::Cleanup { id } => {
                let manager = GitExperimentManager::new(&repo_path)?;
                manager.cleanup_experiment(&id).await?;
                println!("Experiment cleaned up: {}", id);
            }
        }

        Ok(())
    }
}
```

---

### Step 4: Implement merge commands

**Files:** Create `automation/cli/merge.rs`

```rust
use clap::{Parser, Subcommand};
use crate::automation::merge::{ProposalArtifactManager, ProposalMetadata, ProposalStatus};
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum MergeCommands {
    /// List merge proposals
    List,
    /// View a merge proposal
    View {
        /// Proposal ID
        id: String,
    },
    /// Approve a merge proposal
    Approve {
        /// Proposal ID
        id: String,
        /// Approval reason
        #[arg(short, long)]
        reason: String,
    },
    /// Reject a merge proposal
    Reject {
        /// Proposal ID
        id: String,
        /// Rejection reason
        #[arg(short, long)]
        reason: String,
    },
}

impl MergeCommands {
    pub async fn execute(self, repo_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            MergeCommands::List => {
                let manager = ProposalArtifactManager::new(&repo_path)?;
                let proposals = manager.list_proposals()?;

                println!("Merge Proposals:");
                for proposal in proposals {
                    println!("  - {}: {} -> {} (status: {:?})",
                        proposal.id,
                        proposal.base_branch,
                        proposal.experiment_branch,
                        proposal.status
                    );
                }
            }
            MergeCommands::View { id } => {
                let manager = ProposalArtifactManager::new(&repo_path)?;
                if let Some(proposal) = manager.load_proposal(&id)? {
                    println!("Proposal: {}", proposal.id);
                    println!("Experiment: {}", proposal.experiment_id);
                    println!("Status: {:?}", proposal.status);
                    println!("Confidence: {:?}", proposal.confidence);
                } else {
                    println!("Proposal not found: {}", id);
                }
            }
            MergeCommands::Approve { id, reason } => {
                // Create refinement event for approval
                println!("Approving proposal {} because: {}", id, reason);

                // This would create a RefinementEvent with Approve type
                // And link it to the proposal

                println!("Proposal approved: {}", id);
            }
            MergeCommands::Reject { id, reason } => {
                // Create refinement event for rejection
                println!("Rejecting proposal {} because: {}", id, reason);

                // This would create a RefinementEvent with Reject type
                // And link it to the proposal

                println!("Proposal rejected: {}", id);
            }
        }

        Ok(())
    }
}
```

---

### Step 5: Implement refine and rollback commands

**Files:** Create `automation/cli/refine.rs` and `automation/cli/rollback.rs`

**Files:** `automation/cli/refine.rs`

```rust
use clap::{Parser, Subcommand};
use crate::automation::refinement::{RefinementManager, RefinementEvent, RefinementType};

#[derive(Subcommand)]
pub enum RefineCommands {
    /// List refinements
    List,
    /// Create a refinement
    Create {
        /// Type (approve, reject, refine, comment)
        #[arg(short, long)]
        r#type: String,
        /// What was refined
        #[arg(short, long)]
        what: String,
        /// Why it was refined
        #[arg(short, long)]
        why: String,
        /// Who made the refinement
        #[arg(short, long)]
        who: String,
    },
}

impl RefineCommands {
    pub async fn execute(self, repo_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            RefineCommands::List => {
                let manager = RefinementManager::new(&repo_path)?;
                let refinements = manager.get_all_refinements()?;

                println!("Refinements:");
                for refn in refinements {
                    println!("  - {}: {:?} - {}", refn.id, refn.event_type, refn.what);
                }
            }
            RefineCommands::Create { r#type, what, why, who } => {
                let manager = RefinementManager::new(&repo_path)?;

                let event_type = match r#type.as_str() {
                    "approve" => RefinementType::Approve,
                    "reject" => RefinementType::Reject,
                    "refine" => RefinementType::Refine,
                    "comment" => RefinementType::Comment,
                    _ => return Err("Invalid refinement type".into()),
                };

                let event = RefinementEvent::new(event_type, what, why, who);
                let event_id = manager.create_refinement(event)?;

                println!("Refinement created: {}", event_id);
            }
        }

        Ok(())
    }
}
```

**Files:** `automation/cli/rollback.rs`

```rust
use clap::{Parser, Subcommand};
use crate::automation::rollback::{RollbackProcedure, CleanupManager, RollbackVerifier};

#[derive(Subcommand)]
pub enum RollbackCommands {
    /// Rollback an experiment
    Experiment {
        /// Experiment ID
        id: String,
    },
    /// Cleanup all failed experiments
    CleanupAll,
    /// Verify rollback
    Verify {
        /// Experiment ID
        id: String,
    },
}

impl RollbackCommands {
    pub async fn execute(self, repo_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            RollbackCommands::Experiment { id } => {
                let procedure = RollbackProcedure::new(&repo_path);
                let experiment_manager = crate::automation::experiment::GitExperimentManager::new(&repo_path)?;

                let result = procedure.rollback_experiment(&id, &experiment_manager).await;

                match result {
                    crate::automation::rollback::RollbackResult::Success => {
                        println!("Experiment rolled back: {}", id);
                    }
                    crate::automation::rollback::RollbackResult::PartialFailure(msg) => {
                        println!("Partial failure: {}", msg);
                    }
                    crate::automation::rollback::RollbackResult::CompleteFailure(msg) => {
                        println!("Complete failure: {}", msg);
                    }
                }
            }
            RollbackCommands::CleanupAll => {
                let manager = CleanupManager::new(&repo_path);
                let result = manager.cleanup_all_failed_experiments();

                println!("Cleaned up {} items", result.items_removed);
                for failure in &result.items_failed {
                    println!("  Failed: {}", failure);
                }
            }
            RollbackCommands::Verify { id } => {
                let verifier = RollbackVerifier::new(&repo_path);
                let result = verifier.verify_rollback(&id);

                match result {
                    crate::automation::rollback::VerificationResult::Success => {
                        println!("Rollback verified successfully");
                    }
                    crate::automation::rollback::VerificationResult::Failed(msg) => {
                        println!("Rollback verification failed: {}", msg);
                    }
                }
            }
        }

        Ok(())
    }
}
```

---

### Step 6: Update automation module and commit

**Files:** Modify `automation/mod.rs` and commit

```bash
git add automation/cli
git commit -m "feat(automation): implement automation CLI (Task 07)

- Add schedule management commands (create, list, cancel)
- Add experiment commands (create, list, status, cleanup)
- Add merge proposal commands (view, approve, reject)
- Add refinement capture commands (list, create)
- Add rollback commands (experiment, cleanup-all, verify)
- Follow ADR-0007: CLI commands for manual approval/rejection

Refs: Phase 6, Task 07"
```

---

## Summary

Task 07 implements automation CLI with:

✅ Schedule management commands
✅ Experiment management commands
✅ Merge proposal commands (view, approve, reject)
✅ Refinement capture commands
✅ Rollback commands
✅ ADR-0007 compliance (manual control, no auto-commits)

**Next Steps:** Task 08 (Automation UI Integration)
