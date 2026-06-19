<!--
Source Session: ses_17a245a9cffeWo9uVFAyuF6C9I
Message Length: 153049 characters
YAML Sections: 180
Embedded Prompts: 88
Agentic Keywords: 15
Complexity: HIGH
Source Files: attempt-9a-etl-pipeline.yml, attempt-9b-content-moderation.yml, attempt-9c-devops-analysis.yml
-->

workflow_id: meta_workflow_generator_v9a
name: "Meta-Workflow Generator v9a - ETL Pipeline"
description: "3-step meta-workflow with validate step that NEVER renames step references. Uses new task: ETL data pipeline."
version: "9.0.0"
author: "Whitt Execution Engine"
tags: [meta-workflow, yaml-generation, attempt-9a]
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
        TASK: Create a data pipeline ETL workflow that extracts customer order data from multiple sources (CSV files, JSON API responses, database dumps), normalizes and validates each record against a schema (required fields, data types, range checks), transforms dates and currencies to standard formats, deduplicates records by customer_id + order_id, enriches records with geographic data based on zip codes, loads validated records into a structured output, and generates a comprehensive data quality report with statistics (total records, valid/invalid counts, transformation log, deduplication stats).
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
        12. The workflow MUST be about ETL DATA PIPELINE processing, not code review
        13. Template interpolation MUST use the EXACT step name from depends_on. If a step is named step_3_check_deps then use {{step.step_3_check_deps.output}} NOT {{step.step_3_deps_check.output}}
      model_overrides:
        temperature: 0.15
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/v9a-generated-raw.yml"
          - log:
              to_file_path: "./real-workflow-attempts/logs/v9a-generator.log"
              event_fields: [step_name, duration_ms, token_count]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v9a-generator.log"
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
          - save_to: "./real-workflow-attempts/outputs/v9a-cleaned.yml"
          - log:
              to_file_path: "./real-workflow-attempts/logs/v9a-generator.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v9a-generator.log"
              event_fields: [step_name, error_message]
    # ── PHASE 3: VALIDATE + FIX ────────────────────────────────────
    # v9 fix: NEVER rename step references. Use EXACT step names from depends_on.
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
              args: ["./real-workflow-attempts/scripts/validate-yaml.py", "./real-workflow-attempts/outputs/v9a-cleaned.yml"]
              fail_on_error: false
          - log:
              to_file_path: "./real-workflow-attempts/logs/v9a-generator.log"
              event_fields: [step_name, shell_output]
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/v9a-final-workflow.yml"
          - shell:
              command: "python3"
              args: ["./real-workflow-attempts/scripts/validate-yaml.py", "./real-workflow-attempts/outputs/v9a-final-workflow.yml"]
              fail_on_error: false
          - log:
              to_file_path: "./real-workflow-attempts/logs/v9a-generator.log"
              event_fields: [step_name, duration_ms, shell_output]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v9a-generator.log"
              event_fields: [step_name, error_message]

workflow_id: meta_workflow_generator_v9b
name: "Meta-Workflow Generator v9b - Content Moderation"
description: "3-step meta-workflow with validate step that NEVER renames step references. Uses new task: content moderation pipeline."
version: "9.0.0"
author: "Whitt Execution Engine"
tags: [meta-workflow, yaml-generation, attempt-9b]
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
        TASK: Create a content moderation pipeline that processes user-submitted text content through toxicity detection (hate speech, harassment, threats), sentiment analysis (positive/negative/neutral scoring), PII detection (emails, phone numbers, SSNs, credit cards), spam detection (promotional content, repetitive patterns, URL flooding), and generates a moderation decision report with confidence scores, flagged content excerpts, recommended actions (approve/flag/reject), and aggregated platform safety metrics.
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
        12. The workflow MUST be about CONTENT MODERATION, not code review
        13. Template interpolation MUST use the EXACT step name from depends_on. If a step is named step_3_check_deps then use {{step.step_3_check_deps.output}} NOT {{step.step_3_deps_check.output}}
      model_overrides:
        temperature: 0.15
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/v9b-generated-raw.yml"
          - log:
              to_file_path: "./real-workflow-attempts/logs/v9b-generator.log"
              event_fields: [step_name, duration_ms, token_count]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v9b-generator.log"
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
          - save_to: "./real-workflow-attempts/outputs/v9b-cleaned.yml"
          - log:
              to_file_path: "./real-workflow-attempts/logs/v9b-generator.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v9b-generator.log"
              event_fields: [step_name, error_message]
    # ── PHASE 3: VALIDATE + FIX ────────────────────────────────────
    # v9 fix: NEVER rename step references. Use EXACT step names from depends_on.
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
              args: ["./real-workflow-attempts/scripts/validate-yaml.py", "./real-workflow-attempts/outputs/v9b-cleaned.yml"]
              fail_on_error: false
          - log:
              to_file_path: "./real-workflow-attempts/logs/v9b-generator.log"
              event_fields: [step_name, shell_output]
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/v9b-final-workflow.yml"
          - shell:
              command: "python3"
              args: ["./real-workflow-attempts/scripts/validate-yaml.py", "./real-workflow-attempts/outputs/v9b-final-workflow.yml"]
              fail_on_error: false
          - log:
              to_file_path: "./real-workflow-attempts/logs/v9b-generator.log"
              event_fields: [step_name, duration_ms, shell_output]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v9b-generator.log"
              event_fields: [step_name, error_message]

