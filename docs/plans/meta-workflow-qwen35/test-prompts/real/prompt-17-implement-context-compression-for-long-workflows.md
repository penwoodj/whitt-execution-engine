# Task: Implement Context Compression for Long Workflows

## Objective
Workflows with 20+ steps accumulate output that exceeds the 32K context window. The engine has no compression mechanism — it just truncates output silently.

## Requirements
1. Read `src/benchmark/runner.rs` to understand how step outputs are accumulated
2. Read `src/workflow/hooks/actions.rs` to understand the bookmark system
3. Design a `ContextCompressor` struct that:
   - Tracks total accumulated context across all steps
   - When context exceeds 24K tokens (75% of 32K), triggers compression
   - Compression: summarize prior step outputs into a single "context summary" bookmark
   - Replaces individual `{{step.X.output}}` references with `{{bookmarks.context_summary}}`
4. Implement the compression function using a separate LLM call to summarize
5. Write the complete implementation

## Output
A markdown document with the full `ContextCompressor` implementation, including:
- Struct definition
- Context tracking logic
- Compression trigger conditions
- Summarization prompt template
- Integration code for `src/benchmark/runner.rs`
