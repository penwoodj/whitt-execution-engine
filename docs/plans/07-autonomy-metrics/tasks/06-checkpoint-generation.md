# Task 06: Checkpoint Generation

**Estimated Time**: 5 days
**Priority**: HIGH - Critical for state preservation
**Dependencies**: Task 00 (contracts), Task 01 (metrics), Task 03 (intervention tracking)

## Overview

Generate periodic and event-triggered checkpoints for state preservation. Checkpoints enable recovery and provide audit trails.

## Files

### Create
- `src/checkpoint/mod.rs` - Module exports
- `src/checkpoint/structure.rs` - Checkpoint structure
- `src/checkpoint/periodic.rs` - Periodic checkpoint generation
- `src/checkpoint/triggered.rs` - Event-triggered checkpoints
- `src/checkpoint/restoration.rs` - Restoration logic
- `tests/checkpoint/checkpoint_test.rs` - Unit and integration tests

### Modify
- `src/lib.rs` - Add `pub mod checkpoint;`

---

## Implementation Steps

### Step 1: Define checkpoint structure

**File**: `src/checkpoint/structure.rs`

```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Checkpoint data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub checkpoint_id: String,
    pub workflow_id: String,
    pub timestamp: DateTime<Utc>,
    pub checkpoint_type: CheckpointType,
    pub workflow_state: WorkflowState,
    pub metrics_snapshot: MetricsSnapshot,
    pub intervention_history: Vec<String>,
    pub metadata: CheckpointMetadata,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointType {
    Periodic,
    OnIntervention,
    OnStopCondition,
    OnActionComplete,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowState {
    pub action_count: usize,
    pub current_action_id: Option<String>,
    pub pending_actions: Vec<String>,
    pub completed_actions: Vec<String>,
    pub autonomy_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    pub success_rate: f64,
    pub actions_completed: usize,
    pub actions_failed: usize,
    pub interventions_count: usize,
    pub custom_metrics: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointMetadata {
    pub size_bytes: usize,
    pub storage_location: String,
    pub created_by: String,
}
```

---

### Step 2: Implement periodic checkpoint generation

**File**: `src/checkpoint/periodic.rs`

```rust
use super::structure::*;
use std::time::Duration;
use tokio::time::interval;

pub struct PeriodicCheckpointGenerator {
    workflow_id: String,
    interval: Duration,
    checkpoint_dir: String,
}

impl PeriodicCheckpointGenerator {
    pub fn new(workflow_id: String, interval: Duration, checkpoint_dir: String) -> Self {
        Self { workflow_id, interval, checkpoint_dir }
    }

    pub async fn start<F>(&self, state_provider: F) -> Result<(), CheckpointError>
    where
        F: Fn() -> Checkpoint + Send + Sync + 'static,
    {
        let mut ticker = interval(self.interval);

        loop {
            ticker.tick().await;

            let checkpoint = state_provider();
            self.save_checkpoint(&checkpoint).await?;
        }
    }

    async fn save_checkpoint(&self, checkpoint: &Checkpoint) -> Result<(), CheckpointError> {
        let filename = format!(
            "{}_{}.json",
            checkpoint.workflow_id,
            checkpoint.timestamp.format("%Y%m%d_%H%M%S")
        );
        let filepath = std::path::PathBuf::from(&self.checkpoint_dir)
            .join("periodic")
            .join(filename);

        std::fs::create_dir_all(filepath.parent().unwrap())
            .map_err(CheckpointError::IoError)?;

        let json = serde_json::to_string_pretty(checkpoint)
            .map_err(CheckpointError::SerializationError)?;

        std::fs::write(&filepath, json)
            .map_err(CheckpointError::IoError)?;

        tracing::info!(
            checkpoint_id = %checkpoint.checkpoint_id,
            "Periodic checkpoint saved to {}",
            filepath.display()
        );

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CheckpointError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}
```

---

### Step 3: Implement event-triggered checkpoints

