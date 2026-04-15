# Configuration Schema Design

## Overview

Proper schema placement for infinite context integration in yaml-to-rust-agent SDK.

## Schema Structure

### 1. Workspace Configuration (`workspace.yaml`)

Defines backend availability and global settings.

```yaml
workspace:
  # Backend availability (presence = enabled)
  backends:
    ollama:
      endpoint: http://localhost:11434
      default_model: llama3.2
      timeout_ms: 30000
      max_retries: 3
    
    llama_cpp:
      library_path: /usr/local/lib/libllama.so
      model_path: /models/llama-3.2-q4.gguf
      num_gpu_layers: 33
      context_size: 16384
      gpu_backend: Vulkan  # Vulkan|CUDA|ROCm|Metal
    
    vector_store:
      type: chroma  # chroma|faiss|weaviate
      chroma:
        path: ./chroma_db
        collection_name: documents
      faiss:
        dimension: 768
        index_type: flat
    
    rlm:
      max_depth: 1  # CRITICAL: Always 1
      token_budget: 1000000
      repl_type: local  # local|docker|restricted
      timeout_ms: 60000
```

### 2. Workflow Configuration (`workflow.yaml`)

Defines per-workflow context orchestration settings.

```yaml
workflow:
  execution:
    mode: parallel|serial|hybrid
    
    # Context orchestration under execution
    context_orchestration:
      # Strategy thresholds
      rlm_threshold: 100000      # 100K tokens → use RLM
      rag_threshold: 50000       # 50K tokens → use RAG
      
      # Default strategy
      default_strategy: auto       # auto|direct|rag|rlm|hybrid
      
      # Optional strategy overrides
      rag_config:
        top_k: 5
        score_threshold: 0.3
      
      rlm_config:
        depth: 1  # Should always be 1
  
  steps:
    - type: llm_context_aware
      id: analyze
      model: "${workspace.backends.ollama.default_model}"
      prompt: "Analyze: {{ context }}"
      
      # Context configuration
      context:
        source: file
        path: ./large_document.txt
```

## Key Design Principles

### 1. No "Enabled" Flags

**Wrong** (opencode style):
```yaml
backends:
  rlm:
    enabled: true
```

**Correct** (SDK style):
```yaml
backends:
  rlm:
    max_depth: 1
```

Presence in config = enabled. No boolean flags.

### 2. Proper Hierarchy

```
workspace.yaml (global availability)
    ↓
workflow.execution.context_orchestration (per-workflow settings)
    ↓
step.context (per-step override)
```

### 3. Configuration Mediation

Workspace config defines what's AVAILABLE. Workflow config defines HOW to use it.

```yaml
# workspace.yaml (backends available)
backends:
  rlm:
    max_depth: 1
  vector_store:
    type: chroma

# workflow.yaml (how to use them)
execution:
  context_orchestration:
    default_strategy: auto
    rlm_threshold: 100000
```

Orchestrator checks:
1. Is RLM available? (workspace.backends.rlm exists?)
2. Is RLM requested? (workflow or step says use rlm?)
3. Use RLM or fallback to direct

### 4. Fallback Behavior

```yaml
# Scenario: Workflow requests RLM, but RLM not in workspace

workflow:
  execution:
    context_orchestration:
      default_strategy: rlm  # Request RLM

workspace:
  backends:
    # rlm: NOT PRESENT
    ollama: ...  # But ollama available

Result:
  1. Orchestrator checks workspace.backends.rlm
  2. Not found → fallback to direct or rag
  3. Logs warning: "RLM requested but not configured, using direct"
```

## Configuration Flow

```
┌─────────────────────────────────────────────────┐
│  Load workspace.yaml                │
│  - Available backends                │
│  - Global settings                  │
└────────────┬────────────────────────────┘
             │
             ↓
┌─────────────────────────────────────────────────┐
│  Load workflow.yaml                   │
│  - Context orchestration settings      │
│  - Step definitions                 │
└────────────┬────────────────────────────┘
             │
             ↓
┌─────────────────────────────────────────────────┐
│  Context Orchestrator Initialized      │
│                                     │
│  1. Merge workspace + workflow config│
│  2. Determine backend availability     │
│  3. Validate strategy compatibility    │
└────────────┬────────────────────────────┘
             │
             ↓
┌─────────────────────────────────────────────────┐
│  Execute Workflow                   │
│                                     │
│  For each step:                     │
│  - Select strategy (based on config)  │
│  - Check backend availability        │
│  - Execute or fallback              │
└─────────────────────────────────────────────┘
```

## Rust Schema Types

```rust
// Workspace configuration
#[derive(Debug, Clone, Deserialize)]
pub struct WorkspaceConfig {
    pub backends: BackendsConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BackendsConfig {
    pub ollama: Option<OllamaConfig>,
    pub llama_cpp: Option<LlamaCppConfig>,
    pub vector_store: Option<VectorStoreConfig>,
    pub rlm: Option<RLMBackendConfig>,
    // Presence = enabled, no "enabled" field
}

// Workflow configuration
#[derive(Debug, Clone, Deserialize)]
pub struct WorkflowConfig {
    pub execution: ExecutionConfig,
    pub steps: Vec<Step>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExecutionConfig {
    pub mode: ExecutionMode,  // parallel|serial|hybrid
    pub context_orchestration: Option<ContextOrchestrationConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContextOrchestrationConfig {
    pub rlm_threshold: Option<usize>,
    pub rag_threshold: Option<usize>,
    pub default_strategy: StrategyType,
    pub rag_config: Option<RAGConfig>,
    pub rlm_config: Option<RLMConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum StrategyType {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "direct")]
    Direct,
    #[serde(rename = "rag")]
    Rag,
    #[serde(rename = "rlm")]
    Rlm,
    #[serde(rename = "hybrid")]
    Hybrid,
}
```

