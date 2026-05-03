use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
#[cfg(feature = "sqlite")]
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::debug;

#[cfg(feature = "sqlite")]
use rusqlite::{params, Connection, Result as SqliteResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowState {
    pub workflow_id: String,
    pub current_step: String,
    pub completed_steps: Vec<String>,
    pub variables: HashMap<String, serde_json::Value>,
    pub checkpoints: Vec<Checkpoint>,
    pub status: WorkflowStatus,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkflowStatus {
    Running,
    Paused,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub step_name: String,
    pub timestamp: String,
    pub state: HashMap<String, serde_json::Value>,
}

/// Format SystemTime to ISO 8601 string
fn format_timestamp() -> String {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("SystemTime before UNIX_EPOCH");
    let secs = duration.as_secs();

    // Convert seconds since epoch to ISO 8601 format
    let days = secs / 86400;
    let rem = secs % 86400;

    // Unix epoch starts on Thursday, 1970-01-01
    // Use simple calculation for date
    let year = 1970 + (days / 365) as i32;
    let month = 1;
    let day = (days % 365) as u32 + 1;

    // Time components
    let hour = (rem / 3600) as u32;
    let minute = ((rem % 3600) / 60) as u32;
    let second = (rem % 60) as u32;

    format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z", year, month, day, hour, minute, second)
}

#[async_trait]
pub trait PersistenceBackend: Send + Sync {
    async fn save(&self, state: &WorkflowState) -> Result<(), anyhow::Error>;
    fn sync_save(&self, state: &WorkflowState) -> Result<(), anyhow::Error>;
    async fn load(&self, workflow_id: &str) -> Result<WorkflowState, anyhow::Error>;
    fn sync_load(&self, workflow_id: &str) -> Result<WorkflowState, anyhow::Error>;
    async fn list_workflows(&self) -> Result<Vec<String>, anyhow::Error>;
    fn sync_list_workflows(&self) -> Result<Vec<String>, anyhow::Error>;
    async fn delete(&self, workflow_id: &str) -> Result<(), anyhow::Error>;
    fn sync_delete(&self, workflow_id: &str) -> Result<(), anyhow::Error>;
    async fn exists(&self, workflow_id: &str) -> bool;
    fn sync_exists(&self, workflow_id: &str) -> bool;
}

pub struct JsonPersistence {
    storage_path: PathBuf,
}

impl JsonPersistence {
    pub fn new(storage_path: PathBuf) -> Result<Self, anyhow::Error> {
        std::fs::create_dir_all(&storage_path)?;
        Ok(Self { storage_path })
    }

    pub fn sync_new(storage_path: PathBuf) -> Result<Self, anyhow::Error> {
        Self::new(storage_path)
    }

    fn get_workflow_path(&self, workflow_id: &str) -> PathBuf {
        self.storage_path.join(format!("{}.json", workflow_id))
    }
}

#[async_trait]
impl PersistenceBackend for JsonPersistence {
    async fn save(&self, state: &WorkflowState) -> Result<(), anyhow::Error> {
        debug!("Saving workflow state (JSON): {}", state.workflow_id);

        let file_path = self.get_workflow_path(&state.workflow_id);
        let json = serde_json::to_string_pretty(state)?;

        tokio::fs::write(file_path, json).await?;
        debug!("Workflow state saved successfully (JSON)");
        Ok(())
    }

    fn sync_save(&self, state: &WorkflowState) -> Result<(), anyhow::Error> {
        debug!("Saving workflow state (JSON): {}", state.workflow_id);

        let file_path = self.get_workflow_path(&state.workflow_id);
        let json = serde_json::to_string_pretty(state)?;

        std::fs::write(file_path, json)?;
        debug!("Workflow state saved successfully (JSON)");
        Ok(())
    }

    async fn load(&self, workflow_id: &str) -> Result<WorkflowState, anyhow::Error> {
        debug!("Loading workflow state (JSON): {}", workflow_id);

        let file_path = self.get_workflow_path(workflow_id);
        let content = tokio::fs::read_to_string(file_path).await?;
        let state: WorkflowState = serde_json::from_str(&content)?;

        debug!("Workflow state loaded successfully (JSON)");
        Ok(state)
    }

    fn sync_load(&self, workflow_id: &str) -> Result<WorkflowState, anyhow::Error> {
        debug!("Loading workflow state (JSON): {}", workflow_id);

        let file_path = self.get_workflow_path(workflow_id);
        let content = std::fs::read_to_string(file_path)?;
        let state: WorkflowState = serde_json::from_str(&content)?;

        debug!("Workflow state loaded successfully (JSON)");
        Ok(state)
    }

    async fn list_workflows(&self) -> Result<Vec<String>, anyhow::Error> {
        debug!("Listing workflows in storage (JSON)");

        let mut entries = tokio::fs::read_dir(&self.storage_path).await?;
        let mut workflow_ids = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "json") {
                if let Some(file_stem) = path.file_stem() {
                    if let Some(id) = file_stem.to_str() {
                        workflow_ids.push(id.to_string());
                    }
                }
            }
        }

        debug!("Found {} workflows (JSON)", workflow_ids.len());
        Ok(workflow_ids)
    }

    fn sync_list_workflows(&self) -> Result<Vec<String>, anyhow::Error> {
        debug!("Listing workflows in storage (JSON)");

        let mut workflow_ids = Vec::new();

        for entry in std::fs::read_dir(&self.storage_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "json") {
                if let Some(file_stem) = path.file_stem() {
                    if let Some(id) = file_stem.to_str() {
                        workflow_ids.push(id.to_string());
                    }
                }
            }
        }

        debug!("Found {} workflows (JSON)", workflow_ids.len());
        Ok(workflow_ids)
    }

    async fn delete(&self, workflow_id: &str) -> Result<(), anyhow::Error> {
        debug!("Deleting workflow (JSON): {}", workflow_id);

        let file_path = self.get_workflow_path(workflow_id);
        tokio::fs::remove_file(file_path).await?;

        debug!("Workflow deleted successfully (JSON)");
        Ok(())
    }

    fn sync_delete(&self, workflow_id: &str) -> Result<(), anyhow::Error> {
        debug!("Deleting workflow (JSON): {}", workflow_id);

        let file_path = self.get_workflow_path(workflow_id);
        std::fs::remove_file(file_path)?;

        debug!("Workflow deleted successfully (JSON)");
        Ok(())
    }

    async fn exists(&self, workflow_id: &str) -> bool {
        let file_path = self.get_workflow_path(workflow_id);
        tokio::fs::metadata(file_path).await.is_ok()
    }

    fn sync_exists(&self, workflow_id: &str) -> bool {
        let file_path = self.get_workflow_path(workflow_id);
        file_path.exists()
    }
}

