# 03 — Sub-Workflow Specifications

> **Status:** Planning Phase — Documentation Only  
> **Phase:** Meta-Workflow Generator Development (Qwen 3.5-9B)  
> **Focus:** I/O contracts for SW1-SW5, hook configs, evaluation criteria

## SW1: Task Deconstruction Specification

**File:** `workflows/sw1-task-deconstruction.yml`

**Purpose:** Break high-complexity prompts into atomic tasks with story-point complexity scoring.

**Input Format:** Natural language prompt (via workflow input variable or shell cat from file)

**Output Format:** Markdown file (`tasks.md`) with hierarchical task tree structure

### Input Specification

**Prompt Input via Workflow Variable:**

```yaml
agentic_workflow:
  inputs:
    prompt_text:
      type: string
      description: "Raw prompt text (natural language task description)"
```

**Prompt Input via File (Alternative):**

```yaml
agentic_workflow:
  steps:
    load_prompt:
      tool: file_read
      input:
        file_path: "./workspace/prompts/prompt-001.md"
      when:
        after_step_succeeds:
          - bookmark: prompt_content
```

**Prompt Content Characteristics:**

- Length: 500-5000 characters (short task descriptions to complex multi-page specifications)
- Format: Natural language, may include technical jargon, file paths, tool names
- Complexity: Low (1-3 actions) to high (8+ actions with dependencies)
- Structure: May be single paragraph or multi-paragraph with bullet points

### Output Specification

**tasks.md File Structure:**

```markdown
# Task Deconstruction for Prompt: <prompt_summary>

## Root Tasks

### 1. <Task Name>
- **Description:** <Task description>
- **Story Points:** <1-13>
- **Subtasks:** 
  - 1.1 <Subtask Name> (<Story Points>)
  - 1.2 <Subtask Name> (<Story Points>)
  - ...

### 2. <Task Name>
- **Description:** <Task description>
- **Story Points:** <1-13>
- **Subtasks:** None (atomic)

...

## Task Statistics
- Total tasks: <count>
- Atomic tasks: <count>
- Composite tasks: <count>
- Max story points: <value>
- Avg story points: <value>
- Max nesting depth: <value>
```

**Output Requirements:**

1. **Hierarchical Structure** — Use numbered outline format (1, 1.1, 1.1.1) for nesting
2. **Task Names** — Verb-noun pattern (e.g., "read configuration files", not "config")
3. **Descriptions** — Clear, specific, actionable (no vague language)
4. **Story Points** — 1-13 scale based on effort estimation (see complexity scoring rules)
5. **Subtasks** — Present only if task is composite (story points ≥ 5)
6. **Statistics Section** — Summary of task breakdown at end of file

### Internal Processing Steps

**Step 1: Initial Task Breakdown**

**Purpose:** Break prompt into candidate atomic tasks.

**Input:** Raw prompt text.

**Processing:**

1. LLM call with prompt template emphasizing task decomposition:
   ```
   Break the following prompt into atomic tasks. Each task should:
   - Be independent (minimal dependencies on other tasks)
   - Be cohesive (accomplish single objective)
   - Be manageable (can be completed in reasonable time)
   - Follow verb-noun naming pattern (e.g., "read files", not "file reading")
   
   Output tasks in numbered list format. Aim for 3-7 root tasks.
   
   Prompt: {{inputs.prompt_text}}
   ```

2. Model configuration: temperature 0.1 (deterministic), top_p 0.8 (conservative), max_tokens 2048

3. Output: Initial task list (3-7 candidate tasks)

**Step 2: Complexity Scoring Loop**

**Purpose:** Score each task's complexity in story points.

**Input:** Initial task list from Step 1.

**Processing:**

1. For each task (or group of ~3 tasks for efficiency):
   
2. LLM call to expand task into agentic subtasks:
   ```
   For task "<task_name>":
   
   Expand this task into a detailed list of subtasks that would be needed to complete it.
   Each subtask should:
   - Be a concrete action (read, write, transform, validate, etc.)
   - Have a clear completion criterion
   - Represent meaningful work (not trivial)
   
   Then, assign a story point score (1-13) based on:
   - Number of subtasks (more subtasks = higher score)
   - Complexity of subtasks (file operations = 1-2 pts, code generation = 3-5 pts, system configuration = 5-8 pts)
   - Dependencies (many dependencies = +1-2 pts)
   - Risk/uncertainty (high risk = +1-2 pts)
   
   Story point scale reference:
   - 1 point: Trivial (file read, simple transform)
   - 2-3 points: Simple (few file operations, basic validation)
   - 5-8 points: Medium (code generation, moderate complexity)
   - 10-13 points: Complex (multiple components, high uncertainty)
   
   Output format:
   Task: <task_name>
   Subtasks:
   - <subtask1>
   - <subtask2>
   ...
   Story Points: <score>
   Reasoning: <explanation>
   ```

3. Model configuration: temperature 0.2 (balanced), top_p 0.9 (default), max_tokens 4096

4. Output: Task list with story point scores

**Step 3: Decomposition Loop**

**Purpose:** Ensure all tasks are atomic (< 5 story points).

**Input:** Task list with story point scores from Step 2.

**Processing:**

1. Identify tasks scoring ≥ 5 story points (complex tasks that need decomposition)

2. For each complex task:
   
3. LLM call to decompose into subtasks:
   ```
   Task "<task_name>" has complexity score of <score> story points, which exceeds the atomic threshold of 5 points.
   
   Decompose this task into 2-4 subtasks, each of which:
   - Is atomic (≤ 5 story points)
   - Together accomplish the original task's objective
   - Have clear dependencies (subtask1 must complete before subtask2)
   
   Assign story point scores to each subtask using the same scoring logic.
   
   Output format:
   Original Task: <task_name> (<score> points)
   Decomposition:
   - 1. <subtask1_name> (<subtask1_score> points)
     - Description: <subtask1_description>
     - Dependencies: None (or subtaskX)
   - 2. <subtask2_name> (<subtask2_score> points)
     - Description: <subtask2_description>
     - Dependencies: subtask1
   ...
   ```

4. Model configuration: temperature 0.2 (balanced), top_p 0.9 (default), max_tokens 3072

5. Validation: Check that all new subtasks are < 5 points. If any subtask ≥ 5 points, repeat decomposition for that subtask.

6. Max decomposition depth: 3 levels (root → subtask → sub-subtask). If depth exceeded, mark task as "complex atomic" and proceed.

7. Output: Updated task list with complex tasks decomposed into atomic subtasks

**Step 4: GWT Quality Gate**

**Purpose:** Ensure tasks meet quality criteria before final assembly.

**Input:** Task list with atomic tasks and story point scores.

**Processing:**

1. GWT evaluation for each task (or group of tasks):

```yaml
when:
  after_step_succeeds:
    - gwt:
        # Gate 1: Atomicity
        - given: "task.is_atomic == true"
          when: "Task is atomic (no subtasks or all subtasks atomic)"
          then: { route_to: validate_naming }
        - given: "task.is_atomic == false"
          when: "Task is not atomic (has subtasks with further subtasks)"
          then: { route_to: decompose_further }
        
        # Gate 2: Well-named
        - given: "task.name follows verb-noun pattern"
          when: "Task name follows verb-noun pattern (e.g., 'read files')"
          then: { route_to: validate_verbosity }
        - given: "task.name does not follow verb-noun pattern"
          when: "Task name does not follow verb-noun pattern (e.g., 'config')"
          then: { route_to: rename_task }
        
        # Gate 3: Not overly verbose
        - given: "task.name_word_count <= 15"
          when: "Task name is concise (≤ 15 words)"
          then: { route_to: validate_principles }
        - given: "task.name_word_count > 15"
          when: "Task name is verbose (> 15 words)"
          then: { route_to: shorten_name }
        
        # Gate 4: Quality engineering principles
        - given: "task.follows_engineering_principles == true"
          when: "Task follows quality engineering principles"
          then: { route_to: validate_gwt_criteria }
        - given: "task.follows_engineering_principles == false"
          when: "Task violates engineering principles (too broad, too narrow, ambiguous)"
          then: { route_to: refactor_task }
        
        # Gate 5: GWT criteria flushed out
        - given: "task.has_gwt_criteria == true"
          when: "Task has clear acceptance criteria in GWT format"
          then: { route_to: accept_task }
        - given: "task.has_gwt_criteria == false"
          when: "Task lacks clear acceptance criteria"
          then: { route_to: add_gwt_criteria }
```

2. Quality Criteria Definitions:

   - **Atomicity**: Task accomplishes single cohesive objective, no internal decomposition needed
   - **Well-named**: Follows verb-noun pattern, clear scope, no ambiguous terminology
   - **Not overly verbose**: ≤ 15 words per task name, concise without sacrificing clarity
   - **Quality engineering**: Follows task decomposition best practices (INVEST criteria: Independent, Negotiable, Valuable, Estimable, Small, Testable)
   - **GWT criteria flushed out**: Each task has clear acceptance criteria in Given-When-Then format

3. Model configuration: temperature 0.1 (deterministic), top_p 0.8 (conservative), max_tokens 2048

4. Output: Validated task list meeting all quality criteria

**Step 5: Final Assembly**

**Purpose:** Assemble final hierarchical task tree with statistics.

**Input:** Validated task list from Step 4.

**Processing:**

1. Assemble hierarchical structure using numbered outline format

2. Calculate statistics:
   - Total tasks (all tasks at all levels)
   - Atomic tasks (tasks with no subtasks)
   - Composite tasks (tasks with subtasks)
   - Max story points (highest score across all tasks)
   - Avg story points (sum of scores / count of tasks)
   - Max nesting depth (deepest level in hierarchy)

3. LLM call to format final output:
   ```
   Format the validated task list into a hierarchical markdown structure.
   
   Structure:
   # Task Deconstruction for Prompt: <prompt_summary>
   
   ## Root Tasks
   
   ### 1. <Task Name>
   - **Description:** <Task description>
   - **Story Points:** <score>
   - **Subtasks:** (if composite)
     - 1.1 <Subtask Name> (<score>)
     - 1.2 <Subtask Name> (<score>)
   
   ### 2. <Task Name>
   ...
   
   ## Task Statistics
   - Total tasks: <count>
   - Atomic tasks: <count>
   - Composite tasks: <count>
   - Max story points: <value>
   - Avg story points: <value>
   - Max nesting depth: <value>
   
   Validated tasks:
   {{validated_tasks_list}}
   ```

4. Model configuration: temperature 0.05 (very deterministic), top_p 0.7 (conservative), max_tokens 2048

5. Output: Final `tasks.md` file with hierarchical structure and statistics

### Evaluation Criteria

**SW1 Pass Conditions:**

1. **100% Input Coverage** — All tasks from original prompt represented in output
2. **Atomicity Compliance** — All atomic tasks have story points ≤ 5
3. **Naming Quality** — All task names follow verb-noun pattern
4. **Conciseness** — All task names ≤ 15 words
5. **Engineering Principles** — All tasks follow INVEST criteria
6. **GWT Criteria** — All tasks have clear acceptance criteria
7. **Statistics Completeness** — All statistics fields present and accurate

