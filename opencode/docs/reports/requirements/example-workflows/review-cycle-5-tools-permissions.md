# Critical Review Cycle 5: Tool and Permission Model

**Date**: 2026-03-08
**Scope**: Tool and Permission Model
**Reviewer**: Sisyphus

---

## Review Process

Reviewed tool definitions, tool execution, permissions (global, step, tool, folder), and permission enforcement.

---

## Findings

### 1. Tool Type Coverage

**Coverage**: Partial - file, shell tools shown, but others missing

**Present**:
- ✅ File operations (read, write, delete, move, search) - ex04
- ✅ Shell operations - ex03, ex04
- ✅ Sub-agent tools - ex03

**Missing**:
- ❌ Directory operations (create, delete, list, traverse)
- ❌ Web operations (fetch, scrape, search, forum search)
- ❌ RAG operations - shown in ex06 but not validated in permission model
- ❌ CLI tool execution
- ❌ Script execution with env vars, working dir, timeout

**Example Gap**: No example shows:
```yaml
tools:
  - name: directory_operations
    operations:
      - create: /workspace/output
      - list: /workspace/src
      - traverse: /workspace
    timeout_secs: 30

  - name: web_operations
    fetch:
      url: https://docs.rs/tokio
      headers:
        User-Agent: "My Agent"
    scrape:
      url: https://example.com
      extract: [text, links, images]
    search:
      engine: google
      max_results: 10
```

---

### 2. Tool Execution Syntax

**Issue**: Inconsistent tool execution approaches

**ex03**:
```yaml
tool:
  type: shell_exec
  command: ["cargo", "test"]
  working_dir: "${workflow.codebase_path}"
  timeout_secs: 300
```

**ex04**:
```yaml
tool:
  type: shell_exec
  command: ["cargo", "test"]
  working_dir: "${workflow.codebase_path}"
  timeout_secs: 300
```

**Question**: Where does `tool:` go? Is it a step attribute or a separate tool call?

**Problem**: ex03 shows tool as part of step, but unclear if this is how all tools are called

**Recommendation**: Unified tool execution syntax:
```yaml
# Option A: Tool as step attribute
- step: run_tests
  id: step_4
  tool_call:
    tool: test_runner
    config:
      command: ["cargo", "test"]
      working_dir: "${workflow.codebase_path}"
      timeout_secs: 300

# Option B: Tool definition, then tool call
tools:
  - name: test_runner
    type: shell_exec
    default_command: ["cargo", "test"]
    default_timeout_secs: 300

pipeline:
  - step: run_tests
    tool: test_runner
    args:
      working_dir: "${workflow.codebase_path}"
```

---

### 3. Tool Permission Scoping

**Issue**: Permission inheritance and override unclear

**ex04**:
```yaml
permissions:
  global:
    allow:
      - file_read: /workspace/src/*
    deny:
      - file_delete: /workspace/src/*

  step_permissions:
    analyze_code:
      allow:
        - file_read: /workspace/src/*
```

**Questions**:
- Do step permissions override global permissions completely or merge?
- If step allows `file_read: /workspace/src/*` and global denies `file_delete: /workspace/src/*`, can step delete files?
- What if both allow and deny specify same operation on same path?

**Recommendation**: Explicit permission scoping rules:
```yaml
permissions:
  global:
    allow:
      - file_read: /workspace/src/*
    deny:
      - file_delete: /workspace/src/*

  step_override:
    step_1:
      strategy: merge  # merge, override, ignore_global
      allow:
        - file_write: /workspace/src/*
      deny:
        - shell_exec: rm -rf

  agent_override:
    code_analyzer:
      allow:
        - tool: grep
      deny:
        - tool: shell_exec
```

---

### 4. Folder Permission Model

**Issue**: Folder permissions not fully validated in examples

**ex04**:
```yaml
folder_permissions:
  - path: /workspace/src
    operations: [read, search]
    allowed_tools: [file_read, grep, file_search]
    allowed_steps: [analyze_code, generate_fixes]
```

**Questions**:
- What happens if step tries to use tool not in `allowed_tools`?
- What happens if step tries to read path outside folder?
- How are folder permissions enforced alongside tool permissions?

