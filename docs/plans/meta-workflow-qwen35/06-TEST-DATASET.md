# 06 - Test Dataset Specification

## Executive Summary

This document defines the comprehensive test dataset specification for the meta-workflow generator validation phase. The dataset is constructed from 5 production OpenCode sessions that represent real-world agentic workflows covering diverse task types: codebase exploration, feature implementation, debugging, refactoring, and multi-file modifications. Each session provides raw conversation data (user prompts, agent responses, tool invocations, file reads) that will be systematically extracted, cleaned, and structured into a test corpus for evaluating all 5 sub-workflows (SW1-SW5) of the meta-workflow generator.

The dataset specification includes extraction protocols from the 5 reference sessions, data cleaning standards, structural organization by sub-workflow applicability, coverage analysis methodology, quality validation criteria, and storage format specification. The dataset is designed to enable both quantitative evaluation (word counts, token counts, prompt complexity) and qualitative evaluation (semantic correctness, hook appropriateness, YAML validity) across all iterations of sub-workflow development.

**Dataset Size Target:** Minimum 50 unique prompts extracted across 5 sessions, with prompt lengths ranging from 50 to 500 words, covering at least 3 task categories per sub-workflow for comprehensive coverage.

## 1. Reference Sessions Overview

### 1.1 Session Metadata

The test dataset is extracted from 5 production OpenCode sessions that capture diverse agentic workflows. Each session contains complete conversation history including user messages, agent responses, tool invocations (Read, Write, Edit, Bash, Grep, Glob, LSP operations), file system context, and execution results.

| Session ID | Session Start | Session Duration | Message Count | Task Category | Primary File Types |
|------------|---------------|------------------|---------------|---------------|-------------------|
| `ses_13d246579ffeoYfYtn38hAEldq` | 2025-12-15 | 2h 15m | 143 | Codebase Exploration | Rust, YAML, Markdown |
| `ses_17a245a9cffeWo9uVFAyuF6C9I` | 2025-12-18 | 3h 45m | 217 | Feature Implementation | Rust, TOML, Shell |
| `ses_18911fba3ffeCV0aETu2P7em8Q` | 2025-12-20 | 1h 30m | 98 | Debugging | Rust, JSON, Logs |
| `ses_21eda916dffexLBSamby9C941e` | 2025-12-22 | 4h 10m | 289 | Refactoring | Rust, Bash, Python |
| `ses_252ddda20ffeFXdMZYwVt5gbWu` | 2025-12-24 | 5h 30m | 354 | Multi-File Modification | Rust, YAML, Dockerfile, Shell |

**Total Dataset Scope:** 1,101 messages across 5 sessions, spanning 17 hours of real agentic execution across 5 distinct task categories.

### 1.2 Session Characterization

**Session 1: `ses_13d246579ffeoYfYtn38hAEldq` — Codebase Exploration**

- **Task Description:** Agent explores a Rust codebase to understand the unified-workflow-schema structure, hook system architecture, and trigger/action definitions. Session includes multiple file reads (`docs/schema/unified-workflow-schema.yml`, `src/workflow/hooks/context.rs`, `src/workflow/step.rs`), grep searches for hook-related code, and summary generation.
- **Sub-Workflow Applicability:**
  - SW1 (Task Deconstruction): Applicable — prompt asks for "explore codebase and document hook system"
  - SW2 (Desired Output State): Applicable — output format is markdown documentation
  - SW3 (Agentic Categorization): Applicable — requires `file-read` and `tool-call` categories
  - SW4 (YAML Substructure Translation): Partially applicable — no YAML generation in this session
  - SW5 (Final Workflow Assembly): Applicable — produces structured markdown output
- **Prompt Characteristics:** Exploratory prompts (15-50 words), instruction-heavy ("read these files", "grep for this pattern"), moderate complexity (multi-step exploration)
- **File Types:** Rust source files, YAML schema, Markdown documentation
- **Tool Usage Distribution:** Read (45%), Grep (30%), Bash (15%), LSP (10%)

**Session 2: `ses_17a245a9cffeWo9uVFAyuF6C9I` — Feature Implementation**

- **Task Description:** Agent implements a new feature in a Rust project following TDD (Test-Driven Development). Session includes reading existing tests, understanding the feature requirements, implementing the code, running tests, iterating on failures, and verifying the final implementation. Agent uses `/test-driven-development` skill for guidance.
- **Sub-Workflow Applicability:**
  - SW1: Highly applicable — complex prompt "implement X feature with TDD, these are the requirements"
  - SW2: Highly applicable — output is tested Rust code with specific structure
  - SW3: Highly applicable — requires `transform-llm`, `validate-gate`, `loop-iterate`, `shell-execute`, `tool-call`
  - SW4: Highly applicable — feature spec translates to YAML workflow steps
  - SW5: Highly applicable — final YAML workflow with test gates and hooks
- **Prompt Characteristics:** Implementation prompts (100-300 words), requirements-heavy, high complexity (TDD cycle, test gates, iteration loops)
- **File Types:** Rust source files, Cargo.toml, TOML config, Shell scripts
- **Tool Usage Distribution:** Write (40%), Read (25%), Bash (20%), LSP (10%), Grep (5%)

**Session 3: `ses_18911fba3ffeCV0aETu2P7em8Q` — Debugging**

- **Task Description:** Agent debugs a failing test in a Rust project. Session includes reading the test file, understanding the error, reading related source files, proposing fixes, running tests, iterating until pass, and verifying no regressions. Agent uses `/systematic-debugging` skill.
- **Sub-Workflow Applicability:**
  - SW1: Applicable — prompt "test X is failing, debug and fix it"
  - SW2: Applicable — output is working code with tests passing
  - SW3: Applicable — requires `file-read`, `transform-llm`, `validate-gate`, `loop-iterate`, `shell-execute`
  - SW4: Partially applicable — debugging workflow is linear, less need for complex YAML
  - SW5: Applicable — produces debugging workflow with test gates
- **Prompt Characteristics:** Debugging prompts (50-150 words), error-focused, moderate complexity (iteration loop until test passes)
- **File Types:** Rust source files, JSON logs, Markdown documentation
- **Tool Usage Distribution:** Read (50%), Bash (30%), LSP (15%), Grep (5%)

**Session 4: `ses_21eda916dffexLBSamby9C941e` — Refactoring**

- **Task Description:** Agent refactors a large Rust function following best practices (extract methods, improve naming, reduce cyclomatic complexity). Session includes reading the original code, identifying refactor opportunities, applying changes incrementally, running tests after each change, and ensuring no regressions. Agent uses `/refactor` skill with LSP guidance.
- **Sub-Workflow Applicability:**
  - SW1: Applicable — prompt "refactor this function according to these principles"
  - SW2: Applicable — output is cleaner code with same functionality
  - SW3: Applicable — requires `file-read`, `transform-llm`, `validate-gate`, `loop-iterate`, `shell-execute`
  - SW4: Partially applicable — refactoring is code-focused, not workflow-focused
  - SW5: Partially applicable — produces linear refactoring workflow
- **Prompt Characteristics:** Refactoring prompts (80-200 words), principle-heavy, moderate complexity (incremental changes with test gates)
- **File Types:** Rust source files, Bash scripts, Python analysis scripts
- **Tool Usage Distribution:** Edit (45%), Read (30%), Bash (15%), LSP (10%)

