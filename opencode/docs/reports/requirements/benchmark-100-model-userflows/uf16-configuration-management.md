# UF16: Configuration Management

## Overview
Manages all benchmark configuration: workflow YAML, model overrides, retry policies, timeout values, output paths, and provider settings. Provides validation, defaults, and environment-specific overrides.

## User Story
As a benchmark operator, I want a single configuration file that controls all benchmark behavior so that I can tune the benchmark for my specific hardware, model set, and requirements without touching code.

## Pre-conditions
- Benchmark configuration file exists
- Configuration schema validated
- Required settings provided (at minimum: model list or provider endpoint)

## Trigger
Benchmark session starts, or configuration is loaded/modified.

## Flow Diagram

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ Load        │────>│ Validate    │────>│ Apply       │
│ Config File │     │ Against     │     │ Defaults    │
│ (YAML)      │     │ Schema      │     │ for Missing │
└──────────────┘     └──────┬───────┘     └──────┬───────┘
                            │                     │
                            ▼                     │
                     ┌──────────────┐              │
                     │ Apply       │              │
                     │ Environment │              │
                     │ Overrides   │              │
                     └──────┬───────┘              │
                            │                     │
                            ▼                     │
                     ┌──────────────┐              │
                     │ Merge       │◄─────────────┘
                     │ All Layers  │
                     └──────┬───────┘
                            │
                            ▼
                     ┌──────────────┐
                     │ Provide     │
                     │ Config to   │
                     │ Benchmark   │
                     │ Engine      │
                     └──────────────┘
```

## Configuration Layers (Priority Order)

| Layer | Source | Priority | Example |
|-------|--------|----------|---------|
| CLI args | Command line | Highest | `--max-models 50` |
| Environment vars | OS environment | High | `BENCH_VRAM_LIMIT=8192` |
| Config file | YAML config | Medium | `benchmark.yml` |
| Defaults | Built-in | Lowest | `timeout: 120s` |

## Step-by-Step

### Step 1: Load Configuration File
- **Action**: Read the benchmark configuration YAML file
- **Schema Properties Used**: All root-level schema properties
- **Input**: Config file path (default: `benchmark-config.yml`)
- **Output**: Parsed configuration object
- **Error Handling**: If file missing, use defaults and warn. If YAML invalid, error with parse details.
- **New Schema Requirements**: `benchmark_config` — root-level configuration section

### Step 2: Validate Against Schema
- **Action**: Validate configuration values against the unified workflow schema
- **Schema Properties Used**: All referenced properties
- **Input**: Parsed config
- **Output**: Validation result (valid/invalid + list of issues)
- **Error Handling**: On validation error, report all issues and abort (invalid config = undefined behavior)
- **New Schema Requirements**: None — schema validation is a core framework feature

### Step 3: Apply Defaults
- **Action**: Fill in missing optional values with sensible defaults
- **Schema Properties Used**: All optional properties
- **Input**: Validated config with possible gaps
- **Output**: Complete config with all values resolved
- **Error Handling**: N/A
- **New Schema Requirements**: None

### Step 4: Apply Environment Overrides
- **Action**: Override config values from environment variables (e.g., `BENCH_TIMEOUT=300` overrides `timeout: 120`)
- **Schema Properties Used**: N/A (environment mapping)
- **Input**: Config + environment variables
- **Output**: Config with env overrides applied
- **Error Handling**: Invalid env var values → log warning, use config file value instead
- **New Schema Requirements**: Environment variable mapping specification

### Step 5: Apply CLI Argument Overrides
- **Action**: Override config values from command-line arguments
- **Schema Properties Used**: N/A (CLI mapping)
- **Input**: Config + CLI args
- **Output**: Final merged config
- **Error Handling**: Invalid CLI args → error and abort
- **New Schema Requirements**: CLI argument specification

### Step 6: Freeze Configuration
- **Action**: Lock the merged configuration so it can't be modified during benchmark execution
- **Schema Properties Used**: N/A
- **Input**: Final merged config
- **Output**: Frozen config object
- **Error Handling**: N/A
- **New Schema Requirements**: None

### Step 7: Provide Config to Engine
- **Action**: Inject the frozen config into all benchmark subsystems
- **Schema Properties Used**: All applicable
- **Input**: Frozen config
- **Output**: All subsystems initialized with correct config
- **Error Handling**: If subsystem rejects config, abort benchmark
- **New Schema Requirements**: None

## Post-conditions
- Single source of truth for all benchmark settings
- Configuration validated and defaults applied
- Environment and CLI overrides respected
- Configuration frozen for the duration of the benchmark run

## Edge Cases
- Conflicting settings across layers (highest priority wins, log warning)
- Config file references non-existent model (validation catches this)
- Config file is empty (all defaults used, warn user)
- Config file is very large (>1000 lines) (parse may be slow, log timing)
- Two benchmark sessions use different configs (each session has its own frozen config)

## Schema Coverage

| Feature | Covered by Schema | Needs Addition |
|---------|-------------------|----------------|
| YAML config parsing | Yes (core schema) | None |
| Schema validation | Yes (core feature) | None |
| Default values | Partially | Explicit defaults documentation |
| Environment overrides | No | Env var mapping spec |
| CLI argument parsing | No | CLI arg spec |
| Config layering | No | Priority resolution spec |
| Config freeze | No | Immutability mechanism |

## Dependencies
- All userflows — configuration is the foundation everything else depends on
- Unified workflow schema — defines the valid configuration structure