#[cfg(feature = "sqlite")]
pub struct SqlitePersistence {
    conn: Arc<Mutex<Connection>>,
}

#[cfg(feature = "sqlite")]
impl SqlitePersistence {
    pub fn new(storage_path: PathBuf) -> Result<Self, anyhow::Error> {
        std::fs::create_dir_all(&storage_path)?;

        let db_path = storage_path.join("workflows.db");
        let conn = Connection::open(&db_path)?;

        let backend = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        backend.init_table()?;
        Ok(backend)
    }

    pub fn sync_new(storage_path: PathBuf) -> Result<Self, anyhow::Error> {
        Self::new(storage_path)
    }

    fn init_table(&self) -> Result<(), anyhow::Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS workflows (
                id TEXT PRIMARY KEY,
                state TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }
}

#[cfg(feature = "sqlite")]
#[async_trait]
impl PersistenceBackend for SqlitePersistence {
    async fn save(&self, state: &WorkflowState) -> Result<(), anyhow::Error> {
        debug!("Saving workflow state (SQLite): {}", state.workflow_id);

        let state_json = serde_json::to_string_pretty(state)?;

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO workflows (id, state, updated_at) VALUES (?1, ?2, ?3)",
            params![&state.workflow_id, &state_json, &state.updated_at],
        )?;

