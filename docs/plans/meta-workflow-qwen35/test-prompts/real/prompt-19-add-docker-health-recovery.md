# Task: Add Docker Health Recovery

## Objective
When the Docker llama.cpp server returns 500 errors or connection failures, the engine retries 3 times rapidly (all fail) and then permanently fails the step. This kills 70-minute pipeline runs.

## Requirements
1. Read `src/benchmark/runner.rs` to find the inference retry logic
2. Read `src/client/http_client.rs` to understand HTTP error handling
3. Implement a `DockerHealthMonitor` that:
   - Detects transient errors (500 Internal Server Error, connection refused)
   - On transient error: waits 10 seconds, restarts Docker container, reloads model
   - After Docker restart: retries the inference with fresh connection
   - Maximum 2 Docker restarts per workflow run
4. The restart function should call `restart_docker_with_env_vars()` (already exists)
5. Write the complete implementation

## Output
Complete Rust implementation including:
- `DockerHealthMonitor` struct
- Error classification (transient vs permanent)
- Restart sequence (wait → restart → reload → retry)
- Integration with existing inference loop
