# Llama.cpp Vulkan Integration

## Overview

Integration of llama.cpp's Vulkan backend for GPU-accelerated LLM inference. llama.cpp supports Vulkan as a high-performance compute backend for running models on GPU devices with fine-grained control over layer offloading and memory budgets.

---

## Why Vulkan?

### Performance Characteristics

| Backend | Throughput | Latency | Memory | Platform Support |
|---------|------------|---------|---------|------------------|
| **Vulkan** | 3-5x CPU | 2-5x faster | Efficient | Linux, Windows, macOS (via MoltenVK) |
| CUDA | Baseline | Baseline | Efficient | Linux (NVIDIA only) |
| Metal | 2-3x CPU | Moderate | Efficient | macOS (Apple Silicon only) |
| CPU | 1x (baseline) | High | Low memory | Cross-platform |

### Advantages

1. **Cross-vendor**: Works on NVIDIA, AMD, Intel, Apple Silicon GPUs (via MoltenVK)
2. **Fine-grained control**: Per-layer offload decisions (not all-or-nothing)
3. **Explicit memory management**: Memory budgets prevent OOM
4. **No external dependencies**: Vulkan drivers included with OS (no CUDA toolkit needed)
5. **Fallback chain**: Vulkan → Metal → CPU automatic fallback

### When to Use Vulkan

- **Large models**: 7B+ parameters benefit significantly from GPU acceleration
- **High-throughput scenarios**: Batch processing, multiple concurrent requests
- **Latency-sensitive**: Interactive workflows where response time matters
- **Resource-constrained**: Need to fit models within GPU memory budget

### When CPU May Be Preferable

- **Small models**: < 1B parameters (CPU fast enough, GPU overhead dominates)
- **Low-latency inference**: Model load time may exceed generation time for small prompts
- **Development/testing**: Faster iteration without GPU setup
- **No GPU available**: Fallback to CPU without workflow changes

---

## Integration Architecture

### Layer 1: Vulkan Backend Initialization

**File**: `src/backends/llamacpp/vulkan.rs`

**Responsibilities:**
- Enumerate available GPU devices via `vulkan-rs`
- Select best device based on heuristics (memory, compute units)
- Initialize Vulkan instance with required extensions
- Configure backend for llama.cpp

**Example:**
```rust
use vulkan_rs::{
    Instance, PhysicalDevice, Device, QueueFamily,
};
use llamacpp_2::VulkanBackend;

pub fn initialize_vulkan() -> Result<(VulkanBackend, VulkanDevice)> {
    // 1. Create Vulkan instance
    let instance = Instance::new(None, &vk_entry)?;

    // 2. Enumerate physical devices
    let devices = instance.enumerate_physical_devices()?;

    // 3. Select device (prefer discrete GPU)
    let device = select_best_device(&devices)?;

    // 4. Create logical device with queues
    let (logical_device, queue) = create_device(&device)?;

    // 5. Initialize llama.cpp Vulkan backend
    let vulkan_backend = VulkanBackend::new(&device)?;

    Ok((vulkan_backend, logical_device))
}

fn select_best_device(devices: &[PhysicalDevice]) -> Result<PhysicalDevice> {
    devices.iter()
        .filter(|d| d.properties().device_type == vk::PhysicalDeviceType::DISCRETE_GPU)
        .max_by_key(|d| d.properties().limits.max_memory_allocation_size)
        .copied()
}
```

**Configuration:**
```yaml
model:
  backend: "llamacpp"
  vulkan:
    device_preference: "auto"  # auto, nvidia, amd, intel
    memory_limit_gb: 16
    enable_validation: true
```

---

### Layer 2: GPU Offload Control

**File**: `src/backends/llamacpp/gpu_offload.rs`

**Responsibilities:**
- Parse layer offload configuration from unified schema
- Calculate per-layer memory requirements
- Validate against GPU memory budget
- Configure llama.cpp offload strategy

**Offload Strategies:**

| Strategy | Description | Use Case |
|----------|-------------|----------|
| **Percentage** | Offload N% of layers to GPU | General-purpose, balanced |
| **Explicit layers** | Specify exact layers to offload | Fine-tuned performance |
| **All layers** | Offload everything that fits | Maximum throughput |
| **None** | Force CPU execution | Debugging, small models |

