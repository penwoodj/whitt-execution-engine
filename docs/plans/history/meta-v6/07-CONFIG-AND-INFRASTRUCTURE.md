# META-v6 Config and Infrastructure

> **Purpose:** Define server config, KV cache strategy, model setup, and infrastructure requirements

## Infrastructure Overview

**Host:** Linux with AMD GPU
**Runtime:** Docker containerization
**Backend:** llama.cpp with Vulkan
**Model:** Qwen3-5-9B-Q4_K_M (6.53 GB)
**Context:** 262144 tokens
**KV Cache:** Q8_0 (~5.43 GB)
**Total RAM:** 11.96 GB (fits 15.5 GB available)

## Docker Configuration

### Container Setup

**Container Name:** `whitt-llama-server`
**Base Config:** `docker/docker-compose.yml` (NOT AMD or NVIDIA variant)
**Image:** llama.cpp with Vulkan backend

**Key Settings:**
```yaml
services:
  llama-server:
    image: ghcr.io/ggerganov/llama.cpp:server-vulkan
    container_name: whitt-llama-server
    ports:
      - "8080:8080"
    volumes:
      - /models:/models:ro
      - /home/jon/code/whitt-execution-engine/docker/entrypoint.sh:/entrypoint.sh:ro
    entrypoint: ['tini', '--', '/entrypoint.sh']
    environment:
      - LLAMA_CUDA=1  # Or Vulkan-specific env vars
```

**Important:**
- Mount entrypoint.sh at `/entrypoint.sh:ro` (NOT `/app/entrypoint.sh`)
- Entrypoint is `['tini', '--', '/entrypoint.sh']` (array format)
- NOT AMD or NVIDIA variant (use base docker-compose.yml)

### Entrypoint Script

**Location:** `docker/entrypoint.sh`
**Purpose:** Initialize llama.cpp server with correct parameters

**Key Parameters:**
```bash
#!/bin/bash
set -e

# Model path
MODEL_PATH="/models/Qwen3-5-9B-Q4_K_M.gguf"

# Context size
N_CTX=262144

# KV cache type (Q8_0 mandated)
CACHE_TYPE_K="q8_0"
CACHE_TYPE_V="q8_0"

# Parallel (single pipeline mandated)
PARALLEL=1

# Port
PORT=8080

# Start server
llama-server \
  --model "$MODEL_PATH" \
  --host 0.0.0.0 \
  --port "$PORT" \
  --ctx-size "$N_CTX" \
  --cache-type-k "$CACHE_TYPE_K" \
  --cache-type-v "$CACHE_TYPE_V" \
  --parallel "$PARALLEL" \
  --log-disable
```

### Container Management

**Start Container:**
```bash
docker compose -f docker/docker-compose.yml up -d
```

**Check Container Status:**
```bash
docker ps | grep whitt-llama-server
```

**View Container Logs:**
```bash
docker logs whitt-llama-server
```

**Stop Container:**
```bash
docker compose -f docker/docker-compose.yml down
```

**Restart Container:**
```bash
docker restart whitt-llama-server
```

## Model Configuration

### Model Details

**Name:** Qwen3-5-9B-Q4_K_M
**File:** `/models/Qwen3-5-9B-Q4_K_M.gguf`
**Size:** 6.53 GB
**Quantization:** Q4_K_M (4-bit quantization)
**Context:** 262144 tokens

### Model Loading

**Load Command (via engine):**
```bash
./target/release/whitt model load \
  --model Qwen3-5-9B \
  --path /models/Qwen3-5-9B-Q4_K_M.gguf \
  --backend llama_vulkan
```

**Load Timeout:** 1800 seconds (30 minutes)

**Unload Command:**
```bash
./target/release/whitt model unload --model Qwen3-5-9B
```

### Model Configuration in YAML

**Required Fields:**
```yaml
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080

models:
  - name: Qwen3-5-9B
    host:
      type: llama_cpp_with_vulkan
      config:
        model_path: /models/Qwen3-5-9B-Q4_K_M.gguf
    parameters:
      n_ctx: 262144
      cache_type_k: q8_0
      cache_type_v: q8_0
      parallel: 1
```

**Schema References:**
- Provider key: schema line 28
- Provider config wrapper: schema lines 29-31
- Host type: schema line 71

## Engine Configuration

### Config File

**Location:** `/home/jon/code/whitt-execution-engine/config.yml`
**Purpose:** Engine-level configuration

**Key Settings:**
```yaml
# KV cache type (Q8_0 mandated)
cache_type_k: "q8_0"
cache_type_v: "q8_0"

# Context size
n_ctx: 262144

# Parallel processing (single pipeline mandated)
parallel: 1

# Server endpoint
server:
  host: localhost
  port: 8080

# Timeout settings
timeout:
  load: 1800  # 30 minutes
  inference: 300  # 5 minutes
```

