# Task: Implement Loop Execution

## Objective
The schema declares two loop types (validation-based and count-based) but the engine never iterates. SW1-SW4 simulate loops via shell hacks.

## Requirements
1. Read `docs/schema/unified-workflow-schema.yml` to find loop definitions
2. Read `src/workflow/step.rs` to understand loop field parsing
3. Implement loop execution:
   - For `count` loops: execute step N times, accumulating output
   - For `validation` loops: execute until quality threshold met or max iterations reached
   - Inject `{{loop.index}}` and `{{loop.max}}` template variables
   - Break on quality check pass or hard cap
4. Write the complete implementation

## Output
Complete Rust implementation including:
- `LoopExecutor` struct
- Count loop logic
- Validation loop logic with quality checking
- Template variable injection
- Termination conditions
