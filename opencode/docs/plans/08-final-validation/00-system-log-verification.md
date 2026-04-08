# Validation 00: System Log Verification

## Purpose
Verify that all 9 logging scopes produce correct structured output with proper log rotation and per-scope log levels.

## Test Procedures

### 1. Verify All 9 Logging Scopes

**Test:** Execute the agent SDK with all 9 logging scopes enabled at DEBUG level.

**Command:**
```bash
cargo run --bin agentsdk -- \
  --log-level debug \
  --log-format json \
  --log-scope workflow \
  --log-scope execution \
  --log-scope model \
  --log-scope backend \
  --log-scope queue \
  --log-scope scheduler \
  --log-scope cli \
  --log-scope system \
  --log-scope security \
  run opencode/examples/simple-workflow.yaml
```

**Expected Results:**
- All 9 scopes produce log entries
- Each log entry has: `timestamp`, `level`, `scope`, `message`, `metadata`
- JSON format is valid (can be parsed by `jq`)
- Scope field contains one of the 9 valid scopes
- Level field is one of: `error`, `warn`, `info`, `debug`, `trace`

**Verification:**
```bash
# Check all scopes appear
cat ~/.agentsdk/logs/agentsdk.log | jq -r '.scope' | sort -u
# Expected output:
# backend
# cli
# execution
# model
# queue
# scheduler
# security
# system
# workflow

# Verify each scope has entries
echo "workflow: $(cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="workflow") | .message' | wc -l)"
echo "execution: $(cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="execution") | .message' | wc -l)"
echo "model: $(cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="model") | .message' | wc -l)"
echo "backend: $(cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="backend") | .message' | wc -l)"
echo "queue: $(cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="queue") | .message' | wc -l)"
echo "scheduler: $(cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="scheduler") | .message' | wc -l)"
echo "cli: $(cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="cli") | .message' | wc -l)"
echo "system: $(cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="system") | .message' | wc -l)"
echo "security: $(cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="security") | .message' | wc -l)"
# All should be > 0

# Verify JSON validity
cat ~/.agentsdk/logs/agentsdk.log | jq '.' > /dev/null
# Expected: No errors (command exits with 0)

# Verify log entry structure
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'keys[]' | head -1 | sort -u
# Expected output:
# level
# message
# metadata
# scope
# timestamp
```

### 2. Verify Log Rotation

**Test:** Generate enough logs to trigger rotation and verify old logs are archived.

**Command:**
```bash
# Clear existing logs
rm -rf ~/.agentsdk/logs/*

# Generate logs that exceed rotation threshold (100MB by default)
for i in {1..100}; do
  cargo run --bin agentsdk -- \
    --log-level debug \
    --log-format json \
    run opencode/examples/simple-workflow.yaml >> /dev/null 2>&1 || true
done
```

**Expected Results:**
- Primary log file is rotated when it exceeds 100MB
- Rotated logs are named with timestamp (e.g., `agentsdk.log.2024-01-15T10:30:00Z`)
- At most 10 rotated log files are kept
- Oldest logs are deleted when limit is exceeded

**Verification:**
```bash
ls -lh ~/.agentsdk/logs/
# Expected: Shows rotated log files with timestamps
# Example output:
# total 500M
# -rw-r--r-- 1 user group 50M Jan 15 10:35 agentsdk.log
# -rw-r--r-- 1 user group 50M Jan 15 10:30 agentsdk.log.2024-01-15T10:30:00Z
# -rw-r--r-- 1 user group 50M Jan 15 10:25 agentsdk.log.2024-01-15T10:25:00Z
# ...

# Verify rotation worked (at least one rotated file exists)
ls ~/.agentsdk/logs/agentsdk.log.* | wc -l
# Expected: >= 1

# Verify current log file exists and is being written to
test -f ~/.agentsdk/logs/agentsdk.log
# Expected: Exit code 0

# Verify rotated log files are valid JSON
for rotated_log in ~/.agentsdk/logs/agentsdk.log.*; do
  cat "$rotated_log" | jq '.' > /dev/null || echo "Invalid JSON in $rotated_log"
done
# Expected: No errors

# Verify rotation count limit (max 10 files)
ls ~/.agentsdk/logs/agentsdk.log.* | wc -l
# Expected: <= 10
```

### 3. Verify Log Formats

**Test:** Test all three log formats (JSON, chat, log).

#### 3.1 JSON Format

**Command:**
```bash
rm -rf ~/.agentsdk/logs/*
cargo run --bin agentsdk -- \
  --log-level info \
  --log-format json \
  run opencode/examples/simple-workflow.yaml
```

**Expected Results:**
- All log entries are valid JSON
- Each line is a complete JSON object
- JSON objects have the required fields

