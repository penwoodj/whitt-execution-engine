# UF01: Model Discovery and Registration

## Overview
Scans local LLM providers (LM Studio, Ollama, llama.cpp), discovers available models, validates resource constraints, and builds a benchmark manifest YAML.

## User Story
As a benchmark operator, I want to automatically discover all models available across my local providers and register them in a benchmark manifest so that I can run benchmarks against 100+ models without manual configuration.

## Pre-conditions
- At least one LLM provider is running (LM Studio, Ollama, or llama.cpp)
- System has sufficient disk space for benchmark outputs
- Provider API endpoints are accessible

## Trigger
User starts a new benchmark session or triggers model refresh.

## Flow Diagram

```
┌─────────────┐    ┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│ Provider     │───>│ Model Enum   │───>│ Resource     │──>│ Register in   │
│ Scan         │    │ & Metadata  │    │ Pre-Check    │    │ Manifest     │
└─────────────┘    └──────────────┘    └──────────────┘    └──────────────┘
                           │                   │                    │
                           ▼                   ▼                    ▼
                    ┌──────────────┐    ┌──────────────┐    ┌──────────────┐
                    │ Deduplicate  │    │ Filter by     │    │ Write        │
                    │ Models       │    │ Requirements │    │ manifest.yml  │
                    └──────────────┘    └──────────────┘    └──────────────┘
```

## Step-by-Step

### Step 1: Enumerate Providers
- **Action**: Query each known provider endpoint for available models
- **Schema Properties Used**: `models.host.type` (lmstudio | ollama | llama_cpp_with_vulkan)
- **Input**: Provider connection settings from config
- **Output**: List of (provider, model_name) pairs
- **Error Handling**: If provider unreachable, log warning and skip. Continue with available providers.
- **New Schema Requirements**: `model_registry.providers` - array of provider configurations with connection settings

### Step 2: Extract Model Metadata
- **Action**: For each discovered model, fetch metadata (parameter count, quantization, context length, file size)
- **Schema Properties Used**: `models.<model_id>.name`, `models.<model_id>.host`
- **Input**: Model list from Step 1
- **Output**: Enriched model metadata (params, quant_level, context_length, file_size_mb, vendor_quant)
- **Error Handling**: If metadata fetch fails, use defaults and flag model as "unverified"
- **New Schema Requirements**: None - covered by existing `models` structure

### Step 3: Resource Pre-Check
- **Action**: Estimate VRAM/RAM required for each model and compare against available system resources
- **Schema Properties Used**: `models.<model_id>.max_allowed.vram`, `models.<model_id>.min_allowed.ram`
- **Input**: Model metadata, system resource info
- **Output**: Model feasibility assessment (can_load: true/false, estimated_vram_mb, estimated_ram_mb)
- **Error Handling**: If resource check fails, mark model as "requires_quantization_downgrade"
- **New Schema Requirements**: None - covered by existing resource limits

### Step 4: Deduplicate Models
- **Action**: Detect models available on multiple providers (e.g., same GGUF via Ollama and llama.cpp), prefer faster local provider
- **Schema Properties Used**: N/A
- **Input**: Full model list with metadata
- **Output**: Deduplicated list with provider preference ordering
- **Error Handling**: N/A
- **New Schema Requirements**: None

### Step 5: Write Benchmark Manifest
- **Action**: Generate `benchmark-manifest.yml` with all registered models, their metadata, resource estimates, and provider assignments
- **Schema Properties Used**: N/A (new file type)
- **Input**: Deduplicated, resource-validated model list
- **Output**: `benchmark-manifest.yml` containing:
  ```yaml
  benchmark_session_id: bench_2026_04_06_001
  total_models: 100
  models:
    - id: 1
      provider: ollama
      model_name: llama-3.2-3b-instruct-q4_k_m
      params: 3.2B
      quantization: q4_k_m
      context_length: 8192
      estimated_vram_mb: 2048
      estimated_ram_mb: 4096
      can_load: true
      provider_preference: 1  # priority order
  ```
- **Error Handling**: If write fails, retry with backup path
- **New Schema Requirements**: New `benchmark_manifest` root-level section

## Post-conditions
- `benchmark-manifest.yml` exists with all discoverable models
- Each model has metadata, resource estimate, and provider assignment
- Models that can't load are flagged with downgrade suggestions

## Edge Cases
- Provider running but no models installed (empty manifest)
- Same model available from multiple providers (deduplication)
- Provider returns inconsistent metadata (mark as unverified)
- Model file corrupted (skip with error log)
- 500+ models discovered (apply filtering criteria)

## Schema Coverage

| Feature | Covered by Schema | Needs Addition |
|---------|-------------------|----------------|
| Provider enumeration | Yes (`models.host.type`) | Provider API discovery protocol |
| Model metadata | Yes (`models.<id>.*`) | `model_file_size_mb`, `vendor_quantization_level` |
| Resource pre-check | Yes (`max_allowed`, `min_allowed`) | System resource introspection |
| Deduplication | No | Cross-provider model matching logic |
| Benchmark manifest | No | New root-level section |

## Dependencies
- None (this is the entry point userflow)