**SW1 Fail Conditions (any of these triggers iteration):**

1. **Missing Tasks** — Some tasks from prompt not represented in output
2. **Overly Coarse Tasks** — Some atomic tasks have story points > 5
3. **Poor Naming** — Some task names don't follow verb-noun pattern
4. **Verbose Names** — Some task names > 15 words
5. **Violates Principles** — Some tasks violate engineering principles
6. **Missing Criteria** — Some tasks lack acceptance criteria
7. **Incomplete Statistics** — Some statistics fields missing or inaccurate

### Fix Loops

**Fix Loop 1: Task Coverage**

**Trigger:** Missing tasks from prompt

**Fix Strategy:**
1. LLM call to identify missing tasks:
   ```
   Compare the original prompt to the generated task list.
   Identify any tasks mentioned in the prompt that are missing from the task list.
   For each missing task, explain why it was missed.
   
   Original prompt: {{original_prompt}}
   Generated tasks: {{generated_tasks}}
   ```

2. Add missing tasks to task list

3. Re-run complexity scoring for new tasks

**Fix Loop 2: Decomposition**

**Trigger:** Atomic tasks with story points > 5

**Fix Strategy:**
1. LLM call to decompose over-coarse tasks:
   ```
   The following tasks have story points > 5, exceeding the atomic threshold:
   {{over_coarse_tasks}}
   
   For each task, decompose into 2-4 atomic subtasks (each ≤ 5 points).
   Ensure subtasks together accomplish the original task's objective.
   ```

2. Replace over-coarse tasks with decomposed subtasks

3. Re-evaluate story points for new subtasks

**Fix Loop 3: Naming**

**Trigger:** Task names don't follow verb-noun pattern

**Fix Strategy:**
1. LLM call to rename tasks:
   ```
   The following task names do not follow the verb-noun pattern:
   {{poorly_named_tasks}}
   
   For each task, rename it to follow the verb-noun pattern while preserving its meaning.
   Examples: "config" → "read configuration files", "implementation" → "implement feature"
   ```

2. Replace poorly named tasks with improved names

3. Validate new names follow pattern

**Fix Loop 4: GWT Criteria**

**Trigger:** Tasks lack clear acceptance criteria

**Fix Strategy:**
1. LLM call to add GWT criteria:
   ```
   The following tasks lack clear acceptance criteria in GWT format:
   {{tasks_without_criteria}}
   
   For each task, add acceptance criteria in Given-When-Then format:
   - Given: Preconditions or context
   - When: Action or condition being tested
   - Then: Expected outcome or observable state
   ```

2. Add GWT criteria to tasks

3. Validate criteria completeness

### Chunk Sizing Strategy

SW1 uses dynamic chunking based on prompt complexity to manage model context (262144 tokens):

**Chunk Size Determination:**

```
If prompt_word_count <= 500:
    chunk_size = 7  # Short prompt, can process all tasks in one pass
Else if prompt_word_count <= 2000:
    chunk_size = 5  # Medium prompt, process 5 tasks at a time
Else if prompt_word_count <= 5000:
    chunk_size = 3  # Long prompt, process 3 tasks at a time
Else:
    chunk_size = 2  # Very long prompt, process 2 tasks at a time
```

**Chunk Growth Logic:**

- Start with initial chunk size based on prompt length
- If quality gates pass on all chunks in iteration 1, increase chunk_size by 1
- If quality gates fail on any chunk, decrease chunk_size by 1 (min 2)
- Maximum chunk_size: 7 (prevents context overflow)
- Minimum chunk_size: 2 (ensures focused iteration)

**Chunk Ordering:**

Order tasks by estimated complexity (high complexity first) to:
- Ensure most difficult tasks get model's full attention early
- Enable focused iteration on complex tasks if needed
- Reduce risk of running out of context before addressing hard tasks

### Model Configuration Reference

**LoadParams for SW1:**

```yaml
load_params:
  context_size: 262144          # Full context window
  batch_size: 2048               # Default batch size
  ubatch_size: 512               # Default micro-batch size
  cache_type_k: "q8_0"           # Q8_0 KV cache K
  cache_type_v: "q8_0"           # Q8_0 KV cache V
  gpu_layers: 99                  # full GPU offload (benchmark-proven fastest on RX580)
  threads: 5                     # 5 CPU threads
  use_mmap: true                 # Memory-mapped file loading
  flash_attn: true               # Flash attention (safe for Vulkan)
  cont_batching: false            # Continuous batching disabled (Vulkan requirement)
  no_cache_prompt: true           # Prompt caching disabled (Vulkan requirement)
  parallel: 1                    # Single parallel slot
```

**Sampling Parameters for SW1:**

```yaml
sampling:
  temperature: 0.1               # Very low for deterministic task names
  top_p: 0.8                     # Conservative token selection
  top_k: 40                      # Default top-k sampling
  max_tokens: 2048               # Half of default, prevents over-generation
  repeat_penalty: 1.0            # No repetition penalty (default)
  seed: null                      # Random seed (default)
```

**Why This Configuration:**

- Low temperature (0.1) ensures consistent task names across iterations
- Conservative top_p (0.8) reduces creative deviations from task decomposition patterns
- Reduced max_tokens (2048) prevents over-generation of tasks, maintains focus
- Default context (262144) enables processing of long prompts without truncation

### Complexity Scoring Rules

**Story Point Scale (1-13):**

**1 Point (Trivial):**
- File read or write (single file)
- Simple string transformation (trim, uppercase)
- Basic validation (null check, existence check)
- Example: "Read configuration file", "Trim whitespace from string"

**2-3 Points (Simple):**
- Multiple file operations (2-5 files)
- Basic data transformation (parse YAML to object)
- Simple validation (schema validation, type checking)
- Example: "Read and parse 5 configuration files", "Validate JSON schema"

**5-8 Points (Medium):**
- Code generation (function, class, module)
- Moderate complexity transformations (aggregate data, apply business logic)
- System configuration (setup tool, configure service)
- Example: "Implement REST API endpoints", "Configure Docker compose for multi-service app"

**10-13 Points (Complex):**
- Complex code generation (entire subsystem, integration layer)
- High-complexity transformations (multi-source data integration, complex algorithms)
- System-level configuration (setup entire infrastructure, configure distributed system)
- Example: "Implement microservices architecture with service mesh", "Configure Kubernetes cluster with ingress, monitoring, logging"

**Scoring Algorithm:**

```
For each task:
    subtask_count = number of subtasks in task decomposition
    base_score = subtask_count
    
    for each subtask:
        if subtask_type == "file_operation":
            subtask_score = 1
        elif subtask_type == "simple_transform":
            subtask_score = 2
        elif subtask_type == "code_generation":
            subtask_score = 4
        elif subtask_type == "system_config":
            subtask_score = 6
        
        if subtask has_many_dependencies:
            subtask_score += 1
        if subtask has_high_risk:
            subtask_score += 1
    
    task_score = average(subtask_scores)
    
    if task_score > 13:
        task_score = 13  # Cap at 13
```

**Why This Scoring:**

- Story points reflect actual human effort (not arbitrary)
- Aligns with agile estimation practices
- Provides clear decomposition threshold (≥ 5 points = must decompose)
- Cap at 13 prevents runaway complexity scores

## SW2: Desired Output State Specification

**File:** `workflows/sw2-desired-output-state.yml`

**Purpose:** Define testable end-state criteria for each task from SW1.

**Input Format:** Markdown file (`tasks.md`) from SW1

**Output Format:** Markdown file (`outputs.md`) with testable desired state per task

### Input Specification

**tasks.md File Structure:**

```markdown
# Task Deconstruction for Prompt: <prompt_summary>

## Root Tasks

### 1. <Task Name>
- **Description:** <Task description>
- **Story Points:** <score>
- **Subtasks:**
  - 1.1 <Subtask Name> (<score>)
  - 1.2 <Subtask Name> (<score>)
  - ...

### 2. <Task Name>
...

## Task Statistics
...
```

**Input Requirements:**

1. File must exist at expected path
2. File must be valid markdown (parseable)
3. File must contain hierarchical task tree
4. Each task must have: name, description, story points
5. File must have statistics section

### Output Specification

**outputs.md File Structure:**

```markdown
# Task Deconstruction for Prompt: <prompt_summary>

[... tasks.md content included ...]

# Desired Output States

## Task 1: <Task Name>

### Desired End-State
<Description of what observable output state proves this task is complete>

### Testable Criteria
- **Given:** <Preconditions or context>
  **When:** <Action or condition being tested>
  **Then:** <Expected outcome or observable state>

- **Given:** <Preconditions or context>
  **When:** <Action or condition being tested>
  **Then:** <Expected outcome or observable state>

## Task 2: <Task Name>

### Desired End-State
...

### Testable Criteria
...

## Criteria Statistics
- Total tasks: <count>
- Total criteria: <count>
- Avg criteria per task: <value>
- Criteria distribution: <breakdown>
```

**Output Requirements:**

1. **Include tasks.md Content** — Output starts with full tasks.md content (accumulation pattern)
2. **One Section Per Task** — Each task from tasks.md has corresponding desired state section
3. **Desired End-State Description** — 2-4 sentences describing observable completion state
4. **Testable Criteria in GWT Format** — 1-3 criteria per task in Given-When-Then format
5. **Criteria Statistics** — Summary of criteria count and distribution at end of file

### Internal Processing Steps

**Step 1: Load Task Tree**

**Purpose:** Read tasks.md and extract atomic tasks.

**Input:** tasks.md file path.

**Processing:**

1. Shell action in `before_step_starts` hook to read tasks.md:
```yaml
when:
  before_step_starts:
    - shell:
        command: "cat {{inputs.tasks_path}}"
        bookmark_as: "tasks_md_content"
        fail_on_error: true
    - log:
        to_file_path: "./workspace/logs/sw2-start.log"
        level: info
        message: "Loaded tasks.md, {{bookmarks.tasks_md_content.word_count}} words"
```

2. Parse tasks.md to extract atomic tasks (leaf nodes in hierarchy)
   - Tasks with no subtasks are atomic
   - Tasks with subtasks are composite (process subtasks instead)

3. LLM call to extract atomic tasks list:
   ```
   From the following task hierarchy, extract all atomic tasks (tasks with no subtasks).
   For each atomic task, provide: task number, name, description.
   
   Task hierarchy:
   {{bookmarks.tasks_md_content}}
   
   Output format:
   - Task X: <task_name> - <task_description>
   - Task Y: <task_name> - <task_description>
   ...
   ```

4. Model configuration: temperature 0.2 (balanced), top_p 0.9 (default), max_tokens 2048

5. Output: List of atomic tasks with names and descriptions

**Step 2: Chunking Strategy**

**Purpose:** Group atomic tasks into chunks for efficient LLM processing.

**Input:** List of atomic tasks from Step 1.

**Processing:**

