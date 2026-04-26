# Model Schema Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement model schema layer mapping unified-workflow-schema.yml `models` section to Rust structs with registry, resource management, and template interpolation (with schema version validation at load time).

**Architecture:** Models section → Rust structs → ModelRegistry → lifecycle management. Template interpolation at parse time (${models.model_name}), runtime values deferred.

**Tech Stack:** serde-saphyr (parsing), minijinja (templates), tokio (async), anyhow (errors).

**Prerequisites:** 01-provider-config.md (provider layer must exist)

---

## Scope

### Unified Schema Sections (unified-workflow-schema.yml lines 64-118)

```yaml
models:
  global_config_path: "./workspace/model-config.yml"
  default_router: automatic

  "primary-analyzer":
    name: "Primary Code Analyzer"
    host:
      type: llama_cpp_with_vulkan
      connection_settings: {}

    ram_allocation:
      strategy: dynamic
    max_allowed:
      ram: 13%
      vram: 3.7GB
      cpu: 49%
      gpu: 74%
      attention_tokens: 150000
      concurrent_requests: 2
    min_allowed:
      ram: 9%
      vram: 2.4GB
      cpu: 49%
      gpu: 74%
      attention_tokens: 73500

    model_memory:
      cache_size: min
      kv_cache_quantization: auto
      attention_context: auto

    execution:
      timeout:
        load_into_memory: 45s
        time_to_first_response: 1m
        total_time_to_response: 4h
      max_turns: 10
      stop_on_tool_failure: false
      accumulate_tool_results: true

    thinking:
      budget_tokens: 4096
      capture_in_output: true
      capture_in_events: true
```

### IN Scope
- Parse `models` section with serde-saphyr
- Implement ModelRegistry with load/unload/health_check
- Resource management (ram_allocation, max_allowed, min_allowed)
- Template interpolation at parse time (${models.model_name})
- Model overrides at step level (model_overrides in steps)
- Model lifecycle state tracking

### OUT of Scope
- RAG embeddings (deferred to Phase 2)
- Tools/guardrails under models (deferred to Phase 2)
- Sub-workflows
- Runtime template interpolation ({{step.name.output}})

---

## Files to Create

### 1. src/model/schema.rs

**Purpose:** Model schema structs mapping to unified schema models section.

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Models configuration section.
#[derive(Debug, Clone, Serialize, Deserialize, garde::Validate)]
pub struct ModelsConfig {
    /// Schema version (from unified-workflow-schema.yml line 804).
    #[serde(default = "default_schema_version")]
    #[garde(skip)]
    pub schema_version: String,

    /// Optional global config file path.
    #[serde(default)]
    pub global_config_path: Option<String>,

    /// Default routing strategy.
    #[serde(default = "default_router")]
    pub default_router: RouterStrategy,

    /// Model definitions (key = model ID, value = ModelSpec).
    #[serde(default)]
    pub models: HashMap<String, ModelSpec>,
}

/// Routing strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RouterStrategy {
    Automatic,
    Manual,
}

/// Model specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSpec {
    /// Human-readable name.
    pub name: String,

    /// Host configuration.
    pub host: ModelHost,

    /// RAM allocation strategy (presence = configured).
    pub ram_allocation: Option<RamAllocation>,

    /// Maximum resource limits.
    #[serde(default)]
    pub max_allowed: MaxAllowed,

    /// Minimum resource requirements.
    #[serde(default)]
    pub min_allowed: MinAllowed,

    /// Model memory settings.
    #[serde(default)]
    pub model_memory: ModelMemory,

    /// Execution settings.
    #[serde(default)]
    pub execution: ExecutionConfig,

    /// Thinking mode (presence = enabled).
    pub thinking: Option<ThinkingConfig>,
}

/// Host configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelHost {
    /// Provider type (only llama_cpp_with_vulkan in POC).
    #[serde(rename = "type")]
    pub provider_type: String,

    /// Provider-specific connection settings (overrides provider defaults).
    #[serde(default)]
    pub connection_settings: serde_json::Value,
}

/// RAM allocation strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RamAllocationStrategy {
    Static,
    Dynamic,
}

