<!--
Source Session: ses_2e81b5decffeLZjpnU2ow7Wn03
Part ID: prt_d2c0eac730019pytD2S7UjWJDr
Character Count: 102828
Extracted: 2026-06-17T07:50:39Z
-->

heres some context.  give me 3 options of next step chains to pick up where we left off after giving me a status update  do all of that because I do want that, all of it, done extensively and with thorough validation and verification with looking and running actual real local files.  After you're done I want you to resume with the last instructions I have in this chat context history of this chat:


These are cross-cutting configuration blocks that attach to step types via the named key.
Applicability Map
Meta Schema 	Step Types
retry 	cli_exec, file_read, file_write, file_delete, http_request, script_run, use_website_ui, db_query, db_mutation, kb_search, kb_write, llm_prompt_response, agentic_attempts, user_input, user_approval, user_review, sub_workflow, sub_workflow_parallel
input 	all 27 step types
output 	all 27 step types
attention 	llm_prompt_response, agentic_attempts
guardrails 	llm_prompt_response, agentic_attempts
hooks 	llm_prompt_response, agentic_attempts
tool_permissions 	cli_exec, file_read, file_write, file_delete, http_request, script_run, use_website_ui, db_query, db_mutation, kb_search, kb_write, agentic_attempts
model_config 	llm_prompt_response, agentic_attempts
retry

retry:
  condition: "error.is_retryable == true"
  max_attempts: 3
  backoff: exponential
  delay_ms: 1000
  base_ms: 1000
  max_ms: 30000
  jitter: 0.2
  level: step_unload
  adjustment_strategy: loosen_tolerance
  tolerance_adjustment: 0.15
  adjust_interdependencies: false
  checkpoint_after_retry: true

┌─────────────────────────────────────────────────────────┐
│ RETRY FLOW                                              │
│                                                         │
│  Step executes ──→ success? ──→ continue               │
│       │                  │                              │
│       │                  ↓ no                           │
│       │           condition met?                        │
│       │           │            │                       │
│       │          yes           no ──→ fail              │
│       │            │                                   │
│       │    attempt < max?                              │
│       │     │          │                               │
│       │    yes        no ──→ fail                     │
│       │     │                                          │
│       │     ├─ apply adjustment_strategy               │
│       │     ├─ apply tolerance_adjustment              │
│       │     ├─ checkpoint (if enabled)                 │
│       │     ├─ calculate backoff delay                 │
│       │     │    linear:  delay_ms * attempt           │
│       │     │    exponential: base_ms * 2^(attempt-1)   │
│       │     │                  + jitter * random         │
│       │     │    capped at max_ms                      │
│       │     ├─ unload/reload at LEVEL                  │
│       │     │    full_workflow_unload_reload             │
│       │     │    workflow_restart                       │
│       │     │    step_unload (model)                    │
│       │     │    step_restart (no model reload)         │
│       │     │    prompt_restart (same model state)       │
│       │     │    tools_retry (just failed tool calls)   │
│       │     └─ re-execute step                         │
│       │                                               │
│       └─── retry loop                                │
└─────────────────────────────────────────────────────────┘

input

input:
  prompt: |
    Analyze the codebase at ${workspace_path}.
    Previous analysis: ${step.step_2.output}
    Aggregated results from parallel phase:
    ${step.step_3_parallel.output}
    System state: ${system_state}
  workspace_path: "/workspace"
  system_state: "${step.step_1.output}"
  branch_analysis: "${step.step_6_analysis.output}"
  aggregated_results: "${step.step_4.output}"
  orchestration_config: "${orchestration}"
  retry_branch_results: "${step.step_7_retry.output}"
  hardcoded_values:
    var_asdf: foo
    var_fdsa:
      - bar: 1
        baz: false
  strict_yes_no_required: false

┌─────────────────────────────────────────────────────────────┐
│ INPUT RESOLUTION ORDER                                     │
│                                                             │
│  ${step.step_2.output}     ← previous step output           │
│  ${var_fdsa.0.baz}         ← hardcoded_values (top-level)   │
│  ${orchestration}           ← workflow-level reference        │
│  ${result.token_usage.total} ← framework runtime variable   │
│  ${duration_ms}             ← framework runtime variable   │
│  ${workflow.trace_id}       ← framework runtime variable   │
│  ${tool.result.output}      ← framework runtime variable   │
│  ${error.is_retryable}      ← framework runtime variable   │
│  ${now_iso8601}             ← framework runtime variable   │
│  ${random_float}            ← framework runtime variable   │
│                                                             │
│  ── Expression engine (available in conditions/interpolation) │
│                                                             │
│  comparison:   ==, !=, >, >=, <, <=                        │
│  logical:      &&, ||, !                                  │
│  string:       contains, starts_with, ends_with, matches,  │
│                 length                                     │
│  collection:   in, not_in, empty, not_empty, length, all,  │
│                 any, none, has                             │
│  type:         is_null, is_not_null, is_type              │
│  existence:    defined, undefined                           │
│  arithmetic:   *, /, %, +, -                                │
│  get:          . (fault-tolerant nested access)              │
│                                                             │
│  Example: result.accumulated_tool_calls any .is_error == true │
│  Example: ${step.step_4.output.weighted_overall_score} >= 0.85│
└─────────────────────────────────────────────────────────────┘

output

output:
  save_to: aggregated_results
  format: json
  fields:
    - code_quality_score
    - documentation_completeness
    - all_sub_workflows_valid
    - performance_acceptable
    - weighted_overall_score
    - final_decision
    - criteria_breakdown
    - recommendations
    - logging:
        log_path:
        log_file_written:
        last_log_entry:
  structured_output: true
  schema_ref: "/schemas/analysis_output.json"
  quality_score:
    weighted_validation: true
    criteria:
      - name: code_quality_score
        weight: 0.3
        threshold: 0.9
        actual: "${step.step_3_parallel.output.code_quality_score}"
        passed: true
      - name: documentation_completeness
        weight: 0.25
        threshold: 0.85
        actual: "${step.step_3_parallel.output.documentation_completeness}"
        passed: true
      - name: all_sub_workflows_valid
        weight: 0.25
        threshold: 1.0
        actual: "${step.step_3_parallel.output.all_sub_workflows_valid}"
        passed: false
      - name: performance_acceptable
        weight: 0.2
        threshold: 0.85
        actual: "${step.step_3_parallel.output.performance_acceptable}"
        passed: true
    aggregate_method: weighted_sum
    overall_threshold: 0.85
  provenance:
    execution_id: "exec_abc123"
    pipeline_trace: ["step_1", "step_2", "step_3_parallel", "step_4"]
  duration_ms: 4523
  token_usage:
    input: 1250
    output: 480
    total: 1730

┌──────────────────────────────────────────────────────────────┐
│ OUTPUT PIPELINE                                           │
│                                                           │
│  step produces result                                     │
│       │                                                   │
│       ├─→ save_to: aggregated_results                     │
│       │     (stored for ${step.step_4.output} access)     │
│       │                                                   │
│       ├─→ format: json                                    │
│       │                                                   │
│       ├─→ fields: flat or nested list                    │
│       │     (declares expected output shape)               │
│       │                                                   │
│       ├─→ structured_output validation (deterministic)   │
│       │     ├─ schema_ref: JSON schema                   │
│       │     ├─ validate conformance                      │
│       │     └─ reject if invalid                        │
│       │                                                   │
│       ├─→ quality scoring (non-deterministic)            │
│       │     ├─ per-criteria: weight * threshold * actual │
│       │     ├─ aggregate: weighted_sum                    │
│       │     ├─ threshold: >= 0.85                        │
│       │     └─ result: passed/failed                     │
│       │                                                   │
│       ├─→ provenance: execution trace                   │
│       ├─→ duration_ms: wall-clock time                  │
│       └─→ token_usage: if LLM was involved               │
└──────────────────────────────────────────────────────────────┘

attention

attention:
  focus_areas:
    - "error handling patterns in async Rust code"
    - "tokio runtime usage and best practices"
    - "unsafe block usage and safety justifications"
  avoid_areas:
    - "code formatting and style nits"
    - "import ordering"
    - "comment quality"
  style_hint: "concise, technical, with specific line references"
  priority_ranking:
    - safety_correctness
    - performance
    - maintainability
    - readability
  context_boundary: "only analyze files under /workspace/src, ignore test files"
  output_format_preference: "structured with code references"
  domain_knowledge_area: "systems programming"
  complexity_assumption: "intermediate"
  audience_level: "senior engineer"

┌──────────────────────────────────────────────────────────────┐
│ ATTENTION MODEL                                           │
│                                                           │
│  Unlike guardrails (block/allow) and permissions          │
│  (authorize/deny), attention is a SOFT HINT.              │
│                                                           │
│  focus_areas ─────→ "generally pay attention to X"        │
│  avoid_areas  ─────→ "generally ignore X"                  │
│  style_hint   ─────→ "write like this"                    │
│  priority_ranking → "when in doubt, prioritize this order" │
│  context_boundary → "stay within this scope"              │
│                                                           │
│  ┌─────────────────────────────────────────────────┐      │
│  │ KEY BEHAVIOR:                                │      │
│  │                                                 │      │
│  │ • Not enforced. Model may ignore attention.    │      │
│  │ • Gracefully accepts unknown fields.           │      │
│  │   If the model hallucinates an attention       │      │
│  │   key like "food_preferences", it is stored    │      │
│  │   silently, not rejected.                     │      │
│  │ • No blocking, no warning, no logging of       │      │
│  │   non-compliance.                             │      │
│  │ • Separate from prompt — this is metadata       │      │
│  │   about WHAT to focus on, not WHAT to do.      │      │
│  └─────────────────────────────────────────────────┘      │
└──────────────────────────────────────────────────────────────┘

guardrails

guardrails:
  enforcement_policy: block
  input:
    guards:
      - prompt_injection
      - pii_redaction
      - max_length
    prompt_injection:
      sensitivity: high
      on_match: block
      log: true
      trigger_patterns:
        - "ignore previous instructions"
        - "you are now"
        - "system prompt:"
    pii_redaction:
      patterns:
        - email
        - phone
        - ssn
        - api_key
        - custom: "/regex/credit_card_number/"
      replacement: "[REDACTED]"
      on_match: sanitize
      log: true
    max_length:
      max_tokens: 8000
      on_match: truncate
      truncate_to: 7500
  output:
    guards:
      - toxicity_filter
      - pii_redaction
      - format_validation
      - max_length
    toxicity_filter:
      trigger_tokens:
        - "ignore previous instructions"
        - "you are now"
      on_match: block
      log: true
    pii_redaction:
      patterns:
        - email
        - phone
        - api_key
      replacement: "[REDACTED]"
    format_validation:
      format: json
      strict: true
      on_match: retry
      max_retries: 2
    max_length:
      max_tokens: 4096
      on_match: truncate
  tool_use:
    guards:
      - no_web_access
      - no_file_access
      - no_script_access
      - no_terminal_access
      - no_mcp_access
    no_web_access:
      on_match: block
      log: true
    no_file_access:
      on_match: block
      log: true
    no_script_access:
      on_match: block
      log: true
    no_terminal_access:
      on_match: block
      log: true
    no_mcp_access:
      on_match: block
      log: true

