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

3. Python 3 with PyYAML (for post-processor):
   ```bash
   pip install pyyaml
   ```

## Quick Start

### 1. Generate a Workflow from a Prompt

```bash
TASK="Read the CSV file at /tmp/sales-data.csv and produce a summary report"
RUN_ID="my-run-$(date +%Y%m%d-%H%M%S)"
mkdir -p "real-workflow-attempts/outputs/${RUN_ID}/logs"

# Inject task into v4 template
sed "s|__TASK_PLACEHOLDER__|${TASK}|g; s|__RUN_ID__|${RUN_ID}|g" \
  real-workflow-attempts/meta-workflow-v4.yml > /tmp/meta-run.yml

# Run the meta-workflow generator (~3 min)
./target/release/whitt benchmark \
  --workflow /tmp/meta-run.yml \
  --min-tmp-space 1 \
  2>&1 | tee "real-workflow-attempts/outputs/${RUN_ID}/meta-run.log"
```

### 2. Post-Process and Validate

```bash
# Post-process: strip fences, fix step nesting, inject hooks and prompts
python3 real-workflow-attempts/scripts/post-process-workflow.py \
  "real-workflow-attempts/outputs/${RUN_ID}/final-workflow.yml" \
  "real-workflow-attempts/outputs/${RUN_ID}/03-prompts.txt"

# Validate the result
python3 real-workflow-attempts/scripts/validate-yaml.py \
  "real-workflow-attempts/outputs/${RUN_ID}/final-workflow.yml"
```

### 3. Run the Generated Workflow

```bash
cp "real-workflow-attempts/outputs/${RUN_ID}/final-workflow.yml" /tmp/my-workflow.yml

./target/release/whitt benchmark \
  --workflow /tmp/my-workflow.yml \
  --min-tmp-space 1 \
  2>&1 | tee "real-workflow-attempts/outputs/${RUN_ID}/gen-run.log"

# Check outputs
ls outputs/output/
```

## Template Variables

The v3 meta-workflow template uses these placeholders:

| Variable | Where | Replace With |
|----------|-------|-------------|
| `__TASK_PLACEHOLDER__` | step_01_decompose prompt | Your natural language task description |
| `__RUN_ID__` | All `save_to` and `log` paths | A unique directory name like `my-run-20260603-120000` |

## v3 Pipeline Architecture

The v3 meta-workflow decomposes workflow generation into 5 specialized steps:

| Step | Model | Output | Purpose |
|------|-------|--------|---------|
| step_01_decompose | 4B planner | `01-actions.txt` | Extract action items from task prompt |
| step_02_plan_steps | 4B planner | `02-plan.txt` | Turn actions into ordered step plan |
| step_03_write_prompts | 4B planner | `03-prompts.txt` | Write detailed prompts for each step |
| step_04_assemble | 3B coder | `04-assembled.yml` | Generate YAML skeleton with hooks |
| step_05_validate | 3B coder | `final-workflow.yml` | Fix all issues, validate structure |

### Post-Processor Transforms

