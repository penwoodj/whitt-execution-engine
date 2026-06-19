<!--
Source Session: ses_17a245a9cffeWo9uVFAyuF6C9I
Message Length: 102015 characters
YAML Sections: 140
Embedded Prompts: 36
Agentic Keywords: 14
Complexity: HIGH
Source Files: attempt-8-name-preserve.yml, benchmark-15-models.yml, benchmark-3-models.yml
-->

workflow_id: meta_workflow_generator_v8
name: "Meta-Workflow Generator v8 - Name-Preserving Validate"
description: "3-step meta-workflow with validate step that NEVER renames step references. Uses new task: security audit pipeline."
version: "8.0.0"
author: "Whitt Execution Engine"
tags: [meta-workflow, yaml-generation, attempt-8]
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
    hosting:
      gpu_layers: 999
models:
  "coder-model":
    name: "Qwen2.5-Coder-3B-Instruct-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "reviewer-model":
    name: "Qwen3-4B-Instruct-2507-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
workflow_execution_strategy:
  memory:
    model_lifecycle:
      unload_unused: true
agentic_workflow:
  steps:
    # ── PHASE 1: GENERATE ──────────────────────────────────────────
    step_1_generate:
      generative_entity: "${models.coder-model}"
      prompt: |
        Generate a COMPLETE YAML agentic workflow file for the following task:
        TASK: Create a security audit pipeline that scans source code repositories for vulnerabilities, checks dependency manifests against known CVE databases, analyzes code patterns for injection attacks (SQL, XSS, command injection), reviews authentication and authorization implementations, checks cryptographic usage for weak algorithms, and produces a comprehensive security audit report with risk scores, vulnerability classifications (OWASP Top 10 mapping), remediation priorities, and compliance status indicators.
        Follow this EXAMPLE exactly. Copy its structure, notation, and style:
        ---
        workflow_id: code_review_pipeline
        name: "Code Review Pipeline"
        description: "Automated code review with static analysis, security scan, and final report"
        version: "2.0.0"
        author: "Whitt Execution Engine"
        tags: [code-review, security, analysis]
        schema_version: "2.0.0"
        providers:
          llama_cpp_with_vulkan:
            config:
              host: localhost
              port: 8080
            hosting:
              gpu_layers: 999
        models:
          "target-model":
            name: "Qwen2.5-Coder-3B-Instruct-Q8_0"
            host:
              type: llama_cpp_with_vulkan
        workflow_execution_strategy:
          memory:
            model_lifecycle:
              unload_unused: true
        agentic_workflow:
          steps:
            step_1_read_code:
              generative_entity: "${models.target-model}"
              prompt: |
                Read and display the complete contents of the source file provided.
                Output every line with line numbers prefixed.
                Preserve all whitespace, comments, and formatting exactly.
                If the file is very long, output the first 500 lines.
              model_overrides:
                temperature: 0.2
                max_tokens: 50000
              when:
                after_step_succeeds:
                  - save_to: "./outputs/step1-source-code.txt"
                  - log:
                      to_file_path: "./logs/workflow.log"
                      event_fields: [step_name, duration_ms]
                after_step_fails:
                  - log:
                      to_file_path: "./logs/workflow.log"
                      event_fields: [step_name, error_message]
            step_2_static_analysis:
              generative_entity: "${models.target-model}"
              depends_on: [step_1_read_code]
              prompt: |
                Perform a thorough static analysis of the following source code.
                Check for these specific issues:
                - Dead code and unreachable branches
                - Unused variables and imports
                - Type mismatches and potential runtime errors
                - Performance bottlenecks (O(n²) loops, unnecessary allocations)
                - Naming convention violations
                SOURCE CODE TO ANALYZE:
                {{step.step_1_read_code.output}}
                Output a numbered list of issues with file location, severity (critical/warning/info),
                and a brief explanation of each issue.
              model_overrides:
                temperature: 0.3
                max_tokens: 50000
              when:
                after_step_succeeds:
                  - save_to: "./outputs/step2-static-analysis.txt"
                  - log:
                      to_file_path: "./logs/workflow.log"
                      event_fields: [step_name, duration_ms]
                after_step_fails:
                  - log:
                      to_file_path: "./logs/workflow.log"
                      event_fields: [step_name, error_message]
            step_3_security_scan:
              generative_entity: "${models.target-model}"
              depends_on: [step_1_read_code]
              prompt: |
                Perform a security scan of the following source code.
                Look for these vulnerability categories:
                - SQL injection and command injection
                - Cross-site scripting (XSS) vectors
                - Insecure cryptographic usage
                - Hardcoded secrets or credentials
                - Path traversal vulnerabilities
                SOURCE CODE TO SCAN:
                {{step.step_1_read_code.output}}
                Output each finding with: vulnerability type, location (line number),
                severity (critical/high/medium/low), and remediation advice.
              model_overrides:
                temperature: 0.2
                max_tokens: 50000
              when:
                after_step_succeeds:
                  - save_to: "./outputs/step3-security-scan.txt"
                  - log:
                      to_file_path: "./logs/workflow.log"
                      event_fields: [step_name, duration_ms]
                after_step_fails:
                  - log:
                      to_file_path: "./logs/workflow.log"
                      event_fields: [step_name, error_message]
            step_4_compile_report:
              generative_entity: "${models.target-model}"
              depends_on: [step_2_static_analysis, step_3_security_scan]
              prompt: |
                Compile a comprehensive code review report from the analysis results below.
                STATIC ANALYSIS RESULTS:
                {{step.step_2_static_analysis.output}}
                SECURITY SCAN RESULTS:
                {{step.step_3_security_scan.output}}
                Create a markdown report with:
                1. Executive summary with total issue count by severity
                2. Critical issues section (must fix before merge)
                3. Warnings section (should fix)
                4. Info section (nice to fix)
                5. Prioritized action items list
                Format each issue as: [SEVERITY] File:Line - Description
              model_overrides:
                temperature: 0.4
                max_tokens: 50000
              when:
                after_step_succeeds:
                  - save_to: "./outputs/step4-final-report.md"
                  - log:
                      to_file_path: "./logs/workflow.log"
                      event_fields: [step_name, duration_ms]
                after_step_fails:
                  - log:
                      to_file_path: "./logs/workflow.log"
                      event_fields: [step_name, error_message]
        ---
        IMPORTANT PATTERNS IN THE EXAMPLE ABOVE:
        - step_2 depends_on: [step_1] and uses {{step.step_1_read_code.output}} in its prompt
        - step_3 ALSO depends_on: [step_1] (PARALLEL with step_2)
        - step_4 depends_on BOTH: [step_2, step_3] and uses BOTH outputs
        - Every prompt is 5+ lines with specific instructions
        - Every step after step 1 uses {{step.X.output}} template interpolation
        NOW generate a COMPLETE YAML workflow for the TASK described at the top.
        CRITICAL RULES:
        1. Output ONLY raw YAML. NO markdown fences. NO ```yaml```. NO ``` anywhere.
        2. First line: workflow_id:  Last line: a YAML value.
        3. Steps MUST use MAP notation (step_name: with indented properties), NOT list notation
        4. Generate at least 6 steps
        5. At least 2 steps must run in PARALLEL (share the same dependency)
        6. At least 1 step must depend on 2+ other steps
        7. Every step after the first MUST have depends_on and MUST include {{step.X.output}} in its prompt
        8. Every prompt must be 5+ lines with specific detailed instructions
        9. Every step MUST have: generative_entity, prompt, model_overrides, when
        10. Every step MUST have model_overrides with temperature (0.1-0.7) and max_tokens: 50000
        11. Every step MUST have when: with after_step_succeeds (save_to + log) and after_step_fails (log)
        12. The workflow MUST be about SECURITY AUDITING, not code review
        13. Template interpolation MUST use the EXACT step name from depends_on. If a step is named step_3_check_deps then use {{step.step_3_check_deps.output}} NOT {{step.step_3_deps_check.output}}
      model_overrides:
        temperature: 0.15
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/v8-generated-raw.yml"
          - log:
              to_file_path: "./real-workflow-attempts/logs/v8-generator.log"
              event_fields: [step_name, duration_ms, token_count]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v8-generator.log"
              event_fields: [step_name, error_message]
    # ── PHASE 2: CLEAN + STRIP FENCES ──────────────────────────────
    step_2_clean:
      generative_entity: "${models.reviewer-model}"
      depends_on: [step_1_generate]
      prompt: |
        You are a YAML cleanup specialist. Your ONLY job is to strip formatting artifacts.
        INPUT:
        {{step.step_1_generate.output}}
        Perform these steps:
        1. Strip any markdown code fences (```yaml or ```) from start and end
        2. Strip any leading/trailing text before "workflow_id:" or after the last YAML line
        3. Ensure file starts with "workflow_id:" (no leading whitespace)
        4. Ensure proper YAML indentation (2 spaces per level)
        Output ONLY the cleaned YAML. No explanations. No markdown fences.
        First line: workflow_id:
        Last line: a YAML value, not a backtick.
      model_overrides:
        temperature: 0.05
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/v8-cleaned.yml"
          - log:
              to_file_path: "./real-workflow-attempts/logs/v8-generator.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v8-generator.log"
              event_fields: [step_name, error_message]
    # ── PHASE 3: VALIDATE + FIX ────────────────────────────────────
    # v8 fix: NEVER rename step references. Use EXACT step names from depends_on.
    step_3_validate_fix:
      generative_entity: "${models.reviewer-model}"
      depends_on: [step_2_clean]
      prompt: |
        You are a YAML workflow validation and repair specialist. You MUST fix the workflow to pass ALL checks below.
        INPUT WORKFLOW:
        {{step.step_2_clean.output}}
        MANDATORY CHECKS — Fix EVERY issue you find:
        CHECK 1: MAP notation. Steps must be "step_name:" with indented properties. NOT "- step_name:". Fix any list notation to map notation.
        CHECK 2: Required fields per step. Every step MUST have: generative_entity, prompt, model_overrides, when. Add any missing ones using this pattern:
          generative_entity: "${models.target-model}"
          model_overrides:
            temperature: 0.3
            max_tokens: 50000
          when:
            after_step_succeeds:
              - save_to: "./outputs/stepN-name.txt"
              - log:
                  to_file_path: "./logs/workflow.log"
                  event_fields: [step_name, duration_ms]
            after_step_fails:
              - log:
                  to_file_path: "./logs/workflow.log"
                  event_fields: [step_name, error_message]
        CHECK 3: depends_on. Every step except the first MUST have depends_on with at least one step name. Remove depends_on from the first step.
        CHECK 4: TEMPLATE INTERPOLATION — THIS IS THE MOST IMPORTANT CHECK.
        For EVERY step that has depends_on, its prompt MUST include at least one {{step.X.output}} reference.
        If a step's prompt does NOT contain {{step. then you MUST add this text block to its prompt:
            INPUT FROM PREVIOUS STEP:
            {{step.DEPENDENCY_STEP_NAME.output}}
        CRITICAL: Use the EXACT step name as it appears in the depends_on list.
        Do NOT abbreviate, rename, or guess step names.
        If depends_on says [step_2_check_deps], use {{step.step_2_check_deps.output}} exactly.
        If depends_on says [step_4_analyze_crypto, step_5_check_auth], use both exactly:
            {{step.step_4_analyze_crypto.output}}
            {{step.step_5_check_auth.output}}
        NEVER change an existing {{step.X.output}} reference. Keep it exactly as-is.
        CHECK 5: Prompt quality. Every prompt must be 5+ lines with specific instructions. Replace any placeholder text like "<prompt text here>" with a real detailed prompt relevant to the workflow's task.
        CHECK 6: Parallel structure. At least 2 steps must share the same parent in depends_on. At least 1 step must have 2+ items in depends_on.
        CHECK 7: Remove empty depends_on: [] from the first step.
        Output ONLY the fixed, validated YAML. No explanations. No markdown fences.
        First line: workflow_id:
        Last line: a YAML value.
      model_overrides:
        temperature: 0.05
        max_tokens: 50000
      when:
        before_step_starts:
          - shell:
              command: "python3"
              args: ["./real-workflow-attempts/scripts/validate-yaml.py", "./real-workflow-attempts/outputs/v8-cleaned.yml"]
              fail_on_error: false
          - log:
              to_file_path: "./real-workflow-attempts/logs/v8-generator.log"
              event_fields: [step_name, shell_output]
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/v8-final-workflow.yml"
          - shell:
              command: "python3"
              args: ["./real-workflow-attempts/scripts/validate-yaml.py", "./real-workflow-attempts/outputs/v8-final-workflow.yml"]
              fail_on_error: false
          - log:
              to_file_path: "./real-workflow-attempts/logs/v8-generator.log"
              event_fields: [step_name, duration_ms, shell_output]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v8-generator.log"
              event_fields: [step_name, error_message]

# ═══════════════════════════════════════════════════════════════════
# ADR Generation Benchmark — 15 Models
# Schema-compliant per docs/schema/unified-workflow-schema.yml v2.0
# Single step with iterate_values hook for model iteration
# ═══════════════════════════════════════════════════════════════════
workflow_id: adr_benchmark_15_models
name: "ADR Generation Benchmark - 15 Models"
description: "Generate JSON parsable ADR for vanilla JS TODO app using 15 models"
version: "2.0.0"
author: "Whitt Execution Engine"
tags: [benchmark, adr]
min_schema_version: "2.0.0"
schema_version: "2.0.0"
# ── PROVIDERS ───────────────────────────────────────────────────
providers:
  llama_cpp_with_vulkan:                               # schema line 28
    config:                                             # schema line 29
      host: localhost                                   # schema line 30
      port: 8080                                        # schema line 31
# ── MODELS (15 models across size range) ────────────────────────
# Model name field used to match GGUF file in models-dir
models:
  "qwen-05b":                                           # schema line 68
    name: "Qwen2.5-0.5B-Instruct-Q4_K_M"
    host:                                               # schema line 70
      type: llama_cpp_with_vulkan                       # schema line 71
  "qwen3-17b-abl":
    name: "Qwen3-1.7B-abliterated-q4_k_m"
    host:
      type: llama_cpp_with_vulkan
  "flan-sum":
    name: "flan-summarizer-v0.Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "lfm2-12b":
    name: "LFM2-1.2B-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "lfm2-12b-ext":
    name: "LFM2-1.2B-Extract-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "lfm25-12b-inst":
    name: "LFM2.5-1.2B-Instruct-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "lfm25-12b-think":
    name: "LFM2.5-1.2B-Thinking-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "llama-1b":
    name: "llama-3.2-1b-instruct-q8_0"
    host:
      type: llama_cpp_with_vulkan
  "falcon-h1-15b-deep":
    name: "Falcon-H1-1.5B-Deep-Instruct-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "refact-16b":
    name: "smallcloudai-Refact-1_6B-fim-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "stablelm-16b":
    name: "stablelm-2-zephyr-1_6b-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "sc2-3b":
    name: "starcoder2-3b-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "falcon-h1-3b":
    name: "Falcon-H1-3B-Instruct-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "readerlm-v2":
    name: "ReaderLM-v2.Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "qwen-15b":
    name: "qwen2.5-1.5b-instruct-q8_0"
    host:
      type: llama_cpp_with_vulkan
# ── WORKFLOW EXECUTION STRATEGY ────────────────────────────────
workflow_execution_strategy:                             # schema line 503
  memory:
    model_lifecycle:
      unload_unused: true                               # schema line 521
# ── AGENTIC WORKFLOW ───────────────────────────────────────────
# Single step iterated over 15 models via iterate_values hook.
# Runner expands into 15 resolved steps at execution time.
# Template vars: {{step.model_ref}}, {{step.model_name}}, {{iteration}}
agentic_workflow:                                        # schema line 196
  steps:                                                # schema line 293
    benchmark_adr:
      generative_entity: "${models.{{step.model_ref}}}"   # schema line 297 — resolved per iteration
      prompt: |
        Write a JSON file for an Architecture Decision Record (ADR) about building a vanilla JavaScript TODO app. No frameworks. Just HTML, JS, CSS files.
        Output a single JSON object with these exact fields filled in with real data:
        {"id":"todo-app","title":"Vanilla JS TODO App","status":"accepted","date":"2025-01-15","context":{"problem":"Need a simple task manager","constraints":["No frameworks","Only HTML JS CSS"]},"decision":{"approach":"Vanilla JS with localStorage","files":["index.html","style.css","app.js"]},"consequences":{"pros":["No dependencies","Fast load"],"cons":["Manual state management"]},"technical_spec":{"data_model":{"todo":{"id":"number","text":"string","done":"boolean"}},"functions":["addTodo","deleteTodo","toggleTodo","editTodo","filterTodos"],"ui_elements":["input#todo-input","button#add-btn","ul#todo-list","div#filters"],"storage":{"key":"todos","type":"array"}}}
        That example shows the structure. Now write a COMPLETE version with MORE detail in every field. Expand the arrays with more items. Add description strings to objects. Fill with realistic content.
        CRITICAL FORMAT RULES:
        - First character: { and last character: }
        - No backticks, no code fences, no markdown
        - No JavaScript code, no function expressions, no =>, no const, no let, no onclick
        - No comments, no // or /* or #
        - All strings in double quotes only
        - If a string contains a double quote, escape it with backslash like: "say \"hello\""
        - Do NOT use single quotes inside JSON strings
        - Do NOT put HTML attributes in string values
        - Keep UI element names simple like: "input#todo-input", "button#add-btn", "ul#todo-list"
        - Keep function names simple like: "addTodo", "deleteTodo", "toggleTodo"
        - Keep pros and cons as plain strings like: "No dependencies", "Fast to load"
      model_overrides:                                  # schema line 303
        max_tokens: 4096
      when:                                             # schema line 314
        before_step_starts:                             # schema line 315
          - iterate_values:
              step.model_ref: ["qwen-05b", "qwen3-17b-abl", "flan-sum", "lfm2-12b", "lfm2-12b-ext", "lfm25-12b-inst", "lfm25-12b-think", "llama-1b", "falcon-h1-15b-deep", "refact-16b", "stablelm-16b", "sc2-3b", "falcon-h1-3b", "readerlm-v2", "qwen-15b"]
              step.model_name: ["Qwen2.5-0.5B-Instruct-Q4_K_M", "Qwen3-1.7B-abliterated-q4_k_m", "flan-summarizer-v0.Q8_0", "LFM2-1.2B-Q8_0", "LFM2-1.2B-Extract-Q8_0", "LFM2.5-1.2B-Instruct-Q8_0", "LFM2.5-1.2B-Thinking-Q8_0", "llama-3.2-1b-instruct-q8_0", "Falcon-H1-1.5B-Deep-Instruct-Q8_0", "smallcloudai-Refact-1_6B-fim-Q8_0", "stablelm-2-zephyr-1_6b-Q8_0", "starcoder2-3b-Q4_K_M", "Falcon-H1-3B-Instruct-Q4_K_M", "ReaderLM-v2.Q8_0", "qwen2.5-1.5b-instruct-q8_0"]
        after_step_succeeds:                            # schema line 323
          - save_to: "./docs/benchmarks/outputs/output/{{step.model_name}}.json"  # schema line 476
          - shell:
              command: "free"
              args: ["-h"]
              fail_on_error: false
          - log:                                        # schema line 275
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms, json_parsable]
        after_step_fails:                               # schema line 328
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]

# ═══════════════════════════════════════════════════════════════════
# ADR Generation Benchmark — Top 3 Models (from 50-model sweep)
# Schema-compliant per docs/schema/unified-workflow-schema.yml v2.0
# Selected: Ministral-3-3B (10KB/11 keys), Qwen3-4B (7KB/8 keys),
#           Qwen2.5-Coder-3B (1.1KB/8 keys) — all valid JSON ADR
# ═══════════════════════════════════════════════════════════════════
workflow_id: adr_benchmark_3_models
name: "ADR Generation Benchmark - Top 3 Models"
description: "Generate JSON parsable ADR for vanilla JS TODO app — best 3 models from 50-model sweep"
version: "2.0.0"
author: "Whitt Execution Engine"
tags: [benchmark, adr]
min_schema_version: "2.0.0"
schema_version: "2.0.0"
# ── PROVIDERS ───────────────────────────────────────────────────
providers:
  llama_cpp_with_vulkan:                               # schema line 28
    config:                                             # schema line 29
      host: localhost                                   # schema line 30
      port: 8080                                        # schema line 31
# ── MODELS (top 3 from 50-model sweep by JSON validity + ADR quality) ──
models:
  "ministral-3b":                                       # schema line 68
    name: "Ministral-3-3B-Instruct-2512-Q4_K_M"
    host:                                               # schema line 70
      type: llama_cpp_with_vulkan                       # schema line 71
  "qwen3-4b":
    name: "Qwen3-4B-Instruct-2507-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "qwen2.5-coder-3b":
    name: "Qwen2.5-Coder-3B-Instruct-Q8_0"
    host:
      type: llama_cpp_with_vulkan
# ── WORKFLOW EXECUTION STRATEGY ────────────────────────────────
workflow_execution_strategy:                             # schema line 503
  memory:
    model_lifecycle:
      unload_unused: true                               # schema line 521
# ── AGENTIC WORKFLOW ───────────────────────────────────────────
# Single step iterated over 3 models via iterate_values hook.
# Runner expands into 3 resolved steps at execution time.
# Template vars: {{step.model_ref}}, {{step.model_name}}, {{iteration}}
agentic_workflow:                                        # schema line 196
  steps:                                                # schema line 293
    benchmark_adr:
      generative_entity: "${models.{{step.model_ref}}}"   # schema line 297 — resolved per iteration
      prompt: |
        Write a JSON file for an Architecture Decision Record (ADR) about building a vanilla JavaScript TODO app. No frameworks. Just HTML, JS, CSS files.
        Output a single JSON object with these exact fields filled in with real data:
        {"id":"todo-app","title":"Vanilla JS TODO App","status":"accepted","date":"2025-01-15","context":{"problem":"Need a simple task manager","constraints":["No frameworks","Only HTML JS CSS"]},"decision":{"approach":"Vanilla JS with localStorage","files":["index.html","style.css","app.js"]},"consequences":{"pros":["No dependencies","Fast load"],"cons":["Manual state management"]},"technical_spec":{"data_model":{"todo":{"id":"number","text":"string","done":"boolean"}},"functions":["addTodo","deleteTodo","toggleTodo","editTodo","filterTodos"],"ui_elements":["input#todo-input","button#add-btn","ul#todo-list","div#filters"],"storage":{"key":"todos","type":"array"}}}
        That example shows the structure. Now write a COMPLETE version with MORE detail in every field. Expand the arrays with more items. Add description strings to objects. Fill with realistic content.
        CRITICAL FORMAT RULES:
        - First character: { and last character: }
        - No backticks, no code fences, no markdown
        - No JavaScript code, no function expressions, no =>, no const, no let, no onclick
        - No comments, no // or /* or #
        - All strings in double quotes only
        - If a string contains a double quote, escape it with backslash like: "say \"hello\""
        - Do NOT use single quotes inside JSON strings
        - Do NOT put HTML attributes in string values
        - Keep UI element names simple like: "input#todo-input", "button#add-btn", "ul#todo-list"
        - Keep function names simple like: "addTodo", "deleteTodo", "toggleTodo"
        - Keep pros and cons as plain strings like: "No dependencies", "Fast to load"
      model_overrides:                                  # schema line 303
        max_tokens: 10000
        temperature: 0.1
      when:                                             # schema line 314
        before_step_starts:                             # schema line 315
          - iterate_values:
              step.model_ref: ["ministral-3b", "qwen3-4b", "qwen2.5-coder-3b"]
              step.model_name: ["Ministral-3-3B-Instruct-2512-Q4_K_M", "Qwen3-4B-Instruct-2507-Q4_K_M", "Qwen2.5-Coder-3B-Instruct-Q8_0"]
        after_step_succeeds:                            # schema line 323
          - save_to: "./docs/benchmarks/outputs/output/{{step.model_name}}.json"  # schema line 476
          - shell:
              command: "free"
              args: ["-h"]
              fail_on_error: false
          - log:                                        # schema line 275
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms, json_parsable]
        after_step_fails:                               # schema line 328
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]

# ═══════════════════════════════════════════════════════════════════
# ADR Generation Benchmark — 5 Models
# Schema-compliant per docs/schema/unified-workflow-schema.yml v2.0
# Single step with iterate_values hook for model iteration
# ═══════════════════════════════════════════════════════════════════
workflow_id: adr_benchmark_5_models
name: "ADR Generation Benchmark - 5 Models"
description: "Generate JSON parsable ADR for vanilla JS TODO app using 5 models"
version: "2.0.0"
author: "Whitt Execution Engine"
tags: [benchmark, adr]
min_schema_version: "2.0.0"
schema_version: "2.0.0"
# ── PROVIDERS ───────────────────────────────────────────────────
providers:
  llama_cpp_with_vulkan:                               # schema line 28
    config:                                             # schema line 29
      host: localhost                                   # schema line 30
      port: 8080                                        # schema line 31
# ── MODELS (each individually listed) ──────────────────────────
# Model name field used to match GGUF file in models-dir
models:
  "qwen-05b":                                           # schema line 68
    name: "Qwen2.5-0.5B-Instruct-Q4_K_M"
    host:                                               # schema line 70
      type: llama_cpp_with_vulkan                       # schema line 71
  "llama-1b":
    name: "llama-3.2-1b-instruct-q8_0"
    host:
      type: llama_cpp_with_vulkan
  "qwen-15b":
    name: "qwen2.5-1.5b-instruct-q8_0"
    host:
      type: llama_cpp_with_vulkan
  "phi-4-mini":
    name: "Phi-4-mini-instruct-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "qwen3-4b":
    name: "Qwen3-4B-Instruct-2507-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
# ── WORKFLOW EXECUTION STRATEGY ────────────────────────────────
workflow_execution_strategy:                             # schema line 503
  memory:
    model_lifecycle:
      unload_unused: true                               # schema line 521
# ── AGENTIC WORKFLOW ───────────────────────────────────────────
# Single step iterated over 5 models via iterate_values hook.
# Runner expands into 5 resolved steps at execution time.
# Template vars: {{step.model_ref}}, {{step.model_name}}, {{iteration}}
agentic_workflow:                                        # schema line 196
  steps:                                                # schema line 293
    benchmark_adr:
      generative_entity: "${models.{{step.model_ref}}}"   # schema line 297 — resolved per iteration
      prompt: |
        Write a JSON file for an Architecture Decision Record (ADR) about building a vanilla JavaScript TODO app. No frameworks. Just HTML, JS, CSS files.
        Output a single JSON object with these exact fields filled in with real data:
        {"id":"todo-app","title":"Vanilla JS TODO App","status":"accepted","date":"2025-01-15","context":{"problem":"Need a simple task manager","constraints":["No frameworks","Only HTML JS CSS"]},"decision":{"approach":"Vanilla JS with localStorage","files":["index.html","style.css","app.js"]},"consequences":{"pros":["No dependencies","Fast load"],"cons":["Manual state management"]},"technical_spec":{"data_model":{"todo":{"id":"number","text":"string","done":"boolean"}},"functions":["addTodo","deleteTodo","toggleTodo","editTodo","filterTodos"],"ui_elements":["input#todo-input","button#add-btn","ul#todo-list","div#filters"],"storage":{"key":"todos","type":"array"}}}
        That example shows the structure. Now write a COMPLETE version with MORE detail in every field. Expand the arrays with more items. Add description strings to objects. Fill with realistic content.
        CRITICAL FORMAT RULES:
        - First character: { and last character: }
        - No backticks, no code fences, no markdown
        - No JavaScript code, no function expressions, no =>, no const, no let, no onclick
        - No comments, no // or /* or #
        - All strings in double quotes only
        - If a string contains a double quote, escape it with backslash like: "say \"hello\""
        - Do NOT use single quotes inside JSON strings
        - Do NOT put HTML attributes in string values
        - Keep UI element names simple like: "input#todo-input", "button#add-btn", "ul#todo-list"
        - Keep function names simple like: "addTodo", "deleteTodo", "toggleTodo"
        - Keep pros and cons as plain strings like: "No dependencies", "Fast to load"
      model_overrides:                                  # schema line 303
        max_tokens: 4096
      when:                                             # schema line 314
        before_step_starts:                             # schema line 315
          - iterate_values:
              step.model_ref: ["qwen-05b", "llama-1b", "qwen-15b", "phi-4-mini", "qwen3-4b"]
              step.model_name: ["Qwen2.5-0.5B-Instruct-Q4_K_M", "llama-3.2-1b-instruct-q8_0", "qwen2.5-1.5b-instruct-q8_0", "Phi-4-mini-instruct-Q4_K_M", "Qwen3-4B-Instruct-2507-Q4_K_M"]
        after_step_succeeds:                            # schema line 323
          - save_to: "./docs/benchmarks/outputs/output/{{step.model_name}}.json"  # schema line 476
          - shell:
              command: "free"
              args: ["-h"]
              fail_on_error: false
          - log:                                        # schema line 275
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms, json_parsable]
        after_step_fails:                               # schema line 328
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]

# ═══════════════════════════════════════════════════════════════════
# ADR Generation Benchmark — 50 Models
# Schema-compliant per docs/schema/unified-workflow-schema.yml v2.0
# Single step with iterate_values hook for model iteration
# ═══════════════════════════════════════════════════════════════════
workflow_id: adr_benchmark_50_models
name: "ADR Generation Benchmark - 50 Models"
description: "Generate JSON parsable ADR for vanilla JS TODO app using 50 models"
version: "2.0.0"
author: "Whitt Execution Engine"
tags: [benchmark, adr]
min_schema_version: "2.0.0"
schema_version: "2.0.0"
# ── PROVIDERS ───────────────────────────────────────────────────
providers:
  llama_cpp_with_vulkan:                               # schema line 28
    config:                                             # schema line 29
      host: localhost                                   # schema line 30
      port: 8080                                        # schema line 31
# ── MODELS (50 models, all available on disk) ───────────────────────
# Model name field used to match GGUF file in models-dir
models:
  "qwen-05b":                                           # schema line 68
    name: "Qwen2.5-0.5B-Instruct-Q4_K_M"
    host:                                               # schema line 70
      type: llama_cpp_with_vulkan                       # schema line 71
  "qwen3-17b-abl":
    name: "Qwen3-1.7B-abliterated-q4_k_m"
    host:
      type: llama_cpp_with_vulkan
  "flan-sum":
    name: "flan-summarizer-v0.Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "lfm2-12b":
    name: "LFM2-1.2B-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "lfm2-12b-ext":
    name: "LFM2-1.2B-Extract-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "lfm25-12b-inst":
    name: "LFM2.5-1.2B-Instruct-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "lfm25-12b-think":
    name: "LFM2.5-1.2B-Thinking-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "llama-1b":
    name: "llama-3.2-1b-instruct-q8_0"
    host:
      type: llama_cpp_with_vulkan
  "falcon-h1-15b-deep":
    name: "Falcon-H1-1.5B-Deep-Instruct-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "refact-16b":
    name: "smallcloudai-Refact-1_6B-fim-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "stablelm-16b":
    name: "stablelm-2-zephyr-1_6b-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "sc2-3b":
    name: "starcoder2-3b-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "falcon-h1-3b":
    name: "Falcon-H1-3B-Instruct-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "readerlm-v2":
    name: "ReaderLM-v2.Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "qwen-15b":
    name: "qwen2.5-1.5b-instruct-q8_0"
    host:
      type: llama_cpp_with_vulkan
  "qwen-15b-coder":
    name: "qwen2.5-coder-1.5b-instruct-q8_0"
    host:
      type: llama_cpp_with_vulkan
  "smollm3":
    name: "SmolLM3-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "llama-3b":
    name: "Llama-3.2-3B-Instruct-Q4_K_S"
    host:
      type: llama_cpp_with_vulkan
  "smollm3-128k":
    name: "SmolLM3-3B-128K-UD-Q4_K_XL"
    host:
      type: llama_cpp_with_vulkan
  "flan-t5-xl":
    name: "flan-t5-xl-summary-map-reduce-1024-q5_0"
    host:
      type: llama_cpp_with_vulkan
  "stable-code-3b":
    name: "stable-code-3b-q5_k_m"
    host:
      type: llama_cpp_with_vulkan
  "phi-4-mini-abl":
    name: "Phi-4-mini-instruct-abliterated-Q3_K_M"
    host:
      type: llama_cpp_with_vulkan
  "ministral-3b":
    name: "Ministral-3-3B-Instruct-2512-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "sc2-3b-inst":
    name: "starcoder2-3b-instruct.i1-Q5_K_M"
    host:
      type: llama_cpp_with_vulkan
  "flan-t5-grm":
    name: "flan-t5-xl-grammar-synthesis-q6_k"
    host:
      type: llama_cpp_with_vulkan
  "sc2-3b-q6":
    name: "starcoder2-3b.Q6_K"
    host:
      type: llama_cpp_with_vulkan
  "phi-4-mini":
    name: "Phi-4-mini-instruct-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "phi-4-mini-reason":
    name: "Phi-4-mini-reasoning-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "qwen3-4b":
    name: "Qwen3-4B-Instruct-2507-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "qwen3-4b-think":
    name: "Qwen3-4B-Thinking-2507-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "granite-4h-micro":
    name: "granite-4.0-h-micro-Q6_K"
    host:
      type: llama_cpp_with_vulkan
  "jan-v3-4b":
    name: "Jan-v3-4b-base-instruct-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "aescoder-4b":
    name: "AesCoder-4B.Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "lfm2-26b-lmsg":
    name: "lfm2-2.6b-lmsguide-q8_0"
    host:
      type: llama_cpp_with_vulkan
  "lfm2-26b-sdg":
    name: "LFM2-2.6B-SDG-q8"
    host:
      type: llama_cpp_with_vulkan
  "granite-4-micro":
    name: "granite-4.0-micro-Q6_K"
    host:
      type: llama_cpp_with_vulkan
  "granite-3b-code-inst":
    name: "granite-3b-code-instruct-128k.i1-Q6_K"
    host:
      type: llama_cpp_with_vulkan
  "qwen-3b-coder":
    name: "Qwen2.5-Coder-3B-Instruct-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "jan-nano-128k":
    name: "jan-nano-128k-Q6_K"
    host:
      type: llama_cpp_with_vulkan
  "instella-3b":
    name: "Instella-3B-Q8"
    host:
      type: llama_cpp_with_vulkan
  "falcon3-3b":
    name: "Falcon3-3B-Instruct-q8_0"
    host:
      type: llama_cpp_with_vulkan
  "yi-6b":
    name: "Yi-6B-200K-Airo-Claude-Puffin-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "granite-3b-code":
    name: "granite-3b-code-base-128k.Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "lwm-text-chat":
    name: "LWM-Text-Chat-1M-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "mistral-7b":
    name: "mistral-7b-v0.1.Q4_K_S"
    host:
      type: llama_cpp_with_vulkan
  "mistral-7b-inst":
    name: "mistral-7b-instruct-v0.2.Q4_K_S"
    host:
      type: llama_cpp_with_vulkan
  "granite-4h-tiny":
    name: "granite-4.0-h-tiny-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "hermes-2-pro-7b":
    name: "Hermes-2-Pro-Mistral-7B.Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "mistral-7b-inst-v03":
    name: "Mistral-7B-Instruct-v0.3-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "falcon-h1-7b":
    name: "Falcon-H1-7B-Instruct-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
# ── WORKFLOW EXECUTION STRATEGY ────────────────────────────────
workflow_execution_strategy:                             # schema line 503
  memory:
    model_lifecycle:
      unload_unused: true                               # schema line 521
# ── AGENTIC WORKFLOW ───────────────────────────────────────────
# Single step iterated over 50 models via iterate_values hook.
# Runner expands into 50 resolved steps at execution time.
# Template vars: {{step.model_ref}}, {{step.model_name}}, {{iteration}}
agentic_workflow:                                        # schema line 196
  steps:                                                # schema line 293
    benchmark_adr:
      generative_entity: "${models.{{step.model_ref}}}"   # schema line 297 — resolved per iteration
      prompt: |                                         # schema line 298
        Write a JSON file for an Architecture Decision Record (ADR) about building a vanilla JavaScript TODO app. No frameworks. Just HTML, JS, CSS files.
        Output a single JSON object with these exact fields filled in with real data:
        {"id":"todo-app","title":"Vanilla JS TODO App","status":"accepted","date":"2025-01-15","context":{"problem":"Need a simple task manager","constraints":["No frameworks","Only HTML JS CSS"]},"decision":{"approach":"Vanilla JS with localStorage","files":["index.html","style.css","app.js"]},"consequences":{"pros":["No dependencies","Fast load"],"cons":["Manual state management"]},"technical_spec":{"data_model":{"todo":{"id":"number","text":"string","done":"boolean"}},"functions":["addTodo","deleteTodo","toggleTodo","editTodo","filterTodos"],"ui_elements":["input#todo-input","button#add-btn","ul#todo-list","div#filters"],"storage":{"key":"todos","type":"array"}}}
        That example shows the structure. Now write a COMPLETE version with MORE detail in every field. Expand the arrays with more items. Add description strings to objects. Fill with realistic content.
        CRITICAL FORMAT RULES:
        - First character: { and last character: }
        - No backticks, no code fences, no markdown
        - No JavaScript code, no function expressions, no =>, no const, no let, no onclick
        - No comments, no // or /* or #
        - All strings in double quotes only
        - If a string contains a double quote, escape it with backslash like: "say \"hello\""
        - Do NOT use single quotes inside JSON strings
        - Do NOT put HTML attributes in string values
        - Keep UI element names simple like: "input#todo-input", "button#add-btn", "ul#todo-list"
        - Keep function names simple like: "addTodo", "deleteTodo", "toggleTodo"
        - Keep pros and cons as plain strings like: "No dependencies", "Fast to load"
      model_overrides:                                  # schema line 303
        max_tokens: 10000
        temperature: 0.1
      when:                                             # schema line 314
        before_step_starts:                             # schema line 315
          - iterate_values:
              step.model_ref: ["qwen-05b", "qwen3-17b-abl", "flan-sum", "lfm2-12b", "lfm2-12b-ext", "lfm25-12b-inst", "lfm25-12b-think", "llama-1b", "falcon-h1-15b-deep", "refact-16b", "stablelm-16b", "sc2-3b", "falcon-h1-3b", "readerlm-v2", "qwen-15b", "qwen-15b-coder", "smollm3", "llama-3b", "smollm3-128k", "flan-t5-xl", "stable-code-3b", "phi-4-mini-abl", "ministral-3b", "sc2-3b-inst", "flan-t5-grm", "sc2-3b-q6", "phi-4-mini", "phi-4-mini-reason", "qwen3-4b", "qwen3-4b-think", "granite-4h-micro", "jan-v3-4b", "aescoder-4b", "lfm2-26b-lmsg", "lfm2-26b-sdg", "granite-4-micro", "granite-3b-code-inst", "qwen-3b-coder", "jan-nano-128k", "instella-3b", "falcon3-3b", "yi-6b", "granite-3b-code", "lwm-text-chat", "mistral-7b", "mistral-7b-inst", "granite-4h-tiny", "hermes-2-pro-7b", "mistral-7b-inst-v03", "falcon-h1-7b"]
              step.model_name: ["Qwen2.5-0.5B-Instruct-Q4_K_M", "Qwen3-1.7B-abliterated-q4_k_m", "flan-summarizer-v0.Q8_0", "LFM2-1.2B-Q8_0", "LFM2-1.2B-Extract-Q8_0", "LFM2.5-1.2B-Instruct-Q8_0", "LFM2.5-1.2B-Thinking-Q8_0", "llama-3.2-1b-instruct-q8_0", "Falcon-H1-1.5B-Deep-Instruct-Q8_0", "smallcloudai-Refact-1_6B-fim-Q8_0", "stablelm-2-zephyr-1_6b-Q8_0", "starcoder2-3b-Q4_K_M", "Falcon-H1-3B-Instruct-Q4_K_M", "ReaderLM-v2.Q8_0", "qwen2.5-1.5b-instruct-q8_0", "qwen2.5-coder-1.5b-instruct-q8_0", "SmolLM3-Q4_K_M", "Llama-3.2-3B-Instruct-Q4_K_S", "SmolLM3-3B-128K-UD-Q4_K_XL", "flan-t5-xl-summary-map-reduce-1024-q5_0", "stable-code-3b-q5_k_m", "Phi-4-mini-instruct-abliterated-Q3_K_M", "Ministral-3-3B-Instruct-2512-Q4_K_M", "starcoder2-3b-instruct.i1-Q5_K_M", "flan-t5-xl-grammar-synthesis-q6_k", "starcoder2-3b.Q6_K", "Phi-4-mini-instruct-Q4_K_M", "Phi-4-mini-reasoning-Q4_K_M", "Qwen3-4B-Instruct-2507-Q4_K_M", "Qwen3-4B-Thinking-2507-Q4_K_M", "granite-4.0-h-micro-Q6_K", "Jan-v3-4b-base-instruct-Q4_K_M", "AesCoder-4B.Q4_K_M", "lfm2-2.6b-lmsguide-q8_0", "LFM2-2.6B-SDG-q8", "granite-4.0-micro-Q6_K", "granite-3b-code-instruct-128k.i1-Q6_K", "Qwen2.5-Coder-3B-Instruct-Q8_0", "jan-nano-128k-Q6_K", "Instella-3B-Q8", "Falcon3-3B-Instruct-q8_0", "Yi-6B-200K-Airo-Claude-Puffin-Q4_K_M", "granite-3b-code-base-128k.Q8_0", "LWM-Text-Chat-1M-Q4_K_M", "mistral-7b-v0.1.Q4_K_S", "mistral-7b-instruct-v0.2.Q4_K_S", "granite-4.0-h-tiny-Q4_K_M", "Hermes-2-Pro-Mistral-7B.Q4_K_M", "Mistral-7B-Instruct-v0.3-Q4_K_M", "Falcon-H1-7B-Instruct-Q4_K_M"]
        after_step_succeeds:                            # schema line 323
          - save_to: "./docs/benchmarks/outputs/output/{{step.model_name}}.json"  # schema line 476
          - shell:
              command: "free"
              args: ["-h"]
              fail_on_error: false
          - log:                                        # schema line 275
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms, json_parsable]
        after_step_fails:                               # schema line 328
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]

# ═══════════════════════════════════════════════════════════════════
# ADR Generation Benchmark — 15 Models
# Schema-compliant per docs/schema/unified-workflow-schema.yml v2.0
# Single step with iterate_values hook for model iteration
# ═══════════════════════════════════════════════════════════════════
workflow_id: adr_benchmark_15_models
name: "ADR Generation Benchmark - 15 Models"
description: "Generate JSON parsable ADR for vanilla JS TODO app using 15 models"
version: "2.0.0"
author: "Whitt Execution Engine"
tags: [benchmark, adr]
min_schema_version: "2.0.0"
schema_version: "2.0.0"
# ── PROVIDERS ───────────────────────────────────────────────────
providers:
  llama_cpp_with_vulkan:                               # schema line 28
    config:                                             # schema line 29
      host: localhost                                   # schema line 30
      port: 8080                                        # schema line 31
# ── MODELS (15 models across size range) ────────────────────────
# Model name field used to match GGUF file in models-dir
models:
  "qwen-05b":                                           # schema line 68
    name: "Qwen2.5-0.5B-Instruct-Q4_K_M"
    host:                                               # schema line 70
      type: llama_cpp_with_vulkan                       # schema line 71
  "qwen3-17b-abl":
    name: "Qwen3-1.7B-abliterated-q4_k_m"
    host:
      type: llama_cpp_with_vulkan
  "flan-sum":
    name: "flan-summarizer-v0.Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "lfm2-12b":
    name: "LFM2-1.2B-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "lfm2-12b-ext":
    name: "LFM2-1.2B-Extract-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "lfm25-12b-inst":
    name: "LFM2.5-1.2B-Instruct-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "lfm25-12b-think":
    name: "LFM2.5-1.2B-Thinking-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "llama-1b":
    name: "llama-3.2-1b-instruct-q8_0"
    host:
      type: llama_cpp_with_vulkan
  "falcon-h1-15b-deep":
    name: "Falcon-H1-1.5B-Deep-Instruct-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "refact-16b":
    name: "smallcloudai-Refact-1_6B-fim-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "stablelm-16b":
    name: "stablelm-2-zephyr-1_6b-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "sc2-3b":
    name: "starcoder2-3b-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "falcon-h1-3b":
    name: "Falcon-H1-3B-Instruct-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "readerlm-v2":
    name: "ReaderLM-v2.Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "qwen-15b":
    name: "qwen2.5-1.5b-instruct-q8_0"
    host:
      type: llama_cpp_with_vulkan
# ── WORKFLOW EXECUTION STRATEGY ────────────────────────────────
workflow_execution_strategy:                             # schema line 503
  memory:
    model_lifecycle:
      unload_unused: true                               # schema line 521
# ── AGENTIC WORKFLOW ───────────────────────────────────────────
# Single step iterated over 15 models via iterate_values hook.
# Runner expands into 15 resolved steps at execution time.
# Template vars: {{step.model_ref}}, {{step.model_name}}, {{iteration}}
agentic_workflow:                                        # schema line 196
  steps:                                                # schema line 293
    benchmark_adr:
      generative_entity: "${models.{{step.model_ref}}}"   # schema line 297 — resolved per iteration
      prompt: |
        Write a JSON file for an Architecture Decision Record (ADR) about building a vanilla JavaScript TODO app. No frameworks. Just HTML, JS, CSS files.
        Output a single JSON object with these exact fields filled in with real data:
        {"id":"todo-app","title":"Vanilla JS TODO App","status":"accepted","date":"2025-01-15","context":{"problem":"Need a simple task manager","constraints":["No frameworks","Only HTML JS CSS"]},"decision":{"approach":"Vanilla JS with localStorage","files":["index.html","style.css","app.js"]},"consequences":{"pros":["No dependencies","Fast load"],"cons":["Manual state management"]},"technical_spec":{"data_model":{"todo":{"id":"number","text":"string","done":"boolean"}},"functions":["addTodo","deleteTodo","toggleTodo","editTodo","filterTodos"],"ui_elements":["input#todo-input","button#add-btn","ul#todo-list","div#filters"],"storage":{"key":"todos","type":"array"}}}
        That example shows the structure. Now write a COMPLETE version with MORE detail in every field. Expand the arrays with more items. Add description strings to objects. Fill with realistic content.
        CRITICAL FORMAT RULES:
        - First character: { and last character: }
        - No backticks, no code fences, no markdown
        - No JavaScript code, no function expressions, no =>, no const, no let, no onclick
        - No comments, no // or /* or #
        - All strings in double quotes only
        - If a string contains a double quote, escape it with backslash like: "say \"hello\""
        - Do NOT use single quotes inside JSON strings
        - Do NOT put HTML attributes in string values
        - Keep UI element names simple like: "input#todo-input", "button#add-btn", "ul#todo-list"
        - Keep function names simple like: "addTodo", "deleteTodo", "toggleTodo"
        - Keep pros and cons as plain strings like: "No dependencies", "Fast to load"
      model_overrides:                                  # schema line 303
        max_tokens: 4096
      when:                                             # schema line 314
        before_step_starts:                             # schema line 315
          - iterate_values:
              step.model_ref: ["qwen-05b", "qwen3-17b-abl", "flan-sum", "lfm2-12b", "lfm2-12b-ext", "lfm25-12b-inst", "lfm25-12b-think", "llama-1b", "falcon-h1-15b-deep", "refact-16b", "stablelm-16b", "sc2-3b", "falcon-h1-3b", "readerlm-v2", "qwen-15b"]
              step.model_name: ["Qwen2.5-0.5B-Instruct-Q4_K_M", "Qwen3-1.7B-abliterated-q4_k_m", "flan-summarizer-v0.Q8_0", "LFM2-1.2B-Q8_0", "LFM2-1.2B-Extract-Q8_0", "LFM2.5-1.2B-Instruct-Q8_0", "LFM2.5-1.2B-Thinking-Q8_0", "llama-3.2-1b-instruct-q8_0", "Falcon-H1-1.5B-Deep-Instruct-Q8_0", "smallcloudai-Refact-1_6B-fim-Q8_0", "stablelm-2-zephyr-1_6b-Q8_0", "starcoder2-3b-Q4_K_M", "Falcon-H1-3B-Instruct-Q4_K_M", "ReaderLM-v2.Q8_0", "qwen2.5-1.5b-instruct-q8_0"]
        after_step_succeeds:                            # schema line 323
          - save_to: "./docs/benchmarks/outputs/output/{{step.model_name}}.json"  # schema line 476
          - shell:
              command: "free"
              args: ["-h"]
              fail_on_error: false
          - log:                                        # schema line 275
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms, json_parsable]
        after_step_fails:                               # schema line 328
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]

# ═══════════════════════════════════════════════════════════════════
# ADR Generation Benchmark — Top 3 Models (from 50-model sweep)
# Schema-compliant per docs/schema/unified-workflow-schema.yml v2.0
# Selected: Ministral-3-3B (10KB/11 keys), Qwen3-4B (7KB/8 keys),
#           Qwen2.5-Coder-3B (1.1KB/8 keys) — all valid JSON ADR
# ═══════════════════════════════════════════════════════════════════
workflow_id: adr_benchmark_3_models
name: "ADR Generation Benchmark - Top 3 Models"
description: "Generate JSON parsable ADR for vanilla JS TODO app — best 3 models from 50-model sweep"
version: "2.0.0"
author: "Whitt Execution Engine"
tags: [benchmark, adr]
min_schema_version: "2.0.0"
schema_version: "2.0.0"
# ── PROVIDERS ───────────────────────────────────────────────────
providers:
  llama_cpp_with_vulkan:                               # schema line 28
    config:                                             # schema line 29
      host: localhost                                   # schema line 30
      port: 8080                                        # schema line 31
# ── MODELS (top 3 from 50-model sweep by JSON validity + ADR quality) ──
models:
  "ministral-3b":                                       # schema line 68
    name: "Ministral-3-3B-Instruct-2512-Q4_K_M"
    host:                                               # schema line 70
      type: llama_cpp_with_vulkan                       # schema line 71
  "qwen3-4b":
    name: "Qwen3-4B-Instruct-2507-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "qwen2.5-coder-3b":
    name: "Qwen2.5-Coder-3B-Instruct-Q8_0"
    host:
      type: llama_cpp_with_vulkan
# ── WORKFLOW EXECUTION STRATEGY ────────────────────────────────
workflow_execution_strategy:                             # schema line 503
  memory:
    model_lifecycle:
      unload_unused: true                               # schema line 521
# ── AGENTIC WORKFLOW ───────────────────────────────────────────
# Single step iterated over 3 models via iterate_values hook.
# Runner expands into 3 resolved steps at execution time.
# Template vars: {{step.model_ref}}, {{step.model_name}}, {{iteration}}
agentic_workflow:                                        # schema line 196
  steps:                                                # schema line 293
    benchmark_adr:
      generative_entity: "${models.{{step.model_ref}}}"   # schema line 297 — resolved per iteration
      prompt: |
        Write a JSON file for an Architecture Decision Record (ADR) about building a vanilla JavaScript TODO app. No frameworks. Just HTML, JS, CSS files.
        Output a single JSON object with these exact fields filled in with real data:
        {"id":"todo-app","title":"Vanilla JS TODO App","status":"accepted","date":"2025-01-15","context":{"problem":"Need a simple task manager","constraints":["No frameworks","Only HTML JS CSS"]},"decision":{"approach":"Vanilla JS with localStorage","files":["index.html","style.css","app.js"]},"consequences":{"pros":["No dependencies","Fast load"],"cons":["Manual state management"]},"technical_spec":{"data_model":{"todo":{"id":"number","text":"string","done":"boolean"}},"functions":["addTodo","deleteTodo","toggleTodo","editTodo","filterTodos"],"ui_elements":["input#todo-input","button#add-btn","ul#todo-list","div#filters"],"storage":{"key":"todos","type":"array"}}}
        That example shows the structure. Now write a COMPLETE version with MORE detail in every field. Expand the arrays with more items. Add description strings to objects. Fill with realistic content.
        CRITICAL FORMAT RULES:
        - First character: { and last character: }
        - No backticks, no code fences, no markdown
        - No JavaScript code, no function expressions, no =>, no const, no let, no onclick
        - No comments, no // or /* or #
        - All strings in double quotes only
        - If a string contains a double quote, escape it with backslash like: "say \"hello\""
        - Do NOT use single quotes inside JSON strings
        - Do NOT put HTML attributes in string values
        - Keep UI element names simple like: "input#todo-input", "button#add-btn", "ul#todo-list"
        - Keep function names simple like: "addTodo", "deleteTodo", "toggleTodo"
        - Keep pros and cons as plain strings like: "No dependencies", "Fast to load"
      model_overrides:                                  # schema line 303
        max_tokens: 10000
        temperature: 0.1
      when:                                             # schema line 314
        before_step_starts:                             # schema line 315
          - iterate_values:
              step.model_ref: ["ministral-3b", "qwen3-4b", "qwen2.5-coder-3b"]
              step.model_name: ["Ministral-3-3B-Instruct-2512-Q4_K_M", "Qwen3-4B-Instruct-2507-Q4_K_M", "Qwen2.5-Coder-3B-Instruct-Q8_0"]
        after_step_succeeds:                            # schema line 323
          - save_to: "./docs/benchmarks/outputs/output/{{step.model_name}}.json"  # schema line 476
          - shell:
              command: "free"
              args: ["-h"]
              fail_on_error: false
          - log:                                        # schema line 275
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms, json_parsable]
        after_step_fails:                               # schema line 328
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]

# ═══════════════════════════════════════════════════════════════════
# ADR Generation Benchmark — 5 Models
# Schema-compliant per docs/schema/unified-workflow-schema.yml v2.0
# Single step with iterate_values hook for model iteration
# ═══════════════════════════════════════════════════════════════════
workflow_id: adr_benchmark_5_models
name: "ADR Generation Benchmark - 5 Models"
description: "Generate JSON parsable ADR for vanilla JS TODO app using 5 models"
version: "2.0.0"
author: "Whitt Execution Engine"
tags: [benchmark, adr]
min_schema_version: "2.0.0"
schema_version: "2.0.0"
# ── PROVIDERS ───────────────────────────────────────────────────
providers:
  llama_cpp_with_vulkan:                               # schema line 28
    config:                                             # schema line 29
      host: localhost                                   # schema line 30
      port: 8080                                        # schema line 31
# ── MODELS (each individually listed) ──────────────────────────
# Model name field used to match GGUF file in models-dir
models:
  "qwen-05b":                                           # schema line 68
    name: "Qwen2.5-0.5B-Instruct-Q4_K_M"
    host:                                               # schema line 70
      type: llama_cpp_with_vulkan                       # schema line 71
  "llama-1b":
    name: "llama-3.2-1b-instruct-q8_0"
    host:
      type: llama_cpp_with_vulkan
  "qwen-15b":
    name: "qwen2.5-1.5b-instruct-q8_0"
    host:
      type: llama_cpp_with_vulkan
  "phi-4-mini":
    name: "Phi-4-mini-instruct-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "qwen3-4b":
    name: "Qwen3-4B-Instruct-2507-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
# ── WORKFLOW EXECUTION STRATEGY ────────────────────────────────
workflow_execution_strategy:                             # schema line 503
  memory:
    model_lifecycle:
      unload_unused: true                               # schema line 521
# ── AGENTIC WORKFLOW ───────────────────────────────────────────
# Single step iterated over 5 models via iterate_values hook.
# Runner expands into 5 resolved steps at execution time.
# Template vars: {{step.model_ref}}, {{step.model_name}}, {{iteration}}
agentic_workflow:                                        # schema line 196
  steps:                                                # schema line 293
    benchmark_adr:
      generative_entity: "${models.{{step.model_ref}}}"   # schema line 297 — resolved per iteration
      prompt: |
        Write a JSON file for an Architecture Decision Record (ADR) about building a vanilla JavaScript TODO app. No frameworks. Just HTML, JS, CSS files.
        Output a single JSON object with these exact fields filled in with real data:
        {"id":"todo-app","title":"Vanilla JS TODO App","status":"accepted","date":"2025-01-15","context":{"problem":"Need a simple task manager","constraints":["No frameworks","Only HTML JS CSS"]},"decision":{"approach":"Vanilla JS with localStorage","files":["index.html","style.css","app.js"]},"consequences":{"pros":["No dependencies","Fast load"],"cons":["Manual state management"]},"technical_spec":{"data_model":{"todo":{"id":"number","text":"string","done":"boolean"}},"functions":["addTodo","deleteTodo","toggleTodo","editTodo","filterTodos"],"ui_elements":["input#todo-input","button#add-btn","ul#todo-list","div#filters"],"storage":{"key":"todos","type":"array"}}}
        That example shows the structure. Now write a COMPLETE version with MORE detail in every field. Expand the arrays with more items. Add description strings to objects. Fill with realistic content.
        CRITICAL FORMAT RULES:
        - First character: { and last character: }
        - No backticks, no code fences, no markdown
        - No JavaScript code, no function expressions, no =>, no const, no let, no onclick
        - No comments, no // or /* or #
        - All strings in double quotes only
        - If a string contains a double quote, escape it with backslash like: "say \"hello\""
        - Do NOT use single quotes inside JSON strings
        - Do NOT put HTML attributes in string values
        - Keep UI element names simple like: "input#todo-input", "button#add-btn", "ul#todo-list"
        - Keep function names simple like: "addTodo", "deleteTodo", "toggleTodo"
        - Keep pros and cons as plain strings like: "No dependencies", "Fast to load"
      model_overrides:                                  # schema line 303
        max_tokens: 4096
      when:                                             # schema line 314
        before_step_starts:                             # schema line 315
          - iterate_values:
              step.model_ref: ["qwen-05b", "llama-1b", "qwen-15b", "phi-4-mini", "qwen3-4b"]
              step.model_name: ["Qwen2.5-0.5B-Instruct-Q4_K_M", "llama-3.2-1b-instruct-q8_0", "qwen2.5-1.5b-instruct-q8_0", "Phi-4-mini-instruct-Q4_K_M", "Qwen3-4B-Instruct-2507-Q4_K_M"]
        after_step_succeeds:                            # schema line 323
          - save_to: "./docs/benchmarks/outputs/output/{{step.model_name}}.json"  # schema line 476
          - shell:
              command: "free"
              args: ["-h"]
              fail_on_error: false
          - log:                                        # schema line 275
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms, json_parsable]
        after_step_fails:                               # schema line 328
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]

# ═══════════════════════════════════════════════════════════════════
# ADR Generation Benchmark — 50 Models
# Schema-compliant per docs/schema/unified-workflow-schema.yml v2.0
# Single step with iterate_values hook for model iteration
# ═══════════════════════════════════════════════════════════════════
workflow_id: adr_benchmark_50_models
name: "ADR Generation Benchmark - 50 Models"
description: "Generate JSON parsable ADR for vanilla JS TODO app using 50 models"
version: "2.0.0"
author: "Whitt Execution Engine"
tags: [benchmark, adr]
min_schema_version: "2.0.0"
schema_version: "2.0.0"
# ── PROVIDERS ───────────────────────────────────────────────────
providers:
  llama_cpp_with_vulkan:                               # schema line 28
    config:                                             # schema line 29
      host: localhost                                   # schema line 30
      port: 8080                                        # schema line 31
# ── MODELS (50 models, all available on disk) ───────────────────────
# Model name field used to match GGUF file in models-dir
models:
  "qwen-05b":                                           # schema line 68
    name: "Qwen2.5-0.5B-Instruct-Q4_K_M"
    host:                                               # schema line 70
      type: llama_cpp_with_vulkan                       # schema line 71
  "qwen3-17b-abl":
    name: "Qwen3-1.7B-abliterated-q4_k_m"
    host:
      type: llama_cpp_with_vulkan
  "flan-sum":
    name: "flan-summarizer-v0.Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "lfm2-12b":
    name: "LFM2-1.2B-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "lfm2-12b-ext":
    name: "LFM2-1.2B-Extract-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "lfm25-12b-inst":
    name: "LFM2.5-1.2B-Instruct-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "lfm25-12b-think":
    name: "LFM2.5-1.2B-Thinking-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "llama-1b":
    name: "llama-3.2-1b-instruct-q8_0"
    host:
      type: llama_cpp_with_vulkan
  "falcon-h1-15b-deep":
    name: "Falcon-H1-1.5B-Deep-Instruct-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "refact-16b":
    name: "smallcloudai-Refact-1_6B-fim-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "stablelm-16b":
    name: "stablelm-2-zephyr-1_6b-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "sc2-3b":
    name: "starcoder2-3b-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "falcon-h1-3b":
    name: "Falcon-H1-3B-Instruct-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "readerlm-v2":
    name: "ReaderLM-v2.Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "qwen-15b":
    name: "qwen2.5-1.5b-instruct-q8_0"
    host:
      type: llama_cpp_with_vulkan
  "qwen-15b-coder":
    name: "qwen2.5-coder-1.5b-instruct-q8_0"
    host:
      type: llama_cpp_with_vulkan
  "smollm3":
    name: "SmolLM3-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "llama-3b":
    name: "Llama-3.2-3B-Instruct-Q4_K_S"
    host:
      type: llama_cpp_with_vulkan
  "smollm3-128k":
    name: "SmolLM3-3B-128K-UD-Q4_K_XL"
    host:
      type: llama_cpp_with_vulkan
  "flan-t5-xl":
    name: "flan-t5-xl-summary-map-reduce-1024-q5_0"
    host:
      type: llama_cpp_with_vulkan
  "stable-code-3b":
    name: "stable-code-3b-q5_k_m"
    host:
      type: llama_cpp_with_vulkan
  "phi-4-mini-abl":
    name: "Phi-4-mini-instruct-abliterated-Q3_K_M"
    host:
      type: llama_cpp_with_vulkan
  "ministral-3b":
    name: "Ministral-3-3B-Instruct-2512-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "sc2-3b-inst":
    name: "starcoder2-3b-instruct.i1-Q5_K_M"
    host:
      type: llama_cpp_with_vulkan
  "flan-t5-grm":
    name: "flan-t5-xl-grammar-synthesis-q6_k"
    host:
      type: llama_cpp_with_vulkan
  "sc2-3b-q6":
    name: "starcoder2-3b.Q6_K"
    host:
      type: llama_cpp_with_vulkan
  "phi-4-mini":
    name: "Phi-4-mini-instruct-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "phi-4-mini-reason":
    name: "Phi-4-mini-reasoning-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "qwen3-4b":
    name: "Qwen3-4B-Instruct-2507-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "qwen3-4b-think":
    name: "Qwen3-4B-Thinking-2507-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "granite-4h-micro":
    name: "granite-4.0-h-micro-Q6_K"
    host:
      type: llama_cpp_with_vulkan
  "jan-v3-4b":
    name: "Jan-v3-4b-base-instruct-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "aescoder-4b":
    name: "AesCoder-4B.Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "lfm2-26b-lmsg":
    name: "lfm2-2.6b-lmsguide-q8_0"
    host:
      type: llama_cpp_with_vulkan
  "lfm2-26b-sdg":
    name: "LFM2-2.6B-SDG-q8"
    host:
      type: llama_cpp_with_vulkan
  "granite-4-micro":
    name: "granite-4.0-micro-Q6_K"
    host:
      type: llama_cpp_with_vulkan
  "granite-3b-code-inst":
    name: "granite-3b-code-instruct-128k.i1-Q6_K"
    host:
      type: llama_cpp_with_vulkan
  "qwen-3b-coder":
    name: "Qwen2.5-Coder-3B-Instruct-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "jan-nano-128k":
    name: "jan-nano-128k-Q6_K"
    host:
      type: llama_cpp_with_vulkan
  "instella-3b":
    name: "Instella-3B-Q8"
    host:
      type: llama_cpp_with_vulkan
  "falcon3-3b":
    name: "Falcon3-3B-Instruct-q8_0"
    host:
      type: llama_cpp_with_vulkan
  "yi-6b":
    name: "Yi-6B-200K-Airo-Claude-Puffin-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "granite-3b-code":
    name: "granite-3b-code-base-128k.Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "lwm-text-chat":
    name: "LWM-Text-Chat-1M-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "mistral-7b":
    name: "mistral-7b-v0.1.Q4_K_S"
    host:
      type: llama_cpp_with_vulkan
  "mistral-7b-inst":
    name: "mistral-7b-instruct-v0.2.Q4_K_S"
    host:
      type: llama_cpp_with_vulkan
  "granite-4h-tiny":
    name: "granite-4.0-h-tiny-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "hermes-2-pro-7b":
    name: "Hermes-2-Pro-Mistral-7B.Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "mistral-7b-inst-v03":
    name: "Mistral-7B-Instruct-v0.3-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "falcon-h1-7b":
    name: "Falcon-H1-7B-Instruct-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
# ── WORKFLOW EXECUTION STRATEGY ────────────────────────────────
workflow_execution_strategy:                             # schema line 503
  memory:
    model_lifecycle:
      unload_unused: true                               # schema line 521
# ── AGENTIC WORKFLOW ───────────────────────────────────────────
# Single step iterated over 50 models via iterate_values hook.
# Runner expands into 50 resolved steps at execution time.
# Template vars: {{step.model_ref}}, {{step.model_name}}, {{iteration}}
agentic_workflow:                                        # schema line 196
  steps:                                                # schema line 293
    benchmark_adr:
      generative_entity: "${models.{{step.model_ref}}}"   # schema line 297 — resolved per iteration
      prompt: |                                         # schema line 298
        Write a JSON file for an Architecture Decision Record (ADR) about building a vanilla JavaScript TODO app. No frameworks. Just HTML, JS, CSS files.
        Output a single JSON object with these exact fields filled in with real data:
        {"id":"todo-app","title":"Vanilla JS TODO App","status":"accepted","date":"2025-01-15","context":{"problem":"Need a simple task manager","constraints":["No frameworks","Only HTML JS CSS"]},"decision":{"approach":"Vanilla JS with localStorage","files":["index.html","style.css","app.js"]},"consequences":{"pros":["No dependencies","Fast load"],"cons":["Manual state management"]},"technical_spec":{"data_model":{"todo":{"id":"number","text":"string","done":"boolean"}},"functions":["addTodo","deleteTodo","toggleTodo","editTodo","filterTodos"],"ui_elements":["input#todo-input","button#add-btn","ul#todo-list","div#filters"],"storage":{"key":"todos","type":"array"}}}
        That example shows the structure. Now write a COMPLETE version with MORE detail in every field. Expand the arrays with more items. Add description strings to objects. Fill with realistic content.
        CRITICAL FORMAT RULES:
        - First character: { and last character: }
        - No backticks, no code fences, no markdown
        - No JavaScript code, no function expressions, no =>, no const, no let, no onclick
        - No comments, no // or /* or #
        - All strings in double quotes only
        - If a string contains a double quote, escape it with backslash like: "say \"hello\""
        - Do NOT use single quotes inside JSON strings
        - Do NOT put HTML attributes in string values
        - Keep UI element names simple like: "input#todo-input", "button#add-btn", "ul#todo-list"
        - Keep function names simple like: "addTodo", "deleteTodo", "toggleTodo"
        - Keep pros and cons as plain strings like: "No dependencies", "Fast to load"
      model_overrides:                                  # schema line 303
        max_tokens: 10000
        temperature: 0.1
      when:                                             # schema line 314
        before_step_starts:                             # schema line 315
          - iterate_values:
              step.model_ref: ["qwen-05b", "qwen3-17b-abl", "flan-sum", "lfm2-12b", "lfm2-12b-ext", "lfm25-12b-inst", "lfm25-12b-think", "llama-1b", "falcon-h1-15b-deep", "refact-16b", "stablelm-16b", "sc2-3b", "falcon-h1-3b", "readerlm-v2", "qwen-15b", "qwen-15b-coder", "smollm3", "llama-3b", "smollm3-128k", "flan-t5-xl", "stable-code-3b", "phi-4-mini-abl", "ministral-3b", "sc2-3b-inst", "flan-t5-grm", "sc2-3b-q6", "phi-4-mini", "phi-4-mini-reason", "qwen3-4b", "qwen3-4b-think", "granite-4h-micro", "jan-v3-4b", "aescoder-4b", "lfm2-26b-lmsg", "lfm2-26b-sdg", "granite-4-micro", "granite-3b-code-inst", "qwen-3b-coder", "jan-nano-128k", "instella-3b", "falcon3-3b", "yi-6b", "granite-3b-code", "lwm-text-chat", "mistral-7b", "mistral-7b-inst", "granite-4h-tiny", "hermes-2-pro-7b", "mistral-7b-inst-v03", "falcon-h1-7b"]
              step.model_name: ["Qwen2.5-0.5B-Instruct-Q4_K_M", "Qwen3-1.7B-abliterated-q4_k_m", "flan-summarizer-v0.Q8_0", "LFM2-1.2B-Q8_0", "LFM2-1.2B-Extract-Q8_0", "LFM2.5-1.2B-Instruct-Q8_0", "LFM2.5-1.2B-Thinking-Q8_0", "llama-3.2-1b-instruct-q8_0", "Falcon-H1-1.5B-Deep-Instruct-Q8_0", "smallcloudai-Refact-1_6B-fim-Q8_0", "stablelm-2-zephyr-1_6b-Q8_0", "starcoder2-3b-Q4_K_M", "Falcon-H1-3B-Instruct-Q4_K_M", "ReaderLM-v2.Q8_0", "qwen2.5-1.5b-instruct-q8_0", "qwen2.5-coder-1.5b-instruct-q8_0", "SmolLM3-Q4_K_M", "Llama-3.2-3B-Instruct-Q4_K_S", "SmolLM3-3B-128K-UD-Q4_K_XL", "flan-t5-xl-summary-map-reduce-1024-q5_0", "stable-code-3b-q5_k_m", "Phi-4-mini-instruct-abliterated-Q3_K_M", "Ministral-3-3B-Instruct-2512-Q4_K_M", "starcoder2-3b-instruct.i1-Q5_K_M", "flan-t5-xl-grammar-synthesis-q6_k", "starcoder2-3b.Q6_K", "Phi-4-mini-instruct-Q4_K_M", "Phi-4-mini-reasoning-Q4_K_M", "Qwen3-4B-Instruct-2507-Q4_K_M", "Qwen3-4B-Thinking-2507-Q4_K_M", "granite-4.0-h-micro-Q6_K", "Jan-v3-4b-base-instruct-Q4_K_M", "AesCoder-4B.Q4_K_M", "lfm2-2.6b-lmsguide-q8_0", "LFM2-2.6B-SDG-q8", "granite-4.0-micro-Q6_K", "granite-3b-code-instruct-128k.i1-Q6_K", "Qwen2.5-Coder-3B-Instruct-Q8_0", "jan-nano-128k-Q6_K", "Instella-3B-Q8", "Falcon3-3B-Instruct-q8_0", "Yi-6B-200K-Airo-Claude-Puffin-Q4_K_M", "granite-3b-code-base-128k.Q8_0", "LWM-Text-Chat-1M-Q4_K_M", "mistral-7b-v0.1.Q4_K_S", "mistral-7b-instruct-v0.2.Q4_K_S", "granite-4.0-h-tiny-Q4_K_M", "Hermes-2-Pro-Mistral-7B.Q4_K_M", "Mistral-7B-Instruct-v0.3-Q4_K_M", "Falcon-H1-7B-Instruct-Q4_K_M"]
        after_step_succeeds:                            # schema line 323
          - save_to: "./docs/benchmarks/outputs/output/{{step.model_name}}.json"  # schema line 476
          - shell:
              command: "free"
              args: ["-h"]
              fail_on_error: false
          - log:                                        # schema line 275
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms, json_parsable]
        after_step_fails:                               # schema line 328
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]

workflow_id: security_audit_pipeline
name: "Security Audit Pipeline"
description: "Automated security audit with static analysis, dependency scan, and final report"
version: "2.0.0"
author: "Whitt Execution Engine"
tags:
  - security-audit
  - analysis
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
    hosting:
      gpu_layers: 999
models:
  "target-model":
    name: "Qwen2.5-Coder-3B-Instruct-Q8_0"
    host:
      type: llama_cpp_with_vulkan
workflow_execution_strategy:
  memory:
    model_lifecycle:
      unload_unused: true
agentic_workflow:
  steps:
    step_1_read_code:
      generative_entity: "${models.target-model}"
      prompt: |
        Read and display the complete contents of the source file provided.
        Output every line with line numbers prefixed.
        Preserve all whitespace, comments, and formatting exactly.
        If the file is very long, output the first 500 lines.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step1-source-code.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_2_static_analysis:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_code]
      prompt: |
        Perform a thorough static analysis of the following source code.
        Check for these specific issues:
        - Dead code and unreachable branches
        - Unused variables and imports
        - Type mismatches and potential runtime errors
        - Performance bottlenecks (O(n²) loops, unnecessary allocations)
        - Naming convention violations
        SOURCE CODE TO ANALYZE:
        {{step.step_1_read_code.output}}
        Output a numbered list of issues with file location, severity (critical/warning/info),
        and a brief explanation of each issue.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step2-static-analysis.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_3_dependency_scan:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_code]
      prompt: |
        Perform a dependency scan of the following source code.
        Check for known vulnerabilities in package manifests (e.g., npm, Maven).
        Output each vulnerability with: package name, version, severity, and remediation advice.
        SOURCE CODE TO SCAN:
        {{step.step_1_read_code.output}}
        Output each finding with: vulnerability type, location (line number),
        severity (critical/high/medium/low), and remediation advice.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step3-dependency-scan.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_4_code_pattern_analysis:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_code]
      prompt: |
        Analyze the code patterns of the following source code for injection attacks (SQL, XSS, command injection).
        Output each pattern with: type (SQL, XSS, command), location (line number),
        severity (critical/high/medium/low), and remediation advice.
        SOURCE CODE TO ANALYZE:
        {{step.step_1_read_code.output}}
        Output each finding with: vulnerability type, location (line number),
        severity (critical/high/medium/low), and remediation advice.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step4-code-pattern-analysis.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_5_authentication_analysis:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_code]
      prompt: |
        Analyze the authentication and authorization implementations of the following source code.
        Check for common vulnerabilities such as weak password hashing, insecure session management,
        and lack of access control.
        SOURCE CODE TO ANALYZE:
        {{step.step_1_read_code.output}}
        Output each vulnerability with: type (weak password hashing, session management, access control),
        location (line number), severity (critical/high/medium/low), and remediation advice.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step5-authentication-analysis.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_6_crypto_analysis:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_code]
      prompt: |
        Analyze the cryptographic usage of the following source code for weak algorithms.
        Output each instance with: algorithm name, location (line number),
        severity (critical/high/medium/low), and remediation advice.
        SOURCE CODE TO ANALYZE:
        {{step.step_1_read_code.output}}
        Output each finding with: vulnerability type, location (line number),
        severity (critical/high/medium/low), and remediation advice.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step6-crypto-analysis.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_7_compile_report:
      generative_entity: "${models.target-model}"
      depends_on: [step_2_static_analysis, step_3_dependency_scan, step_4_code_pattern_analysis, step_5_authentication_analysis, step_6_crypto_analysis]
      prompt: |
        Compile a comprehensive security audit report from the analysis results below.
        STATIC ANALYSIS RESULTS:
        {{step.step_2_static_analysis.output}}
        DEPENDENCY SCAN RESULTS:
        {{step.step_3_dependency_scan.output}}
        CODE PATTERN ANALYSIS RESULTS:
        {{step.step_4_code_pattern_analysis.output}}
        AUTHENTICATION ANALYSIS RESULTS:
        {{step.step_5_authentication_analysis.output}}
        CRYPTOGRAPHY ANALYSIS RESULTS:
        {{step.step_6_crypto_analysis.output}}
        Create a markdown report with:
        1. Executive summary with total issue count by severity
        2. Critical issues section (must fix before merge)
        3. Warnings section (should fix)
        4. Info section (nice to fix)
        5. Prioritized action items list
        Format each issue as: [SEVERITY] File:Line - Description
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step7-final-report.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]

workflow_id: security_audit_pipeline
name: "Security Audit Pipeline"
description: "Automated security audit with static analysis, dependency scan, and final report"
version: "2.0.0"
author: "Whitt Execution Engine"
tags:
  - security-audit
  - analysis
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
    hosting:
      gpu_layers: 999
models:
  "target-model":
    name: "Qwen2.5-Coder-3B-Instruct-Q8_0"
    host:
      type: llama_cpp_with_vulkan
workflow_execution_strategy:
  memory:
    model_lifecycle:
      unload_unused: true
agentic_workflow:
  steps:
    step_1_read_code:
      generative_entity: "${models.target-model}"
      prompt: |
        Read and display the complete contents of the source file provided.
        Output every line with line numbers prefixed.
        Preserve all whitespace, comments, and formatting exactly.
        If the file is very long, output the first 500 lines.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step1-source-code.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_2_static_analysis:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_code]
      prompt: |
        Perform a thorough static analysis of the following source code.
        Check for these specific issues:
        - Dead code and unreachable branches
        - Unused variables and imports
        - Type mismatches and potential runtime errors
        - Performance bottlenecks (O(n²) loops, unnecessary allocations)
        - Naming convention violations
        SOURCE CODE TO ANALYZE:
        {{step.step_1_read_code.output}}
        Output a numbered list of issues with file location, severity (critical/warning/info),
        and a brief explanation of each issue.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step2-static-analysis.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_3_dependency_scan:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_code]
      prompt: |
        Perform a dependency scan of the following source code.
        Check for known vulnerabilities in package manifests (e.g., npm, Maven).
        Output each vulnerability with: package name, version, severity, and remediation advice.
        SOURCE CODE TO SCAN:
        {{step.step_1_read_code.output}}
        Output each finding with: vulnerability type, location (line number),
        severity (critical/high/medium/low), and remediation advice.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step3-dependency-scan.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_4_code_pattern_analysis:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_code]
      prompt: |
        Analyze the code patterns of the following source code for injection attacks (SQL, XSS, command injection).
        Output each pattern with: type (SQL, XSS, command), location (line number),
        severity (critical/high/medium/low), and remediation advice.
        SOURCE CODE TO ANALYZE:
        {{step.step_1_read_code.output}}
        Output each finding with: vulnerability type, location (line number),
        severity (critical/high/medium/low), and remediation advice.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step4-code-pattern-analysis.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_5_authentication_analysis:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_code]
      prompt: |
        Analyze the authentication and authorization implementations of the following source code.
        Check for common vulnerabilities such as weak password hashing, insecure session management,
        and lack of access control.
        SOURCE CODE TO ANALYZE:
        {{step.step_1_read_code.output}}
        Output each vulnerability with: type (weak password hashing, session management, access control),
        location (line number), severity (critical/high/medium/low), and remediation advice.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step5-authentication-analysis.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_6_crypto_analysis:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_code]
      prompt: |
        Analyze the cryptographic usage of the following source code for weak algorithms.
        Output each instance with: algorithm name, location (line number),
        severity (critical/high/medium/low), and remediation advice.
        SOURCE CODE TO ANALYZE:
        {{step.step_1_read_code.output}}
        Output each finding with: vulnerability type, location (line number),
        severity (critical/high/medium/low), and remediation advice.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step6-crypto-analysis.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_7_compile_report:
      generative_entity: "${models.target-model}"
      depends_on: [step_2_static_analysis, step_3_dependency_scan, step_4_code_pattern_analysis, step_5_authentication_analysis, step_6_crypto_analysis]
      prompt: |
        Compile a comprehensive security audit report from the analysis results below.
        STATIC ANALYSIS RESULTS:
        {{step.step_2_static_analysis.output}}
        DEPENDENCY SCAN RESULTS:
        {{step.step_3_dependency_scan.output}}
        CODE PATTERN ANALYSIS RESULTS:
        {{step.step_4_code_pattern_analysis.output}}
        AUTHENTICATION ANALYSIS RESULTS:
        {{step.step_5_authentication_analysis.output}}
        CRYPTOGRAPHY ANALYSIS RESULTS:
        {{step.step_6_crypto_analysis.output}}
        Create a markdown report with:
        1. Executive summary with total issue count by severity
        2. Critical issues section (must fix before merge)
        3. Warnings section (should fix)
        4. Info section (nice to fix)
        5. Prioritized action items list
        Format each issue as: [SEVERITY] File:Line - Description
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step7-final-report.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]

```yaml
workflow_id: security_audit_pipeline
name: "Security Audit Pipeline"
description: "Automated security audit with static analysis, dependency scan, and final report"
version: "2.0.0"
author: "Whitt Execution Engine"
tags: [security-audit, analysis]
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
    hosting:
      gpu_layers: 999