1. Determine chunk size based on task count and complexity:
```
If atomic_task_count <= 5:
    chunk_size = 5  # Small task set, process all in one chunk
Else if atomic_task_count <= 10:
    chunk_size = 4  # Medium task set, process 4 at a time
Else if atomic_task_count <= 15:
    chunk_size = 3  # Large task set, process 3 at a time
Else:
    chunk_size = 2  # Very large task set, process 2 at a time
```

2. Group atomic tasks into chunks by complexity:
   - Group low-complexity tasks together (1-2 story points)
   - Group medium-complexity tasks together (3-4 story points)
   - Group high-complexity tasks together (5+ story points)
   - This enables focused prompt templates per complexity group

3. Output: List of chunks, each containing 2-5 atomic tasks

**Step 3: Desired State Generation Loop**

**Purpose:** Generate desired end-state for each task in chunk.

**Input:** Chunk of atomic tasks (2-5 tasks).

**Processing:**

1. For each chunk:

2. LLM call to generate desired states:
   ```
   For each of the following tasks, define the "desired end-state" — what observable output state proves this task is complete?
   
   Each desired end-state should:
   - Be specific (observable, measurable)
   - Be comprehensive (cover all task requirements)
   - Be actionable (can be verified via shell or tool)
   - Not over-scope (match task scope exactly)
   
   Tasks in this chunk:
   {{chunk_tasks}}
   
   Output format for each task:
   Task: <task_name>
   Desired End-State: <2-4 sentences describing observable completion state>
   ```

3. Model configuration: temperature 0.15 (balanced), top_p 0.85 (default), max_tokens 3072

4. Output: Desired end-state for each task in chunk

**Step 4: GWT Criteria Generation Loop**

**Purpose:** Generate testable criteria in GWT format for each task.

**Input:** Desired end-states from Step 3.

**Processing:**

1. For each task:

2. LLM call to generate GWT criteria:
   ```
   For task "<task_name>" with desired end-state "<desired_end_state>", generate 1-3 testable criteria in Given-When-Then format.
   
   Each criterion should:
   - Start with "Given:" (preconditions or context)
   - Include "When:" (action or condition being tested)
   - End with "Then:" (expected outcome or observable state)
   - Be testable (can be verified via shell command, file check, or tool output)
   - Be unambiguous (clear pass/fail condition)
   
   Task: <task_name>
   Desired End-State: <desired_end_state>
   
   Output format:
   Criteria 1:
   - Given: <preconditions>
     When: <action>
     Then: <expected outcome>
   
   Criteria 2:
   - Given: <preconditions>
     When: <action>
     Then: <expected outcome>
   ```

3. Model configuration: temperature 0.15 (balanced), top_p 0.85 (default), max_tokens 2048

4. Validation: Check that each criterion is testable, unambiguous, and matches task scope

5. Output: 1-3 GWT criteria per task

**Step 5: GWT Quality Gate per Chunk**

**Purpose:** Ensure desired states and criteria meet quality standards.

**Input:** Desired states + GWT criteria for chunk.

**Processing:**

1. GWT evaluation for each task in chunk:

```yaml
when:
  after_step_succeeds:
    - gwt:
        # Gate 1: Testable
        - given: "criteria.is_testable == true"
          when: "All criteria are testable (observable, measurable)"
          then: { route_to: validate_unambiguous }
        - given: "criteria.is_testable == false"
          when: "Some criteria are not testable (subjective, unobservable)"
          then: { route_to: fix_testability }
        
        # Gate 2: Unambiguous
        - given: "criteria.is_unambiguous == true"
          when: "All criteria are unambiguous (clear pass/fail conditions)"
          then: { route_to: validate_scope }
        - given: "criteria.is_unambiguous == false"
          when: "Some criteria are ambiguous (unclear conditions, subjective judgments)"
          then: { route_to: fix_ambiguity }
        
        # Gate 3: Tied to specific task
        - given: "criteria.task_reference == true"
          when: "All criteria reference specific task by number/name"
          then: { route_to: validate_not_overscoped }
        - given: "criteria.task_reference == false"
          when: "Some criteria don't reference specific task"
          then: { route_to: add_task_reference }
        
        # Gate 4: Not over-scoped
        - given: "criteria.scope_matches_task == true"
          when: "All criteria match task scope (don't exceed boundaries)"
          then: { route_to: accept_chunk }
        - given: "criteria.scope_matches_task == false"
          when: "Some criteria exceed task scope (over-constrained)"
          then: { route_to: fix_scope }
```

2. Quality Criteria Definitions:

   - **Testable**: Criteria can be verified via shell command, file check, or tool output
   - **Unambiguous**: Criteria have clear pass/fail conditions, no subjective judgments
   - **Tied to specific task**: Criteria reference task by number/name from tasks.md
   - **Not over-scoped**: Criteria don't exceed task boundaries (appropriate granularity)

3. Model configuration: temperature 0.1 (deterministic), top_p 0.8 (conservative), max_tokens 2048

4. Output: Validated desired states + criteria meeting all quality criteria

**Step 6: Merge and Finalize**

**Purpose:** Merge all chunk outputs into single outputs.md.

**Input:** Validated outputs from all chunks.

**Processing:**

1. Assemble outputs.md structure:
   - Start with full tasks.md content (preserves traceability)
   - Append "# Desired Output States" header
   - Append desired state sections for all tasks
   - Append "# Criteria Statistics" header with summary

2. LLM call to format final output:
   ```
   Format the desired output states and criteria into a single markdown file.
   
   Structure:
   # Task Deconstruction for Prompt: <prompt_summary>
   
   [Include full tasks.md content here]
   
   # Desired Output States
   
   ## Task 1: <Task Name>
   ### Desired End-State
   <desired_end_state>
   ### Testable Criteria
   <criteria>
   
   [Continue for all tasks]
   
   # Criteria Statistics
   - Total tasks: <count>
   - Total criteria: <count>
   - Avg criteria per task: <value>
   - Criteria distribution: <breakdown by criteria count>
   
   Desired states and criteria:
   {{all_desired_states_and_criteria}}
   ```

3. Model configuration: temperature 0.05 (very deterministic), top_p 0.7 (conservative), max_tokens 3072

4. Output: Final outputs.md with accumulated content

### Evaluation Criteria

**SW2 Pass Conditions:**

1. **100% Task Coverage** — Every task from tasks.md has corresponding desired state
2. **Testability** — All criteria are testable (observable, measurable)
3. **Unambiguity** — All criteria are unambiguous (clear pass/fail)
4. **Task Reference** — All criteria reference specific task
5. **Scope Alignment** — All criteria match task scope (not over-scoped)
6. **GWT Syntax** — All criteria parse without GWT syntax errors
7. **Statistics Completeness** — All statistics fields present and accurate

**SW2 Fail Conditions (any of these triggers iteration):**

1. **Missing Desired States** — Some tasks lack desired state criteria
2. **Untestable Criteria** — Some criteria cannot be verified (subjective, unobservable)
3. **Ambiguous Criteria** — Some criteria have unclear pass/fail conditions
4. **Missing Task Reference** — Some criteria don't reference specific task
5. **Over-Scoped Criteria** — Some criteria exceed task boundaries
6. **GWT Syntax Errors** — Some criteria have GWT parsing errors
7. **Incomplete Statistics** — Some statistics fields missing or inaccurate

### Fix Loops

**Fix Loop 1: Missing Desired States**

**Trigger:** Tasks lack desired state criteria

**Fix Strategy:**
1. LLM call to generate missing desired states:
   ```
   The following tasks from tasks.md lack desired state criteria:
   {{tasks_without_desired_states}}
   
   For each task, generate a desired end-state (2-4 sentences describing observable completion state) and 1-3 testable criteria in GWT format.
   ```

2. Add missing desired states and criteria

3. Validate new criteria meet quality criteria

**Fix Loop 2: Testability**

**Trigger:** Criteria are untestable

**Fix Strategy:**
1. LLM call to fix testability:
   ```
   The following criteria are not testable (cannot be verified via shell, file check, or tool output):
   {{untestable_criteria}}
   
   For each criterion, rewrite it to be testable by:
   - Making it observable (specify what to check: file exists, process running, output contains text)
   - Making it measurable (specify quantity, threshold, or presence)
   - Using verifiable conditions (file exists, command succeeds, output contains string)
   ```

2. Replace untestable criteria with improved versions

3. Validate improved criteria are testable

**Fix Loop 3: Ambiguity**

**Trigger:** Criteria are ambiguous

**Fix Strategy:**
1. LLM call to fix ambiguity:
   ```
   The following criteria are ambiguous (unclear pass/fail conditions, subjective judgments):
   {{ambiguous_criteria}}
   
   For each criterion, rewrite it to be unambiguous by:
   - Specifying clear pass/fail conditions
   - Removing subjective language (e.g., "good", "adequate")
   - Using concrete checks (e.g., "file contains exactly 5 lines", not "file has reasonable content")
   ```

2. Replace ambiguous criteria with improved versions

3. Validate improved criteria are unambiguous

**Fix Loop 4: Scope Alignment**

**Trigger:** Criteria exceed task scope

**Fix Strategy:**
1. LLM call to fix scope:
   ```
   The following criteria exceed the scope of their corresponding tasks (too broad, over-constrained):
   {{overscoped_criteria}}
   
   For each criterion, rewrite it to match task scope by:
   - Focusing on task-specific outcomes (not broader system states)
   - Reducing criteria count if over-constrained (from 3 to 1-2 criteria)
   - Ensuring criteria directly validate task completion
   ```

2. Replace over-scoped criteria with improved versions

3. Validate improved criteria match task scope

### Chunk Sizing Strategy

SW2 uses adaptive chunking based on task complexity:

**Initial Chunk Size:**

```
If atomic_task_count <= 5:
    initial_chunk_size = 5
Else if atomic_task_count <= 10:
    initial_chunk_size = 4
Else if atomic_task_count <= 15:
    initial_chunk_size = 3
Else:
    initial_chunk_size = 2
```

**Chunk Growth Logic:**

- Start with initial chunk size based on task count
- If quality gates pass on all chunks in iteration 1, increase chunk_size by 1
- If quality gates fail on any chunk, decrease chunk_size by 1 (min 2)
- Maximum chunk_size: 5 (prevents context overflow)
- Minimum chunk_size: 2 (ensures focused iteration)

**Chunk Grouping Strategy:**

- Group tasks by complexity (low, medium, high)
- Group similar complexity tasks together (all 1-2 point tasks together, etc.)
- This enables focused prompt templates per complexity group
- Reduces context switching between different task types

### Model Configuration Reference

**LoadParams for SW2:**

```yaml
load_params:
  context_size: 262144          # Full context window
  batch_size: 2048               # Default batch size
  ubatch_size: 512               # Default micro-batch size
  cache_type_k: "q8_0"           # Q8_0 KV cache K
  cache_type_v: "q8_0"           # Q8_0 KV cache V
  gpu_layers: 99                  # full GPU offload (benchmark-proven fastest on RX580)
  threads: 5                     # 5 CPU threads
  use_mmap: true                 # Memory-mapped file loading
  flash_attn: true               # Flash attention (safe for Vulkan)
  cont_batching: false            # Continuous batching disabled (Vulkan requirement)
  no_cache_prompt: true           # Prompt caching disabled (Vulkan requirement)
  parallel: 1                    # Single parallel slot
```