workflow_id: meta_workflow_generator_v9c
name: "Meta-Workflow Generator v9c - DevOps Analysis"
description: "3-step meta-workflow with validate step that NEVER renames step references. Uses new task: DevOps infrastructure analysis."
version: "9.0.0"
author: "Whitt Execution Engine"
tags: [meta-workflow, yaml-generation, attempt-9c]
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
        TASK: Create a DevOps infrastructure analysis workflow that reads infrastructure-as-code files (Terraform, Docker Compose, Kubernetes manifests), analyzes resource configurations for cost optimization (right-sizing instances, unused resources, spot instance opportunities), checks security best practices (network policies, secret management, least privilege), evaluates reliability patterns (health checks, circuit breakers, retry policies), reviews scalability configurations (auto-scaling, load balancing, cache strategies), and produces an infrastructure assessment report with risk scores, cost savings estimates, and prioritized remediation recommendations.
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
        12. The workflow MUST be about DEVOPS INFRASTRUCTURE ANALYSIS, not code review
        13. Template interpolation MUST use the EXACT step name from depends_on. If a step is named step_3_check_deps then use {{step.step_3_check_deps.output}} NOT {{step.step_3_deps_check.output}}
      model_overrides:
        temperature: 0.15
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/v9c-generated-raw.yml"
          - log:
              to_file_path: "./real-workflow-attempts/logs/v9c-generator.log"
              event_fields: [step_name, duration_ms, token_count]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v9c-generator.log"
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
          - save_to: "./real-workflow-attempts/outputs/v9c-cleaned.yml"
          - log:
              to_file_path: "./real-workflow-attempts/logs/v9c-generator.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v9c-generator.log"
              event_fields: [step_name, error_message]
    # ── PHASE 3: VALIDATE + FIX ────────────────────────────────────
    # v9 fix: NEVER rename step references. Use EXACT step names from depends_on.
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
              args: ["./real-workflow-attempts/scripts/validate-yaml.py", "./real-workflow-attempts/outputs/v9c-cleaned.yml"]
              fail_on_error: false
          - log:
              to_file_path: "./real-workflow-attempts/logs/v9c-generator.log"
              event_fields: [step_name, shell_output]
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/v9c-final-workflow.yml"
          - shell:
              command: "python3"
              args: ["./real-workflow-attempts/scripts/validate-yaml.py", "./real-workflow-attempts/outputs/v9c-final-workflow.yml"]
              fail_on_error: false
          - log:
              to_file_path: "./real-workflow-attempts/logs/v9c-generator.log"
              event_fields: [step_name, duration_ms, shell_output]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v9c-generator.log"
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

workflow_id: data_pipeline_etl
name: "Data Pipeline ETL Workflow"
description: "Automated ETL workflow for extracting, normalizing, validating, transforming, deduplicating, enriching, and loading customer order data."
version: "2.0.0"
author: "Your Name"
tags:
  - data-pipeline
  - etl
  - normalization
  - validation
  - transformation
  - deduplication
  - enrichment
  - loading
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
    step_1_extract_data:
      generative_entity: "${models.target-model}"
      prompt: |
        Extract customer order data from multiple sources (CSV files, JSON API responses, database dumps).
        Output the extracted data in a structured format.
        Ensure that all fields are included and correctly formatted.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step1-extracted-data.json"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_2_normalize_data:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_extract_data]
      prompt: |
        Normalize the extracted customer order data.
        Ensure that all fields are consistent and correctly formatted.
        Handle missing values appropriately.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step2-normalized-data.json"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_3_validate_data:
      generative_entity: "${models.target-model}"
      depends_on: [step_2_normalize_data]
      prompt: |
        Validate the normalized customer order data against a schema.
        Ensure that all required fields are present and correctly formatted.
        Perform range checks for numerical fields.
        Handle any validation errors appropriately.
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step3-validated-data.json"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_4_transform_data:
      generative_entity: "${models.target-model}"
      depends_on: [step_3_validate_data]
      prompt: |
        Transform the validated customer order data.
        Convert dates and currencies to standard formats.
        Handle any transformation errors appropriately.
      model_overrides:
        temperature: 0.5
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step4-transformed-data.json"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_5_deduplicate_data:
      generative_entity: "${models.target-model}"
      depends_on: [step_4_transform_data]
      prompt: |
        Deduplicate the transformed customer order data by customer_id + order_id.
        Ensure that each record is unique and consistent.
        Handle any deduplication errors appropriately.
      model_overrides:
        temperature: 0.6
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step5-deduplicated-data.json"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_6_enrich_data:
      generative_entity: "${models.target-model}"
      depends_on: [step_5_deduplicate_data]
      prompt: |
        Enrich the deduplicated customer order data with geographic data based on zip codes.
        Ensure that all enriched fields are correctly formatted and consistent.
        Handle any enrichment errors appropriately.
      model_overrides:
        temperature: 0.7
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step6-enriched-data.json"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_7_load_data:
      generative_entity: "${models.target-model}"
      depends_on: [step_6_enrich_data]
      prompt: |
        Load the enriched customer order data into a structured output.
        Ensure that all records are correctly formatted and consistent.
        Handle any loading errors appropriately.
      model_overrides:
        temperature: 0.8
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step7-loaded-data.json"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_8_generate_report:
      generative_entity: "${models.target-model}"
      depends_on: [step_7_load_data]
      prompt: |
        Generate a comprehensive data quality report with statistics.
        Include the following information:
        - Total records
        - Valid/invalid counts
        - Transformation log (include any errors or warnings)
        - Deduplication stats (include duplicates removed and count)
        DATA QUALITY REPORT:
        {{step.step_7_load_data.output}}
        Create a markdown report with:
        1. Executive summary with total issue count by severity
        2. Critical issues section (must fix before merge)
        3. Warnings section (should fix)
        4. Info section (nice to fix)
        5. Prioritized action items list
        Format each issue as: [SEVERITY] File:Line - Description
      model_overrides:
        temperature: 0.9
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step8-final-report.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]