┌──────────────────────────────────────────────────────────────┐
│ GUARDRAIL ENFORCEMENT                                     │
│                                                           │
│  enforcement_policy: block │ warn │ allow                   │
│                                                           │
│  INPUT ──→ [prompt_injection] ──→ [pii_redaction] ──→     │
│             [max_length]                                  │
│              │                                            │
│              ↓                                            │
│         block │ warn │ sanitize │ truncate                │
│                                                           │
│  OUTPUT ──→ [toxicity_filter] ──→ [pii_redaction] ──→     │
│             [format_validation] ──→ [max_length]           │
│              │                                            │
│              ↓                                            │
│         block │ warn │ sanitize │ retry │ truncate       │
│                                                           │
│  TOOL_USE ──→ [no_web_access] ──→ [no_file_access] ──→   │
│              [no_script_access] ──→ [no_terminal_access]  │
│              [no_mcp_access]                              │
│              │                                            │
│              ↓                                            │
│         block │ warn                                      │
│                                                           │
│  ════════════════════════════════════════════════════════ │
│  ATTENTION vs GUARDRAILS vs PERMISSIONS                   │
│                                                           │
│  attention    = "focus on X"        (no enforcement)     │
│  guardrails   = "never produce X"    (blocks/warns)      │
│  permissions  = "can't access X"      (prevents action)   │
└──────────────────────────────────────────────────────────────┘

hooks

hooks:
  on_create:
    stop_step:
      condition: "workspace_path == null"
      reason: "No workspace path configured"
    log:
      level: info
      message: "Agent instance created for ${agent.name}"
    set_variable:
      key: total_runs_executed
      value: 0
      scope: workflow
  on_run_start:
    set_variable:
      key: last_run_started_at
      value: "${now_iso8601}"
      scope: workflow
    set_timeout:
      max_ms: 60000
  on_run_complete:
    log:
      level: info
      message: "Run complete: done=${result.done}, turns=${result.turns}, tokens=${result.token_usage.total}, duration=${duration_ms}ms"
    metrics:
      - name: "run.duration_ms"
        value: "${duration_ms}"
        unit: milliseconds
      - name: "run.turns"
        value: "${result.turns}"
        unit: turns
      - name: "run.tokens"
        value: "${result.token_usage.total}"
        unit: tokens
    increment_variable:
      key: total_runs_executed
      delta: 1
      scope: workflow
    conditional_publish:
      topic: "run.success"
      payload:
        task_id: "${task.id}"
        response: "${result.response}"
      condition: "result.done == true"
      topic_else: "run.incomplete"
      payload_else:
        task_id: "${task.id}"
        turns_used: "${result.turns}"
        max_turns: "${result.max_turns}"
    update_memory:
      add_summary: "Run completed with ${result.turns} turns. Final output: ${result.response[0..200]}"
    checkpoint:
      path: "${workflow.output_path}/checkpoints/${task.id}/post_run.ckpt"
      include_memory: true
      include_variables: true
  on_run_error:
    log:
      level: error
      message: "Run failed: ${error.type} - ${error.message} (attempt ${error.attempt})"
    retry:
      condition: "error.is_retryable == true"
      reason: "Retryable error detected"
      backoff: exponential
      level: prompt_restart
      max_attempts: 3
    stop_workflow:
      condition: "error.type == 'content_filtered'"
      reason: "Content filtered, halting workflow"
  on_retry:
    log:
      level: warn
      message: "Retrying run (attempt ${error.attempt}/${retry.max_attempts}): ${error.message}"
  on_turn_start:
    rate_limit:
      max_calls_per_minute: 120
      max_calls_per_run: 50
      on_exceeded: queue
  on_turn_complete:
    log_trace:
      span_name: "agent.turn"
      attributes:
        agent: "${agent.name}"
        task: "${task.id}"
        turn_index: "${turn.index}"
        tool_call_count: "${turn.output.tool_call_count}"
  on_turn_error:
    retry:
      condition: "error.type in ['timeout', 'rate_limited']"
      level: tools_retry
      max_attempts: 2
  on_thinking:
    stream_to_file:
      path: "${workflow.output_path}/logs/thinking_${task.id}.md"
      append: true
      format: markdown
      flush_interval_ms: 100
  on_tool_call:
    permission_check:
      required_permission: "tool.${tool.call.tool_name}"
      on_denied: abort
    validate_args:
      schema_path: "/schemas/tool_args/${tool.call.tool_name}.json"
      on_failure: warn
  on_tool_start:
    log:
      level: debug
      message: "Tool call: ${tool.call.tool_name}(${tool.call.tool_args_json})"
  on_tool_result:
    log:
      level: debug
      message: "Tool result: ${tool.call.tool_name} → success=${tool.result.success}, duration=${duration_ms}ms"
    conditional_publish:
      topic: "tool.completed"
      payload:
        tool_name: "${tool.call.tool_name}"
        success: "${tool.result.success}"
        output_length: "${tool.result.output_length}"
  on_tool_error:
    log:
      level: error
      message: "Tool error: ${tool.call.tool_name} → ${error.message}"
  on_stream_chunk:
    stream_to_console:
      prefix: "▶ "
      color: green
  on_stream_complete:
    save_to_file:
      path: "${workflow.output_path}/runs/${task.id}/stream_log.md"
      content: "${stream_response.full_text}"
      encoding: utf-8
  on_memory_recall:
    log:
      level: debug
      message: "Memory recalled for query context"
  on_memory_store:
    log:
      level: debug
      message: "Memory stored: ${result.response[0..100]}..."

┌──────────────────────────────────────────────────────────────┐
│ HOOK LIFECYCLE                                           │
│                                                           │
│  on_create ───────────── on_run_start                      │
│       │                       │                            │
│       │                  on_turn_start ──── on_tool_call──│
│       │                       │              │   on_tool_start│
│       │                       │          on_thinking   on_tool_result
│       │                       │              │                │
│       │                       │        on_turn_complete    on_tool_error
│       │                       │                    │              │
│       │                 on_turn_error          on_stream_chunk│
│       │                       │                                │
│       │                  on_run_complete ◄── on_stream_complete│
│       │                       │                                │
│       │                  on_run_error                     │
│       │                       │                                │
│       │                  on_retry                         │
│       │                       │                                │
│       │                on_memory_recall                    │
│       │                on_memory_store                      │
│       │                                                  │
│       └── available context in every hook:                │
│           workflow, run, now, now_iso8601, now_epoch_ms,   │
│           random_float, task, error, result, turn,        │
│           tool.call, tool.result, chunk, stream_response,   │
│           duration_ms, validation                          │
│                                                           │
│  ── ACTIONS AVAILABLE IN EACH HOOK ──                     │
│                                                           │
│  CONTROL FLOW:  stop_step, stop_workflow, retry, skip      │
│  LOGGING:      log, log_trace                             │
│  STATE:        set_variable, increment_variable              │
│  PUBLISHING:   publish, conditional_publish, emit_event     │
│  FILE I/O:     save_to_file, append_to_file, stream_to_file│
│  PROMPT:       enrich_prompt, inject_system_message        │
│  VALIDATION:   validate_args, validate_result,              │
│                permission_check, schema_validate            │
│  CONTENT:      redact, truncate, sanitize                  │
│  RATE LIMIT:   rate_limit                                │
│  STREAMING:    stream_to_console                          │
│  EXTERNAL:     callback, http_request, webhook             │
│  CHECKPOINT:   checkpoint, restore                        │
│  NOTIFY:       notify                                   │
│  MEMORY:       update_memory                              │
│  TIMEOUT:      set_timeout                               │
└──────────────────────────────────────────────────────────────┘

tool_permissions

tool_permissions:
  file_operations:
    read:
      enabled: true
      require_confirmation: false
      allowed_paths:
        - /workspace/src
        - /workspace/config
        - /workspace/docs
    write:
      enabled: true
      require_confirmation: true
      allowed_paths:
        - /workspace/output
        - /workspace/checkpoints
        - /workspace/backups
      backup_existing: true
    delete:
      enabled: true
      require_confirmation: true
      allowed_paths:
        - /workspace/temp
        - /workspace/backups
  web_operations:
    fetch:
      enabled: true
      require_confirmation: false
      max_concurrent_requests: 5
    scrape:
      enabled: true
      require_confirmation: false
      respect_robots_txt: true
  shell_operations:
    exec:
      enabled: true
      require_confirmation: true
      timeout_seconds: 30
      allowed_commands:
        - cargo
        - rustc
        - git
        - grep
        - find
  browser_operations:
    navigate:
      enabled: true
      require_confirmation: false
      allowed_domains:
        - docs.rs
        - github.com
        - stackoverflow.com
    interact:
      enabled: true
      require_confirmation: true
      allowed_domains: []

┌──────────────────────────────────────────────────────────────┐
│ PERMISSION HIERARCHY                                      │
│                                                           │
│  step-level permission ──→ OVERRIDES global defaults       │
│  global defaults    ──→ set in tool_permissions section    │
│  per-tool on agent   ──→ can override further              │
│                                                           │
│  ┌─────────────────────────────────────────────────┐      │
│  │ FILE           │ WEB           │ SHELL         │     │
│  │───────────────│───────────────│───────────────│     │
│  │ read          │ fetch         │ exec           │     │
│  │  ✓enabled    │  ✓enabled    │  ✓enabled      │     │
│  │  ✗confirm    │  ✗confirm    │  ✓confirm      │     │
│  │  paths:[..]   │  max:5        │  timeout:30s    │     │
│  │               │               │  commands:[..]  │     │
│  │ write         │ scrape        │                │     │
│  │  ✓enabled    │  ✓enabled    │                │     │
│  │  ✓confirm    │  ✗confirm    │                │     │
│  │  paths:[..]   │  robots:✓     │                │     │
│  │  backup:✓     │               │                │     │
│  │ delete        │               │                │     │
│  │  ✓enabled    │               │                │     │
│  │  ✓confirm    │               │                │     │
│  │  paths:[..]   │               │                │     │
│  └─────────────────────────────────────────────────┘      │
│                                                           │
│  BROWSER                                                 │
│  │ navigate: ✓, domains:[docs.rs, github.com, ...]        │
│  │ interact: ✓, requires confirmation                     │
│                                                           │
│  PER-TOOL OVERRIDE (on agent step):                        │
│  tools:                                                  │
│    - name: dangerous_script                              │
│      permission: always_trust   ← overrides global ask   │
│    - name: file_search                                  │
│      permission: always_trust   ← overrides global ask   │
└──────────────────────────────────────────────────────────────┘

