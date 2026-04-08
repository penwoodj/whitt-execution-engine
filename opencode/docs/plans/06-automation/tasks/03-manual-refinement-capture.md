# Task 03: Manual Refinement Capture

**Component:** Event capture, linking to artifacts, approval/rejection workflows

**Dependencies:** Task 02 (Merge Proposal Generation)

**Estimated Time:** 5-7 days

**Goal:** Build a refinement capture system that records all manual changes (approvals, rejections, refinements) as immutable events in `.glyphnova/refinements/`, linked to artifacts for complete audit trails.

---

## Overview

The manual refinement capture system:

- Records all refinement events (what, why, who, when)
- Links events to proposals, experiments, and other artifacts
- Provides immutable event log
- Supports approval/rejection workflows
- Enables rollback by reversing event log

**ADR-0007 Compliance:**
- All manual refinements preserved as artifacted events
- Complete audit trail with linking
- Immutable event log (no modifications)
- Rollback can reverse event log

---

## File Structure

**New Files:**
- `automation/refinement/mod.rs` - Module exports
- `automation/refinement/events.rs` - Refinement event types
- `automation/refinement/capture.rs` - Event capture logic
- `automation/refinement/linking.rs` - Artifact linking

---

## Implementation Steps

### Step 1: Create refinement module structure

**Files:** Create `automation/refinement/mod.rs`

```rust
pub mod events;
pub mod capture;
pub mod linking;

pub use events::{RefinementEvent, RefinementType, RefinementStatus};
pub use capture::{RefinementCapture, RefinementManager};
pub use linking::{ArtifactLink, LinkedArtifacts};

use std::path::PathBuf;

/// Refinement artifact directory
pub const REFINEMENT_ARTIFACT_DIR: &str = ".glyphnova/refinements";
```

---

### Step 2: Implement refinement event types

