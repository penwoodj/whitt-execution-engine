# Example Workflow Differences: v1 → v2 Schema

> Summary of key differences between v1 (old) and v2 (current) unified schema patterns, based on the 53 example workflows in this repository.

---

## 1. Top-Level Structure

| Aspect | v1 | v2 |
|--------|----|----|
| Workflow container | `pipeline:` | `agentic_workflow:` |
| Schema size | 1705 lines | 801 lines |
| Design philosophy | Explicit everything | Presence = enabled, deduplicated |

### Example

```yaml
# v1 (DEPRECATED)
pipeline:
  workflow_id: my-workflow
  name: "My Workflow"
  steps: [...]

# v2 (CURRENT)
agentic_workflow:
  workflow_id: my-workflow
  name: "My Workflow"
  steps: [...]
```

---

## 2. Step Type Declaration

| Aspect | v1 | v2 |
|--------|----|----|
| Step type | Explicit `type:` key on every step | Inferred from step keys |
| Identification | `id:` field + `type:` | Step key name only |

### Example

```yaml
# v1
steps:
  my_step:
    id: step_1
    type: agent
    generative_entity: model_1
    prompt: "Do something"

# v2
steps:
  my_step:
    generative_entity: model_1
    prompt: "Do something"
    # Step type inferred: has generative_entity → agent step
```

**v2 step type inference rules:**
- `generative_entity` + `prompt` → agent/LLM step
- `tool:` + `file_read` / `shell_exec` → tool step
- `when:` → conditional/GWT step
- `sub_workflow:` → sub-workflow invocation
- `loop:` → loop step (count, validation, retry, infinite, time)
- `user_input:` → user input step
- `abort:` → abort step

---

## 3. Input Variables

| Aspect | v1 | v2 |
|--------|----|----|
| Key name | `input_variables:` | `inputs:` |
| Location | Per-step | Workflow-level + per-step |

### Example

```yaml
# v1
steps:
  analyze:
    input_variables:
      - name: document
        source: "${upload.output}"

# v2
agentic_workflow:
  inputs:
    document:
      description: "Document to analyze"
  steps:
    analyze:
      inputs:
        document: "${inputs.document}"
```

---

## 4. Enabled/Disabled Flags

| Aspect | v1 | v2 |
|--------|----|----|
| Enable feature | `enabled: true` | Omit the key (presence = enabled) |
| Disable feature | `enabled: false` | `disabled: true` |

### Example

```yaml
# v1
tool_permissions:
  web_operations:
    enabled: true
  shell_exec:
    enabled: false

# v2
tool_permissions:
  web_operations: {}        # presence = enabled
  shell_exec:
    disabled: true           # explicit disable
```

---

## 5. Output Configuration

| Aspect | v1 | v2 |
|--------|----|----|
| Output section | Explicit `output:` on every step | Auto-created via `{{step.name.output}}` + hooks |
| Saving | `save_to:` / `append_to:` | Aliases, same action |

---

## 6. Model Configuration

| Aspect | v1 | v2 |
|--------|----|----|
| Backend framework | `framework:` key | Removed (implementation detail) |
| Model routing | `models.routing:` | Removed (model router project) |
| Thinking mode | Not present | `thinking:` with budget tokens |
| Guardrails | Not present | `guardrails:` with enforcement policies |
| Resource allocation | `allocation:` | `ram_allocation:` |

### Example

```yaml
# v1
models:
  main_model:
    provider: lmstudio
    model_id: llama-3
    framework: Agentsdk
    allocation: adaptive

# v2
models:
  main_model:
    provider: lmstudio
    model_id: llama-3
    ram_allocation: 4GB
    thinking:
      budget_tokens: 2000
    guardrails:
      input:
        enabled: false
      output:
        max_tokens: 4096
```

---

## 7. Features Demonstrated (Removed)