**Verification:**
```bash
# Verify JSON validity
cat ~/.agentsdk/logs/agentsdk.log | jq '.' > /dev/null
# Expected: Exit code 0

# Verify each line is a complete JSON object
while IFS= read -r line; do
  echo "$line" | jq '.' > /dev/null || echo "Invalid JSON line: $line"
done < ~/.agentsdk/logs/agentsdk.log
# Expected: No errors

# Check log entry structure
head -5 ~/.agentsdk/logs/agentsdk.log | jq '.'
# Expected output (example):
# {
#   "timestamp": "2024-01-15T10:30:00.123Z",
#   "level": "info",
#   "scope": "workflow",
#   "message": "Starting workflow execution",
#   "metadata": {
#     "workflow_id": "abc123",
#     "workflow_name": "simple-workflow"
#   }
# }
```

#### 3.2 Chat Format

**Command:**
```bash
rm -rf ~/.agentsdk/logs/*
cargo run --bin agentsdk -- \
  --log-level info \
  --log-format chat \
  run opencode/examples/simple-workflow.yaml
```

**Expected Results:**
- Human-readable chat-like format
- Each entry has `[LEVEL] [SCOPE]` prefix
- Message follows in natural language

**Verification:**
```bash
head -20 ~/.agentsdk/logs/agentsdk.log
# Expected output (example):
# [INFO] [workflow] Starting workflow execution: simple-workflow
# [DEBUG] [model] Preparing model request to backend
# [INFO] [execution] Executing step 1 of 3
# [DEBUG] [backend] Sending request to http://localhost:11434
# [INFO] [backend] Received response in 1.234s
# [INFO] [execution] Step 1 completed
# [INFO] [execution] Executing step 2 of 3
# ...

# Verify format pattern (LEVEL and SCOPE in brackets)
grep -c "\[INFO\] \[" ~/.agentsdk/logs/agentsdk.log
# Expected: > 0

# Verify scope appears in brackets
grep -c "\[INFO\] \[workflow\]" ~/.agentsdk/logs/agentsdk.log
# Expected: > 0
```

#### 3.3 Log Format

**Command:**
```bash
rm -rf ~/.agentsdk/logs/*
cargo run --bin agentsdk -- \
  --log-level info \
  --log-format log \
  run opencode/examples/simple-workflow.yaml
```

**Expected Results:**
- Traditional log format with timestamp and scope in brackets
- Format: `TIMESTAMP LEVEL [scope] Message`

**Verification:**
```bash
head -20 ~/.agentsdk/logs/agentsdk.log
# Expected output (example):
# 2024-01-15T10:30:00.123Z INFO [workflow] Starting workflow execution
# 2024-01-15T10:30:01.456Z DEBUG [model] Preparing model request
# 2024-01-15T10:30:02.789Z INFO [execution] Executing step 1 of 3
# 2024-01-15T10:30:04.012Z DEBUG [backend] Sending request to backend
# ...

# Verify timestamp format (ISO 8601)
grep -c "^2024-[0-9][0-9]-[0-9][0-9]T[0-9][0-9]:[0-9][0-9]:[0-9][0-9]\." ~/.agentsdk/logs/agentsdk.log
# Expected: > 0

# Verify level appears after timestamp
grep -c "^2024-.*INFO " ~/.agentsdk/logs/agentsdk.log
# Expected: > 0
```

### 4. Verify Per-Scope Log Levels

**Test:** Set different log levels for different scopes.

**Command:**
```bash
rm -rf ~/.agentsdk/logs/*
cargo run --bin agentsdk -- \
  --log-level error \
  --log-scope-level workflow=debug \
  --log-scope-level model=info \
  --log-scope-level execution=warn \
  run opencode/examples/simple-workflow.yaml
```

**Expected Results:**
- `workflow` scope shows DEBUG and above
- `model` scope shows INFO and above
- `execution` scope shows WARN and above
- Other scopes only show ERROR and above

**Verification:**
```bash
# Verify workflow has DEBUG entries
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="workflow" and .level=="debug") | .message' | wc -l
# Expected: > 0

# Verify workflow has INFO entries
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="workflow" and .level=="info") | .message' | wc -l
# Expected: > 0

# Verify model has INFO entries but no DEBUG
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="model" and .level=="info") | .message' | wc -l
# Expected: > 0

cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="model" and .level=="debug") | .message' | wc -l
# Expected: 0

# Verify execution has WARN entries but no INFO
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="execution" and .level=="warn") | .message' | wc -l
# Expected: > 0

cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="execution" and .level=="info") | .message' | wc -l
# Expected: 0

# Verify other scopes only have ERROR
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="backend" and .level=="error") | .message' | wc -l
# Expected: >= 0 (may be 0 if no errors occurred)

cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="backend" and .level=="warn") | .message' | wc -l
# Expected: 0
```

### 5. Verify Log Filtering by Scope

**Test:** Enable only specific scopes and verify others are not logged.

**Command:**
```bash
rm -rf ~/.agentsdk/logs/*
cargo run --bin agentsdk -- \
  --log-level debug \
  --log-scope workflow \
  --log-scope model \
  --log-scope backend \
  run opencode/examples/simple-workflow.yaml
```