**Configuration from Unified Schema:**
```yaml
model:
  backend: "llamacpp"
  gpu_offload:
    strategy: "percentage"  # percentage, explicit, none
    layers_to_gpu: 80  # 80% of layers
    memory_budget_gb: 12  # GPU memory limit
    fallback: "cpu"  # fallback to CPU if OOM
```

**Example Per-Layer Configuration:**
```yaml
model:
  gpu_offload:
    layer_rules:
      - layer: "q_proj"
        offload: true
        priority: "high"
      - layer: "k_proj"
        offload: true
        priority: "high"
      - layer: "v_proj"
        offload: false  # Force CPU for attention projection
        priority: "low"
```

**Validation Logic:**
```rust
pub fn validate_offload_config(
    config: &OffloadConfig,
    model: &ModelSpec,
    gpu_memory_gb: u64,
) -> Result<OffloadPlan> {
    // 1. Calculate total model memory
    let model_memory = calculate_model_memory_mb(model)?;

    // 2. Check against budget
    if model_memory > gpu_memory_gb * 1024 {
        return Err(OffloadError::InsufficientMemory {
            required_mb: model_memory,
            available_mb: gpu_memory_gb * 1024,
        });
    }

    // 3. Apply offload strategy
    let plan = match config.strategy {
        Strategy::Percentage(pct) => plan_percentage_offload(pct, model),
        Strategy::Explicit(layers) => plan_explicit_offload(layers, model),
        Strategy::None => Ok(OffloadPlan::cpu_only()),
    };

    Ok(plan)
}
```

---

### Layer 3: Model Lifecycle with Vulkan

**File**: `src/backends/llamacpp/model_lifecycle.rs`

**Responsibilities:**
- Backend initialization sequence with error handling
- Model load with Vulkan context
- KV-cache allocation on GPU or CPU
- Vocabulary loading
- Resource cleanup on failure

**Load Sequence:**
```
1. Parse model file (GGUF format)
   ↓
2. Select backend (Vulkan preferred)
   ↓
3. Initialize Vulkan instance
   ↓
4. Allocate GPU memory for model weights
   ↓
5. Transfer weights to GPU memory
   ↓
6. Allocate KV-cache buffer (GPU or CPU based on offload)
   ↓
7. Load vocabulary to CPU
   ↓
8. Ready for inference
```

**Error Handling:**
```rust
pub async fn load_model_vulkan(
    model_path: &str,
    offload_config: &OffloadConfig,
) -> Result<LoadedModel> {
    // 1. Initialize Vulkan backend
    let (backend, vulkan_device) = initialize_vulkan().await?;

    // 2. Load model with Vulkan context
    let model = match llamacpp_2::load_model_with_vulkan(
        model_path,
        &vulkan_device,
        &backend,
    ).await {
        Ok(m) => m,
        Err(e) => {
            // 3. Fallback to CPU if Vulkan fails
            warn!("Vulkan load failed, falling back to CPU: {}", e);
            load_model_cpu(model_path).await?
        }
    };

    // 4. Setup KV-cache on appropriate device
    let kv_cache = if offload_config.offload_kv_cache {
        allocate_kv_cache_gpu(&vulkan_device).await?
    } else {
        allocate_kv_cache_cpu().await?
    };

    Ok(LoadedModel { model, kv_cache })
}
```

**Cleanup on Failure:**
```rust
impl Drop for LoadedModel {
    fn drop(&mut self) {
        // 1. Signal cancellation to in-flight requests
        self.cancel_requests();

        // 2. Wait for pending completions
        tokio::time::timeout(Duration::from_secs(5)).await;

        // 3. Free GPU resources in correct order
        drop(self.kv_cache);  // Drop KV-cache first
        drop(self.vulkan_context);  // Then Vulkan context
        drop(self.model_weights);  // Then weights
    }
}
```

---

### Layer 4: Multi-Instance Management

**File**: `src/backends/llamacpp/model_manager.rs`

**Responsibilities:**
- Multi-instance configuration (multiple models loaded concurrently)
- Instance selection logic for routing requests
- Model switching support (unload A, load B)
- Thread-safe access (Arc<RwLock>)