**Recommendation**: Add permission enforcement semantics:
```yaml
folder_permissions:
  - path: /workspace/src
    operations: [read, search]
    allowed_tools: [file_read, grep, file_search]
    allowed_steps: [analyze_code, generate_fixes]
    enforcement:
      strict: true  # Block disallowed operations
      log_denials: true
      escalation: warn  # warn, block, escalate
```

---

### 5. Tool-Level Permissions

**Gap**: Tool-level permissions not shown in examples

**Missing Features**:
- Per-tool permission settings
- Tool timeouts
- Tool retry configuration
- Tool output validation

**Use Case**: `grep` tool should have different timeout than `shell_exec`

**Example Gap**:
```yaml
# NOT SHOWN IN EXAMPLES
tools:
  - name: grep
    type: grep
    permissions:
      max_file_size_mb: 100
      max_search_results: 1000
      timeout_secs: 30
    retry:
      max_attempts: 3
      on_timeout: reduce_search_scope

  - name: shell_exec
    type: shell_exec
    permissions:
      allowed_commands: [cargo, git, python]
      denied_commands: [rm, sudo, docker]
      max_execution_time_secs: 300
```

---

### 6. URL Parameter Support

**Gap**: URL operations not shown with full parameter set

**Requirements mention**:
- Headers
- Timeout
- Method (GET, POST)
- Follow redirects
- SSL validation
- Authentication

**Example Gap**: No example shows:
```yaml
# NOT SHOWN IN EXAMPLES
web_fetch:
  url: https://api.example.com/data
  method: POST
  headers:
    Authorization: "Bearer ${workflow.api_key}"
    Content-Type: "application/json"
  body:
    query: "${step.previous_output}"
  timeout_secs: 30
  follow_redirects: true
  validate_ssl: true
  auth:
    type: bearer_token
    token: "${workflow.api_key}"
```

---

### 7. Script and CLI Execution

**Gap**: Script execution with environment variables not shown

**Requirements mention**:
- Working directory
- Environment variables
- Timeout
- Output capture
- Error handling

**Example Gap**: No example shows:
```yaml
# NOT SHOWN IN EXAMPLES
script_run:
  script: /scripts/analyze.sh
  args:
    - "--input"
    - "${workflow.input_file}"
    - "--output"
    - "${workflow.output_file}"
  working_dir: /workspace
  env_vars:
    MODEL_PATH: "${workflow.model_path}"
    API_KEY: "${workflow.api_key}"
    MAX_RETRIES: "3"
  timeout_secs: 60
  capture_output:
    stdout: /workspace/logs/script_output.log
    stderr: /workspace/logs/script_errors.log
  on_failure:
    retry: true
    notify: true
```

---

### 8. Permission Denial Handling

**Gap**: What happens when permission is denied?

**Use Cases**:
- Step tries to read file outside allowed paths
- Step tries to execute denied command
- Step tries to use tool not in allow list

**Example Gap**: No example shows:
```yaml
# NOT SHOWN IN EXAMPLES
permissions:
  on_denial:
    action: block  # block, warn, log_only, escalate
    notify_user: true
    log_to: /workspace/permission_denials.log
    fallback:
      step: notify_admin
      action: request_approval
```

---

### 9. Dynamic Permission Changes

**Gap**: Can permissions change during workflow execution?

**Use Case**: Grant more permissions after certain step succeeds

**Example Gap**:
```yaml
# NOT SHOWN IN EXAMPLES
permissions:
  dynamic:
    enabled: true
    rules:
      - trigger: step_3_success
        grant:
          - file_write: /workspace/src/*
        reason: "Code validated, safe to write"

      - trigger: step_5_failure
        revoke:
          - file_delete: /workspace/*
        reason: "Error occurred, unsafe to delete"
```

---

### 10. Permission Logging and Auditing

**Gap**: Permission audit trail not shown in examples

**Use Cases**:
- Track all permission denials
- Audit who accessed what
- Compliance reporting

**Example Gap**:
```yaml
# NOT SHOWN IN EXAMPLES
permissions:
  audit:
    enabled: true
    log_all_operations: true
    log_denials: true
    audit_file: /workspace/audit/permissions.log
    report:
      - generate_on_completion: true
      - format: json
      - include:
          - operation_type
          - step_id
          - agent_id
          - path
          - outcome
          - timestamp
```

---