model_config

model_config:
  host: lmstudio
  allowed_processors: "cpu_main | cpu_alt_1 & gpu_cluster2"
  max_allowed:
    ram: 13%
    vram: 3.7gb
    cpu: 49%
    gpu: 74%
    attention_tokens: 150000
    concurrent_requests: 2
  min_allowed:
    ram: 9%
    vram: 24%
    cpu: 49%
    gpu: 74%
    attention_tokens: 73500
  model_memory_cache_size: min
  thinking:
    budget_tokens: 4096
    capture_in_output: true
    capture_in_events: true
    stream_to_log: true
  temperature: 0.7
  top_p: 0.95
  max_output_tokens: 4096
  gpu_layers: 99
  rope_freq_base: 0
  rope_freq_scale: 0
  context_length: 128000
  batch_size: 512

┌──────────────────────────────────────────────────────────────┐
│ MODEL LOADING PIPELINE                                    │
│                                                           │
│  1. RESOLVE HARDWARE                                   │
│     allowed_processors: "cpu_main | cpu_alt_1 & gpu_2"    │
│                                                           │
│     │ = fallback (try cpu_main first, use alt_1 if busy) │
│     & = parallel (load into BOTH if available)            │
│                                                           │
│     Check available bandwidth against model requirements   │
│     │                                                   │
│     ├─ max_allowed ram: 13% → 13% of system RAM         │
│     ├─ min_allowed ram: 9%  → HARD FLOOR                │
│     │   If system can't meet → RUNTIME ERROR with         │
│     │   helpful message + alternative model suggestions   │
│     │                                                   │
│     ├─ max_allowed vram: 3.7gb → cap on GPU memory       │
│     ├─ min_allowed vram: 24% → HARD FLOOR               │
│     │                                                   │
│     └─ attention_tokens: 73500-150000 → context window   │
│         If insufficient for min → RUNTIME ERROR          │
│                                                           │
│  2. CALCULATE KV CACHE SIZE                             │
│     model_memory_cache_size: min                        │
│     Maps to smallest KV cache that still fits            │
│     model usefulness minimums (from global config)        │
│                                                           │
│  3. LOAD MODEL                                         │
│     ┌─────────────────────────────────┐                   │
│     │ cpu_main (main context)       │                   │
│     │  70 layers on CPU             │                   │
│     │  quantization: q4_k_m         │                   │
│     └─────────────────────────────────┘                   │
│     ┌─────────────────────────────────┐                   │
│     │ gpu_cluster2 (offload)         │                   │
│     │  29 layers on GPU              │                   │
│     │  quantization: q4_k_m         │                   │
│     └─────────────────────────────────┘                   │
│                                                           │
│  4. RUNTIME BEHAVIOR                                    │
│     concurrent_requests: 2 → max parallel inferences    │
│     thinking.budget_tokens: 4096 → reasoning budget     │
│     thinking.capture_in_output: true → in response      │
│     thinking.stream_to_log: true → write to log file    │
│     temperature, top_p: sampling controls              │
│     max_output_tokens: generation cap                    │
└──────────────────────────────────────────────────────────────┘

Step Types
cli_exec

cli_exec:
  id: run_cargo_build
  command: "cargo build --release 2>&1"
  args:
    - "cargo"
    - "build"
    - "--release"
  working_directory: "/workspace"
  timeout_seconds: 120
  allowed_commands:
    - cargo
    - rustc
  require_confirmation: false
  capture_output: true
  exit_code_handling: fail
  env:
    RUST_BACKTRACE: "1"
    CARGO_INCREMENTAL: "0"
  input:
    prompt: "Build the project in release mode"
  output:
    save_to: build_result
    fields:
      - success
      - exit_code
      - stdout
      - stderr
  retry:
    max_attempts: 2
    backoff: linear
    delay_ms: 5000
    level: step_restart

┌──────────────────────────────────────────┐
│ CLI_EXEC                               │
│                                          │
│  command ──→ shell execution             │
│       │                                  │
│       ├─ check allowed_commands          │
│       ├─ apply timeout                   │
│       ├─ set working_directory           │
│       ├─ inject env vars                 │
│       │                                  │
│       ↓                                  │
│  stdout + stderr ──→ capture_output      │
│       │                                  │
│       ↓                                  │
│  exit_code ──→ 0 = success              │
│                non-0 = check handling    │
│                  fail │ warn │ ignore  │
└──────────────────────────────────────────┘

file_read

file_read:
  id: read_config
  path: "${workspace_path}/config/settings.json"
  encoding: utf-8
  offset: 0
  limit: 10000
  require_confirmation: false
  input:
    workspace_path: "/workspace"
  output:
    save_to: config_contents
    format: text
    fields:
      - content
      - path
      - size_bytes
  tool_permissions:
    file_operations:
      read:
        allowed_paths:
          - /workspace

┌──────────────────────────────────────────┐
│ FILE_READ                              │
│                                          │
│  path ──→ check allowed_paths            │
│       │                                  │
│       ├─ open file                       │
│       ├─ apply encoding                  │
│       ├─ seek to offset                  │
│       ├─ read up to limit bytes          │
│       │                                  │
│       ↓                                  │
│  content ──→ save_to config_contents     │
└──────────────────────────────────────────┘

file_write

file_write:
  id: write_output
  path: "/workspace/output/refactored_code.rs"
  content: "${step.step_5_success.output.refactored_code}"
  encoding: utf-8
  backup_existing: true
  require_confirmation: true
  append: false
  create_directories: true
  input:
    prompt: "Write refactored code to output"
  output:
    save_to: write_result
    fields:
      - success
      - file_path
      - file_size
      - backup_path
  tool_permissions:
    file_operations:
      write:
        allowed_paths:
          - /workspace/output

┌──────────────────────────────────────────┐
│ FILE_WRITE                             │
│                                          │
│  content ──→ check allowed_paths        │
│       │                                  │
│       ├─ require_confirmation?           │
│       │   └─ yes → prompt user           │
│       ├─ backup_existing?                │
│       │   └─ yes → copy to .bak          │
│       ├─ create_directories?            │
│       │                                  │
│       ↓                                  │
│  write file ──→ save_to write_result     │
└──────────────────────────────────────────┘

file_delete

file_delete:
  id: cleanup_temp
  path: "/workspace/temp/output.json"
  recursive: false
  require_confirmation: true
  input:
    prompt: "Clean up temporary output file"
  output:
    save_to: delete_result
    fields:
      - success
      - deleted_path
  tool_permissions:
    file_operations:
      delete:
        allowed_paths:
          - /workspace/temp

┌──────────────────────────────────────────┐
│ FILE_DELETE                            │
│                                          │
│  path ──→ check allowed_paths            │
│       │                                  │
│       ├─ require_confirmation: always     │
│       │   └─ yes → prompt user           │
│       ├─ recursive? (files only or dir)  │
│       │                                  │
│       ↓                                  │
│  delete ──→ save_to delete_result        │
└──────────────────────────────────────────┘

http_request

http_request:
  id: fetch_github_prs
  url: "https://api.github.com/repos/${repo_owner}/${repo_name}/pulls"
  method: GET
  headers:
    Authorization: "Bearer ${secrets.GITHUB_TOKEN}"
    Accept: "application/vnd.github.v3+json"
    Content-Type: "application/json"
  body: null
  timeout_seconds: 30
  expected_status: 200
  parse_as: json
  max_concurrent_requests: 5
  retry_on_status:
    - 429
    - 503
    - 502
  input:
    repo_owner: "penwoodj"
    repo_name: "agentic-workflow"
  output:
    save_to: pr_list
    fields:
      - status_code
      - response
      - body
  retry:
    max_attempts: 3
    backoff: exponential
    base_ms: 1000
    max_ms: 30000
    jitter: 0.2
  tool_permissions:
    web_operations:
      fetch:
        enabled: true
        max_concurrent_requests: 5

┌──────────────────────────────────────────────────────┐
│ HTTP_REQUEST                                        │
│                                                      │
│  method + url + headers + body                       │
│       │                                              │
│       ├─ apply timeout_seconds                       │
│       ├─ send request                                │
│       │                                              │
│       ↓                                              │
│  response                                           │
│       │                                              │
│       ├─ status_code == expected_status?              │
│       │   └─ yes → parse_as (json|xml|text|binary)   │
│       │                                              │
│       ├─ status_code in retry_on_status?              │
│       │   └─ yes → retry with backoff                  │
│       │                                              │
│       └─ otherwise → check parse_as                   │
│                                                      │
│  PARSED RESPONSE ──→ save_to pr_list                  │
└──────────────────────────────────────────────────────┘

script_run

script_run:
  id: run_analysis
  path: "/workspace/scripts/analyze.py"
  args:
    - "--input"
    - "${step.step_1.output.analysis_target}"
    - "--format"
    - "json"
  runtime: python3
  working_directory: "/workspace"
  timeout_seconds: 60
  env:
    PYTHONPATH: "/workspace/lib"
    LOG_LEVEL: debug
  require_confirmation: false
  input:
    prompt: "Run analysis script on target"
  output:
    save_to: analysis_result
    fields:
      - exit_code
      - stdout
      - stderr
  retry:
    max_attempts: 2
    backoff: linear
    delay_ms: 2000

┌──────────────────────────────────────────┐
│ SCRIPT_RUN                             │
│                                          │
│  path.ext ──→ auto-detect runtime       │
│    .py    → python3                      │
│    .js    → node                        │
│    .sh    → bash                        │
│    .rb    → ruby                        │
│    (or use explicit runtime override)     │
│       │                                  │
│       ├─ apply timeout                   │
│       ├─ set working_directory           │
│       ├─ inject env vars                 │
│       ├─ pass args                       │
│       │                                  │
│       ↓                                  │
│  stdout + stderr ──→ save_to result      │
└──────────────────────────────────────────┘

use_website_ui

