# Gap Analysis: Multi-Model Benchmarking System

## Current State Summary

### What We Have (Working)
1. **Benchmark command** (`whitt benchmark`): Single-model benchmark with --max-tokens, --prompt, --no-stream flags. Reports tps and token count.
2. **Model lifecycle API**: `client.load_model()`, `client.unload_model()`, `client.list_models()` — HTTP calls to llama.cpp server
3. **Sequential model chain** (`model_chain.rs`): Load→Infer→Unload pattern with timing tracking. Container recreation for router discovery.
4. **Docker management**: Start/stop/restart server. `MODEL_HOST_PATH` env var for external drive mount.
5. **Workflow YAML parsing**: UnifiedConfig parses schema-aligned YAML with providers, models, steps
6. **Agent execution**: ReActAgent with tool filtering, max_iterations loop, tool sandbox
7. **Schema-defined loops**: `loop.validation.max_iterations`, `loop.count.max_iterations`, `iteration_variable` in unified-workflow-schema.yml (lines 411-447)
8. **Checkpoint/persistence**: WorkflowState, Checkpoint structs for resumable workflows

### What's Missing (Must Implement)

#### CRITICAL — Multi-Model Benchmarking
| Gap | Description | Effort |
|-----|-------------|--------|
| **BENCH-01** | No multi-model benchmark command. Current `benchmark` only tests loaded model. Need `benchmark --models-dir PATH` or `benchmark --model-list FILE` | Medium |
| **BENCH-02** | No sequential model swap in benchmark. Need: unload current → load next → benchmark → repeat | Medium |
| **BENCH-03** | No GPU vs CPU comparison mode. Need dual-run per model: one GPU-offloaded, one CPU-only | Medium |
| **BENCH-04** | No benchmark result aggregation/reporting. Need JSON output with per-model metrics, comparison table | Medium |
| **BENCH-05** | `--concurrent` flag in benchmark is dead code. Either implement or remove | Small |
| **BENCH-06** | No disk space monitoring. Risk of filling main drive during 50-model run | Small |

#### CRITICAL — Workflow Loop Execution
| Gap | Description | Effort |
|-----|-------------|--------|
| **LOOP-01** | Schema defines loops (lines 411-447) but NO execution code. WorkflowExecutor doesn't handle `loop` key in steps | Large |
| **LOOP-02** | No `loop.validation` support. Need: run step → check criteria → repeat if fail (max_iterations) | Large |
| **LOOP-03** | No `loop.count` support. Need: iterate N times with `iteration_variable` | Medium |
| **LOOP-04** | No loop hooks (after_loop_iteration_fails, etc.) | Medium |

#### HIGH — Agentic Summarization-Expansion Workflow
| Gap | Description | Effort |
|-----|-------------|--------|
| **AGENT-01** | No "summarize-expansion" step type. Need step that: reads file chunk → summarizes → expands → repeats 3x | Large |
| **AGENT-02** | No chunked file reading in workflow. Agent tools don't support reading arbitrary files from disk as input | Medium |
| **AGENT-03** | No output capture between loop iterations. Need ability to feed iteration N output into iteration N+1 prompt | Medium |
| **AGENT-04** | No plan file as input to workflow. Need workflow to accept file path as parameter and read it | Small |

#### MEDIUM — Infrastructure
| Gap | Description | Effort |
|-----|-------------|--------|
| **INFRA-01** | No symlinking strategy for models. Models must be accessible to Docker without copying to main drive | Small |
| **INFRA-02** | No benchmark YAML file generation. Need tool to generate valid YAML from model list | Medium |
| **INFRA-03** | No cleanup after model unload. Temp files on main drive not removed | Small |
| **INFRA-04** | No benchmark runner script/command that orchestrates the full workflow | Medium |

### Implementation Order (Dependency Graph)

```
Phase A: Infrastructure (unblocks everything)
  INFRA-01 → Symlink model access from external drive
  INFRA-03 → Cleanup after model swap
  BENCH-06 → Disk space monitoring

Phase B: Core Benchmarking Loop
  BENCH-01 → Multi-model benchmark command
  BENCH-02 → Sequential model swap
  BENCH-05 → Remove dead --concurrent code
  BENCH-04 → Result aggregation

Phase C: GPU/CPU Comparison
  BENCH-03 → Dual-run mode (GPU vs CPU)

Phase D: Workflow Loop Engine
  LOOP-03 → Count-based loops (simpler, needed first)
  LOOP-01 → Validation-based loops
  LOOP-04 → Loop hooks

Phase E: Agentic Workflow
  AGENT-04 → File path as workflow input
  AGENT-02 → Chunked file reading tool
  AGENT-03 → Output capture between iterations
  AGENT-01 → Summarize-expansion step

Phase F: Benchmark YAML Suite
  INFRA-02 → YAML generation from model list
  INFRA-04 → Benchmark runner command
```

### Environment Constraints
- **Main drive**: 159GB free (btrfs, encrypted). Must not fill.
- **Data drive**: 3.1TB free on 5.5TB ext4 at `/run/media/jon/data/models/`
- **GPU**: AMD 8GB VRAM (Vulkan). Constraints: `--no-cache-prompt`, no `--cont-batching`, KV cache f16 only
- **CPU**: 4 threads available
- **Docker**: Must mount external drive via `MODEL_HOST_PATH` env var
- **Models**: 209 GGUF files ≤6GB fit in VRAM. 50 models selected for benchmark.

### Model Selection Strategy (50 models)
Select diverse set covering:
- **Size tiers**: 0.5B, 1.5B, 3B, 4B, 7B (at Q4_K_M quantization)
- **Architectures**: Qwen, Llama, Mistral, Gemma, Phi, Falcon, Granite, IBM, etc.
- **Specializations**: Code, reasoning, chat, instruct, thinking
- **Priority**: Models that already exist on data drive (no download needed)
