# UF05: Eight-Prompt Chain Workflow Definition

## Overview
Defines the benchmark workflow structure: an 8-step prompt chain where each step tests a different LLM capability (code generation, summarization, reasoning, creative writing, factual QA, instruction following, multi-turn, tool use planning).

## User Story
As a benchmark designer, I want to define a reusable 8-prompt benchmark workflow so that every model is evaluated on the same diverse set of capabilities for fair comparison.

## Pre-conditions
- Benchmark manifest exists (UF01)
- Models are installed (UF02)
- Workflow schema is defined in unified-workflow-schema.yml

## Trigger
Benchmark execution is initiated for a model.

## Flow Diagram

```
┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐
│ Step 1   │  │ Step 2   │  │ Step 3   │  │ Step 4   │
│ Code Gen  │  │ Summary │  │ Reasoning │  │ Creative│
└────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘
     │             │             │             │
     ▼             ▼             ▼             ▼
┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐
│ Step 5   │  │ Step 6   │  │ Step 7   │  │ Step 8   │
│ Factual │  │ Instruct. │  │ Multi    │  │ Tool     │
│ QA      │  │ Follow.  │  │ Turn    │  │ Planning│
└────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘
     │             │             │             │
     ▼             ▼             ▼             ▼
┌──────────┐
│ Score &  │
│ Save    │
└──────────┘
```

## Prompt Types and Expected Outputs

| Step | Prompt Type | Tests Capability | Expected Output Format |
|------|------------|-----------------|----------------------|
| 1 | Code Generation | Functional coding | Valid code in specified language |
| 2 | Summarization | Comprehension | Concise summary of long text |
| 3 | Reasoning | Logic & math | Step-by-step reasoning chain |
| 4 | Creative Writing | Style & narrative | Short story or descriptive text |
| 5 | Factual QA | Knowledge accuracy | Correct answer with source citation |
| 6 | Instruction Following | Compliance | Output matching exact format spec |
| 7 | Multi-Turn Conversation | Context retention | Coherent response referencing prior turns |
| 8 | Tool Use Planning | Planning ability | Structured plan with tool calls |

## Step-by-Step

### Step 1: Define Workflow Skeleton
- **Action**: Create YAML workflow with 8 steps using `agentic_workflow.steps:` object format
- **Schema Properties Used**: `agentic_workflow.steps`, `workflow_id`, `name`, `description`
- **Input**: Benchmark manifest
- **Output**: Workflow YAML structure with 8 step placeholders
- **Error Handling**: N/A
- **New Schema Requirements**: None

### Step 2: Configure Step 1 - Code Generation
- **Action**: Set up code generation prompt with language specification and complexity requirement
- **Schema Properties Used**: `steps.step_1.prompt`, `steps.step_1.output.format`
- **Input**: Language (e.g., "rust"), complexity level, topic
- **Output**: Structured code output with metadata
- **Error Handling**: If output isn't valid code, score 0 but don't retry
- **New Schema Requirements**: None

### Step 3: Configure Steps 2-8
- **Action**: Set up each remaining step with appropriate prompt template
- **Schema Properties Used**: `steps.step_N.prompt`, `steps.step_N.output`, `steps.step_N.inputs`
- **Input**: Prompt templates per type
- **Output**: Complete workflow YAML
- **Error Handling**: N/A at definition time
- **New Schema Requirements**: None

### Step 4: Configure Variable Passing
- **Action**: Set up `inputs` so each step can reference previous step outputs
- **Schema Properties Used**: `inputs`, `"${step.step_N.output}"`
- **Input**: Step dependency chain
- **Output**: Workflow with interconnected variables
- **Error Handling**: If variable reference is circular, validation should catch it
- **New Schema Requirements**: None

### Step 5: Set Output Configuration
- **Action**: Configure each step to save to a variable and optionally to a file
- **Schema Properties Used**: `output.save_to`, `output.format`, `output.file_output`
- **Input**: Output format preferences
- **Output**: Each step's result captured for scoring
- **Error Handling**: If file output fails, fall back to variable-only capture
- **New Schema Requirements**: None

### Step 6: Add Timing and Token Tracking
- **Action**: Configure each step to capture timing and token usage metadata
- **Schema Properties Used**: `model_overrides.timeout.time_to_first_response`, `output.fields`
- **Input**: Timing configuration
- **Output**: Workflow with per-step timing capture
- **Error Handling**: N/A
- **New Schema Requirements**: None

### Step 7: Add Retry Configuration Per Step
- **Action**: Set conservative retries (1-2 attempts) per step, don't retry forever on 100 models
- **Schema Properties Used**: `retry.max_attempts`, `retry.condition`, `retry.level: prompt_restart`
- **Input**: Retry policy
- **Output**: Workflow with retry config per step
- Error Handling**: Max retries prevents infinite loops on stubborn models
- **New Schema Requirements**: None

### Step 8: Validate Workflow
- **Action**: Run YAML validation against schema to ensure workflow is structurally correct
- **Schema Properties Used**: All referenced properties
- **Input**: Complete workflow YAML
- **Output**: Validated workflow ready for execution
- **Error Handling**: Report validation errors for fix
- **New Schema Requirements**: None

## Post-conditions
- Reusable 8-prompt benchmark workflow YAML exists
- Each step has defined input/output schema
- Variable passing chain is valid
- Retry and timeout configured appropriately for batch execution

## Edge Cases
- Model context window too small for multi-turn (step 7 may fail)
- Tool use planning step references tools not available in the benchmark
- Model generates valid output but wrong format (scoring handles this)
- Steps need different token budgets (some prompts are longer)

## Schema Coverage

| Feature | Covered by Schema | Needs Addition |
|---------|-------------------|----------------|
| 8-step workflow | Yes (`agentic_workflow.steps`) | None |
| Variable passing between steps | Yes (`inputs`) | None |
| Per-step output format | Yes (`output.format`) | None |
| Per-step retry | Yes (`retry.max_attempts`) | None |
| Per-step timeout | Yes (`model_overrides.timeout`) | None |
| Per-model prompt customization | Partially | Template injection mechanism |

## Dependencies
- UF01 (Discovery) - for model list
- UF05 (Orchestration) - to execute workflow against models
- UF08 (History Capture) - to save step results