workflow_id: data_pipeline_etl
name: "Data Pipeline ETL Workflow"
description: "Automated ETL workflow for extracting, normalizing, validating, transforming, deduplicating, enriching, and loading customer order data."
version: "2.0.0"
author: "Your Name"
tags:
  - data-pipeline
  - etl
  - normalization
  - validation
  - transformation
  - deduplication
  - enrichment
  - loading
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
    step_1_extract_data:
      generative_entity: "${models.target-model}"
      prompt: |
        Extract customer order data from multiple sources (CSV files, JSON API responses, database dumps).
        Output the extracted data in a structured format.
        Ensure that all fields are included and correctly formatted.
        Include timestamps for each record extraction.
        Handle edge cases such as missing or malformed source data gracefully.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step1-extracted-data.json"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_2_normalize_data:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_extract_data]
      prompt: |
        Normalize the extracted customer order data.
        Ensure that all fields are consistent and correctly formatted.
        Handle missing values appropriately.
        INPUT FROM PREVIOUS STEP:
        {{step.step_1_extract_data.output}}
        Convert all date fields to ISO 8601 format.
        Standardize currency fields to USD with 2 decimal places.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step2-normalized-data.json"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_3_validate_data:
      generative_entity: "${models.target-model}"
      depends_on: [step_2_normalize_data]
      prompt: |
        Validate the normalized customer order data against a schema.
        Ensure that all required fields are present and correctly formatted.
        Perform range checks for numerical fields.
        Handle any validation errors appropriately.
        INPUT FROM PREVIOUS STEP:
        {{step.step_2_normalize_data.output}}
        Verify that all required fields (customer_id, order_id, amount, date) exist in each record.
        Validate that amounts are positive and within a reasonable range (0 to 100000).
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step3-validated-data.json"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_4_transform_data:
      generative_entity: "${models.target-model}"
      depends_on: [step_3_validate_data]
      prompt: |
        Transform the validated customer order data.
        Convert dates and currencies to standard formats.
        Handle any transformation errors appropriately.
        INPUT FROM PREVIOUS STEP:
        {{step.step_3_validate_data.output}}
        Format all date fields as YYYY-MM-DD.
        Convert currency amounts from local units to USD using a fixed exchange rate of 1.0.
      model_overrides:
        temperature: 0.5
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step4-transformed-data.json"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_5_deduplicate_data:
      generative_entity: "${models.target-model}"
      depends_on: [step_4_transform_data]
      prompt: |
        Deduplicate the transformed customer order data by customer_id + order_id.
        Ensure that each record is unique and consistent.
        Handle any deduplication errors appropriately.
        INPUT FROM PREVIOUS STEP:
        {{step.step_4_transformed_data.output}}
        Remove duplicate records where both customer_id and order_id match exactly.
        Log the number of duplicates removed in a metadata field.
      model_overrides:
        temperature: 0.6
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step5-deduplicated-data.json"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_6_enrich_data:
      generative_entity: "${models.target-model}"
      depends_on: [step_5_deduplicate_data]
      prompt: |
        Enrich the deduplicated customer order data with geographic data based on zip codes.
        Ensure that all enriched fields are correctly formatted and consistent.
        Handle any enrichment errors appropriately.
        INPUT FROM PREVIOUS STEP:
        {{step.step_5_deduplicate_data.output}}
        For each record, enrich with city, state, country from the zip code using a public geolocation API.
        If no location is found, set all geographic fields to "Unknown".
      model_overrides:
        temperature: 0.7
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step6-enriched-data.json"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_7_load_data:
      generative_entity: "${models.target-model}"
      depends_on: [step_6_enrich_data]
      prompt: |
        Load the enriched customer order data into a structured output.
        Ensure that all records are correctly formatted and consistent.
        Handle any loading errors appropriately.
        INPUT FROM PREVIOUS STEP:
        {{step.step_6_enriched_data.output}}
        Format the final dataset as JSON with proper schema validation.
        Store in a file with timestamped filename to ensure versioning.
      model_overrides:
        temperature: 0.8
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step7-loaded-data.json"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_8_generate_report:
      generative_entity: "${models.target-model}"
      depends_on: [step_7_load_data]
      prompt: |
        Generate a comprehensive data quality report with statistics.
        Include the following information:
        - Total records
        - Valid/invalid counts
        - Transformation log (include any errors or warnings)
        - Deduplication stats (include duplicates removed and count)
        DATA QUALITY REPORT:
        {{step.step_7_load_data.output}}
        Create a markdown report with:
        1. Executive summary with total issue count by severity
        2. Critical issues section (must fix before merge)
        3. Warnings section (should fix)
        4. Info section (nice to fix)
        5. Prioritized action items list
        Format each issue as: [SEVERITY] File:Line - Description
      model_overrides:
        temperature: 0.9
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step8-final-report.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]

```yaml
workflow_id: data_pipeline_etl
name: "Data Pipeline ETL Workflow"
description: "Automated ETL workflow for extracting, normalizing, validating, transforming, deduplicating, enriching, and loading customer order data."
version: "2.0.0"
author: "Your Name"
tags: [data-pipeline, etl, normalization, validation, transformation, deduplication, enrichment, loading]
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
    step_1_extract_data:
      generative_entity: "${models.target-model}"
      prompt: |
        Extract customer order data from multiple sources (CSV files, JSON API responses, database dumps).
        Output the extracted data in a structured format.
        Ensure that all fields are included and correctly formatted.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step1-extracted-data.json"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_2_normalize_data:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_extract_data]
      prompt: |
        Normalize the extracted customer order data.
        Ensure that all fields are consistent and correctly formatted.
        Handle missing values appropriately.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step2-normalized-data.json"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_3_validate_data:
      generative_entity: "${models.target-model}"
      depends_on: [step_2_normalize_data]
      prompt: |
        Validate the normalized customer order data against a schema.
        Ensure that all required fields are present and correctly formatted.
        Perform range checks for numerical fields.
        Handle any validation errors appropriately.
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step3-validated-data.json"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_4_transform_data:
      generative_entity: "${models.target-model}"
      depends_on: [step_3_validate_data]
      prompt: |
        Transform the validated customer order data.
        Convert dates and currencies to standard formats.
        Handle any transformation errors appropriately.
      model_overrides:
        temperature: 0.5
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step4-transformed-data.json"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_5_deduplicate_data:
      generative_entity: "${models.target-model}"
      depends_on: [step_4_transform_data]
      prompt: |
        Deduplicate the transformed customer order data by customer_id + order_id.
        Ensure that each record is unique and consistent.
        Handle any deduplication errors appropriately.
      model_overrides:
        temperature: 0.6
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step5-deduplicated-data.json"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_6_enrich_data:
      generative_entity: "${models.target-model}"
      depends_on: [step_5_deduplicate_data]
      prompt: |
        Enrich the deduplicated customer order data with geographic data based on zip codes.
        Ensure that all enriched fields are correctly formatted and consistent.
        Handle any enrichment errors appropriately.
      model_overrides:
        temperature: 0.7
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step6-enriched-data.json"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_7_load_data:
      generative_entity: "${models.target-model}"
      depends_on: [step_6_enrich_data]
      prompt: |
        Load the enriched customer order data into a structured output.
        Ensure that all records are correctly formatted and consistent.
        Handle any loading errors appropriately.
      model_overrides:
        temperature: 0.8
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step7-loaded-data.json"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_8_generate_report:
      generative_entity: "${models.target-model}"
      depends_on: [step_7_load_data]
      prompt: |
        Generate a comprehensive data quality report with statistics.
        Include the following information:
        - Total records
        - Valid/invalid counts
        - Transformation log (include any errors or warnings)
        - Deduplication stats (include duplicates removed and count)
        DATA QUALITY REPORT:
        {{step.step_7_load_data.output}}
        Create a markdown report with:
        1. Executive summary with total issue count by severity
        2. Critical issues section (must fix before merge)
        3. Warnings section (should fix)
        4. Info section (nice to fix)
        5. Prioritized action items list
        Format each issue as: [SEVERITY] File:Line - Description
      model_overrides:
        temperature: 0.9
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step8-final-report.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
```