### Required Flags for Vulkan Backend

**Mandatory:**
- `--no-cache-prompt` - Vulkan cannot serialize KV cache state
- `--cont-batching` NOT USED - Triggers KV cache serialization on slot release

**Optional:**
- `--flash-attn on` - Safe and recommended

**Example Command:**
```bash
./target/release/whitt benchmark \
  --workflow docs/benchmarks/workflows/sw1-task-analysis.yml \
  --no-cache-prompt \
  --load-timeout 1800
```

## KV Cache Strategy

### Cache Type Selection

**Selected:** Q8_0 (8-bit quantization)
**Why:**
- Better precision than Q4_0 (less quantization error)
- Mandated by user specification
- Fits within 15.5 GB RAM limit

**Calculation:**
- Model: 6.53 GB
- KV cache (Q8_0 @ 262144): ~5.43 GB
- Total: 11.96 GB
- Available: 15.5 GB
- Headroom: 3.54 GB

### KV Cache Configuration

**In Config File:**
```yaml
cache_type_k: "q8_0"
cache_type_v: "q8_0"
```

**In Model Parameters:**
```yaml
parameters:
  cache_type_k: q8_0
  cache_type_v: q8_0
```

**In Entrypoint Script:**
```bash
CACHE_TYPE_K="q8_0"
CACHE_TYPE_V="q8_0"
```

### KV Cache Constraints

**Mandatory Flags:**
- `--no-cache-prompt` - Cannot serialize KV cache state

**Forbidden Flags:**
- `--cont-batching` - Triggers KV cache serialization on slot release

**Reason:** Vulkan backend does not support KV cache serialization

### KV Cache Monitoring

**Check KV Cache Usage:**
```bash
# In container logs
docker logs whitt-llama-server | grep -i "kv cache"

# Or via llama.cpp server API
curl http://localhost:8080/props
```

## Memory Management

### Total Memory Calculation

```
Model Size: 6.53 GB (Qwen3-5-9B-Q4_K_M)
KV Cache: 5.43 GB (Q8_0 @ 262144 tokens)
Overhead: 0.5 GB (llama.cpp runtime, buffers)
---
Total: 12.46 GB
Available: 15.5 GB
Headroom: 3.04 GB
```

### OOM Risk Mitigation

**Risk 1: Context exceeds 262144 tokens**
- Detection: Prompt truncation warning
- Mitigation: Chunk input, process in multiple passes
- Fallback: Reduce context to 131072 tokens

**Risk 2: KV cache too large**
- Detection: Server crash, port 8080 unresponsive
- Mitigation: Reduce KV cache to Q4_0 (~2.72 GB)
- Fallback: Reduce context to 131072 tokens

**Risk 3: Model + KV exceeds RAM**
- Detection: Server OOM, kernel kills process
- Mitigation: Reduce context, reduce KV cache quantization
- Fallback: Use smaller model (7B instead of 9B)

### Memory Monitoring

**Check Container Memory:**
```bash
docker stats whitt-llama-server
```

**Check Host Memory:**
```bash
free -h
```

**Check Process Memory:**
```bash
ps aux | grep llama-server
```

## Network Configuration

### Server Endpoint

**Protocol:** HTTP
**Host:** localhost
**Port:** 8080

**Full URL:** `http://localhost:8080`

### Port Conflicts

**Check if Port 8080 is in Use:**
```bash
lsof -i :8080
```

**If Port in Use:**
1. Stop conflicting service
2. Or change port in docker-compose.yml
3. Update config.yml port setting

### Network Connectivity

**Test Server Connectivity:**
```bash
curl http://localhost:8080/health
```

**Test Model Loading:**
```bash
curl http://localhost:8080/models/load \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{"model": "/models/Qwen3-5-9B-Q4_K_M.gguf"}'
```

## File System Layout

### Directory Structure

