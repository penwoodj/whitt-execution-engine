# Meta-Workflow Generator Guide

Generate executable YAML workflows from natural language prompts using the Whitt Execution Engine.

## Prerequisites

1. Docker llama.cpp server running with models loaded:
   ```bash
   docker compose -f docker/docker-compose.yml up -d
   curl http://localhost:8080/health  # verify server is up
   ```

2. Whitt CLI built with client features:
   ```bash
   cargo build --release --features client
   ```

## Quick Start

### 1. Generate a Workflow from a Prompt

```bash
# Set your task prompt
TASK="Read the CSV file at /tmp/sales-data.csv and produce a summary report with row counts, column statistics, and anomaly detection"

# Create output directory
RUN_ID="my-run-$(date +%Y%m%d-%H%M%S)"
mkdir -p "real-workflow-attempts/outputs/${RUN_ID}/logs"

# Run the meta-workflow generator
./target/release/whitt benchmark \
  --workflow real-workflow-attempts/meta-workflow-template.yml \
  --min-tmp-space 1 \
  2>&1 | tee "real-workflow-attempts/outputs/${RUN_ID}/run.log"
```

Before running, edit `meta-workflow-template.yml` and replace these placeholders:
- `__TASK_PLACEHOLDER__` → your task description
- `__RUN_ID__` → your run directory name (e.g., `my-run-20260603-120000`)

### 2. Post-Process and Validate

```bash
# Post-process: strip fences, fix step nesting, inject hooks
python3 real-workflow-attempts/scripts/post-process-workflow.py \
  "real-workflow-attempts/outputs/${RUN_ID}/final-workflow.yml"

# Validate the result
python3 real-workflow-attempts/scripts/validate-yaml.py \
  "real-workflow-attempts/outputs/${RUN_ID}/final-workflow.yml"

# Check logs
cat "real-workflow-attempts/outputs/${RUN_ID}/logs/meta-workflow.log"
```

### 3. Run the Generated Workflow

```bash
# Copy to a clean location and customize paths
cp "real-workflow-attempts/outputs/${RUN_ID}/final-workflow.yml" /tmp/my-workflow.yml

# Edit file paths in the YAML to match your actual data
# Then run it
./target/release/whitt benchmark \
  --workflow /tmp/my-workflow.yml \
  --min-tmp-space 1 \
  2>&1 | tee /tmp/my-workflow-run.log

# Check outputs
ls outputs/output/
```

## Template Variables

The meta-workflow template uses these placeholders that must be replaced before running:

| Variable | Where | Replace With |
|----------|-------|-------------|
| `__TASK_PLACEHOLDER__` | step_1_generate prompt | Your natural language task description |
| `__RUN_ID__` | All `save_to` and `log` paths | A unique directory name like `run-20260603-120000` |

## Models Used

The meta-workflow uses two models by default:

| Role | Model | Purpose |
|------|-------|---------|
| Generator | `Qwen3-4B-Instruct-2507-Q4_K_M` | Generates, cleans, and validates YAML |
| Coder | `Qwen2.5-Coder-3B-Instruct-Q8_0` | Used in generated workflows for execution |

To change models, edit the `models:` section of the template. Available models can be listed with:
```bash
curl -s http://localhost:8080/v1/models | jq '.data[].id' -r
```

## Generated Workflow Structure

The meta-workflow produces YAML files with this structure:

```yaml
workflow_id: <unique-id>
name: "<descriptive name>"
schema_version: "2.0.0"

providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080

models:
  "target-model":
    name: "<model-filename>"
    host:
      type: llama_cpp_with_vulkan

agentic_workflow:
  steps:
    step_01_name:          # ← ZERO-PAD step names! (step_01, step_02, ...)
      generative_entity: "${models.target-model}"
      prompt: |
        <detailed prompt with {{step.X.output}} references>
      when:
        before_step_starts:
          shell:           # Read files via shell hooks
            command: "cat"
            args: ["<filepath>"]
        after_step_succeeds:
          - save_to: "./outputs/stepN-output.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
```

## Critical Rules for Generated Workflows