workflow_id: content_moderation_pipeline
name: "Content Moderation Pipeline"
description: "Automated content moderation pipeline with toxicity detection, sentiment analysis, PII detection, spam detection, and report generation"
version: "2.0.0"
author: "Whitt Execution Engine"
tags:
  - content-moderation
  - toxicity-detection
  - sentiment-analysis
  - pii-detection
  - spam-detection
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
    hosting:
      gpu_layers: 999
models:
  toxicity_model:
    name: "Qwen2.5-Content-Moderation-Toxicity-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  sentiment_model:
    name: "Qwen2.5-Content-Moderation-Sentiment-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  pii_detection_model:
    name: "Qwen2.5-Content-Moderation-PII-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  spam_detection_model:
    name: "Qwen2.5-Content-Moderation-Spam-Q8_0"
    host:
      type: llama_cpp_with_vulkan
workflow_execution_strategy:
  memory:
    model_lifecycle:
      unload_unused: true
agentic_workflow:
  steps:
    step_1_read_content:
      generative_entity: "${models.toxicity_model}"
      prompt: |
        Read and display the complete contents of the user-submitted text content.
        Output every line with line numbers prefixed.
        Preserve all whitespace, punctuation, and formatting exactly.
        If the content is very long, output the first 500 lines.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step1-content.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_2_toxicity_detection:
      generative_entity: "${models.toxicity_model}"
      depends_on: [step_1_read_content]
      prompt: |
        Perform a toxicity detection on the following content.
        Classify each line as toxic (hate speech, harassment, threats) or non-toxic.
        CONTENT TO ANALYZE:
        {{step.step_1_read_content.output}}
        Output a numbered list of lines with their toxicity classification (toxic/non-toxic).
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step2-toxicity-detection.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_3_sentiment_analysis:
      generative_entity: "${models.sentiment_model}"
      depends_on: [step_1_read_content]
      prompt: |
        Perform a sentiment analysis on the following content.
        Classify each line as positive, negative, or neutral.
        CONTENT TO ANALYZE:
        {{step.step_1_read_content.output}}
        Output a numbered list of lines with their sentiment classification (positive/negative/neutral).
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step3-sentiment-analysis.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_4_pii_detection:
      generative_entity: "${models.pii_detection_model}"
      depends_on: [step_1_read_content]
      prompt: |
        Perform a PII detection on the following content.
        Identify and extract any personally identifiable information (PII) such as emails, phone numbers, SSNs, credit cards.
        CONTENT TO ANALYZE:
        {{step.step_1_read_content.output}}
        Output a numbered list of lines with detected PII elements (email/phone number/SSN/Credit Card).
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step4-pii-detection.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_5_spam_detection:
      generative_entity: "${models.spam_detection_model}"
      depends_on: [step_1_read_content]
      prompt: |
        Perform a spam detection on the following content.
        Identify and flag any promotional content, repetitive patterns, or URL flooding.
        CONTENT TO ANALYZE:
        {{step.step_1_read_content.output}}
        Output a numbered list of lines with detected spam elements (promotional/content/repetitive/patterns/URL flooding).
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step5-spam-detection.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_6_moderation_decision_report:
      generative_entity: "${models.toxicity_model}"
      depends_on: [step_2_toxicity_detection, step_3_sentiment_analysis, step_4_pii_detection, step_5_spam_detection]
      prompt: |
        Generate a comprehensive moderation decision report based on the analysis results below.
        TOXICITY DETECTION RESULTS:
        {{step.step_2_toxicity_detection.output}}
        SENTIMENT ANALYSIS RESULTS:
        {{step.step_3_sentiment_analysis.output}}
        PII DETECTION RESULTS:
        {{step.step_4_pii_detection.output}}
        SPAM DETECTION RESULTS:
        {{step.step_5_spam_detection.output}}
        Create a markdown report with:
        1. Executive summary with total flagged content count by toxicity type
        2. Toxicity sections (toxic/non-toxic) with flagged lines, confidence scores, excerpts, and recommended actions (approve/flag/reject)
        3. Sentiment sections (positive/negative/neutral) with flagged lines, confidence scores, excerpts, and recommended actions (approve/flag/reject)
        4. PII sections (email/phone number/SSN/Credit Card) with flagged elements, confidence scores, excerpts, and recommended actions (approve/flag/reject)
        5. Spam sections (promotional/content/repetitive/patterns/URL flooding) with flagged lines, confidence scores, excerpts, and recommended actions (approve/flag/reject)
        Format each section as: [TOXICITY/SENTIMENT/PII/SPAM] - Line Number - Confidence Score - Excerpt - Recommended Action
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step6-moderation-report.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_7_aggregated_platform_safety_metrics:
      generative_entity: "${models.toxicity_model}"
      depends_on: [step_2_toxicity_detection, step_3_sentiment_analysis, step_4_pii_detection, step_5_spam_detection]
      prompt: |
        Generate aggregated platform safety metrics based on the analysis results below.
        TOXICITY DETECTION RESULTS:
        {{step.step_2_toxicity_detection.output}}
        SENTIMENT ANALYSIS RESULTS:
        {{step.step_3_sentiment_analysis.output}}
        PII DETECTION RESULTS:
        {{step.step_4_pii_detection.output}}
        SPAM DETECTION RESULTS:
        {{step.step_5_spam_detection.output}}
        Calculate and output the following metrics:
        1. Total flagged content count by toxicity type
        2. Average confidence score for each toxicity type
        3. Percentage of flagged content that is toxic
        4. Number of unique PII elements detected
        5. Number of URLs flagged as spam
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step7-platform-safety-metrics.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]