After generation, `post-process-workflow.py` applies these fixes (because 3B models can't reliably produce them):

1. Strip markdown fences (`\`\`\`yaml ... \`\`\``)
2. Move top-level steps under `agentic_workflow: steps:`
3. Inject actual prompts from `03-prompts.txt`
4. Remove placeholder shell hooks (`command: "none"`)
5. Inject `{{bookmarks.shell_output.stdout}}` into prompts with shell hooks
6. Inject missing `after_step_succeeds` / `after_step_fails` hooks
7. Preserve multi-line strings as YAML block scalars
8. Remove invalid `hosting: gpu_layers` fields

## Models

| Role | Model | Purpose |
|------|-------|---------|
| Planner | `Qwen3-4B-Instruct-2507-Q4_K_M` | Decompose, plan, write prompts |
| Coder | `Qwen2.5-Coder-3B-Instruct-Q8_0` | Assemble YAML, validate |
| Executor | `Qwen2.5-Coder-3B-Instruct-Q8_0` | Run in generated workflows |

Available models:
```bash
curl -s http://localhost:8080/v1/models | jq '.data[].id' -r
```

## Generated Workflow Structure

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
        Here is the data to process:
        {{bookmarks.shell_output.stdout}}

        <task instructions>
      when:
        before_step_starts:
          - shell:
              command: "cat"
              args: ["<filepath>"]
              fail_on_error: false
        after_step_succeeds:
          - save_to: "./outputs/step_01_name-output.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
              level: error
```

## Critical Rules

1. **Zero-pad step names**: Use `step_01`, `step_02`, not `step_1`, `step_2`. YAML mapping keys sort alphabetically, so `step_10` sorts before `step_2`.

2. **Shell hooks for file reading**: Use `before_step_starts` shell hooks with `cat`/`head`/`find` and reference `{{bookmarks.shell_output.stdout}}` in prompts. The post-processor injects this automatically.

3. **Template interpolation**: Use `{{step.step_name.output}}` to pass output between steps.

4. **Keep prompts under 4K tokens**: 3B model has ~4K token context. Concise and specific prompts only.

5. **Small chunks, many steps**: Break complex tasks into multiple small steps.

6. **Always run post-processor**: The 3B model can't reliably produce correct hook structure. Post-processor fixes this.

## Directory Structure

```
real-workflow-attempts/
├── meta-workflow-v3.yml          # v3 generator (RECOMMENDED)
├── meta-workflow-v2.yml          # v2 generator (deprecated)
├── meta-workflow-template.yml    # v1 generator (deprecated)
├── scripts/
│   ├── validate-yaml.py          # Check generated YAML against schema
│   └── post-process-workflow.py  # Fix structure, inject hooks/prompts
└── outputs/
    ├── v3-test1-* through v3-test4-*  # v3 live test results
    ├── v2-test-*/                      # v2 live test results
    ├── v1-attempts/                    # v1 attempt ymls + generator logs
    ├── gen-*/                          # v1 generation runs
    ├── test-*/                         # v1 test runs
    ├── idea-board/                     # 10-step note analysis outputs
    └── config-drift-run/              # Config drift analysis outputs
```

## Troubleshooting

### Server Connection Errors
```
500 Internal Server Error – proxy error: Could not establish connection
```
Model not loaded. Check available models:
```bash
curl -s http://localhost:8080/v1/models | jq '.data[].id' -r
```

### Steps Execute Out of Order
Zero-pad step names. `step_10` sorts before `step_2`. Always use `step_01` through `step_99`.

### Empty Output or "NO IDEAS FOUND"
3B model may lack capability. Try:
- Larger model (7B if available)
- More specific prompt with examples
- Smaller input data (`head -c 8000` instead of `head -c 12000`)

### YAML Parse Error in Post-Processor
The 3B model may generate shell commands with special characters that break YAML parsing. Post-processor handles most cases automatically.

### /tmp Space Full
```bash
df -h /tmp
rm -rf /tmp/tech-notes-extract /tmp/idea-board-* /tmp/run*.log
```
Use `--min-tmp-space 1` to bypass minimum space check.

## Current Status (as of 2026-06-03)

### v4 Meta-Workflow (RECOMMENDED)
- ✅ 6-step pipeline: classify→decompose→plan→prompts→assemble→validate
- ✅ Complexity classification (SIMPLE/MEDIUM/COMPLEX) routes to appropriate decomposition
- ✅ Research-informed: Continuous Execution Lock, no STOP language, tighter decomposition rules
- ✅ Shell command auto-split: `execute_shell()` handles commands like `ls /tmp/` natively
- ✅ Post-processor: fence stripping, step nesting, hook injection, bookmark injection, block-style YAML
- ✅ Generated workflows execute end-to-end with REAL data processing
- ✅ **BREAKTHROUGH**: LLM processes real shell output via bookmarks instead of hallucinating
- ✅ 7 live tests completed (v3-test1-4, v4-test1-2b)

### v3 Meta-Workflow (DEPRECATED)
- Still functional but v4 produces better quality output

### Test Results Summary
| Test | Prompt | Steps | Meta | Generated | Quality |
|------|--------|-------|------|-----------|---------|
| v4-test1a | Read /etc/hostname | 2 | 6/6 | 2/2 | ✅ HIGH — correct output "myhost.example.com" |
| v4-test1b | Read /etc/hostname (rerun) | 2 | 6/6 | 2/2 | ✅ HIGH — correct output |
| v4-test2 | Count .txt files in /tmp | 3 | 6/6 | 3/3 | ✅ MEDIUM — granite produced perfect YAML, shell hooks failed (pre-fix) |
| v4-test2b | Count .txt files in /tmp (shell fix) | 3 | 6/6 | 3/3 | ✅ HIGH — LLM counted 25 .txt files from REAL data |
| v3-test1 | Markdown TOC | 6 | 5/5 | 6/6 | ⚠️ Low (explains vs does) |
| v3-test2 | CSV line count | 8 | 5/5 | 8/8 | ⚠️ Medium (JSON valid, some hallucination) |
| v3-test3 | List files with sizes | 6 | 5/5 | 6/6 | ⚠️ Medium (shell cmds preserved, table generated) |
| v3-test4 | Count .txt files | 6 | 5/5 | 6/6 | ⚠️ Medium (bookmarks injected, some data used) |

### Known Limitations
- ⚠️ Shell commands with `{{step.X.output}}` template refs fail (not resolved in command field)
- ⚠️ Complexity classifier sometimes underestimates (SIMPLE when should be MEDIUM)
- ⚠️ 3B model occasionally explains instead of doing — inherent limitation
- ❌ Sub-workflow support not yet implemented
- ❌ No iterative verify/fix cycles within generation
