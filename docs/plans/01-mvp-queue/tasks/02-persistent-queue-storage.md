# Task 02: Persistent Queue Storage

**Goal:** Implement sled-based persistent storage for queue state, enabling recovery from process crashes and providing durable job metadata, execution history, and checkpoint data.

**Files:**
- Create: `src/queue/storage.rs`
- Modify: `src/chat_session/manager.rs` (implement SessionStorage with sled)
- Modify: `src/queue/mod.rs` (export storage)
- Create: `tests/unit/queue_storage_test.rs`

---

## Rust Definitions

### `src/queue/storage.rs`

```rust
use crate::chat_session::{ChatSession, SessionId, SessionStorage};
use crate::queue::{Job, JobId, QueueState};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sled::Db;
use std::path::Path;
use thiserror::Error;

/// Key prefixes for sled storage
const JOBS_PREFIX: &str = "jobs:";
const SESSIONS_PREFIX: &str = "sessions:";
const INDEX_PREFIX: &str = "index:";

/// Storage configuration
#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub db_path: String,
    pub cache_size: usize,
    pub flush_every_ms: Option<u64>,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            db_path: "./workspace/queue".to_string(),
            cache_size: 1024 * 1024 * 256, // 256MB
            flush_every_ms: Some(1000),
        }
    }
}

/// Persistent storage for queue state
pub struct QueueStorage {
    db: Db,
    config: StorageConfig,
}

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Sled error: {0}")]
    SledError(#[from] sled::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Job not found: {0}")]
    JobNotFound(JobId),

    #[error("Session not found: {0}")]
    SessionNotFound(SessionId),

    #[error("Index error: {0}")]
    IndexError(String),
}

pub type Result<T> = std::result::Result<T, StorageError>;

impl QueueStorage {
    /// Open or create storage
    pub fn open(config: StorageConfig) -> Result<Self> {
        let db = sled::Config::default()
            .path(&config.db_path)
            .cache_capacity(config.cache_size)
            .flush_every_ms(config.flush_every_ms)
            .open()?;

        Ok(Self { db, config })
    }

    /// Create in-memory storage for testing
    pub fn open_in_memory() -> Result<Self> {
        let db = sled::Config::default()
            .temporary(true)
            .open()?;

        Ok(Self {
            db,
            config: StorageConfig::default(),
        })
    }

    /// Generate job key
    fn job_key(job_id: &JobId) -> Vec<u8> {
        format!("{}{}", JOBS_PREFIX, job_id.as_str()).into_bytes()
    }

    /// Generate session key
    fn session_key(session_id: &SessionId) -> Vec<u8> {
        format!("{}{}", SESSIONS_PREFIX, session_id.as_str()).into_bytes()
    }

    /// Generate index key for job state queries
    fn state_index_key(state: QueueState) -> Vec<u8> {
        format!("{}state:{}", INDEX_PREFIX, state).into_bytes()
    }

    /// Generate index key for workflow queries
    fn workflow_index_key(workflow_id: &str) -> Vec<u8> {
        format!("{}workflow:{}", INDEX_PREFIX, workflow_id).into_bytes()
    }

    /// Save job
    pub fn save_job(&self, job: &Job) -> Result<()> {
        let key = Self::job_key(&job.id);
        let value = serde_json::to_vec(job)?;

        // Save job
        self.db.insert(&key, value)?;

        // Update state index
        self.update_state_index(&job.id, &job.state)?;

        // Update workflow index
        self.update_workflow_index(&job.id, &job.workflow_id)?;

        // Flush to ensure durability
        self.db.flush_async()?;

        Ok(())
    }

    /// Load job
    pub fn load_job(&self, job_id: &JobId) -> Result<Option<Job>> {
        let key = Self::job_key(job_id);
        match self.db.get(&key)? {
            Some(value) => {
                let job = serde_json::from_slice(&value)?;
                Ok(Some(job))
            }
            None => Ok(None),
        }
    }

    /// Delete job
    pub fn delete_job(&self, job_id: &JobId) -> Result<Job> {
        let job = self
            .load_job(job_id)?
            .ok_or_else(|| StorageError::JobNotFound(job_id.clone()))?;

        let key = Self::job_key(job_id);
        self.db.remove(&key)?;

        // Remove from state index
        self.remove_from_state_index(job_id, &job.state)?;

        // Remove from workflow index
        self.remove_from_workflow_index(job_id, &job.workflow_id)?;

        self.db.flush_async()?;

        Ok(job)
    }

    /// List all jobs
    pub fn list_jobs(&self) -> Result<Vec<Job>> {
        let prefix = JOBS_PREFIX.as_bytes();
        let mut jobs = Vec::new();

        for result in self.db.scan_prefix(prefix) {
            let (_key, value) = result?;
            let job = serde_json::from_slice(&value)?;
            jobs.push(job);
        }

        Ok(jobs)
    }

    /// Query jobs by state
    pub fn query_jobs_by_state(&self, state: QueueState) -> Result<Vec<Job>> {
        let index_key = Self::state_index_key(state);
        let mut jobs = Vec::new();

        if let Some(index_value) = self.db.get(&index_key)? {
            let job_ids: Vec<String> = serde_json::from_slice(&index_value)?;
            for job_id_str in job_ids {
                if let Ok(Some(job)) = self.load_job(&JobId(job_id_str)) {
                    if job.state == state {
                        jobs.push(job);
                    }
                }
            }
        }

        Ok(jobs)
    }

    /// Query jobs by workflow ID
    pub fn query_jobs_by_workflow(&self, workflow_id: &str) -> Result<Vec<Job>> {
        let index_key = Self::workflow_index_key(workflow_id);
        let mut jobs = Vec::new();

        if let Some(index_value) = self.db.get(&index_key)? {
            let job_ids: Vec<String> = serde_json::from_slice(&index_value)?;
            for job_id_str in job_ids {
                if let Ok(Some(job)) = self.load_job(&JobId(job_id_str)) {
                    if job.workflow_id == workflow_id {
                        jobs.push(job);
                    }
                }
            }
        }

        Ok(jobs)
    }

    /// Get job count
    pub fn job_count(&self) -> Result<usize> {
        let prefix = JOBS_PREFIX.as_bytes();
        let mut count = 0;

        for _ in self.db.scan_prefix(prefix) {
            count += 1;
        }

        Ok(count)
    }

    /// Update state index
    fn update_state_index(&self, job_id: &JobId, state: &QueueState) -> Result<()> {
        let index_key = Self::state_index_key(*state);
        let job_id_str = job_id.as_str().to_string();

        let mut job_ids: Vec<String> = match self.db.get(&index_key)? {
            Some(value) => serde_json::from_slice(&value)?,
            None => Vec::new(),
        };

        if !job_ids.contains(&job_id_str) {
            job_ids.push(job_id_str);
            let value = serde_json::to_vec(&job_ids)?;
            self.db.insert(&index_key, value)?;
        }

        Ok(())
    }

    /// Remove job from state index
    fn remove_from_state_index(&self, job_id: &JobId, state: &QueueState) -> Result<()> {
        let index_key = Self::state_index_key(*state);
        let job_id_str = job_id.as_str().to_string();

        if let Some(index_value) = self.db.get(&index_key)? {
            let mut job_ids: Vec<String> = serde_json::from_slice(&index_value)?;
            job_ids.retain(|id| id != &job_id_str);

            if job_ids.is_empty() {
                self.db.remove(&index_key)?;
            } else {
                let value = serde_json::to_vec(&job_ids)?;
                self.db.insert(&index_key, value)?;
            }
        }

        Ok(())
    }

    /// Update workflow index
    fn update_workflow_index(&self, job_id: &JobId, workflow_id: &str) -> Result<()> {
        let index_key = Self::workflow_index_key(workflow_id);
        let job_id_str = job_id.as_str().to_string();

        let mut job_ids: Vec<String> = match self.db.get(&index_key)? {
            Some(value) => serde_json::from_slice(&value)?,
            None => Vec::new(),
        };

        if !job_ids.contains(&job_id_str) {
            job_ids.push(job_id_str);
            let value = serde_json::to_vec(&job_ids)?;
            self.db.insert(&index_key, value)?;
        }

        Ok(())
    }

    /// Remove job from workflow index
    fn remove_from_workflow_index(&self, job_id: &JobId, workflow_id: &str) -> Result<()> {
        let index_key = Self::workflow_index_key(workflow_id);
        let job_id_str = job_id.as_str().to_string();

        if let Some(index_value) = self.db.get(&index_key)? {
            let mut job_ids: Vec<String> = serde_json::from_slice(&index_value)?;
            job_ids.retain(|id| id != &job_id_str);

            if job_ids.is_empty() {
                self.db.remove(&index_key)?;
            } else {
                let value = serde_json::to_vec(&job_ids)?;
                self.db.insert(&index_key, value)?;
            }
        }

        Ok(())
    }

    /// Recover jobs after process crash (returns jobs in Running state)
    pub fn recover_running_jobs(&self) -> Result<Vec<Job>> {
        self.query_jobs_by_state(QueueState::Running)
    }

    /// Flush all pending writes
    pub fn flush(&self) -> Result<()> {
        self.db.flush()?;
        Ok(())
    }

    /// Get database statistics
    pub fn stats(&self) -> DbStats {
        DbStats {
            size_on_disk: self.db.size_on_disk(),
            job_count: self.job_count().unwrap_or(0),
        }
    }
}

/// Database statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbStats {
    pub size_on_disk: Option<u64>,
    pub job_count: usize,
}

/// Implement SessionStorage trait for QueueStorage
#[async_trait]
impl SessionStorage for QueueStorage {
    async fn save_session(&self, session: &ChatSession) -> agentsdk::chat_session::Result<()> {
        let key = Self::session_key(&session.id);
        let value = serde_json::to_vec(session)
            .map_err(|e| agentsdk::chat_session::SessionManagerError::SerializationError(e))?;

        self.db.insert(&key, value)
            .map_err(|e| agentsdk::chat_session::SessionManagerError::StorageError(e.to_string()))?;

        self.db.flush_async()
            .map_err(|e| agentsdk::chat_session::SessionManagerError::StorageError(e.to_string()))?;

        Ok(())
    }

    async fn load_session(&self, id: SessionId) -> agentsdk::chat_session::Result<Option<ChatSession>> {
        let key = Self::session_key(&id);
        match self.db.get(&key)
            .map_err(|e| agentsdk::chat_session::SessionManagerError::StorageError(e.to_string()))? {
            Some(value) => {
                let session = serde_json::from_slice(&value)
                    .map_err(|e| agentsdk::chat_session::SessionManagerError::SerializationError(e))?;
                Ok(Some(session))
            }
            None => Ok(None),
        }
    }

    async fn delete_session(&self, id: SessionId) -> agentsdk::chat_session::Result<()> {
        let key = Self::session_key(&id);
        self.db.remove(&key)
            .map_err(|e| agentsdk::chat_session::SessionManagerError::StorageError(e.to_string()))?;

        Ok(())
    }

    async fn list_sessions(&self) -> agentsdk::chat_session::Result<Vec<ChatSession>> {
        let prefix = SESSIONS_PREFIX.as_bytes();
        let mut sessions = Vec::new();

        for result in self.db.scan_prefix(prefix) {
            let (_key, value) = result
                .map_err(|e| agentsdk::chat_session::SessionManagerError::StorageError(e.to_string()))?;
            let session = serde_json::from_slice(&value)
                .map_err(|e| agentsdk::chat_session::SessionManagerError::SerializationError(e))?;
            sessions.push(session);
        }

        Ok(sessions)
    }
}
```

