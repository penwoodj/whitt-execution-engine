# Task 00: ChatSession Containers

**Goal:** Implement ChatSession work containers that serve as the foundation for all workflow execution, providing persistent ID generation, lifecycle management, and metadata tracking.

**Files:**
- Create: `src/chat_session/mod.rs`
- Create: `src/chat_session/manager.rs`
- Create: `src/chat_session/types.rs`
- Create: `tests/unit/chat_session_test.rs`

---

## Rust Definitions

### `src/chat_session/types.rs`

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Unique identifier for a ChatSession
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(Uuid);

impl SessionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }

    pub fn as_str(&self) -> &str {
        self.0.to_hyphenated().to_string().leak()
    }
}

/// Lifecycle state of a ChatSession
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionState {
    Created,
    Active,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

impl SessionState {
    pub fn can_transition_to(&self, new_state: &SessionState) -> bool {
        match (self, new_state) {
            (SessionState::Created, SessionState::Active) => true,
            (SessionState::Active, SessionState::Paused) => true,
            (SessionState::Active, SessionState::Completed) => true,
            (SessionState::Active, SessionState::Failed) => true,
            (SessionState::Active, SessionState::Cancelled) => true,
            (SessionState::Paused, SessionState::Active) => true,
            (SessionState::Paused, SessionState::Cancelled) => true,
            _ => false,
        }
    }
}

/// Metadata for a ChatSession
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMetadata {
    pub name: String,
    pub description: Option<String>,
    pub workflow_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: String,
    pub tags: Vec<String>,
}

/// Human interaction event (confirmations, approvals, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanInteraction {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub interaction_type: InteractionType,
    pub operation_id: Option<String>,
    pub decision: InteractionDecision,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionType {
    Confirmation,
    Approval,
    Rejection,
    Pause,
    Resume,
    Cancel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionDecision {
    Approved,
    Rejected,
    Deferred,
}

/// Execution checkpoint for resumption after failure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionCheckpoint {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub step_id: String,
    pub checkpoint_data: serde_json::Value,
    pub can_resume: bool,
}

/// ChatSession - the primary work container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: SessionId,
    pub state: SessionState,
    pub metadata: SessionMetadata,
    pub inputs: HashMap<String, serde_json::Value>,
    pub outputs: HashMap<String, serde_json::Value>,
    pub interactions: Vec<HumanInteraction>,
    pub checkpoints: Vec<ExecutionCheckpoint>,
    pub execution_log: Vec<LogEntry>,
}

