use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::debug;

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

pub struct WorkflowPersistence {
    storage_path: PathBuf,
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

impl WorkflowPersistence {
    pub fn new(storage_path: PathBuf) -> Self {
        // Create storage directory if it doesn't exist
        tokio::runtime::Handle::current()
            .block_on(async {
                tokio::fs::create_dir_all(&storage_path).await
            })
            .expect("Failed to create storage directory");

        Self { storage_path }
    }

    pub fn sync_new(storage_path: PathBuf) -> Result<Self, anyhow::Error> {
        std::fs::create_dir_all(&storage_path)?;
        Ok(Self { storage_path })
    }

    pub async fn save(&self, state: &WorkflowState) -> Result<(), anyhow::Error> {
        debug!("Saving workflow state: {}", state.workflow_id);

        let file_path = self.get_workflow_path(&state.workflow_id);
        let json = serde_json::to_string_pretty(state)?;

        tokio::fs::write(file_path, json).await?;
        debug!("Workflow state saved successfully");
        Ok(())
    }

    pub fn sync_save(&self, state: &WorkflowState) -> Result<(), anyhow::Error> {
        debug!("Saving workflow state: {}", state.workflow_id);

        let file_path = self.get_workflow_path(&state.workflow_id);
        let json = serde_json::to_string_pretty(state)?;

        std::fs::write(file_path, json)?;
        debug!("Workflow state saved successfully");
        Ok(())
    }

    pub async fn load(&self, workflow_id: &str) -> Result<WorkflowState, anyhow::Error> {
        debug!("Loading workflow state: {}", workflow_id);

        let file_path = self.get_workflow_path(workflow_id);
        let content = tokio::fs::read_to_string(file_path).await?;
        let state: WorkflowState = serde_json::from_str(&content)?;

        debug!("Workflow state loaded successfully");
        Ok(state)
    }

    pub fn sync_load(&self, workflow_id: &str) -> Result<WorkflowState, anyhow::Error> {
        debug!("Loading workflow state: {}", workflow_id);

        let file_path = self.get_workflow_path(workflow_id);
        let content = std::fs::read_to_string(file_path)?;
        let state: WorkflowState = serde_json::from_str(&content)?;

        debug!("Workflow state loaded successfully");
        Ok(state)
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

        // Save state with new checkpoint
        self.save(state).await?;

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

        // Save state with new checkpoint
        self.sync_save(state)?;

        debug!("Checkpoint created successfully");
        Ok(())
    }

    pub async fn list_workflows(&self) -> Result<Vec<String>, anyhow::Error> {
        debug!("Listing workflows in storage");

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

        debug!("Found {} workflows", workflow_ids.len());
        Ok(workflow_ids)
    }

    pub fn sync_list_workflows(&self) -> Result<Vec<String>, anyhow::Error> {
        debug!("Listing workflows in storage");

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

        debug!("Found {} workflows", workflow_ids.len());
        Ok(workflow_ids)
    }

    pub async fn delete(&self, workflow_id: &str) -> Result<(), anyhow::Error> {
        debug!("Deleting workflow: {}", workflow_id);

        let file_path = self.get_workflow_path(workflow_id);
        tokio::fs::remove_file(file_path).await?;

        debug!("Workflow deleted successfully");
        Ok(())
    }

    pub fn sync_delete(&self, workflow_id: &str) -> Result<(), anyhow::Error> {
        debug!("Deleting workflow: {}", workflow_id);

        let file_path = self.get_workflow_path(workflow_id);
        std::fs::remove_file(file_path)?;

        debug!("Workflow deleted successfully");
        Ok(())
    }

    fn get_workflow_path(&self, workflow_id: &str) -> PathBuf {
        self.storage_path.join(format!("{}.json", workflow_id))
    }

    pub async fn exists(&self, workflow_id: &str) -> bool {
        let file_path = self.get_workflow_path(workflow_id);
        tokio::fs::metadata(file_path).await.is_ok()
    }

    pub fn sync_exists(&self, workflow_id: &str) -> bool {
        let file_path = self.get_workflow_path(workflow_id);
        file_path.exists()
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
}