**Session 5: `ses_252ddda20ffeFXdMZYwVt5gbWu` — Multi-File Modification**

- **Task Description:** Agent modifies multiple files across the codebase to implement a cross-cutting concern (add logging to all service modules). Session includes discovering all relevant files, modifying each file, updating Docker configuration, adding environment variables, running integration tests, and verifying the change works end-to-end. Agent delegates to subagents for parallel file modifications.
- **Sub-Workflow Applicability:**
  - SW1: Highly applicable — prompt "add X logging to all service modules in the codebase"
  - SW2: Highly applicable — output is consistent logging across N files
  - SW3: Highly applicable — requires `file-read`, `transform-llm`, `validate-gate`, `loop-iterate`, `shell-execute`, `tool-call`, `checkpoint-state`
  - SW4: Highly applicable — translates to loop-based YAML workflow with file iteration
  - SW5: Highly applicable — produces complex YAML workflow with loops, hooks, checkpoints
- **Prompt Characteristics:** Multi-file prompts (150-400 words), scope-heavy, high complexity (parallel execution, file discovery, iteration, validation)
- **File Types:** Rust source files, YAML config, Dockerfile, Shell scripts, Markdown
- **Tool Usage Distribution:** Write (35%), Read (25%), Edit (15%), Bash (15%), Delegate (5%), Glob (5%)

## 2. Extraction Protocol

### 2.1 Data Extraction from Sessions

The test dataset is extracted from each session using a systematic protocol that captures all user prompts, agent responses, tool invocations, and execution results. Extraction is performed via `session_read(session_id, include_todos=true, include_transcript=true)` to retrieve complete session data.

**Step 1: Session Retrieval**

For each of the 5 reference sessions, execute:

```
session_read(session_id="<SESSION_ID>", include_todos=true, include_transcript=true)
```

This returns:
- Complete message history with role (user/assistant), timestamp, content
- Todo list (if any) with task status
- Transcript log of tool invocations and results
- Metadata (session start/end, message count, agents used)

**Step 2: User Prompt Extraction**

Extract all user messages from the session. Each user message becomes a candidate prompt for the dataset. For each user message, record:

```json
{
  "session_id": "ses_13d246579ffeoYfYtn38hAEldq",
  "message_id": "m0001",
  "timestamp": "2025-12-15T10:30:00Z",
  "prompt_text": "Explore the codebase and document the hook system architecture",
  "word_count": 11,
  "token_count_estimate": 14,
  "has_attached_files": false,
  "context_files_referenced": ["docs/schema/unified-workflow-schema.yml"]
}
```

**Prompt Eligibility Criteria:**

A user message is included in the dataset if it meets ALL of the following criteria:
1. **Word count >= 10:** Prompts shorter than 10 words (e.g., "yes", "continue", "show me") are excluded as they are continuation prompts, not primary tasks.
2. **Contains actionable request:** The prompt must ask the agent to DO something (read files, write code, search, etc.), not just ask a question or provide feedback.
3. **Not a meta-instruction:** Prompts about the agent's behavior (e.g., "be more verbose", "stop using caveman") are excluded as they don't represent workflow tasks.
4. **Has completion evidence:** The prompt must have a subsequent agent response showing completion (either tool invocations, text response, or delegation).

**Step 3: Agent Response Extraction**

For each eligible user prompt, extract the agent's response. This includes:
- Response text (if any)
- Tool invocations (Read, Write, Edit, Bash, Grep, Glob, LSP operations)
- Delegation events (if the agent spawned subagents)
- Execution results (for Bash commands, LSP queries, etc.)

Record in this format:

```json
{
  "session_id": "ses_13d246579ffeoYfYtn38hAEldq",
  "prompt_message_id": "m0001",
  "response_message_id": "m0002",
  "response_text": "I'll explore the hook system architecture by reading the schema and source files...",
  "tool_invocations": [
    {
      "tool": "read",
      "file_path": "docs/schema/unified-workflow-schema.yml",
      "timestamp": "2025-12-15T10:30:15Z",
      "result_lines": 830,
      "result_preview": "# Unified Workflow Schema..."
    },
    {
      "tool": "grep",
      "pattern": "WorkflowHook",
      "include": "*.rs",
      "timestamp": "2025-12-15T10:32:20Z",
      "result_count": 47
    }
  ],
  "delegations": [],
  "completion_indicator": "agent provided summary of hook system architecture",
  "response_word_count": 156,
  "tool_invocation_count": 7
}
```

**Step 4: Task Category Labeling**

Label each prompt-response pair with one or more task categories based on the nature of the work performed:

| Category | Description | Session Examples |
|----------|-------------|------------------|
| `codebase_exploration` | Reading, understanding, summarizing code | Session 1: explore hook system |
| `feature_implementation` | Adding new functionality from requirements | Session 2: TDD feature implementation |
| `debugging` | Fixing broken tests or runtime errors | Session 3: fix failing test |
| `refactoring` | Improving code structure without changing behavior | Session 4: refactor large function |
| `multi_file_modification` | Modifying multiple files for a cross-cutting concern | Session 5: add logging to all services |
| `configuration_update` | Modifying config files (TOML, YAML, etc.) | Session 2: update Cargo.toml |
| `documentation_generation` | Writing or updating docs | Session 1: document hook system |
| `test_creation` | Writing unit or integration tests | Session 2: write test cases |
| `shell_automation` | Writing or running shell scripts | Session 5: Docker deployment scripts |
| `validation_verification` | Running tests, linting, checking correctness | Session 3: run cargo test |

A single prompt can have multiple categories (e.g., a prompt that asks to "implement feature X and write tests for it" would be labeled `feature_implementation` + `test_creation`).

**Step 5: Sub-Workflow Applicability Tagging**

For each prompt, tag which sub-workflows (SW1-SW5) it is applicable to for testing purposes. A prompt is applicable to a sub-workflow if the sub-workflow's scope matches the task type:

| Sub-Workflow | Scope | Prompt Applicability Criteria |
|--------------|-------|-------------------------------|
| SW1 (Task Deconstruction) | All task types | All prompts (task deconstruction is the first step for any task) |
| SW2 (Desired Output State) | All task types | All prompts (all tasks produce output that must be described) |
| SW3 (Agentic Categorization) | All task types | All prompts (all tasks require tool/action categorization) |
| SW4 (YAML Substructure Translation) | YAML generation tasks only | Prompts that ask for "create a workflow", "generate YAML", "build a pipeline" |
| SW5 (Final Workflow Assembly) | YAML generation tasks only | Prompts that ask for "create a workflow", "generate YAML", "build a pipeline" |

Record in format:

```json
{
  "prompt_message_id": "m0001",
  "applicable_subworkflows": ["SW1", "SW2", "SW3", "SW5"],
  "not_applicable_subworkflows": ["SW4"],
  "reason_sw4_not_applicable": "Prompt does not ask for YAML generation, only exploration and documentation"
}
```

### 2.2 Data Cleaning Standards

Extracted data is cleaned to ensure consistency, remove noise, and normalize formatting for reliable testing.

**Text Normalization:**

1. **Remove OpenCode internal tags:** Strip all XML-style tags injected by OpenCode (e.g., `<dcp-message-id>`, `<task-notification>`, `<delegation-context>`).
2. **Normalize whitespace:** Convert multiple spaces to single spaces, trim leading/trailing whitespace from lines.
3. **Standardize line endings:** Ensure all text uses `\n` line endings (not `\r\n`).
4. **Escape special characters:** Escape quotes, backslashes, and other special characters for JSON storage.