**File**: `src/checkpoint/triggered.rs`

```rust
use super::structure::*;
use super::periodic::*;
use tokio::sync::mpsc;

pub struct TriggeredCheckpointGenerator {
    workflow_id: String,
    checkpoint_dir: String,
}

impl TriggeredCheckpointGenerator {
    pub fn new(workflow_id: String, checkpoint_dir: String) -> Self {
        Self { workflow_id, checkpoint_dir }
    }

    pub fn create_handler(&self) -> TriggeredCheckpointHandler {
        TriggeredCheckpointHandler::new(self.workflow_id.clone(), self.checkpoint_dir.clone())
    }
}

pub struct TriggeredCheckpointHandler {
    workflow_id: String,
    checkpoint_dir: String,
    sender: mpsc::UnboundedSender<CheckpointTriggerEvent>,
}

impl TriggeredCheckpointHandler {
    pub fn new(workflow_id: String, checkpoint_dir: String) -> Self {
        let (sender, mut receiver) = mpsc::unbounded_channel();

        let workflow_id_clone = workflow_id.clone();
        let checkpoint_dir_clone = checkpoint_dir.clone();

        tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                if let Some(checkpoint) = event.checkpoint {
                    Self::save_checkpoint(&checkpoint_dir_clone, &checkpoint).await;
                }
            }
        });

        Self { workflow_id, checkpoint_dir, sender }
    }

    pub fn trigger_checkpoint(
        &self,
        checkpoint_type: CheckpointType,
        workflow_state: WorkflowState,
        metrics_snapshot: MetricsSnapshot,
        intervention_history: Vec<String>,
    ) -> Result<(), CheckpointError> {
        let checkpoint = Checkpoint {
            checkpoint_id: uuid::Uuid::new_v4().to_string(),
            workflow_id: self.workflow_id.clone(),
            timestamp: Utc::now(),
            checkpoint_type,
            workflow_state,
            metrics_snapshot,
            intervention_history,
            metadata: CheckpointMetadata {
                size_bytes: 0,
                storage_location: String::new(),
                created_by: "system".to_string(),
            },
        };

        self.sender.send(CheckpointTriggerEvent {
            checkpoint: Some(checkpoint),
            reason: String::new(),
        }).map_err(|_| CheckpointError::IoError(std::io::Error::new(
            std::io::ErrorKind::BrokenPipe,
            "Failed to send checkpoint event"
        )))?;

        Ok(())
    }

    async fn save_checkpoint(checkpoint_dir: &str, checkpoint: &Checkpoint) {
        let filename = format!(
            "{}_{}.json",
            checkpoint.workflow_id,
            checkpoint.timestamp.format("%Y%m%d_%H%M%S_%3f")
        );
        let filepath = std::path::PathBuf::from(checkpoint_dir)
            .join("triggered")
            .join(filename);

        std::fs::create_dir_all(filepath.parent().unwrap()).ok();

        if let Ok(json) = serde_json::to_string_pretty(checkpoint) {
            std::fs::write(&filepath, json).ok();
        }
    }
}

#[derive(Debug)]
struct CheckpointTriggerEvent {
    checkpoint: Option<Checkpoint>,
    reason: String,
}
```

---

### Step 4: Implement restoration logic

**File**: `src/checkpoint/restoration.rs`

