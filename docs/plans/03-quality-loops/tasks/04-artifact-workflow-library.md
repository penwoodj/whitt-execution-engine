# Task 4: Artifact Workflow Library

**Files:**
- Create: `crates/quality/workflow/src/lib.rs`
- Create: `crates/quality/workflow/src/types.rs`
- Create: `crates/quality/workflow/src/storage.rs`
- Create: `crates/quality/workflow/src/transform.rs`
- Create: `crates/quality/workflow/src/review.rs`
- Create: `crates/quality/workflow/Cargo.toml`
- Test: `crates/quality/workflow/tests/workflow_tests.rs`

**Duration:** 1.5 weeks

## Overview

Build versioned workflow storage with transformation API and review controls. The workflow library stores workflow definitions with versioning and provides transformation capabilities for analysis and execution.

## Architecture

The workflow library consists of:

1. **ArtifactWorkflow** — Versioned workflow definition with metadata
2. **Storage** — Persistent storage with version history
3. **Transform API** — Convert workflows between formats (YAML, JSON, TOML)
4. **Review Controls** — Approvals, drafts, staged workflows

---

## Implementation Steps

### Step 1: Crate Setup

- [ ] **Create workflow crate with Cargo.toml**
  ```toml
  [package]
  name = "agentsdk-workflow"
  version = "0.1.0"
  edition = "2021"

  [dependencies]
  agentsdk-types = { path = "../../types" }
  serde = { version = "1.0", features = ["derive"] }
  serde_json = "1.0"
  serde_yaml = "0.9"
  toml = "0.8"
  thiserror = "1.0"
  tokio = { version = "1.0", features = ["full"] }
  chrono = { version = "0.4", features = ["serde"] }
  uuid = { version = "1.0", features = ["v4", "serde"] }
  git2 = "0.18"
  ```

### Step 2: Define Types

