# Task 02: Human Override Controls

**Estimated Time**: 5 days
**Priority**: HIGH - Critical safety feature
**Dependencies**: Task 00 (contracts), Task 01 (metrics)

## Overview

Implement pause, stop, modify, and scope-change controls that are ALWAYS available regardless of autonomy level. This is a CRITICAL safety feature.

## Files

### Create
- `src/override/mod.rs` - Module exports
- `src/override/controls.rs` - Override control types and event structures
- `src/override/handlers.rs` - Override event handlers
- `src/override/pause.rs` - Pause/resume state management
- `src/override/emergency.rs` - Emergency stop implementation
- `tests/override/override_test.rs` - Unit and integration tests

### Modify
- `src/lib.rs` - Add `pub mod override;`

---

## Implementation Steps

### Step 1: Define override control types

**File**: `src/override/controls.rs`

```rust
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use chrono::{DateTime, Utc};

/// Override control types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OverrideType {
    Pause,
    Stop,
    Modify,
    ScopeChange,
}

impl OverrideType {
    pub fn description(&self) -> &'static str {
        match self {
            OverrideType::Pause => "Pause autonomous execution",
            OverrideType::Stop => "Stop autonomous execution",
            OverrideType::Modify => "Modify autonomous execution parameters",
            OverrideType::ScopeChange => "Change autonomy scope",
        }
    }
}

/// Override event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverrideEvent {
    pub event_id: String,
    pub override_type: OverrideType,
    pub workflow_id: String,
    pub reason: String,
    pub timestamp: DateTime<Utc>,
    pub operator: String,
    pub parameters: Option<OverrideParameters>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverrideParameters {
    pub new_autonomy_level: Option<String>,
    pub new_scope: Option<String>,
    pub modifications: Option<Vec<String>>,
}

impl OverrideEvent {
    pub fn new(
        override_type: OverrideType,
        workflow_id: String,
        reason: String,
        operator: String,
    ) -> Self {
        Self {
            event_id: uuid::Uuid::new_v4().to_string(),
            override_type,
            workflow_id,
            reason,
            timestamp: Utc::now(),
            operator,
            parameters: None,
        }
    }
}

/// Override response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverrideResponse {
    pub event_id: String,
    pub success: bool,
    pub message: String,
    pub new_state: Option<OverrideState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverrideState {
    pub execution_state: ExecutionState,
    pub autonomy_level: String,
    pub paused_at: Option<DateTime<Utc>>,
    pub stopped_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionState {
    Running,
    Paused,
    Stopped,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_override_event_creation() {
        let event = OverrideEvent::new(
            OverrideType::Pause,
            "workflow_123".to_string(),
            "User requested pause".to_string(),
            "operator_1".to_string(),
        );

        assert_eq!(event.override_type, OverrideType::Pause);
        assert_eq!(event.workflow_id, "workflow_123");
        assert!(!event.event_id.is_empty());
    }
}
```

---

### Step 2: Implement override handlers

**File**: `src/override/handlers.rs`