/// RAM allocation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RamAllocation {
    #[serde(rename = "strategy")]
    pub allocation_strategy: RamAllocationStrategy,
}

/// Maximum resource limits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaxAllowed {
    /// RAM limit (e.g., "13%" or "8GB").
    pub ram: ResourceLimit,

    /// VRAM limit (e.g., "3.7GB").
    #[serde(default)]
    pub vram: Option<ResourceLimit>,

    /// CPU limit (e.g., "49%").
    #[serde(default)]
    pub cpu: Option<ResourceLimit>,

    /// GPU limit (e.g., "74%").
    #[serde(default)]
    pub gpu: Option<ResourceLimit>,

    /// Attention tokens limit.
    #[serde(default)]
    pub attention_tokens: Option<usize>,

    /// Concurrent requests limit.
    #[serde(default)]
    pub concurrent_requests: Option<usize>,
}

/// Minimum resource requirements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinAllowed {
    /// RAM minimum.
    pub ram: ResourceLimit,

    /// VRAM minimum.
    #[serde(default)]
    pub vram: Option<ResourceLimit>,

    /// CPU minimum.
    #[serde(default)]
    pub cpu: Option<ResourceLimit>,

    /// GPU minimum.
    #[serde(default)]
    pub gpu: Option<ResourceLimit>,

    /// Attention tokens minimum.
    #[serde(default)]
    pub attention_tokens: Option<usize>,
}

/// Resource limit (percentage or absolute).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResourceLimit {
    Percentage(String),
    Absolute(String),
}

/// Model memory configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMemory {
    #[serde(default = "default_cache_size")]
    pub cache_size: CacheSize,

    #[serde(default = "default_kv_quantization")]
    pub kv_cache_quantization: KvQuantization,

    #[serde(default = "default_attention_context")]
    pub attention_context: AttentionContext,
}

/// Cache size strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CacheSize {
    Min,
    Max,
    Medium,
    MediumMin,
    MediumMax,
}

/// KV cache quantization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KvQuantization {
    Auto,
    Q4Km,
    Q4_0,
    Q5Km,
    Q5_0,
    Q6K,
    Q8_0,
}

/// Attention context strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AttentionContext {
    Auto,
    FromMaxAllowed,
    ManualOverride,
}

/// Execution configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionConfig {
    /// Timeout settings.
    #[serde(default)]
    pub timeout: TimeoutConfig,

    /// Maximum agent turns.
    #[serde(default = "default_max_turns")]
    pub max_turns: usize,

    #[serde(default = "default_stop_on_tool_failure")]
    pub stop_on_tool_failure: bool,

    #[serde(default = "default_accumulate_tool_results")]
    pub accumulate_tool_results: bool,
}

/// Timeout configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutConfig {
    /// Time to load model into memory.
    #[serde(default = "default_load_timeout")]
    pub load_into_memory: String,

    /// Time to first token.
    #[serde(default = "default_ttfb_timeout")]
    pub time_to_first_response: String,

    /// Total time budget.
    #[serde(default = "default_total_timeout")]
    pub total_time_to_response: String,
}

/// Thinking mode configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThinkingConfig {
    /// Budget in tokens (0-16384).
    #[serde(default = "default_thinking_budget")]
    pub budget_tokens: usize,

    #[serde(default)]
    pub capture_in_output: bool,

    #[serde(default)]
    pub capture_in_events: bool,
}

// Defaults
fn default_schema_version() -> String {
    "2.0.0".into()
}

fn default_router() -> RouterStrategy {
    RouterStrategy::Automatic
}

fn default_cache_size() -> CacheSize {
    CacheSize::Min
}

fn default_kv_quantization() -> KvQuantization {
    KvQuantization::Auto
}

fn default_attention_context() -> AttentionContext {
    AttentionContext::Auto
}

fn default_max_turns() -> usize {
    10
}

fn default_stop_on_tool_failure() -> bool {
    false
}

fn default_accumulate_tool_results() -> bool {
    true
}

fn default_load_timeout() -> String {
    "45s".into()
}

fn default_ttfb_timeout() -> String {
    "1m".into()
}

fn default_total_timeout() -> String {
    "4h".into()
}