/// Log entry within a session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub message: String,
    pub context: Option<HashMap<String, serde_json::Value>>,
}
```

### `src/chat_session/manager.rs`

```rust
use crate::chat_session::types::*;
use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SessionManagerError {
    #[error("Session not found: {0}")]
    SessionNotFound(SessionId),

    #[error("Invalid state transition: {0:?} -> {1:?}")]
    InvalidStateTransition(SessionState, SessionState),

    #[error("Session storage error: {0}")]
    StorageError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, SessionManagerError>;

#[async_trait]
pub trait SessionStorage: Send + Sync {
    async fn save_session(&self, session: &ChatSession) -> Result<()>;
    async fn load_session(&self, id: SessionId) -> Result<Option<ChatSession>>;
    async fn delete_session(&self, id: SessionId) -> Result<()>;
    async fn list_sessions(&self) -> Result<Vec<ChatSession>>;
}

/// Manages ChatSession lifecycle
pub struct SessionManager<S: SessionStorage> {
    storage: S,
}

impl<S: SessionStorage> SessionManager<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }

    /// Create a new ChatSession
    pub async fn create_session(
        &self,
        name: String,
        description: Option<String>,
        workflow_id: String,
        created_by: String,
        tags: Vec<String>,
        inputs: HashMap<String, serde_json::Value>,
    ) -> Result<ChatSession> {
        let now = Utc::now();
        let session = ChatSession {
            id: SessionId::new(),
            state: SessionState::Created,
            metadata: SessionMetadata {
                name,
                description,
                workflow_id,
                created_at: now,
                updated_at: now,
                created_by,
                tags,
            },
            inputs,
            outputs: HashMap::new(),
            interactions: Vec::new(),
            checkpoints: Vec::new(),
            execution_log: Vec::new(),
        };

        self.storage.save_session(&session).await?;
        Ok(session)
    }

    /// Activate a session (Created -> Active)
    pub async fn activate_session(&self, id: SessionId) -> Result<ChatSession> {
        let mut session = self.load_session(id).await?;
        self.transition_state(&mut session, SessionState::Active)?;
        session.metadata.updated_at = Utc::now();
        self.storage.save_session(&session).await?;
        Ok(session)
    }

    /// Pause a session (Active -> Paused)
    pub async fn pause_session(&self, id: SessionId) -> Result<ChatSession> {
        let mut session = self.load_session(id).await?;
        self.transition_state(&mut session, SessionState::Paused)?;
        session.metadata.updated_at = Utc::now();
        self.storage.save_session(&session).await?;
        Ok(session)
    }

    /// Resume a session (Paused -> Active)
    pub async fn resume_session(&self, id: SessionId) -> Result<ChatSession> {
        let mut session = self.load_session(id).await?;
        self.transition_state(&mut session, SessionState::Active)?;
        session.metadata.updated_at = Utc::now();
        self.storage.save_session(&session).await?;
        Ok(session)
    }

    /// Complete a session (Active -> Completed)
    pub async fn complete_session(&self, id: SessionId, outputs: HashMap<String, serde_json::Value>) -> Result<ChatSession> {
        let mut session = self.load_session(id).await?;
        self.transition_state(&mut session, SessionState::Completed)?;
        session.outputs = outputs;
        session.metadata.updated_at = Utc::now();
        self.storage.save_session(&session).await?;
        Ok(session)
    }

    /// Fail a session (Active -> Failed)
    pub async fn fail_session(&self, id: SessionId, error: String) -> Result<ChatSession> {
        let mut session = self.load_session(id).await?;
        self.transition_state(&mut session, SessionState::Failed)?;
        self.log(&mut session, "error", &format!("Session failed: {}", error), None);
        session.metadata.updated_at = Utc::now();
        self.storage.save_session(&session).await?;
        Ok(session)
    }

    /// Cancel a session (Active/Paused -> Cancelled)
    pub async fn cancel_session(&self, id: SessionId) -> Result<ChatSession> {
        let mut session = self.load_session(id).await?;
        self.transition_state(&mut session, SessionState::Cancelled)?;
        session.metadata.updated_at = Utc::now();
        self.storage.save_session(&session).await?;
        Ok(session)
    }

    /// Add human interaction to session
    pub async fn add_interaction(&self, id: SessionId, interaction: HumanInteraction) -> Result<ChatSession> {
        let mut session = self.load_session(id).await?;
        session.interactions.push(interaction);
        session.metadata.updated_at = Utc::now();
        self.storage.save_session(&session).await?;
        Ok(session)
    }

    /// Add checkpoint to session
    pub async fn add_checkpoint(&self, id: SessionId, checkpoint: ExecutionCheckpoint) -> Result<ChatSession> {
        let mut session = self.load_session(id).await?;
        session.checkpoints.push(checkpoint);
        session.metadata.updated_at = Utc::now();
        self.storage.save_session(&session).await?;
        Ok(session)
    }

    /// Add log entry to session
    pub async fn add_log_entry(
        &self,
        id: SessionId,
        level: String,
        message: String,
        context: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<ChatSession> {
        let mut session = self.load_session(id).await?;
        self.log(&mut session, &level, &message, context);
        session.metadata.updated_at = Utc::now();
        self.storage.save_session(&session).await?;
        Ok(session)
    }

    /// Load a session by ID
    pub async fn get_session(&self, id: SessionId) -> Result<ChatSession> {
        self.load_session(id).await
    }

    /// List all sessions
    pub async fn list_sessions(&self) -> Result<Vec<ChatSession>> {
        self.storage.list_sessions().await
    }

    /// Delete a session
    pub async fn delete_session(&self, id: SessionId) -> Result<()> {
        self.storage.delete_session(id).await
    }

    // Internal helper methods

    async fn load_session(&self, id: SessionId) -> Result<ChatSession> {
        self.storage
            .load_session(id)
            .await?
            .ok_or(SessionManagerError::SessionNotFound(id))
    }

    fn transition_state(&self, session: &mut ChatSession, new_state: SessionState) -> Result<()> {
        if session.state.can_transition_to(&new_state) {
            session.state = new_state;
            Ok(())
        } else {
            Err(SessionManagerError::InvalidStateTransition(session.state, new_state))
        }
    }

    fn log(
        &self,
        session: &mut ChatSession,
        level: &str,
        message: &str,
        context: Option<HashMap<String, serde_json::Value>>,
    ) {
        session.execution_log.push(LogEntry {
            timestamp: Utc::now(),
            level: level.to_string(),
            message: message.to_string(),
            context,
        });
    }
}
```

### `src/chat_session/mod.rs`

```rust
pub mod manager;
pub mod types;

pub use manager::{SessionManager, SessionManagerError, SessionStorage};
pub use types::*;
```

---

## Implementation Steps

- [ ] **Step 1: Create test file with failing tests**

```rust
// tests/unit/chat_session_test.rs
use agentsdk::chat_session::{SessionManager, SessionId, SessionState, SessionStorage};
use std::collections::HashMap;
use async_trait::async_trait;

