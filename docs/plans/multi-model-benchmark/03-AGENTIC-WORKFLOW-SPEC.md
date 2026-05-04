# Agentic Summarization-Expansion Workflow Specification

## Overview

Implement a YAML-defined workflow that reads a large document (e.g., plan file), processes it in chunks through an oscillating summarization-expansion loop, and produces a refined output. The workflow alternates between compression (summarization) and elaboration (expansion) across multiple oscillations per chunk.

## Research Foundation

### Chain of Density (Adams et al., 2023)
- Start with entity-sparse summary
- Iteratively identify and fuse 1-3 missing entities without increasing length
- Repeat 5x → progressively denser output
- 20x latency reduction vs GPT-4 full context

### Recursive Summarization (Wang et al., 2023)
- LLM memorizes small dialogue contexts
- Recursively produces new memory using previous + following contexts
- Generates response from latest memory

### Accordion-Thinking (2025)
- Alternate detailed reasoning + compact summaries
- Discard intermediate activation states
- Lossless compression via self-regulation

## Workflow Architecture

### Step Type: `oscillate_abstraction`

A new step type that encapsulates the summarization-expansion oscillation pattern.

```yaml
- step: refine_document
  type: oscillate_abstraction
  input:
    file_path: "./docs/plans/my-plan.md"
    chunk_size: 4000           # characters per chunk
    oscillations: 3            # number of summarize→expand cycles
    overlap: 200               # character overlap between chunks
  prompts:
    summarize: |
      Summarize the following text, extracting key technical points.
      Remove filler, hedging, and repetition.
      Preserve all specific numbers, file paths, function names, and constraints.
      Target length: {{chunk.target_length}} characters.
      
      Text:
      {{loop.previous_output}}
    expand: |
      Expand on the following summary by adding:
      1. Concrete examples where abstract
      2. Missing context that a senior engineer would infer
      3. Edge cases and error conditions
      4. Performance implications
      
      Maintain all existing technical substance.
      Target length: {{chunk.target_length}} characters.
      
      Summary:
      {{loop.previous_output}}
  output:
    save_to: refined_output
    format: text
    path: "./workspace/output/refined_plan.md"
```

### Execution Flow

```
Input File (e.g., 20,000 chars)
    │
    ├── Chunk 1 (chars 0-4000)
    │   ├── Oscillation 1: Summarize → Expand
    │   ├── Oscillation 2: Summarize → Expand
    │   └── Oscillation 3: Summarize → Expand → Final
    │
    ├── Chunk 2 (chars 3800-7800, overlap=200)
    │   ├── Oscillation 1: Summarize → Expand
    │   ├── Oscillation 2: Summarize → Expand
    │   └── Oscillation 3: Summarize → Expand → Final
    │
    ├── ... (more chunks)
    │
    └── Output: Concatenated refined chunks
```

### Oscillation Detail

Each chunk goes through `2 * oscillations` iterations:

| Iteration | Phase | Input | Output |
|-----------|-------|-------|--------|
| 0 | Summarize | Original chunk text | Compressed key points |
| 1 | Expand | Summarized output | Elaborated version |
| 2 | Summarize | Expanded output | Refined compression |
| 3 | Expand | Summarized output | Deep elaboration |
| 4 | Summarize | Expanded output | Final compression |
| 5 | Expand | Summarized output | Final refined output |

**For 3 oscillations**: 6 LLM calls per chunk. For 10 chunks: 60 total calls.

## Implementation Components

### E1: FileChunker

**File**: `src/agent/chunker.rs` (new)

```rust
pub struct FileChunker {
    chunk_size: usize,     // characters per chunk
    overlap: usize,        // character overlap between chunks
}

pub struct Chunk {
    index: usize,
    content: String,
    start_offset: usize,
    end_offset: usize,
    target_length: usize,  // for prompt template
}

impl FileChunker {
    pub fn chunk_file(path: &Path) -> Result<Vec<Chunk>>;
    pub fn chunk_text(text: &str) -> Vec<Chunk>;
}
```

**Smart chunking**: Split at paragraph boundaries, not mid-sentence. If chunk would split a paragraph, extend to next paragraph boundary.

### E2: OscillationExecutor

**File**: `src/agent/oscillation.rs` (new)

```rust
pub struct OscillationExecutor {
    oscillations: usize,
    summarize_prompt: String,
    expand_prompt: String,
}

pub struct OscillationResult {
    chunk_index: usize,
    final_output: String,
    iterations: usize,
    total_tokens_used: usize,
}

impl OscillationExecutor {
    pub async fn execute_chunk(
        &self,
        chunk: &Chunk,
        context: &mut WorkflowContext,
        llm: &dyn LlmBackend,
    ) -> Result<OscillationResult>;
}
```