**Instance Registry:**
```rust
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ModelRegistry {
    instances: Arc<RwLock<HashMap<ModelId, ModelInstance>>>,
}

impl ModelRegistry {
    pub async fn get_or_load(
        &self,
        model_id: &str,
        config: &ModelConfig,
    ) -> Result<Arc<ModelInstance>> {
        // 1. Check if already loaded
        {
            let registry = self.instances.read().await;
            if let Some(instance) = registry.get(model_id) {
                return Ok(instance);
            }
        }

        // 2. Load model (Vulkan or CPU based on config)
        let instance = load_model_with_config(config).await?;

        // 3. Register in registry
        {
            let mut registry = self.instances.write().await;
            registry.insert(model_id.to_string(), instance);
        }

        Ok(Arc::new(instance))
    }

    pub async fn switch_model(
        &self,
        from_id: &str,
        to_id: &str,
    ) -> Result<()> {
        // 1. Unload old model (Vulkan cleanup)
        {
            let mut registry = self.instances.write().await;
            if let Some(old) = registry.remove(from_id) {
                drop(old);  // Triggers Vulkan cleanup
            }
        }

        // 2. Load new model
        self.get_or_load(to_id, config).await?;

        Ok(())
    }
}
```

**Thread Safety:**
- **Read-heavy access**: Most operations only need read access (get_model)
- **Coordinated writes**: Model load/switch requires write lock
- **Arc wrapping**: Share instances across workers without copying

---

## Unified Schema Integration

### New Fields for Model Configuration

Add to unified schema `model:` section:

```yaml
model:
  # ... existing fields ...
  backend: "llamacpp"
  vulkan:
    enabled: true
    device_preference: "auto"  # auto, nvidia, amd, intel
    memory_limit_gb: 16
    fallback_to_cpu: true
  gpu_offload:
    strategy: "percentage"  # percentage, explicit, none
    layers_to_gpu: 80
    kv_cache_on_gpu: true
    memory_budget_gb: 12
```

**Feature Flags:**
```toml
[dependencies]
llamacpp-2 = { version = "0.1.0", features = ["vulkan"] }
vulkan-rs = "0.1.0"

[features]
default = ["vulkan-support"]
cpu-only = []
```

---

## Dependencies

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **llama-cpp-2** | 0.1.0+ | Rust bindings for llama.cpp with Vulkan support | **REQUIRED** |
| **vulkan-rs** | 0.1.0+ | GPU device querying, Vulkan instance management | **REQUIRED** |
| **ash** (optional) | 0.38+ | Alternative to vulkan-rs, lower-level | Optional |
| **tokio** | 1.0+ | Async runtime for GPU operations | Already in project |

---

## Performance Considerations

### GPU Memory Management

| Model Size | Weights | KV-cache | Context (4K) | Total | GPU Needed |
|-----------|---------|---------|---------------|-------|------------|
| 1B | ~4 GB | ~1 GB | ~2 GB | ~7 GB |
| 7B | ~28 GB | ~2 GB | ~2 GB | ~32 GB |
| 13B | ~52 GB | ~2 GB | ~2 GB | ~56 GB |

**Recommendation:**
- 7B models require 32+ GB GPU memory (RTX 4090 minimum)
- 1B models fit on 8 GB GPUs (RTX 3060, RX 6600 XT)
- KV-cache can be offloaded to CPU if GPU memory tight

### Layer Offload Trade-offs

| Offload % | Throughput | GPU Memory | Latency |
|-----------|------------|------------|---------|
| 100% | Max | Max | Lowest (all GPU) |
| 80% | 0.95x max | 0.80x max | Low |
| 50% | 0.60x max | 0.50x max | Moderate |
| 0% (CPU) | 1x (baseline) | Min | Highest (no GPU) |

**Guideline:**
- Start with 80% offload for balanced performance
- Reduce if OOM errors occur
- Increase to 100% for throughput-critical workflows

---

## Testing

### Unit Tests

