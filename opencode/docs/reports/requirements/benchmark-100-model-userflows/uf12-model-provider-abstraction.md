# UF12: Model Provider Abstraction

## Overview
Provides a unified interface for interacting with different LLM providers (LM Studio, Ollama, llama.cpp/Vulkan) so that benchmark logic is provider-agnostic and can run against any supported backend.

## User Story
As a benchmark operator, I want the system to abstract provider differences so that I can benchmark models regardless of which provider serves them, without writing provider-specific code for each.

## Pre-conditions
- At least one provider is running
- Provider API endpoints configured
- Model-to-provider mapping established (UF01)

## Trigger
Any model operation (load, infer, unload) needs to be executed.

## Flow Diagram

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ Benchmark    │────>│ Provider    │────>│ Provider    │
│ Request      │     │ Abstraction │     │ Adapter     │
│ (load/       │     │ Layer       │     │ (LM Studio/ │
│  infer/      │     │             │     │  Ollama/    │
│  unload)     │     │             │     │  llama.cpp) │
└──────────────┘     └──────────────┘     └──────┬───────┘
                                                  │
                           ┌──────────────────────┤
                           │                      │
                           ▼                      ▼
                    ┌──────────────┐       ┌──────────────┐
                    │ Normalize   │       │ Provider-   │
                    │ Response    │       │ Specific    │
                    │             │       │ API Call    │
                    └──────┬───────┘       └──────────────┘
                           │
                           ▼
                    ┌──────────────┐
                    │ Return      │
                    │ Unified     │
                    │ Result      │
                    └──────────────┘
```

## Provider API Comparison

| Feature | LM Studio | Ollama | llama.cpp |
|---------|-----------|--------|-----------|
| Load Model | POST /v1/models/load | POST /api/load | CLI argument |
| Infer | POST /v1/chat/completions | POST /api/chat | stdin/stdout |
| Unload | POST /v1/models/unload | POST /api/generate (unload) | Process kill |
| List Models | GET /v1/models | GET /api/tags | N/A |
| Health Check | GET /v1/models | GET /api/version | Process ping |
| KV Cache Quant | Model params | Model params | CLI flags |
| Stream Response | SSE | SSE | Streaming output |

## Step-by-Step

### Step 1: Detect Available Providers
- **Action**: Probe known endpoints to discover which providers are running
- **Schema Properties Used**: `models.host.type` (lmstudio | ollama | llama_cpp_with_vulkan)
- **Input**: Provider configuration from benchmark config
- **Output**: List of active providers with versions
- **Error Handling**: Provider not responding → mark as unavailable, try alternatives
- **New Schema Requirements**: `models.host.endpoint: string` — configurable API endpoint URL

### Step 2: Initialize Provider Adapters
- **Action**: Create adapter instances for each active provider with connection pooling
- **Schema Properties Used**: `models.host.type`, `models.host.endpoint`
- **Input**: Active provider list
- **Output**: Initialized adapter pool
- **Error Handling**: Adapter init failure → remove provider from pool
- **New Schema Requirements**: None

### Step 3: Map Model to Provider
- **Action**: For each model, determine which provider can serve it (from manifest or auto-detection)
- **Schema Properties Used**: `benchmark_manifest.models[].provider`
- **Input**: Model ID, provider pool
- **Output**: Model-to-provider mapping
- **Error Handling**: No provider available → skip model
- **New Schema Requirements**: None

### Step 4: Unified Load
- **Action**: Translate a generic "load model" request into provider-specific API call
- **Schema Properties Used**: `models.<id>.host.type`, `models.<id>.model_memory.kv_cache_quantization`
- **Input**: Model ID, quantization config
- **Output**: Load result (success/failure, timing)
- **Error Handling**: Provider-specific errors normalized to unified error types
- **New Schema Requirements**: None

### Step 5: Unified Infer
- **Action**: Translate a generic "send prompt" request into provider-specific API call
- **Schema Properties Used**: `model_overrides.timeout.time_to_first_response`, `model_overrides.timeout.completion`
- **Input**: Prompt, model ID, generation parameters
- **Output**: Unified response (text, tokens, timing, tool_calls)
- **Error Handling**: Provider timeout → normalized timeout error; parse errors → unified parse error
- **New Schema Requirements**: None

### Step 6: Unified Unload
- **Action**: Translate a generic "unload model" request into provider-specific API call
- **Schema Properties Used**: `workflow_execution_strategy.load_unload`
- **Input**: Model ID
- **Output**: Unload result (success/failure)
- **Error Handling**: Force unload if graceful fails
- **New Schema Requirements**: None

### Step 7: Provider Health Monitoring
- **Action**: Periodically check provider health and update adapter pool status
- **Schema Properties Used**: N/A
- **Input**: Provider endpoints
- **Output**: Provider health status (healthy/degraded/down)
- **Error Handling**: Provider down → route to alternative provider or queue model
- **New Schema Requirements**: `benchmark_execution.provider_health_check_interval_seconds: int`

## Post-conditions
- Benchmark logic interacts with a single unified API regardless of provider
- Provider failures are normalized and handled consistently
- Provider health is monitored throughout benchmark
- Model-to-provider mapping is maintained

## Edge Cases
- Provider updates mid-benchmark (API changes) — adapter may break, detect via health check
- Multiple providers serve the same model — prefer lowest-latency provider
- Provider returns non-standard response format — adapter normalizes
- Provider rate limits — back off and retry
- llama.cpp subprocess crashes — adapter detects and restarts

## Schema Coverage

| Feature | Covered by Schema | Needs Addition |
|---------|-------------------|----------------|
| Provider type enum | Yes (`models.host.type`) | None |
| Provider endpoint config | Partially | `models.host.endpoint` |
| Provider-specific params | Yes (`models.<id>.*`) | None |
| Provider health monitoring | No | Health check schema |
| Provider failover | No | Automatic failover config |
| Response normalization | No | Adapter interface spec |

## Dependencies
- UF01 (Discovery) — provider detection
- UF02 (Installation) — provider-specific install
- UF04 (Loading) — uses provider abstraction for loads
- UF06 (Orchestration) — uses provider abstraction for inference
- UF11 (Concurrent Execution) — coordinates multiple providers