**Context passing**: Each iteration's output replaces `{{loop.previous_output}}` in the next iteration's prompt template.

### E3: Workflow Integration

The oscillate_abstraction step integrates with the workflow loop engine:

```yaml
- step: refine_document
  type: oscillate_abstraction
  input:
    file_path: "{{workflow.input_file}}"
    chunk_size: 4000
    oscillations: 3
  loop:
    count:
      max_iterations: "{{chunks.total_count}}"  # dynamic from file size
      iteration_variable: current_chunk
  output:
    save_to: refined_output
```

**Mechanism**:
1. `oscillate_abstraction` step reads input file
2. Chunks file using `FileChunker`
3. Outer loop: iterate over chunks (count-based)
4. Inner loop: for each chunk, oscillate summarize/expand N times
5. Collect all refined chunks
6. Concatenate into final output

### E4: Template Variables

Available in prompts during oscillation:

| Variable | Description |
|----------|-------------|
| `{{loop.previous_output}}` | Output from previous iteration |
| `{{loop.iteration}}` | Current iteration number (0-based) |
| `{{loop.phase}}` | "summarize" or "expand" |
| `{{chunk.index}}` | Current chunk index |
| `{{chunk.total}}` | Total number of chunks |
| `{{chunk.target_length}}` | Target character length |
| `{{chunk.start_offset}}` | Start offset in original file |
| `{{chunk.end_offset}}` | End offset in original file |

## YAML Workflow Example: Full Document Refinement

```yaml
workflow_id: document_refinement
name: "Document Refinement via Abstraction Oscillation"
description: "Read document, chunk it, oscillate summarize/expand 3x per chunk"
min_schema_version: "2.0.0"

models:
  primary:
    provider: lmstudio
    model: "Qwen3-4B-Instruct-2507-Q4_K_M"

execution:
  mode: serial
  memory:
    max_allocated_memory_mb: 8192
    unload_unused: true

logging:
  global:
    level: info
    detail: medium
    output_type: chat
  performance_metrics:
    level: debug
    detail: very_high

agentic_workflow:
  - step: refine_document
    id: refine_step
    type: oscillate_abstraction
    input:
      file_path: "./docs/plans/multi-model-benchmark/01-IMPLEMENTATION-PLAN.md"
      chunk_size: 4000
      oscillations: 3
      overlap: 200
    prompts:
      summarize: |
        Compress the following text to its essential technical content.
        Keep: numbers, paths, function names, struct fields, error messages.
        Drop: filler words, hedging, repetition, transitions.
        Target: {{chunk.target_length}} chars max.
        
        Text:
        {{loop.previous_output}}
      expand: |
        Elaborate on this summary. Add:
        - Missing implementation details a senior engineer would infer
        - Edge cases and error handling considerations
        - Performance implications and tradeoffs
        Keep all existing technical facts. Target: {{chunk.target_length}} chars.
        
        Summary:
        {{loop.previous_output}}
    output:
      save_to: refined_plan
      format: text
      path: "./workspace/output/refined-implementation-plan.md"

  - step: validate_output
    id: validate
    model: "${models.primary}"
    loop:
      validation:
        max_iterations: 3
        tolerance: 0.1
        exact_criteria:
          - metric: completeness_score
            operator: ">="
            target: 0.85
    input:
      prompt: |
        Rate this refined document on a 0-1 scale for:
        1. Technical completeness (all details preserved)
        2. Clarity (no ambiguity)
        3. Actionability (can implement from this alone)
        
        Refined document:
        {{step.refine_step.output}}
    output:
      save_to: validation_result
```

## Token Budget Estimation

For a 20,000 character document with 3 oscillations:

| Component | Count | Tokens Each | Total Tokens |
|-----------|-------|-------------|-------------|
| Chunks (4000 chars) | 5 | ~1000 | ~5000 |
| Summarize calls | 15 (5×3) | ~800 prompt + ~400 output | ~18000 |
| Expand calls | 15 (5×3) | ~800 prompt + ~600 output | ~21000 |
| **Total** | | | **~44000 tokens** |

At ~40 tokens/sec (Qwen3-4B on Vulkan): ~18 minutes total.

## Success Criteria

1. `oscillate_abstraction` step type parses from YAML
2. File chunking respects paragraph boundaries
3. Each chunk oscillates summarize/expand N times
4. Previous iteration output feeds into next iteration
5. Final output is coherent and preserves key technical substance
6. Validation loop can score the output quality
7. Round-trip YAML → parse → execute → output works end-to-end