use_website_ui:
  id: scrape_docs
  action: scrape
  url: "https://docs.rs/tokio/latest/tokio/"
  selector: "table.docblock-method"
  allowed_domains:
    - docs.rs
    - github.com
  require_confirmation: false
  respect_robots_txt: true
  timeout_seconds: 30
  scrape_format: markdown
  input:
    prompt: "Scrape tokio documentation"
  output:
    save_to: scraped_docs
    fields:
      - content
      - url
      - scrape_format
  tool_permissions:
    browser_operations:
      navigate:
        allowed_domains:
          - docs.rs
      interact:
        require_confirmation: true

┌──────────────────────────────────────────┐
│ USE_WEBSITE_UI                          │
│                                          │
│  actions:                                │
│  navigate ──→ load URL                   │
│  click    ──→ find element, click       │
│  fill     ──→ find input, type value     │
│  select   ──→ find dropdown, pick option│
│  screenshot ──→ capture page as image   │
│  scrape   ──→ extract content as text   │
│  wait     ──→ wait for condition        │
│  evaluate ──→ run JS in page context    │
│                                          │
│  ALL actions check:                      │
│  ├─ allowed_domains whitelist            │
│  ├─ respect_robots_txt                  │
│  ├─ timeout_seconds                     │
│  └─ selector for element targeting      │
└──────────────────────────────────────────┘

db_query

db_query:
  id: get_validation_history
  connection: "${secrets.DATABASE_URL}"
  statement: "SELECT step_id, criteria, score, passed, created_at FROM validation_results WHERE workflow_run = $1 ORDER BY created_at DESC LIMIT $2"
  params:
    - "${workflow.run_id}"
    - 100
  timeout_seconds: 10
  row_limit: 100
  parse_as: json
  input:
    workflow_run: "${workflow.run_id}"
  output:
    save_to: validation_history
    fields:
      - rows
      - row_count
  tool_permissions:
    shell_operations:
      exec:
        timeout_seconds: 10

┌──────────────────────────────────────────┐
│ DB_QUERY                               │
│                                          │
│  connection + statement + params         │
│       │                                  │
│       ├─ parameterized (no injection)     │
│       ├─ apply timeout                   │
│       ├─ cap at row_limit                 │
│       │                                  │
│       ↓                                  │
│  result rows ──→ parse_as (json|csv|raw)│
│       │                                  │
│       ↓                                  │
│  save_to validation_history              │
└──────────────────────────────────────────┘

db_mutation

db_mutation:
  id: save_results
  connection: "${secrets.DATABASE_URL}"
  statement: "INSERT INTO analysis_results (workflow_id, step_id, output, score, created_at) VALUES ($1, $2, $3, $4, NOW())"
  params:
    - "${workflow.id}"
    - "step_4"
    - "${step.step_4.output}"
    - "${step.step_4.output.weighted_overall_score}"
  timeout_seconds: 10
  transaction: true
  require_confirmation: true
  backup_query: "SELECT * FROM analysis_results WHERE workflow_id = $1 AND step_id = $2"
  input:
    workflow_id: "${workflow.id}"
  output:
    save_to: mutation_result
    fields:
      - rows_affected
      - success

┌──────────────────────────────────────────┐
│ DB_MUTATION                            │
│                                          │
│  connection + statement + params         │
│       │                                  │
│       ├─ parameterized (no injection)     │
│       ├─ wrap in transaction?             │
│       ├─ require_confirmation: yes        │
│       ├─ backup_query runs FIRST          │
│       │   (snapshot before mutation)      │
│       ├─ apply timeout                   │
│       │                                  │
│       ↓                                  │
│  rows_affected ──→ save_to result        │
└──────────────────────────────────────────┘

kb_search

kb_search:
  id: find_relevant_docs
  query: "${step.step_2.output.user_query}"
  namespace: "project_knowledge"
  top_k: 10
  score_threshold: 0.7
  filters:
    - project: "${workflow.project_name}"
    - doc_type: [api_reference, tutorial]
  rerank: true
  rerank_model: "BAAI/bge-reranker-large"
  input:
    user_query: "${step.step_2.output.user_query}"
  output:
    save_to: relevant_docs
    fields:
      - results
      - result_count
      - scores

┌──────────────────────────────────────────┐
│ KB_SEARCH                              │
│                                          │
│  query ──→ embed query vector           │
│       │                                  │
│       ├─ search namespace               │
│       ├─ apply metadata filters          │
│       ├─ return top_k results           │
│       ├─ filter by score_threshold       │
│       │                                  │
│       ↓                                  │
│  results ──→ rerank?                    │
│       │    yes → apply rerank model      │
│       │                                  │
│       ↓                                  │
│  scored results ──→ save_to              │
└──────────────────────────────────────────┘

kb_write

kb_write:
  id: ingest_documentation
  documents:
    - path: "/workspace/docs/api_reference.md"
      type: file
    - path: "${step.scrape_step.output.content}"
      type: raw
    - url: "https://example.com/tutorial"
      type: url
  namespace: "project_knowledge"
  embeddings_model: "BAAI/bge-small-en-v1.5"
  chunking_strategy: semantic
  chunk_size: 512
  chunk_overlap: 64
  upsert: true
  input:
    scrape_step: "step_scrape"
  output:
    save_to: ingest_result
    fields:
      - documents_processed
      - chunks_created
      - namespace

┌──────────────────────────────────────────┐
│ KB_WRITE                               │
│                                          │
│  documents (file/path/url/raw)           │
│       │                                  │
│       ├─ chunking_strategy:             │
│       │   none | fixed_size | semantic    │
│       │   sentence                       │
│       ├─ chunk_size + chunk_overlap     │
│       ├─ generate embeddings             │
│       │   (embeddings_model)             │
│       ├─ upsert: update if exists?      │
│       │                                  │
│       ↓                                  │
│  ingest into namespace ──→ save_to       │
└──────────────────────────────────────────┘

branch

branch:
  id: assess_system_state
  rules:
    - branch_id: parallel_execution
      next_step: step_3_parallel
      description: "Execute sub-agents in parallel"
      enabled_by: "initial_assessment.decision == 'parallel'"
    - branch_id: resume_execution
      next_step: step_3_resume
      description: "Resume from checkpoint"
      enabled_by: "initial_assessment.decision == 'resume'"
    - branch_id: defer_execution
      next_step: step_defer
      description: "Defer due to resource constraints"
      enabled_by: "initial_assessment.decision == 'defer'"
    - branch_id: fail_execution
      next_step: step_fail
      description: "Configuration error"
      enabled_by: "initial_assessment.decision == 'fail'"
  default_next: step_fail
  strict_evaluation: true
  input:
    decision_ref: "${step.step_2.output}"
  output:
    save_to: branch_decision
    fields:
      - selected_branch
      - branch_id
      - next_step

┌──────────────────────────────────────────┐
│ BRANCH                                 │
│                                          │
│  rules evaluated in ORDER:                │
│                                          │
│  rule 1: "decision == 'parallel'"        │
│    │                                       │
│    ├─ true  → next_step: step_3_parallel │
│    └─ false → next rule                  │
│                                          │
│  rule 2: "decision == 'resume'"          │
│    │                                       │
│    ├─ true  → next_step: step_3_resume    │
│    └─ false → next rule                  │
│                                          │
│  rule 3: "decision == 'defer'"            │
│    │                                       │
│    ├─ true  → next_step: step_defer       │
│    └─ false → next rule                  │
│                                          │
│  rule 4: "decision == 'fail'"             │
│    │                                       │
│    ├─ true  → next_step: step_fail        │
│    └─ false → default_next                │
│                                          │
│  NO RULE MATCHED:                         │
│    ├─ strict=true → FAIL workflow         │
│    └─ strict=false → default_next         │
└──────────────────────────────────────────┘

parallel

parallel:
  id: execute_sub_agents
  branches:
    - branch_id: code_analysis
      next_step: sub_workflow_code_analysis
    - branch_id: doc_research
      next_step: sub_workflow_doc_research
    - branch_id: refactor_generation
      next_step: sub_workflow_refactor
    - branch_id: validation
      next_step: sub_workflow_validate
  max_parallel: 4
  join_mode: all
  quorum: 3
  result_merge: dict
  cancel_remaining_on_join: true
  error_handling: continue
  input:
    prompt: "Execute all sub-agents in parallel"
  output:
    save_to: parallel_results
    fields:
      - sub_agents_executed
      - branch_results
      - failed_branches

┌──────────────────────────────────────────────────────┐
│ PARALLEL                                           │
│                                                      │
│  branch_1 ──→ sub_workflow_code_analysis              │
│  branch_2 ──→ sub_workflow_doc_research                │
│  branch_3 ──→ sub_workflow_refactor                   │
│  branch_4 ──→ sub_workflow_validate                   │
│                                                      │
│  max_parallel: 4 (hardware constraint)                 │
│  Only 4 run at once; others queue                    │
│                                                      │
│  WAIT FOR JOIN:                                      │
│  join_mode: all ── wait for ALL branches             │
│  join_mode: any ── wait for FIRST branch              │
│  join_mode: quorum(3) ── wait for N branches         │
│                                                      │
│  ON JOIN:                                            │
│  result_merge: dict (combine by branch_id)            │
│  cancel_remaining_on_join: true (kill unfinished)     │
│                                                      │
│  ON BRANCH ERROR:                                   │
│  error_handling: continue (other branches keep going)  │
│  error_handling: fail_all (cancel everything)        │
│  error_handling: fail_branch (skip failed branch)    │
└──────────────────────────────────────────────────────┘

merge

merge:
  id: aggregate_results
  sources:
    - "${step.parallel_results.branch_results.code_analysis}"
    - "${step.parallel_results.branch_results.doc_research}"
    - "${step.parallel_results.branch_results.refactor}"
    - "${step.parallel_results.branch_results.validation}"
  strategy: weighted_score
  weights:
    code_analysis: 0.3
    doc_research: 0.2
    refactor: 0.3
    validation: 0.2
  threshold: 0.85
  on_conflict: last_wins
  input:
    parallel_results: "${step.execute_sub_agents.output}"
  output:
    save_to: merged_results
    fields:
      - aggregate_score
      - per_source_scores
      - passed
      - conflict_count

┌──────────────────────────────────────────┐
│ MERGE                                 │
│                                          │
│  source 1 → score * 0.3               │
│  source 2 → score * 0.2               │
│  source 3 → score * 0.3               │
│  source 4 → score * 0.2               │
│       │                                  │
│       ↓                                  │
│  strategy:                              │
│  concat ─────→ [arr1, arr2, arr3, ...]  │
│  dict ─────→ {k1: v1, k2: v2, ...}   │
│  weighted_score → sum(score*weight)    │
│  first ─────→ take first non-null       │
│  last ──────→ take last non-null        │
│       │                                  │
│       ↓                                  │
│  aggregate >= threshold?                │
│    ├─ yes → passed: true               │
│    └─ no  → passed: false              │
└──────────────────────────────────────────┘