**Files:** Create `automation/refinement/events.rs`

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefinementType {
    Approve,
    Reject,
    Refine,
    Comment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefinementStatus {
    Pending,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefinementEvent {
    pub id: String,
    pub event_type: RefinementType,
    pub status: RefinementStatus,
    pub what: String,           // What was refined
    pub why: String,            // Why the refinement was made
    pub who: String,            // Who made the refinement
    pub when: DateTime<Utc>,    // When the refinement was made
    pub linked_artifacts: Vec<String>,  // IDs of linked artifacts
    pub metadata: serde_json::Value,    // Additional metadata
}

impl RefinementEvent {
    pub fn new(
        event_type: RefinementType,
        what: String,
        why: String,
        who: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            event_type,
            status: RefinementStatus::Pending,
            what,
            why,
            who,
            when: Utc::now(),
            linked_artifacts: Vec::new(),
            metadata: serde_json::json!({}),
        }
    }

    pub fn with_linked_artifacts(mut self, artifacts: Vec<String>) -> Self {
        self.linked_artifacts = artifacts;
        self
    }

    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn mark_completed(&mut self) {
        self.status = RefinementStatus::Completed;
    }

    pub fn mark_cancelled(&mut self) {
        self.status = RefinementStatus::Cancelled;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_refinement_event() {
        let event = RefinementEvent::new(
            RefinementType::Approve,
            "Merge proposal".to_string(),
            "All tests pass".to_string(),
            "user@example.com".to_string(),
        );

        assert_eq!(event.what, "Merge proposal");
        assert_eq!(event.why, "All tests pass");
        assert_eq!(event.who, "user@example.com");
        assert!(matches!(event.status, RefinementStatus::Pending));
    }

    #[test]
    fn test_mark_completed() {
        let mut event = RefinementEvent::new(
            RefinementType::Approve,
            "Merge proposal".to_string(),
            "All tests pass".to_string(),
            "user@example.com".to_string(),
        );

        event.mark_completed();
        assert!(matches!(event.status, RefinementStatus::Completed));
    }
}
```

---

### Step 3: Implement refinement capture

**Files:** Create `automation/refinement/capture.rs`

```rust
use super::events::{RefinementEvent, RefinementStatus};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub struct RefinementCapture {
    event_log_path: PathBuf,
}

impl RefinementCapture {
    pub fn new(repo_path: &Path) -> Result<Self, std::io::Error> {
        let refinement_dir = repo_path.join(super::REFINEMENT_ARTIFACT_DIR);
        fs::create_dir_all(&refinement_dir)?;

        let event_log_path = refinement_dir.join("event-log.jsonl");
        Ok(Self { event_log_path })
    }

    pub fn capture_event(&self, event: &RefinementEvent) -> Result<(), std::io::Error> {
        let mut event_to_save = event.clone();
        event_to_save.mark_completed();

        let event_json = serde_json::to_string(&event_to_save)?;
        let mut log = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.event_log_path)?;

        use std::io::Write;
        writeln!(log, "{}", event_json)?;

        Ok(())
    }

    pub fn get_events(&self) -> Result<Vec<RefinementEvent>, std::io::Error> {
        let mut events = Vec::new();

        if self.event_log_path.exists() {
            let content = fs::read_to_string(&self.event_log_path)?;
            for line in content.lines() {
                if let Ok(event) = serde_json::from_str::<RefinementEvent>(line) {
                    events.push(event);
                }
            }
        }

        events.sort_by(|a, b| b.when.cmp(&a.when));
        Ok(events)
    }

    pub fn get_events_for_artifact(&self, artifact_id: &str) -> Result<Vec<RefinementEvent>, std::io::Error> {
        let all_events = self.get_events()?;
        let filtered = all_events
            .into_iter()
            .filter(|e| e.linked_artifacts.contains(&artifact_id.to_string()))
            .collect();
        Ok(filtered)
    }
}

pub struct RefinementManager {
    repo_path: PathBuf,
    capture: RefinementCapture,
}

impl RefinementManager {
    pub fn new(repo_path: &Path) -> Result<Self, std::io::Error> {
        let capture = RefinementCapture::new(repo_path)?;
        Ok(Self {
            repo_path: repo_path.to_path_buf(),
            capture,
        })
    }

    pub fn create_refinement(
        &self,
        event: RefinementEvent,
    ) -> Result<String, std::io::Error> {
        self.capture.capture_event(&event)?;
        Ok(event.id.clone())
    }

    pub fn get_all_refinements(&self) -> Result<Vec<RefinementEvent>, std::io::Error> {
        self.capture.get_events()
    }

    pub fn get_refinements_for_artifact(
        &self,
        artifact_id: &str,
    ) -> Result<Vec<RefinementEvent>, std::io::Error> {
        self.capture.get_events_for_artifact(artifact_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capture_event() {
        let temp_dir = tempfile::tempdir().unwrap();
        let capture = RefinementCapture::new(temp_dir.path()).unwrap();

        let event = RefinementEvent::new(
            super::events::RefinementType::Approve,
            "Test approval".to_string(),
            "Good changes".to_string(),
            "user@example.com".to_string(),
        );

        capture.capture_event(&event).unwrap();

        let events = capture.get_events().unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].what, "Test approval");
    }

    #[test]
    fn test_get_events_for_artifact() {
        let temp_dir = tempfile::tempdir().unwrap();
        let capture = RefinementCapture::new(temp_dir.path()).unwrap();

        let mut event1 = RefinementEvent::new(
            super::events::RefinementType::Approve,
            "Test approval".to_string(),
            "Good changes".to_string(),
            "user@example.com".to_string(),
        );
        event1.linked_artifacts = vec!["artifact-1".to_string()];
        capture.capture_event(&event1).unwrap();

        let events = capture.get_events_for_artifact("artifact-1").unwrap();
        assert_eq!(events.len(), 1);
    }
}
```

---

### Step 4: Implement artifact linking

**Files:** Create `automation/refinement/linking.rs`

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactLink {
    pub artifact_id: String,
    pub artifact_type: String,  // "proposal", "experiment", "refinement", etc.
    pub link_type: String,      // "approves", "rejects", "references", etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedArtifacts {
    pub source_id: String,
    pub links: Vec<ArtifactLink>,
}

impl LinkedArtifacts {
    pub fn new(source_id: String) -> Self {
        Self {
            source_id,
            links: Vec::new(),
        }
    }

    pub fn add_link(mut self, link: ArtifactLink) -> Self {
        self.links.push(link);
        self
    }

    pub fn get_links_by_type(&self, link_type: &str) -> Vec<&ArtifactLink> {
        self.links
            .iter()
            .filter(|l| l.link_type == link_type)
            .collect()
    }

    pub fn get_links_by_artifact_type(&self, artifact_type: &str) -> Vec<&ArtifactLink> {
        self.links
            .iter()
            .filter(|l| l.artifact_type == artifact_type)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_link() {
        let mut linked = LinkedArtifacts::new("source-1".to_string());

        let link = ArtifactLink {
            artifact_id: "artifact-1".to_string(),
            artifact_type: "proposal".to_string(),
            link_type: "approves".to_string(),
        };

        linked = linked.add_link(link);

        assert_eq!(linked.links.len(), 1);
        assert_eq!(linked.links[0].artifact_id, "artifact-1");
    }

    #[test]
    fn test_get_links_by_type() {
        let mut linked = LinkedArtifacts::new("source-1".to_string());

        linked = linked.add_link(ArtifactLink {
            artifact_id: "artifact-1".to_string(),
            artifact_type: "proposal".to_string(),
            link_type: "approves".to_string(),
        });

        linked = linked.add_link(ArtifactLink {
            artifact_id: "artifact-2".to_string(),
            artifact_type: "proposal".to_string(),
            link_type: "rejects".to_string(),
        });

        let approves = linked.get_links_by_type("approves");
        assert_eq!(approves.len(), 1);
        assert_eq!(approves[0].artifact_id, "artifact-1");
    }
}
```

---

### Step 5: Create integration test for refinement capture

**Files:** Create `tests/integration/refinement_capture_test.rs`

```rust
use agentsdk::automation::refinement::{
    RefinementManager, RefinementEvent, RefinementType, LinkedArtifacts, ArtifactLink,
};

#[test]
fn test_refinement_manager() {
    let temp_dir = tempfile::tempdir().unwrap();
    let manager = RefinementManager::new(temp_dir.path()).unwrap();

    // Create refinement
    let event = RefinementEvent::new(
        RefinementType::Approve,
        "Merge proposal".to_string(),
        "All tests pass".to_string(),
        "user@example.com".to_string(),
    );

    let event_id = manager.create_refinement(event).unwrap();
    assert!(!event_id.is_empty());

    // Get all refinements
    let refinements = manager.get_all_refinements().unwrap();
    assert_eq!(refinements.len(), 1);
}

#[test]
fn test_linked_artifacts() {
    let linked = LinkedArtifacts::new("refinement-1".to_string());

    let linked = linked.add_link(ArtifactLink {
        artifact_id: "proposal-1".to_string(),
        artifact_type: "proposal".to_string(),
        link_type: "approves".to_string(),
    });

    let approves = linked.get_links_by_type("approves");
    assert_eq!(approves.len(), 1);
    assert_eq!(approves[0].artifact_id, "proposal-1");
}
```

---

### Step 6: Update automation module to include refinement

**Files:** Modify `automation/mod.rs`

```rust
pub mod cron;
pub mod experiment;
pub mod merge;
pub mod refinement;

pub use cron::{
    CronScheduler, CronSchedulerInstance, CronExpression, CronParseError,
    ScheduleSafetyConfig, ScheduleValidator, ValidationError,
};
pub use experiment::{
    ExperimentConfig, ExperimentManager, ExperimentManifest, ExperimentResult,
    GitExperimentManager, IsolationEnforcer, MergePolicy, MergePolicyType,
};
pub use merge::{
    MergeProposalGenerator, ProposalArtifactManager, ValidationCriteria,
    MergeValidator, ConfidenceScorer, ConfidenceLevel, ProposalMetadata,
    ProposalStatus, PROPOSAL_ARTIFACT_DIR,
};
pub use refinement::{
    RefinementManager, RefinementEvent, RefinementType, RefinementStatus,
    LinkedArtifacts, ArtifactLink, REFINEMENT_ARTIFACT_DIR,
};
```

---

### Step 7: Write documentation

**Files:** Create `automation/refinement/README.md`

```markdown
# Manual Refinement Capture

The refinement capture system records all manual changes (approvals, rejections,
refinements) as immutable events in `.glyphnova/refinements/`.

## Features

- Event capture (what, why, who, when)
- Artifact linking
- Immutable event log
- Approval/rejection workflows
- Rollback support via event reversal

## ADR-0007 Compliance

- All manual refinements preserved as artifacted events
- Complete audit trail with linking
- Immutable event log

## Usage

```rust
use agentsdk::automation::refinement::{RefinementManager, RefinementEvent, RefinementType};

let manager = RefinementManager::new("/path/to/repo")?;

// Create refinement event
let event = RefinementEvent::new(
    RefinementType::Approve,
    "Merge proposal".to_string(),
    "All tests pass".to_string(),
    "user@example.com".to_string(),
);

let event_id = manager.create_refinement(event)?;

// Get all refinements
let refinements = manager.get_all_refinements()?;
```
```

---

### Step 8: Commit all changes

**Files:** Commit

```bash
git add automation/refinement tests/integration/refinement_capture_test.rs automation/mod.rs
git commit -m "feat(automation): implement manual refinement capture (Task 03)

- Add refinement event types (approve/reject/refine/comment)
- Implement event capture with immutable event log
- Add artifact linking for complete audit trails
- Support approval/rejection workflows
- Write comprehensive unit and integration tests
- Follow ADR-0007: all manual refinements preserved as artifacted events

Refs: Phase 6, Task 03"
```

---

## Summary

Task 03 implements manual refinement capture with:

✅ Refinement event types (approve/reject/refine/comment)
✅ Event capture with immutable event log
✅ Artifact linking for audit trails
✅ Approval/rejection workflows
✅ Unit and integration tests
✅ ADR-0007 compliance (all refinements artifacted)

**Next Steps:** Task 04 (Experiment Result Tracking) or Task 07 (Automation CLI)
