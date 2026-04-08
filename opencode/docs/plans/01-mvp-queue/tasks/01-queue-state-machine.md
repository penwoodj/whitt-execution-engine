# Task 01: Queue State Machine

**Goal:** Implement queue state machine with all 8 states, valid transitions, event validation, and state change notifications.

**Files:**
- Create: `src/queue/state.rs`
- Create: `src/queue/job.rs`
- Create: `src/queue/mod.rs`
- Create: `tests/unit/queue_test.rs`

---

## Rust Definitions

### `src/queue/state.rs`

```rust
use serde::{Deserialize, Serialize};
use std::fmt;

/// Queue job states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QueueState {
    /// Job created, waiting for scheduler
    Pending,
    /// Job assigned to worker, waiting for execution slot
    Scheduled,
    /// Job actively executing steps
    Running,
    /// Job paused by user or system (can resume)
    Paused,
    /// Job finished successfully
    Completed,
    /// Job failed after retry attempts exhausted
    Failed,
    /// Job cancelled by user
    Cancelled,
    /// Job exceeded maximum execution time
    Timeout,
}

impl QueueState {
    /// Check if transition to new_state is valid
    pub fn can_transition_to(&self, new_state: &QueueState) -> bool {
        match (self, new_state) {
            // Pending can become Scheduled or Cancelled
            (QueueState::Pending, QueueState::Scheduled) => true,
            (QueueState::Pending, QueueState::Cancelled) => true,

            // Scheduled can become Running, Cancelled, or Timeout
            (QueueState::Scheduled, QueueState::Running) => true,
            (QueueState::Scheduled, QueueState::Cancelled) => true,
            (QueueState::Scheduled, QueueState::Timeout) => true,

            // Running can become Paused, Completed, Failed, Cancelled, or Timeout
            (QueueState::Running, QueueState::Paused) => true,
            (QueueState::Running, QueueState::Completed) => true,
            (QueueState::Running, QueueState::Failed) => true,
            (QueueState::Running, QueueState::Cancelled) => true,
            (QueueState::Running, QueueState::Timeout) => true,

            // Paused can become Running or Cancelled
            (QueueState::Paused, QueueState::Running) => true,
            (QueueState::Paused, QueueState::Cancelled) => true,

            // Terminal states cannot transition
            _ => false,
        }
    }

    /// Check if state is a terminal state (no further transitions possible)
    pub fn is_terminal(&self) -> bool {
        matches!(self, QueueState::Completed | QueueState::Failed | QueueState::Cancelled | QueueState::Timeout)
    }

    /// Check if state is an active state (not terminal)
    pub fn is_active(&self) -> bool {
        !self.is_terminal()
    }
}

impl fmt::Display for QueueState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueueState::Pending => write!(f, "pending"),
            QueueState::Scheduled => write!(f, "scheduled"),
            QueueState::Running => write!(f, "running"),
            QueueState::Paused => write!(f, "paused"),
            QueueState::Completed => write!(f, "completed"),
            QueueState::Failed => write!(f, "failed"),
            QueueState::Cancelled => write!(f, "cancelled"),
            QueueState::Timeout => write!(f, "timeout"),
        }
    }
}

/// Events that trigger state transitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueueEvent {
    /// Job created
    Created { job_id: String },
    /// Job scheduled for execution
    Scheduled { job_id: String, worker_id: String },
    /// Job started execution
    Started { job_id: String },
    /// Job paused
    Paused { job_id: String, reason: String },
    /// Job resumed
    Resumed { job_id: String },
    /// Job completed successfully
    Completed { job_id: String, outputs: serde_json::Value },
    /// Job failed
    Failed { job_id: String, error: String },
    /// Job cancelled
    Cancelled { job_id: String, reason: String },
    /// Job timed out
    Timeout { job_id: String, elapsed_secs: u64 },
}

impl QueueEvent {
    pub fn job_id(&self) -> &str {
        match self {
            QueueEvent::Created { job_id } => job_id,
            QueueEvent::Scheduled { job_id, .. } => job_id,
            QueueEvent::Started { job_id } => job_id,
            QueueEvent::Paused { job_id, .. } => job_id,
            QueueEvent::Resumed { job_id } => job_id,
            QueueEvent::Completed { job_id, .. } => job_id,
            QueueEvent::Failed { job_id, .. } => job_id,
            QueueEvent::Cancelled { job_id, .. } => job_id,
            QueueEvent::Timeout { job_id, .. } => job_id,
        }
    }

    /// Get the expected state transition for this event
    pub fn expected_transition(&self, from: QueueState) -> Result<QueueState, String> {
        match (self, from) {
            (QueueEvent::Created { .. }, QueueState::Pending) => Ok(QueueState::Pending),
            (QueueEvent::Scheduled { .. }, QueueState::Pending) => Ok(QueueState::Scheduled),
            (QueueEvent::Started { .. }, QueueState::Scheduled) => Ok(QueueState::Running),
            (QueueEvent::Paused { .. }, QueueState::Running) => Ok(QueueState::Paused),
            (QueueEvent::Resumed { .. }, QueueState::Paused) => Ok(QueueState::Running),
            (QueueEvent::Completed { .. }, QueueState::Running) => Ok(QueueState::Completed),
            (QueueEvent::Failed { .. }, QueueState::Running) => Ok(QueueState::Failed),
            (QueueEvent::Cancelled { .. }, QueueState::Pending) => Ok(QueueState::Cancelled),
            (QueueEvent::Cancelled { .. }, QueueState::Scheduled) => Ok(QueueState::Cancelled),
            (QueueEvent::Cancelled { .. }, QueueState::Running) => Ok(QueueState::Cancelled),
            (QueueEvent::Cancelled { .. }, QueueState::Paused) => Ok(QueueState::Cancelled),
            (QueueEvent::Timeout { .. }, QueueState::Scheduled) => Ok(QueueState::Timeout),
            (QueueEvent::Timeout { .. }, QueueState::Running) => Ok(QueueState::Timeout),
            _ => Err(format!("Invalid transition: {:?} with event {:?}", from, self)),
        }
    }
}

/// State change notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateChangeNotification {
    pub job_id: String,
    pub old_state: QueueState,
    pub new_state: QueueState,
    pub event: QueueEvent,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl StateChangeNotification {
    pub fn new(job_id: String, old_state: QueueState, new_state: QueueState, event: QueueEvent) -> Self {
        Self {
            job_id,
            old_state,
            new_state,
            event,
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Error types for state machine
#[derive(Debug, thiserror::Error)]
pub enum StateMachineError {
    #[error("Invalid state transition: {0:?} -> {1:?}")]
    InvalidTransition(QueueState, QueueState),

    #[error("Invalid event for current state: {event:?} in state {state:?}")]
    InvalidEvent { state: QueueState, event: QueueEvent },

    #[error("Job not found: {0}")]
    JobNotFound(String),
}
```