**Expected Results:**
- Only `workflow`, `model`, and `backend` scopes produce log entries
- Other scopes (execution, queue, scheduler, cli, system, security) do not appear

**Verification:**
```bash
# Verify enabled scopes appear
cat ~/.agentsdk/logs/agentsdk.log | jq -r '.scope' | sort -u
# Expected output:
# backend
# model
# workflow

# Verify disabled scopes do not appear
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="execution") | .message' | wc -l
# Expected: 0

cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="queue") | .message' | wc -l
# Expected: 0

cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="scheduler") | .message' | wc -l
# Expected: 0

cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="cli") | .message' | wc -l
# Expected: 0

cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="system") | .message' | wc -l
# Expected: 0

cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="security") | .message' | wc -l
# Expected: 0
```

### 6. Verify Log Metadata

**Test:** Verify that log entries include relevant metadata.

**Command:**
```bash
rm -rf ~/.agentsdk/logs/*
cargo run --bin agentsdk -- \
  --log-level info \
  --log-format json \
  run opencode/examples/simple-workflow.yaml
```

**Expected Results:**
- Log entries include relevant metadata (workflow_id, step_id, model_id, etc.)
- Metadata is structured and queryable

**Verification:**
```bash
# Check metadata field exists
cat ~/.agentsdk/logs/agentsdk.log | jq 'select(.metadata != null) | .message' | wc -l
# Expected: > 0

# Check workflow_id in metadata
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="workflow") | .metadata.workflow_id' | head -1
# Expected: A valid UUID or workflow identifier

# Check model metadata in model scope logs
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="model") | .metadata.model_name' | head -1
# Expected: A model name (e.g., "llama2", "mistral", etc.)

# Check timing metadata
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.metadata.duration != null) | .metadata.duration' | head -1
# Expected: A duration value (e.g., "1.234s", "1234ms")
```

### 7. Verify Error Logging

**Test:** Intentionally trigger an error and verify error logging works correctly.

**Command:**
```bash
rm -rf ~/.agentsdk/logs/*
cargo run --bin agentsdk -- \
  --log-level error \
  --log-format json \
  run opencode/examples/invalid-workflow.yaml
```

**Expected Results:**
- Error logs are produced with level `error`
- Error messages are descriptive
- Error metadata includes stack trace or error code

**Verification:**
```bash
# Check for error logs
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.level=="error") | .message'
# Expected: Descriptive error messages

# Check error metadata
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.level=="error") | .metadata' | head -20
# Expected: Error metadata with error_code, stack_trace, etc.

# Count error logs
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.level=="error") | .message' | wc -l
# Expected: > 0
```

## Success Criteria

- ✅ All 9 logging scopes (workflow, execution, model, backend, queue, scheduler, cli, system, security) produce output
- ✅ All log entries are valid JSON (when using JSON format)
- ✅ Log rotation works correctly (files rotate at 100MB, max 10 files)
- ✅ All three log formats (json, chat, log) work correctly
- ✅ Per-scope log levels are respected
- ✅ Log filtering by scope works correctly
- ✅ Log metadata is included and structured
- ✅ Error logging captures errors with proper metadata
- ✅ No log entries are malformed or missing required fields

## Performance Targets

- Log entry writing: < 1ms per entry
- Log rotation: < 100ms to rotate file
- Log parsing with jq: < 10s for 10MB log file

## Failure Procedures

**If a scope is not producing output:**
1. Check scope spelling in command-line arguments
2. Verify code has log statements for that scope
3. Check if log level filters are too restrictive (DEBUG may be needed)
4. Review code to ensure scope is actually used in the relevant module

**If rotation is not working:**
1. Check log file size thresholds in configuration file
2. Verify disk permissions on log directory
3. Check if log files are being held by other processes (lsof on the log file)
4. Review rotation implementation for bugs

**If per-scope levels are not working:**
1. Verify command-line argument syntax (`--log-scope-level scope=level`)
2. Check if config file overrides command-line arguments
3. Review log filter implementation for bugs
4. Verify level comparison logic (is DEBUG < INFO < WARN < ERROR?)

**If log formats are not working:**
1. Check format argument value (`--log-format json|chat|log`)
2. Verify format implementation exists for the specified format
3. Review format-specific code for bugs
4. Test with different formats to isolate the issue

**If metadata is missing:**
1. Verify metadata is being set in log statements
2. Check if metadata fields are properly structured
3. Review metadata serialization logic
4. Test with different operations to see if metadata appears elsewhere

## Sign-Off Checklist

- [ ] All 9 scopes verified to produce output
- [ ] Log rotation verified with > 100MB of logs
- [ ] All three formats (json, chat, log) tested and working
- [ ] Per-scope levels verified with different scopes at different levels
- [ ] Log filtering by scope verified
- [ ] Log metadata verified to include relevant information
- [ ] Error logging verified with intentional error
- [ ] No malformed log entries found
- [ ] All verification commands pass
- [ ] Performance targets met