loop

loop:
  id: validation_convergence_loop
  body_step: evaluate_quality
  while: "evaluation.weighted_overall_score < 0.85 && loop.iteration < loop.max_iterations"
  until: "evaluation.weighted_overall_score >= 0.95"
  min_iterations: 2
  max_iterations: 10
  loop_state_ref: validation_loop_state
  on_exhaustion: continue_with_last
  loop_to_step: aggregate_and_evaluate
  break_on:
    - condition: "evaluation.weighted_overall_score >= 0.95"
      reason: "Converged above optimal threshold"
    - condition: "evaluation.code_quality_score >= 0.95 AND evaluation.all_sub_workflows_valid == true"
      reason: "All criteria exceeded"
  max_duration_ms: 300000
  yield_on_each: true
  collect_per_iteration:
    - iteration
    - timestamp
    - weighted_overall_score
    - adjustments_made
  finally:
    run_step: final_evaluation
  input:
    initial_evaluation: "${step.step_4.output}"
  output:
    save_to: convergence_result
    fields:
      - iterations_completed
      - final_score
      - converged
      - best_iteration
      - per_iteration_history
  retry:
    max_attempts: 2
    level: step_restart

┌──────────────────────────────────────────────────────────────┐
│ LOOP                                                         │
│                                                              │
│  START                                                       │
│    │                                                          │
│    ├─ iteration = 0                                              │
│    │                                                          │
│    ├─ min_iterations reached? ──→ no → skip until check       │
│    │   │                                                       │
│    │   yes                                                      │
│    │                                                           │
│    ├─ execute body_step (evaluate_quality)                      │
│    │                                                           │
│    ├─ collect per_iteration results                             │
│    │    {iteration, timestamp, score, adjustments}             │
    │                                                           │
│    ├─ yield_on_each: true (intermediate results available)    │
│    │                                                           │
│    ├─ UNTIL check: score >= 0.95? ──→ yes → BREAK           │
│    │                                                           │
│    ├─ WHILE check: score < 0.85? ──→ no → BREAK             │
│    │                                                           │
│    ├─ break_on conditions:                                     │
│    │    ├─ score >= 0.95 → BREAK (converged)                  │
│    │    └─ code_quality >= 0.95 AND all_valid → BREAK         │
    │                                                           │
│    ├─ iteration >= max_iterations? ──→ yes → on_exhaustion    │
    │    │   continue_with_last: use last result               │
│    │    │   fail: hard stop with error                         │
    │    │   return_accumulated: return all results             │
    │                                                           │
│    ├─ max_duration_ms exceeded? ──→ yes → BREAK              │
│    │                                                           │
│    ├─ iteration++                                              │
│    │                                                          │
│    └─ loop_to_step: aggregate_and_evaluate                   │
│       (re-aggregate after adjustment, then re-check)            │
│                                                              │
│  AFTER LOOP (finally):                                        │
│    └─ run_step: final_evaluation                             │
│                                                              │
│  STATE (loop_state_ref):                                     │
│    iteration, score_history, adjustment_count,                │
│    best_score, best_iteration, convergence_trend              │
└──────────────────────────────────────────────────────────────┘

fallback

fallback:
  id: model_fallback
  rules:
    - trigger: "error.type == 'context_length_exceeded'"
      target: smaller_model_step
      mode: switch_model
      config_override:
        model: "llama-3.2-3b-instruct"
        max_output_tokens: 1024
    - trigger: "error.type == 'rate_limited'"
      target: queue_and_retry_step
      mode: reroute
      delay_ms: 5000
    - trigger: "error.type == 'server_error'"
      target: local_cache_step
      mode: degrade
  default_target: fail_workflow_step
  input:
    prompt: "Try primary model"
  output:
    save_to: fallback_result
    fields:
      - selected_target
      - trigger_reason

┌──────────────────────────────────────────┐
│ FALLBACK                               │
│                                          │
│  try primary target                     │
│       │                                  │
│       ├─ success → done                   │
│       └─ failure                         │
│            │                            │
│            ↓                            │
│  rule 1 trigger?                         │
│    ├─ yes → mode determines behavior:    │
│    │   reroute ──→ try different step   │
│    │   switch_model → use smaller model │
│    │   degrade ──→ use cached/fallback  │
│    └─ no → next rule                  │
│            │                            │
│            ↓                            │
│  rule 2 trigger?                         │
│    ├─ yes → execute                    │
│    └─ no → next rule                  │
│            │                            │
│            ↓                            │
│  ... repeat all rules ...               │
│            │                            │
│            ↓                            │
│  no rules matched: default_target      │
│    └─ fail_workflow_step               │
└──────────────────────────────────────────┘

compete

compete:
  id: fastest_llm_response
  branches:
    - branch_id: gpt4_turbo
      next_step: llm_call_gpt4
    - branch_id: local_fast
      next_step: llm_call_local
    - branch_id: local_creative
      next_step: llm_call_local_creative
  win_condition: first_complete
  cancel_losers: true
  score_expression: "result.quality_score"
  input:
    prompt: "Generate code summary"
  output:
    save_to: compete_result
    fields:
      - winner_branch
      - winner_response
      - all_responses
      - duration_ms_per_branch

┌──────────────────────────────────────────┐
│ COMPETE                               │
│                                          │
│  branch_1 ──→ LLM call (gpt4)        │
│  branch_2 ──→ LLM call (local)       │
│  branch_3 ──→ LLM call (creative)     │
│       │                                  │
│       │   ALL running in parallel        │
│       │                                  │
│       ↓                                  │
│  win_condition determines winner:        │
│                                          │
│  first_complete                         │
│    └→ first to finish wins             │
│       cancel others                     │
│                                          │
│  best_score                              │
│    └→ all complete, pick highest score   │
│       cancel none (need all results)     │
│                                          │
│  lowest_cost                            │
│    └→ all complete, pick cheapest       │
│                                          │
│  cancel_losers: true → kill unfinished   │
└──────────────────────────────────────────┘

sub_workflow

sub_workflow:
  id: run_code_analysis
  workflow_ref: code_analysis_workflow
  input_mapping:
    workspace_path: "${workspace_path}"
    target_directory: "${step.step_1.output.analysis_target}"
    strict_mode: true
  output_capture:
    - source: code_analysis.output.analysis_report
      as: analysis_report
    - source: code_analysis.output.quality_metrics
      as: quality_metrics
    - source: code_analysis.output.token_usage
      as: analysis_token_usage
  wait_for_completion: true
  isolate_state: false
  inherit_context: true
  timeout_secs: 300
  input:
    prompt: "Run code analysis on the src/ directory"
    workspace_path: "/workspace"
  output:
    save_to: code_analysis_results
    fields:
      - analysis_report
      - quality_metrics
      - duration_secs
  retry:
    max_attempts: 2
    level: step_restart