```
/home/jon/code/whitt-execution-engine/
├── config.yml                          # Engine config
├── target/release/whitt                # Engine binary
├── docs/
│   ├── benchmarks/
│   │   ├── workflows/                  # YAML workflows
│   │   │   ├── sw1-task-analysis.yml
│   │   │   ├── sw2-output-structure.yml
│   │   │   ├── sw3-category-mapping.yml
│   │   │   ├── sw4-struct-generation.yml
│   │   │   └── sw5-workflow-assembly.yml
│   │   └── outputs/
│   │       └── meta-workflow/          # Execution outputs
│   │           └── meta-<run_id>-sw<N>-<timestamp>/
│   │               ├── run.log
│   │               ├── output.md
│   │               └── workflow.yml
│   ├── plans/
│   │   └── meta-v6/                    # Plan files
│   │       ├── 00-MASTER-PLAN.md
│   │       ├── 01-OBJECTIVES-AND-SCOPE.md
│   │       ├── 02-ARCHITECTURE.md
│   │       ├── 03-SUBWORKFLOW-SPECIFICATIONS.md
│   │       ├── 04-ITERATION-PROTOCOL.md
│   │       ├── 05-QUALITY-BENCHMARK.md
│   │       ├── 06-HOOKS-STRATEGY.md
│   │       ├── 07-CONFIG-AND-INFRASTRUCTURE.md
│   │       ├── 08-TESTING-STRATEGY.md
│   │       └── 09-EXECUTION-CHECKLIST.md
│   └── schema/
│       └── unified-workflow-schema.yml # Schema source of truth
├── scripts/
│   └── meta-v6/                        # Run scripts
│       ├── run-sw1.sh
│       ├── run-sw2.sh
│       ├── run-sw3.sh
│       ├── run-sw4.sh
│       └── run-sw5.sh
├── docker/
│   ├── docker-compose.yml              # Docker config
│   └── entrypoint.sh                   # Entrypoint script
└── .current-meta-run                   # Tracks run ID
```

### Output Directory Pattern

**Pattern:** `docs/benchmarks/outputs/meta-workflow/meta-<run_id>-sw<N>-<timestamp>/`

**Components:**
- `<run_id>`: Tracked in `.current-meta-run`
- `<N>`: Sub-workflow number (1-5)
- `<timestamp>`: YYYYMMDD-HHMMSS

**Example:** `docs/benchmarks/outputs/meta-workflow/meta-001-sw1-20260614-143022/`

### Run ID Management

**File:** `.current-meta-run`
**Format:** Integer (auto-increment)
**Usage:**
1. Read current run ID from file
2. Increment by 1
3. Use for output directory naming
4. Write back to file

**Example:**
```bash
# Read current run ID
RUN_ID=$(cat .current-meta-run)

# Increment
RUN_ID=$((RUN_ID + 1))

# Use in output path
OUTPUT_DIR="docs/benchmarks/outputs/meta-workflow/meta-$RUN_ID-sw1-$(date +%Y%m%d-%H%M%S)"

# Write back
echo $RUN_ID > .current-meta-run
```

## Run Scripts Configuration

### Script Locations

**Base Directory:** `scripts/meta-v6/`

**Scripts:**
- `run-sw1.sh` - Task analysis
- `run-sw2.sh` - Output structure
- `run-sw3.sh` - Category mapping
- `run-sw4.sh` - Struct generation
- `run-sw5.sh` - Workflow assembly

### Common Script Structure

```bash
#!/bin/bash
set -e

# Read run ID
RUN_ID=$(cat .current-meta-run)
RUN_ID=$((RUN_ID + 1))

# Generate timestamp
TIMESTAMP=$(date +%Y%m%d-%H%M%S)

# Create output directory
OUTPUT_DIR="docs/benchmarks/outputs/meta-workflow/meta-$RUN_ID-sw<N>-$TIMESTAMP"
mkdir -p "$OUTPUT_DIR"

# Run workflow
./target/release/whitt benchmark \
  --workflow docs/benchmarks/workflows/sw<N>-*.yml \
  --output-dir "$OUTPUT_DIR" \
  --no-cache-prompt \
  --load-timeout 1800

# Write back run ID
echo $RUN_ID > .current-meta-run
```

### Script Permissions

**Make Scripts Executable:**
```bash
chmod +x scripts/meta-v6/run-sw*.sh
```

### Script Usage

**Run SW1:**
```bash
./scripts/meta-v6/run-sw1.sh --input tasks.md
```

**Run SW2-SW5:**
```bash
./scripts/meta-v6/run-sw2.sh
./scripts/meta-v6/run-sw3.sh
./scripts/meta-v6/run-sw4.sh
./scripts/meta-v6/run-sw5.sh
```

## Infrastructure Prerequisites

### Hardware Requirements

**Minimum:**
- RAM: 15.5 GB (model 6.53 GB + KV 5.43 GB + overhead)
- GPU: AMD GPU with Vulkan support
- Storage: 10 GB free (model + outputs + logs)
- CPU: 4 cores minimum

**Recommended:**
- RAM: 16 GB or more
- GPU: AMD GPU with 8 GB VRAM
- Storage: 20 GB free
- CPU: 8 cores or more

### Software Requirements