workflow_id: content_moderation_pipeline
name: "Content Moderation Pipeline"
description: "Automated content moderation pipeline with toxicity detection, sentiment analysis, PII detection, spam detection, and report generation"
version: "2.0.0"
author: "Whitt Execution Engine"
tags:
  - content-moderation
  - toxicity-detection
  - sentiment-analysis
  - pii-detection
  - spam-detection
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
    hosting:
      gpu_layers: 999
models:
  toxicity_model:
    name: "Qwen2.5-Content-Moderation-Toxicity-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  sentiment_model:
    name: "Qwen2.5-Content-Moderation-Sentiment-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  pii_detection_model:
    name: "Qwen2.5-Content-Moderation-PII-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  spam_detection_model:
    name: "Qwen2.5-Content-Moderation-Spam-Q8_0"
    host:
      type: llama_cpp_with_vulkan
workflow_execution_strategy:
  memory:
    model_lifecycle:
      unload_unused: true
agentic_workflow:
  steps:
    step_1_read_content:
      generative_entity: "${models.toxicity_model}"
      prompt: |
        Read and display the complete contents of the user-submitted text content.
        Output every line with line numbers prefixed.
        Preserve all whitespace, punctuation, and formatting exactly.
        If the content is very long, output the first 500 lines.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step1-content.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_2_toxicity_detection:
      generative_entity: "${models.toxicity_model}"
      depends_on: [step_1_read_content]
      prompt: |
        Perform a toxicity detection on the following content.
        Classify each line as toxic (hate speech, harassment, threats) or non-toxic.
        CONTENT TO ANALYZE:
        {{step.step_1_read_content.output}}
        Output a numbered list of lines with their toxicity classification (toxic/non-toxic).
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step2-toxicity-detection.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_3_sentiment_analysis:
      generative_entity: "${models.sentiment_model}"
      depends_on: [step_1_read_content]
      prompt: |
        Perform a sentiment analysis on the following content.
        Classify each line as positive, negative, or neutral.
        CONTENT TO ANALYZE:
        {{step.step_1_read_content.output}}
        Output a numbered list of lines with their sentiment classification (positive/negative/neutral).
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step3-sentiment-analysis.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_4_pii_detection:
      generative_entity: "${models.pii_detection_model}"
      depends_on: [step_1_read_content]
      prompt: |
        Perform a PII detection on the following content.
        Identify and extract any personally identifiable information (PII) such as emails, phone numbers, SSNs, credit cards.
        CONTENT TO ANALYZE:
        {{step.step_1_read_content.output}}
        Output a numbered list of lines with detected PII elements (email/phone number/SSN/Credit Card).
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step4-pii-detection.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_5_spam_detection:
      generative_entity: "${models.spam_detection_model}"
      depends_on: [step_1_read_content]
      prompt: |
        Perform a spam detection on the following content.
        Identify and flag any promotional content, repetitive patterns, or URL flooding.
        CONTENT TO ANALYZE:
        {{step.step_1_read_content.output}}
        Output a numbered list of lines with detected spam elements (promotional/content/repetitive/patterns/URL flooding).
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step5-spam-detection.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_6_moderation_decision_report:
      generative_entity: "${models.toxicity_model}"
      depends_on: [step_2_toxicity_detection, step_3_sentiment_analysis, step_4_pii_detection, step_5_spam_detection]
      prompt: |
        Generate a comprehensive moderation decision report based on the analysis results below.
        TOXICITY DETECTION RESULTS:
        {{step.step_2_toxicity_detection.output}}
        SENTIMENT ANALYSIS RESULTS:
        {{step.step_3_sentiment_analysis.output}}
        PII DETECTION RESULTS:
        {{step.step_4_pii_detection.output}}
        SPAM DETECTION RESULTS:
        {{step.step_5_spam_detection.output}}
        Create a markdown report with:
        1. Executive summary with total flagged content count by toxicity type
        2. Toxicity sections (toxic/non-toxic) with flagged lines, confidence scores, excerpts, and recommended actions (approve/flag/reject)
        3. Sentiment sections (positive/negative/neutral) with flagged lines, confidence scores, excerpts, and recommended actions (approve/flag/reject)
        4. PII sections (email/phone number/SSN/Credit Card) with flagged elements, confidence scores, excerpts, and recommended actions (approve/flag/reject)
        5. Spam sections (promotional/content/repetitive/patterns/URL flooding) with flagged lines, confidence scores, excerpts, and recommended actions (approve/flag/reject)
        Format each section as: [TOXICITY/SENTIMENT/PII/SPAM] - Line Number - Confidence Score - Excerpt - Recommended Action
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step6-moderation-report.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_7_aggregated_platform_safety_metrics:
      generative_entity: "${models.toxicity_model}"
      depends_on: [step_2_toxicity_detection, step_3_sentiment_analysis, step_4_pii_detection, step_5_spam_detection]
      prompt: |
        Generate aggregated platform safety metrics based on the analysis results below.
        TOXICITY DETECTION RESULTS:
        {{step.step_2_toxicity_detection.output}}
        SENTIMENT ANALYSIS RESULTS:
        {{step.step_3_sentiment_analysis.output}}
        PII DETECTION RESULTS:
        {{step.step_4_pii_detection.output}}
        SPAM DETECTION RESULTS:
        {{step.step_5_spam_detection.output}}
        Calculate and output the following metrics:
        1. Total flagged content count by toxicity type
        2. Average confidence score for each toxicity type
        3. Percentage of flagged content that is toxic
        4. Number of unique PII elements detected
        5. Number of URLs flagged as spam
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step7-platform-safety-metrics.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]