┌──────────────────────────────────────────┐
│ SUB_WORKFLOW                           │
│                                          │
│  parent state                             │
│       │                                  │
│       ├─ input_mapping: map parent vars   │
│       │   to child input params           │
│       │                                  │
│       ├─ isolate_state: false             │
│       │   (child can see parent state)    │
│       ├─ inherit_context: true             │
│       │   (child gets parent's model)     │
│       │                                  │
│       ↓                                  │
│  ┌─────────────────────────────────┐     │
│  │ CHILD WORKFLOW                   │     │
│  │                                 │     │
│  │  ...steps execute...              │     │
│  │                                 │     │
│  └──────────┬──────────────────────┘     │
│             │                               │
│             ↓                               │
│  output_capture: pick child outputs      │
│       │                                  │
│       ↓                                  │
│  wait_for_completion: true (blocks)      │
│  timeout_secs: 300 (max wall clock)      │
│       │                                  │
│       ↓                                  │
│  save_to code_analysis_results           │
└──────────────────────────────────────────┘

sub_workflow_parallel

sub_workflow_parallel:
  id: run_all_sub_agents
  sub_workflows:
    - agent_id: code_analyzer_sub
      workflow_ref: code_analysis_workflow
      validation_config: "${sub_workflows[0].validation_config}"
      interdependent_validations: "${orchestration.relationships[0].interdependent_validations}"
    - agent_id: doc_researcher_sub
      workflow_ref: doc_research_workflow
      validation_config: "${sub_workflows[1].validation_config}"
    - agent_id: refactor_generator_sub
      workflow_ref: refactor_generation_workflow
      validation_config: "${sub_workflows[2].validation_config}"
      interdependent_validations: "${orchestration.relationships[2].interdependent_validations}"
    - agent_id: code_validator_sub
      workflow_ref: code_validation_workflow
      validation_config: "${sub_workflows[3].validation_config}"
    - agent_id: doc_generator_sub
      workflow_ref: doc_generation_workflow
      validation_config: "${sub_workflows[4].validation_config}"
    - agent_id: knowledge_base_sub
      workflow_ref: rag_knowledge_base_workflow
      validation_config: "${sub_workflows[5].validation_config}"
  max_parallel: 4
  interdependent_validations:
    on_validation_result:
      aggregate_at_top_level: true
      propagate_to_dependents: true
    on_interdependent_failure:
      notify_dependent_sub_agents: true
      adjust_validation_criteria: true
      propagate_failure_up: true
    on_checkpoint:
      save_state_snapshot: true
      notify_all_sub_agents: true
  aggregation:
    strategy: weighted_score
    criteria:
      - source: code_quality_score
        weight: 0.3
      - source: documentation_completeness
        weight: 0.25
      - source: all_sub_workflows_valid
        weight: 0.25
      - source: performance_acceptable
        weight: 0.2
  error_handling: continue
  input:
    orchestration_config: "${orchestration}"
  output:
    save_to: parallel_execution_results
    fields:
      - sub_agents_executed
      - validation_results
      - interdependency_status
      - aggregate_score
  timeout_secs: 300

┌──────────────────────────────────────────────────────────────┐
│ SUB_WORKFLOW_PARALLEL                                         │
│                                                              │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐           │
│  │  code    │ │  doc     │ │ refactor │ │ validate │           │
│  │ analyzer│ │ researcher│ │ generator│ │  or     │           │
│  └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘           │
│       │           │           │           │                        │
│       └─────┬─────┘           │           │                        │
│             │           │           │                            │
│             │       ┌───┴───┐       │                            │
│             │       │ interdep│       │                            │
│             │       │ check │       │                            │
│             │       └───┬───┘       │                            │
│             │           │           │                            │
│       max_parallel: 4    │           │                            │
│       (only 4 run at once)│           │                            │
│                          ↓           ↓                            │
│                                                              │
│  EVENT PROPAGATION:                                            │
│  on_validation_result:                                      │
│    → aggregate at top level                                  │
│    → propagate to dependent sub-agents                       │
│    → log result                                              │
│                                                              │
│  on_interdependent_failure:                                 │
│    → notify dependent sub-agents (adjust their criteria)    │
│    → propagate failure up                                     │
│                                                              │
│  on_checkpoint:                                             │
│    → save state snapshot                                     │
│    → notify all sub-agents of checkpoint                   │
│                                                              │
│  AGGREGATE (when all complete):                               │
│    weighted_score across all results                        │
│    on_fail: continue (other branches keep going)             │
└──────────────────────────────────────────────────────────────┘

sub_workflow_whenever

sub_workflow_whenever:
  id: background_metrics
  workflow_ref: metrics_collection_workflow
  input_mapping:
    run_id: "${run.id}"
    workspace_path: "${workspace_path}"
  callback_ref: "on_metrics_collected"
  input:
    prompt: "Start background metrics collection"

┌──────────────────────────────────────────┐
│ SUB_WORKFLOW_WHENEVER                   │
│                                          │
│  parent fires sub_workflow_whenever      │
│       │                                  │
│       ├─ input_mapping: send data        │
│       │                                  │
│       ↓                                  │
│  child workflow starts IMMEDIATELY        │
│  parent does NOT block                   │
│       │                                  │
│       ↓                                  │
│  parent continues to next step           │
│       │                                  │
│       ↓                                  │
│  ...later...                             │
│       │                                  │
│       ↓                                  │
│  child completes → callback_ref fires     │
│       (optional, for notification)         │
└──────────────────────────────────────────┘

checkpoint_save

checkpoint_save:
  id: save_pre_operation
  path: "/workspace/checkpoints/before_refactor_${run.number}.ckpt"
  include_memory: true
  include_variables: true
  label: "pre-refactor checkpoint"
  tags:
    - pre-operation
    - run_${run.number}
    - refactoring
  input:
    prompt: "Save state before refactoring"
  output:
    save_to: checkpoint_result
    fields:
      - checkpoint_path
      - label
      - saved_at

┌──────────────────────────────────────────┐
│ CHECKPOINT_SAVE                        │
│                                          │
│  current state                           │
│       ├─ workflow variables              │
│       ├─ agent memory                   │
│       ├─ step outputs                  │
│       └─ execution position            │
│       │                                  │
│       ↓                                  │
│  serialize to path ──→ checkpoint file  │
│  include_memory: yes                    │
│  include_variables: yes                  │
│  label + tags for discovery             │
└──────────────────────────────────────────┘

checkpoint_restore

checkpoint_restore:
  id: resume_from_checkpoint
  path: "/workspace/checkpoints/before_refactor_3.ckpt"
  validate_integrity: true
  restore_dependencies: true
  overwrite_memory: false
  overwrite_variables: true
  input:
    checkpoint_ref: "${step.step_2.output.resume_checkpoint}"
  output:
    save_to: restore_result
    fields:
      - state_restored
      - restored_from
      - remaining_steps

┌──────────────────────────────────────────┐
│ CHECKPOINT_RESTORE                      │
│                                          │
│  checkpoint file                          │
│       │                                  │
│       ├─ validate_integrity: check       │
│       │   file not corrupted?             │
│       ├─ restore_dependencies: reload      │
│       │   dependent sub-workflow states?   │
│       ├─ overwrite_memory: false          │
│       │   (merge with current memory)     │
│       ├─ overwrite_variables: true       │
│       │   (replace current variables)     │
│       │                                  │
│       ↓                                  │
│  state restored ──→ continue workflow    │
└──────────────────────────────────────────┘

send_event

send_event:
  id: notify_completion
  target: orchestration
  name: sub_workflow_completed
  payload_ref: "${step.run_code_analysis.output}"
  start_if_missing: true
  input:
    prompt: "Notify orchestration of completion"
  output:
    save_to: send_result
    fields:
      - event_name
      - target
      - sent

┌──────────────────────────────────────────┐
│ SEND_EVENT                             │
│                                          │
│  target ──→ workflow/step reference      │
│  name ────→ event identifier            │
│  payload ─→ data to send               │
│                                          │
│  start_if_missing: true                  │
│    └─ target doesn't exist yet?          │
│       └─ start it before sending        │
└──────────────────────────────────────────┘

wait_for_event

wait_for_event:
  id: await_approval
  name: approval_decision
  timeout: 3600
  on_timeout: escalate
  input:
    prompt: "Wait for approval decision"
  output:
    save_to: event_result
    fields:
      - event_name
      - payload
      - received_at

┌──────────────────────────────────────────┐
│ WAIT_FOR_EVENT                        │
│                                          │
│  BLOCK until event "approval_decision"   │
│  arrives from any source                 │
│                                          │
│  timeout: 3600s                         │
│    ├─ on_timeout: fail                  │
│    ├─ on_timeout: continue_with_default  │
│    └─ on_timeout: escalate             │
│                                          │
│  when received:                           │
│       ├─ capture payload                 │
│       └─ continue to next step          │
└──────────────────────────────────────────┘

validate

validate:
  id: validate_output_schema
  source_ref: "${step.agentic_attempts.output}"
  schema_ref: "/schemas/analysis_output.json"
  rules:
    - field: "weighted_overall_score"
      type: number
      min: 0.0
      max: 1.0
      required: true
    - field: "final_decision"
      type: string
      values:
        - success
        - fail
        - needs_review
      required: true
    - field: "criteria_breakdown"
      type: object
      required: true
      children_required: false
  on_failure: warn
  max_retries: 1
  input:
    prompt: "Validate analysis output against schema"
  output:
    save_to: validation_result
    fields:
      - passed
      - errors
      - warnings

┌──────────────────────────────────────────┐
│ VALIDATE                              │
│                                          │
│  source_ref ──→ data to validate        │
│  schema_ref  ──→ JSON Schema            │
│  rules       ──→ individual checks       │
│       │                                  │
│       ├─ type check (number, string...)  │
│       ├─ range check (min, max)         │
│       ├─ required check                 │
│       ├─ enum check (values list)       │
│       ├─ nested object children        │
│       │                                  │
│       ↓                                  │
│  ALL pass?                               │
│    ├─ yes → passed: true                │
│    └─ no  → on_failure:               │
│         warn  → log errors, continue     │
│         fail  → halt pipeline            │
│         retry → re-validate             │
│                                          │
│  max_retries: 1 (one chance to fix)    │
└──────────────────────────────────────────┘

log

log:
  id: log_pipeline_progress
  level: info
  message: "Step ${step.step_name} completed in ${duration_ms}ms. Decision: ${step.branch_analysis.output.decision}. Turns: ${result.turns}. Tokens: ${result.token_usage.total}."
  fields:
    step_name: "${step.step_name}"
    step_id: "${step.step_id}"
    workflow_id: "${workflow.id}"
    run_number: "${run.number}"
    decision: "${step.branch_analysis.output.decision}"
    turns: "${result.turns}"
    token_usage: "${result.token_usage.total}"
    quality_score: "${result.quality_score}"
    duration_ms: "${duration_ms}"
  output_type: both
  format: json
  metrics:
    - name: "pipeline.step.duration_ms"
      value: "${duration_ms}"
      unit: milliseconds
    - name: "pipeline.step.turns"
      value: "${result.turns}"
      unit: turns
  input:
    prompt: "Log pipeline progress"
  output:
    save_to: log_entry

┌──────────────────────────────────────────┐
│ LOG                                   │
│                                          │
│  level: info │ warn │ error │ debug │ fatal│
│                                          │
│  message + interpolated variables           │
│  fields: structured key-value pairs       │
│                                          │
│  output_type:                            │
│    log   → engine log                    │
│    file  → write to file                 │
│    both  → both                         │
│                                          │
│  format: json │ text                     │
│                                          │
│  inline metrics:                         │
│    name + value + unit (for prometheus     │
│    or statsd export)                    │
└──────────────────────────────────────────┘

llm_prompt_response

llm_prompt_response:
  id: generate_plan
  model: "${models.primary_analyzer}"
  model_overrides:
    temperature: 0.8
    max_output_tokens: 8192
  context:
    system_instructions: |
      You are a senior Rust codebase analyzer. Always provide
      specific file paths and line references in your output.
      Structure your analysis as valid JSON.
    injected_context:
      - ref: "${step.code_analysis.output.summary}"
        as: analysis_summary
      - ref: "${step.doc_research.output.relevant_docs}"
        as: documentation_context
    few_shot_examples:
      - input: "Analyze the error handling in main.rs"
        output: '{"error_handling": "uses Result<T, E> throughout", "assessment": "good"}'
      - input: "Check async patterns"
        output: '{"async_patterns": ["tokio::spawn", "async move"], "assessment": "needs improvement"}'
    rag_context:
      ref: "${step.kb_search_step.output.results}"
    conversation_history_ref: "${step.previous_turns.output.conversation_history}"
    max_context_tokens: 120000
    truncation_strategy: smart
  prompt: |
    Analyze the codebase at ${workspace_path} for:
    1. Error handling patterns (Result<T,E> usage)
    2. Async patterns (tokio::spawn usage)
    3. Code quality metrics
    System analysis results: ${step.step_1.output}
  temperature: 0.7
  top_p: 0.95
  max_output_tokens: 4096
  structured_output: true
  schema_ref: "/schemas/code_analysis_output.json"
  thinking:
    budget_tokens: 4096
    capture_in_output: true
    capture_in_events: true
    stream_to_log: true
  guardrails:
    enforcement_policy: block
    input:
      guards:
        - prompt_injection
        - max_length
      max_length:
        max_tokens: 8000
        on_match: truncate
    output:
      guards:
        - format_validation
      format_validation:
        format: json
        strict: true
        on_match: retry
  hooks:
    on_run_complete:
      log:
        level: info
        message: "Plan generation complete: ${duration_ms}ms, ${result.token_usage.total} tokens"
    on_run_error:
      retry:
        condition: "error.type in ['timeout', 'rate_limited']"
        level: prompt_restart
        max_attempts: 3
  input:
    workspace_path: "/workspace"
  output:
    save_to: plan_output
    fields:
      - response
      - reasoning_content
      - token_usage
      - quality_score

┌──────────────────────────────────────────────────────────────┐
│ LLM_PROMPT_RESPONSE                                                │
│                                                                      │
│  ┌──────── CONTEXT ASSEMBLY ──────────────────────────────────┐   │
│  │                                                              │   │
│  │  system_instructions  (prepended, not in user context)        │   │
│  │  "You are a senior Rust analyzer..."                            │   │
│  │                                                              │   │
│  │  + injected_context:                                          │   │
│  │    analysis_summary ── from step output (by ref)             │   │
│  │    documentation_context ── from step output (by ref)       │   │
│  │                                                              │   │
│  │  + rag_context:                                               │   │
│ │    kb_search results ─── from kb_search step (by ref)       │   │
│ │                                                              │   │
│  │  + few_shot_examples:                                        │   │
│ │    input/output pairs for in-context learning                  │   │
│ │                                                              │   │
│  │  + conversation_history:                                      │   │
│ │    prior turns ─── for multi-turn context                   │   │
│ │                                                              │   │
│  │  + prompt (the actual user message)                        │   │
│ │    with ${} interpolated variables                             │   │
│ │                                                              │   │
│  │  ── max_context_tokens: 120000                            │   │
│ │  ── truncation_strategy: smart (drop least useful parts)     │   │
│  │                                                              │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                      │
│                              ↓                                   │
│                                                                      │
│  ┌──────── INFERENCE ───────────────────────────────────────────┐   │
│  │                                                              │   │
│  │  model: llama-3.2-3b-instruct                                 │   │
│  │  model_overrides: temperature, max_output_tokens              │   │
│  │  temperature: 0.7                                           │   │
│  │  top_p: 0.95                                                 │   │
│  │  max_output_tokens: 4096                                    │   │
│  │  structured_output: true → force JSON schema                 │   │
│  │  schema_ref: /schemas/code_analysis_output.json               │   │
│  │                                                              │   │
│  │  THINKING (optional):                                       │   │
│ │  budget_tokens: 4096                                         │   │
│  │  capture_in_output: true → reasoning in response              │   │
│  │  stream_to_log: true → write reasoning to file              │   │
│ │                                                              │   │
│  │  GUARDRAILS:                                                  │   │
│  │  input: prompt_injection, max_length                        │   │
│  │  output: format_validation (json strict)                      │   │
│ │  enforcement: block                                           │   │
│  │                                                              │   │
│  │  HOOKS:                                                      │   │
│  │  on_run_complete → log metrics                                │   │
│  │  on_run_error → retry on timeout/rate_limit                   │   │
│  │                                                              │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                      │
│                              ↓                                   │
│                                                                      │
│  RESPONSE                                                             │
│    ├─ response (the generated text)                               │
│    ├─ reasoning_content (if thinking enabled)                    │
│    ├─ token_usage (input, output, total)                          │
│    └─ quality_score (if validation ran)                          │
│                                                                      │
│  → save_to plan_output                                            │
└──────────────────────────────────────────────────────────────┘

agentic_attempts

agentic_attempts:
  id: refactor_agent
  model: "${models.primary_analyzer}"
  custom_executor_name: react_agent
  prompt: |
    Refactor the codebase based on the analysis results:
    ${step.code_analysis.output}
    Ensure all tests pass and documentation is updated.
  max_turns: 10
  tool_use:
    stop_on_tool_failure: false
    accumulate_tool_results: true
    permission: always_ask
    output:
      format: structured
      summary_length: 200
  tools:
    - name: file_search
      type: tool
      tool_type: grep
    - name: read_file
      type: tool
      tool_type: file_read
    - name: edit_file
      type: tool
      tool_type: file_write
    - name: run_cargo
      type: default_terminal
      run: |
        cargo test --workspace /workspace 2>&1
    - name: run_clippy
      type: default_terminal
      run: |
        cargo clippy --workspace /workspace 2>&1
  limited_tools:
    - file_search
    - read_file
  additional_tools:
    - run_cargo
    - run_clippy
  guardrails:
    enforcement_policy: block
    tool_use:
      guards:
        - no_web_access
        - no_mcp_access
    hooks:
    on_run_complete:
      log:
        level: info
        message: "Refactor complete: ${result.done}, turns=${result.turns}, tools=${result.tool_call_count}"
    on_tool_result:
      log:
        level: debug
        message: "Tool ${tool.call.tool_name}: ${tool.result.output[0..200]}"
    on_turn_error:
      retry:
        condition: "error.type in ['timeout', 'rate_limited', 'server_error']"
        level: tools_retry
  thinking:
    budget_tokens: 2048
    capture_in_output: false
  input:
    analysis_output: "${step.code_analysis.output}"
  output:
    save_to: refactor_result
    fields:
      - done
      - response
      - turns
      - tool_call_count
      - token_usage
      - quality_score
  tool_permissions:
    file_operations:
      read:
        allowed_paths:
          - /workspace/src
          - /workspace/config
      write:
        allowed_paths:
          - /workspace/src
        backup_existing: true
        require_confirmation: true
    shell_operations:
      exec:
        allowed_commands:
          - cargo
          - git
        timeout_seconds: 120
  retry:
    max_attempts: 3
    backoff: exponential
    level: prompt_restart
    adjustment_strategy: reduce_complexity
    tolerance_adjustment: 0.1

┌──────────────────────────────────────────────────────────────┐
│ AGENTIC_ATTEMPTS                                                  │
│                                                                      │
│  ┌───────── AGENT LOOP ────────────────────────────────────┐    │
│  │                                                               │    │
│  │  prompt: "Refactor the codebase..."                        │    │
│  │  model: llama-3.2-3b-instruct                         │    │
│  │  max_turns: 10                                           │    │
│  │  custom_executor: react_agent                               │    │
│  │                                                               │    │
│  │  ┌──── TURN 1 ──────────────────────────────────────┐    │    │
│  │  │  REASON: "Need to check existing structure"    │    │    │
│  │  │  THINK → decide tool                         │    │    │
│  │  │  TOOL: grep → search for patterns        │    │    │
│  │  │  OBSERVE: "Found 12 uses of X"             │    │    │
│  │  │  REASON: "X usage looks problematic"     │    │    │
│  │  │  TOOL: read_file → read main.rs            │    │    │
│  │  │  OBSERVE: "Line 45 needs fix"             │    │    │
│  │  │  TOOL: edit_file → modify code            │    │    │
│  │  │  OBSERVE: "File modified successfully"    │    │    │
│  │  │  REASON: "Check if tests still pass"     │    │    │
│  │  │  TOOL: run_cargo → cargo test            │    │    │
│  │  │  OBSERVE: "All tests pass"                 │    │    │
│  │  │  TOOL: run_clippy → cargo clippy         │    │    │
│  │  │  OBSERVE: "0 warnings"                     │    │    │
│  │  └────────────────────────────────────────────┘    │    │
│  │                                                               │    │
│  │  ┌──── TURN 2 ──────────────────────────────────────┐    │    │
│  │  │  accumulate_tool_results: true                    │    │    │
│  │  │  (has history of turn 1 tool results)          │    │    │
│  │  │  REASON: "Based on turn 1 results..."    │    │    │
│  │  │  ...continue...                                   │    │    │
│  │  └────────────────────────────────────────────┘    │    │
│  │                                                               │    │
│  │  ... turns 3-10 ...                                         │    │    │
│  │                                                               │    │
│  │  TERMINATION CONDITIONS:                                     │    │
│  │  ├─ done=true (model decides task complete)        │    │
│  │  ├─ max_turns reached (10)                           │    │
│  │  ├─ tool_use: stop_on_tool_failure + failure    │    │
│  │  └─ hooks: retry on timeout/rate_limit           │    │
│  │                                                               │    │
│  └───────────────────────────────────────────────────────┘    │
│                                                                      │
│  PERMISSION MODEL:                                                │
│  │                                                             │
│  │  always_ask    → every tool requires confirmation           │    │
│  │  always_trust  → run without asking (dangerous)          │    │
│  │  trust_confirm→ first time ask, then auto-approve    │    │
│  │                 subsequent calls with ANY args         │    │
│  │                                                             │
│  │  limited_tools: [file_search, read_file]                   │    │
│  │  → ONLY these tools available (replaces default list)    │    │
│  │                                                             │
│  │  additional_tools: [run_cargo, run_clippy]              │    │
│  │  → ADD these to the agent's existing tools               │    │
│  │                                                             │
│  │  accumulate_tool_results: true                             │    │
│  │  → full history available in context each turn             │    │
│  │                                                             │
│  └───────────────────────────────────────────────────────────┘    │
│                                                                      │
│  META SCHEMAS ON THIS STEP:                                       │
│    guardrails (input/output/tool_use) + hooks (14 hooks)       │
│    + tool_permissions + model_config (via model)              │
│    + thinking + retry + attention                            │
│    + input + output                                           │
└──────────────────────────────────────────────────────────────┘

user_input

user_input:
  id: get_project_description
  prompt: "Describe the project you want refactored. Include the main purpose, key files, and any specific concerns."
  ui_hint: text
  default_value: null
  allow_custom: true
  required: true
  timeout: 3600
  on_timeout: use_default
  tab_groups:
    - name: Project Details
      panels:
        - label: Description
          input: project_description
          ui_hint: text
        - label: Priority
          input: priority
          ui_hint: multi_choice
          options:
            - high
            - medium
            - low
        - label: Deadline
          input: deadline
          ui_hint: text
    - name: Constraints
      panels:
        - label: Budget Constraints
          input: budget_notes
          ui_hint: text
        - label: Performance Requirements
          input: perf_requirements
          ui_hint: multi_select
          options:
            - latency_critical
            - throughput_critical
            - memory_constrained
            - none
          allow_custom: true
  dynamic_population_ref: "${step.initial_analysis.output.suggested_options"
  input:
    prompt: "Get project description from user"
  output:
    save_to: project_description
    fields:
      - project_description
      - priority
      - deadline
      - budget_notes
      - perf_requirements
      - user_response_timestamp

┌──────────────────────────────────────────────────────────────┐
│ USER_INPUT                                                         │
│                                                                      │
│  ┌────────── UI RENDERING ────────────────────────────────────┐  │
│  │                                                           │  │
│  │  Tab Navigation:                                              │  │
│  │  ┌────────────────┬─────────────────┐                        │  │
│  │  │ Project Details │  Constraints  │                        │  │
│  │  └────────────────┴─────────────────┘                        │  │
│  │                                                           │  │
│  │  Tab: Project Details                                       │  │
│  │  ┌───────────────────────────────────────────────────┐     │  │
│  │  │ Description                                         │     │  │
│  │  │ [text area with Shift+Enter for newlines]          │     │  │
│  │  │ Enter to confirm, Esc to cancel                        │     │  │
│  │  │                                                   │     │  │
│  │  ├─ Priority                                          │     │  │
│  │  │  [high] [medium] [low]                             │     │  │
│  │  │  ← single selection                               │     │  │
│  │  │                                                   │     │  │
│  │  ├─ Deadline                                         │     │  │
│  │  │  [text input field]                                │     │  │
│  │  │                                                   │     │  │
│  │  └───────────────────────────────────────────────────┘     │  │
│  │                                                           │  │
│  │  Tab: Constraints                                         │  │
│  │  ┌───────────────────────────────────────────────────┐     │  │
│  │  │ Budget Constraints                                 │     │  │
│  │  │  [text area]                                       │     │  │
│  │  │                                                   │     │  │
│  │  ├─ Performance Requirements                         │     │  │
│  │  │  [✓ latency_critical]                               │     │  │
│  │  │  [✓ throughput_critical]                              │     │  │
│  │  │  [✓ memory_constrained]                              │     │  │
│  │  │  [✗ none]                                          │     │  │
│  │  │  [type custom option...        ]                     │     │  │
│  │  └───────────────────────────────────────────────────┘     │  │
│  │                                                           │  │
│  │  [Confirm ✓]  [Cancel ✗]                                │  │
│  └─────────────────────────────────────────────────────────┘  │
│                                                                      │
│  Features:                                                           │
│    dynamic_population_ref: options filled from workflow var     │
│    timeout + on_timeout: use_default | fail | defer             │
│    allow_custom: user can type option not in list              │
│    tab_groups: organize inputs across multiple tabs               │
└──────────────────────────────────────────────────────────────┘

user_approval

user_approval:
  id: approve_deployment
  prompt: "Approve deployment of refactored code to /workspace/output/"
  context_ref: "${step.refactor_result.output}"
  require_reason: false
  timeout: 86400
  on_timeout: reject
  on_reject: branch_to
  reject_target: step_manual_review
  input:
    refactor_result: "${step.refactor_result.output}"
  output:
    save_to: approval_decision
    fields:
      - decision
      - reason
      - timestamp

┌──────────────────────────────────────────┐
│ USER_APPROVAL                        │
│                                          │
│  prompt: "Approve deployment?"           │
│  context_ref: what's being approved     │
│                                          │
│  ┌────────────────────────────────────┐  │
│  │  [ ✓ Approve ]  [ ✗ Reject ]      │  │
│  │                                    │  │
│  │  require_reason: false             │  │
│  │  (just click approve)                │  │
│  │                                    │  │
│  │  require_reason: true              │  │
│  │  [ ✓ Approve ]  [ ✗ Reject ]      │  │
│  │  "Reason: ____________"            │  │
│  │  [text input + Enter to submit]     │  │
│  └────────────────────────────────────┘  │
│                                          │
│  on_timeout: reject │ approve │ defer    │
│  on_reject: fail │ branch_to │ defer     │
│                                          │
│  Binary outcome: approved or rejected     │
└──────────────────────────────────────────┘

user_review

user_review:
  id: review_refactoring
  prompt: "Review the refactoring changes for code quality, correctness, and completeness."
  subject_ref: "${step.refactor_result.output.diff}"
  rubric_ref: "/rubrics/code_review.yml"
  scoring: true
  min_score: 0.8
  allow_edits: true
  allow_comments: true
  timeout: 172800
  on_timeout: proceed_without
  input:
    refactoring_output: "${step.refactor_result.output}"
  output:
    save_to: review_result
    fields:
      - decision
      - scores
      - comments
      - edits_made
      - reviewer

┌──────────────────────────────────────────────────────────────┐
│ USER_REVIEW                                                       │
│                                                                      │
│  prompt: "Review refactoring changes"                           │
│  subject_ref: the diff/changes to review                        │
│  rubric_ref: scoring criteria                                      │
│                                                                      │
│  ┌────────────── REVIEW UI ───────────────────────────────┐    │
│  │                                                            │    │
│  │  Subject (diff view):                                     │    │
│  │  ┌─────────────────────────────────────────────┐          │    │
│  │  │ - src/main.rs:15  │  + async fn process() │          │    │
│  │  │ + async fn process() {                     │          │    │
│  │  │ +     let result = do_work().await;         │          │    │
│  │  │ + }                                     │          │    │
│  │  │ + src/main.rs:42  │  - if let Err(e) {       │          │    │
│  │  │ -     return Err(e);                     │          │    │
│  │  └─────────────────────────────────────────────┘          │    │
│  │                                                            │    │
│  │  Scoring (if scoring: true):                               │    │
│  │  ┌──────────────────────────────────────────────┐          │    │
│  │  │ correctness:  [──── slider 0.9 ─────]   0.9/1.0  │          │    │
│  │  │ readability:  [──── slider 0.8 ────]  0.8/1.0  │          │    │
│  │  │ performance:  [──── slider 0.7 ───]  0.7/1.0  │          │    │
│  │  │ style:         [──── slider 0.9 ─────]  0.9/1.0  │          │    │
│  │  └──────────────────────────────────────────────┘          │    │
│  │                                                            │    │
│  │  min_score: 0.8 (aggregate threshold to pass)          │    │
│  │                                                            │    │
│  │  Comments (if allow_comments: true):                         │    │
│  │  ┌──────────────────────────────────────────────┐          │    │
│  │  │ [text area for reviewer notes]                  │          │    │
│  │  └──────────────────────────────────────────────┘          │    │
│  │                                                            │    │
│  │  allow_edits: true (reviewer can edit diff)           │    │
│  │  ┌──────────────────────────────────────────────┐          │    │
│  │  │ [inline edit capabilities in diff view]        │          │    │
│  │  └──────────────────────────────────────────────┘          │    │
│  │                                                            │    │
│  │  [ ✓ Approve ]  [ ✗ Request Changes ]  [ ↩ Full ] │    │
│  │                                                            │
│  └─────────────────────────────────────────────────────┘    │
│                                                                      │
│  timeout: 48 hours                                            │
│  on_timeout: proceed_without | fail | defer                       │
│                                                                      │
│  OUTPUT: decision, per-criteria scores, comments, edits     │
└──────────────────────────────────────────────────────────────┘

wait

wait:
  id: cooldown_between_requests
  duration: PT5S
  jitter: 0.5
  input:
    prompt: "Wait between API calls"
  output:
    save_to: wait_result
    fields:
      - waited_duration_ms

┌──────────────────────────────────────────┐
│ WAIT                                  │
│                                          │
│  duration: PT5S (5 seconds)              │
│  jitter: 0.5 (random 0-2.5s added)   │
│                                          │
│  Used for: rate limiting, cooldowns,       │
│  preventing tight loops                   │
└──────────────────────────────────────────┘

wait_until

wait_until:
  id: wait_for_resources
  duration: PT1M
  condition: "system.available_ram_pct > 30 AND system.available_vram_pct > 50"
  timestamp: "2026-04-01T00:00:00Z"
  recheck_interval: 10000
  timeout: PT1H
  on_timeout: escalate
  input:
    prompt: "Wait for sufficient resources"
  output:
    save_to: wait_result
    fields:
      - waited_duration_ms
      - triggered_by
      - trigger_value

┌──────────────────────────────────────────────────────────────┐
│ WAIT_UNTIL                                                        │
│                                                                      │
│  Three mutually exclusive trigger modes (first match wins):     │
│                                                                      │
│  DURATION:                                                        │
│    "Wait for 1 minute" → simple time-based wait                    │
│    jitter applies                                                   │
│                                                                      │
│  CONDITION:                                                      │
│    "Wait until RAM > 30% AND VRAM > 50%"                       │
│    recheck every 10 seconds                                     │
│    → polls a boolean expression                                 │
│    → useful for waiting for resource availability                │
│    → useful for waiting for external state change                │
│                                                                      │
│  TIMESTAMP:                                                      │
│    "Wait until 2026-04-01T00:00:00Z"                           │
│    → waits for absolute point in time                            │
│    → useful for scheduled execution                           │
│                                                                      │
│  ALWAYS:                                                           │
│    timeout: absolute maximum (overrides all modes)               │
│    on_timeout: fail | continue | escalate                         │
│                                                                      │
│  ── WAITING ──→ check trigger ──→ met? ──→ continue       │
└──────────────────────────────────────────────────────────────┘

defer

defer:
  id: defer_insufficient_resources
  reason: "Insufficient memory to run workflow. Need at least 16GB free RAM."
  save_path: "/workspace/checkpoints/deferred_${run.number}.json"
  suggested_reschedule: "2026-04-01T09:00:00Z"
  resource_requirements:
    estimated_ram_gb: 16
    estimated_vram_gb: 4
    estimated_duration_secs: 300
    required_attention_tokens: 73500
    suggested_alternatives:
      - model: "llama-3.2-1b-instruct"
        reason: "Smaller model, requires less memory"
        workflow_adjustments:
          - max_turns: 5
          - context_budget: 50%
          - skip_rag: true
  input:
    system_state: "${step.step_2.output}"
  output:
    save_to: defer_result
    fields:
      - defer_reason
      - saved_path
      - suggested_reschedule
      - resource_requirements

┌──────────────────────────────────────────────────────────────┐
│ DEFER                                                             │
│                                                                      │
│  "Not enough resources right now, try again later"                  │
│                                                                      │
│  SAVE STATE:                                                    │
│    checkpoint → /workspace/checkpoints/deferred.json               │
│    includes: current step, workflow position, all variables       │
│                                                                      │
│  RESCHEDULE:                                                     │
│    suggested_reschedule: 2026-04-01T09:00:00Z                       │
│    (can be consumed by scheduler_controller)                       │
│                                                                      │
│  RESOURCE REQUIREMENTS:                                          │
│    estimated_ram_gb, estimated_vram_gb                             │
│    estimated_duration_secs, required_attention_tokens                  │
│                                                                      │
│  SUGGESTED ALTERNATIVES:                                       │
│    ┌──────────────────────────────────────────────────┐           │
│    │ model: llama-3.2-1b-instruct (smaller)   │           │
│    │   └─ workflow adjustments:                     │           │
│    │      max_turns: 5                              │           │
│    │      context_budget: 50%                       │           │
│    │      skip_rag: true                             │           │
│    └──────────────────────────────────────────────────┘           │
│                                                                      │
│  → workflow paused, can be resumed manually or by schedule      │
└──────────────────────────────────────────────────────────────┘