struct MockStorage;
#[async_trait]
impl SessionStorage for MockStorage {
    async fn save_session(&self, _session: &agentsdk::chat_session::ChatSession) -> agentsdk::chat_session::Result<()> { Ok(()) }
    async fn load_session(&self, _id: agentsdk::chat_session::SessionId) -> agentsdk::chat_session::Result<Option<agentsdk::chat_session::ChatSession>> { Ok(None) }
    async fn delete_session(&self, _id: agentsdk::chat_session::SessionId) -> agentsdk::chat_session::Result<()> { Ok(()) }
    async fn list_sessions(&self) -> agentsdk::chat_session::Result<Vec<agentsdk::chat_session::ChatSession>> { Ok(vec![]) }
}

#[tokio::test]
async fn test_session_id_generation() {
    let id = SessionId::new();
    let id_str = id.as_str();
    assert!(uuid::Uuid::parse_str(id_str).is_ok());
}

#[tokio::test]
async fn test_state_transitions() {
    assert!(SessionState::Created.can_transition_to(&SessionState::Active));
    assert!(SessionState::Active.can_transition_to(&SessionState::Paused));
    assert!(!SessionState::Created.can_transition_to(&SessionState::Completed));
}

#[tokio::test]
async fn test_create_session() {
    let storage = MockStorage;
    let manager = SessionManager::new(storage);
    let session = manager.create_session(
        "test-session".to_string(),
        Some("test description".to_string()),
        "workflow-123".to_string(),
        "user-1".to_string(),
        vec!["test".to_string()],
        HashMap::new(),
    ).await.unwrap();

    assert_eq!(session.state, SessionState::Created);
    assert_eq!(session.metadata.name, "test-session");
    assert!(!session.id.as_str().is_empty());
}

#[tokio::test]
async fn test_activate_session() {
    let storage = MockStorage;
    let manager = SessionManager::new(storage);
    let session = manager.create_session(
        "test-session".to_string(),
        None,
        "workflow-123".to_string(),
        "user-1".to_string(),
        vec![],
        HashMap::new(),
    ).await.unwrap();

    let session_id = session.id;
    // Note: This test will need proper storage mock to work
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test chat_session --lib`
Expected: FAIL with compilation errors (modules don't exist yet)

- [ ] **Step 3: Implement types.rs**

```rust
// src/chat_session/types.rs (complete code from above)
```

- [ ] **Step 4: Implement manager.rs**

```rust
// src/chat_session/manager.rs (complete code from above)
```

- [ ] **Step 5: Implement mod.rs**

```rust
// src/chat_session/mod.rs (complete code from above)
```

- [ ] **Step 6: Add chat_session module to lib.rs**

```rust
// src/lib.rs
pub mod chat_session;
```

- [ ] **Step 7: Implement proper storage mock for tests**

```rust
// tests/unit/chat_session_test.rs
use agentsdk::chat_session::{ChatSession, SessionId, SessionManager, SessionStorage, SessionState};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;

struct InMemoryStorage {
    sessions: Arc<Mutex<HashMap<SessionId, ChatSession>>>,
}

impl InMemoryStorage {
    fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl SessionStorage for InMemoryStorage {
    async fn save_session(&self, session: &ChatSession) -> agentsdk::chat_session::Result<()> {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(session.id, session.clone());
        Ok(())
    }

    async fn load_session(&self, id: SessionId) -> agentsdk::chat_session::Result<Option<ChatSession>> {
        let sessions = self.sessions.lock().unwrap();
        Ok(sessions.get(&id).cloned())
    }

    async fn delete_session(&self, id: SessionId) -> agentsdk::chat_session::Result<()> {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.remove(&id);
        Ok(())
    }

    async fn list_sessions(&self) -> agentsdk::chat_session::Result<Vec<ChatSession>> {
        let sessions = self.sessions.lock().unwrap();
        Ok(sessions.values().cloned().collect())
    }
}
```

- [ ] **Step 8: Run tests to verify they pass**

Run: `cargo test chat_session --lib`
Expected: PASS for all tests

- [ ] **Step 9: Commit**

```bash
git add src/chat_session/ tests/unit/chat_session_test.rs
git commit -m "feat: implement ChatSession containers with lifecycle management"
```

---

## Integration Notes

This module provides the foundation for all workflow execution:
- Used by queue to store job execution context
- Used by executor to track step execution
- Used by human gating to store interaction history
- Used by checkpoint system for resumption

The SessionStorage trait will be implemented using sled in Task 02.

---

## QA Cross-References

- **QA Criteria**: [QA-01-01](../../qa/phase-01/QA-CRITERIA.md)
- **Test Cases**: [P01-001](../../qa/phase-01/QA-TEST-CASES.md), [P01-002](../../qa/phase-01/QA-TEST-CASES.md)
- **Schema Ref**: N/A (execution engine component)
