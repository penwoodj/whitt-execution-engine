# Task 3: File-Type Capability Matrix

**Files:**
- Create: `crates/quality/capability/src/lib.rs`
- Create: `crates/quality/capability/src/types.rs`
- Create: `crates/quality/capability/src/matrix.rs`
- Create: `crates/quality/capability/src/registry.rs`
- Create: `crates/quality/capability/src/gap_analysis.rs`
- Create: `crates/quality/capability/Cargo.toml`
- Test: `crates/quality/capability/tests/capability_tests.rs`

**Duration:** 1.5 weeks

## Overview

Create the knowledge graph of workflow capabilities per file type with quality thresholds. The capability matrix enables gap analysis, dynamic workflow selection, and roadmap planning.

## Architecture

The capability system consists of:

1. **FileCapability** — Describes what a workflow can do for a file type
2. **CapabilityMatrix** — Knowledge graph of all (workflow, file_type) mappings
3. **Registry** — Registration and lookup of capabilities
4. **Gap Analysis** — Identify missing capabilities and improvement opportunities

---

## Implementation Steps

### Step 1: Crate Setup

- [ ] **Create capability crate with Cargo.toml**
  ```toml
  [package]
  name = "agentsdk-capability"
  version = "0.1.0"
  edition = "2021"

  [dependencies]
  agentsdk-types = { path = "../../types" }
  serde = { version = "1.0", features = ["derive"] }
  serde_json = "1.0"
  thiserror = "1.0"
  tokio = { version = "1.0", features = ["full"] }
  chrono = { version = "0.4", features = ["serde"] }
  uuid = { version = "1.0", features = ["v4", "serde"] }
  ```

### Step 2: Define Types

- [ ] **Create types.rs with FileCapability and QualityThreshold**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCapability {
    pub workflow_id: String,
    pub workflow_version: String,
    pub file_type: String,
    pub supported_operations: Vec<String>,
    pub quality_threshold: QualityThreshold,
    pub performance_profile: PerformanceProfile,
    pub confidence: f64,
    pub last_verified: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityThreshold {
    pub min_quality_score: f64,
    pub min_confidence: f64,
    pub required_checks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceProfile {
    pub avg_duration_ms: u64,
    pub p95_duration_ms: u64,
    pub avg_cost_usd: f64,
}
```

- [ ] **Write and run tests**
- [ ] **Commit**

### Step 3: Implement Matrix

- [ ] **Create matrix.rs with CapabilityMatrix**

```rust
pub struct CapabilityMatrix {
    capabilities: HashMap<(String, String), FileCapability>, // (workflow_id, file_type) -> capability
    by_file_type: HashMap<String, Vec<String>>, // file_type -> workflow_ids
}

impl CapabilityMatrix {
    pub fn new() -> Self { /* ... */ }

    pub fn register(&mut self, capability: FileCapability) -> Result<(), CapabilityError> {
        let key = (capability.workflow_id.clone(), capability.file_type.clone());
        self.capabilities.insert(key, capability.clone());

        self.by_file_type
            .entry(capability.file_type.clone())
            .or_insert_with(Vec::new)
            .push(capability.workflow_id.clone());

        Ok(())
    }

    pub fn get(&self, workflow_id: &str, file_type: &str) -> Option<&FileCapability> {
        self.capabilities.get(&(workflow_id.to_string(), file_type.to_string()))
    }

    pub fn get_for_file_type(&self, file_type: &str) -> Vec<&FileCapability> {
        self.by_file_type
            .get(file_type)
            .and_then(|ids| ids.iter().filter_map(|id| self.capabilities.get(&(id.clone(), file_type.to_string()))).collect())
            .unwrap_or_default()
    }

    pub fn find_best_workflow(&self, file_type: &str, requirements: &CapabilityRequirements) -> Option<&FileCapability> {
        let capabilities = self.get_for_file_type(file_type);
        capabilities
            .into_iter()
            .filter(|c| c.quality_threshold.min_quality_score >= requirements.min_quality_score)
            .max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap())
    }
}

pub struct CapabilityRequirements {
    pub min_quality_score: f64,
    pub required_operations: Vec<String>,
    pub max_cost_usd: Option<f64>,
    pub max_duration_ms: Option<u64>,
}
```

- [ ] **Write and run tests**
- [ ] **Commit**

### Step 4: Implement Registry

- [ ] **Create registry.rs**

```rust
pub struct CapabilityRegistry {
    matrix: Arc<RwLock<CapabilityMatrix>>,
}

impl CapabilityRegistry {
    pub async fn register(&self, capability: FileCapability) -> Result<(), CapabilityError> {
        let mut matrix = self.matrix.write().await;
        matrix.register(capability)
    }

    pub async fn query(&self, query: CapabilityQuery) -> Vec<FileCapability> {
        let matrix = self.matrix.read().await;
        // Execute query against matrix
        Vec::new()
    }
}

pub struct CapabilityQuery {
    pub file_type: Option<String>,
    pub workflow_id: Option<String>,
    pub min_confidence: Option<f64>,
    pub supports_operations: Option<Vec<String>>,
}
```

- [ ] **Write and run tests**
- [ ] **Commit**

### Step 5: Implement Gap Analysis

- [ ] **Create gap_analysis.rs**

```rust
pub struct GapAnalysis {
    pub missing_capabilities: Vec<MissingCapability>,
    pub quality_gaps: Vec<QualityGap>,
    pub opportunities: Vec<ImprovementOpportunity>,
}

pub fn analyze_gaps(matrix: &CapabilityMatrix, requirements: Vec<Requirement>) -> GapAnalysis {
    // Find capabilities that don't exist
    // Find capabilities that don't meet quality thresholds
    // Suggest improvements
    GapAnalysis {
        missing_capabilities: Vec::new(),
        quality_gaps: Vec::new(),
        opportunities: Vec::new(),
    }
}
```

- [ ] **Write and run tests**
- [ ] **Commit**

### Step 6: Documentation

- [ ] **Create README.md with usage examples**

## Completion Criteria

Task 3 is complete when:

- ✅ FileCapability struct implemented
- ✅ CapabilityMatrix with registration and querying
- ✅ Registry with async operations
- ✅ Gap analysis implemented
- ✅ All tests passing
- ✅ Documentation complete