## Usage Examples

### Example 1: Auto Selection (Default)

```yaml
# workspace.yaml
backends:
  rlm:
    max_depth: 1
  vector_store:
    type: chroma

# workflow.yaml
execution:
  context_orchestration:
    default_strategy: auto  # Let orchestrator decide
    rlm_threshold: 100000
    rag_threshold: 50000
```

**Behavior**:
- < 50K tokens → Direct LLM
- 50K-100K tokens → RAG (if vector store available)
- > 100K tokens → RLM (if RLM available)

### Example 2: Force RAG

```yaml
# workflow.yaml
execution:
  context_orchestration:
    default_strategy: rag
    rag_config:
      top_k: 10
```

**Behavior**:
- Always use RAG (if vector store available)
- Fallback to direct if vector store not in workspace

### Example 3: Force RLM

```yaml
# workflow.yaml
execution:
  context_orchestration:
    default_strategy: rlm
    rlm_config:
      depth: 1
```

**Behavior**:
- Always use RLM (if RLM available)
- Fallback to direct if RLM not in workspace

### Example 4: Per-Step Override

```yaml
# workflow.yaml
execution:
  context_orchestration:
    default_strategy: auto  # Workflow default

steps:
  - type: llm_context_aware
    id: step1
    # Uses workflow default (auto)
  
  - type: llm_context_aware
    id: step2
    context:
      strategy: rag  # Override to use RAG
```

**Behavior**:
- step1: Uses auto (workflow default)
- step2: Forces RAG (per-step override)

## Validation Rules

### Workspace Config Validation

```rust
impl WorkspaceConfig {
    pub fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();
        
        // At least one backend must be configured
        if self.backends.ollama.is_none() &&
           self.backends.llama_cpp.is_none() {
            errors.push(ValidationError::NoLLMBackend);
        }
        
        // If RLM configured, validate settings
        if let Some(rlm) = &self.backends.rlm {
            if rlm.max_depth > 1 {
                errors.push(ValidationError::InvalidRLMDepth);
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
```

### Workflow Config Validation

```rust
impl WorkflowConfig {
    pub fn validate(&self, workspace: &WorkspaceConfig) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();
        
        // Validate context orchestration settings
        if let Some(co) = &self.execution.context_orchestration {
            // Check thresholds make sense
            if let (Some(rag), Some(rlm)) = (&co.rag_threshold, &co.rlm_threshold) {
                if rag > rlm {
                    errors.push(ValidationError::InvalidThresholds);
                }
            }
            
            // Validate strategy compatibility with workspace
            match co.default_strategy {
                StrategyType::Rag => {
                    if workspace.backends.vector_store.is_none() {
                        errors.push(ValidationError::StrategyNotAvailable("RAG"));
                    }
                },
                StrategyType::Rlm => {
                    if workspace.backends.rlm.is_none() {
                        errors.push(ValidationError::StrategyNotAvailable("RLM"));
                    }
                },
                StrategyType::Hybrid => {
                    if workspace.backends.vector_store.is_none() || 
                       workspace.backends.rlm.is_none() {
                        errors.push(ValidationError::StrategyNotAvailable("Hybrid"));
                    }
                },
                _ => {},
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
```

## Configuration Loading

```rust
pub struct ConfigLoader {
    workspace_dir: PathBuf,
    workflow_dir: PathBuf,
}

impl ConfigLoader {
    pub fn load_all(&self) -> Result<(WorkspaceConfig, WorkflowConfig)> {
        // 1. Load workspace config
        let workspace_path = self.workspace_dir.join("workspace.yaml");
        let workspace: WorkspaceConfig = serde_yaml::from_str(
            &fs::read_to_string(&workspace_path)?
        )?;
        
        // 2. Validate workspace config
        workspace.validate()?;
        
        // 3. Load workflow config
        let workflow_path = self.workflow_dir.join("workflow.yaml");
        let workflow: WorkflowConfig = serde_yaml::from_str(
            &fs::read_to_string(&workflow_path)?
        )?;
        
        // 4. Validate workflow config (with workspace context)
        workflow.validate(&workspace)?;
        
        Ok((workspace, workflow))
    }
    
    pub fn load_workspace_only(&self) -> Result<WorkspaceConfig> {
        let workspace_path = self.workspace_dir.join("workspace.yaml");
        let workspace: WorkspaceConfig = serde_yaml::from_str(
            &fs::read_to_string(&workspace_path)?
        )?;
        workspace.validate()?;
        Ok(workspace)
    }
}
```

## Migration Path

### From Old Schema to New Schema

**Old** (incorrect):
```yaml
config:
  context_engine:
    enabled: true
    backends:
      rlm:
        enabled: true
```

**New** (correct):
```yaml
workspace:
  backends:
    rlm:
      max_depth: 1
```

Migration script:
```rust
pub fn migrate_config(old_path: &Path, new_path: &Path) -> Result<()> {
    // 1. Load old config
    let old_config: OldConfig = serde_yaml::from_str(&fs::read_to_string(old_path)?)?;
    
    // 2. Transform to new schema
    let new_workspace = WorkspaceConfig {
        backends: BackendsConfig {
            ollama: old_config.context_engine.backends.ollama.filter(|b| b.enabled),
            rlm: old_config.context_engine.backends.rlm.filter(|b| b.enabled),
            // Remove "enabled" field
        },
    };
    
    // 3. Write new config
    let yaml = serde_yaml::to_string(&new_workspace)?;
    fs::write(new_path, yaml)?;
    
    Ok(())
}
```

---

*Last updated: April 13, 2026*
