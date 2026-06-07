# 🧪 Testing Guide

This guide explains how to test the 50 example workflows included with the Whitt Execution Engine framework.

## Table of Contents

1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Test Setup](#test-setup)
4. [Running Workflow Tests](#running-workflow-tests)
5. [Validating Workflows](#validating-workflows)
6. [Testing Categories](#testing-categories)
7. [Common Issues](#common-issues)
8. [Advanced Testing](#advanced-testing)

---

## Overview

The framework includes **50 example workflows** organized into **18 categories**, demonstrating all features of the YAML workflow system:

| Category | Examples | Key Features |
|----------|-----------|--------------|
| Model Configuration | 4 | Provider selection, parameters, lifecycle, cost tracking |
| Step Types | 4 | LLM inference, code execution, tool invocation, hybrid |
| Data Flow | 3 | Variables, outputs, context injection |
| Parallel Execution | 3 | Parallel groups, concurrency, load balancing |
| Loops & Convergence | 4 | For/foreach/while loops, convergence |
| File Operations | 2 | Read/write, permissions, backup |
| Web Operations | 3 | Fetch/scrape, API integration, URL parameters |
| RAG Operations | 2 | Document indexing, context-aware generation |
| Script & CLI | 2 | Script execution, CLI commands |
| Sub-Workflows | 2 | Nested references, composition patterns |
| Conditional Branching | 2 | Event-based branching, decision logic |
| Error Handling & Retries | 3 | Retry strategies, propagation, recovery |
| Logging & Monitoring | 3 | Hierarchical logging, metrics, output formats |
| Checkpointing & State | 2 | Save/restore, state management |
| Resource Management | 3 | Memory allocation, CPU/GPU scheduling, throttling |
| Tool Permissions | 2 | Allow/deny lists, step control |
| User Inputs & UI | 2 | Input validation, interactive feedback |

**Location**: `../workflows/examples/requirements-oriented-auto/`

---

## Prerequisites

### Required

- **Rust 1.70+**: Framework implementation
- **LLM Provider**: LM Studio, Ollama, or OpenAI (for testing)
- **Disk Space**: 5 GB+ for models and workspace

### Recommended

- **GPU**: 4 GB+ VRAM for llama.cpp
- **RAM**: 16 GB+ for parallel workflows
- **CPU**: 4+ cores for concurrent execution

---

## Test Setup

### 1. Configure Environment

```bash
# Copy example environment file
cp .env.example .env

# Edit .env with your provider
nano .env

# Example configuration for LM Studio
LMSTUDIO_HOST=http://localhost:1234/v1
LMSTUDIO_MODEL=llama-3.2-3b-instruct

# Example configuration for Ollama
OLLAMA_HOST=http://localhost:11434
OLLAMA_MODEL=llama3.2
```

### 2. Start LLM Provider

**For LM Studio**:
1. Launch LM Studio
2. Go to Server tab
3. Click "Start Server"
4. Verify: `curl http://localhost:1234/v1/models`

**For Ollama**:
```bash
# Start Ollama server
ollama serve

# Verify connection
curl http://localhost:11434/api/tags
```

**For OpenAI**:
```bash
# Set API key in .env
OPENAI_API_KEY=sk-proj-your-key-here
```

### 3. Build Framework

```bash
# Build in debug mode for testing
cargo build

# Verify build
cargo test
```

---

## Running Workflow Tests

### Test Single Workflow

```bash
# Run specific workflow
cargo run -- run ../workflows/examples/requirements-oriented-auto/01-model-configuration/01-basic-model-selection-providers.yaml

# View output
cat ./workspace/output/step_1_output

# View logs
cat ./workspace/logs/workflow.log
```

### Test All Workflows in Category

```bash
# Test all model configuration workflows (4 workflows)
for file in ../workflows/examples/requirements-oriented-auto/01-model-configuration/*.yaml; do
    echo "Testing: $file"
    cargo run --run "$file"
done
```

### Test All Workflows

```bash
# Test all 50 workflows (when framework is fully implemented)
cargo run --test-all-workflows

# This will:
# 1. Validate all schemas
# 2. Execute all workflows
# 3. Generate report with results
```

---

## Validating Workflows

### Schema Validation

```bash
# Validate workflow schema only
cargo run --validate ../workflows/examples/requirements-oriented-auto/01-model-configuration/01-basic-model-selection-providers.yaml

# Expected output:
# ✓ Schema valid
# or
# ✗ Error: Missing required field: 'models'
```

### Dry Run

```bash
# Validate and plan execution without running
cargo run --dry-run ../workflows/examples/requirements-oriented-auto/01-model-configuration/01-basic-model-selection-providers.yaml

# Expected output:
# Execution plan:
# 1. Load model: llama-3.2-3b-instruct
# 2. Execute step: analyze_code
# 3. Save output: ./workspace/output/analysis
```

---

## Testing Categories

### Category 1: Model Configuration

**Workflows**: 4 examples

**Test Command**:
```bash
# Test 1: Basic model selection
cargo run --run ../workflows/examples/requirements-oriented-auto/01-model-configuration/01-basic-model-selection-providers.yaml

# Test 2: Model parameters tuning
cargo run --run ../workflows/examples/requirements-oriented-auto/01-model-configuration/02-model-parameters-tuning.yaml

# Test 3: Model lifecycle management
cargo run --run ../workflows/examples/requirements-oriented-auto/01-model-configuration/03-model-lifecycle-management.yaml

# Test 4: Cost tracking and budgets
cargo run --run ../workflows/examples/requirements-oriented-auto/01-model-configuration/04-cost-tracking-budgets.yaml
```

**Expected Output**:
- Model loaded and configured
- Parameters applied (temperature, top-p, max tokens)
- Lifecycle events logged
- Cost metrics collected

### Category 2: Step Types

**Workflows**: 4 examples

**Test Command**:
```bash
# Test LLM inference steps
cargo run --run ../workflows/examples/requirements-oriented-auto/02-step-types/01-llm-inference-steps.yaml

# Test code execution steps
cargo run --run ../workflows/examples/requirements-oriented-auto/02-step-types/02-code-execution-steps.yaml

# Test tool invocation steps
cargo run --run ../workflows/examples/requirements-oriented-auto/02-step-types/03-tool-invocation-steps.yaml

# Test hybrid step workflows
cargo run --run ../workflows/examples/requirements-oriented-auto/02-step-types/04-hybrid-step-workflows.yaml
```

**Expected Output**:
- LLM inference executes prompt
- Code execution runs and captures output
- Tool calls invoke external tools
- Hybrid steps combine multiple operations

### Category 4: Parallel Execution

### Category 04: Loops & Convergence

**Workflows**: 4 examples

**Test Command**:
```bash
# Test for loops
cargo run --run ../workflows/examples/requirements-oriented-auto/04-loops-convergence/01-for-loop-iteration.yaml

# Test foreach loops
cargo run --run ../workflows/examples/requirements-oriented-auto/04-loops-convergence/02-foreach-iteration.yaml
```

**Expected Output**:
- Loop iterations execute correctly
- Convergence criteria met
- Results aggregated properly

### Category 10: Conditional Branching

**Workflows**: 2 examples

**Test Command**:
```bash
# Test event-based branching
cargo run --run ../workflows/examples/requirements-oriented-auto/10-conditional-branching/01-event-based-branching.yaml

# Test decision logic workflows
cargo run --run ../workflows/examples/requirements-oriented-auto/10-conditional-branching/02-decision-logic-workflows.yaml
```

**Expected Output**:
- Branch conditions evaluated
- Correct path taken
- Decision results logged
- Multiple branches tested with different inputs

### Category 12: Error Handling & Retries

**Workflows**: 3 examples

**Test Command**:
```bash
# Test retry strategies and backoff
cargo run --run ../workflows/examples/requirements-oriented-auto/11-error-handling-retries/01-retry-strategies-backoff.yaml

# Test error propagation and escalation
cargo run --run ../workflows/examples/requirements-oriented-auto/11-error-handling-retries/02-error-propagation-escalation.yaml

# Test graceful failure and recovery
cargo run --run ../workflows/examples/requirements-oriented-auto/11-error-handling-retries/03-graceful-failure-recovery.yaml
```

**Expected Output**:
- Retries executed on failure
- Backoff strategies applied
- Errors propagated correctly
- Graceful recovery on critical failures

### Category 13: Logging & Monitoring

**Workflows**: 3 examples

**Test Command**:
```bash
# Test hierarchical logging system
cargo run --run ../workflows/examples/requirements-oriented-auto/12-logging-monitoring/01-hierarchical-logging-system.yaml

# Test metrics collection
cargo run --run ../workflows/examples/requirements-oriented-auto/12-logging-monitoring/02-metrics-collection.yaml

# Test structured output formats
cargo run --run ../workflows/examples/requirements-oriented-auto/12-logging-monitoring/03-structured-output-formats.yaml
```

**Expected Output**:
- Logs generated at all levels
- Metrics collected for all operations
- Output formats (JSON, chat, log) correct
- Hierarchical logging structure preserved

---

## Common Issues

### Issue: Connection Refused

**Error**: `Connection refused to localhost:1234`

**Solution**:
```bash
# Verify LLM provider is running
curl http://localhost:1234/v1/models  # LM Studio
curl http://localhost:11434/api/tags      # Ollama

# Start provider if not running
# LM Studio: Launch application and start server
# Ollama: ollama serve
```

### Issue: Model Not Found

**Error**: `Model file not found: llama-3.2-3b-instruct`

**Solution**:
```bash
# Download model
# LM Studio: Download through GUI
# Ollama: ollama pull llama3.2
# llama.cpp: Download GGUF from Hugging Face

# Verify model path in .env
cat .env | grep MODEL
```

### Issue: Out of Memory

**Error**: `Out of memory during execution`

**Solution**:
```bash
# Reduce model memory in .env
MODEL_MEMORY_MB=2048

# Reduce parallel executions
MAX_PARALLEL_EXECUTIONS=2

# Use serial execution mode
# In workflow YAML: execution: mode: serial
```

### Issue: Permission Denied

**Error**: `Permission denied: /workspace/output/file.txt`

**Solution**:
```bash
# Check file permissions in .env
# Disable tool confirmation
REQUIRE_TOOL_CONFIRMATION=false

# Add path to allowed paths
ALLOWED_FILE_PATHS=./workspace/output
```

---

## Advanced Testing

### Test with Validation Loops

```bash
# Run workflow with validation loop
cargo run --run ../workflows/examples/requirements-oriented-auto/04-loops-convergence/04-convergence-reduction-aggregation.yaml

# Check convergence in logs
grep "convergence" ./workspace/logs/workflow.log

# Verify validation score
jq '.validation_score' ./workspace/metrics/run_*.json
```

### Test with Sub-Workflows

```bash
# Test nested workflow execution
cargo run --run ../workflows/examples/requirements-oriented-auto/09-sub-workflows/02-workflow-composition-patterns.yaml

# Check sub-workflow output
ls -la ./workspace/output/sub_workflows/

# Verify interdependent validation
grep "interdependent" ./workspace/logs/workflow.log
```

### Test with Checkpointing

```bash
# Run workflow with checkpointing
cargo run --run ../workflows/examples/requirements-oriented-auto/13-checkpointing-state/01-checkpointing-save-restore.yaml

# Verify checkpoints created
ls -la ./workspace/checkpoints/

# Restore from checkpoint
# (Feature to be implemented)
```

---

## 📊 Test Report Generation

After testing all workflows, generate a comprehensive report:

```bash
# Generate test report (when implemented)
cargo run --test-report

# Report will include:
# - Total workflows tested: 50
# - Passed: X
# - Failed: Y
# - Performance metrics
# - Validation results
# - Error logs
```

---

## Best Practices for Testing

1. **Start Simple**: Begin with basic model configuration workflows
2. **Test Progressively**: Move from simple → complex workflows
3. **Verify Output**: Check output files and logs after each test
4. **Document Issues**: Note any errors or unexpected behavior
5. **Isolate Problems**: Test specific features individually
6. **Use Debug Mode**: Enable `DEBUG_MODE=true` for troubleshooting
7. **Monitor Resources**: Watch memory and CPU usage during tests

---

## 📊 Test Coverage by Category

| Category | Workflows | Status |
|-----------|-----------|---------|
| 01 - Model Configuration | 4 | ✅ Schema Complete |
| 02 - Step Types | 4 | ✅ Schema Complete |
| 03 - Data Flow | 3 | ✅ Schema Complete |
| 04 - Parallel Execution | 3 | ✅ Schema Complete |
| 05 - Loops & Convergence | 4 | ✅ Schema Complete |
| 06 - File Operations | 2 | ✅ Schema Complete |
| 07 - Web Operations | 3 | ✅ Schema Complete |
| 08 - RAG Operations | 2 | ✅ Schema Complete |
| 09 - Script & CLI | 2 | ✅ Schema Complete |
| 10 - Sub-Workflows | 2 | ✅ Schema Complete |
| 11 - Conditional Branching | 2 | ✅ Schema Complete |
| 12 - Error Handling | 3 | ✅ Schema Complete |
| 13 - Logging & Monitoring | 3 | ✅ Schema Complete |
| 14 - Checkpointing & State | 2 | ✅ Schema Complete |
| 15 - Resource Management | 3 | ✅ Schema Complete |
| 16 - Tool Permissions | 2 | ✅ Schema Complete |
| 17 - User Inputs & UI | 2 | ✅ Schema Complete |
| 18 - Hooks & Lifecycle | 4 | ✅ Schema Complete |
| **TOTAL** | **50** | **100% Schema Complete** |

---

## Next Steps

1. Start with **Category 01** (Model Configuration) workflows
2. Progress through categories in order
3. Test edge cases with complex workflows (Category 18)
4. Validate all workflows before framework implementation is complete
5. Generate test report when testing is finished

---

## 🔗 Related Documentation

| Document | Description |
|----------|-------------|
| [../README.md](../README.md) | Documentation index |
| [developer-guide.md](./developer-guide.md) | Development setup and workflow |
| [environment-variables.md](./environment-variables.md) | Configuration reference |
| [install.md](./install.md) | Installation guide |
| [contributing.md](./contributing.md) | Contribution guidelines |
| [../schema/unified-workflow-schema.yml](../schema/unified-workflow-schema.yml) | Schema reference |

---

**Happy Testing!** 🧪