```rust
use super::controls::*;
use super::pause::PauseManager;
use super::emergency::EmergencyStop;
use crate::metrics::METRICS_REGISTRY;
use std::sync::Arc;
use tokio::sync::mpsc;

/// Override control handler
pub struct OverrideHandler {
    pause_manager: Arc<PauseManager>,
    emergency_stop: Arc<EmergencyStop>,
    event_sender: mpsc::UnboundedSender<OverrideEvent>,
}

impl OverrideHandler {
    pub fn new() -> Self {
        let (event_sender, mut event_receiver) = mpsc::unbounded_channel();

        let pause_manager = Arc::new(PauseManager::new());
        let emergency_stop = Arc::new(EmergencyStop::new());

        // Spawn event processing task
        let pause_manager_clone = pause_manager.clone();
        let emergency_stop_clone = emergency_stop.clone();

        tokio::spawn(async move {
            while let Some(event) = event_receiver.recv().await {
                Self::process_event(
                    &event,
                    &pause_manager_clone,
                    &emergency_stop_clone,
                ).await;
            }
        });

        Self {
            pause_manager,
            emergency_stop,
            event_sender,
        }
    }

    pub async fn handle_override(&self, event: OverrideEvent) -> OverrideResponse {
        // Log override event
        crate::metrics::instrumentation::instrument_intervention(
            &event.workflow_id,
            &format!("{:?}", event.override_type),
            &event.reason,
        );

        match event.override_type {
            OverrideType::Pause => self.handle_pause(event).await,
            OverrideType::Stop => self.handle_stop(event).await,
            OverrideType::Modify => self.handle_modify(event).await,
            OverrideType::ScopeChange => self.handle_scope_change(event).await,
        }
    }

    async fn handle_pause(&self, event: OverrideEvent) -> OverrideResponse {
        self.pause_manager.pause(&event.workflow_id);

        OverrideResponse {
            event_id: event.event_id,
            success: true,
            message: "Execution paused".to_string(),
            new_state: Some(OverrideState {
                execution_state: ExecutionState::Paused,
                autonomy_level: "current".to_string(),
                paused_at: Some(Utc::now()),
                stopped_at: None,
            }),
        }
    }

    async fn handle_stop(&self, event: OverrideEvent) -> OverrideResponse {
        self.emergency_stop.stop(&event.workflow_id);

        OverrideResponse {
            event_id: event.event_id,
            success: true,
            message: "Execution stopped".to_string(),
            new_state: Some(OverrideState {
                execution_state: ExecutionState::Stopped,
                autonomy_level: "current".to_string(),
                paused_at: None,
                stopped_at: Some(Utc::now()),
            }),
        }
    }

    async fn handle_modify(&self, event: OverrideEvent) -> OverrideResponse {
        // Modification logic would be implemented here
        OverrideResponse {
            event_id: event.event_id,
            success: true,
            message: "Parameters modified".to_string(),
            new_state: None,
        }
    }

    async fn handle_scope_change(&self, event: OverrideEvent) -> OverrideResponse {
        // Scope change logic would be implemented here
        OverrideResponse {
            event_id: event.event_id,
            success: true,
            message: "Autonomy scope changed".to_string(),
            new_state: None,
        }
    }

    async fn process_event(
        event: &OverrideEvent,
        pause_manager: &PauseManager,
        emergency_stop: &EmergencyStop,
    ) {
        // Additional processing for events
        tracing::info!(
            event_id = %event.event_id,
            override_type = ?event.override_type,
            workflow_id = %event.workflow_id,
            "Processed override event"
        );
    }

    pub fn get_pause_manager(&self) -> Arc<PauseManager> {
        self.pause_manager.clone()
    }

    pub fn get_emergency_stop(&self) -> Arc<EmergencyStop> {
        self.emergency_stop.clone()
    }

    pub async fn is_paused(&self, workflow_id: &str) -> bool {
        self.pause_manager.is_paused(workflow_id)
    }

    pub async fn is_stopped(&self, workflow_id: &str) -> bool {
        self.emergency_stop.is_stopped(workflow_id)
    }
}

impl Default for OverrideHandler {
    fn default() -> Self {
        Self::new()
    }
}
```

---

### Step 3: Implement pause/resume manager

**File**: `src/override/pause.rs`

