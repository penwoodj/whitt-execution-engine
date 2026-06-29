# Task: Implement Streaming Error Detection

## Objective
The `during_step_streaming` trigger is defined but NOT WIRED in the runner. The engine uses `stream: false` and gets complete responses only. If the model starts refusing mid-generation, it's not detected until the full response is received.

## Requirements
1. Read `src/agent/streaming.rs` to understand existing streaming infrastructure
2. Read `src/benchmark/runner.rs` to find where streaming could be enabled
3. Implement streaming with real-time error detection:
   - Enable SSE streaming in inference requests
   - Process chunks as they arrive
   - Check each chunk for refusal patterns ("I cannot", "As an AI")
   - If refusal detected: terminate generation early, save partial output
   - Fire `during_step_streaming` hook with each chunk
4. Write the complete implementation

## Output
Complete Rust implementation including:
- SSE chunk processing
- Real-time refusal detection
- Early termination logic
- Hook firing per chunk
- Integration with existing inference path
