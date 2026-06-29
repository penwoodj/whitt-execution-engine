# Task: Add Error Recovery to Inference Loop

## Objective
The benchmark runner's inference loop in `src/benchmark/runner.rs` currently retries 3 times on failure with the SAME prompt. This is blind retry — it never works if the first attempt failed.

## Requirements
1. Read `src/benchmark/runner.rs` and find the retry loop (search for `MAX_RETRIES` and `attempts failed`)
2. Implement an `AdjustmentStrategy` enum with 3 variants: `ShortenPrompt`, `IncreaseMaxTokens`, `SimplifyTemperature`
3. On each retry, apply the adjustment:
   - Retry 1: ShortenPrompt (remove system prompt, keep only user message)
   - Retry 2: IncreaseMaxTokens (double the max_tokens)
   - Retry 3: SimplifyTemperature (set temperature to 0.1 for deterministic output)
4. Add a quality check after each inference: if output contains "I cannot" or "I'm unable to", treat as failure
5. Write the implementation as a complete Rust module

## Output
A markdown document containing:
- The `AdjustmentStrategy` enum definition
- The modified retry loop
- The quality check function
- Integration notes for `src/benchmark/runner.rs`
