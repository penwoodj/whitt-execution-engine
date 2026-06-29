# P15 Failure Analysis (2026-06-28)

## Prompt
P15: "Implement the agent ReAct layer for a Rust workflow engine"

## Result
**❌ COMPLETE FAILURE** — No deliverable produced. 16 of 22 steps skipped.

## Root Cause: SW4 Emitted Wrong Source File Paths

SW4 LLM generated shell hooks that `cat` files using bare names at repo root:
- `cat config.yaml` → **MISSING** (actual: `src/config/mod.rs`)
- `cat tools.rs` → **MISSING** (actual: `src/agent/tools.rs`)
- `cat react.rs` → **MISSING** (actual: `src/agent/react.rs`)

**16 steps affected:**
- `step_t1_analyze_yaml`, `step_t1_1_extract_workflow` → `config.yaml` missing
- `step_t4_generate_structs` through `step_t13_integrate_shared_state` → `tools.rs` missing
- `step_t14_generate_react_agent` through `step_t17_implement_agent_response` → `react.rs` missing

## Cascade Failure Pattern

1. Shell hook `cat config.yaml` fails (file not found at repo root)
2. `before_step_starts` hook returns error
3. Step is marked "Skipped by hook"
4. Downstream steps that `depends_on` the skipped step also skip
5. Synthesis step has no upstream output to synthesize
6. No deliverable produced

## Why SW4 Got the Paths Wrong

The SW4 prompt instructs the LLM to emit shell hooks that read source files for context injection. But the prompt does NOT include a map of actual file locations in the project. The LLM guessed based on common Rust project conventions:

| LLM Guessed | Actual Location |
|-------------|-----------------|
| `config.yaml` | `src/config/mod.rs` (Rust module, not YAML) |
| `tools.rs` | `src/agent/tools.rs` (in agent/ subdir) |
| `react.rs` | `src/agent/react.rs` (in agent/ subdir) |

## Fix: Source File Map in SW4 Prompt

Add a section to SW4 prompt that provides actual source file locations:

```
SOURCE FILE MAP (use these EXACT paths in shell hooks):
- Config loading: src/config/mod.rs, src/config/provider.rs, src/config/unified.rs
- Agent tools: src/agent/tools.rs
- Agent executor: src/agent/executor.rs
- ReAct agent: src/agent/react.rs
- Agent streaming: src/agent/streaming.rs
- Agent persistence: src/agent/persistence.rs
- Agent sandbox: src/agent/sandbox.rs
- Workflow step: src/workflow/step.rs
- Workflow hooks: src/workflow/hooks/actions.rs, src/workflow/hooks/context.rs
- Benchmark runner: src/benchmark/runner.rs
- Model schema: src/model/schema.rs
- LLM backend: src/backend/llm_backend.rs
- Error types: src/error.rs
- CLI: src/bin/whitt.rs
```

This prevents the LLM from guessing file locations and ensures shell hooks reference real files.

## Validation

After adding the source file map to SW4 prompt, the dry-run validator (`validate-workflow.py`) will verify all shell targets exist. The pipeline gate in `sw5-finalize.sh` will block execution if any file reference is broken.

## Impact

Fixing this single issue (SW4 source file map) would:
1. Make P15 pass (16 steps would no longer skip)
2. Prevent similar failures on any prompt that references source files
3. Bring total to 10/11 or potentially 11/11 PASS