## Critical Gaps

### 1. Complete Tool Catalog

**Missing**: No comprehensive tool catalog shown

**Gap**: What tools are available? How to define custom tools?

**Recommendation**: Tool catalog schema:
```yaml
tools:
  catalog:
    - name: file_read
      type: built_in
      description: Read file contents
      parameters:
        - name: path
          type: string
          required: true
        - name: encoding
          type: string
          default: utf-8
      permissions:
        default_deny: false

    - name: grep
      type: built_in
      description: Search file contents
      parameters:
        - name: pattern
          type: string
          required: true
        - name: path
          type: string
          required: true
```

---

### 2. Custom Tool Definition

**Missing**: How to define custom tools

**Use Case**: Add project-specific tools

**Example Gap**:
```yaml
# NOT SHOWN IN EXAMPLES
tools:
  custom:
    - name: project_analyzer
      type: script
      script: /tools/project_analyzer.py
      interface:
        - name: analyze
          input:
            - name: codebase
              type: string
          output:
            - name: metrics
              type: object
        - name: report
          input:
            - name: metrics
              type: object
          output:
            - name: report
              type: string
```

---

### 3. Tool Composition

**Missing**: Can tools be combined?

**Use Case**: Chain multiple tools in one step

**Example Gap**:
```yaml
# NOT SHOWN IN EXAMPLES
- step: analyze_and_report
  tools:
    - tool: grep
      config:
        pattern: "async fn"
        path: /workspace/src
      output_to: grep_results

    - tool: file_analyzer
      input:
        results: "${grep_results}"
```

---

## Summary of Tool and Permission Gaps

1. ❌ Directory operations missing
2. ❌ Web operations missing
3. ❌ RAG operations not validated in permission model
4. ❌ CLI tool execution missing
5. ❌ Script execution with env vars missing
6. ❌ Tool execution syntax inconsistent
7. ❌ Permission inheritance/override unclear
8. ❌ Folder permission enforcement undefined
9. ❌ Tool-level permissions not shown
10. ❌ URL parameters incomplete
11. ❌ Permission denial handling missing
12. ❌ Dynamic permission changes missing
13. ❌ Permission auditing missing
14. ❌ Complete tool catalog missing
15. ❌ Custom tool definition missing
16. ❌ Tool composition missing

**Total Gaps**: 16

---

## Action Items

### High Priority
1. Add directory operation examples
2. Add web operation examples (fetch, scrape, search, forum)
3. Define tool execution syntax clearly
4. Add permission scoping rules (inherit, override, merge)

### Medium Priority
5. Add tool-level permission examples
6. Add URL parameter examples
7. Add permission denial handling
8. Add permission auditing

### Low Priority
9. Add tool catalog schema
10. Add custom tool definition
11. Add dynamic permission changes
12. Add tool composition examples

---

## Review Cycle Summary

### Overall Findings

**Total Schema Gaps Identified**: 66
- Cycle 1 (Schema Completeness): 12 gaps
- Cycle 2 (Workflow Coherence): 8 gaps
- Cycle 3 (Model Management): 13 gaps
- Cycle 4 (Control Flow): 15 gaps
- Cycle 5 (Tool & Permissions): 16 gaps

**Total Inconsistencies Identified**: 11
- Loop integration inconsistencies
- Variable reference inconsistencies
- Retry scoping inconsistencies
- Tool execution syntax inconsistencies
- Memory management inconsistencies
- Model loading inconsistencies
- RAG operation inconsistencies
- Permission scoping inconsistencies

**Recommended Schema Improvements**: 40+
- Unified model configuration
- Unified permission model
- Unified retry schema
- Unified loop schema
- Unified tool execution syntax
- Explicit state management
- Explicit error handling
- Step dependency specification
- Performance tracking
- Version control

---

## Conclusion

Schema is conceptually sound but has significant gaps and inconsistencies that must be addressed before implementation.

**Priority Actions**:
1. Address 16 high-priority gaps identified across all review cycles
2. Resolve 11 major inconsistencies
3. Create missing examples for critical features (RAG, web operations, directory operations)
4. Standardize terminology and syntax across all schema elements

**Next Phase**: Refine schema based on review findings, then begin implementation planning.

---

## End of Review Cycles

All 5 critical review cycles completed.