**Sampling Parameters for SW2:**

```yaml
sampling:
  temperature: 0.15              # Between SW1's 0.1 and default 0.2
  top_p: 0.85                    # Between SW1's 0.8 and default 0.9
  top_k: 40                      # Default top-k sampling
  max_tokens: 3072               # 3/4 of default 4096
  repeat_penalty: 1.0            # No repetition penalty (default)
  seed: null                      # Random seed (default)
```

**Why This Configuration:**

- Medium temperature (0.15) balances determinism and creativity for GWT criteria generation
- Balanced top_p (0.85) allows diverse criteria while maintaining consistency
- Reduced max_tokens (3072) allows comprehensive criteria without over-generation
- Default context (262144) enables processing of large task trees without truncation

## SW3: Agentic Categorization Specification

**File:** `workflows/sw3-agentic-categorization.yml`

**Purpose:** Assign behavior categories to each task to guide YAML structure mapping.

**Input Format:** tasks.md from SW1 + outputs.md from SW2

**Output Format:** Markdown file (`categories.md`) with category labels and reasoning per task

### Input Specification

**tasks.md + outputs.md Combined Structure:**

```markdown
# Task Deconstruction for Prompt: <prompt_summary>

[... task hierarchy ...]

# Desired Output States

[... desired states and GWT criteria ...]
```

**Input Requirements:**

1. Both files must exist at expected paths
2. Both files must be valid markdown (parseable)
3. tasks.md must contain hierarchical task tree
4. outputs.md must contain desired states and criteria
5. Task counts must match (same number of tasks in both files)

### Output Specification

**categories.md File Structure:**

```markdown
# Task Deconstruction for Prompt: <prompt_summary>

[... tasks.md content included ...]

# Desired Output States

[... outputs.md content included ...]

# Category Assignments

## Task 1: <Task Name>

### Category: <category_label>

### Reasoning
<2-3 sentences explaining why category fits, referencing task characteristics>

## Task 2: <Task Name>

### Category: <category_label>

### Reasoning
...

## Category Statistics
- Total tasks: <count>
- Categories used: <count>
- Category distribution: <breakdown>
```

**Output Requirements:**

1. **Include tasks.md + outputs.md Content** — Output starts with full content from both files
2. **One Assignment Per Task** — Each task from tasks.md has category assignment
3. **Canonical Category Labels** — All categories from canonical set (no invented categories)
4. **Reasoning Provided** — 2-3 sentences explaining assignment
5. **Category Statistics** — Summary of category distribution at end of file

### Internal Processing Steps

**Step 1: Load Task and Criteria**

**Purpose:** Read tasks.md + outputs.md and extract atomic tasks with criteria.

**Input:** tasks.md path + outputs.md path.

**Processing:**

1. Shell actions in `before_step_starts` hook to read both files:
```yaml
when:
  before_step_starts:
    - shell:
        command: "cat {{inputs.tasks_path}}"
        bookmark_as: "tasks_md_content"
        fail_on_error: true
    - shell:
        command: "cat {{inputs.outputs_path}}"
        bookmark_as: "outputs_md_content"
        fail_on_error: true
    - log:
        to_file_path: "./workspace/logs/sw3-start.log"
        level: info
        message: "Loaded tasks.md ({{bookmarks.tasks_md_content.word_count}} words) and outputs.md ({{bookmarks.outputs_md_content.word_count}} words)"
```

2. Parse combined content to extract atomic tasks with:
   - Task number/name
   - Task description
   - Desired end-state
   - GWT criteria

3. LLM call to extract task summaries:
   ```
   From the following task hierarchy and desired states, extract a summary for each atomic task.
   
   For each task, provide: task number, name, description, desired end-state, criteria count.
   
   Tasks and desired states:
   {{bookmarks.tasks_md_content}}
   
   {{bookmarks.outputs_md_content}}
   
   Output format:
   - Task X: <task_name> - <task_description>
     Desired end-state: <desired_end_state>
     Criteria count: <number>
   - Task Y: <task_name> - <task_description>
     Desired end-state: <desired_end_state>
     Criteria count: <number>
   ```

4. Model configuration: temperature 0.2 (balanced), top_p 0.9 (default), max_tokens 3072

5. Output: List of atomic tasks with summaries

**Step 2: Canonical Set Definition**

**Purpose:** Define canonical category set for categorization.

**Input:** None (canonical set is fixed).

**Processing:**

1. Canonical category set (fixed for all prompts):

```
Canonical Categories (10 total):
1. file-read: Read file or directory contents
2. transform-llm: Use LLM to transform content (generate, summarize, translate)
3. validate-gate: Check condition, route based on result
4. loop-iterate: Iterate over collection or repeat until condition
5. branch-decision: Conditional routing based on GWT expression
6. shell-execute: Run shell command
7. tool-call: Use specific tool (grep, file_read, etc.)
8. checkpoint-state: Save state for later retrieval
9. notify-external: Send notification outside workflow
10. sub-workflow-ref: Invoke external workflow
```

2. Store canonical set as workflow variable or markdown section

3. Output: Canonical category set definition

**Step 3: Categorization Loop**

**Purpose:** Assign category from canonical set to each task.

**Input:** Task summaries from Step 1 + canonical set from Step 2.

**Processing:**

1. For each task (or group of 3-5 tasks for efficiency):

2. LLM call to assign category:
   ```
   For the following task, assign a category from the canonical set that best describes its behavior.
   
   Task:
   - Name: <task_name>
   - Description: <task_description>
   - Desired end-state: <desired_end_state>
   - GWT criteria: <criteria_count> criteria
   
   Canonical categories:
   1. file-read: Read file or directory contents
   2. transform-llm: Use LLM to transform content (generate, summarize, translate)
   3. validate-gate: Check condition, route based on result
   4. loop-iterate: Iterate over collection or repeat until condition
   5. branch-decision: Conditional routing based on GWT expression
   6. shell-execute: Run shell command
   7. tool-call: Use specific tool (grep, file_read, etc.)
   8. checkpoint-state: Save state for later retrieval
   9. notify-external: Send notification outside workflow
   10. sub-workflow-ref: Invoke external workflow
   
   Assign the category that best fits by analyzing:
   - Task description keywords (read, write, generate, validate, etc.)
   - Desired end-state characteristics (file operations, transformations, conditions)
   - GWT criteria patterns (file checks, content generation, routing logic)
   
   Output format:
   Task: <task_name>
   Category: <category_label>
   Reasoning: <2-3 sentences explaining why category fits, referencing specific task characteristics>
   ```

3. Model configuration: temperature 0.1 (low temperature for consistency), top_p 0.9 (default), max_tokens 2048

4. Validation: Check that assigned category is from canonical set

5. Output: Category assignment + reasoning for each task

**Step 4: GWT Quality Gate per Task**

**Purpose:** Ensure category assignments meet quality standards.

**Input:** Category assignments from Step 3.

**Processing:**

1. GWT evaluation for each task:

```yaml
when:
  after_step_succeeds:
    - gwt:
        # Gate 1: From canonical set
        - given: "category.in_canonical_set == true"
          when: "Category is from canonical set (one of 10 defined categories)"
          then: { route_to: validate_mutually_exclusive }
        - given: "category.in_canonical_set == false"
          when: "Category is not from canonical set (invented category)"
          then: { route_to: fix_category }
        
        # Gate 2: Mutually exclusive
        - given: "task.category_count == 1"
          when: "One category per task (mutually exclusive)"
          then: { route_to: validate_reasoning }
        - given: "task.category_count > 1"
          when: "Multiple categories per task (not mutually exclusive)"
          then: { route_to: select_single_category }
        
        # Gate 3: Reasoning provided
        - given: "category.reasoning_present == true"
          when: "Reasoning explanation provided for category assignment"
          then: { route_to: validate_consistency }
        - given: "category.reasoning_present == false"
          when: "No reasoning explanation provided"
          then: { route_to: add_reasoning }
        
        # Gate 4: Category consistency
        - given: "similar_tasks_same_category == true"
          when: "Similar tasks have same category assignment"
          then: { route_to: accept_assignment }
        - given: "similar_tasks_same_category == false"
          when: "Similar tasks have different category assignments (inconsistent)"
          then: { route_to: harmonize_categories }
```

2. Quality Criteria Definitions:

   - **From canonical set**: Category must be one of 10 defined categories (no invented categories)
   - **Mutually exclusive**: One category per task (no multiple categories)
   - **Reasoning provided**: 2-3 sentences explaining assignment
   - **Category consistency**: Similar tasks get same category

3. Model configuration: temperature 0.1 (deterministic), top_p 0.8 (conservative), max_tokens 2048

4. Output: Validated category assignments meeting all quality criteria

**Step 5: Category Distribution Validation**

**Purpose:** Ensure category distribution is appropriate (not all tasks same category).

**Input:** All category assignments from Step 4.

**Processing:**

1. Calculate category distribution statistics:
   - Count tasks per category
   - Calculate category diversity metric (number of categories / total tasks)
   - Identify dominant categories (> 50% of tasks)

2. LLM call to validate distribution:
   ```
   Review the category assignments and validate that the category distribution is appropriate.
   
   Category assignments:
   {{all_category_assignments}}
   
   Distribution statistics:
   - Total tasks: <count>
   - Categories used: <count>
   - Category diversity: <percentage> (categories used / total tasks)
   - Dominant categories: <list of categories with >50% of tasks>
   
   Validate that:
   - Category diversity >= 0.3 (at least 30% of tasks have different categories)
   - No single category has > 80% of tasks (prevents over-generalization)
   - Distribution matches task complexity (diverse tasks should have diverse categories)
   
   Output: PASS or FAIL with specific issues
   ```

3. Model configuration: temperature 0.1 (deterministic), top_p 0.8 (conservative), max_tokens 1024

4. Output: Category distribution validation result

**Step 6: Final Assembly**

**Purpose:** Assemble final categories.md with accumulated content.

**Input:** tasks.md + outputs.md + validated category assignments.

**Processing:**

1. Assemble categories.md structure:
   - Start with full tasks.md content
   - Append full outputs.md content
   - Append "# Category Assignments" header
   - Append category assignment sections for all tasks
   - Append "# Category Statistics" header with summary

2. LLM call to format final output:
   ```
   Format the task hierarchy, desired states, and category assignments into a single markdown file.
   
   Structure:
   # Task Deconstruction for Prompt: <prompt_summary>
   
   [Include full tasks.md content]
   
   # Desired Output States
   
   [Include full outputs.md content]
   
   # Category Assignments
   
   ## Task 1: <Task Name>
   ### Category: <category_label>
   ### Reasoning
   <reasoning>
   
   [Continue for all tasks]
   
   # Category Statistics
   - Total tasks: <count>
   - Categories used: <count>
   - Category distribution: <breakdown>
   
   Content to format:
   {{tasks_md_content}}
   {{outputs_md_content}}
   {{category_assignments}}
   ```