### `src/queue/job.rs`

```rust
use crate::chat_session::SessionId;
use crate::queue::state::{QueueEvent, QueueState, StateMachineError, StateChangeNotification};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Unique identifier for a job
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct JobId(pub String);

impl JobId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Job priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// Execution statistics for a job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobStats {
    pub created_at: DateTime<Utc>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub execution_duration_secs: Option<u64>,
    pub retry_count: u32,
    pub steps_executed: u32,
    pub steps_total: u32,
}

/// Queue job representing a workflow execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: JobId,
    pub session_id: SessionId,
    pub workflow_id: String,
    pub state: QueueState,
    pub priority: Priority,
    pub inputs: HashMap<String, serde_json::Value>,
    pub outputs: Option<HashMap<String, serde_json::Value>>,
    pub error: Option<String>,
    pub stats: JobStats,
    pub worker_id: Option<String>,
    pub tags: Vec<String>,
}

impl Job {
    pub fn new(
        session_id: SessionId,
        workflow_id: String,
        inputs: HashMap<String, serde_json::Value>,
        steps_total: u32,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: JobId::new(),
            session_id,
            workflow_id,
            state: QueueState::Pending,
            priority: Priority::Normal,
            inputs,
            outputs: None,
            error: None,
            stats: JobStats {
                created_at: now,
                scheduled_at: None,
                started_at: None,
                completed_at: None,
                execution_duration_secs: None,
                retry_count: 0,
                steps_executed: 0,
                steps_total,
            },
            worker_id: None,
            tags: vec![],
        }
    }

    /// Apply a state transition with validation
    pub fn transition(&mut self, event: QueueEvent) -> Result<StateChangeNotification, StateMachineError> {
        let old_state = self.state;
        let new_state = event.expected_transition(old_state)?;

        if !old_state.can_transition_to(&new_state) {
            return Err(StateMachineError::InvalidTransition(old_state, new_state));
        }

        self.state = new_state;
        self.update_stats(&event);

        Ok(StateChangeNotification::new(
            self.id.as_str().to_string(),
            old_state,
            new_state,
            event,
        ))
    }

    /// Update job statistics based on event
    fn update_stats(&mut self, event: &QueueEvent) {
        let now = Utc::now();
        match event {
            QueueEvent::Scheduled { worker_id, .. } => {
                self.stats.scheduled_at = Some(now);
                self.worker_id = Some(worker_id.clone());
            }
            QueueEvent::Started { .. } => {
                self.stats.started_at = Some(now);
            }
            QueueEvent::Completed { .. } | QueueEvent::Failed { .. } | QueueEvent::Cancelled { .. } | QueueEvent::Timeout { .. } => {
                self.stats.completed_at = Some(now);
                if let Some(started) = self.stats.started_at {
                    self.stats.execution_duration_secs = Some((now - started).num_seconds().max(0) as u64);
                }
            }
            _ => {}
        }
    }

    /// Increment step execution count
    pub fn increment_step_count(&mut self) {
        self.stats.steps_executed += 1;
    }

    /// Increment retry count
    pub fn increment_retry_count(&mut self) {
        self.stats.retry_count += 1;
    }

    /// Check if job has exceeded time limit
    pub fn is_timed_out(&self, max_duration_secs: u64) -> bool {
        if let Some(started) = self.stats.started_at {
            let elapsed = (Utc::now() - started).num_seconds().max(0) as u64;
            elapsed >= max_duration_secs
        } else {
            false
        }
    }

    /// Check if job has exceeded retry limit
    pub fn has_exceeded_retries(&self, max_retries: u32) -> bool {
        self.stats.retry_count >= max_retries
    }
}
```

