# Cycle 2 Design — Engine Tool Implementation (BACKUP)

**Status:** SKETCH — only execute if Cycle 1 (template rewrite) fails to achieve ≥8/11 parity.

## Rationale

If template rewrite insufficient, root cause = engine lacks real tool access. Implement minimal set:

## Minimum Viable Tool Set

### 1. ShellTool (HIGHEST PRIORITY)

**Why:** Bash access unlocks file read, file write (via echo), grep, cargo run, etc. Single tool = many capabilities.

**Implementation:**
```rust
pub struct ShellTool;

impl Tool for ShellTool {
    fn name(&self) -> &str { "shell_exec" }
    
    async fn execute(&self, input: serde_json::Value) -> Result<serde_json::Value> {
        let command = input["command"].as_str()
            .ok_or_else(|| anyhow::anyhow!("command required"))?;
        let args = input["args"].as_array()
            .map(|a| a.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();
        let timeout_secs = input["timeout_seconds"].as_u64().unwrap_or(30);
        let working_dir = input["working_dir"].as_str().unwrap_or(".");
        
        let mut cmd = tokio::process::Command::new(command);
        cmd.args(&args).current_dir(working_dir);
        
        let output = tokio::time::timeout(
            std::time::Duration::from_secs(timeout_secs),
            cmd.output()
        ).await??;
        
        Ok(serde_json::json!({
            "stdout": String::from_utf8_lossy(&output.stdout),
            "stderr": String::from_utf8_lossy(&output.stderr),
            "exit_code": output.status.code().unwrap_or(-1),
            "success": output.status.success(),
        }))
    }
}
```

**Wiring:**
- Add to ToolRegistry in src/agent/tools.rs
- In benchmark/runner.rs execute_workflow_step: if step has `tool: shell_exec`, dispatch to ShellTool instead of chat_completion
- Step output becomes tool result (JSON), not model text

**Schema update:** unified-workflow-schema.yml line 379-382 already declares example. Make it real.

### 2. FileWriteTool (MEDIUM PRIORITY)

**Why:** Direct file write without shell `echo` escaping issues.

**Implementation:** Similar pattern. Takes `path` + `content`. Writes via tokio::fs.

### 3. FileReadTool for steps (LOW PRIORITY)

**Why:** Already implemented for ReAct. Just wire to workflow steps.

**Wiring change:** In execute_workflow_step, if `tool: file_read`, dispatch to existing FileReadTool.

### 4. GrepTool + WebFetchTool (DEFERRED)

**Why:** Only needed for specific prompts (research-heavy). Implement only if Cycle 2 still fails on those.

## Workflow Step Dispatch Logic

```rust
// In execute_workflow_step (pseudocode)
match &step.tool {
    Some(tool_name) => {
        // Tool-based execution
        let tool = registry.get(tool_name)?;
        let result = tool.execute(step.input.clone()).await?;
        let output_text = serde_json::to_string_pretty(&result)?;
        // Store as step output, run hooks
    }
    None => {
        // Existing chat_completion path
        let response = client.chat_completion(request).await?;
        // ...existing logic
    }
}
```

## ReAct-style Step (ADVANCED — Cycle 3)

For prompts needing multi-step reasoning with tools:
- Step declares `react: true` + `tools: [shell_exec, file_read]`
- Engine runs ReAct loop within step
- Model can call tools iteratively until final_answer

This would match opencode baseline most closely. But significant engine work.

## Time Estimates

| Component | Est. Hours |
|-----------|-----------|
| ShellTool + registry wire | 3-4h |
| FileWriteTool | 1-2h |
| FileReadTool for steps | 1h |
| Tests for each | 2h |
| **Cycle 2 total** | **7-9h** |

| ReAct-style steps (Cycle 3) | +6-8h |

## Decision Rule

Run Cycle 2 ONLY IF:
- Cycle 1 parity score < 8/11
- AND gap analysis shows tool access is blocker (not template quality)

Otherwise: declare acceptable parity, document limitations, move to Phase 8.

## Risks

1. **Sandbox safety:** ShellTool runs arbitrary commands. Need sandboxing (limit dirs, env vars, timeouts).
2. **Backward compat:** Don't break existing workflows. Make `tool:` field optional.
3. **Schema migration:** Update unified-workflow-schema.yml. Validate existing workflows still pass.