- [ ] **Create types.rs with ArtifactWorkflow**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactWorkflow {
    pub id: WorkflowId,
    pub name: String,
    pub description: String,
    pub version: WorkflowVersion,
    pub definition: WorkflowDefinition,
    pub metadata: WorkflowMetadata,
    pub state: WorkflowState,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub type WorkflowId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre_release: Option<String>,
    pub build_metadata: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowState {
    Draft,
    Staged,
    Approved,
    Deprecated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDefinition {
    pub steps: Vec<WorkflowStep>,
    pub parameters: WorkflowParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub id: String,
    pub name: String,
    pub operation: String,
    pub inputs: serde_json::Value,
    pub outputs: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowMetadata {
    pub author: String,
    pub tags: Vec<String>,
    pub category: String,
    pub quality_score: Option<f64>,
}
```

- [ ] **Write and run tests**
- [ ] **Commit**

### Step 3: Implement Storage

- [ ] **Create storage.rs**

```rust
pub struct WorkflowStorage {
    base_path: PathBuf,
}

impl WorkflowStorage {
    pub fn open(base_path: &PathBuf) -> Result<Self, StorageError> {
        std::fs::create_dir_all(base_path)?;
        Ok(Self { base_path: base_path.clone() })
    }

    pub async fn save(&self, workflow: &ArtifactWorkflow) -> Result<(), StorageError> {
        let version_dir = self.base_path
            .join(&workflow.id.to_string())
            .join(&workflow.version.to_string());
        std::fs::create_dir_all(&version_dir)?;

        let file_path = version_dir.join("workflow.json");
        let json = serde_json::to_string_pretty(workflow)?;
        std::fs::write(&file_path, json)?;

        Ok(())
    }

    pub async fn load(&self, id: &WorkflowId, version: &WorkflowVersion) -> Result<ArtifactWorkflow, StorageError> {
        let version_dir = self.base_path
            .join(&id.to_string())
            .join(&version.to_string());
        let file_path = version_dir.join("workflow.json");

        let json = std::fs::read_to_string(&file_path)?;
        let workflow: ArtifactWorkflow = serde_json::from_str(&json)?;

        Ok(workflow)
    }

    pub async fn list_versions(&self, id: &WorkflowId) -> Result<Vec<WorkflowVersion>, StorageError> {
        let workflow_dir = self.base_path.join(&id.to_string());
        if !workflow_dir.exists() {
            return Ok(Vec::new());
        }

        let mut versions = Vec::new();
        for entry in std::fs::read_dir(&workflow_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let version_str = entry.file_name().to_string_lossy().to_string();
                let version: WorkflowVersion = version_str.parse()
                    .map_err(|e| StorageError::Parse(format!("Invalid version: {}", e)))?;
                versions.push(version);
            }
        }

        Ok(versions)
    }
}
```

- [ ] **Write and run tests**
- [ ] **Commit**

### Step 4: Implement Transform API

- [ ] **Create transform.rs**

```rust
pub struct WorkflowTransformer;

impl WorkflowTransformer {
    pub fn to_yaml(workflow: &ArtifactWorkflow) -> Result<String, TransformError> {
        serde_yaml::to_string(workflow)
            .map_err(|e| TransformError::Serialization(e.to_string()))
    }

    pub fn from_yaml(yaml: &str) -> Result<ArtifactWorkflow, TransformError> {
        serde_yaml::from_str(yaml)
            .map_err(|e| TransformError::Deserialization(e.to_string()))
    }

    pub fn to_json(workflow: &ArtifactWorkflow) -> Result<String, TransformError> {
        serde_json::to_string_pretty(workflow)
            .map_err(|e| TransformError::Serialization(e.to_string()))
    }

    pub fn from_json(json: &str) -> Result<ArtifactWorkflow, TransformError> {
        serde_json::from_str(json)
            .map_err(|e| TransformError::Deserialization(e.to_string()))
    }

    pub fn to_toml(workflow: &ArtifactWorkflow) -> Result<String, TransformError> {
        toml::to_string_pretty(workflow)
            .map_err(|e| TransformError::Serialization(e.to_string()))
    }

    pub fn from_toml(toml: &str) -> Result<ArtifactWorkflow, TransformError> {
        toml::from_str(toml)
            .map_err(|e| TransformError::Deserialization(e.to_string()))
    }

    pub fn convert(input_format: Format, output_format: Format, input: &str) -> Result<String, TransformError> {
        let workflow = match input_format {
            Format::Yaml => Self::from_yaml(input)?,
            Format::Json => Self::from_json(input)?,
            Format::Toml => Self::from_toml(input)?,
        };

        match output_format {
            Format::Yaml => Self::to_yaml(&workflow),
            Format::Json => Self::to_json(&workflow),
            Format::Toml => Self::to_toml(&workflow),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Yaml,
    Json,
    Toml,
}
```

- [ ] **Write and run tests**
- [ ] **Commit**

### Step 5: Implement Review Controls

- [ ] **Create review.rs**

```rust
pub struct ReviewManager {
    storage: Arc<WorkflowStorage>,
}

impl ReviewManager {
    pub async fn submit_for_review(&self, workflow: ArtifactWorkflow) -> Result<(), ReviewError> {
        let mut workflow = workflow;
        workflow.state = WorkflowState::Staged;
        self.storage.save(&workflow).await?;
        Ok(())
    }

    pub async fn approve(&self, id: &WorkflowId, version: &WorkflowVersion, reviewer: &str) -> Result<(), ReviewError> {
        let mut workflow = self.storage.load(id, version).await?;
        workflow.state = WorkflowState::Approved;
        self.storage.save(&workflow).await?;
        Ok(())
    }

    pub async fn reject(&self, id: &WorkflowId, version: &WorkflowVersion, reason: &str) -> Result<(), ReviewError> {
        let mut workflow = self.storage.load(id, version).await?;
        workflow.state = WorkflowState::Draft;
        self.storage.save(&workflow).await?;
        Ok(())
    }

    pub async fn get_review_status(&self, id: &WorkflowId, version: &WorkflowVersion) -> Result<ReviewStatus, ReviewError> {
        let workflow = self.storage.load(id, version).await?;
        Ok(ReviewStatus {
            state: workflow.state,
            reviewers: Vec::new(), // Would be stored in separate review table
            comments: Vec::new(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ReviewStatus {
    pub state: WorkflowState,
    pub reviewers: Vec<String>,
    pub comments: Vec<Comment>,
}

#[derive(Debug, Clone)]
pub struct Comment {
    pub author: String,
    pub text: String,
    pub timestamp: DateTime<Utc>,
}
```

- [ ] **Write and run tests**
- [ ] **Commit**

### Step 6: Documentation

- [ ] **Create README.md**

## Completion Criteria

Task 4 is complete when:

- ✅ ArtifactWorkflow with versioning
- ✅ Storage with version history
- ✅ Transform API (YAML/JSON/TOML)
- ✅ Review controls (approve/reject/status)
- ✅ All tests passing
- ✅ Documentation complete

---

## QA Cross-References

- **QA Criteria**: ['$qa_criteria']('$file')
- **Test Cases**: ['$test_case']('$file')
- **Schema Ref**: $schema_ref