| Aspect | v1 | v2 |
|--------|----|----|
| Metadata key | `features_demonstrated:` at end of workflow | Removed entirely |

```yaml
# v1 (DEPRECATED - remove this section)
features_demonstrated:
  - zero_shot_prompting
  - chain_of_thought_reasoning
  - structured_data_extraction

# v2 — No equivalent. This was a review artifact.
```

---

## 8. Loop Constructs

| Aspect | v1 | v2 |
|--------|----|----|
| Loop types | Separate constructs | Unified under single `loop:` key with 5 types |
| Types | Basic for/while | `count`, `time`, `validation`, `retry`, `infinite` |

### Example

```yaml
# v2 — Unified loop syntax
steps:
  refine_until_good:
    loop:
      type: validation          # count | time | validation | retry | infinite
      validation_step: check_quality
      max_iterations: 10
      exit_condition: "${check_quality.output.score > 0.9}"
```

---

## 9. System Storage Paths

| Aspect | v1 | v2 |
|--------|----|----|
| Workspace path | `.glyphnova/` | `./workspace/` |
| Memory storage | `.glyphnova/memory/` | `./workspace/memory/` |
| Run artifacts | `.glyphnova/runs/` | `./workspace/runs/` |
| System of record | `.glyphnova/` system-of-record | `workspace` system-of-record |

---

## 10. AI Operations

| Aspect | v1 | v2 |
|--------|----|----|
| Section | `ai_operations:` top-level | Absorbed into `tool_permissions` with rate limits |

```yaml
# v1
ai_operations:
  enabled: true
  max_concurrent: 5

# v2
tool_permissions:
  ai_operations:
    max_concurrent: 5
```

---

## 11. Hook System

| Aspect | v1 | v2 |
|--------|----|----|
| Lifecycle hooks | `lifecycle_hooks:` | Both work; `when:` preferred |
| GWT conditional | `gwt:` / `given_when_then:` | Both work; `gwt:` preferred |
| Logging | `logging:` top-level section | Logging only through `when` hooks |

---

## 12. Workflow Files Affected

### v1-Only Examples (Archived)
- `manual/agentic-workflow-manual-brainstorm.yml` — Pre-v2 brainstorm
- `chatgpt/agentic-workflow-interaction-examples.yml` — v1alpha1 draft

### v2-Compliant Examples (52 files)
All 52 files in `requirements-oriented-auto/` across 19 categories:
- 01-model-configuration (4 files)
- 02-step-types (4 files)
- 03-data-flow (3 files)
- 04-parallel-execution (3 files)
- 04-loops-convergence (4 files)
- 05-file-operations (2 files)
- 06-web-operations (3 files)
- 07-rag-operations (2 files)
- 08-script-cli (2 files)
- 09-sub-workflows (2 files)
- 10-conditional-branching (2 files)
- 11-error-handling-retries (3 files)
- 12-logging-monitoring (3 files)
- 13-checkpointing-state (2 files)
- 14-resource-management (3 files)
- 15-tool-permissions (2 files)
- 16-user-inputs-ui (2 files)
- 17-hooks-lifecycle (4 files)
- 18-comprehensive-integration (2 files)

---

## Migration Checklist

- [ ] Replace `pipeline:` → `agentic_workflow:`
- [ ] Remove `type:` from step definitions
- [ ] Rename `input_variables:` → `inputs:`
- [ ] Remove `enabled: true` (presence = enabled)
- [ ] Change `enabled: false` → `disabled: true`
- [ ] Remove `features_demonstrated:` sections
- [ ] Remove `framework:` from model definitions
- [ ] Rename `allocation:` → `ram_allocation:`
- [ ] Replace `.glyphnova/` → `./workspace/`
- [ ] Move `ai_operations:` into `tool_permissions:`
- [ ] Remove `models.routing:` (handled by model router)
- [ ] Remove `logging:` top-level (use `when` hooks)
- [ ] Unify loops under `loop:` key
