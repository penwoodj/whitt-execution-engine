# 02 — Architecture

> **Status:** Planning Phase — Documentation Only  
> **Phase:** Meta-Workflow Generator Development (Qwen 3.5-9B)  
> **Focus:** System architecture, data flow, component interaction

## Meta-Workflow Orchestrator Design

The meta-workflow orchestrator (`meta-workflow-generator-v6.yml`) is a YAML workflow that chains the five sub-workflows (SW1-SW5) together via shell invocations. The orchestrator does NOT use the `sub_workflow:` step because the engine treats it as a STUB. Instead, it uses shell actions in `before_step_starts` hooks to invoke `whitt benchmark --workflow <sub-workflow-N.yml> --output-dir <run-dir>`. Each sub-workflow runs as a standalone `whitt benchmark` invocation, with state passing via the filesystem (each sub-workflow reads predecessor's .md output via shell `cat`).

### Orchestrator Workflow Structure

```yaml
workflow:
  workflow_id: "meta-workflow-generator-v6"
  name: "Meta-Workflow Generator v6"
  description: "Chains 5 sub-workflows to transform prompts into executable workflows"
  version: "1.0.0"
  author: "Sisyphus-Junior"
  tags: [meta-workflow, qwen-3.5-9b, agentic-pipeline]

providers:
  llama_cpp_with_vulkan:
    config_file: "./providers/llama_cpp.yml"

models:
  "qwen35":
    name: "Qwen3.5-9B-UD-Q4_K_XL.gguf"
    host:
      type: llama_cpp_with_vulkan
      connection_settings:
        host: localhost
        port: 8080
    load_params:
      context_size: 262144
      batch_size: 2048
      ubatch_size: 512
      cache_type_k: "q8_0"
      cache_type_v: "q8_0"
      gpu_layers: 0
      threads: 5
      use_mmap: true
      flash_attn: true
      cont_batching: false
      no_cache_prompt: true
      parallel: 1
    sampling:
      temperature: 0.2
      top_p: 0.9
      top_k: 40
      max_tokens: 4096

agentic_workflow:
  inputs:
    prompt_file:
      type: string
      description: "Path to prompt file (.md)"
    run_id:
      type: string
      description: "Unique run identifier for this pipeline execution"
    output_dir:
      type: string
      default: "./workspace/meta-output"
      description: "Base directory for all outputs"

  steps:
    orchestrator_start:
      generative_entity: "${models.qwen35}"
      prompt: "Initialize meta-workflow generator run. Create output directory structure and log start metadata."
      when:
        before_step_starts:
          - shell:
              command: "mkdir -p {{inputs.output_dir}}/sw1 {{inputs.output_dir}}/sw2 {{inputs.output_dir}}/sw3 {{inputs.output_dir}}/sw4 {{inputs.output_dir}}/sw5"
              fail_on_error: true
          - log:
              to_file_path: "{{inputs.output_dir}}/orchestrator.log"
              level: info
              message: "Orchestrator started for run_id={{inputs.run_id}}, prompt_file={{inputs.prompt_file}}"
        after_step_succeeds:
          - bookmark: true
          - log:
              to_file_path: "{{inputs.output_dir}}/orchestrator.log"
              level: info
              message: "Orchestrator initialization complete, directory structure created"

    run_sw1:
      generative_entity: "${models.qwen35}"
      prompt: "Execute SW1: Task Deconstruction sub-workflow. Read prompt from {{inputs.prompt_file}}, produce tasks.md with hierarchical task tree and story-point complexity scoring."
      when:
        before_step_starts:
          - shell:
              command: "./target/release/whitt benchmark --workflow workflows/sw1-task-deconstruction.yml --output-dir {{inputs.output_dir}}/sw1"
              fail_on_error: true
          - log:
              to_file_path: "{{inputs.output_dir}}/orchestrator.log"
              level: info
              message: "Starting SW1: Task Deconstruction"
        after_step_succeeds:
          - shell:
              command: "if [ ! -f {{inputs.output_dir}}/sw1/tasks.md ]; then echo 'ERROR: SW1 did not produce tasks.md' && exit 1; fi"
              fail_on_error: true
          - log:
              to_file_path: "{{inputs.output_dir}}/orchestrator.log"
              level: info
              event_fields: [step_name, duration_ms]
              message: "SW1 complete, tasks.md validated"
          - bookmark: true
        after_step_fails:
          - log:
              to_file_path: "{{inputs.output_dir}}/orchestrator.log"
              level: error
              event_fields: [error_message]
              message: "SW1 failed, aborting pipeline"
          - fail: "SW1 Task Deconstruction failed, cannot continue"

    run_sw2:
      generative_entity: "${models.qwen35}"
      prompt: "Execute SW2: Desired Output State sub-workflow. Read tasks.md from SW1, produce outputs.md with testable end-state criteria per task."
      requires: [run_sw1]
      when:
        before_step_starts:
          - shell:
              command: "./target/release/whitt benchmark --workflow workflows/sw2-desired-output-state.yml --output-dir {{inputs.output_dir}}/sw2"
              fail_on_error: true
          - log:
              to_file_path: "{{inputs.output_dir}}/orchestrator.log"
              level: info
              message: "Starting SW2: Desired Output State"
        after_step_succeeds:
          - shell:
              command: "if [ ! -f {{inputs.output_dir}}/sw2/outputs.md ]; then echo 'ERROR: SW2 did not produce outputs.md' && exit 1; fi"
              fail_on_error: true
          - log:
              to_file_path: "{{inputs.output_dir}}/orchestrator.log"
              level: info
              event_fields: [step_name, duration_ms]
              message: "SW2 complete, outputs.md validated"
          - bookmark: true
        after_step_fails:
          - log:
              to_file_path: "{{inputs.output_dir}}/orchestrator.log"
              level: error
              event_fields: [error_message]
              message: "SW2 failed, aborting pipeline"
          - fail: "SW2 Desired Output State failed, cannot continue"

    run_sw3:
      generative_entity: "${models.qwen35}"
      prompt: "Execute SW3: Agentic Categorization sub-workflow. Read tasks.md + outputs.md, produce categories.md with behavior category assignments."
      requires: [run_sw2]
      when:
        before_step_starts:
          - shell:
              command: "./target/release/whitt benchmark --workflow workflows/sw3-agentic-categorization.yml --output-dir {{inputs.output_dir}}/sw3"
              fail_on_error: true
          - log:
              to_file_path: "{{inputs.output_dir}}/orchestrator.log"
              level: info
              message: "Starting SW3: Agentic Categorization"
        after_step_succeeds:
          - shell:
              command: "if [ ! -f {{inputs.output_dir}}/sw3/categories.md ]; then echo 'ERROR: SW3 did not produce categories.md' && exit 1; fi"
              fail_on_error: true
          - log:
              to_file_path: "{{inputs.output_dir}}/orchestrator.log"
              level: info
              event_fields: [step_name, duration_ms]
              message: "SW3 complete, categories.md validated"
          - bookmark: true
        after_step_fails:
          - log:
              to_file_path: "{{inputs.output_dir}}/orchestrator.log"
              level: error
              event_fields: [error_message]
              message: "SW3 failed, aborting pipeline"
          - fail: "SW3 Agentic Categorization failed, cannot continue"

    run_sw4:
      generative_entity: "${models.qwen35}"
      prompt: "Execute SW4: YAML Substructure Translation sub-workflow. Read tasks.md + outputs.md + categories.md, produce structs.md with YAML skeleton structures."
      requires: [run_sw3]
      when:
        before_step_starts:
          - shell:
              command: "./target/release/whitt benchmark --workflow workflows/sw4-yaml-substructure-translation.yml --output-dir {{inputs.output_dir}}/sw4"
              fail_on_error: true
          - log:
              to_file_path: "{{inputs.output_dir}}/orchestrator.log"
              level: info
              message: "Starting SW4: YAML Substructure Translation"
        after_step_succeeds:
          - shell:
              command: "if [ ! -f {{inputs.output_dir}}/sw4/structs.md ]; then echo 'ERROR: SW4 did not produce structs.md' && exit 1; fi"
              fail_on_error: true
          - log:
              to_file_path: "{{inputs.output_dir}}/orchestrator.log"
              level: info
              event_fields: [step_name, duration_ms]
              message: "SW4 complete, structs.md validated"
          - bookmark: true
        after_step_fails:
          - log:
              to_file_path: "{{inputs.output_dir}}/orchestrator.log"
              level: error
              event_fields: [error_message]
              message: "SW4 failed, aborting pipeline"
          - fail: "SW4 YAML Substructure Translation failed, cannot continue"

    run_sw5:
      generative_entity: "${models.qwen35}"
      prompt: "Execute SW5: Final Workflow Assembly sub-workflow. Read structs.md, assemble full workflow.yml, validate against schema."
      requires: [run_sw4]
      when:
        before_step_starts:
          - shell:
              command: "./target/release/whitt benchmark --workflow workflows/sw5-final-workflow-assembly.yml --output-dir {{inputs.output_dir}}/sw5"
              fail_on_error: true
          - log:
              to_file_path: "{{inputs.output_dir}}/orchestrator.log"
              level: info
              message: "Starting SW5: Final Workflow Assembly"
        after_step_succeeds:
          - shell:
              command: "if [ ! -f {{inputs.output_dir}}/sw5/workflow.yml ]; then echo 'ERROR: SW5 did not produce workflow.yml' && exit 1; fi"
              fail_on_error: true
          - shell:
              command: "scripts/validate-yaml.py --schema docs/schema/unified-workflow-schema.yml --input {{inputs.output_dir}}/sw5/workflow.yml"
              fail_on_error: true
          - log:
              to_file_path: "{{inputs.output_dir}}/orchestrator.log"
              level: info
              event_fields: [step_name, duration_ms]
              message: "SW5 complete, workflow.yml validated"
          - save_to:
              - "./workspace/final-workflows/{{inputs.run_id}}-workflow.yml"
              - final_workflow
          - bookmark: true
        after_step_fails:
          - log:
              to_file_path: "{{inputs.output_dir}}/orchestrator.log"
              level: error
              event_fields: [error_message]
              message: "SW5 failed, pipeline incomplete"
          - fail: "SW5 Final Workflow Assembly failed, pipeline incomplete"

    pipeline_complete:
      generative_entity: "${models.qwen35}"
      prompt: "Meta-workflow pipeline complete. Generate summary report for run_id={{inputs.run_id}} with all outputs and quality metrics."
      requires: [run_sw5]
      when:
        after_step_succeeds:
          - log:
              to_file_path: "{{inputs.output_dir}}/orchestrator.log"
              level: info
              message: "Pipeline complete, final workflow ready at ./workspace/final-workflows/{{inputs.run_id}}-workflow.yml"
          - notify:
              message: "Meta-workflow generator complete: {{inputs.run_id}}. Final workflow available at ./workspace/final-workflows/{{inputs.run_id}}-workflow.yml"

workflow_execution_strategy:
  load_unload: one_at_a_time
  timeout:
    total: 4h
    per_operation:
      tool_call: 30m
      step: 8m
      sub_workflow: 300s
      time_to_first_result_secs: 120
    timeout_strategy: continue_with_partial
```

### Orchestrator Design Decisions

**Why Shell-Based Orchestration Instead of sub_workflow: Step:**

The `sub_workflow:` step is a STUB in the current engine implementation. The runner does not validate sub-workflow references, does not create nested execution contexts, and does not handle sub-workflow input/output mapping. Using shell invocations provides:

1. **Deterministic execution** — Each sub-workflow runs as independent process with isolated environment.
2. **Explicit state passing** — Filesystem-based state transfer is explicit and traceable.
3. **Failure isolation** — Sub-workflow failures don't crash the orchestrator; handled via shell exit codes.
4. **Run ID substitution** — Using `sed` pattern from v5 for run ID injection into sub-workflow paths.
5. **Logging granularity** — Each sub-workflow has dedicated log files for detailed debugging.

**Why Single run_id Parameter for All Sub-Workflows:**

Run ID provides traceability across all sub-workflow outputs. The directory structure uses run_id for organization:

```
./workspace/meta-output/<run_id>/
├── orchestrator.log
├── sw1/tasks.md
├── sw2/outputs.md
├── sw3/categories.md
├── sw4/structs.md
└── sw5/workflow.yml
```

This structure enables:

- Reproducibility: same prompt with same run_id should produce same outputs
- Parallelization: different run_ids can execute simultaneously without conflict
- Debugging: all artifacts for a single pipeline run in one directory
- Archiving: completed runs can be archived or deleted as units

**Why Validation Checks in Shell Hooks Instead of GWT:**

Shell command validation provides immediate feedback and clear error messages. GWT expressions could perform the same checks, but:

1. Shell validation is more explicit (easier to debug)
2. Shell exit codes are standard (0 = success, non-zero = failure)
3. Shell error messages are captured in orchestrator logs
4. Shell commands can perform file existence checks more reliably than GWT file_path evaluation

**Why Bookmark State in Each Step:**

Bookmarks create checkpoints that enable:

1. **Resume capability** — If orchestrator fails mid-pipeline, can resume from last bookmark
2. **State inspection** — Can inspect bookmark state to understand what completed
3. **Rollback support** — Can roll back to previous bookmark if iteration needed
4. **Progress tracking** — Bookmark presence indicates which stages completed

## 5-Sub-Workflow Pipeline Architecture

The five-sub-workflow pipeline transforms unstructured prompts into executable YAML workflows through a staged refinement process. Each stage has a specific responsibility and produces a single markdown artifact that accumulates through the pipeline.

### SW1: Task Deconstruction Architecture

**Purpose:** Break high-complexity prompts into atomic tasks with story-point complexity scoring.

**Input:** Raw user prompt (natural language task description).

**Output:** `tasks.md` — Hierarchical task tree with complexity scores.

**Internal Processing Steps:**

1. **Initial Task Breakdown** — LLM call breaks prompt into candidate atomic tasks (group of ~5). This uses the generative_entity `${models.qwen35}` with a prompt template emphasizing task decomposition best practices.

2. **Complexity Scoring Loop** — For each task group, LLM call scores complexity by:
   - Expanding each task into agentic subtasks
   - Counting subtasks (more subtasks = higher complexity)
   - Evaluating subtask complexity (file reads vs code generation vs system configuration)
   - Assigning story point score (1-13 scale based on subtask count + complexity)

3. **Decomposition Loop** — Any task scoring ≥ 5 pts MUST be expanded into subtasks:
   - If task ≥ 5 pts: expand into subtasks, repeat complexity scoring
   - If task < 5 pts: mark as atomic, add to final tree
   - Loop until all tasks are atomic or decomposition depth limit reached (max 3 levels)

4. **GWT Quality Gate** — Tasks must satisfy criteria:
   - Atomic: Task accomplishes single cohesive objective
   - Well-named: Verb-noun pattern, clear scope
   - Not overly verbose: ≤ 15 words per task name
   - Quality engineering: Follows task decomposition best practices
   - GWT criteria flushed: Each task has clear acceptance criteria
   - If any fail: route back to fix step with specific error guidance

5. **Final Assembly** — Accumulate all atomic tasks into hierarchical tree, write to `tasks.md`.

**Hook Integration Points:**

- **before_step_starts**: Log task breakdown start, bookmark initial state, load prompt from input variable.
- **during_step_succeeds**: (Not wired — streaming hook, requires SSE integration)
- **after_step_succeeds**: Save `tasks.md` to output directory, log quality metrics (task count, avg complexity), bookmark completion state.
- **after_step_fails**: Log failure details (error type, message), route to fix step with specific error guidance (invalid structure vs scoring failure vs decomposition loop error).
- **after_all_retries_exhausted**: Log catastrophic failure, fail workflow with diagnostic information.

**Chunking Strategy:**

SW1 uses dynamic chunking based on prompt complexity:

- **Low complexity prompts** (≤ 3 actions): Single chunk, process all tasks in one pass
- **Medium complexity prompts** (4-7 actions): 2 chunks of 3-4 tasks each
- **High complexity prompts** (8+ actions): 3+ chunks of 2-3 tasks each

Chunking prevents context overflow (262144 tokens) and enables focused iteration on specific task groups.

**Model Configuration:**

SW1 uses default Qwen 3.5-9B configuration with modified sampling for deterministic task breakdown:

- **temperature: 0.1** (lower than default 0.2) — More deterministic, less creative task names
- **top_p: 0.8** (lower than default 0.9) — More conservative token selection
- **max_tokens: 2048** (half of default 4096) — Prevents over-generation of tasks

### SW2: Desired Output State Architecture

**Purpose:** Define testable end-state criteria for each task from SW1.

**Input:** `tasks.md` from SW1.

**Output:** `outputs.md` — Testable desired state per task with GWT criteria.

**Internal Processing Steps:**

1. **Load Task Tree** — Read `tasks.md` via shell `cat` in `before_step_starts` hook. Parse hierarchical structure to extract atomic tasks (leaf nodes).

2. **Chunking Strategy** — Group atomic tasks by complexity (start with chunks of 3, grow to 5):
   - **Initial chunk size**: 3 tasks per chunk
   - **Growth logic**: If quality gates pass on all chunks, increase to 4, then 5
   - **Max chunk size**: 5 tasks (prevents context overflow)
   - **Chunk grouping**: Similar complexity tasks grouped together (all low-scoring tasks together, etc.)

3. **Desired State Generation Loop** — For each chunk:
   - LLM call produces "desired end-state" for each task
   - End-state definition: What observable output state proves this task is complete?
   - Include testable criteria in GWT format: "Given <context>, When <condition>, Then <expected outcome>"
   - Criteria count per task: 1-3 (not over-constrained, not under-specified)

4. **GWT Quality Gate per Chunk** — Each desired state must satisfy:
   - Testable: Observable, measurable, verifiable via shell or tool
   - Unambiguous: Clear pass/fail conditions, no subjective judgments
   - Tied to specific task: References task number/name from tasks.md
   - Not over-scoped: Criteria don't exceed task boundaries
   - If any fail: route back to fix step for specific chunk only (efficient iteration)

5. **Merge and Finalize** — Merge all chunk outputs into single `outputs.md`. Verify criteria count matches task count (one section per task).

**Hook Integration Points:**

- **before_step_starts**: Load tasks from `tasks.md` via shell `cat`, log chunking strategy (chunk count, tasks per chunk), bookmark initial state.
- **after_step_succeeds**: Save `outputs.md` to output directory, validate criteria count vs task count (must match), log quality metrics (avg criteria per task), bookmark completion state.
- **after_step_fails**: Log specific criteria failure (testability issue, over-scoping, ambiguity), route to fix step with chunk-specific guidance, preserve successful chunks in bookmark.
- **after_all_retries_exhausted**: Log catastrophic failure, fail workflow with diagnostic information, identify which chunks failed.

**Quality Gate Evaluation:**

SW2 implements a sophisticated GWT-based quality gate system:

```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "criteria_count == task_count"
          when: "All tasks have corresponding desired state criteria"
          then: { route_to: validate_testability }
        - given: "criteria_count > task_count"
          when: "Over-constrained: too many criteria per task"
          then: { route_to: fix_over_constraint }
        - given: "criteria_count < task_count"
          when: "Under-constrained: some tasks missing criteria"
          then: { route_to: fix_under_constraint }
```

**Chunking Strategy Details:**

SW2 uses adaptive chunking based on task complexity:

```
Chunk Size Determination:
  If avg_task_complexity <= 2: chunk_size = 5 (simpler tasks, can process more)
  If avg_task_complexity <= 4: chunk_size = 4 (medium complexity)
  If avg_task_complexity > 4: chunk_size = 3 (complex tasks, process fewer)

Chunk Grouping:
  Group by complexity: all 1-2 point tasks together, 3-4 point tasks together, 5+ point tasks together
  This enables focused prompt templates per complexity group
```

**Model Configuration:**

SW2 uses standard Qwen 3.5-9B configuration with temperature tuned for testable criteria generation:

- **temperature: 0.15** (between SW1's 0.1 and default 0.2) — Balanced determinism and creativity
- **top_p: 0.85** (between SW1's 0.8 and default 0.9) — Balanced conservative and diverse tokens
- **max_tokens: 3072** (3/4 of default 4096) — Allows comprehensive criteria without over-generation

### SW3: Agentic Categorization Architecture

**Purpose:** Assign behavior categories to each task to guide YAML structure mapping.

**Input:** `tasks.md` from SW1 + `outputs.md` from SW2.

**Output:** `categories.md` — Category labels with reasoning per task.

**Internal Processing Steps:**

1. **Load Task and Criteria** — Read `tasks.md` + `outputs.md` via shell `cat` in `before_step_starts` hook. Parse to extract task names, descriptions, and desired state criteria.

2. **Canonical Set Definition** — Define canonical category set (fixed for all prompts):
   - file-read: Read file or directory contents
   - transform-llm: Use LLM to transform content
   - validate-gate: Check condition, route based on result
   - loop-iterate: Iterate over collection or repeat until condition
   - branch-decision: Conditional routing based on GWT expression
   - shell-execute: Run shell command
   - tool-call: Use specific tool
   - checkpoint-state: Save state for later retrieval
   - notify-external: Send notification outside workflow
   - sub-workflow-ref: Invoke external workflow

3. **Categorization Loop** — For each task:
   - LLM call analyzes task description + desired state criteria
   - Assigns category from canonical set based on task behavior
   - Provides reasoning explanation (2-3 sentences explaining why category fits)
   - References specific task characteristics (keywords, action verbs, output type)

4. **GWT Quality Gate per Task** — Each category assignment must satisfy:
   - From canonical set: Must be one of the 10 defined categories
   - Mutually exclusive: One category per task (no multiple categories)
   - Reasoning provided: 2-3 sentences explaining assignment
   - Category consistency: Similar tasks get same category
   - If any fail: route back to fix step for specific task only

5. **Final Assembly** — Assemble all category assignments into `categories.md`. Verify one category per task, all categories from canonical set.

**Hook Integration Points:**

- **before_step_starts**: Load tasks + outputs via shell `cat`, log canonical set (10 categories), bookmark initial state.
- **after_step_succeeds**: Save `categories.md` to output directory, validate category distribution (should have mix of categories, not all same), log quality metrics (category counts), bookmark completion state.
- **after_step_fails**: Log specific categorization failure (invalid category, missing reasoning, consistency issue), route to fix step with task-specific guidance.
- **after_all_retries_exhausted**: Log catastrophic failure, fail workflow with diagnostic information, identify which tasks failed categorization.

**Category Consistency Validation:**

SW3 implements consistency checks to ensure similar tasks get same category:

```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "category_diversity >= 0.3"
          when: "Sufficient category diversity (not all tasks same category)"
          then: { route_to: validate_consistency }
        - given: "category_diversity < 0.3"
          when: "Insufficient diversity, all tasks same category"
          then: { route_to: fix_diversity }
```

**Consistency Rules:**

- File operation tasks (read, write, transform) → file-read or transform-llm or shell-execute
- LLM tasks (generate, summarize, translate) → transform-llm
- Condition checks (if/then logic) → validate-gate or branch-decision
- Iteration tasks (repeat over collection) → loop-iterate
- State management tasks (checkpoint, restore) → checkpoint-state
- External communication tasks (email, webhook) → notify-external

**Model Configuration:**

SW3 uses standard Qwen 3.5-9B configuration with temperature tuned for consistent categorization:

- **temperature: 0.1** (low temperature) — Maximizes consistency, similar tasks get same category
- **top_p: 0.9** (default) — Allows diverse reasoning while maintaining consistent assignments
- **max_tokens: 2048** — Allows comprehensive reasoning without over-generation

### SW4: YAML Substructure Translation Architecture

**Purpose:** Translate categorized tasks into YAML skeleton structures per category.

**Input:** `tasks.md` from SW1 + `outputs.md` from SW2 + `categories.md` from SW3.

**Output:** `structs.md` — YAML code blocks with skeleton structures + intent prose.

**Internal Processing Steps:**

1. **Load All Previous Outputs** — Read `tasks.md` + `outputs.md` + `categories.md` via shell `cat` in `before_step_starts` hook. Parse to create unified task view with names, descriptions, criteria, and categories.

2. **Group by Category** — Group tasks by category label from `categories.md`. This enables category-specific YAML structure generation.

3. **YAML Skeleton Generation Loop** — For each category group:
   - LLM call emits YAML substructure skeleton for each task in that category
   - Skeleton uses `agentic_workflow.steps.<step_name>:` shape from schema lines 318-522
   - Includes: generative_entity, prompt, when hooks, requires dependencies
   - Model reference: `${models.primary-analyzer}` (not `${models.qwen35}`)
   - Hook wiring: Appropriate triggers for category (e.g., file-read → after_step_succeeds with log + save_to)

4. **GWT Quality Gate per Skeleton** — Each YAML substructure must satisfy:
   - Correct schema shape: Follows `agentic_workflow.steps.<step_name>:` structure
   - Model reference correct: References defined model (`${models.primary-analyzer}`)
   - Hook wiring appropriate: Hooks match category expectations
   - Dependencies included: `requires:` list references prior steps
   - No non-YAML code blocks: Only ```yaml blocks, no ```rust, ```python, ```bash
   - If any fail: route back to fix step for specific task only

5. **Final Assembly** — Merge all category outputs into `structs.md`. Include intent prose explaining YAML structure decisions.

**Hook Integration Points:**

- **before_step_starts**: Load all 3 previous outputs via shell `cat`, log category groups (count per category), bookmark initial state.
- **after_step_succeeds**: Save `structs.md` to output directory, validate YAML syntax via `cargo run -- validate-yaml`, validate schema shape (check against unified schema line numbers), log quality metrics (skeleton count per category), bookmark completion state.
- **after_step_fails**: Log specific YAML generation error (syntax error, schema mismatch, wrong model reference), route to fix step with task-specific guidance, preserve successful skeletons in bookmark.
- **after_all_retries_exhausted**: Log catastrophic failure, fail workflow with diagnostic information, identify which tasks failed YAML generation.

**YAML Validation Strategy:**

SW4 implements two-level validation:

1. **Syntax Validation** — Parse YAML with serde_yaml, check for syntax errors:
   ```bash
   cargo run -- parse-yaml --input structs.md
   ```

2. **Schema Validation** — Check structure against unified schema:
   ```yaml
   when:
     after_step_succeeds:
       - shell:
           command: "cargo run -- validate-yaml --schema docs/schema/unified-workflow-schema.yml --input structs.md"
           fail_on_error: true
   ```

**Schema Line Reference Mapping:**

SW4 prompts include specific schema line references to guide YAML generation:

```
Generate YAML skeleton for task "read configuration files" (file-read category):

Schema reference:
- Step structure: docs/schema/unified-workflow-schema.yml lines 318-367
- Hook configuration: docs/schema/unified-workflow-schema.yml lines 298-367
- Template interpolation: docs/schema/unified-workflow-schema.yml lines 752-765
- Required fields: generative_entity, prompt, when (optional)

Expected output:
```yaml
read_config:
  generative_entity: "${models.primary-analyzer}"
  prompt: "Read configuration from {{inputs.config_path}}"
  when:
    after_step_succeeds:
      - save_to:
          - config_data
          - "./workspace/output/config.yaml"
      - log:
          to_file_path: "./workspace/logs/steps.log"
          event_fields: [step_name, duration_ms]
```
```

**Model Configuration:**

SW4 uses standard Qwen 3.5-9B configuration with temperature tuned for YAML syntax correctness:

- **temperature: 0.05** (very low temperature) — Maximizes YAML syntax correctness, minimizes creative deviations from schema
- **top_p: 0.7** (lower than default) — More conservative token selection for precise YAML generation
- **max_tokens: 4096** (default) — Allows complete YAML structures for complex steps

### SW5: Final Workflow Assembly Architecture

**Purpose:** Assemble valid executable workflow YAML from skeleton structures.

**Input:** `structs.md` from SW4.

**Output:** `workflow.yml` — Executable validated agentic workflow YAML.

**Internal Processing Steps:**

1. **Load Skeleton Structures** — Read `structs.md` via shell `cat` in `before_step_starts` hook. Parse to extract YAML skeleton code blocks and intent prose.

2. **Header Assembly** — Generate workflow header:
   - workflow_id: Generated from run_id or timestamp
   - name: Derived from original prompt or auto-generated
   - description: Summarized from original prompt
   - version: "1.0.0" (fixed)
   - author: "Qwen 3.5-9B Meta-Workflow Generator"
   - tags: [meta-workflow, qwen-3.5-9b, auto-generated]

3. **Providers and Models Section** — Generate from template:
   - providers: llama_cpp_with_vulkan with config_file reference
   - models: primary-analyzer referencing Qwen 3.5-9B with full LoadParams
   - Model configuration matches Qwen 3.5-9B spec from orchestrator

4. **Execution Strategy Section** — Generate from template:
   - workflow_execution_strategy with load_unload, timeout, error_handling configs
   - Uses unified schema lines 528-609 as reference

5. **Steps Assembly** — Merge all YAML skeleton structures from `structs.md`:
   - Extract step names from skeletons
   - Resolve `requires:` dependencies (ensure referenced steps exist)
   - Merge `when:` hooks (step-level hooks override workflow-level defaults)
   - Insert into `agentic_workflow.steps:` section

6. **Deterministic Post-Processing** — Run fix and validation scripts:
   - `scripts/fix-generated-yaml.py` — Fix common YAML issues (indentation, missing keys)
   - `scripts/validate-yaml.py` — Validate against unified schema

7. **Exhaustive Review Loop** — Per-task verification:
   - For each task in `tasks.md`: verify task appears in workflow
   - For each task in `tasks.md`: verify task has corresponding step in YAML
   - For each task in `tasks.md`: verify step has appropriate hooks
   - If any task missing or under-implemented: route back to fix step

8. **Final Pass** — Emit `workflow.yml` with complete workflow.

**Hook Integration Points:**

- **before_step_starts**: Load structs via shell `cat`, log assembly strategy (step count, category distribution), bookmark initial state.
- **after_step_succeeds**: Save `workflow.yml` to output directory, validate against schema (schema_valid=true in logs), verify executable (run quick test), log quality metrics (step count, hook count), bookmark completion state.
- **after_step_fails**: Log specific assembly error (missing step, broken dependency, validation failure), route to fix step with task-specific guidance, preserve successful assembly in bookmark.
- **after_all_retries_exhausted**: Log catastrophic failure, fail workflow with diagnostic information, identify which tasks missing from workflow.

**Exhaustive Review Loop Implementation:**

SW5 implements a GWT-based review loop to ensure complete task coverage:

```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "tasks_in_md == steps_in_yaml"
          when: "All tasks from tasks.md present in workflow"
          then: { route_to: validate_hooks }
        - given: "tasks_in_md > steps_in_yaml"
          when: "Some tasks missing from workflow"
          then: { route_to: add_missing_steps }
    - gwt:
        - given: "hook_count_per_step >= 3"
          when: "Each step has sufficient hook coverage"
          then: { route_to: validate_execution }
        - given: "hook_count_per_step < 3"
          when: "Some steps have insufficient hook coverage"
          then: { route_to: add_hooks }
```

**Post-Processing Scripts:**

SW5 relies on two deterministic scripts:

1. **fix-generated-yaml.py** — Fixes common YAML issues:
   - Indentation errors (tabs vs spaces)
   - Missing required keys (adds defaults)
   - Duplicate keys (removes duplicates, keeps first)
   - Invalid template references (removes broken interpolations)
   - Schema non-compliance (removes unknown keys)

2. **validate-yaml.py** — Validates against schema:
   - Parses YAML structure
   - Checks against unified schema line-by-line
   - Validates field types (strings, numbers, booleans, lists)
   - Checks nesting structure (workflow → agentic_workflow → steps)
   - Validates hook configurations (trigger → action format)
   - Returns exit code 0 if valid, non-zero if invalid

**Model Configuration:**

SW5 uses standard Qwen 3.5-9B configuration with temperature tuned for assembly accuracy:

- **temperature: 0.08** (very low temperature) — Maximizes assembly accuracy, preserves skeleton structures
- **top_p: 0.75** (lower than default) — More conservative token selection for precise assembly
- **max_tokens: 8192** (2x default) — Allows complete workflow assembly for complex prompts

## Data Flow Between Sub-Workflows

The sub-workflows communicate exclusively through the filesystem, with each stage producing a single markdown file that subsequent stages read and append to.

### Filesystem State Passing Pattern

```
Phase 1: SW1 produces
  docs/plans/meta-workflow-qwen35/workspace/<run_id>/sw1/tasks.md
  (File contains: hierarchical task tree with complexity scores)

Phase 2: SW2 reads and appends
  docs/plans/meta-workflow-qwen35/workspace/<run_id>/sw1/tasks.md (read via shell cat)
  → docs/plans/meta-workflow-qwen35/workspace/<run_id>/sw2/outputs.md
  (File contains: tasks.md content + appended desired state criteria)

Phase 3: SW3 reads and appends
  docs/plans/meta-workflow-qwen35/workspace/<run_id>/sw2/outputs.md (read via shell cat)
  → docs/plans/meta-workflow-qwen35/workspace/<run_id>/sw3/categories.md
  (File contains: tasks.md + outputs.md content + appended category assignments)

Phase 4: SW4 reads and appends
  docs/plans/meta-workflow-qwen35/workspace/<run_id>/sw3/categories.md (read via shell cat)
  → docs/plans/meta-workflow-qwen35/workspace/<run_id>/sw4/structs.md
  (File contains: tasks.md + outputs.md + categories.md content + appended YAML skeletons)

Phase 5: SW5 reads and produces
  docs/plans/meta-workflow-qwen35/workspace/<run_id>/sw4/structs.md (read via shell cat)
  → docs/plans/meta-workflow-qwen35/workspace/<run_id>/sw5/workflow.yml
  (File contains: assembled workflow YAML, not markdown)
```

### File Reading via Shell Hooks

Each sub-workflow uses shell hooks in `before_step_starts` to read predecessor outputs:

```yaml
when:
  before_step_starts:
    - shell:
        command: "cat {{inputs.output_dir}}/sw1/tasks.md"
        bookmark_as: "predecessor_tasks"
        fail_on_error: true
    - log:
        to_file_path: "./workspace/logs/sw2-start.log"
        level: info
        message: "Loaded tasks.md from SW1, {{bookmarks.predecessor_tasks.word_count}} words"
```

The `bookmark_as: "predecessor_tasks"` directive stores the shell output in the bookmark store under the key "predecessor_tasks", accessible via `{{bookmarks.predecessor_tasks}}` in subsequent steps.

### File Writing via Save_To Actions

Each sub-workflow uses `save_to` actions in `after_step_succeeds` to write outputs:

```yaml
when:
  after_step_succeeds:
    - save_to:
        - "./workspace/sw2-outputs/{{inputs.run_id}}-outputs.md"
        - accumulated_outputs
    - log:
        to_file_path: "./workspace/logs/sw2-complete.log"
        level: info
        message: "Wrote outputs.md to ./workspace/sw2-outputs/{{inputs.run_id}}-outputs.md"
```

The `save_to` action writes to both file path and bookmark variable, enabling traceability and subsequent access.

### Run ID Substitution Pattern

The orchestrator uses run ID substitution to ensure all outputs are organized by run:

```yaml
# In orchestrator YAML
inputs:
  run_id:
    type: string

# In sub-workflow invocation
- shell:
    command: "sed 's/__RUN_ID__/{{inputs.run_id}}/g' workflows/sw1-task-deconstruction.yml > /tmp/sw1-{{inputs.run_id}}.yml && ./target/release/whitt benchmark --workflow /tmp/sw1-{{inputs.run_id}}.yml --output-dir ./workspace/meta-output/{{inputs.run_id}}/sw1"
    fail_on_error: true
```

This pattern from v5 ensures:
- Each run has isolated output directory
- No conflicts between concurrent runs
- Easy archiving and deletion of completed runs
- Reproducibility (same run_id = same outputs)

### Failure Propagation Across Stages

If a sub-workflow fails, subsequent stages cannot proceed. The orchestrator implements failure propagation:

```yaml
run_sw1:
  when:
    after_step_fails:
      - log:
          to_file_path: "{{inputs.output_dir}}/orchestrator.log"
          level: error
          event_fields: [error_message]
          message: "SW1 failed, aborting pipeline"
      - fail: "SW1 Task Deconstruction failed, cannot continue"

run_sw2:
  requires: [run_sw1]  # Will not execute if SW1 fails
  when:
    on_requires_failed:
      - log:
          to_file_path: "{{inputs.output_dir}}/orchestrator.log"
          level: error
          message: "Dependency SW1 failed, SW2 cannot execute"
      - fail: "Prerequisite SW1 failed, aborting pipeline"
```

## Single .md Output File Per Sub-Workflow

Each sub-workflow produces exactly one markdown output file that accumulates through the pipeline. This design choice has specific architectural implications.

### Output File Structure

**SW1 Output (tasks.md):**

```markdown
# Task Deconstruction for Prompt: <prompt_summary>

## Root Tasks

### 1. Read Configuration Files
- **Description:** Read all configuration files from /etc/app/ and /usr/local/etc/app/
- **Story Points:** 2
- **Subtasks:** None (atomic)

### 2. Parse Configuration Data
- **Description:** Parse YAML and JSON configuration files into structured data
- **Story Points:** 3
- **Subtasks:** None (atomic)

### 3. Validate Configuration Schema
- **Description:** Validate parsed configurations against schema definitions
- **Story Points:** 4
- **Subtasks:**
  - 3.1 Load schema files (1 point)
  - 3.2 Validate structure (2 points)
  - 3.3 Report validation errors (1 point)

## Task Statistics
- Total tasks: 3
- Atomic tasks: 2
- Composite tasks: 1
- Max story points: 4
- Avg story points: 3.0
```

**SW2 Output (outputs.md):** (Accumulates tasks.md + appends)

```markdown
# Task Deconstruction for Prompt: <prompt_summary>

[... tasks.md content ...]

# Desired Output States

## Task 1: Read Configuration Files

### Desired End-State
Configuration files are loaded into memory and accessible to subsequent steps.

### Testable Criteria
- Given configuration file paths, When files are read, Then file contents are available in memory
- Given invalid file path, When read attempt fails, Then error is logged and workflow continues

## Task 2: Parse Configuration Data

### Desired End-State
Configuration data is parsed into structured format (YAML objects, JSON objects).

### Testable Criteria
- Given YAML configuration file, When file is parsed, Then structured data is accessible
- Given JSON configuration file, When file is parsed, Then structured data is accessible
- Given invalid syntax, When parse attempt fails, Then error is logged and workflow continues

## Task 3: Validate Configuration Schema

### Desired End-State
Parsed configurations are validated against schema definitions with errors reported.

### Testable Criteria
- Given parsed configuration data, When validation runs, Then errors are reported if schema violations exist
- Given valid configuration, When validation runs, Then no errors reported

## Criteria Statistics
- Total tasks: 3
- Total criteria: 6
- Avg criteria per task: 2.0
```

**SW3 Output (categories.md):** (Accumulates tasks.md + outputs.md + appends)

```markdown
# Task Deconstruction for Prompt: <prompt_summary>

[... tasks.md content ...]

# Desired Output States

[... outputs.md content ...]

# Category Assignments

## Task 1: Read Configuration Files

### Category: file-read

### Reasoning
Task involves reading files from filesystem, no transformation or processing required. Fits file-read category which is defined for "Read file or directory contents" operations.

## Task 2: Parse Configuration Data

### Category: transform-llm

### Reasoning
Task involves using LLM to parse and structure configuration data (YAML to objects, JSON to objects). Fits transform-llm category which is defined for "Use LLM to transform content" operations.

## Task 3: Validate Configuration Schema

### Category: validate-gate

### Reasoning
Task involves checking conditions and routing based on validation results (errors vs no errors). Fits validate-gate category which is defined for "Check condition, route based on result" operations.

## Category Statistics
- Total tasks: 3
- Categories used: 3 (file-read, transform-llm, validate-gate)
- Category distribution: file-read (33%), transform-llm (33%), validate-gate (33%)
```

**SW4 Output (structs.md):** (Accumulates tasks.md + outputs.md + categories.md + appends)

```markdown
# Task Deconstruction for Prompt: <prompt_summary>

[... tasks.md content ...]

# Desired Output States

[... outputs.md content ...]

# Category Assignments

[... categories.md content ...]

# YAML Skeleton Structures

## Category: file-read

### Task 1: Read Configuration Files

```yaml
read_config_files:
  generative_entity: "${models.primary-analyzer}"
  prompt: "Read all configuration files from /etc/app/ and /usr/local/etc/app/"
  when:
    after_step_succeeds:
      - save_to:
          - config_files_content
          - "./workspace/output/config_files.yaml"
      - log:
          to_file_path: "./workspace/logs/file-read.log"
          event_fields: [step_name, file_count]
          level: info
```

## Category: transform-llm

### Task 2: Parse Configuration Data

```yaml
parse_config_data:
  generative_entity: "${models.primary-analyzer}"
  prompt: "Parse YAML and JSON configuration files into structured data using config_files_content"
  requires: [read_config_files]
  when:
    after_step_succeeds:
      - save_to:
          - parsed_config_data
          - "./workspace/output/parsed_config.yaml"
      - log:
          to_file_path: "./workspace/logs/transform-llm.log"
          event_fields: [step_name, config_count]
          level: info
```

## Category: validate-gate

### Task 3: Validate Configuration Schema

```yaml
validate_config_schema:
  generative_entity: "${models.primary-analyzer}"
  prompt: "Validate parsed_config_data against schema definitions, report errors"
  requires: [parse_config_data]
  when:
    after_step_succeeds:
      - gwt:
          - given: "validation_errors.length == 0"
            then: { route_to: success_report }
            else: { route_to: error_report }
      - log:
          to_file_path: "./workspace/logs/validate-gate.log"
          event_fields: [step_name, error_count]
          level: info
```

## Skeleton Statistics
- Total tasks: 3
- Total skeletons: 3
- Skeletons per category: file-read (1), transform-llm (1), validate-gate (1)
```

**SW5 Output (workflow.yml):** (Not markdown, final YAML)

```yaml
workflow:
  workflow_id: "auto-generated-20260617-143052"
  name: "Configuration Validation Workflow"
  description: "Read, parse, and validate configuration files"
  version: "1.0.0"
  author: "Qwen 3.5-9B Meta-Workflow Generator"
  tags: [meta-workflow, qwen-3.5-9b, auto-generated, configuration]

providers:
  llama_cpp_with_vulkan:
    config_file: "./providers/llama_cpp.yml"

models:
  primary-analyzer:
    name: "Qwen3.5-9B-UD-Q4_K_XL.gguf"
    host:
      type: llama_cpp_with_vulkan
      connection_settings:
        host: localhost
        port: 8080
    load_params:
      context_size: 262144
      batch_size: 2048
      ubatch_size: 512
      cache_type_k: "q8_0"
      cache_type_v: "q8_0"
      gpu_layers: 0
      threads: 5
      use_mmap: true
      flash_attn: true
      cont_batching: false
      no_cache_prompt: true
      parallel: 1
    sampling:
      temperature: 0.2
      top_p: 0.9
      top_k: 40
      max_tokens: 4096

agentic_workflow:
  steps:
    read_config_files:
      generative_entity: "${models.primary-analyzer}"
      prompt: "Read all configuration files from /etc/app/ and /usr/local/etc/app/"
      when:
        after_step_succeeds:
          - save_to:
              - config_files_content
              - "./workspace/output/config_files.yaml"
          - log:
              to_file_path: "./workspace/logs/file-read.log"
              event_fields: [step_name, file_count]
              level: info

    parse_config_data:
      generative_entity: "${models.primary-analyzer}"
      prompt: "Parse YAML and JSON configuration files into structured data using config_files_content"
      requires: [read_config_files]
      when:
        after_step_succeeds:
          - save_to:
              - parsed_config_data
              - "./workspace/output/parsed_config.yaml"
          - log:
              to_file_path: "./workspace/logs/transform-llm.log"
              event_fields: [step_name, config_count]
              level: info

    validate_config_schema:
      generative_entity: "${models.primary-analyzer}"
      prompt: "Validate parsed_config_data against schema definitions, report errors"
      requires: [parse_config_data]
      when:
        after_step_succeeds:
          - gwt:
              - given: "validation_errors.length == 0"
                then: { route_to: success_report }
                else: { route_to: error_report }
          - log:
              to_file_path: "./workspace/logs/validate-gate.log"
              event_fields: [step_name, error_count]
              level: info

workflow_execution_strategy:
  load_unload: one_at_a_time
  timeout:
    total: 4h
    per_operation:
      tool_call: 30m
      step: 8m
      sub_workflow: 300s
      time_to_first_result_secs: 120
    timeout_strategy: continue_with_partial
```

### Accumulation Pattern Benefits

The single-file accumulation pattern provides several architectural benefits:

1. **Traceability** — Each stage's output includes all previous stages' content, enabling full traceability from original prompt to final workflow.

2. **Debugging** — If an error occurs in SW5, can inspect `structs.md` to see all accumulated content up to SW4, isolating the failure stage.

3. **Iteration** — If SW3 produces poor category assignments, can fix SW3 prompt and re-run SW3-SW5 without re-running SW1-SW2.

4. **Validation** — Each stage can validate previous stages' content (e.g., SW4 validates that tasks from SW1 have skeletons, SW5 validates that all tasks appear in workflow).

5. **Archive** — Each stage's output is a complete artifact, enabling archiving and version control of intermediate results.

### Accumulation Pattern Drawbacks

The single-file accumulation pattern has some drawbacks:

1. **File Size Growth** — Files grow larger through pipeline (tasks.md ~2KB → outputs.md ~5KB → categories.md ~7KB → structs.md ~15KB). For complex prompts, may hit limits.

2. **Redundant Content** — Previous stages' content is duplicated in each stage's output, wasting disk space and I/O bandwidth.

3. **Parsing Overhead** — Each stage must parse accumulated markdown, adding processing overhead.

4. **Merge Conflicts** — If multiple stages modify same content (e.g., SW2 and SW3 both add to task descriptions), merge conflicts can occur.

Despite these drawbacks, the accumulation pattern is retained for this project because the benefits (traceability, debugging, iteration) outweigh the costs, and the file sizes remain manageable for typical prompts (≤ 20KB final structs.md).

## Hook Integration Points

The hook system is woven throughout the sub-workflows at specific integration points that enable quality tracking, iteration loops, and state management.

### Trigger Firing Points in Runner

The benchmark runner at `src/benchmark/runner.rs` fires hooks at specific line numbers:

| Trigger | Runner Line | Firing Condition | Context Struct |
|---------|-------------|------------------|----------------|
| before_step_starts | 1257 | Before step execution begins | BeforeStepStartsContext |
| after_step_starts | 1286 | After step execution begins (rarely used) | AfterStepStartsContext |
| after_step_fails | 1318 | When step execution fails | AfterStepFailsContext |
| after_all_retries_exhausted | 1355 | When all retry attempts fail | AfterAllRetriesExhaustedContext |
| after_step_succeeds | 1340 | When step execution succeeds | AfterStepSucceedsContext |
| on_requires_failed | 1535 | When step dependencies fail | OnRequiresFailedContext |
| after_loop_iteration_fails | 1592 | When loop iteration fails | AfterLoopIterationFailsContext |

### Hook Execution Flow

The hook execution flow in the runner follows this path:

1. **Trigger Point Reached** — Runner reaches line where trigger fires (e.g., line 1340 for after_step_succeeds).

2. **Context Construction** — Runner builds context struct with relevant data:
   - For `after_step_succeeds`: constructs `AfterStepSucceedsContext` with token_count, output, duration_ms
   - For `after_step_fails`: constructs `AfterStepFailsContext` with error_message, error_type, error.is_retryable

3. **Hook Lookup** — Runner retrieves hook configuration from workflow YAML:
   - Check step-level `when:` hooks
   - Check workflow-level default hooks (merged with step-level)

4. **Action Dispatch** — Runner calls `execute_action()` for each action in hook configuration:
   - Action dispatcher at `src/workflow/hooks/actions.rs`
   - Routes to specific execute_* function based on action type

5. **Action Execution** — Each execute_* function performs its operation:
   - `execute_log()` writes to file or stdout
   - `execute_save_to()` saves to file or bookmark
   - `execute_shell()` runs shell command
   - `execute_gwt()` evaluates GWT expression, routes based on result

6. **Result Merging** — Hook results from multiple actions merged via `HookResult::merge()`:
   - Priority order: Fail > SkipRemaining > RouteTo > SkipLoop > SkipStep > Continue
   - Highest priority result determines workflow control flow

7. **Control Flow** — Workflow continues, routes, skips, or fails based on merged result.

### Hook Chains in Sub-Workflows

Each sub-workflow uses hook chains to enable iteration loops and quality gates:

**SW1 Hook Chain:**

```yaml
when:
  before_step_starts:
    - log:
        to_file_path: "./workspace/logs/sw1-start.log"
        level: info
        message: "SW1: Task Deconstruction started"
    - bookmark:
        flag: true
        path: "./workspace/checkpoints/sw1-start.md"
        name: "sw1_start_checkpoint"

  after_step_succeeds:
    - save_to:
        - tasks_md_output
        - "./workspace/sw1/tasks.md"
    - log:
        to_file_path: "./workspace/logs/sw1-complete.log"
        level: info
        event_fields: [step_name, duration_ms, task_count, avg_complexity]
        message: "SW1: Task Deconstruction complete, {{step.task_count}} tasks, {{step.avg_complexity}} avg complexity"
    - gwt:
        - given: "task_count > 0 && avg_complexity <= 5"
          when: "Task deconstruction meets quality gates"
          then: { route_to: success_report }
        - given: "task_count == 0"
          when: "No tasks generated"
          then: { route_to: fix_no_tasks }
        - given: "avg_complexity > 5"
          when: "Average complexity too high, need more decomposition"
          then: { route_to: fix_decomposition }

  after_step_fails:
    - log:
        to_file_path: "./workspace/logs/sw1-fail.log"
        level: error
        event_fields: [error_message, error_type]
        message: "SW1: Task Deconstruction failed: {{error.message}}"
    - save_to:
        - "./workspace/sw1/failure-evidence.md"
    - route_to: fix_task_decomposition
```

**SW5 Hook Chain (most complex):**

```yaml
when:
  before_step_starts:
    - shell:
        command: "cat {{inputs.output_dir}}/sw4/structs.md | wc -w"
        bookmark_as: "structs_word_count"
    - log:
        to_file_path: "./workspace/logs/sw5-start.log"
        level: info
        message: "SW5: Workflow Assembly started, {{bookmarks.structs_word_count}} words in structs.md"
    - bookmark:
        flag: true
        path: "./workspace/checkpoints/sw5-start.md"
        name: "sw5_start_checkpoint"

  after_step_succeeds:
    - shell:
        command: "scripts/fix-generated-yaml.py --input {{inputs.output_dir}}/sw5/workflow.yml"
        fail_on_error: true
    - shell:
        command: "scripts/validate-yaml.py --schema docs/schema/unified-workflow-schema.yml --input {{inputs.output_dir}}/sw5/workflow.yml"
        fail_on_error: true
    - gwt:
        - given: "schema_valid == true && tasks_in_yaml == tasks_in_md"
          when: "Workflow assembled correctly and all tasks present"
          then: { route_to: success_report }
        - given: "schema_valid == false"
          when: "Schema validation failed"
          then: { route_to: fix_schema_validation }
        - given: "tasks_in_yaml < tasks_in_md"
          when: "Some tasks missing from workflow"
          then: { route_to: add_missing_tasks }
    - save_to:
        - final_workflow_yaml
        - "./workspace/final-workflows/{{inputs.run_id}}-workflow.yml"
        - "./workspace/sw5/workflow.yml"
    - log:
        to_file_path: "./workspace/logs/sw5-complete.log"
        level: info
        event_fields: [step_name, duration_ms, schema_valid, task_count]
        message: "SW5: Workflow Assembly complete, schema_valid={{schema_valid}}, task_count={{task_count}}"
    - bookmark:
        flag: true
        path: "./workspace/checkpoints/sw5-complete.md"
        name: "sw5_complete_checkpoint"

  after_step_fails:
    - log:
        to_file_path: "./workspace/logs/sw5-fail.log"
        level: error
        event_fields: [error_message, error_type, validation_errors]
        message: "SW5: Workflow Assembly failed: {{error.message}}, validation errors: {{validation_errors}}"
    - shell:
        command: "cp {{inputs.output_dir}}/sw5/workflow.yml {{inputs.output_dir}}/sw5/workflow-failed-{{now}}.yml"
        fail_on_error: false
    - route_to: fix_workflow_assembly

  after_all_retries_exhausted:
    - log:
        to_file_path: "./workspace/logs/sw5-catastrophic-fail.log"
        level: critical
        message: "SW5: Workflow Assembly failed after all retries, aborting pipeline"
    - fail: "SW5 Final Workflow Assembly failed after 5 iterations, pipeline incomplete"
```

### Hook Result Priority System

The hook result priority system at `src/workflow/hooks/mod.rs` defines merge logic:

```rust
pub fn merge(self, other: HookResult) -> HookResult {
    // Priority: Fail > SkipRemaining > RouteTo > SkipLoop > SkipStep > Continue
    match (self, other) {
        (HookResult::Fail(_), _) | (_, HookResult::Fail(_)) => {
            // Fail always wins
            if let HookResult::Fail(reason) = self { HookResult::Fail(reason) } 
            else { other }
        }
        (HookResult::SkipRemaining(_), _) | (_, HookResult::SkipRemaining(_)) => {
            // SkipRemaining wins over everything except Fail
            if let HookResult::SkipRemaining(reason) = self { HookResult::SkipRemaining(reason) }
            else { other }
        }
        (HookResult::RouteTo(_), _) | (_, HookResult::RouteTo(_)) => {
            // RouteTo wins over SkipLoop, SkipStep, Continue
            if let HookResult::RouteTo(targets) = self { HookResult::RouteTo(targets) }
            else { other }
        }
        // ... similar logic for SkipLoop, SkipStep, Continue
        (HookResult::Continue, HookResult::Continue) => HookResult::Continue
    }
}
```

This priority system enables complex hook chains where multiple actions fire on the same trigger, with the highest-priority result determining workflow control flow.

## Shell-Based Orchestration Pattern

The orchestrator uses shell-based orchestration to chain sub-workflows together. This pattern has specific architectural implications and implementation details.

### Shell Invocation in before_step_starts Hooks

The orchestrator invokes sub-workflows via shell actions in `before_step_starts` hooks:

```yaml
run_sw1:
  when:
    before_step_starts:
      - shell:
          command: "./target/release/whitt benchmark --workflow workflows/sw1-task-deconstruction.yml --output-dir {{inputs.output_dir}}/sw1"
          fail_on_error: true
          timeout_seconds: 600
          working_directory: "/home/jon/code/whitt-execution-engine"
```

**Why before_step_starts Instead of Tool Step:**

1. **Tool step requires tool registry** — Tool steps are for registered tools (grep, file_read, etc.), not for executing workflows.
2. **before_step_starts fires before LLM call** — Allows sub-workflow to complete before LLM generates, enabling LLM to use sub-workflow output in prompt.
3. **Shell action is purpose-built for commands** — Designed for shell execution with proper error handling and timeout.

### Shell Command Composition

Shell commands in orchestrator follow this composition pattern:

```
<whitt_binary_path> benchmark --workflow <workflow_path> --output-dir <output_dir>
```

Components:

1. **whitt_binary_path** — `./target/release/whitt` (or absolute path if needed)
2. **benchmark** — Benchmark mode (preferred over workflow mode for better logging)
3. **--workflow** — Path to sub-workflow YAML file
4. **--output-dir** — Directory for sub-workflow outputs (logs, artifacts)

**Additional Optional Flags:**

- `--load-timeout 1800` — 30-minute model load timeout
- `--no-cache-prompt` — Required for Vulkan backend
- `--cont-batching false` — Explicitly disable (Vulkan requirement)
- `--model <model_name>` — Override model (not used here, always qwen35)

### Shell Error Handling

Shell actions implement comprehensive error handling:

```yaml
- shell:
    command: "./target/release/whitt benchmark --workflow workflows/sw1-task-deconstruction.yml --output-dir {{inputs.output_dir}}/sw1"
    fail_on_error: true
    timeout_seconds: 600
    bookmark_as: "sw1_output"
```

**Error Handling Flow:**

1. **Command Execution** — Shell command runs in subprocess with timeout monitoring.
2. **Exit Code Check** — If exit code ≠ 0, action fails (if fail_on_error: true).
3. **Timeout Check** — If timeout exceeded, command killed, action fails.
4. **Output Capture** — stdout and stderr captured for logging.
5. **Bookmark Storage** — If successful, output stored in bookmark (if bookmark_as provided).

**Failure Scenarios:**

- **Exit code 0** — Success, continue workflow
- **Exit code non-zero** — Failure, if fail_on_error: true then workflow fails
- **Timeout** — Failure, command killed, workflow fails if fail_on_error: true
- **Command not found** — Failure, whitt binary path incorrect, workflow fails

### Run ID Substitution Implementation

Run ID substitution uses `sed` pattern to inject run IDs into sub-workflow paths:

```yaml
- shell:
    command: |
      sed 's/__RUN_ID__/{{inputs.run_id}}/g' workflows/sw1-task-deconstruction.yml > /tmp/sw1-{{inputs.run_id}}.yml
      ./target/release/whitt benchmark --workflow /tmp/sw1-{{inputs.run_id}}.yml --output-dir ./workspace/meta-output/{{inputs.run_id}}/sw1
      rm /tmp/sw1-{{inputs.run_id}}.yml
    fail_on_error: true
```

**Components:**

1. **sed substitution** — Replaces `__RUN_ID__` placeholder with actual run ID
2. **Temp file creation** — Creates temporary YAML with run ID injected
3. **Workflow execution** — Runs benchmark with temp file
4. **Cleanup** — Removes temp file (optional, for disk efficiency)

**Why sed Instead of Template Interpolation:**

Template interpolation (`{{inputs.run_id}}`) doesn't work in sub-workflow YAMLs because sub-workflows are standalone files loaded from disk. Sed substitution provides deterministic, explicit run ID injection at orchestration time.

### Shell Hook Output Validation

The orchestrator validates sub-workflow outputs via shell commands in `after_step_succeeds` hooks:

```yaml
run_sw1:
  when:
    after_step_succeeds:
      - shell:
          command: "if [ ! -f {{inputs.output_dir}}/sw1/tasks.md ]; then echo 'ERROR: SW1 did not produce tasks.md' && exit 1; fi"
          fail_on_error: true
      - log:
          to_file_path: "{{inputs.output_dir}}/orchestrator.log"
          level: info
          message: "SW1 complete, tasks.md validated"
```

**Validation Checks:**

1. **File existence** — Verify output file exists at expected path
2. **File non-empty** — Verify file has content (not 0 bytes)
3. **File readable** — Verify file can be read (permission check)
4. **Format validation** — Verify file format (markdown, YAML)

**If Validation Fails:**

- Shell command exits with non-zero code
- Action fails (fail_on_error: true)
- Workflow fails (no subsequent steps execute)
- Error logged in orchestrator.log

---

**Document Status:** Draft  
**Last Updated:** 2026-06-17  
**Author:** Sisyphus-Junior (Whitt Execution Engine Planning)  
**Review Status:** Ready for Execution