```rust
#[tokio::test]
async fn test_vulkan_device_selection() {
    let devices = enumerate_vulkan_devices().await.unwrap();
    assert!(devices.len() > 0, "At least one GPU device");

    // Verify discrete GPU preferred
    let selected = select_device(&devices);
    assert!(selected.is_discrete());
}

#[tokio::test]
async fn test_gpu_offload_validation() {
    let config = OffloadConfig {
        strategy: Strategy::Percentage(80),
        memory_budget_gb: 8,
    };

    let result = validate_offload_config(&config, &model_7b()).await;
    assert!(result.is_ok());
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_vulkan_model_load() {
    // Requires actual GPU for this test
    if !has_vulkan_gpu() {
        return; // Skip on CI
    }

    let model = load_model_vulkan("test-model.gguf", &default_config()).await.unwrap();
    assert!(model.is_gpu_accelerated());

    let response = model.generate("Hello").await.unwrap();
    assert!(response.tokens.len() > 0);
}
```

### Benchmarks

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

criterion_group!(benches);
criterion_main!(benches);

fn bench_vulkan_inference(c: &mut Criterion) {
    c.bench_function("vulkan_7b", |b| {
        b.iter(|| {
            let model = load_model_vulkan("7b.gguf", &config);
            let _ = block_on(model.generate("test prompt"));
        });
    });
}

fn bench_cpu_inference(c: &mut Criterion) {
    c.bench_function("cpu_7b", |b| {
        b.iter(|| {
            let model = load_model_cpu("7b.gguf", &config);
            let _ = block_on(model.generate("test prompt"));
        });
    });
}
```

---

## Platform Support Matrix

| Platform | Vulkan Support | llama.cpp Support | Implementation Status |
|----------|----------------|------------------|-------------------|
| **Linux** | Native | Native | ✅ Full support |
| **Windows** | Native | Native (via Vulkan loaders) | ✅ Full support |
| **macOS** | Via MoltenVK | Native (via MoltenVK) | ✅ Full support |
| **Docker** | Passthrough | GPU passthrough | ⚠️ Requires `--gpus all` flag |
| **WSL2** | Via MoltenVK | GPU passthrough | ⚠️ Requires GPU passthrough setup |

---

## Error Handling

### Common Vulkan Errors

| Error | Cause | Recovery |
|--------|--------|-----------|
| **VK_ERROR_OUT_OF_HOST_MEMORY** | GPU memory full | Reduce offload % or smaller model |
| **VK_ERROR_DEVICE_LOST** | GPU crash | Fallback to CPU, reinitialize |
| **VK_ERROR_LAYER_NOT_PRESENT** | Extension missing | Fallback to CPU, log warning |
| **llama-cpp load failure** | Model corrupt or invalid | Try CPU fallback, report error |

**Fallback Chain:**
```
Vulkan enabled? → Try Vulkan load
   ↓
Success? → Model ready
   ↓
Fail? → Fallback to CPU load
   ↓
Success? → Model ready (CPU)
   ↓
Fail? → Error to user (check model file)
```

---

## Monitoring and Diagnostics

### Metrics to Collect

```rust
pub struct VulkanMetrics {
    pub gpu_memory_used_mb: u64,
    pub gpu_memory_limit_mb: u64,
    pub layers_offloaded_count: usize,
    pub layers_cpu_count: usize,
    pub inference_time_ms: u64,
    pub tokens_per_second: f64,
    pub oom_count: u64,
}
```

**Diagnostics Output:**
```yaml
vulkan_status:
  backend: "vulkan"
  device_name: "NVIDIA GeForce RTX 4090"
  device_memory_gb: 24
  current_usage_gb: 14.2
  offloaded_layers: 42
  cpu_layers: 10
  inference_stats:
    avg_time_ms: 245
    tokens_per_sec: 42.3
```

---

## Next Steps

1. Implement `vulkan.rs` module with device enumeration and initialization
2. Implement `gpu_offload.rs` with layer offload configuration and validation
3. Add Vulkan-specific tests to `tests/backends/llamacpp_test.rs`
4. Add Vulkan feature to `Cargo.toml` with optional feature flag
5. Document platform-specific setup (Docker, WSL2, GPU passthrough)
6. Integrate with ModelProvider trait for unified backend abstraction
7. Add performance benchmarks comparing Vulkan vs CPU vs CUDA (if available)
