# UF08: Chat and Tool History Capture

## Overview
Captures the complete LLM conversation history (prompts, responses, tool calls, tool results) for each benchmark step into a structured format suitable for detail.md generation and post-benchmark analysis.

## User Story
As a benchmark operator, I want to capture the full chat and tool execution history for every model's benchmark run so that I can review exactly what each model said, what tools it attempted to use, and why it scored the way it did.

## Pre-conditions
- Model loaded and executing workflow steps (UF06)
- Logging infrastructure configured
- Output directory structure created

## Trigger
Each step of the benchmark workflow produces a prompt/response cycle.

## Flow Diagram

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ Step Sends   │────>│ Model       │────>│ Capture     │
│ Prompt       │     │ Responds    │     │ Response    │
└──────────────┘     └──────┬───────┘     └──────┬───────┘
                            │                     │
                            ▼                     │
                     ┌──────────────┐              │
                     │ Tool Calls? │              │
                     └──────┬───────┘              │
                            │                     │
                     Yes ───┤                     │
                     │      ▼                     │
                     │  ┌──────────────┐          │
                     │  │ Execute     │          │
                     │  │ Tool        │          │
                     │  └──────┬───────┘          │
                     │         │                  │
                     │         ▼                  │
                     │  ┌──────────────┐          │
                     │  │ Capture     │          │
                     │  │ Tool Result │          │
                     │  └──────┬───────┘          │
                     │         │                  │
                     │         ▼                  │
                     │  ┌──────────────┐          │
                     │  │ Model       │          │
                     │  │ Responds    │          │
                     │  │ to Tool     │          │
                     │  │ Result      │          │
                     │  └──────────────┘          │
                     │                            │
                     └────────────────────────────┤
                                                  │
                                                  ▼
                                           ┌──────────────┐
                                           │ Append to    │
                                           │ Step History │
                                           └──────────────┘
```

## Step-by-Step

### Step 1: Capture Prompt Sent
- **Action**: Record the full prompt text sent to the model, including system prompt, injected variables, and any context
- **Schema Properties Used**: `logging.scopes.step_execution`, `logging.global.detail: high`
- **Input**: Prompt text, step ID, model ID, timestamp
- **Output**: Prompt entry in step history
- **Error Handling**: If capture fails, log warning but don't abort step
- **New Schema Requirements**: None — covered by existing logging

### Step 2: Capture Model Response
- **Action**: Record the full model response including text, token counts, timing, and finish reason
- **Schema Properties Used**: `logging.scopes.step_execution`, `output.fields`
- **Input**: Response text, tokens_used, time_to_first_token, time_total, finish_reason
- **Output**: Response entry in step history
- **Error Handling**: If response capture fails, record raw response and flag as "incomplete_capture"
- **New Schema Requirements**: `output.fields` should include `finish_reason`, `tokens_prompt`, `tokens_completion`

### Step 3: Detect and Capture Tool Calls
- **Action**: If model response includes tool/function calls, parse and record each call
- **Schema Properties Used**: `tools.*.enabled`, `logging.scopes.tool_execution`
- **Input**: Model response with tool call JSON
- **Output**: Tool call entries (tool_name, tool_input, timestamp)
- **Error Handling**: If tool call parsing fails, record raw response as text output
- **New Schema Requirements**: None — covered by existing tool logging

### Step 4: Execute and Capture Tool Results
- **Action**: Execute each tool call and record the result
- **Schema Properties Used**: `tools.*.enabled`, `tools.*.timeout_seconds`, `logging.scopes.tool_execution.detail: very_high`
- **Input**: Tool call definition, tool configuration
- **Output**: Tool result (success/failure, result_data, execution_time_ms)
- **Error Handling**: On tool failure: record error, continue with error result sent back to model
- **New Schema Requirements**: None

### Step 5: Capture Follow-up Response
- **Action**: After tool results are returned to the model, capture the model's follow-up response
- **Schema Properties Used**: Same as Step 2
- **Input**: Follow-up response from model
- **Output**: Follow-up response entry in step history
- **Error Handling**: Same as Step 2
- **New Schema Requirements**: None

### Step 6: Build Step History Object
- **Action**: Assemble all captures for this step into a structured history object
- **Schema Properties Used**: `logging.global.format: json`
- **Input**: All prompt/response/tool captures for this step
- **Output**: Step history JSON object
- **Error Handling**: N/A
- **New Schema Requirements**: None

### Step 7: Store History for detail.md
- **Action**: Pass the step history to the detail.md generator (UF09) for inclusion in the model's output file
- **Schema Properties Used**: `output.file_output`
- **Input**: Step history object
- **Output**: History stored in memory for detail.md generation
- **Error Handling**: N/A
- **New Schema Requirements**: None

## Post-conditions
- Complete conversation history captured for every step of every model
- Tool calls and results captured with timing
- History stored in structured JSON format
- Ready for detail.md generation (UF09)

## Edge Cases
- Model generates tool calls in a loop (max tool rounds = 5 per step)
- Tool returns binary data (encode as base64, note in history)
- Model response is truncated due to max_tokens (record as "truncated")
- Multiple tool calls in single response (capture all, execute in parallel if possible)
- Tool call references undefined tool (record as "unknown_tool", skip execution)

## Schema Coverage

| Feature | Covered by Schema | Needs Addition |
|---------|-------------------|----------------|
| Step-level logging | Yes (`logging.scopes.step_execution`) | None |
| Tool execution logging | Yes (`logging.scopes.tool_execution`) | None |
| Output field capture | Yes (`output.fields`) | `finish_reason`, `tokens_prompt` |
| Chat output format | Yes (`logging.global.output_type: chat`) | None |
| Structured history storage | No | History schema definition for detail.md |
| Token count capture | Partially | Explicit token fields |

## Dependencies
- UF06 (Orchestration) — executes the steps that produce history
- UF09 (detail.md Generation) — consumes the captured history
- UF05 (Workflow Definition) — defines what steps to capture
