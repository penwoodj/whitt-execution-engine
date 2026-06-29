# Task: Add Quality Gate Between Workflow Steps

## Objective
Currently, step output quality is never checked. A step that produces empty output, refusal text, or garbage still "succeeds" and its output propagates to downstream steps.

## Requirements
1. Read `src/benchmark/runner.rs` to find where `after_step_succeeds` hook fires
2. Read `src/workflow/hooks/actions.rs` to understand hook execution
3. Implement a `QualityGate` struct that:
   - Checks output size (minimum 100 bytes)
   - Checks for refusal patterns ("I cannot", "I'm unable to", "As an AI")
   - Checks for empty/garbage output (repeated characters, encoding errors)
   - Returns `QualityVerdict::Pass` or `QualityVerdict::Fail(reason)`
4. When quality gate fails:
   - Log the failure reason
   - Retry the step with adjusted prompt (add "Your previous response was too short/refused. Please provide a complete response.")
   - If retry also fails, mark step as failed
5. Write the complete implementation

## Output
Complete Rust implementation of `QualityGate` with:
- Enum definitions (QualityVerdict, FailureReason)
- Check functions (size, refusal, garbage)
- Retry integration logic
- Hook wiring code