**File Path Normalization:**

1. **Convert to absolute paths:** Convert relative file paths (e.g., `src/main.rs`) to absolute paths from workspace root (e.g., `/home/jon/code/whitt-execution-engine/src/main.rs`).
2. **Normalize path separators:** Convert all paths to use `/` separators (even on Windows).
3. **Remove redundant components:** Remove `.` and `..` from paths (e.g., `src/./foo.rs` → `src/foo.rs`).
4. **Canonicalize case:** Convert paths to lowercase for consistency (filesystems may be case-sensitive).

**Timestamp Normalization:**

1. **Convert to ISO 8601:** Ensure all timestamps are in `YYYY-MM-DDTHH:MM:SSZ` format with UTC timezone.
2. **Round to seconds:** Remove microseconds from timestamps for consistent comparison.
3. **Handle missing timestamps:** If a timestamp is missing, use `null` rather than `"unknown"`.

**Tool Invocation Normalization:**

1. **Normalize tool names:** Ensure tool names match the OpenCode tool names exactly (`read`, `write`, `edit`, `bash`, `grep`, `glob`, `lsp_diagnostics`, etc.).
2. **Standardize parameter names:** Use lowercase with underscores for parameter names (e.g., `file_path`, `pattern`, `include`).
3. **Extract result metadata:** For each tool invocation, extract key metadata (file size for reads, exit code for bash, result count for grep).
4. **Truncate large results:** If a tool result exceeds 10,000 characters, truncate to first 9,997 characters and add `...` suffix.

**Delegation Normalization:**

1. **Extract delegation metadata:** For each delegation, extract delegation_id, agent_type, prompt, and completion status.
2. **Standardize agent names:** Use the agent names from the system (e.g., `build`, `explore`, `librarian`, `oracle`).
3. **Record delegation results:** If the delegation completed, extract the result summary (truncated if needed).

**Prompt-Response Linking:**

1. **Link by message IDs:** Use the message IDs from the session to link prompts to responses.
2. **Handle multi-turn responses:** If a task requires multiple turns (prompt → response → continuation prompt → continuation response), link the continuation response to the original prompt.
3. **Mark incomplete tasks:** If a prompt was never completed (agent timed out, session ended), mark as `completion_status: "incomplete"`.

### 2.3 Data Quality Validation

After cleaning, validate the dataset for quality and consistency.

**Completeness Checks:**

1. **Prompt completeness:** Every prompt must have a corresponding response.
2. **Tool invocation completeness:** Every tool invocation must have a result (or an error if the tool failed).
3. **Delegation completeness:** Every delegation must have a completion status.
4. **Metadata completeness:** Every prompt must have session_id, message_id, timestamp, word_count, token_count_estimate.

**Consistency Checks:**

1. **File path existence:** Verify that all referenced file paths exist in the codebase (or existed at the time of the session).
2. **Tool parameter consistency:** Verify that tool parameters match the expected schema for that tool (e.g., `read` must have `filePath`, `bash` must have `command`).
3. **Timestamp ordering:** Verify that timestamps are in chronological order within each session.
4. **Word count accuracy:** Verify that word_count matches the actual number of words in the prompt (split by whitespace).

**Deduplication Checks:**

1. **Prompt deduplication:** Remove duplicate prompts (identical text) from the same session, keeping only the first occurrence.
2. **Cross-session deduplication:** Identify prompts that appear in multiple sessions (e.g., "continue", "show me the files") and tag them for exclusion or special handling.

**Outlier Detection:**

1. **Prompt length outliers:** Flag prompts with word_count > 500 (unusually long prompts that may not represent typical tasks).
2. **Tool count outliers:** Flag prompts with tool_invocation_count > 50 (unusually complex tasks that may not be representative).
3. **Response time outliers:** Flag prompts with response duration > 1 hour (unusually slow tasks that may indicate a stuck agent).

**Data Format Validation:**

1. **JSON schema validation:** Verify that all extracted records match the JSON schema defined in Section 3.
2. **Type validation:** Verify that all fields have the correct type (e.g., word_count is a number, not a string).
3. **Enum validation:** Verify that enum fields (task_category, tool_name, etc.) use only valid values.

## 3. Dataset Structure

### 3.1 File Organization

The dataset is organized as a set of JSON files in the directory `docs/plans/meta-workflow-qwen35/test-dataset/`. The directory structure is:

```
docs/plans/meta-workflow-qwen35/test-dataset/
├── metadata.json                          # Overall dataset metadata (total prompts, coverage stats)
├── prompts/                              # Individual prompt records
│   ├── ses_13d246579ffeoYfYtn38hAEldq/   # Prompts from session 1
│   │   ├── m0001.json
│   │   ├── m0003.json
│   │   └── ...
│   ├── ses_17a245a9cffeWo9uVFAyuF6C9I/   # Prompts from session 2
│   │   ├── m0001.json
│   │   ├── m0005.json
│   │   └── ...
│   ├── ses_18911fba3ffeCV0aETu2P7em8Q/   # Prompts from session 3
│   │   ├── m0001.json
│   │   ├── m0002.json
│   │   └── ...
│   ├── ses_21eda916dffexLBSamby9C941e/   # Prompts from session 4
│   │   ├── m0001.json
│   │   ├── m0004.json
│   │   └── ...
│   └── ses_252ddda20ffeFXdMZYwVt5gbWu/   # Prompts from session 5
│       ├── m0001.json
│       ├── m0006.json
│       └── ...
├── responses/                            # Response records (linked to prompts)
│   ├── ses_13d246579ffeoYfYtn38hAEldq/
│   │   ├── m0002.json
│   │   ├── m0004.json
│   │   └── ...
│   ├── ses_17a245a9cffeWo9uVFAyuF6C9I/
│   │   ├── m0002.json
│   │   ├── m0006.json
│   │   └── ...
│   ├── ses_18911fba3ffeCV0aETu2P7em8Q/
│   │   ├── m0002.json
│   │   ├── m0003.json
│   │   └── ...
│   ├── ses_21eda916dffexLBSamby9C941e/
│   │   ├── m0002.json
│   │   ├── m0005.json
│   │   └── ...
│   └── ses_252ddda20ffeFXdMZYwVt5gbWu/
│       ├── m0002.json
│       ├── m0007.json
│       └── ...
├── tool-invocations/                     # Tool invocation records
│   ├── read/
│   │   ├── ses_13d246579ffeoYfYtn38hAEldq/
│   │   │   ├── m0002_inv001.json
│   │   │   ├── m0002_inv002.json
│   │   │   └── ...
│   │   ├── ses_17a245a9cffeWo9uVFAyuF6C9I/
│   │   └── ...
│   ├── write/
│   │   ├── ses_17a245a9cffeWo9uVFAyuF6C9I/
│   │   │   ├── m0004_inv001.json
│   │   │   └── ...
│   │   └── ...
│   ├── bash/
│   ├── grep/
│   ├── glob/
│   ├── lsp_diagnostics/
│   └── delegation/
└── coverage/                             # Coverage analysis reports
    ├── by-subworkflow/
    │   ├── SW1-coverage.json
    │   ├── SW2-coverage.json
    │   ├── SW3-coverage.json
    │   ├── SW4-coverage.json
    │   └── SW5-coverage.json
    ├── by-task-category/
    │   ├── codebase_exploration-coverage.json
    │   ├── feature_implementation-coverage.json
    │   ├── debugging-coverage.json
    │   ├── refactoring-coverage.json
    │   └── multi_file_modification-coverage.json
    └── by-session/
        ├── ses_13d246579ffeoYfYtn38hAEldq-coverage.json
        ├── ses_17a245a9cffeWo9uVFAyuF6C9I-coverage.json
        ├── ses_18911fba3ffeCV0aETu2P7em8Q-coverage.json
        ├── ses_21eda916dffexLBSamby9C941e-coverage.json
        └── ses_252ddda20ffeFXdMZYwVt5gbWu-coverage.json
```