fn default_thinking_budget() -> usize {
    4096
}
```

### 2. src/model/registry.rs

**Purpose:** ModelRegistry with lifecycle management.

```rust
use super::schema::{ModelSpec, ModelsConfig};
use super::resource::ResourceManager;

use crate::backend::llm_backend::{LlmBackend, HealthStatus};

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Model lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelState {
    Unloaded,
    Loading,
    Loaded,
    Unloading,
    Error,
}

/// Model entry in registry.
struct ModelEntry {
    spec: ModelSpec,
    state: ModelState,
    backend: Option<Arc<dyn LlmBackend>>,
}

/// Model registry.
pub struct ModelRegistry {
    models: Arc<RwLock<HashMap<String, ModelEntry>>>,
    resource_manager: ResourceManager,
}

impl ModelRegistry {
    /// Create new registry from config.
    pub fn new(config: ModelsConfig, resource_manager: ResourceManager) -> Self {
        let mut models = HashMap::new();

        for (model_id, spec) in config.models {
            models.insert(
                model_id.clone(),
                ModelEntry {
                    spec,
                    state: ModelState::Unloaded,
                    backend: None,
                },
            );
        }

        Self {
            models: Arc::new(RwLock::new(models)),
            resource_manager,
        }
    }

    /// Load model into memory.
    pub async fn load_model(&self, model_id: &str) -> anyhow::Result<()> {
        let mut models = self.models.write().await;

        let entry = models
            .get_mut(model_id)
            .ok_or_else(|| anyhow::anyhow!("Model {} not found", model_id))?;

        match entry.state {
            ModelState::Loaded => {
                tracing::info!(model = model_id, "Model already loaded");
                return Ok(());
            }
            ModelState::Loading => {
                return Err(anyhow::anyhow!("Model {} is already loading", model_id));
            }
            _ => {}
        }

        entry.state = ModelState::Loading;
        drop(models); // Release lock before async operation

        // Check resource availability
        self.resource_manager
            .check_resources(&entry.spec)
            .await?;

        // TODO: Create backend instance from provider config
        // let backend = self.create_backend(&entry.spec).await?;
        // backend.load_model(model_id).await?;

        let mut models = self.models.write().await;
        if let Some(entry) = models.get_mut(model_id) {
            entry.state = ModelState::Loaded;
            // entry.backend = Some(Arc::new(backend));
        }

        tracing::info!(model = model_id, "Model loaded successfully");
        Ok(())
    }

    /// Unload model from memory.
    pub async fn unload_model(&self, model_id: &str) -> anyhow::Result<()> {
        let mut models = self.models.write().await;

        let entry = models
            .get_mut(model_id)
            .ok_or_else(|| anyhow::anyhow!("Model {} not found", model_id))?;

        match entry.state {
            ModelState::Unloaded => {
                tracing::info!(model = model_id, "Model already unloaded");
                return Ok(());
            }
            ModelState::Unloading => {
                return Err(anyhow::anyhow!("Model {} is already unloading", model_id));
            }
            _ => {}
        }

        entry.state = ModelState::Unloading;
        drop(models);

        // TODO: Unload from backend
        // if let Some(backend) = entry.backend {
        //     backend.unload_model(model_id).await?;
        // }

        let mut models = self.models.write().await;
        if let Some(entry) = models.get_mut(model_id) {
            entry.state = ModelState::Unloaded;
            entry.backend = None;
        }

        self.resource_manager.release_resources(model_id).await?;
        tracing::info!(model = model_id, "Model unloaded successfully");
        Ok(())
    }

    /// Check model health.
    pub async fn health_check(&self, model_id: &str) -> anyhow::Result<HealthStatus> {
        let models = self.models.read().await;

        let entry = models
            .get(model_id)
            .ok_or_else(|| anyhow::anyhow!("Model {} not found", model_id))?;

        if let Some(backend) = &entry.backend {
            Ok(backend.health_check().await?)
        } else {
            Ok(HealthStatus::Unhealthy)
        }
    }

    /// Get model specification.
    pub async fn get_spec(&self, model_id: &str) -> Option<ModelSpec> {
        let models = self.models.read().await;
        models.get(model_id).map(|e| e.spec.clone())
    }