3. Model configuration: temperature 0.05 (very deterministic), top_p 0.7 (conservative), max_tokens 4096

4. Output: Final categories.md with accumulated content

### Evaluation Criteria

**SW3 Pass Conditions:**

1. **100% Task Coverage** — Every task from tasks.md has category assignment
2. **Canonical Set Adherence** — All categories from canonical set (no invented categories)
3. **Mutual Exclusivity** — One category per task (no multiple categories)
4. **Reasoning Presence** — All assignments have reasoning explanation
5. **Category Consistency** — Similar tasks have same category assignment
6. **Distribution Diversity** — Category diversity ≥ 0.3 (at least 30% of tasks have different categories)
7. **Statistics Completeness** — All statistics fields present and accurate

**SW3 Fail Conditions (any of these triggers iteration):**

1. **Missing Category Assignments** — Some tasks lack category assignments
2. **Invalid Categories** — Some categories not from canonical set
3. **Multiple Categories Per Task** — Some tasks have multiple categories
4. **Missing Reasoning** — Some assignments lack reasoning explanation
5. **Inconsistent Categorization** — Similar tasks have different categories
6. **Poor Diversity** — Category diversity < 0.3 (all tasks same category)
7. **Incomplete Statistics** — Some statistics fields missing or inaccurate

### Fix Loops

**Fix Loop 1: Missing Category Assignments**

**Trigger:** Tasks lack category assignments

**Fix Strategy:**
1. LLM call to assign categories to missing tasks:
   ```
   The following tasks from tasks.md lack category assignments:
   {{tasks_without_categories}}
   
   For each task, assign a category from the canonical set that best describes its behavior.
   Provide 2-3 sentences of reasoning explaining the assignment.
   ```

2. Add missing category assignments

3. Validate assignments meet quality criteria

**Fix Loop 2: Invalid Categories**

**Trigger:** Categories not from canonical set

**Fix Strategy:**
1. LLM call to fix invalid categories:
   ```
   The following tasks have category assignments that are not from the canonical set:
   {{tasks_with_invalid_categories}}
   
   Canonical categories: [file-read, transform-llm, validate-gate, loop-iterate, branch-decision, shell-execute, tool-call, checkpoint-state, notify-external, sub-workflow-ref]
   
   For each task, reassign to the closest matching canonical category based on task behavior.
   Provide reasoning explaining the reassignment.
   ```

2. Replace invalid categories with canonical categories

3. Validate new categories from canonical set

**Fix Loop 3: Inconsistent Categorization**

**Trigger:** Similar tasks have different categories

**Fix Strategy:**
1. LLM call to harmonize categories:
   ```
   The following tasks have similar descriptions or desired states but different category assignments (inconsistent categorization):
   {{inconsistent_categorizations}}
   
   Review these tasks and harmonize their category assignments so that similar tasks have the same category.
   Provide reasoning for the harmonized category.
   ```

2. Replace inconsistent categories with harmonized versions

3. Validate similar tasks now have same category

### Chunk Sizing Strategy

SW3 processes tasks individually or in small groups for consistent categorization:

**Chunk Size Determination:**

```
If atomic_task_count <= 5:
    chunk_size = 5  # Small task set, process 5 at a time
Else if atomic_task_count <= 10:
    chunk_size = 4  # Medium task set, process 4 at a time
Else if atomic_task_count <= 15:
    chunk_size = 3  # Large task set, process 3 at a time
Else:
    chunk_size = 2  # Very large task set, process 2 at a time
```

**Why Smaller Chunks:**

Categorization benefits from smaller chunks because:
- Each task requires careful analysis of behavior characteristics
- Category consistency requires comparing similar tasks
- Smaller chunks reduce context pressure for consistent assignments
- Enables focused iteration on specific categorization issues

### Model Configuration Reference

**LoadParams for SW3:**

```yaml
load_params:
  context_size: 262144          # Full context window
  batch_size: 2048               # Default batch size
  ubatch_size: 512               # Default micro-batch size
  cache_type_k: "q8_0"           # Q8_0 KV cache K
  cache_type_v: "q8_0"           # Q8_0 KV cache V
  gpu_layers: 99                  # full GPU offload (benchmark-proven fastest on RX580)
  threads: 5                     # 5 CPU threads
  use_mmap: true                 # Memory-mapped file loading
  flash_attn: true               # Flash attention (safe for Vulkan)
  cont_batching: false            # Continuous batching disabled (Vulkan requirement)
  no_cache_prompt: true           # Prompt caching disabled (Vulkan requirement)
  parallel: 1                    # Single parallel slot
```

**Sampling Parameters for SW3:**

```yaml
sampling:
  temperature: 0.1               # Low temperature for consistent categorization
  top_p: 0.9                     # Default top-p
  top_k: 40                      # Default top-k sampling
  max_tokens: 2048               # Allows comprehensive reasoning without over-generation
  repeat_penalty: 1.0            # No repetition penalty (default)
  seed: null                      # Random seed (default)
```

**Why This Configuration:**

- Low temperature (0.1) maximizes consistency, similar tasks get same category
- Default top_p (0.9) allows diverse reasoning while maintaining consistent assignments
- Reduced max_tokens (2048) allows comprehensive reasoning without over-generation
- Default context (262144) enables processing of large task trees without truncation

## SW4: YAML Substructure Translation Specification

**File:** `workflows/sw4-yaml-substructure-translation.yml`

**Purpose:** Translate categorized tasks into YAML skeleton structures per category.

**Input Format:** tasks.md from SW1 + outputs.md from SW2 + categories.md from SW3

**Output Format:** Markdown file (`structs.md`) with YAML code blocks with skeleton structures + intent prose

### Input Specification

**tasks.md + outputs.md + categories.md Combined Structure:**

```markdown
# Task Deconstruction for Prompt: <prompt_summary>

[... task hierarchy ...]

# Desired Output States

[... desired states and GWT criteria ...]

# Category Assignments

[... category labels and reasoning ...]
```

**Input Requirements:**

1. All three files must exist at expected paths
2. All three files must be valid markdown (parseable)
3. Task counts must match across all three files
4. Each task must have: name, description, category, desired state, criteria

### Output Specification

**structs.md File Structure:**

```markdown
# Task Deconstruction for Prompt: <prompt_summary>

[... tasks.md content included ...]

# Desired Output States

[... outputs.md content included ...]

# Category Assignments

[... categories.md content included ...]

# YAML Skeleton Structures

## Category: <category_name>

### Task 1: <Task Name>

```yaml
<task_name>:
  generative_entity: "${models.primary-analyzer}"
  prompt: "<prompt text>"
  requires: [<list of dependent steps>]
  when:
    after_step_succeeds:
      - <hook actions>
    after_step_fails:
      - <hook actions>
```

### Task 2: <Task Name>

```yaml
<task_name>:
...
```

## Skeleton Statistics
- Total tasks: <count>
- Total skeletons: <count>
- Skeletons per category: <breakdown>
```

**Output Requirements:**

1. **Include All Previous Content** — Output starts with tasks.md + outputs.md + categories.md
2. **One Skeleton Per Task** — Each task has corresponding YAML skeleton structure
3. **YAML Code Blocks Only** — Only ```yaml blocks, no ```rust, ```python, ```bash
4. **Correct Schema Shape** — Follows `agentic_workflow.steps.<step_name>:` structure from schema
5. **Model Reference Correct** — Uses `${models.primary-analyzer}` (not `${models.qwen35}`)
6. **Hook Wiring Appropriate** — Hooks match category expectations
7. **Statistics Completeness** — Skeleton statistics present and accurate

### Internal Processing Steps

**Step 1: Load All Previous Outputs**

**Purpose:** Read tasks.md + outputs.md + categories.md and create unified task view.

**Input:** Three file paths.

**Processing:**

1. Shell actions in `before_step_starts` hook to read all three files:
```yaml
when:
  before_step_starts:
    - shell:
        command: "cat {{inputs.tasks_path}}"
        bookmark_as: "tasks_md_content"
        fail_on_error: true
    - shell:
        command: "cat {{inputs.outputs_path}}"
        bookmark_as: "outputs_md_content"
        fail_on_error: true
    - shell:
        command: "cat {{inputs.categories_path}}"
        bookmark_as: "categories_md_content"
        fail_on_error: true
    - log:
        to_file_path: "./workspace/logs/sw4-start.log"
        level: info
        message: "Loaded tasks.md ({{bookmarks.tasks_md_content.word_count}} words), outputs.md ({{bookmarks.outputs_md_content.word_count}} words), categories.md ({{bookmarks.categories_md_content.word_count}} words)"
```

2. LLM call to create unified task view:
   ```
   From the following task hierarchy, desired states, and category assignments, create a unified view for each task.
   
   For each atomic task, extract:
   - Task number/name
   - Task description
   - Desired end-state
   - GWT criteria (count)
   - Category label
   - Category reasoning
   
   Task hierarchy:
   {{bookmarks.tasks_md_content}}
   
   Desired states:
   {{bookmarks.outputs_md_content}}
   
   Category assignments:
   {{bookmarks.categories_md_content}}
   
   Output format for each task:
   - Task <number>: <task_name> (<category>)
     Description: <task_description>
     Desired end-state: <desired_end_state>
     Criteria count: <number>
     Category reasoning: <reasoning>
   ```

3. Model configuration: temperature 0.2 (balanced), top_p 0.9 (default), max_tokens 4096

4. Output: Unified task view with all information for each task

**Step 2: Group by Category**

**Purpose:** Group tasks by category for category-specific YAML generation.

**Input:** Unified task view from Step 1.

**Processing:**

1. Group tasks by category label from canonical set

2. Output: List of category groups, each containing 1-N tasks

**Step 3: YAML Skeleton Generation Loop**

**Purpose:** Generate YAML skeleton structure for each task in category.

**Input:** Category group (tasks with same category).

**Processing:**

1. For each category:

2. LLM call to generate YAML skeletons:
   ```
   For each task in the <category_name> category, generate a YAML skeleton structure using the unified workflow schema.
   
   Schema reference:
   - Step structure: docs/schema/unified-workflow-schema.yml lines 318-367
   - Hook configuration: docs/schema/unified-workflow-schema.yml lines 298-367
   - Template interpolation: docs/schema/unified-workflow-schema.yml lines 752-765
   
   Required fields for each step:
   - generative_entity: Must be "${models.primary-analyzer}" (not "${models.qwen35}")
   - prompt: Task-specific prompt text (can reference variables)
   - requires: List of dependent step names (if any)
   - when: Hook configuration (at minimum: after_step_succeeds)
   
   Category-specific guidance for <category_name>:
   <category_guidance>
   
   Tasks in this category:
   {{category_tasks}}
   
   Output format:
   ## Category: <category_name>
   
   ### Task 1: <task_name>
   
   ```yaml
   <task_name>:
     generative_entity: "${models.primary-analyzer}"
     prompt: "<prompt text derived from task description and desired end-state>"
     requires: [<step1>, <step2>]
     when:
       after_step_succeeds:
         - save_to:
             - <bookmark_name>
             - "./workspace/output/<file_path>.yaml"
         - log:
             to_file_path: "./workspace/logs/<log_file>.log"
             event_fields: [step_name, duration_ms]
             level: info
       after_step_fails:
         - log:
             to_file_path: "./workspace/logs/<log_file>.log"
             event_fields: [error_message]
             level: error
   ```
   
   ### Task 2: <task_name>
   ...
   ```