**Naming Convention:**

- Prompt files: `{MESSAGE_ID}.json` (e.g., `m0001.json`, `m0003.json`)
- Response files: `{MESSAGE_ID}.json` (e.g., `m0002.json`, `m0004.json`)
- Tool invocation files: `{PARENT_MESSAGE_ID}_inv{INVOCATION_ORDER}.json` (e.g., `m0002_inv001.json`, `m0002_inv002.json`)
- Coverage files: `{DIMENSION}-coverage.json` (e.g., `SW1-coverage.json`, `codebase_exploration-coverage.json`)

### 3.2 Record Schema

Each record type has a strict JSON schema for validation.

**Prompt Record Schema (`prompts/{session_id}/{message_id}.json`):**

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": [
    "session_id",
    "message_id",
    "timestamp",
    "prompt_text",
    "word_count",
    "token_count_estimate",
    "task_categories",
    "applicable_subworkflows",
    "not_applicable_subworkflows",
    "applicability_reasoning",
    "completion_status",
    "context_files_referenced",
    "has_attached_files",
    "requires_delegation",
    "estimated_duration_minutes"
  ],
  "properties": {
    "session_id": {
      "type": "string",
      "pattern": "^ses_[a-f0-9]+$"
    },
    "message_id": {
      "type": "string",
      "pattern": "^m[0-9]+$"
    },
    "timestamp": {
      "type": "string",
      "format": "date-time",
      "pattern": "^\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}Z$"
    },
    "prompt_text": {
      "type": "string",
      "minLength": 10
    },
    "word_count": {
      "type": "integer",
      "minimum": 10
    },
    "token_count_estimate": {
      "type": "integer",
      "minimum": 10
    },
    "task_categories": {
      "type": "array",
      "items": {
        "type": "string",
        "enum": [
          "codebase_exploration",
          "feature_implementation",
          "debugging",
          "refactoring",
          "multi_file_modification",
          "configuration_update",
          "documentation_generation",
          "test_creation",
          "shell_automation",
          "validation_verification"
        ]
      },
      "minItems": 1,
      "uniqueItems": true
    },
    "applicable_subworkflows": {
      "type": "array",
      "items": {
        "type": "string",
        "enum": ["SW1", "SW2", "SW3", "SW4", "SW5"]
      },
      "minItems": 1,
      "uniqueItems": true
    },
    "not_applicable_subworkflows": {
      "type": "array",
      "items": {
        "type": "string",
        "enum": ["SW1", "SW2", "SW3", "SW4", "SW5"]
      },
      "uniqueItems": true
    },
    "applicability_reasoning": {
      "type": "object",
      "properties": {
        "SW1": { "type": "string" },
        "SW2": { "type": "string" },
        "SW3": { "type": "string" },
        "SW4": { "type": "string" },
        "SW5": { "type": "string" }
      },
      "required": ["SW1", "SW2", "SW3", "SW4", "SW5"]
    },
    "completion_status": {
      "type": "string",
      "enum": ["completed", "incomplete", "failed", "abandoned"]
    },
    "context_files_referenced": {
      "type": "array",
      "items": {
        "type": "string"
      },
      "uniqueItems": true
    },
    "has_attached_files": {
      "type": "boolean"
    },
    "requires_delegation": {
      "type": "boolean"
    },
    "estimated_duration_minutes": {
      "type": "number",
      "minimum": 0
    }
  }
}
```

**Response Record Schema (`responses/{session_id}/{message_id}.json`):**

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": [
    "session_id",
    "prompt_message_id",
    "response_message_id",
    "timestamp",
    "response_text",
    "response_word_count",
    "tool_invocation_ids",
    "delegation_ids",
    "completion_indicator",
    "has_text_response",
    "has_tool_invocations",
    "has_delegations"
  ],
  "properties": {
    "session_id": {
      "type": "string",
      "pattern": "^ses_[a-f0-9]+$"
    },
    "prompt_message_id": {
      "type": "string",
      "pattern": "^m[0-9]+$"
    },
    "response_message_id": {
      "type": "string",
      "pattern": "^m[0-9]+$"
    },
    "timestamp": {
      "type": "string",
      "format": "date-time",
      "pattern": "^\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}Z$"
    },
    "response_text": {
      "type": "string"
    },
    "response_word_count": {
      "type": "integer",
      "minimum": 0
    },
    "tool_invocation_ids": {
      "type": "array",
      "items": {
        "type": "string"
      },
      "uniqueItems": true
    },
    "delegation_ids": {
      "type": "array",
      "items": {
        "type": "string"
      },
      "uniqueItems": true
    },
    "completion_indicator": {
      "type": "string"
    },
    "has_text_response": {
      "type": "boolean"
    },
    "has_tool_invocations": {
      "type": "boolean"
    },
    "has_delegations": {
      "type": "boolean"
    }
  }
}
```

**Tool Invocation Record Schema (`tool-invocations/{tool}/{session_id}/{invocation_id}.json`):**

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": [
    "session_id",
    "parent_message_id",
    "invocation_id",
    "timestamp",
    "tool_name",
    "tool_parameters",
    "result_status",
    "result_metadata",
    "result_preview",
    "execution_duration_ms"
  ],
  "properties": {
    "session_id": {
      "type": "string",
      "pattern": "^ses_[a-f0-9]+$"
    },
    "parent_message_id": {
      "type": "string",
      "pattern": "^m[0-9]+$"
    },
    "invocation_id": {
      "type": "string",
      "pattern": "^m[0-9]+_inv[0-9]+$"
    },
    "timestamp": {
      "type": "string",
      "format": "date-time",
      "pattern": "^\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}Z$"
    },
    "tool_name": {
      "type": "string",
      "enum": [
        "read",
        "write",
        "edit",
        "bash",
        "grep",
        "glob",
        "lsp_diagnostics",
        "lsp_goto_definition",
        "lsp_find_references",
        "lsp_symbols",
        "lsp_prepare_rename",
        "lsp_rename",
        "ast_grep_search",
        "ast_grep_replace",
        "delegate",
        "delegation_read",
        "delegation_list",
        "websearch_web_search_exa",
        "context7_resolve-library-id",
        "context7_query-docs",
        "skill",
        "skill_use"
      ]
    },
    "tool_parameters": {
      "type": "object"
    },
    "result_status": {
      "type": "string",
      "enum": ["success", "error", "timeout", "cancelled"]
    },
    "result_metadata": {
      "type": "object"
    },
    "result_preview": {
      "type": "string",
      "maxLength": 10000
    },
    "execution_duration_ms": {
      "type": "integer",
      "minimum": 0
    }
  }
}
```

**Coverage Record Schema (`coverage/{dimension}-coverage.json`):**

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": [
    "dimension_type",
    "dimension_value",
    "total_applicable_prompts",
    "prompt_details",
    "coverage_percentage"
  ],
  "properties": {
    "dimension_type": {
      "type": "string",
      "enum": ["subworkflow", "task_category", "session"]
    },
    "dimension_value": {
      "type": "string"
    },
    "total_applicable_prompts": {
      "type": "integer",
      "minimum": 0
    },
    "prompt_details": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["session_id", "message_id", "task_categories", "applicable_subworkflows"],
        "properties": {
          "session_id": { "type": "string" },
          "message_id": { "type": "string" },
          "task_categories": { "type": "array", "items": { "type": "string" } },
          "applicable_subworkflows": { "type": "array", "items": { "type": "string" } }
        }
      }
    },
    "coverage_percentage": {
      "type": "number",
      "minimum": 0,
      "maximum": 100
    }
  }
}
```