    /// List all registered models.
    pub async fn list_models(&self) -> Vec<String> {
        let models = self.models.read().await;
        models.keys().cloned().collect()
    }
}
```

### 3. src/model/resource.rs

**Purpose:** Resource management (RAM, VRAM, CPU, GPU).

```rust
use super::schema::{ResourceLimit, MaxAllowed, MinAllowed};

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Resource manager for tracking allocations.
#[derive(Clone)]
pub struct ResourceManager {
    allocations: Arc<RwLock<HashMap<String, ResourceAllocation>>>,
    total_resources: TotalResources,
}

/// Current resource allocation.
#[derive(Debug, Clone)]
struct ResourceAllocation {
    ram_mb: usize,
    vram_mb: usize,
    cpu_percent: usize,
    gpu_percent: usize,
}

/// Total available resources.
#[derive(Debug, Clone)]
pub struct TotalResources {
    pub ram_mb: usize,
    pub vram_mb: usize,
    pub cpu_cores: usize,
}

impl ResourceManager {
    /// Create new resource manager.
    pub fn new(total_resources: TotalResources) -> Self {
        Self {
            allocations: Arc::new(RwLock::new(HashMap::new())),
            total_resources,
        }
    }

    /// Parse resource limit string.
    fn parse_limit(limit: &ResourceLimit) -> anyhow::Result<usize> {
        match limit {
            ResourceLimit::Percentage(s) => {
                let pct: f64 = s
                    .trim_end_matches('%')
                    .parse()
                    .map_err(|e| anyhow::anyhow!("Invalid percentage: {}", e))?;

                Ok((pct / 100.0 * 100.0) as usize) // Assume 100MB base
            }
            ResourceLimit::Absolute(s) => {
                if s.contains("GB") {
                    let gb: f64 = s
                        .trim_end_matches("GB")
                        .parse()
                        .map_err(|e| anyhow::anyhow!("Invalid GB value: {}", e))?;

                    Ok((gb * 1024.0) as usize)
                } else if s.contains("MB") {
                    let mb: usize = s
                        .trim_end_matches("MB")
                        .parse()
                        .map_err(|e| anyhow::anyhow!("Invalid MB value: {}", e))?;

                    Ok(mb)
                } else {
                    Err(anyhow::anyhow!("Unknown unit: {}", s))
                }
            }
        }
    }

    /// Check if model resources are available.
    pub async fn check_resources(
        &self,
        max_allowed: &MaxAllowed,
    ) -> anyhow::Result<()> {
        let required_ram = Self::parse_limit(&max_allowed.ram)?;
        let required_vram = max_allowed
            .vram
            .as_ref()
            .map(|l| Self::parse_limit(l))
            .transpose()?
            .unwrap_or(0);

        let allocations = self.allocations.read().await;
        let used_ram: usize = allocations.values().map(|a| a.ram_mb).sum();
        let used_vram: usize = allocations.values().map(|a| a.vram_mb).sum();

        if used_ram + required_ram > self.total_resources.ram_mb {
            return Err(anyhow::anyhow!(
                "Insufficient RAM: {}MB required, {}MB available",
                used_ram + required_ram,
                self.total_resources.ram_mb
            ));
        }

        if used_vram + required_vram > self.total_resources.vram_mb {
            return Err(anyhow::anyhow!(
                "Insufficient VRAM: {}MB required, {}MB available",
                used_vram + required_vram,
                self.total_resources.vram_mb
            ));
        }

        Ok(())
    }

    /// Release resources for model.
    pub async fn release_resources(&self, model_id: &str) -> anyhow::Result<()> {
        let mut allocations = self.allocations.write().await;
        allocations.remove(model_id);
        Ok(())
    }