1. **Zero-pad step names**: Use `step_01`, `step_02`, not `step_1`, `step_2`. YAML mapping keys are sorted alphabetically, so `step_10` sorts before `step_2`.

2. **Use shell hooks for file reading**: Don't ask the LLM to "read a file" in the prompt. Use `before_step_starts` shell hooks with `cat` and reference `{{bookmarks.shell_output.stdout}}` in the prompt.

3. **Template interpolation**: Use `{{step.step_name.output}}` to pass output from one step to the next. The `step.` prefix is mandatory.

4. **Keep prompts under 4K tokens**: The 3B model has ~4K token context. Keep prompts concise and specific.

5. **Small chunks, many steps**: Instead of one complex step, break into multiple small steps with validate-and-fix cycles.

## Troubleshooting

### Server Connection Errors
```
500 Internal Server Error – proxy error: Could not establish connection
```
The model may not be loaded on the server. List available models:
```bash
curl -s http://localhost:8080/v1/models | jq '.data[].id' -r
```

### Steps Execute Out of Order
Zero-pad step names. `step_1` through `step_9` work, but `step_10` sorts before `step_2`. Always use `step_01` through `step_99`.

### Empty Output or "NO IDEAS FOUND"
The 3B model may not be capable enough for nuanced extraction. Try:
- Using a larger model (7B if available)
- Making the prompt more specific with examples
- Reducing the input data size (use `head -c 8000` instead of `head -c 12000`)

### /tmp Space Full
```bash
df -h /tmp
# If 99%+ full, clean up:
rm -rf /tmp/tech-notes-extract /tmp/idea-board-* /tmp/run*.log
```
Use `--min-tmp-space 1` flag to bypass the minimum space check.

## File Locations

| File | Purpose |
|------|---------|
| `real-workflow-attempts/meta-workflow-template.yml` | The meta-workflow generator template |
| `real-workflow-attempts/scripts/validate-yaml.py` | YAML validation script |
| `real-workflow-attempts/outputs/` | Generated workflow outputs |
| `outputs/output/` | Output from running generated workflows |
| `docs/schema/unified-workflow-schema.yml` | Schema reference for valid YAML |

## Current Status (as of 2026-06-03)

### v3 Meta-Workflow (RECOMMENDED)
- ✅ 5-step decomposed generator (decompose→plan→prompts→assemble→validate)
- ✅ Generates structurally valid YAML from arbitrary prompts
- ✅ Post-processor fixes structure (fence stripping, step nesting, hook injection)
- ✅ Post-processor injects `{{bookmarks.shell_output.stdout}}` into prompts with shell hooks
- ✅ Post-processor preserves multi-line strings as YAML block scalars
- ✅ Generated workflows execute end-to-end (6/6 steps pass)
- ✅ Output files saved for each step via save_to hooks
- ✅ Template interpolation chains steps together
- ✅ Bookmark system passes data between steps
- ✅ 4 live tests completed (v3-test1 through v3-test4)

### Known Limitations
- ⚠️ 3B model explains HOW instead of DOING — inherent to model size, not engine
- ⚠️ Shell hook commands with special chars may fail YAML parsing (post-processor handles most)
- ⚠️ Generated workflows may hallucinate data instead of reading real files
- ⚠️ Steps sometimes appear at top level instead of under `agentic_workflow: steps:` (post-processor fixes)
- ❌ Sub-workflow support not yet implemented
- ❌ No iterative verify/fix cycles within generation

### Test Results Summary
| Test | Prompt | Steps | Meta | Generated | Quality |
|------|--------|-------|------|-----------|---------|
| v3-test1 | Markdown TOC | 6 | 5/5 | 6/6 | Low (explains vs does) |
| v3-test2 | CSV line count | 8 | 5/5 | 8/8 | Medium (JSON valid, some hallucination) |
| v3-test3 | List files with sizes | 6 | 5/5 | 6/6 | Medium (shell cmds preserved, table generated) |
| v3-test4 | Count .txt files | 6 | 5/5 | 6/6 | Medium (bookmarks injected, some data used) |