```yaml
workflow_id: content_moderation_pipeline
name: "Content Moderation Pipeline"
description: "Automated content moderation pipeline with toxicity detection, sentiment analysis, PII detection, spam detection, and report generation"
version: "2.0.0"
author: "Whitt Execution Engine"
tags: [content-moderation, toxicity-detection, sentiment-analysis, pii-detection, spam-detection]
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
    hosting:
      gpu_layers: 999
models:
  "toxicity_model":
    name: "Qwen2.5-Content-Moderation-Toxicity-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "sentiment_model":
    name: "Qwen2.5-Content-Moderation-Sentiment-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "pii_detection_model":
    name: "Qwen2.5-Content-Moderation-PII-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "spam_detection_model":
    name: "Qwen2.5-Content-Moderation-Spam-Q8_0"
    host:
      type: llama_cpp_with_vulkan
workflow_execution_strategy:
  memory:
    model_lifecycle:
      unload_unused: true
agentic_workflow:
  steps:
    step_1_read_content:
      generative_entity: "${models.toxicity_model}"
      prompt: |
        Read and display the complete contents of the user-submitted text content.
        Output every line with line numbers prefixed.
        Preserve all whitespace, punctuation, and formatting exactly.
        If the content is very long, output the first 500 lines.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step1-content.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_2_toxicity_detection:
      generative_entity: "${models.toxicity_model}"
      depends_on: [step_1_read_content]
      prompt: |
        Perform a toxicity detection on the following content.
        Classify each line as toxic (hate speech, harassment, threats) or non-toxic.
        CONTENT TO ANALYZE:
        {{step.step_1_read_content.output}}
        Output a numbered list of lines with their toxicity classification (toxic/non-toxic).
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step2-toxicity-detection.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_3_sentiment_analysis:
      generative_entity: "${models.sentiment_model}"
      depends_on: [step_1_read_content]
      prompt: |
        Perform a sentiment analysis on the following content.
        Classify each line as positive, negative, or neutral.
        CONTENT TO ANALYZE:
        {{step.step_1_read_content.output}}
        Output a numbered list of lines with their sentiment classification (positive/negative/neutral).
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step3-sentiment-analysis.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_4_pii_detection:
      generative_entity: "${models.pii_detection_model}"
      depends_on: [step_1_read_content]
      prompt: |
        Perform a PII detection on the following content.
        Identify and extract any personally identifiable information (PII) such as emails, phone numbers, SSNs, credit cards.
        CONTENT TO ANALYZE:
        {{step.step_1_read_content.output}}
        Output a numbered list of lines with detected PII elements (email/phone number/SSN/Credit Card).
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step4-pii-detection.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_5_spam_detection:
      generative_entity: "${models.spam_detection_model}"
      depends_on: [step_1_read_content]
      prompt: |
        Perform a spam detection on the following content.
        Identify and flag any promotional content, repetitive patterns, or URL flooding.
        CONTENT TO ANALYZE:
        {{step.step_1_read_content.output}}
        Output a numbered list of lines with detected spam elements (promotional/content/repetitive/patterns/URL flooding).
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step5-spam-detection.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_6_moderation_decision_report:
      generative_entity: "${models.toxicity_model}"
      depends_on: [step_2_toxicity_detection, step_3_sentiment_analysis, step_4_pii_detection, step_5_spam_detection]
      prompt: |
        Generate a comprehensive moderation decision report based on the analysis results below.
        TOXICITY DETECTION RESULTS:
        {{step.step_2_toxicity_detection.output}}
        SENTIMENT ANALYSIS RESULTS:
        {{step.step_3_sentiment_analysis.output}}
        PII DETECTION RESULTS:
        {{step.step_4_pii_detection.output}}
        SPAM DETECTION RESULTS:
        {{step.step_5_spam_detection.output}}
        Create a markdown report with:
        1. Executive summary with total flagged content count by toxicity type
        2. Toxicity sections (toxic/non-toxic) with flagged lines, confidence scores, excerpts, and recommended actions (approve/flag/reject)
        3. Sentiment sections (positive/negative/neutral) with flagged lines, confidence scores, excerpts, and recommended actions (approve/flag/reject)
        4. PII sections (email/phone number/SSN/Credit Card) with flagged elements, confidence scores, excerpts, and recommended actions (approve/flag/reject)
        5. Spam sections (promotional/content/repetitive/patterns/URL flooding) with flagged lines, confidence scores, excerpts, and recommended actions (approve/flag/reject)
        Format each section as: [TOXICITY/SENTIMENT/PII/SPAM] - Line Number - Confidence Score - Excerpt - Recommended Action
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step6-moderation-report.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_7_aggregated_platform_safety_metrics:
      generative_entity: "${models.toxicity_model}"
      depends_on: [step_2_toxicity_detection, step_3_sentiment_analysis, step_4_pii_detection, step_5_spam_detection]
      prompt: |
        Generate aggregated platform safety metrics based on the analysis results below.
        TOXICITY DETECTION RESULTS:
        {{step.step_2_toxicity_detection.output}}
        SENTIMENT ANALYSIS RESULTS:
        {{step.step_3_sentiment_analysis.output}}
        PII DETECTION RESULTS:
        {{step.step_4_pii_detection.output}}
        SPAM DETECTION RESULTS:
        {{step.step_5_spam_detection.output}}
        Calculate and output the following metrics:
        1. Total flagged content count by toxicity type
        2. Average confidence score for each toxicity type
        3. Percentage of flagged content that is toxic
        4. Number of unique PII elements detected
        5. Number of URLs flagged as spam
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step7-platform-safety-metrics.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
```