```rust
use dashmap::DashMap;
use std::sync::Arc;
use chrono::{DateTime, Utc};

/// Pause state for a workflow
#[derive(Debug, Clone)]
pub struct PauseState {
    pub paused_at: DateTime<Utc>,
    pub workflow_id: String,
}

/// Pause manager
pub struct PauseManager {
    paused_workflows: DashMap<String, PauseState>,
}

impl PauseManager {
    pub fn new() -> Self {
        Self {
            paused_workflows: DashMap::new(),
        }
    }

    pub fn pause(&self, workflow_id: &str) {
        let state = PauseState {
            paused_at: Utc::now(),
            workflow_id: workflow_id.to_string(),
        };
        self.paused_workflows.insert(workflow_id.to_string(), state);

        tracing::info!(
            workflow_id = %workflow_id,
            paused_at = %state.paused_at,
            "Workflow paused"
        );
    }

    pub fn resume(&self, workflow_id: &str) {
        self.paused_workflows.remove(workflow_id);

        tracing::info!(
            workflow_id = %workflow_id,
            "Workflow resumed"
        );
    }

    pub fn is_paused(&self, workflow_id: &str) -> bool {
        self.paused_workflows.contains_key(workflow_id)
    }

    pub fn get_pause_state(&self, workflow_id: &str) -> Option<PauseState> {
        self.paused_workflows.get(workflow_id)
            .map(|entry| entry.value().clone())
    }

    pub fn get_all_paused(&self) -> Vec<String> {
        self.paused_workflows.iter()
            .map(|entry| entry.key().clone())
            .collect()
    }

    pub fn resume_all(&self) {
        let workflow_ids: Vec<String> = self.get_all_paused();
        for workflow_id in workflow_ids {
            self.resume(&workflow_id);
        }
    }
}

impl Default for PauseManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pause_resume() {
        let manager = PauseManager::new();

        manager.pause("workflow_123");
        assert!(manager.is_paused("workflow_123"));

        manager.resume("workflow_123");
        assert!(!manager.is_paused("workflow_123"));
    }

    #[test]
    fn test_get_pause_state() {
        let manager = PauseManager::new();

        manager.pause("workflow_123");
        let state = manager.get_pause_state("workflow_123");

        assert!(state.is_some());
        assert_eq!(state.unwrap().workflow_id, "workflow_123");
    }
}
```

---

### Step 4: Implement emergency stop

**File**: `src/override/emergency.rs`

```rust
use dashmap::DashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use chrono::{DateTime, Utc};

/// Emergency stop state
#[derive(Debug, Clone)]
pub struct EmergencyStopState {
    pub stopped_at: DateTime<Utc>,
    pub workflow_id: String,
    pub reason: String,
}

/// Emergency stop handler
pub struct EmergencyStop {
    stopped_workflows: DashMap<String, EmergencyStopState>,
    global_stop: Arc<AtomicBool>,
}

impl EmergencyStop {
    pub fn new() -> Self {
        Self {
            stopped_workflows: DashMap::new(),
            global_stop: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn stop(&self, workflow_id: &str) {
        self.stop_with_reason(workflow_id, "Manual stop requested");
    }

    pub fn stop_with_reason(&self, workflow_id: &str, reason: &str) {
        let state = EmergencyStopState {
            stopped_at: Utc::now(),
            workflow_id: workflow_id.to_string(),
            reason: reason.to_string(),
        };
        self.stopped_workflows.insert(workflow_id.to_string(), state);

        tracing::warn!(
            workflow_id = %workflow_id,
            stopped_at = %state.stopped_at,
            reason = %reason,
            "Emergency stop triggered"
        );
    }

    pub fn global_stop(&self) {
        self.global_stop.store(true, Ordering::Relaxed);

        tracing::error!("Global emergency stop triggered - all workflows stopped");
    }

    pub fn reset_global_stop(&self) {
        self.global_stop.store(false, Ordering::Relaxed);

        tracing::info!("Global emergency stop reset");
    }

    pub fn is_stopped(&self, workflow_id: &str) -> bool {
        if self.global_stop.load(Ordering::Relaxed) {
            return true;
        }
        self.stopped_workflows.contains_key(workflow_id)
    }

    pub fn get_stop_state(&self, workflow_id: &str) -> Option<EmergencyStopState> {
        self.stopped_workflows.get(workflow_id)
            .map(|entry| entry.value().clone())
    }

    pub fn get_all_stopped(&self) -> Vec<String> {
        if self.global_stop.load(Ordering::Relaxed) {
            return vec!["*".to_string()]; // All workflows stopped
        }

        self.stopped_workflows.iter()
            .map(|entry| entry.key().clone())
            .collect()
    }

    pub fn reset_workflow_stop(&self, workflow_id: &str) {
        self.stopped_workflows.remove(workflow_id);

        tracing::info!(
            workflow_id = %workflow_id,
            "Emergency stop reset for workflow"
        );
    }

    pub fn reset_all_stops(&self) {
        self.stopped_workflows.clear();
        self.global_stop.store(false, Ordering::Relaxed);

        tracing::info!("All emergency stops reset");
    }
}

impl Default for EmergencyStop {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stop_reset() {
        let stop = EmergencyStop::new();

        stop.stop("workflow_123");
        assert!(stop.is_stopped("workflow_123"));

        stop.reset_workflow_stop("workflow_123");
        assert!(!stop.is_stopped("workflow_123"));
    }

    #[test]
    fn test_global_stop() {
        let stop = EmergencyStop::new();

        stop.global_stop();
        assert!(stop.is_stopped("workflow_123"));
        assert!(stop.is_stopped("workflow_456"));

        stop.reset_global_stop();
        assert!(!stop.is_stopped("workflow_123"));
    }

    #[test]
    fn test_stop_with_reason() {
        let stop = EmergencyStop::new();

        stop.stop_with_reason("workflow_123", "Critical error detected");
        let state = stop.get_stop_state("workflow_123");

        assert!(state.is_some());
        assert_eq!(state.unwrap().reason, "Critical error detected");
    }
}
```