### `src/queue/mod.rs`

```rust
pub mod job;
pub mod state;

pub use job::{Job, JobId, JobStats, Priority};
pub use state::{QueueEvent, QueueState, StateChangeNotification, StateMachineError};
```

---

## Implementation Steps

- [ ] **Step 1: Create test file with failing tests**

```rust
// tests/unit/queue_test.rs
use agentsdk::queue::{Job, JobId, Priority, QueueEvent, QueueState};

#[test]
fn test_state_transitions() {
    assert!(QueueState::Pending.can_transition_to(&QueueState::Scheduled));
    assert!(QueueState::Scheduled.can_transition_to(&QueueState::Running));
    assert!(QueueState::Running.can_transition_to(&QueueState::Paused));
    assert!(QueueState::Paused.can_transition_to(&QueueState::Running));
    assert!(QueueState::Running.can_transition_to(&QueueState::Completed));
    assert!(!QueueState::Pending.can_transition_to(&QueueState::Completed));
}

#[test]
fn test_terminal_states() {
    assert!(QueueState::Completed.is_terminal());
    assert!(QueueState::Failed.is_terminal());
    assert!(QueueState::Cancelled.is_terminal());
    assert!(QueueState::Timeout.is_terminal());
    assert!(!QueueState::Running.is_terminal());
}

#[test]
fn test_job_creation() {
    let session_id = agentsdk::chat_session::SessionId::new();
    let job = Job::new(
        session_id,
        "workflow-123".to_string(),
        std::collections::HashMap::new(),
        5,
    );

    assert_eq!(job.state, QueueState::Pending);
    assert_eq!(job.workflow_id, "workflow-123");
    assert_eq!(job.stats.steps_total, 5);
}

#[test]
fn test_job_transition() {
    let session_id = agentsdk::chat_session::SessionId::new();
    let mut job = Job::new(
        session_id,
        "workflow-123".to_string(),
        std::collections::HashMap::new(),
        1,
    );

    let notification = job
        .transition(QueueEvent::Scheduled {
            job_id: job.id.as_str().to_string(),
            worker_id: "worker-1".to_string(),
        })
        .unwrap();

    assert_eq!(job.state, QueueState::Scheduled);
    assert_eq!(job.worker_id, Some("worker-1".to_string()));
    assert_eq!(notification.old_state, QueueState::Pending);
    assert_eq!(notification.new_state, QueueState::Scheduled);
}

#[test]
fn test_invalid_transition() {
    let session_id = agentsdk::chat_session::SessionId::new();
    let mut job = Job::new(
        session_id,
        "workflow-123".to_string(),
        std::collections::HashMap::new(),
        1,
    );

    let result = job.transition(QueueEvent::Completed {
        job_id: job.id.as_str().to_string(),
        outputs: serde_json::json!({}),
    });

    assert!(result.is_err());
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test queue --lib`
Expected: FAIL with compilation errors (modules don't exist yet)

- [ ] **Step 3: Implement state.rs**

```rust
// src/queue/state.rs (complete code from above)
```

- [ ] **Step 4: Implement job.rs**

```rust
// src/queue/job.rs (complete code from above)
```

- [ ] **Step 5: Implement mod.rs**

```rust
// src/queue/mod.rs (complete code from above)
```

- [ ] **Step 6: Add queue module to lib.rs**

```rust
// src/lib.rs
pub mod chat_session;
pub mod queue;
```

- [ ] **Step 7: Run tests to verify they pass**

Run: `cargo test queue --lib`
Expected: PASS for all tests

- [ ] **Step 8: Commit**

```bash
git add src/queue/ tests/unit/queue_test.rs
git commit -m "feat: implement queue state machine with 8 states and valid transitions"
```

---

## Integration Notes

This module provides the state machine foundation for the queue:
- Used by scheduler to manage job lifecycle
- StateChangeNotification sent through tokio channels for event broadcasting
- Job metadata used for prioritization and scheduling decisions
- Stats tracked for monitoring and retry logic
