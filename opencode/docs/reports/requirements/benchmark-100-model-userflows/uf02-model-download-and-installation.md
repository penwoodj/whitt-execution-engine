# UF02: Model Download and Installation

## Overview
Downloads, verifies, and installs LLM models from HuggingFace, Ollama registry, or local files, then registers them with the appropriate provider.

## User Story
As a benchmark operator, I want to specify model names in my benchmark manifest and have the system automatically download and configure them so that I can benchmark models not yet installed locally.

## Pre-conditions
- Internet access (for remote model downloads) or local GGUF file path
- Sufficient disk space for model files
- Target provider is running and accepting model loads

## Trigger
Model in benchmark manifest is flagged `can_load: false` or `installed: false`.

## Flow Diagram

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ Check       │────>│ Resolve      │────>│ Download     │────>│ Verify      │
│ Manifest    │     │ Source       │     │ Model File   │     │ Integrity   │
│ for Missing │     │ (HF/Ollama/  │     │              │     │ (SHA256)    │
└──────────────┘     │ Local)       │     └──────────────┘     └──────┬───────┘
                           │              │                                │
                           ▼              ▼                                ▼
                    ┌──────────────┐ ┌──────────────┐            ┌───────────┐
                    │ Select       │ │ Configure    │            │ Install   │
                    │ Quantization│ │ Provider     │            │ & Register│
                    └──────────────┘ └──────────────┘            └───────────┘
```

## Step-by-Step

### Step 1: Identify Missing Models
- **Action**: Compare benchmark manifest against provider inventories to find models needing download
- **Schema Properties Used**: `benchmark_manifest.models[].installed`
- **Input**: Benchmark manifest from UF01
- **Output**: List of models needing installation with preferred quantization
- **Error Handling**: If manifest is missing, error and require UF01 to run first
- **New Schema Requirements**: `benchmark_manifest.models[].installed: bool`

### Step 2: Resolve Model Source
- **Action**: Determine where to get the model from (HuggingFace repo, Ollama registry, local GGUF path)
- **Schema Properties Used**: N/A (resolution logic)
- **Input**: Model name, preferred provider
- **Output**: Download URL or local file path
- **Error Handling**: If no source found, mark model as "unavailable" in manifest
- **New Schema Requirements**: `models.<id>.source.url`, `models.<id>.source.registry`

### Step 3: Download Model File
- **Action**: Download GGUF/weights file to local model storage directory
- **Schema Properties Used**: N/A
- **Input**: Download URL, target directory, expected file size
- **Output**: Local model file path
- **Error Handling**: On download failure: retry with backoff (3 attempts). On partial download: resume. On network timeout: try alternative mirror.
- **New Schema Requirements**: `workspace.model_storage_path`

### Step 4: Verify Integrity
- **Action**: Check SHA256 hash of downloaded file against expected hash
- **Schema Properties Used**: N/A
- **Input**: Downloaded file path, expected hash
- **Output**: Verification result (pass/fail)
- **Error Handling**: On hash mismatch: delete file, retry download. On missing hash: warn and proceed with verification.
- **New Schema Requirements**: None

### Step 5: Install to Provider
- **Action**: Copy/link model file to provider's model directory and trigger provider reload
- **Schema Properties Used**: `models.host.type` (determines install location)
- **Input**: Verified model file path, target provider
- **Output**: Model registered with provider, ready for loading
- **Error Handling**: On install failure: retry, then try alternative provider
- **New Schema Requirements**: None - provider-specific

### Step 6: Update Manifest
- **Action**: Mark model as `installed: true` in benchmark manifest with installation metadata
- **Schema Properties Used**: `benchmark_manifest.models[]`
- **Input**: Installation result, file path, provider used
- **Output**: Updated manifest
- **Error Handling**: Write to temp file first, then atomic rename
- **New Schema Requirements**: `benchmark_manifest.models[].install_path`, `.install_date`

## Post-conditions
- All models in manifest are installed (or flagged as unavailable)
- Each installed model has provenance metadata (source, hash, install date)
- Provider can immediately load installed models

## Edge Cases
- Model name exists on HuggingFace but not in expected format
- Multiple quantization options available (select based on resource budget)
- Disk full during download (pause, prompt user)
- Provider requires specific file naming convention
- Large model (70B+) takes significant time to download

## Schema Coverage

| Feature | Covered by Schema | Needs Addition |
|---------|-------------------|----------------|
| Model source resolution | No | `models.<id>.source` section |
| Download with resume | No | `tool_operations.download` integration |
| Integrity verification | No | SHA256 hash in model metadata |
| Provider install paths | Partially | Provider-specific path mapping |
| Manifest update | No | `benchmark_manifest` section |

## Dependencies
- UF01 (Model Discovery) - needs manifest to know what to install
- UF03 (Adaptive Quantization) - may need to quantize after install if memory insufficient