workflow_id: infrastructure_analysis_pipeline
name: "Infrastructure Analysis Pipeline"
description: "Automated analysis of DevOps infrastructure configurations for cost optimization, security best practices, reliability patterns, and scalability."
version: "2.0.0"
author: "Whitt Execution Engine"
tags:
  - infrastructure-analysis
  - security
  - cost-optimization
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
    step_1_read_infrastructure_files:
      generative_entity: "${models.target-model}"
      prompt: |
        Read and display the complete contents of all infrastructure-as-code files provided.
        Output every line with line numbers prefixed.
        Preserve all whitespace, comments, and formatting exactly.
        If any file is very long, output the first 500 lines for each file.
        INFRASTRUCTURE AS-CODE FILES TO READ:
        - Terraform configuration files (e.g., *.tf)
        - Docker Compose files (e.g., docker-compose.yml)
        - Kubernetes manifests (e.g., *.yaml)
        Output a numbered list of files with line numbers prefixed.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step1-infrastructure-files.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_2_analyze_cost_optimization:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_infrastructure_files]
      prompt: |
        Analyze the cost optimization potential of the infrastructure-as-code files provided.
        Check for these specific issues:
        - Right-sizing instances to match workload demands
        - Identifying and removing unused resources
        - Evaluating spot instance opportunities
        INFRASTRUCTURE AS-CODE FILES TO ANALYZE:
        {{step.step_1_read_infrastructure_files.output}}
        Output a numbered list of cost optimization recommendations with file location, resource type,
        current usage, recommended size or type, and estimated savings.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step2-cost-optimization.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_3_check_security_best_practices:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_infrastructure_files]
      prompt: |
        Check the infrastructure-as-code files for security best practices.
        Look for these specific issues:
        - Network policies to restrict access
        - Secret management and least privilege principles
        - Encryption of sensitive data
        INFRASTRUCTURE AS-CODE FILES TO CHECK:
        {{step.step_1_read_infrastructure_files.output}}
        Output a numbered list of security best practice recommendations with file location,
        specific issues, remediation steps, and risk scores.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step3-security-best-practices.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_4_evaluate_reliability_patterns:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_infrastructure_files]
      prompt: |
        Evaluate the reliability patterns in the infrastructure-as-code files.
        Check for these specific issues:
        - Health checks to monitor system health
        - Circuit breakers and retry policies to handle failures
        - Load balancing strategies to distribute traffic
        INFRASTRUCTURE AS-CODE FILES TO EVALUATE:
        {{step.step_1_read_infrastructure_files.output}}
        Output a numbered list of reliability pattern recommendations with file location,
        specific issues, remediation steps, and expected benefits.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step4-reliability-patterns.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_5_review_scalability_configurations:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_infrastructure_files]
      prompt: |
        Review the scalability configurations in the infrastructure-as-code files.
        Check for these specific issues:
        - Auto-scaling policies to adjust resources based on demand
        - Load balancing strategies to distribute traffic efficiently
        - Cache strategies to improve performance
        INFRASTRUCTURE AS-CODE FILES TO REVIEW:
        {{step.step_1_read_infrastructure_files.output}}
        Output a numbered list of scalability configuration recommendations with file location,
        specific issues, remediation steps, and expected benefits.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step5-scalability-configurations.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_6_compile_infrastructure_assessment_report:
      generative_entity: "${models.target-model}"
      depends_on: [step_2_analyze_cost_optimization, step_3_check_security_best_practices, step_4_evaluate_reliability_patterns, step_5_review_scalability_configurations]
      prompt: |
        Compile a comprehensive infrastructure assessment report from the analysis results below.
        COST OPTIMIZATION RECOMMENDATIONS:
        {{step.step_2_analyze_cost_optimization.output}}
        SECURITY BEST PRACTICE RECOMMENDATIONS:
        {{step.step_3_check_security_best_practices.output}}
        RELIABILITY PATTERNS RECOMMENDATIONS:
        {{step.step_4_evaluate_reliability_patterns.output}}
        SCALABILITY CONFIGURATION RECOMMENDATIONS:
        {{step.step_5_review_scalability_configurations.output}}
        Create a markdown report with:
        1. Executive summary with total recommendation count by category
        2. Cost optimization section (must implement before deployment)
        3. Security best practices section (must follow)
        4. Reliability patterns section (should improve)
        5. Scalability configurations section (nice to enhance)
        6. Prioritized action items list
        Format each recommendation as: [CATEGORY] File:Line - Description
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step6-final-report.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]

workflow_id: infrastructure_analysis_pipeline
name: "Infrastructure Analysis Pipeline"
description: "Automated analysis of DevOps infrastructure configurations for cost optimization, security best practices, reliability patterns, and scalability."
version: "2.0.0"
author: "Whitt Execution Engine"
tags:
  - infrastructure-analysis
  - security
  - cost-optimization
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
    step_1_read_infrastructure_files:
      generative_entity: "${models.target-model}"
      prompt: |
        Read and display the complete contents of all infrastructure-as-code files provided.
        Output every line with line numbers prefixed.
        Preserve all whitespace, comments, and formatting exactly.
        If any file is very long, output the first 500 lines for each file.
        INFRASTRUCTURE AS-CODE FILES TO READ:
        - Terraform configuration files (e.g., *.tf)
        - Docker Compose files (e.g., docker-compose.yml)
        - Kubernetes manifests (e.g., *.yaml)
        Output a numbered list of files with line numbers prefixed.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step1-infrastructure-files.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_2_analyze_cost_optimization:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_infrastructure_files]
      prompt: |
        Analyze the cost optimization potential of the infrastructure-as-code files provided.
        Check for these specific issues:
        - Right-sizing instances to match workload demands
        - Identifying and removing unused resources
        - Evaluating spot instance opportunities
        INPUT FROM PREVIOUS STEP:
        {{step.step_1_read_infrastructure_files.output}}
        INFRASTRUCTURE AS-CODE FILES TO ANALYZE:
        {{step.step_1_read_infrastructure_files.output}}
        Output a numbered list of cost optimization recommendations with file location, resource type,
        current usage, recommended size or type, and estimated savings.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step2-cost-optimization.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_3_check_security_best_practices:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_infrastructure_files]
      prompt: |
        Check the infrastructure-as-code files for security best practices.
        Look for these specific issues:
        - Network policies to restrict access
        - Secret management and least privilege principles
        - Encryption of sensitive data
        INPUT FROM PREVIOUS STEP:
        {{step.step_1_read_infrastructure_files.output}}
        INFRASTRUCTURE AS-CODE FILES TO CHECK:
        {{step.step_1_read_infrastructure_files.output}}
        Output a numbered list of security best practice recommendations with file location,
        specific issues, remediation steps, and risk scores.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step3-security-best-practices.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_4_evaluate_reliability_patterns:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_infrastructure_files]
      prompt: |
        Evaluate the reliability patterns in the infrastructure-as-code files.
        Check for these specific issues:
        - Health checks to monitor system health
        - Circuit breakers and retry policies to handle failures
        - Load balancing strategies to distribute traffic
        INPUT FROM PREVIOUS STEP:
        {{step.step_1_read_infrastructure_files.output}}
        INFRASTRUCTURE AS-CODE FILES TO EVALUATE:
        {{step.step_1_read_infrastructure_files.output}}
        Output a numbered list of reliability pattern recommendations with file location,
        specific issues, remediation steps, and expected benefits.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step4-reliability-patterns.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_5_review_scalability_configurations:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_infrastructure_files]
      prompt: |
        Review the scalability configurations in the infrastructure-as-code files.
        Check for these specific issues:
        - Auto-scaling policies to adjust resources based on demand
        - Load balancing strategies to distribute traffic efficiently
        - Cache strategies to improve performance
        INPUT FROM PREVIOUS STEP:
        {{step.step_1_read_infrastructure_files.output}}
        INFRASTRUCTURE AS-CODE FILES TO REVIEW:
        {{step.step_1_read_infrastructure_files.output}}
        Output a numbered list of scalability configuration recommendations with file location,
        specific issues, remediation steps, and expected benefits.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step5-scalability-configurations.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_6_compile_infrastructure_assessment_report:
      generative_entity: "${models.target-model}"
      depends_on: [
        step_2_analyze_cost_optimization,
        step_3_check_security_best_practices,
        step_4_evaluate_reliability_patterns,
        step_5_review_scalability_configurations
      ]
      prompt: |
        Compile a comprehensive infrastructure assessment report from the analysis results below.
        INPUT FROM PREVIOUS STEP:
        {{step.step_2_analyze_cost_optimization.output}}
        INPUT FROM PREVIOUS STEP:
        {{step.step_3_check_security_best_practices.output}}
        INPUT FROM PREVIOUS STEP:
        {{step.step_4_evaluate_reliability_patterns.output}}
        INPUT FROM PREVIOUS STEP:
        {{step.step_5_review_scalability_configurations.output}}
        COST OPTIMIZATION RECOMMENDATIONS:
        {{step.step_2_analyze_cost_optimization.output}}
        SECURITY BEST PRACTICE RECOMMENDATIONS:
        {{step.step_3_check_security_best_practices.output}}
        RELIABILITY PATTERNS RECOMMENDATIONS:
        {{step.step_4_evaluate_reliability_patterns.output}}
        SCALABILITY CONFIGURATION RECOMMENDATIONS:
        {{step.step_5_review_scalability_configurations.output}}
        Create a markdown report with:
        1. Executive summary with total recommendation count by category
        2. Cost optimization section (must implement before deployment)
        3. Security best practices section (must follow)
        4. Reliability patterns section (should improve)
        5. Scalability configurations section (nice to enhance)
        6. Prioritized action items list
        Format each recommendation as: [CATEGORY] File:Line - Description
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step6-final-report.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]