    /// Allocate resources for model.
    pub async fn allocate_resources(
        &self,
        model_id: String,
        max_allowed: &MaxAllowed,
    ) -> anyhow::Result<()> {
        let ram_mb = Self::parse_limit(&max_allowed.ram)?;
        let vram_mb = max_allowed
            .vram
            .as_ref()
            .map(|l| Self::parse_limit(l))
            .transpose()?
            .unwrap_or(0);

        let mut allocations = self.allocations.write().await;
        allocations.insert(
            model_id,
            ResourceAllocation {
                ram_mb,
                vram_mb,
                cpu_percent: 0,
                gpu_percent: 0,
            },
        );

        Ok(())
    }
}
```

### 4. src/model/interpolation.rs

**Purpose:** Template interpolation with Minijinja (${models.model_name} at parse time).

```rust
use minijinja::{Environment, Error};

/// Template interpolator for parse-time variable resolution.
pub struct TemplateInterpolator {
    env: Environment<'static>,
}

impl TemplateInterpolator {
    /// Create new interpolator.
    pub fn new() -> Self {
        let mut env = Environment::new();
        env.set_debug(false);

        Self { env }
    }

    /// Interpolate template with variables.
    ///
    /// Supports ${models.model_name} syntax (parse-time).
    /// Does NOT support {{step.name.output}} (runtime, deferred to Phase 2).
    pub fn interpolate(
        &self,
        template: &str,
        variables: &std::collections::HashMap<String, String>,
    ) -> Result<String, Error> {
        // Convert ${models.model_name} to {{models.model_name}} for Minijinja
        let template = template.replace("${", "{{").replace("}", "}}");

        self.env.render_str(&template, variables)
    }

    /// Parse variable reference (e.g., "models.primary-analyzer").
    pub fn parse_var_ref(s: &str) -> Option<(String, String)> {
        // Extract "${models.model_name}" pattern
        if s.starts_with("${models.") && s.ends_with("}") {
            let model_name = &s["${models.".len()..s.len() - 1];
            Some(("models".into(), model_name.into()))
        } else {
            None
        }
    }
}

impl Default for TemplateInterpolator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolate_model_ref() {
        let interpolator = TemplateInterpolator::new();

        let mut vars = std::collections::HashMap::new();
        vars.insert("models.primary-analyzer".into(), "llama-3.2-3b-instruct".into());

        let template = "Model: ${models.primary-analyzer}";
        let result = interpolator.interpolate(template, &vars).unwrap();

        assert!(result.contains("llama-3.2-3b-instruct"));
    }

    #[test]
    fn parse_var_ref() {
        let s = "${models.primary-analyzer}";
        let result = TemplateInterpolator::parse_var_ref(s);

        assert_eq!(result, Some(("models".into(), "primary-analyzer".into())));
    }
}
```

### 5. src/model/mod.rs

**Purpose:** Export model modules.

```rust
pub mod schema;
pub mod registry;
pub mod resource;
pub mod interpolation;

pub use schema::{ModelsConfig, ModelSpec, ExecutionConfig, ThinkingConfig};
pub use registry::{ModelRegistry, ModelState};
pub use resource::{ResourceManager, TotalResources, ResourceAllocation};
pub use interpolation::TemplateInterpolator;
```

### 6. tests/model_schema_test.rs

**Purpose:** Unit tests for model schema and interpolation.

```rust
use whitt::model::{ModelsConfig, TemplateInterpolator};

#[test]
fn parse_model_spec() {
    let yaml = r#"
models:
  "test-model":
    name: "Test Model"
    host:
      type: llama_cpp_with_vulkan
      connection_settings: {}
    execution:
      max_turns: 10
"#;

    let config: ModelsConfig = serde_saphyr::from_str(yaml).expect("parse");
    assert!(config.models.contains_key("test-model"));
}

#[test]
fn interpolate_template_variables() {
    let interpolator = TemplateInterpolator::new();

    let mut vars = std::collections::HashMap::new();
    vars.insert("models.primary".into(), "llama-3.2".into());

    let template = "Use model ${models.primary}";
    let result = interpolator.interpolate(template, &vars).unwrap();

    assert!(result.contains("llama-3.2"));
}

#[test]
fn parse_resource_limit_percentage() {
    let yaml = r#"
max_allowed:
  ram: 13%
  vram: 3.7GB
"#;

    let config: serde_saphyr::from_str(yaml).expect("parse");
    assert_eq!(config.ram, ResourceLimit::Percentage("13%".into()));
}