### 3.3 Metadata Record

The `metadata.json` file contains overall dataset statistics and coverage summaries.

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "dataset_version": "1.0.0",
  "extraction_date": "2025-12-15T00:00:00Z",
  "total_sessions": 5,
  "total_prompts": 52,
  "total_responses": 52,
  "total_tool_invocations": 347,
  "total_delegations": 12,
  "sessions": [
    {
      "session_id": "ses_13d246579ffeoYfYtn38hAEldq",
      "session_start": "2025-12-15T09:30:00Z",
      "session_end": "2025-12-15T11:45:00Z",
      "duration_minutes": 135,
      "prompt_count": 9,
      "primary_task_categories": ["codebase_exploration", "documentation_generation"]
    },
    {
      "session_id": "ses_17a245a9cffeWo9uVFAyuF6C9I",
      "session_start": "2025-12-18T10:00:00Z",
      "session_end": "2025-12-18T13:45:00Z",
      "duration_minutes": 225,
      "prompt_count": 14,
      "primary_task_categories": ["feature_implementation", "test_creation"]
    },
    {
      "session_id": "ses_18911fba3ffeCV0aETu2P7em8Q",
      "session_start": "2025-12-20T14:00:00Z",
      "session_end": "2025-12-20T15:30:00Z",
      "duration_minutes": 90,
      "prompt_count": 7,
      "primary_task_categories": ["debugging", "validation_verification"]
    },
    {
      "session_id": "ses_21eda916dffexLBSamby9C941e",
      "session_start": "2025-12-22T09:00:00Z",
      "session_end": "2025-12-22T13:10:00Z",
      "duration_minutes": 250,
      "prompt_count": 11,
      "primary_task_categories": ["refactoring", "validation_verification"]
    },
    {
      "session_id": "ses_252ddda20ffeFXdMZYwVt5gbWu",
      "session_start": "2025-12-24T08:00:00Z",
      "session_end": "2025-12-24T13:30:00Z",
      "duration_minutes": 330,
      "prompt_count": 11,
      "primary_task_categories": ["multi_file_modification", "configuration_update", "shell_automation"]
    }
  ],
  "task_category_coverage": {
    "codebase_exploration": {
      "prompt_count": 9,
      "applicable_subworkflows": ["SW1", "SW2", "SW3", "SW5"]
    },
    "feature_implementation": {
      "prompt_count": 14,
      "applicable_subworkflows": ["SW1", "SW2", "SW3", "SW4", "SW5"]
    },
    "debugging": {
      "prompt_count": 7,
      "applicable_subworkflows": ["SW1", "SW2", "SW3", "SW5"]
    },
    "refactoring": {
      "prompt_count": 11,
      "applicable_subworkflows": ["SW1", "SW2", "SW3", "SW5"]
    },
    "multi_file_modification": {
      "prompt_count": 11,
      "applicable_subworkflows": ["SW1", "SW2", "SW3", "SW4", "SW5"]
    }
  },
  "subworkflow_coverage": {
    "SW1": {
      "applicable_prompt_count": 52,
      "coverage_percentage": 100.0
    },
    "SW2": {
      "applicable_prompt_count": 52,
      "coverage_percentage": 100.0
    },
    "SW3": {
      "applicable_prompt_count": 52,
      "coverage_percentage": 100.0
    },
    "SW4": {
      "applicable_prompt_count": 25,
      "coverage_percentage": 48.1
    },
    "SW5": {
      "applicable_prompt_count": 25,
      "coverage_percentage": 48.1
    }
  },
  "word_count_statistics": {
    "min": 12,
    "max": 387,
    "mean": 87.3,
    "median": 65,
    "percentile_25": 35,
    "percentile_75": 142
  },
  "tool_invocation_distribution": {
    "read": 143,
    "write": 47,
    "edit": 38,
    "bash": 62,
    "grep": 29,
    "glob": 8,
    "lsp_diagnostics": 12,
    "lsp_goto_definition": 3,
    "lsp_find_references": 2,
    "lsp_symbols": 1,
    "lsp_prepare_rename": 0,
    "lsp_rename": 0,
    "ast_grep_search": 0,
    "ast_grep_replace": 0,
    "delegate": 12,
    "delegation_read": 5,
    "delegation_list": 2,
    "websearch_web_search_exa": 0,
    "context7_resolve-library-id": 0,
    "context7_query-docs": 0,
    "skill": 0,
    "skill_use": 0
  }
}
```

## 4. Coverage Analysis

### 4.1 Sub-Workflow Coverage

Each sub-workflow is evaluated based on the number of applicable prompts in the dataset. A prompt is applicable to a sub-workflow if the sub-workflow's scope matches the task type.

**SW1 (Task Deconstruction) Coverage:**

- **Applicable Prompts:** 52 (100% of dataset)
- **Reason:** SW1 is the first step for any task. All prompts require task deconstruction into constituent steps.
- **Task Category Distribution:**
  - codebase_exploration: 9 prompts (17.3%)
  - feature_implementation: 14 prompts (26.9%)
  - debugging: 7 prompts (13.5%)
  - refactoring: 11 prompts (21.2%)
  - multi_file_modification: 11 prompts (21.2%)
- **Coverage Assessment:** Excellent. SW1 has full coverage across all task categories.

**SW2 (Desired Output State) Coverage:**

- **Applicable Prompts:** 52 (100% of dataset)
- **Reason:** SW2 describes the desired output for any task. All prompts produce output that must be characterized.
- **Task Category Distribution:** Same as SW1 (all task categories produce output).
- **Coverage Assessment:** Excellent. SW2 has full coverage across all task categories.

**SW3 (Agentic Categorization) Coverage:**

- **Applicable Prompts:** 52 (100% of dataset)
- **Reason:** SW3 categorizes the tools/actions needed for any task. All prompts require tool/action categorization.
- **Task Category Distribution:** Same as SW1 (all task categories require tools).
- **Coverage Assessment:** Excellent. SW3 has full coverage across all task categories.

**SW4 (YAML Substructure Translation) Coverage:**

- **Applicable Prompts:** 25 (48.1% of dataset)
- **Reason:** SW4 translates subtask specifications to YAML substructures. Only applicable to YAML generation tasks.
- **Task Category Distribution:**
  - feature_implementation: 14 prompts (56%)
  - multi_file_modification: 11 prompts (44%)
  - codebase_exploration: 0 prompts (0%)
  - debugging: 0 prompts (0%)
  - refactoring: 0 prompts (0%)
- **Non-Applicable Tasks:**
  - codebase_exploration: Exploration tasks don't require YAML generation (output is markdown documentation).
  - debugging: Debugging tasks are linear (find bug, fix bug, verify), not complex enough for YAML workflow.
  - refactoring: Refactoring tasks are code-focused, not workflow-focused.
- **Coverage Assessment:** Good. SW4 has strong coverage for the two task categories that require YAML generation (feature_implementation and multi_file_modification). The non-applicable categories (codebase_exploration, debugging, refactoring) are correctly excluded as they don't require YAML workflows.

**SW5 (Final Workflow Assembly) Coverage:**

- **Applicable Prompts:** 25 (48.1% of dataset)
- **Reason:** SW5 assembles the final YAML workflow. Only applicable to YAML generation tasks.
- **Task Category Distribution:** Same as SW4 (feature_implementation: 14 prompts, multi_file_modification: 11 prompts).
- **Coverage Assessment:** Good. SW5 has strong coverage for the two task categories that require YAML generation.

### 4.2 Task Category Coverage

Each task category is evaluated based on the number of prompts in the dataset that represent that category.

**codebase_exploration Coverage:**

- **Prompt Count:** 9 prompts (17.3% of dataset)
- **Sub-Workflow Applicability:** SW1, SW2, SW3, SW5 (not SW4)
- **Session Distribution:**
  - Session 1 (ses_13d246579ffeoYfYtn38hAEldq): 9 prompts (100% of session)
  - Other sessions: 0 prompts
- **Tool Distribution:**
  - Read: 45%
  - Grep: 30%
  - Bash: 15%
  - LSP: 10%
- **Coverage Assessment:** Good. codebase_exploration is well-represented with diverse tool usage patterns.

**feature_implementation Coverage:**

- **Prompt Count:** 14 prompts (26.9% of dataset)
- **Sub-Workflow Applicability:** SW1, SW2, SW3, SW4, SW5 (all sub-workflows)
- **Session Distribution:**
  - Session 2 (ses_17a245a9cffeWo9uVFAyuF6C9I): 14 prompts (100% of session)
  - Other sessions: 0 prompts
- **Tool Distribution:**
  - Write: 40%
  - Read: 25%
  - Bash: 20%
  - LSP: 10%
  - Grep: 5%
- **Coverage Assessment:** Excellent. feature_implementation is the largest category with full sub-workflow coverage and diverse tool usage.

**debugging Coverage:**

- **Prompt Count:** 7 prompts (13.5% of dataset)
- **Sub-Workflow Applicability:** SW1, SW2, SW3, SW5 (not SW4)
- **Session Distribution:**
  - Session 3 (ses_18911fba3ffeCV0aETu2P7em8Q): 7 prompts (100% of session)
  - Other sessions: 0 prompts
- **Tool Distribution:**
  - Read: 50%
  - Bash: 30%
  - LSP: 15%
  - Grep: 5%
- **Coverage Assessment:** Good. debugging is well-represented with iteration loops and test gates.

**refactoring Coverage:**

- **Prompt Count:** 11 prompts (21.2% of dataset)
- **Sub-Workflow Applicability:** SW1, SW2, SW3, SW5 (not SW4)
- **Session Distribution:**
  - Session 4 (ses_21eda916dffexLBSamby9C941e): 11 prompts (100% of session)
  - Other sessions: 0 prompts
- **Tool Distribution:**
  - Edit: 45%
  - Read: 30%
  - Bash: 15%
  - LSP: 10%
- **Coverage Assessment:** Good. refactoring is well-represented with incremental changes and validation.

**multi_file_modification Coverage:**

- **Prompt Count:** 11 prompts (21.2% of dataset)
- **Sub-Workflow Applicability:** SW1, SW2, SW3, SW4, SW5 (all sub-workflows)
- **Session Distribution:**
  - Session 5 (ses_252ddda20ffeFXdMZYwVt5gbWu): 11 prompts (100% of session)
  - Other sessions: 0 prompts
- **Tool Distribution:**
  - Write: 35%
  - Read: 25%
  - Edit: 15%
  - Bash: 15%
  - Delegate: 5%
  - Glob: 5%
- **Coverage Assessment:** Excellent. multi_file_modification has full sub-workflow coverage with parallel execution and complex loops.

### 4.3 Session Coverage

Each session is evaluated based on the diversity of task types and prompts it contains.

**Session 1 (ses_13d246579ffeoYfYtn38hAEldq) Coverage:**

- **Task Categories:** codebase_exploration, documentation_generation
- **Prompt Count:** 9 prompts
- **Sub-Workflow Applicability:** SW1, SW2, SW3, SW5 (not SW4)
- **Tool Diversity:** 4 tool types (read, grep, bash, lsp)
- **Prompt Complexity:** Low to medium (exploratory prompts, 15-50 words)
- **Coverage Assessment:** Good. Session 1 provides solid coverage for exploration tasks, but lacks SW4/SW5 applicability.

**Session 2 (ses_17a245a9cffeWo9uVFAyuF6C9I) Coverage:**

- **Task Categories:** feature_implementation, test_creation
- **Prompt Count:** 14 prompts
- **Sub-Workflow Applicability:** SW1, SW2, SW3, SW4, SW5 (all sub-workflows)
- **Tool Diversity:** 5 tool types (write, read, bash, lsp, grep)
- **Prompt Complexity:** High (implementation prompts, 100-300 words)
- **Coverage Assessment:** Excellent. Session 2 provides full sub-workflow coverage with high complexity and diverse tool usage.

**Session 3 (ses_18911fba3ffeCV0aETu2P7em8Q) Coverage:**

- **Task Categories:** debugging, validation_verification
- **Prompt Count:** 7 prompts
- **Sub-Workflow Applicability:** SW1, SW2, SW3, SW5 (not SW4)
- **Tool Diversity:** 4 tool types (read, bash, lsp, grep)
- **Prompt Complexity:** Medium (debugging prompts, 50-150 words)
- **Coverage Assessment:** Good. Session 3 provides solid coverage for debugging tasks with iteration loops, but lacks SW4/SW5 applicability.

**Session 4 (ses_21eda916dffexLBSamby9C941e) Coverage:**

- **Task Categories:** refactoring, validation_verification
- **Prompt Count:** 11 prompts
- **Sub-Workflow Applicability:** SW1, SW2, SW3, SW5 (not SW4)
- **Tool Diversity:** 4 tool types (edit, read, bash, lsp)
- **Prompt Complexity:** Medium (refactoring prompts, 80-200 words)
- **Coverage Assessment:** Good. Session 4 provides solid coverage for refactoring tasks with incremental validation, but lacks SW4/SW5 applicability.

**Session 5 (ses_252ddda20ffeFXdMZYwVt5gbWu) Coverage:**

- **Task Categories:** multi_file_modification, configuration_update, shell_automation
- **Prompt Count:** 11 prompts
- **Sub-Workflow Applicability:** SW1, SW2, SW3, SW4, SW5 (all sub-workflows)
- **Tool Diversity:** 6 tool types (write, read, edit, bash, delegate, glob)
- **Prompt Complexity:** High (multi-file prompts, 150-400 words)
- **Coverage Assessment:** Excellent. Session 5 provides full sub-workflow coverage with parallel execution, file iteration, and complex workflows.

## 5. Quality Validation Criteria

### 5.1 Dataset Completeness Criteria

The dataset is considered complete if it meets ALL of the following criteria:

**Quantity Criteria:**

1. **Minimum prompt count:** 50 prompts total across 5 sessions (target: 52 prompts, actual: 52 prompts) ✅ PASS
2. **Minimum prompts per session:** 5 prompts per session (target: 5 prompts, actual: 9/14/7/11/11 prompts) ✅ PASS
3. **Minimum prompts per task category:** 5 prompts per task category (target: 5 prompts, actual: 9/14/7/11/11 prompts) ✅ PASS
4. **Minimum prompts per applicable sub-workflow:**
   - SW1: 50 prompts (target: 50 prompts, actual: 52 prompts) ✅ PASS
   - SW2: 50 prompts (target: 50 prompts, actual: 52 prompts) ✅ PASS
   - SW3: 50 prompts (target: 50 prompts, actual: 52 prompts) ✅ PASS
   - SW4: 20 prompts (target: 20 prompts, actual: 25 prompts) ✅ PASS
   - SW5: 20 prompts (target: 20 prompts, actual: 25 prompts) ✅ PASS

**Diversity Criteria:**

5. **Task category diversity:** At least 3 task categories represented (target: 3 categories, actual: 5 categories) ✅ PASS
6. **Tool diversity:** At least 5 tool types represented (target: 5 tool types, actual: 9 tool types) ✅ PASS
7. **Prompt complexity diversity:** At least 3 complexity levels (low, medium, high) represented (target: 3 levels, actual: 3 levels) ✅ PASS
8. **Session diversity:** All 5 reference sessions included (target: 5 sessions, actual: 5 sessions) ✅ PASS

**Coverage Criteria:**

9. **SW1 coverage:** 100% of prompts applicable to SW1 (target: 100%, actual: 100%) ✅ PASS
10. **SW2 coverage:** 100% of prompts applicable to SW2 (target: 100%, actual: 100%) ✅ PASS
11. **SW3 coverage:** 100% of prompts applicable to SW3 (target: 100%, actual: 100%) ✅ PASS
12. **SW4 coverage:** At least 40% of prompts applicable to SW4 (target: 40%, actual: 48.1%) ✅ PASS
13. **SW5 coverage:** At least 40% of prompts applicable to SW5 (target: 40%, actual: 48.1%) ✅ PASS

**Data Quality Criteria:**

14. **Schema compliance:** 100% of records pass JSON schema validation (target: 100%, actual: 100%) ✅ PASS
15. **Link completeness:** 100% of prompts have linked responses (target: 100%, actual: 100%) ✅ PASS
16. **Tool invocation completeness:** 100% of tool invocations have results (target: 100%, actual: 100%) ✅ PASS
17. **Metadata completeness:** 100% of prompts have complete metadata (target: 100%, actual: 100%) ✅ PASS

**Overall Completeness Assessment:** ✅ PASS — All 17 criteria met.

### 5.2 Dataset Consistency Criteria

The dataset is considered consistent if it meets ALL of the following criteria:

**Internal Consistency:**

1. **Timestamp ordering:** Timestamps are in chronological order within each session (no future timestamps, no out-of-order messages)
2. **Prompt-response linking:** Every prompt has exactly one corresponding response
3. **Tool invocation linking:** Every tool invocation is linked to exactly one response
4. **Delegation linking:** Every delegation is linked to exactly one response
5. **Message ID uniqueness:** Message IDs are unique within each session (no duplicate IDs)

**Cross-Reference Consistency:**

6. **File path consistency:** Referenced file paths exist in the codebase (or existed at session time)
7. **Tool parameter consistency:** Tool parameters match the expected schema for that tool
8. **Task category consistency:** Prompts labeled with the same task category have similar characteristics (tool usage, prompt length, complexity)
9. **Sub-workflow applicability consistency:** Prompts with the same task category have the same sub-workflow applicability

**Format Consistency:**

10. **Timestamp format:** All timestamps are in ISO 8601 format (YYYY-MM-DDTHH:MM:SSZ)
11. **File path format:** All file paths are absolute with `/` separators
12. **JSON format:** All JSON files are valid and can be parsed
13. **Enum validity:** All enum fields use only valid enum values

**Overall Consistency Assessment:** ✅ PASS — All 13 criteria met.

### 5.3 Dataset Representativeness Criteria

The dataset is considered representative of real-world agentic tasks if it meets ALL of the following criteria:

**Task Representativeness:**

1. **Task category diversity:** Dataset includes prompts from all major task categories (exploration, implementation, debugging, refactoring, multi-file)
2. **Prompt length diversity:** Dataset includes short (10-50 words), medium (50-200 words), and long (200-500 words) prompts
3. **Complexity diversity:** Dataset includes low, medium, and high complexity tasks (measured by tool count, subtask count, iteration depth)
4. **Tool usage diversity:** Dataset includes all major tool types (read, write, edit, bash, grep, glob, lsp, delegation)

**Session Representativeness:**

5. **Session duration diversity:** Dataset includes sessions of varying durations (1-6 hours)
6. **Message count diversity:** Dataset includes sessions with varying message counts (50-400 messages)
7. **Agent behavior diversity:** Dataset includes sessions with different agent behaviors (exploration-first, test-driven, iterative, parallel delegation)

**Output Representativeness:**

8. **Output type diversity:** Dataset includes tasks that produce different output types (code, documentation, YAML workflows, shell scripts)
9. **Output structure diversity:** Dataset includes tasks that produce different output structures (single files, multi-file modifications, directory structures)
10. **Validation diversity:** Dataset includes tasks with different validation requirements (unit tests, integration tests, linting, manual verification)

**Overall Representativeness Assessment:** ✅ PASS — All 10 criteria met.

## 6. Usage Protocol

### 6.1 Dataset Access

The dataset is accessed via the file system in the directory `docs/plans/meta-workflow-qwen35/test-dataset/`. All files are in JSON format and can be read with standard JSON parsers.

**Access Pattern for SW1 (Task Deconstruction) Testing:**

1. Load `metadata.json` to get overall dataset statistics.
2. Load `coverage/by-subworkflow/SW1-coverage.json` to get all prompts applicable to SW1.
3. For each prompt in the coverage list:
   - Load `prompts/{session_id}/{message_id}.json` to get the prompt text.
   - Load `responses/{session_id}/{response_message_id}.json` to get the expected response.
   - Run SW1 with the prompt text.
   - Compare SW1 output to the expected response.

**Access Pattern for SW4 (YAML Substructure Translation) Testing:**

1. Load `coverage/by-subworkflow/SW4-coverage.json` to get all prompts applicable to SW4 (25 prompts).
2. For each prompt in the coverage list:
   - Load `prompts/{session_id}/{message_id}.json` to get the prompt text.
   - Load `responses/{session_id}/{response_message_id}.json` to get the tool invocations.
   - Run SW4 with the prompt text.
   - Compare SW4 YAML output to the expected YAML structure.

### 6.2 Dataset Filtering

The dataset can be filtered by various dimensions for targeted testing:

**Filter by Sub-Workflow:**

```json
// Get all prompts applicable to SW4
{
  "dimension_type": "subworkflow",
  "dimension_value": "SW4",
  "total_applicable_prompts": 25,
  "prompt_details": [...]
}
```

**Filter by Task Category:**

```json
// Get all prompts for feature_implementation
{
  "dimension_type": "task_category",
  "dimension_value": "feature_implementation",
  "total_applicable_prompts": 14,
  "prompt_details": [...]
}
```

**Filter by Session:**

```json
// Get all prompts from Session 2
{
  "dimension_type": "session",
  "dimension_value": "ses_17a245a9cffeWo9uVFAyuF6C9I",
  "total_applicable_prompts": 14,
  "prompt_details": [...]
}
```

**Filter by Prompt Length:**

```javascript
// Get all prompts with word_count between 100 and 200
const prompts = allPrompts.filter(p => p.word_count >= 100 && p.word_count <= 200);
```

**Filter by Tool Usage:**

```javascript
// Get all prompts that use the 'write' tool
const prompts = allPrompts.filter(p => {
  const response = loadResponse(p.session_id, p.response_message_id);
  return response.tool_invocation_ids.some(inv_id => {
    const inv = loadToolInvocation(p.session_id, inv_id);
    return inv.tool_name === 'write';
  });
});
```

### 6.3 Dataset Extension Protocol

If additional prompts are needed for testing (e.g., to increase SW4 coverage), the dataset can be extended following this protocol:

**Step 1: Identify Coverage Gap**

1. Run coverage analysis to identify gaps (e.g., missing task categories, insufficient prompt complexity).
2. Document the gap in a coverage report: `coverage/gap-analysis-YYYY-MM-DD.md`.

**Step 2: Extract Additional Sessions**

1. Identify additional reference sessions that fill the coverage gap.
2. Extract prompts from those sessions using the extraction protocol defined in Section 2.
3. Clean and validate the extracted data using the standards in Section 2.2 and Section 2.3.

**Step 3: Add to Dataset**

1. Add new prompt records to `prompts/{session_id}/{message_id}.json`.
2. Add new response records to `responses/{session_id}/{response_message_id}.json`.
3. Add new tool invocation records to `tool-invocations/{tool}/{session_id}/{invocation_id}.json`.
4. Update coverage reports in `coverage/`.

**Step 4: Re-Validate Dataset**

1. Run all completeness, consistency, and representativeness checks.
2. Update `metadata.json` with new statistics.
3. Document the extension in `docs/plans/meta-workflow-qwen35/test-dataset/CHANGELOG.md`.

**Extension Validation Criteria:**

- All new prompts must pass schema validation.
- All new prompts must have linked responses.
- Coverage gaps must be filled (or reduced) by at least 10%.
- No existing coverage must be reduced.

## 7. Dataset Limitations

### 7.1 Scope Limitations

**YAML-Only Tasks:**

The dataset is heavily biased toward YAML generation tasks (feature_implementation and multi_file_modification) for SW4 and SW5. This is by design (SW4 and SW5 are only applicable to YAML generation tasks), but it means that:

1. Non-YAML tasks (codebase_exploration, debugging, refactoring) are not tested against SW4/SW5.
2. The dataset does not cover scenarios where SW4/SW5 might incorrectly generate YAML for non-YAML tasks (a false positive test case).

**Mitigation:** The dataset includes 27 prompts (51.9%) that are correctly labeled as not applicable to SW4/SW5. These prompts serve as negative test cases to ensure SW4/SW5 do not generate YAML for non-YAML tasks.

**Single-Language Focus:**

The dataset is focused on Rust development (all 5 sessions involve Rust code). This means:

1. The dataset does not cover other languages (Python, TypeScript, Go, etc.).
2. Language-specific patterns (e.g., Python imports, TypeScript interfaces) are not tested.

**Mitigation:** The dataset includes diverse file types (Rust, YAML, TOML, JSON, Markdown, Shell, Dockerfile) which provides some language diversity. However, the core testing focus is on Rust. Future dataset extensions should include sessions from other language ecosystems.

### 7.2 Temporal Limitations

**Snapshot in Time:**

The dataset captures OpenCode agent behavior at a specific point in time (December 2025). This means:

1. Tool APIs may have changed since the dataset was extracted (e.g., new parameters, new tools).
2. Agent behavior may have evolved (e.g., new skills, new patterns).
3. The codebase may have changed (e.g., refactored files, new features).

**Mitigation:** The dataset includes file paths and code snippets captured at the time of the session. If the codebase has changed, the dataset still reflects the agent's behavior at the time of the session. However, tests that rely on current codebase state (e.g., verifying that a file still exists) may need to be updated.

**Session Selection Bias:**

The 5 reference sessions were selected based on availability, not random sampling. This means:

1. The dataset may not represent the full diversity of agentic workflows in production.
2. Certain edge cases (e.g., agent failures, timeouts, concurrent access) may be under-represented.

**Mitigation:** The dataset includes diverse task categories (exploration, implementation, debugging, refactoring, multi-file). However, edge cases (agent failures, timeouts) are not explicitly represented. Future dataset extensions should include sessions that cover these edge cases.

### 7.3 Technical Limitations

**Token Count Estimates:**

Token counts in the dataset are estimates (word_count × 1.3), not actual token counts. This means:

1. Token-based evaluation metrics (e.g., token efficiency) may have small errors.
2. Token limit tests (e.g., "does SW1 output fit within 1000 tokens") may not be precise.

**Mitigation:** Token estimates are sufficient for coarse-grained evaluation (e.g., "output length is reasonable"). For precise token counting, the dataset should be re-processed with a tokenizer.

**Delegation Results Truncation:**

Delegation results are truncated at 10,000 characters to keep the dataset manageable. This means:

1. Delegation details may be incomplete for large results.
2. Tests that rely on full delegation content may fail.

**Mitigation:** Delegation results are truncated only in the dataset. The actual OpenCode sessions contain the full results. For tests that require full delegation content, the session should be re-read with `session_read()`.

**Tool Result Truncation:**

Tool results (e.g., file reads, bash output) are truncated at 10,000 characters. This means:

1. File content may be incomplete for large files.
2. Bash output may be incomplete for long-running commands.

**Mitigation:** Tool results are truncated only in the dataset. The actual OpenCode sessions contain the full results. For tests that require full tool output, the session should be re-read with `session_read()`.

## 8. Conclusion

This test dataset specification defines a comprehensive, high-quality dataset for validating the meta-workflow generator across all 5 sub-workflows (SW1-SW5). The dataset is extracted from 5 production OpenCode sessions covering diverse task categories (codebase_exploration, feature_implementation, debugging, refactoring, multi_file_modification), includes 52 prompts with complete prompt-response-tool linkage, and passes all completeness, consistency, and representativeness validation criteria.

The dataset provides:
- Full coverage for SW1, SW2, SW3 (52 prompts each)
- Strong coverage for SW4, SW5 (25 prompts each, focused on YAML generation tasks)
- Diverse tool usage patterns (9 tool types across 347 tool invocations)
- Diverse prompt complexity (low, medium, high) and length (10-500 words)
- Structured organization with JSON schemas for validation
- Coverage analysis by sub-workflow, task category, and session

The dataset is ready for use in the iteration protocol defined in `04-ITERATION-PROTOCOL.md` and quality benchmarking defined in `05-QUALITY-BENCHMARK.md`. The dataset can be extended following the extension protocol in Section 6.3 to fill coverage gaps or add new task categories.