```yaml
workflow_id: infrastructure_analysis_pipeline
name: "Infrastructure Analysis Pipeline"
description: "Automated analysis of DevOps infrastructure configurations for cost optimization, security best practices, reliability patterns, and scalability."
version: "2.0.0"
author: "Whitt Execution Engine"
tags: [infrastructure-analysis, security, cost-optimization]
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
    step_1_read_infrastructure_files:
      generative_entity: "${models.target-model}"
      prompt: |
        Read and display the complete contents of all infrastructure-as-code files provided.
        Output every line with line numbers prefixed.
        Preserve all whitespace, comments, and formatting exactly.
        If any file is very long, output the first 500 lines for each file.
        INFRASTRUCTURE AS-CODE FILES TO READ:
        - Terraform configuration files (e.g., *.tf)
        - Docker Compose files (e.g., docker-compose.yml)
        - Kubernetes manifests (e.g., *.yaml)
        Output a numbered list of files with line numbers prefixed.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step1-infrastructure-files.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_2_analyze_cost_optimization:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_infrastructure_files]
      prompt: |
        Analyze the cost optimization potential of the infrastructure-as-code files provided.
        Check for these specific issues:
        - Right-sizing instances to match workload demands
        - Identifying and removing unused resources
        - Evaluating spot instance opportunities
        INFRASTRUCTURE AS-CODE FILES TO ANALYZE:
        {{step.step_1_read_infrastructure_files.output}}
        Output a numbered list of cost optimization recommendations with file location, resource type,
        current usage, recommended size or type, and estimated savings.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step2-cost-optimization.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_3_check_security_best_practices:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_infrastructure_files]
      prompt: |
        Check the infrastructure-as-code files for security best practices.
        Look for these specific issues:
        - Network policies to restrict access
        - Secret management and least privilege principles
        - Encryption of sensitive data
        INFRASTRUCTURE AS-CODE FILES TO CHECK:
        {{step.step_1_read_infrastructure_files.output}}
        Output a numbered list of security best practice recommendations with file location,
        specific issues, remediation steps, and risk scores.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step3-security-best-practices.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_4_evaluate_reliability_patterns:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_infrastructure_files]
      prompt: |
        Evaluate the reliability patterns in the infrastructure-as-code files.
        Check for these specific issues:
        - Health checks to monitor system health
        - Circuit breakers and retry policies to handle failures
        - Load balancing strategies to distribute traffic
        INFRASTRUCTURE AS-CODE FILES TO EVALUATE:
        {{step.step_1_read_infrastructure_files.output}}
        Output a numbered list of reliability pattern recommendations with file location,
        specific issues, remediation steps, and expected benefits.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step4-reliability-patterns.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_5_review_scalability_configurations:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_infrastructure_files]
      prompt: |
        Review the scalability configurations in the infrastructure-as-code files.
        Check for these specific issues:
        - Auto-scaling policies to adjust resources based on demand
        - Load balancing strategies to distribute traffic efficiently
        - Cache strategies to improve performance
        INFRASTRUCTURE AS-CODE FILES TO REVIEW:
        {{step.step_1_read_infrastructure_files.output}}
        Output a numbered list of scalability configuration recommendations with file location,
        specific issues, remediation steps, and expected benefits.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step5-scalability-configurations.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_6_compile_infrastructure_assessment_report:
      generative_entity: "${models.target-model}"
      depends_on: [step_2_analyze_cost_optimization, step_3_check_security_best_practices, step_4_evaluate_reliability_patterns, step_5_review_scalability_configurations]
      prompt: |
        Compile a comprehensive infrastructure assessment report from the analysis results below.
        COST OPTIMIZATION RECOMMENDATIONS:
        {{step.step_2_analyze_cost_optimization.output}}
        SECURITY BEST PRACTICE RECOMMENDATIONS:
        {{step.step_3_check_security_best_practices.output}}
        RELIABILITY PATTERNS RECOMMENDATIONS:
        {{step.step_4_evaluate_reliability_patterns.output}}
        SCALABILITY CONFIGURATION RECOMMENDATIONS:
        {{step.step_5_review_scalability_configurations.output}}
        Create a markdown report with:
        1. Executive summary with total recommendation count by category
        2. Cost optimization section (must implement before deployment)
        3. Security best practices section (must follow)
        4. Reliability patterns section (should improve)
        5. Scalability configurations section (nice to enhance)
        6. Prioritized action items list
        Format each recommendation as: [CATEGORY] File:Line - Description
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step6-final-report.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
```