        debug!("Workflow state saved successfully (SQLite)");
        Ok(())
    }

    fn sync_save(&self, state: &WorkflowState) -> Result<(), anyhow::Error> {
        debug!("Saving workflow state (SQLite): {}", state.workflow_id);

        let state_json = serde_json::to_string_pretty(state)?;

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO workflows (id, state, updated_at) VALUES (?1, ?2, ?3)",
            params![&state.workflow_id, &state_json, &state.updated_at],
        )?;

        debug!("Workflow state saved successfully (SQLite)");
        Ok(())
    }

    async fn load(&self, workflow_id: &str) -> Result<WorkflowState, anyhow::Error> {
        debug!("Loading workflow state (SQLite): {}", workflow_id);

        let conn = self.conn.lock().unwrap();
        let state_json: String = conn.query_row(
            "SELECT state FROM workflows WHERE id = ?1",
            params![workflow_id],
            |row| row.get(0),
        )?;

        let state: WorkflowState = serde_json::from_str(&state_json)?;

        debug!("Workflow state loaded successfully (SQLite)");
        Ok(state)
    }

    fn sync_load(&self, workflow_id: &str) -> Result<WorkflowState, anyhow::Error> {
        debug!("Loading workflow state (SQLite): {}", workflow_id);

        let conn = self.conn.lock().unwrap();
        let state_json: String = conn.query_row(
            "SELECT state FROM workflows WHERE id = ?1",
            params![workflow_id],
            |row| row.get(0),
        )?;

        let state: WorkflowState = serde_json::from_str(&state_json)?;

        debug!("Workflow state loaded successfully (SQLite)");
        Ok(state)
    }

    async fn list_workflows(&self) -> Result<Vec<String>, anyhow::Error> {
        debug!("Listing workflows in storage (SQLite)");

        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id FROM workflows ORDER BY updated_at DESC")?;
        let workflow_ids = stmt
            .query_map([], |row| row.get::<_, String>(0))?
            .collect::<SqliteResult<Vec<String>>>()?;

        debug!("Found {} workflows (SQLite)", workflow_ids.len());
        Ok(workflow_ids)
    }

    fn sync_list_workflows(&self) -> Result<Vec<String>, anyhow::Error> {
        debug!("Listing workflows in storage (SQLite)");

        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id FROM workflows ORDER BY updated_at DESC")?;
        let workflow_ids = stmt
            .query_map([], |row| row.get::<_, String>(0))?
            .collect::<SqliteResult<Vec<String>>>()?;

        debug!("Found {} workflows (SQLite)", workflow_ids.len());
        Ok(workflow_ids)
    }

    async fn delete(&self, workflow_id: &str) -> Result<(), anyhow::Error> {
        debug!("Deleting workflow (SQLite): {}", workflow_id);

        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM workflows WHERE id = ?1", params![workflow_id])?;

        debug!("Workflow deleted successfully (SQLite)");
        Ok(())
    }

    fn sync_delete(&self, workflow_id: &str) -> Result<(), anyhow::Error> {
        debug!("Deleting workflow (SQLite): {}", workflow_id);

        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM workflows WHERE id = ?1", params![workflow_id])?;

        debug!("Workflow deleted successfully (SQLite)");
        Ok(())
    }

    async fn exists(&self, workflow_id: &str) -> bool {
        let conn = self.conn.lock().unwrap();
        conn
            .query_row(
                "SELECT 1 FROM workflows WHERE id = ?1",
                params![workflow_id],
                |_| Ok(true),
            )
            .unwrap_or(false)
    }

    fn sync_exists(&self, workflow_id: &str) -> bool {
        let conn = self.conn.lock().unwrap();
        conn
            .query_row(
                "SELECT 1 FROM workflows WHERE id = ?1",
                params![workflow_id],
                |_| Ok(true),
            )
            .unwrap_or(false)
    }
}