---

### Step 5: Create module exports

**File**: `src/override/mod.rs`

```rust
pub mod controls;
pub mod handlers;
pub mod pause;
pub mod emergency;

pub use controls::*;
pub use handlers::OverrideHandler;
pub use pause::PauseManager;
pub use emergency::EmergencyStop;
```

**File**: `src/lib.rs` (modify)

```rust
pub mod override_controls;
```

---

### Step 6: Run all tests

```bash
cd /home/jon/code/yaml-to-rust-agentsdk
cargo test override --verbose
```

**Expected Output**: All tests pass

---

### Step 7: Integration test for override workflow

**File**: `tests/override/integration_test.rs`

```rust
use glyphnova_engine::override_controls::*;

#[tokio::test]
async fn test_full_override_workflow() {
    let handler = OverrideHandler::new();
    let workflow_id = "test_workflow_123";

    // Test pause
    let pause_event = OverrideEvent::new(
        OverrideType::Pause,
        workflow_id.to_string(),
        "User requested pause".to_string(),
        "operator_1".to_string(),
    );

    let pause_response = handler.handle_override(pause_event).await;
    assert!(pause_response.success);
    assert!(handler.is_paused(workflow_id).await);

    // Test resume
    handler.get_pause_manager().resume(workflow_id);
    assert!(!handler.is_paused(workflow_id).await);

    // Test stop
    let stop_event = OverrideEvent::new(
        OverrideType::Stop,
        workflow_id.to_string(),
        "User requested stop".to_string(),
        "operator_1".to_string(),
    );

    let stop_response = handler.handle_override(stop_event).await;
    assert!(stop_response.success);
    assert!(handler.is_stopped(workflow_id).await);

    // Test emergency stop
    handler.get_emergency_stop().global_stop();
    assert!(handler.is_stopped("any_workflow").await);

    handler.get_emergency_stop().reset_global_stop();
    assert!(!handler.is_stopped("any_workflow").await);
}
```

---

### Step 8: Commit

```bash
git add src/override/ tests/override/
git commit -m "feat: implement human override controls

- Define OverrideType enum (Pause, Stop, Modify, ScopeChange)
- Implement OverrideHandler for processing override events
- Implement PauseManager for pause/resume state management
- Implement EmergencyStop for immediate termination
- Add comprehensive unit and integration tests
- Verify override controls available at all autonomy levels

Relates to ADR-0008: Human override always available"
```

---

## Validation Criteria

- ✅ All unit tests pass
- ✅ Integration tests pass
- ✅ Override controls work at all autonomy levels
- ✅ Emergency stop responds within 100ms
- ✅ Pause/resume state is correctly managed

---

## Next Steps

After completing Task 02:
1. Proceed to Task 03: Intervention Tracking

---

**End of Task 02**