3. Model configuration: temperature 0.05 (very low temperature for YAML syntax correctness), top_p 0.7 (conservative), max_tokens 4096

4. Validation: Check that skeleton follows schema shape

5. Output: YAML skeleton structures for all tasks in category

**Category-Specific Guidance:**

```markdown
file-read:
- Hook emphasis: after_step_succeeds with save_to + log
- Prompt pattern: "Read <file_path>" or "Read files from <directory>"
- Dependencies: Usually none (file operations are independent)
- Typical actions: save_to (file content), log (operation completion)

transform-llm:
- Hook emphasis: after_step_succeeds with save_to + gwt (quality gate)
- Prompt pattern: "Transform <input> to <output>" or "Generate <content>"
- Dependencies: May depend on file-read tasks (need input data)
- Typical actions: save_to (transformed content), gwt (quality check), route_to (conditional)

validate-gate:
- Hook emphasis: after_step_succeeds with gwt (condition check, routing)
- Prompt pattern: "Validate <input> against <criteria>" or "Check if <condition>"
- Dependencies: May depend on transform-llm tasks (need data to validate)
- Typical actions: gwt (condition check), route_to (pass/fail routing), log (validation result)

loop-iterate:
- Hook emphasis: after_step_succeeds with gwt (iteration condition)
- Prompt pattern: "Iterate over <collection>" or "Repeat until <condition>"
- Dependencies: May depend on validate-gate tasks (need condition to evaluate)
- Typical actions: gwt (continue/stop condition), bookmark (iteration state), route_to (next iteration)

branch-decision:
- Hook emphasis: after_step_succeeds with gwt (branching logic)
- Prompt pattern: "Branch based on <condition>" or "Route to different path if <condition>"
- Dependencies: May depend on validate-gate tasks (need condition to evaluate)
- Typical actions: gwt (branch condition), route_to (branch selection), log (branch taken)

shell-execute:
- Hook emphasis: after_step_succeeds with shell (execute command) + log (command output)
- Prompt pattern: "Execute command: <command>" or "Run shell: <command>"
- Dependencies: May depend on file-read tasks (need files as input)
- Typical actions: shell (command execution), log (command output), save_to (command results)

tool-call:
- Hook emphasis: after_step_succeeds with save_to (tool output) + log (tool operation)
- Prompt pattern: "Use tool <tool_name> to <action>" or "Call tool: <tool_name>"
- Dependencies: May depend on file-read tasks (need files as input)
- Typical actions: save_to (tool output), log (tool operation), gwt (output validation)

checkpoint-state:
- Hook emphasis: after_step_succeeds with bookmark (state save) + save_to (state file)
- Prompt pattern: "Save state for later retrieval" or "Create checkpoint: <name>"
- Dependencies: Usually depends on previous tasks (checkpoint accumulated state)
- Typical actions: bookmark (state save), save_to (state file), log (checkpoint created)

notify-external:
- Hook emphasis: after_step_succeeds with notify (send notification) + log (notification sent)
- Prompt pattern: "Send notification to <recipient>" or "Notify external: <message>"
- Dependencies: May depend on previous tasks (notify completion status)
- Typical actions: notify (send notification), log (notification sent), save_to (notification record)

sub-workflow-ref:
- Hook emphasis: after_step_succeeds with route_to (sub-workflow invocation) + log (invocation)
- Prompt pattern: "Invoke sub-workflow: <name>" or "Call external workflow: <name>"
- Dependencies: Usually depends on previous tasks (prepare inputs for sub-workflow)
- Typical actions: route_to (sub-workflow reference), log (invocation), save_to (inputs preparation)
```

**Step 4: GWT Quality Gate per Skeleton**

**Purpose:** Ensure YAML skeletons meet quality standards.

**Input:** YAML skeleton structures from Step 3.

**Processing:**

1. GWT evaluation for each skeleton:

```yaml
when:
  after_step_succeeds:
    - gwt:
        # Gate 1: Schema shape compliance
        - given: "yaml.follows_schema_shape == true"
          when: "YAML follows agentic_workflow.steps.<step_name>: structure"
          then: { route_to: validate_model_reference }
        - given: "yaml.follows_schema_shape == false"
          when: "YAML does not follow expected schema structure"
          then: { route_to: fix_schema_shape }
        
        # Gate 2: Model reference correct
        - given: "yaml.model_reference == '${models.primary-analyzer}'"
          when: "YAML references correct model (${models.primary-analyzer})"
          then: { route_to: validate_hook_wiring }
        - given: "yaml.model_reference != '${models.primary-analyzer}'"
          when: "YAML references wrong model (not ${models.primary-analyzer})"
          then: { route_to: fix_model_reference }
        
        # Gate 3: Hook wiring appropriate
        - given: "yaml.has_appropriate_hooks == true"
          when: "YAML has appropriate hooks for category"
          then: { route_to: validate_dependencies }
        - given: "yaml.has_appropriate_hooks == false"
          when: "YAML has inappropriate or missing hooks for category"
          then: { route_to: add_hooks }
        
        # Gate 4: No non-YAML code blocks
        - given: "yaml_blocks_only == true"
          when: "Only YAML code blocks present (no rust, python, bash blocks)"
          then: { route_to: accept_skeleton }
        - given: "yaml_blocks_only == false"
          when: "Non-YAML code blocks present (rust, python, bash)"
          then: { route_to: remove_non_yaml_blocks }
```

2. Quality Criteria Definitions:

   - **Schema shape compliance**: Follows `agentic_workflow.steps.<step_name>:` structure
   - **Model reference correct**: Uses `${models.primary-analyzer}`
   - **Hook wiring appropriate**: Hooks match category expectations
   - **No non-YAML code blocks**: Only ```yaml blocks, no ```rust, ```python, ```bash

3. Model configuration: temperature 0.05 (very deterministic), top_p 0.7 (conservative), max_tokens 2048

4. Output: Validated YAML skeletons meeting all quality criteria

**Step 5: YAML Syntax Validation**

**Purpose:** Validate YAML syntax (parseable by serde_yaml).

**Input:** Validated YAML skeletons from Step 4.

**Processing:**

1. Shell action to validate YAML syntax:
```yaml
when:
  after_step_succeeds:
    - shell:
        command: "cargo run -- parse-yaml --input {{inputs.structs_path}}"
        fail_on_error: true
    - log:
        to_file_path: "./workspace/logs/sw4-validation.log"
        level: info
        message: "YAML syntax validation complete"
```

2. If validation fails, route to fix step with specific error details

3. Output: Syntax-validated YAML skeletons

**Step 6: Schema Validation**

**Purpose:** Validate YAML structure against unified schema.

**Input:** Syntax-validated YAML skeletons from Step 5.

**Processing:**

1. Shell action to validate against schema:
```yaml
when:
  after_step_succeeds:
    - shell:
        command: "cargo run -- validate-yaml --schema docs/schema/unified-workflow-schema.yml --input {{inputs.structs_path}}"
        fail_on_error: true
    - log:
        to_file_path: "./workspace/logs/sw4-validation.log"
        level: info
        message: "Schema validation complete, schema_valid=true"
```

2. If validation fails, route to fix step with specific schema violation details

3. Output: Schema-validated YAML skeletons

**Step 7: Final Assembly**

**Purpose:** Assemble final structs.md with accumulated content.

**Input:** All previous content + validated YAML skeletons.

**Processing:**

1. Assemble structs.md structure:
   - Start with full tasks.md content
   - Append full outputs.md content
   - Append full categories.md content
   - Append "# YAML Skeleton Structures" header
   - Append YAML skeleton code blocks organized by category
   - Append "# Skeleton Statistics" header with summary

2. LLM call to format final output:
   ```
   Format the task hierarchy, desired states, category assignments, and YAML skeletons into a single markdown file.
   
   Structure:
   # Task Deconstruction for Prompt: <prompt_summary>
   
   [Include full tasks.md content]
   
   # Desired Output States
   
   [Include full outputs.md content]
   
   # Category Assignments
   
   [Include full categories.md content]
   
   # YAML Skeleton Structures
   
   [Include YAML skeleton code blocks organized by category]
   
   # Skeleton Statistics
   - Total tasks: <count>
   - Total skeletons: <count>
   - Skeletons per category: <breakdown>
   
   Content to format:
   {{tasks_md_content}}
   {{outputs_md_content}}
   {{categories_md_content}}
   {{yaml_skeletons}}
   ```

3. Model configuration: temperature 0.05 (very deterministic), top_p 0.7 (conservative), max_tokens 8192

4. Output: Final structs.md with accumulated content

### Evaluation Criteria

**SW4 Pass Conditions:**

1. **100% Task Coverage** — Every task from tasks.md has YAML skeleton
2. **YAML Syntax Validity** — All skeletons parseable by serde_yaml (no syntax errors)
3. **Schema Compliance** — All skeletons follow unified schema structure
4. **Model Reference Correctness** — All skeletons use `${models.primary-analyzer}`
5. **Hook Presence** — All skeletons have appropriate hooks for category
6. **Non-YAML Block Absence** — Only ```yaml blocks, no ```rust, ```python, ```bash
7. **Statistics Completeness** — Skeleton statistics present and accurate

**SW4 Fail Conditions (any of these triggers iteration):**