pub struct WorkflowPersistence {
    backend: Box<dyn PersistenceBackend>,
}

impl WorkflowPersistence {
    pub fn new(storage_path: PathBuf) -> Self {
        let backend = JsonPersistence::new(storage_path)
            .expect("Failed to create JSON persistence backend");
        Self {
            backend: Box::new(backend),
        }
    }

    pub fn sync_new(storage_path: PathBuf) -> Result<Self, anyhow::Error> {
        let backend = JsonPersistence::sync_new(storage_path)?;
        Ok(Self {
            backend: Box::new(backend),
        })
    }

    #[cfg(feature = "sqlite")]
    pub fn new_with_sqlite(storage_path: PathBuf) -> Result<Self, anyhow::Error> {
        let backend = SqlitePersistence::new(storage_path)?;
        Ok(Self {
            backend: Box::new(backend),
        })
    }

    #[cfg(feature = "sqlite")]
    pub fn sync_new_with_sqlite(storage_path: PathBuf) -> Result<Self, anyhow::Error> {
        let backend = SqlitePersistence::sync_new(storage_path)?;
        Ok(Self {
            backend: Box::new(backend),
        })
    }

    pub fn with_backend(backend: Box<dyn PersistenceBackend>) -> Self {
        Self { backend }
    }

    pub async fn save(&self, state: &WorkflowState) -> Result<(), anyhow::Error> {
        self.backend.save(state).await
    }

    pub fn sync_save(&self, state: &WorkflowState) -> Result<(), anyhow::Error> {
        self.backend.sync_save(state)
    }

    pub async fn load(&self, workflow_id: &str) -> Result<WorkflowState, anyhow::Error> {
        self.backend.load(workflow_id).await
    }

    pub fn sync_load(&self, workflow_id: &str) -> Result<WorkflowState, anyhow::Error> {
        self.backend.sync_load(workflow_id)
    }

    pub async fn create_checkpoint(
        &self,
        state: &mut WorkflowState,
        step_name: &str,
    ) -> Result<(), anyhow::Error> {
        debug!("Creating checkpoint for step: {}", step_name);

        let checkpoint = Checkpoint {
            step_name: step_name.to_string(),
            timestamp: format_timestamp(),
            state: state.variables.clone(),
        };

        state.checkpoints.push(checkpoint);
        state.updated_at = format_timestamp();

        self.backend.save(state).await?;

        debug!("Checkpoint created successfully");
        Ok(())
    }

    pub fn sync_create_checkpoint(
        &self,
        state: &mut WorkflowState,
        step_name: &str,
    ) -> Result<(), anyhow::Error> {
        debug!("Creating checkpoint for step: {}", step_name);

        let checkpoint = Checkpoint {
            step_name: step_name.to_string(),
            timestamp: format_timestamp(),
            state: state.variables.clone(),
        };

        state.checkpoints.push(checkpoint);
        state.updated_at = format_timestamp();

        self.backend.sync_save(state)?;

        debug!("Checkpoint created successfully");
        Ok(())
    }

    pub async fn list_workflows(&self) -> Result<Vec<String>, anyhow::Error> {
        self.backend.list_workflows().await
    }

    pub fn sync_list_workflows(&self) -> Result<Vec<String>, anyhow::Error> {
        self.backend.sync_list_workflows()
    }

    pub async fn delete(&self, workflow_id: &str) -> Result<(), anyhow::Error> {
        self.backend.delete(workflow_id).await
    }

    pub fn sync_delete(&self, workflow_id: &str) -> Result<(), anyhow::Error> {
        self.backend.sync_delete(workflow_id)
    }

    pub async fn exists(&self, workflow_id: &str) -> bool {
        self.backend.exists(workflow_id).await
    }