#[test]
fn model_override_resolution() {
    let yaml = r#"
models:
  "base-model":
    name: "Base"
    execution:
      max_turns: 10

steps:
  test_step:
    generative_entity: "${models.base-model}"
    model_overrides:
      max_turns: 5
"#;

    // Step-level override should take precedence
    let config: serde_saphyr::from_str(yaml).expect("parse");
    // TODO: Add assertion for override resolution
}
```

---

## Files to Modify

### 1. Cargo.toml

**Purpose:** Add minijinja dependency.

```toml
[dependencies]
# Existing...
minijinja = "2.5"
```

### 2. src/config/unified.rs

**Purpose:** Export ModelsConfig for use by model layer.

```rust
pub use crate::model::ModelsConfig;
pub use crate::model::ModelSpec;
pub use crate::model::ExecutionConfig;
pub use crate::model::ThinkingConfig;
```

---

## Implementation Tasks

### Task 1: Create model schema structs

**Files:**
- Create: `src/model/schema.rs`
- Create: `src/model/mod.rs`

- [ ] **Step 1: Write schema.rs with all structs**

```rust
// Copy from Files to Create section above
```

- [ ] **Step 2: Create model/mod.rs**

```rust
// Copy from Files to Create section above
```

- [ ] **Step 3: Verify schema compiles**

Run: `cargo check --lib`
Expected: PASS

- [ ] **Step 4: Run unit tests**

Run: `cargo test model_schema`
Expected: All PASS

- [ ] **Step 5: Commit**

```bash
git add src/model/schema.rs src/model/mod.rs
git commit -m "feat: add model schema structs matching unified schema"
```

### Task 2: Implement ModelRegistry

**Files:**
- Create: `src/model/registry.rs`

- [ ] **Step 1: Write registry.rs with ModelRegistry**

```rust
// Copy from Files to Create section above
```

- [ ] **Step 2: Verify registry compiles**

Run: `cargo check --lib`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add src/model/registry.rs
git commit -m "feat: implement ModelRegistry with lifecycle management"
```

### Task 3: Implement ResourceManager

**Files:**
- Create: `src/model/resource.rs`

- [ ] **Step 1: Write resource.rs with resource tracking**

```rust
// Copy from Files to Create section above
```

- [ ] **Step 2: Verify resource manager compiles**

Run: `cargo check --lib`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add src/model/resource.rs
git commit -m "feat: implement ResourceManager for allocation tracking"
```

### Task 4: Implement TemplateInterpolator

**Files:**
- Create: `src/model/interpolation.rs`

- [ ] **Step 1: Write interpolation.rs with Minijinja**

```rust
// Copy from Files to Create section above
```

- [ ] **Step 2: Add minijinja dependency**

Run: `cargo add minijinja`
Expected: Add minijinja to Cargo.toml

- [ ] **Step 3: Verify interpolator compiles**

Run: `cargo check --lib`
Expected: PASS

- [ ] **Step 4: Run interpolation tests**

Run: `cargo test interpolation`
Expected: All PASS

- [ ] **Step 5: Commit**

```bash
git add src/model/interpolation.rs Cargo.toml
git commit -m "feat: implement TemplateInterpolator with Minijinja"
```

### Task 5: Integrate model layer with provider

**Files:**
- Modify: `src/config/unified.rs`

- [ ] **Step 1: Export ModelsConfig in unified.rs**

```rust
pub use crate::model::ModelsConfig;
```

- [ ] **Step 2: Verify integration compiles**

Run: `cargo check --lib`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add src/config/unified.rs
git commit -m "feat: export ModelsConfig from unified config"
```

---

## Success Criteria

- [ ] Model schema structs match unified schema lines 64-118
- [ ] ModelRegistry implements load/unload/health_check
- [ ] ResourceManager tracks RAM, VRAM, CPU, GPU allocations
- [ ] TemplateInterpolator resolves ${models.model_name} at parse time
- [ ] Model overrides at step level supported (deferred to 03-agent-react)
- [ ] Unit tests pass (cargo test model_schema)
- [ ] LSP diagnostics clean on all changed files
- [ ] Build passes (cargo build)