**Host:**
- Linux (Ubuntu 22.04 or similar)
- Docker (latest)
- Docker Compose (latest)
- Bash 4.0 or later
- curl (for testing)

**Container:**
- llama.cpp with Vulkan backend
- Qwen3-5-9B-Q4_K_M model file

### Network Requirements

**Port 8080:**
- Must be available (not in use by other services)
- Must be accessible from localhost

**Firewall:**
- Allow localhost:8080
- No external access required

### File Permissions

**Required:**
- Read access to `/models/Qwen3-5-9B-Q4_K_M.gguf`
- Write access to `docs/benchmarks/outputs/`
- Execute permission on `target/release/whitt`
- Execute permission on `scripts/meta-v6/run-sw*.sh`

**Check Permissions:**
```bash
ls -la /models/Qwen3-5-9B-Q4_K_M.gguf
ls -la target/release/whitt
ls -la scripts/meta-v6/run-sw*.sh
```

## Infrastructure Health Checks

### Pre-Execution Checks

**Check 1: Docker Running**
```bash
docker ps
```

**Check 2: Container Running**
```bash
docker ps | grep whitt-llama-server
```

**Check 3: Port Available**
```bash
lsof -i :8080
```

**Check 4: Server Responding**
```bash
curl http://localhost:8080/health
```

**Check 5: Model Loaded**
```bash
curl http://localhost:8080/props | grep Qwen
```

**Check 6: Engine Built**
```bash
ls -la target/release/whitt
```

**Check 7: Scripts Executable**
```bash
ls -la scripts/meta-v6/run-sw*.sh
```

**Check 8: Config Valid**
```bash
cat config.yml | grep cache_type
```

### During Execution Checks

**Check 1: Output Directory Created**
```bash
ls -la docs/benchmarks/outputs/meta-workflow/meta-*
```

**Check 2: Log File Present**
```bash
ls -la docs/benchmarks/outputs/meta-workflow/*/run.log
```

**Check 3: Schema Validation Passed**
```bash
grep "schema_valid=true" docs/benchmarks/outputs/meta-workflow/*/run.log
```

**Check 4: Workflow Completed**
```bash
grep "workflow:end" docs/benchmarks/outputs/meta-workflow/*/run.log
```

### Post-Execution Checks

**Check 1: Output Files Created**
```bash
ls -la docs/benchmarks/outputs/meta-workflow/*/output.md
ls -la docs/benchmarks/outputs/meta-workflow/*/workflow.yml
```

**Check 2: No Panics or Crashes**
```bash
grep -i "panic\|crash\|fatal" docs/benchmarks/outputs/meta-workflow/*/run.log
```

**Check 3: Memory Within Limits**
```bash
docker stats --no-stream | grep whitt-llama-server
```

## Troubleshooting

### Issue 1: Server Not Responding

**Symptom:** `curl http://localhost:8080/health` fails
**Diagnosis:**
```bash
docker ps | grep whitt-llama-server
docker logs whitt-llama-server
```
**Fix:**
```bash
docker restart whitt-llama-server
```

### Issue 2: Port 8080 Already in Use

**Symptom:** `lsof -i :8080` shows other process
**Diagnosis:**
```bash
lsof -i :8080
```
**Fix:**
- Stop conflicting service
- Or change port in docker-compose.yml

### Issue 3: OOM

**Symptom:** Server crash, kernel kills process
**Diagnosis:**
```bash
dmesg | grep -i "killed process"
docker logs whitt-llama-server | tail -20
```
**Fix:**
- Reduce KV cache to Q4_0
- Reduce context to 131072
- Close other processes

### Issue 4: Model Load Timeout

**Symptom:** `--load-timeout 1800` exceeded
**Diagnosis:**
```bash
docker logs whitt-llama-server | tail -20
```
**Fix:**
- Increase timeout to 3600 seconds
- Check model file integrity
- Check available memory

### Issue 5: Schema Validation Fails

**Symptom:** `schema_valid=false` in logs
**Diagnosis:**
```bash
grep "schema_valid=false" docs/benchmarks/outputs/meta-workflow/*/run.log
```
**Fix:**
- Check YAML structure
- Verify provider key is `llama_cpp_with_vulkan`
- Verify host type is `llama_cpp_with_vulkan`
- Remove non-schema keys

### Issue 6: Hooks Not Firing

**Symptom:** No hook logs in output
**Diagnosis:**
```bash
grep -i "hook" docs/benchmarks/outputs/meta-workflow/*/run.log
```
**Fix:**
- Check YAML structure
- Verify hook config is correct
- Check runner firing points

---

**Document Status:** Draft
**Last Updated:** 2026-06-14
**Author:** META-v6 Planning Session
**Review Status:** Pending