models:
  "target-model":
    name: "Qwen2.5-Coder-3B-Instruct-Q8_0"
    host:
      type: llama_cpp_with_vulkan
workflow_execution_strategy:
  memory:
    model_lifecycle:
      unload_unused: true
agentic_workflow:
  steps:
    step_1_read_code:
      generative_entity: "${models.target-model}"
      prompt: |
        Read and display the complete contents of the source file provided.
        Output every line with line numbers prefixed.
        Preserve all whitespace, comments, and formatting exactly.
        If the file is very long, output the first 500 lines.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step1-source-code.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_2_static_analysis:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_code]
      prompt: |
        Perform a thorough static analysis of the following source code.
        Check for these specific issues:
        - Dead code and unreachable branches
        - Unused variables and imports
        - Type mismatches and potential runtime errors
        - Performance bottlenecks (O(n²) loops, unnecessary allocations)
        - Naming convention violations
        SOURCE CODE TO ANALYZE:
        {{step.step_1_read_code.output}}
        Output a numbered list of issues with file location, severity (critical/warning/info),
        and a brief explanation of each issue.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step2-static-analysis.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_3_dependency_scan:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_code]
      prompt: |
        Perform a dependency scan of the following source code.
        Check for known vulnerabilities in package manifests (e.g., npm, Maven).
        Output each vulnerability with: package name, version, severity, and remediation advice.
        SOURCE CODE TO SCAN:
        {{step.step_1_read_code.output}}
        Output each finding with: vulnerability type, location (line number),
        severity (critical/high/medium/low), and remediation advice.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step3-dependency-scan.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_4_code_pattern_analysis:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_code]
      prompt: |
        Analyze the code patterns of the following source code for injection attacks (SQL, XSS, command injection).
        Output each pattern with: type (SQL, XSS, command), location (line number),
        severity (critical/high/medium/low), and remediation advice.
        SOURCE CODE TO ANALYZE:
        {{step.step_1_read_code.output}}
        Output each finding with: vulnerability type, location (line number),
        severity (critical/high/medium/low), and remediation advice.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step4-code-pattern-analysis.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_5_authentication_analysis:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_code]
      prompt: |
        Analyze the authentication and authorization implementations of the following source code.
        Check for common vulnerabilities such as weak password hashing, insecure session management,
        and lack of access control.
        SOURCE CODE TO ANALYZE:
        {{step.step_1_read_code.output}}
        Output each vulnerability with: type (weak password hashing, session management, access control),
        location (line number), severity (critical/high/medium/low), and remediation advice.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step5-authentication-analysis.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_6_crypto_analysis:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_code]
      prompt: |
        Analyze the cryptographic usage of the following source code for weak algorithms.
        Output each instance with: algorithm name, location (line number),
        severity (critical/high/medium/low), and remediation advice.
        SOURCE CODE TO ANALYZE:
        {{step.step_1_read_code.output}}
        Output each finding with: vulnerability type, location (line number),
        severity (critical/high/medium/low), and remediation advice.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step6-crypto-analysis.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_7_compile_report:
      generative_entity: "${models.target-model}"
      depends_on: [step_2_static_analysis, step_3_dependency_scan, step_4_code_pattern_analysis, step_5_authentication_analysis, step_6_crypto_analysis]
      prompt: |
        Compile a comprehensive security audit report from the analysis results below.
        STATIC ANALYSIS RESULTS:
        {{step.step_2_static_analysis.output}}
        DEPENDENCY SCAN RESULTS:
        {{step.step_3_dependency_scan.output}}
        CODE PATTERN ANALYSIS RESULTS:
        {{step.step_4_code_pattern_analysis.output}}
        AUTHENTICATION ANALYSIS RESULTS:
        {{step.step_5_authentication_analysis.output}}
        CRYPTOGRAPHY ANALYSIS RESULTS:
        {{step.step_6_crypto_analysis.output}}
        Create a markdown report with:
        1. Executive summary with total issue count by severity
        2. Critical issues section (must fix before merge)
        3. Warnings section (should fix)
        4. Info section (nice to fix)
        5. Prioritized action items list
        Format each issue as: [SEVERITY] File:Line - Description
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step7-final-report.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
```