1. **Missing Skeletons** — Some tasks lack YAML skeleton structures
2. **YAML Syntax Errors** — Some skeletons have YAML syntax errors
3. **Schema Mismatches** — Some skeletons don't follow unified schema structure
4. **Wrong Model Reference** — Some skeletons don't use `${models.primary-analyzer}`
5. **Missing Hooks** — Some skeletons lack appropriate hooks for category
6. **Non-YAML Blocks** — Non-YAML code blocks present (```rust, ```python, ```bash)
7. **Incomplete Statistics** — Some statistics fields missing or inaccurate

### Fix Loops

**Fix Loop 1: Missing Skeletons**

**Trigger:** Tasks lack YAML skeleton structures

**Fix Strategy:**
1. LLM call to generate missing skeletons:
   ```
   The following tasks from tasks.md lack YAML skeleton structures:
   {{tasks_without_skeletons}}
   
   For each task, generate a YAML skeleton structure using the unified workflow schema and category-specific guidance.
   Ensure the skeleton:
   - Follows agentic_workflow.steps.<step_name>: structure (schema lines 318-367)
   - Uses ${models.primary-analyzer} as model reference
   - Has appropriate hooks for the task's category
   - Includes requires dependencies on prior steps
   ```

2. Add missing YAML skeleton structures

3. Validate skeletons meet quality criteria

**Fix Loop 2: Schema Mismatches**

**Trigger:** Skeletons don't follow unified schema structure

**Fix Strategy:**
1. LLM call to fix schema mismatches:
   ```
   The following YAML skeleton structures do not follow the unified workflow schema:
   {{skeletons_with_schema_mismatches}}
   
   Unified schema reference:
   - Step structure: docs/schema/unified-workflow-schema.yml lines 318-367
   - Hook configuration: docs/schema/unified-workflow-schema.yml lines 298-367
   
   For each skeleton, fix the structure to match the schema by:
   - Ensuring required fields are present (generative_entity, prompt, when)
   - Removing unknown keys (only keys from schema allowed)
   - Correcting nesting structure (agentic_workflow.steps.<step_name>:)
   - Validating hook configuration format
   ```

2. Replace schema-mismatched skeletons with corrected versions

3. Validate corrected skeletons follow schema

**Fix Loop 3: Model Reference Errors**

**Trigger:** Skeletons don't use `${models.primary-analyzer}`

**Fix Strategy:**
1. LLM call to fix model references:
   ```
   The following YAML skeleton structures use incorrect model references (not ${models.primary-analyzer}):
   {{skeletons_with_wrong_model_references}}
   
   For each skeleton, fix the model reference to use ${models.primary-analyzer} instead.
   The model reference must be in this format: ${models.primary-analyzer}
   ```

2. Replace incorrect model references with `${models.primary-analyzer}`

3. Validate all skeletons use correct model reference

### Chunk Sizing Strategy

SW4 processes tasks by category (not by chunk size):

**Category-Based Processing:**

```
Group tasks by category label
For each category:
    Process all tasks in that category together (category-specific prompt template)
```

**Why Category-Based:**

Category-based processing enables:
- Category-specific prompt templates with targeted guidance
- Consistent YAML structures within categories
- Efficient processing (all similar tasks processed together)
- Easier validation (hooks match category patterns)

**Processing Order:**

Process categories in this order (from most common to least common):
1. transform-llm (usually most tasks)
2. file-read
3. validate-gate
4. branch-decision
5. shell-execute
6. loop-iterate
7. tool-call
8. checkpoint-state
9. notify-external
10. sub-workflow-ref

### Model Configuration Reference

**LoadParams for SW4:**

```yaml
load_params:
  context_size: 262144          # Full context window
  batch_size: 2048               # Default batch size
  ubatch_size: 512               # Default micro-batch size
  cache_type_k: "q8_0"           # Q8_0 KV cache K
  cache_type_v: "q8_0"           # Q8_0 KV cache V
  gpu_layers: 99                  # full GPU offload (benchmark-proven fastest on RX580)
  threads: 5                     # 5 CPU threads
  use_mmap: true                 # Memory-mapped file loading
  flash_attn: true               # Flash attention (safe for Vulkan)
  cont_batching: false            # Continuous batching disabled (Vulkan requirement)
  no_cache_prompt: true           # Prompt caching disabled (Vulkan requirement)
  parallel: 1                    # Single parallel slot
```

**Sampling Parameters for SW4:**

```yaml
sampling:
  temperature: 0.05              # Very low temperature for YAML syntax correctness
  top_p: 0.7                     # Lower than default for precise YAML generation
  top_k: 40                      # Default top-k sampling
  max_tokens: 8192               # 2x default for complete YAML structures
  repeat_penalty: 1.0            # No repetition penalty (default)
  seed: null                      # Random seed (default)
```

**Why This Configuration:**

- Very low temperature (0.05) maximizes YAML syntax correctness, minimizes creative deviations
- Lower top_p (0.7) provides more conservative token selection for precise YAML generation
- Increased max_tokens (8192) allows complete YAML structures for complex steps
- Default context (262144) enables processing of large task trees without truncation

## SW5: Final Workflow Assembly Specification

**File:** `workflows/sw5-final-workflow-assembly.yml`

**Purpose:** Assemble valid executable workflow YAML from skeleton structures.

**Input Format:** structs.md from SW4

**Output Format:** YAML file (`workflow.yml`) — Executable validated agentic workflow YAML

### Input Specification

**structs.md File Structure:**

```markdown
# Task Deconstruction for Prompt: <prompt_summary>

[... tasks.md content ...]

# Desired Output States

[... outputs.md content ...]

# Category Assignments

[... categories.md content ...]

# YAML Skeleton Structures

## Category: <category_name>

### Task 1: <Task Name>

```yaml
<task_name>:
  generative_entity: "${models.primary-analyzer}"
  prompt: "<prompt text>"
  requires: [<list of dependent steps>]
  when:
    after_step_succeeds:
      - <hook actions>
```

### Task 2: <Task Name>

```yaml
<task_name>:
...
```

## Skeleton Statistics
...
```

**Input Requirements:**

1. File must exist at expected path
2. File must be valid markdown (parseable)
3. File must contain YAML skeleton code blocks
4. Skeletons must be organized by category
5. All tasks from tasks.md must have corresponding skeletons

### Output Specification

**workflow.yml File Structure:**

```yaml
workflow:
  workflow_id: "<auto-generated-id>"
  name: "<auto-generated-name>"
  description: "<summarized from prompt>"
  version: "1.0.0"
  author: "Qwen 3.5-9B Meta-Workflow Generator"
  tags: [meta-workflow, qwen-3.5-9b, auto-generated]

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
      gpu_layers: 99
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
    <step_name_1>:
      generative_entity: "${models.primary-analyzer}"
      prompt: "<prompt text>"
      requires: [<step_dependencies>]
      when:
        <hook_configuration>
    
    <step_name_2>:
      generative_entity: "${models.primary-analyzer}"
      prompt: "<prompt text>"
      requires: [<step_dependencies>]
      when:
        <hook_configuration>
    
    ...

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
  error_handling:
    default_action:
      type: retry
    retry:
      max_attempts_per_step: 3
      max_attempts_per_workflow: 10
      backoff_strategy:
        type: exponential
        initial_delay: "1s"
        max_delay: "30s"
        multiplier: 2.0
        jitter: true
```

**Output Requirements:**

1. **Valid YAML** — Parseable by serde_yaml (no syntax errors)
2. **Follows Schema** — Follows unified workflow schema structure
3. **No Unknown Keys** — Only keys from unified schema allowed
4. **All Tasks Present** — Every task from tasks.md appears as step
5. **Dependencies Correct** — `requires:` lists reference existing steps
6. **Hooks Configured** — Appropriate hooks at appropriate trigger points
7. **Executable** — Runs on live Docker system without errors

### Internal Processing Steps

**Step 1: Load Skeleton Structures**

**Purpose:** Read structs.md and extract YAML skeleton code blocks.

**Input:** structs.md file path.

**Processing:**

1. Shell action in `before_step_starts` hook to read structs.md:
```yaml
when:
  before_step_starts:
    - shell:
        command: "cat {{inputs.structs_path}}"
        bookmark_as: "structs_md_content"
        fail_on_error: true
    - log:
        to_file_path: "./workspace/logs/sw5-start.log"
        level: info
        message: "Loaded structs.md, {{bookmarks.structs_md_content.word_count}} words"
    - shell:
        command: "grep -c '```yaml' {{inputs.structs_path}}"
        bookmark_as: "yaml_block_count"
        fail_on_error: true
```

2. LLM call to extract YAML skeleton code blocks:
   ```
   From the following YAML skeleton structures document, extract all YAML code blocks and their associated task names and categories.
   
   Document:
   {{bookmarks.structs_md_content}}
   
   For each YAML code block, extract:
   - Task name
   - Category
   - YAML content (without ```yaml markers)
   
   Output format:
   - Task <task_name> (<category>)
     YAML: <yaml content>
   ```

3. Model configuration: temperature 0.05 (very deterministic), top_p 0.7 (conservative), max_tokens 8192

4. Output: List of task names, categories, and YAML contents

**Step 2: Header Assembly**

**Purpose:** Generate workflow header (workflow_id, name, description, version, author, tags).

**Input:** Original prompt text (from tasks.md) + task count.

**Processing:**

1. LLM call to generate header:
   ```
   Generate a workflow header section for a YAML workflow file.
   
   Original prompt:
   {{original_prompt}}
   
   Task breakdown produced <task_count> tasks.
   
   Header should include:
   - workflow_id: Auto-generated unique identifier (e.g., "auto-generated-20260617-143052")
   - name: Concise name summarizing the workflow's purpose
   - description: 1-2 sentence summary of what the workflow does
   - version: "1.0.0" (fixed)
   - author: "Qwen 3.5-9B Meta-Workflow Generator" (fixed)
   - tags: [meta-workflow, qwen-3.5-9b, auto-generated]
   
   Output in YAML format:
   workflow:
     workflow_id: "<workflow_id>"
     name: "<workflow_name>"
     description: "<workflow_description>"
     version: "1.0.0"
     author: "Qwen 3.5-9B Meta-Workflow Generator"
     tags: [meta-workflow, qwen-3.5-9b, auto-generated]
   ```

2. Model configuration: temperature 0.1 (deterministic), top_p 0.8 (conservative), max_tokens 2048

3. Output: Workflow header YAML

**Step 3: Providers and Models Section Assembly**

**Purpose:** Generate providers and models sections with Qwen 3.5-9B configuration.

**Input:** None (providers and models are fixed for this project).

**Processing:**

1. LLM call to generate providers and models sections:
   ```
   Generate the providers and models sections for a YAML workflow file.
   
   Use the following fixed configuration:
   - Provider: llama_cpp_with_vulkan
   - Config file: ./providers/llama_cpp.yml
   - Model: primary-analyzer
   - Model file: Qwen3.5-9B-UD-Q4_K_XL.gguf
   - Model host type: llama_cpp_with_vulkan
   - Connection: localhost:8080
   
   Load parameters (fixed for Qwen 3.5-9B):
   - context_size: 262144
   - batch_size: 2048
   - ubatch_size: 512
   - cache_type_k: q8_0
   - cache_type_v: q8_0
   - gpu_layers: 99
   - threads: 5
   - use_mmap: true
   - flash_attn: true
   - cont_batching: false
   - no_cache_prompt: true
   - parallel: 1
   
   Sampling parameters (default):
   - temperature: 0.2
   - top_p: 0.9
   - top_k: 40
   - max_tokens: 4096
   
   Output in YAML format:
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
         gpu_layers: 99
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
   ```

2. Model configuration: temperature 0.05 (very deterministic), top_p 0.7 (conservative), max_tokens 3072

3. Output: Providers and models YAML sections

**Step 4: Steps Assembly**

**Purpose:** Merge all YAML skeleton structures into agentic_workflow.steps section.

**Input:** YAML skeleton contents from Step 1.

**Processing:**

1. LLM call to assemble steps section:
   ```
   Assemble the agentic_workflow.steps section from the following YAML skeleton structures.
   
   Skeleton structures:
   {{yaml_skeletons}}
   
   Assembly requirements:
   - Merge all skeletons into single agentic_workflow.steps section
   - Ensure all step names are unique
   - Preserve all requires dependencies
   - Preserve all when hooks
   - Ensure proper YAML indentation and nesting
   - Resolve any naming conflicts (e.g., duplicate step names)
   
   Output in YAML format:
   agentic_workflow:
     steps:
       <step_name_1>:
         generative_entity: "${models.primary-analyzer}"
         prompt: "<prompt text>"
         requires: [<step_dependencies>]
         when:
           <hook_configuration>
       
       <step_name_2>:
         generative_entity: "${models.primary-analyzer}"
         prompt: "<prompt text>"
         requires: [<step_dependencies>]
         when:
           <hook_configuration>
       
       ...
   ```

2. Model configuration: temperature 0.08 (very low temperature for assembly accuracy), top_p 0.75 (conservative), max_tokens 8192

3. Output: agentic_workflow.steps section

**Step 5: Execution Strategy Assembly**

**Purpose:** Generate workflow_execution_strategy section.

**Input:** None (execution strategy is fixed template).

**Processing:**

1. LLM call to generate execution strategy:
   ```
   Generate the workflow_execution_strategy section for a YAML workflow file.
   
   Use the following fixed configuration:
   - load_unload: one_at_a_time
   - timeout total: 4h
   - timeout tool_call: 30m
   - timeout step: 8m
   - timeout sub_workflow: 300s
   - timeout time_to_first_result_secs: 120
   - timeout_strategy: continue_with_partial
   - error_handling default_action type: retry
   - error_handling retry max_attempts_per_step: 3
   - error_handling retry max_attempts_per_workflow: 10
   - error_handling retry backoff_strategy type: exponential
   - error_handling retry backoff_strategy initial_delay: "1s"
   - error_handling retry backoff_strategy max_delay: "30s"
   - error_handling retry backoff_strategy multiplier: 2.0
   - error_handling retry backoff_strategy jitter: true
   
   Output in YAML format:
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
     error_handling:
       default_action:
         type: retry
       retry:
         max_attempts_per_step: 3
         max_attempts_per_workflow: 10
         backoff_strategy:
           type: exponential
           initial_delay: "1s"
           max_delay: "30s"
           multiplier: 2.0
           jitter: true
   ```

2. Model configuration: temperature 0.05 (very deterministic), top_p 0.7 (conservative), max_tokens 2048

3. Output: workflow_execution_strategy YAML section

**Step 6: Deterministic Post-Processing**

**Purpose:** Run fix and validation scripts to ensure YAML quality.

**Input:** Assembled workflow YAML from Steps 2-5.

**Processing:**

1. Shell action to run fix script:
```yaml
when:
  after_step_succeeds:
    - shell:
        command: "scripts/fix-generated-yaml.py --input {{inputs.workflow_path}}"
        fail_on_error: true
    - log:
        to_file_path: "./workspace/logs/sw5-postprocess.log"
        level: info
        message: "YAML fix script complete"
```

2. Shell action to run validation script:
```yaml
when:
  after_step_succeeds:
    - shell:
        command: "scripts/validate-yaml.py --schema docs/schema/unified-workflow-schema.yml --input {{inputs.workflow_path}}"
        fail_on_error: true
    - log:
        to_file_path: "./workspace/logs/sw5-postprocess.log"
        level: info
        message: "Schema validation complete, schema_valid=true"
```

3. Output: Fixed and validated workflow YAML

**Step 7: Exhaustive Review Loop**

**Purpose:** Verify all tasks from tasks.md appear in workflow YAML.

**Input:** tasks.md + assembled workflow YAML.

**Processing:**

1. LLM call to extract task names from tasks.md:
   ```
   Extract all task names from the following task hierarchy:
   {{bookmarks.tasks_md_content}}
   
   Output format:
   - Task 1: <task_name>
   - Task 2: <task_name>
   - ...
   ```

2. LLM call to extract step names from workflow YAML:
   ```
   Extract all step names from the following workflow YAML:
   {{assembled_workflow_yaml}}
   
   Output format:
   - Step 1: <step_name>
   - Step 2: <step_name>
   - ...
   ```

3. LLM call to compare and verify coverage:
   ```
   Compare the task names from the task hierarchy to the step names in the workflow YAML.
   
   Tasks from hierarchy:
   {{tasks_from_hierarchy}}
   
   Steps from workflow:
   {{steps_from_workflow}}
   
   Verify that:
   - Every task from hierarchy appears as a step in workflow
   - Step names correspond to task names (or are reasonable variations)
   - No tasks are missing from workflow
   - No extra steps appear in workflow (all steps correspond to tasks)
   
   Output: PASS or FAIL with specific discrepancies
   ```

4. If FAIL (missing tasks), route to fix step with specific task names

5. Output: Coverage verification result

**Step 8: Final Pass**

**Purpose:** Emit final workflow.yml with complete, validated content.

**Input:** Header + providers + models + steps + execution strategy from Steps 2-5 + post-processed YAML from Step 6.

**Processing:**

1. LLM call to assemble final workflow:
   ```
   Assemble the final workflow YAML file from the following components:
   
   Workflow header:
   {{workflow_header}}
   
   Providers and models:
   {{providers_and_models}}
   
   Steps:
   {{steps}}
   
   Execution strategy:
   {{execution_strategy}}
   
   Ensure:
   - Proper YAML indentation and nesting
   - All sections present and correctly formatted
   - No syntax errors
   - Ready for execution
   ```

2. Model configuration: temperature 0.05 (very deterministic), top_p 0.7 (conservative), max_tokens 8192

3. Write final workflow to `workflow.yml` output file

4. Output: Final workflow.yml ready for execution

### Evaluation Criteria

**SW5 Pass Conditions:**

1. **Schema Validity** — 100% schema_valid=true in logs
2. **Input Coverage** — 100% of tasks from tasks.md appear as steps in workflow
3. **YAML Parseable** — Workflow parses without serde_yaml errors
4. **Dependencies Correct** — All requires: references exist in workflow
5. **Hook Completeness** — All steps have appropriate hooks (≥ 2 triggers)
6. **Template Interpolation** — All {{...}} references resolve correctly
7. **Executable** — Workflow runs on live Docker system without errors

**SW5 Fail Conditions (any of these triggers iteration):**

1. **Schema Validation Fails** — Schema_valid=false in logs or unknown keys present
2. **Missing Tasks** — Some tasks from tasks.md missing from workflow steps
3. **YAML Syntax Errors** — Workflow doesn't parse (serde_yaml error)
4. **Broken Dependencies** — Some requires: references don't exist
5. **Missing Hooks** — Some steps lack appropriate hooks
6. **Broken Interpolation** — Some {{...}} references fail to resolve
7. **Execution Failure** — Workflow panics or crashes on live system

### Fix Loops

**Fix Loop 1: Missing Tasks**

**Trigger:** Tasks from tasks.md missing from workflow steps

**Fix Strategy:**
1. LLM call to add missing steps:
   ```
   The following tasks from tasks.md are missing from the workflow YAML:
   {{missing_tasks}}
   
   For each missing task, generate a YAML step structure using the category-specific guidance and include it in the workflow.
   Ensure the step:
   - Has proper YAML structure (agentic_workflow.steps.<step_name>:)
   - Uses ${models.primary-analyzer} as model reference
   - Has appropriate hooks for the task's category
   - Includes requires dependencies on prior steps (if any)
   ```

2. Add missing steps to workflow YAML

3. Re-run exhaustive review to verify all tasks present

**Fix Loop 2: Broken Dependencies**

**Trigger:** Some requires: references don't exist

**Fix Strategy:**
1. LLM call to fix dependencies:
   ```
   The following steps have requires: references to non-existent steps:
   {{steps_with_broken_dependencies}}
   
   For each broken reference, either:
   - Remove the invalid dependency if the step doesn't actually require it
   - Fix the reference to point to an existing step
   - Add the missing step if it was accidentally omitted
   
   Ensure all requires: references point to existing steps in the workflow.
   ```

2. Fix broken dependency references

3. Re-validate all dependencies are correct

**Fix Loop 3: Missing Hooks**

**Trigger:** Steps lack appropriate hooks

**Fix Strategy:**
1. LLM call to add missing hooks:
   ```
   The following steps lack appropriate hook configurations for their categories:
   {{steps_with_missing_hooks}}
   
   For each step, add appropriate hooks based on its category:
   
   Category-specific hook patterns:
   - file-read: after_step_succeeds with save_to + log
   - transform-llm: after_step_succeeds with save_to + gwt
   - validate-gate: after_step_succeeds with gwt
   - branch-decision: after_step_succeeds with gwt + route_to
   - shell-execute: after_step_succeeds with shell + log
   - loop-iterate: after_step_succeeds with gwt + bookmark
   - tool-call: after_step_succeeds with save_to + log
   - checkpoint-state: after_step_succeeds with bookmark + save_to
   - notify-external: after_step_succeeds with notify + log
   - sub-workflow-ref: after_step_succeeds with route_to + log
   
   Ensure each step has at least 2 triggers configured.
   ```

2. Add missing hooks to steps

3. Re-validate all steps have appropriate hooks

### Chunk Sizing Strategy

SW5 processes all steps at once (no chunking):

**No Chunking:**

- All steps processed in single LLM call
- Context window (262144) enables processing of complex workflows
- Simplifies assembly (no chunk merging required)
- Enables comprehensive exhaustive review in single pass

**Why No Chunking:**

- Workflow assembly benefits from seeing all steps together
- Dependency resolution requires complete step list
- Category distribution easier to analyze holistically
- Reduces complexity (no chunk boundary management)

### Model Configuration Reference

**LoadParams for SW5:**

```yaml
load_params:
  context_size: 262144          # Full context window
  batch_size: 2048               # Default batch size
  ubatch_size: 512               # Default micro-batch size
  cache_type_k: "q8_0"           # Q8_0 KV cache K
  cache_type_v: "q8_0"           # Q8_0 KV cache V
  gpu_layers: 99                  # full GPU offload (benchmark-proven fastest on RX580)
  threads: 5                     # 5 CPU threads
  use_mmap: true                 # Memory-mapped file loading
  flash_attn: true               # Flash attention (safe for Vulkan)
  cont_batching: false            # Continuous batching disabled (Vulkan requirement)
  no_cache_prompt: true           # Prompt caching disabled (Vulkan requirement)
  parallel: 1                    # Single parallel slot
```

**Sampling Parameters for SW5:**

```yaml
sampling:
  temperature: 0.08              # Very low temperature for assembly accuracy
  top_p: 0.75                    # Lower than default for precise assembly
  top_k: 40                      # Default top-k sampling
  max_tokens: 8192               # 2x default for complete workflow assembly
  repeat_penalty: 1.0            # No repetition penalty (default)
  seed: null                      # Random seed (default)
```

**Why This Configuration:**

- Very low temperature (0.08) maximizes assembly accuracy, preserves skeleton structures
- Lower top_p (0.75) provides more conservative token selection for precise assembly
- Increased max_tokens (8192) allows complete workflow assembly for complex prompts
- Default context (262144) enables processing of large task trees without truncation

---

**Document Status:** Draft  
**Last Updated:** 2026-06-17  
**Author:** Sisyphus-Junior (Whitt Execution Engine Planning)  
**Review Status:** Ready for Execution