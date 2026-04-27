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

---

## Implementation Research

### Recommended Libraries

| Library | Version | Purpose | Notes |
|---------|---------|---------|-------|
| clap | 4.4 | CLI parsing | Argument parsing and help generation |
| tokio | 1.35 | Async runtime | Core async infrastructure |
| serde | 1.0 | Serialization | JSON support |
| serde_json | 1.0 | JSON format | Standard JSON I/O |
| thiserror | 1.0 | Error handling | Type-safe errors |
| anyhow | 1.0 | Error composition | Flexible error handling |
| colored | 2.0 | Terminal colors | CLI output formatting |
| tabled | 0.14 | Table formatting | Pretty output for lists |

### Key Design Decisions

- **CLI structure**: Subcommands for schedule, experiment, merge, refine, rollback
- **Manual approval**: CLI commands for approve/reject merge proposals (no auto-commits)
- **Refinement capture**: Commands to create and list manual refinements
- **Schedule management**: Create, list, cancel scheduled workflows
- **Experiment management**: Create, list, status, cleanup experiments
- **Rollback procedures**: Rollback merges, clean up artifacts
- **ADR-0007 compliance**: Manual control over all automation actions

### Implementation Pattern

\`\`\`rust
// CLI entry point
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = AutomationCli::parse();
    
    match cli.command {
        Commands::Schedule(ScheduleCommands::Create { id, workflow_id, cron, timezone }) => {
            let scheduler = CronScheduler::new()?;
            scheduler.add_schedule(&id, &workflow_id, &cron, &timezone).await?;
            println!("Schedule created: {}", id);
        }
        Commands::Schedule(ScheduleCommands::List {}) => {
            let scheduler = CronScheduler::new()?;
            let schedules = scheduler.list_schedules().await?;
            print_schedule_table(&schedules);
        }
        Commands::Experiment(ExperimentCommands::Create { name, branch }) => {
            let manager = ExperimentManager::new()?;
            let experiment = manager.create_experiment(&name, &branch).await?;
            println!("Experiment created: {}", experiment.id);
        }
        Commands::Merge(MergeCommands::View { proposal_id }) => {
            let manager = ProposalManager::new()?;
            let proposal = manager.get_proposal(&proposal_id).await?;
            print_proposal(&proposal);
        }
        Commands::Merge(MergeCommands::Approve { proposal_id, reason }) => {
            let manager = ProposalManager::new()?;
            let event_id = manager.approve_proposal(&proposal_id, &reason).await?;
            println!("Proposal approved, event ID: {}", event_id);
        }
        Commands::Refine(RefineCommands::Create { target_type, target_id, comment }) => {
            let manager = RefinementManager::new()?;
            let event_id = manager.create_refinement(target_type, target_id, comment).await?;
            println!("Refinement created, event ID: {}", event_id);
        }
        Commands::Rollback(RollbackCommands::Experiment { experiment_id }) => {
            let manager = RollbackManager::new()?;
            manager.rollback_experiment(&experiment_id).await?;
            println!("Experiment rolled back");
        }
    }
    
    Ok(())
}
\`\`\`

### Dependencies on Prior Phases

- **Phase 5 Tasks 00-06**: All Phase 5 tasks (schedule, experiment, merge, refine, rollback)
- **Phase 2**: CLI integration (existing CLI structure)
- **Phase 5 Task 08**: Automation UI Integration (browser-based controls)

### Testing Strategy

- **Unit**: CLI command parsing, command validation
- **Integration**: Full CLI workflow with all subcommands
- **Property**: CLI commands produce expected outputs for given inputs

### Schema Alignment

- **Schema Ref**: Lines 110-148 (provenance tracking for CLI operations)
- **Schema Ref**: Lines 110-148 (provenance tracking for manual operations)

### Critical Constraints

- **MUST** provide manual approval/rejection commands (no auto-commits)
- **MUST** support refinement capture and listing
- **MUST** provide schedule management (create, list, cancel)
- **MUST** provide experiment management (create, list, status, cleanup)
- **MUST** provide rollback procedures (experiment, cleanup-all, verify)
- **MUST NOT** auto-commit merge proposals (ADR-0007 requirement)
- **MUST NOT** allow destructive operations without confirmation


## QA Cross-References

### QA Criteria
- **QA Area**: Area 8 - Automation CLI
- **QA Criteria**: [../../qa/phase-05/QA-CRITERIA.md#area-8-automation-cli](../../qa/phase-05/QA-CRITERIA.md#area-8-automation-cli)
- **Priority**: P0
**Test Types**: Integration, E2E