    pub fn sync_exists(&self, workflow_id: &str) -> bool {
        self.backend.sync_exists(workflow_id)
    }
}

impl WorkflowState {
    pub fn new(workflow_id: String, initial_step: String) -> Self {
        let now = format_timestamp();
        Self {
            workflow_id,
            current_step: initial_step,
            completed_steps: Vec::new(),
            variables: HashMap::new(),
            checkpoints: Vec::new(),
            status: WorkflowStatus::Running,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    pub fn complete_step(&mut self, step_name: String) {
        self.completed_steps.push(step_name.clone());
        self.updated_at = format_timestamp();
    }

    pub fn set_current_step(&mut self, step_name: String) {
        self.current_step = step_name;
        self.updated_at = format_timestamp();
    }

    pub fn set_variable(&mut self, key: String, value: serde_json::Value) {
        self.variables.insert(key, value);
        self.updated_at = format_timestamp();
    }

    pub fn get_variable(&self, key: &str) -> Option<&serde_json::Value> {
        self.variables.get(key)
    }

    pub fn set_status(&mut self, status: WorkflowStatus) {
        self.status = status;
        self.updated_at = format_timestamp();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
#[cfg(feature = "sqlite")]
use std::sync::{Arc, Mutex};

    #[test]
    fn test_workflow_state_new() {
        let state = WorkflowState::new("test-workflow".to_string(), "step1".to_string());
        assert_eq!(state.workflow_id, "test-workflow");
        assert_eq!(state.current_step, "step1");
        assert!(state.completed_steps.is_empty());
        assert_eq!(state.status, WorkflowStatus::Running);
    }

    #[test]
    fn test_workflow_state_complete_step() {
        let mut state = WorkflowState::new("test-workflow".to_string(), "step1".to_string());
        state.complete_step("step1".to_string());
        state.complete_step("step2".to_string());
        assert_eq!(state.completed_steps.len(), 2);
        assert_eq!(state.completed_steps[0], "step1");
        assert_eq!(state.completed_steps[1], "step2");
    }

    #[test]
    fn test_workflow_state_variables() {
        let mut state = WorkflowState::new("test-workflow".to_string(), "step1".to_string());
        state.set_variable("key1".to_string(), serde_json::json!("value1"));
        assert_eq!(state.get_variable("key1"), Some(&serde_json::json!("value1")));
        assert_eq!(state.get_variable("key2"), None);
    }

    #[test]
    fn test_checkpoint_serialization() {
        let checkpoint = Checkpoint {
            step_name: "step1".to_string(),
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            state: {
                let mut map = HashMap::new();
                map.insert("key".to_string(), serde_json::json!("value"));
                map
            },
        };

        let json = serde_json::to_string(&checkpoint).unwrap();
        let deserialized: Checkpoint = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.step_name, checkpoint.step_name);
    }

    #[cfg(feature = "sqlite")]
    #[test]
    fn test_sqlite_save_load() {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage_path = temp_dir.path().to_path_buf();

        let persistence = SqlitePersistence::new(storage_path).unwrap();
        let state = WorkflowState::new("test-workflow".to_string(), "step1".to_string());

        persistence.sync_save(&state).unwrap();
        let loaded = persistence.sync_load("test-workflow").unwrap();

        assert_eq!(loaded.workflow_id, state.workflow_id);
        assert_eq!(loaded.current_step, state.current_step);
        assert_eq!(loaded.status, state.status);
    }

    #[cfg(feature = "sqlite")]
    #[test]
    fn test_sqlite_list_workflows() {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage_path = temp_dir.path().to_path_buf();

        let persistence = SqlitePersistence::new(storage_path).unwrap();

        let state1 = WorkflowState::new("workflow-1".to_string(), "step1".to_string());
        let state2 = WorkflowState::new("workflow-2".to_string(), "step1".to_string());
        let state3 = WorkflowState::new("workflow-3".to_string(), "step1".to_string());

        persistence.sync_save(&state1).unwrap();
        persistence.sync_save(&state2).unwrap();
        persistence.sync_save(&state3).unwrap();

        let workflows = persistence.sync_list_workflows().unwrap();
        assert_eq!(workflows.len(), 3);
        assert!(workflows.contains(&"workflow-1".to_string()));
        assert!(workflows.contains(&"workflow-2".to_string()));
        assert!(workflows.contains(&"workflow-3".to_string()));
    }

    #[cfg(feature = "sqlite")]
    #[test]
    fn test_sqlite_delete() {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage_path = temp_dir.path().to_path_buf();

        let persistence = SqlitePersistence::new(storage_path).unwrap();
        let state = WorkflowState::new("test-workflow".to_string(), "step1".to_string());

        persistence.sync_save(&state).unwrap();
        assert!(persistence.sync_exists("test-workflow"));

        persistence.sync_delete("test-workflow").unwrap();
        assert!(!persistence.sync_exists("test-workflow"));
    }

    #[cfg(feature = "sqlite")]
    #[test]
    fn test_sqlite_create_checkpoint() {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage_path = temp_dir.path().to_path_buf();

        let persistence = SqlitePersistence::new(storage_path).unwrap();
        let mut state = WorkflowState::new("test-workflow".to_string(), "step1".to_string());

        state.set_variable("key1".to_string(), serde_json::json!("value1"));

        let wp = WorkflowPersistence::with_backend(Box::new(persistence));
        wp.sync_create_checkpoint(&mut state, "checkpoint1").unwrap();

        assert_eq!(state.checkpoints.len(), 1);
        assert_eq!(state.checkpoints[0].step_name, "checkpoint1");
        assert!(state.checkpoints[0].state.contains_key("key1"));

        let loaded = wp.sync_load("test-workflow").unwrap();
        assert_eq!(loaded.checkpoints.len(), 1);
    }

    #[cfg(feature = "sqlite")]
    #[test]
    fn test_sqlite_concurrent_writes() {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage_path = temp_dir.path().to_path_buf();

        let persistence = Arc::new(Mutex::new(
            SqlitePersistence::new(storage_path).unwrap(),
        ));

        let mut handles = vec![];

        for i in 0..10 {
            let p = Arc::clone(&persistence);
            let handle = std::thread::spawn(move || {
                let state = WorkflowState::new(format!("workflow-{}", i), "step1".to_string());
                let p = p.lock().unwrap();
                p.sync_save(&state).unwrap();
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let p = persistence.lock().unwrap();
        let workflows = p.sync_list_workflows().unwrap();
        assert_eq!(workflows.len(), 10);
    }

    #[cfg(feature = "sqlite")]
    #[test]
    fn test_sqlite_update_preserves_state() {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage_path = temp_dir.path().to_path_buf();

        let persistence = SqlitePersistence::new(storage_path).unwrap();
        let mut state = WorkflowState::new("test-workflow".to_string(), "step1".to_string());

        state.set_variable("initial_var".to_string(), serde_json::json!("initial_value"));

        persistence.sync_save(&state).unwrap();

        let mut loaded = persistence.sync_load("test-workflow").unwrap();
        assert_eq!(
            loaded.get_variable("initial_var"),
            Some(&serde_json::json!("initial_value"))
        );

        loaded.set_variable("new_var".to_string(), serde_json::json!("new_value"));
        loaded.complete_step("step1".to_string());
        loaded.set_current_step("step2".to_string());

        persistence.sync_save(&loaded).unwrap();

        let reloaded = persistence.sync_load("test-workflow").unwrap();
        assert_eq!(reloaded.current_step, "step2");
        assert_eq!(reloaded.completed_steps.len(), 1);
        assert_eq!(reloaded.completed_steps[0], "step1");
        assert_eq!(
            reloaded.get_variable("new_var"),
            Some(&serde_json::json!("new_value"))
        );
    }
}