### `src/queue/mod.rs` (update)

```rust
pub mod job;
pub mod state;
pub mod storage;

pub use job::{Job, JobId, JobStats, Priority};
pub use state::{QueueEvent, QueueState, StateChangeNotification, StateMachineError};
pub use storage::{DbStats, QueueStorage, StorageConfig, StorageError};
```

---

## Implementation Steps

- [ ] **Step 1: Create test file with failing tests**

```rust
// tests/unit/queue_storage_test.rs
use agentsdk::chat_session::{ChatSession, SessionManager, SessionState};
use agentsdk::queue::{Job, JobId, QueueState, QueueStorage, StorageConfig};

#[test]
fn test_in_memory_storage() {
    let storage = QueueStorage::open_in_memory().unwrap();
    let stats = storage.stats();
    assert_eq!(stats.job_count, 0);
}

#[test]
fn test_job_save_and_load() {
    let storage = QueueStorage::open_in_memory().unwrap();
    let session_id = agentsdk::chat_session::SessionId::new();

    let job = Job::new(
        session_id,
        "workflow-123".to_string(),
        std::collections::HashMap::new(),
        5,
    );

    storage.save_job(&job).unwrap();
    let loaded = storage.load_job(&job.id).unwrap();

    assert!(loaded.is_some());
    assert_eq!(loaded.unwrap().id, job.id);
}

#[test]
fn test_job_query_by_state() {
    let storage = QueueStorage::open_in_memory().unwrap();
    let session_id = agentsdk::chat_session::SessionId::new();

    let mut job1 = Job::new(
        session_id,
        "workflow-1".to_string(),
        std::collections::HashMap::new(),
        5,
    );

    let mut job2 = Job::new(
        session_id,
        "workflow-2".to_string(),
        std::collections::HashMap::new(),
        3,
    );

    job1.state = QueueState::Running;
    job2.state = QueueState::Pending;

    storage.save_job(&job1).unwrap();
    storage.save_job(&job2).unwrap();

    let pending_jobs = storage.query_jobs_by_state(QueueState::Pending).unwrap();
    assert_eq!(pending_jobs.len(), 1);
    assert_eq!(pending_jobs[0].id, job2.id);

    let running_jobs = storage.query_jobs_by_state(QueueState::Running).unwrap();
    assert_eq!(running_jobs.len(), 1);
    assert_eq!(running_jobs[0].id, job1.id);
}

#[tokio::test]
async fn test_session_storage() {
    let storage = QueueStorage::open_in_memory().unwrap();
    let manager = SessionManager::new(storage.clone());

    let session = manager.create_session(
        "test-session".to_string(),
        Some("test".to_string()),
        "workflow-123".to_string(),
        "user-1".to_string(),
        vec![],
        std::collections::HashMap::new(),
    ).await.unwrap();

    let loaded = manager.get_session(session.id).await.unwrap();
    assert_eq!(loaded.id, session.id);
    assert_eq!(loaded.metadata.name, "test-session");
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test queue_storage --lib`
Expected: FAIL with compilation errors (storage.rs doesn't exist yet)

- [ ] **Step 3: Implement storage.rs**

```rust
// src/queue/storage.rs (complete code from above)
```

- [ ] **Step 4: Update queue/mod.rs**

```rust
// src/queue/mod.rs (complete code from above)
```

- [ ] **Step 5: Run tests to verify they pass**

Run: `cargo test queue_storage --lib`
Expected: PASS for all tests

- [ ] **Step 6: Test recovery scenario**

```rust
#[test]
fn test_job_recovery() {
    let storage = QueueStorage::open_in_memory().unwrap();
    let session_id = agentsdk::chat_session::SessionId::new();

    let mut job = Job::new(
        session_id,
        "workflow-123".to_string(),
        std::collections::HashMap::new(),
        5,
    );

    job.state = QueueState::Running;
    storage.save_job(&job).unwrap();

    // Simulate crash recovery
    let recovered = storage.recover_running_jobs().unwrap();
    assert_eq!(recovered.len(), 1);
    assert_eq!(recovered[0].id, job.id);
    assert_eq!(recovered[0].state, QueueState::Running);
}
```

- [ ] **Step 7: Run all tests**

Run: `cargo test queue_storage --lib`
Expected: PASS for all tests

- [ ] **Step 8: Commit**

```bash
git add src/queue/storage.rs src/queue/mod.rs src/chat_session/manager.rs tests/unit/queue_storage_test.rs
git commit -m "feat: implement sled-based persistent queue storage with recovery"
```

---

## Migration Strategy

Storage schema versioning for future migrations:
```rust
const SCHEMA_VERSION_KEY: &str = "schema:version";
const CURRENT_SCHEMA_VERSION: u32 = 1;

impl QueueStorage {
    pub fn check_schema_version(&self) -> Result<u32> {
        match self.db.get(SCHEMA_VERSION_KEY.as_bytes())? {
            Some(value) => Ok(u32::from_be_bytes(
                value.as_ref().try_into().unwrap_or([0; 4]),
            )),
            None => Ok(0),
        }
    }

    pub fn set_schema_version(&self, version: u32) -> Result<()> {
        self.db.insert(
            SCHEMA_VERSION_KEY.as_bytes(),
            version.to_be_bytes().to_vec(),
        )?;
        Ok(())
    }
}
```

---

## Mock Strategy for Tests

Use `QueueStorage::open_in_memory()` for unit tests:
- Temporary sled database that doesn't persist to disk
- Fast for test execution
- Same API as production storage

For integration tests, use a real sled database with test path:
```rust
let storage = QueueStorage::open(StorageConfig {
    db_path: "/tmp/test-queue-db".to_string(),
    ..Default::default()
}).unwrap();
```