```rust
use super::structure::*;
use super::periodic::*;

pub struct CheckpointRestorer {
    checkpoint_dir: String,
}

impl CheckpointRestorer {
    pub fn new(checkpoint_dir: String) -> Self {
        Self { checkpoint_dir }
    }

    pub fn list_checkpoints(&self, workflow_id: &str) -> Result<Vec<CheckpointMetadata>, CheckpointError> {
        let checkpoints_dir = std::path::PathBuf::from(&self.checkpoint_dir)
            .join("triggered");

        let mut checkpoints = Vec::new();

        if let Ok(entries) = std::fs::read_dir(checkpoints_dir) {
            for entry in entries.flatten() {
                if let Ok(path) = entry.path().into_os_string().into_string() {
                    if path.contains(workflow_id) {
                        if let Ok(content) = std::fs::read_to_string(&path) {
                            if let Ok(checkpoint) = serde_json::from_str::<Checkpoint>(&content) {
                                checkpoints.push(checkpoint.metadata);
                            }
                        }
                    }
                }
            }
        }

        Ok(checkpoints)
    }

    pub fn restore_checkpoint(&self, checkpoint_id: &str) -> Result<Checkpoint, CheckpointError> {
        let checkpoints_dir = std::path::PathBuf::from(&self.checkpoint_dir);

        // Search in both periodic and triggered directories
        for subdir in &["periodic", "triggered"] {
            let dir = checkpoints_dir.join(subdir);
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    if let Ok(path) = entry.path().into_os_string().into_string() {
                        if path.contains(checkpoint_id) {
                            let content = std::fs::read_to_string(&path)?;
                            let checkpoint: Checkpoint = serde_json::from_str(&content)?;
                            return Ok(checkpoint);
                        }
                    }
                }
            }
        }

        Err(CheckpointError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Checkpoint not found"
        )))
    }
}
```

---

### Step 5: Create module exports

**File**: `src/checkpoint/mod.rs`

```rust
pub mod structure;
pub mod periodic;
pub mod triggered;
pub mod restoration;

pub use structure::*;
pub use periodic::*;
pub use triggered::*;
pub use restoration::*;
```

**File**: `src/lib.rs` (modify)

```rust
pub mod checkpoint;
```

---

### Step 6: Run all tests

```bash
cd /home/jon/code/yaml-to-rust-agentsdk
cargo test checkpoint --verbose
```

**Expected Output**: All tests pass

---

### Step 7: Integration test

**File**: `tests/checkpoint/integration_test.rs`

```rust
use glyphnova_engine::checkpoint::*;

#[test]
fn test_checkpoint_workflow() {
    let workflow_id = "test_workflow_123";
    let checkpoint_dir = "/tmp/test_checkpoints";

    let handler = TriggeredCheckpointHandler::new(
        workflow_id.to_string(),
        checkpoint_dir.to_string(),
    );

    handler.trigger_checkpoint(
        CheckpointType::OnIntervention,
        WorkflowState {
            action_count: 10,
            current_action_id: Some("action_5".to_string()),
            pending_actions: vec!["action_6".to_string()],
            completed_actions: vec!["action_1".to_string()],
            autonomy_level: "high".to_string(),
        },
        MetricsSnapshot {
            success_rate: 0.95,
            actions_completed: 10,
            actions_failed: 0,
            interventions_count: 1,
            custom_metrics: std::collections::HashMap::new(),
        },
        vec!["intervention_1".to_string()],
    ).expect("Failed to trigger checkpoint");

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let restorer = CheckpointRestorer::new(checkpoint_dir.to_string());
    let checkpoints = restorer.list_checkpoints(workflow_id).expect("Failed to list checkpoints");

    assert!(!checkpoints.is_empty());
}
```

---

### Step 8: Commit

```bash
git add src/checkpoint/ tests/checkpoint/
git commit -m "feat: implement checkpoint generation

- Define checkpoint structure with workflow state and metrics
- Implement periodic checkpoint generation
- Implement event-triggered checkpoints
- Implement checkpoint restoration logic
- Add comprehensive unit and integration tests

Relates to ADR-0008: State preservation and audit trails"
```

---

## Validation Criteria

- ✅ All unit tests pass
- ✅ Integration tests pass
- ✅ Periodic checkpoints are generated at correct intervals
- ✅ Event-triggered checkpoints are generated on events
- ✅ Checkpoints can be restored successfully

---

## Next Steps

After completing Task 06:
1. Proceed to Task 07: Autonomy Scope & Risk

---

**End of Task 06**
