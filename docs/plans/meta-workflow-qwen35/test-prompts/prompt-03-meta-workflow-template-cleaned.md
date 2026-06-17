<!--
Source Session: ses_17a245a9cffeWo9uVFAyuF6C9I
Message Length: 109869 characters
YAML Sections: 158
Embedded Prompts: 103
Agentic Keywords: 10
Complexity: HIGH
Source Files: meta-workflow-template.yml, cleaned.yml, final-workflow.yml
-->

workflow_id: meta_workflow_generator
name: "Meta-Workflow Generator"
description: "3-step meta-workflow: generate → clean → validate/fix. Produces valid YAML agentic workflows from task descriptions."
version: "10.0.0"
author: "Whitt Execution Engine"
tags: [meta-workflow, yaml-generation, parameterized]
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
    step_1_generate:
      generative_entity: "${models.coder-model}"
      prompt: |
        Generate a COMPLETE YAML agentic workflow file for the following task:
        TASK: __TASK_PLACEHOLDER__
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
        12. Template interpolation MUST use the EXACT step name from depends_on. If a step is named step_3_check_deps then use {{step.step_3_check_deps.output}} NOT {{step.step_3_deps_check.output}}
      model_overrides:
        temperature: 0.15
        max_tokens: 50000
      when:
        before_step_starts:
          - log:
              to_file_path: "./real-workflow-attempts/outputs/__RUN_ID__/logs/meta-workflow.log"
              event_fields: [step_name, model_name, prompt_preview]
              level: info
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/__RUN_ID__/generated-raw.yml"
          - log:
              to_file_path: "./real-workflow-attempts/outputs/__RUN_ID__/logs/meta-workflow.log"
              event_fields: [step_name, duration_ms, token_count, model_name]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/outputs/__RUN_ID__/logs/meta-workflow.log"
              event_fields: [step_name, error_message, error_type]
              level: error
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
        before_step_starts:
          - log:
              to_file_path: "./real-workflow-attempts/outputs/__RUN_ID__/logs/meta-workflow.log"
              event_fields: [step_name, model_name]
              level: info
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/__RUN_ID__/cleaned.yml"
          - log:
              to_file_path: "./real-workflow-attempts/outputs/__RUN_ID__/logs/meta-workflow.log"
              event_fields: [step_name, duration_ms, token_count]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/outputs/__RUN_ID__/logs/meta-workflow.log"
              event_fields: [step_name, error_message]
              level: error
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
        CRITICAL RULES FOR INTERPOLATION:
        - The format is ALWAYS {{step.STEP_NAME.output}} — the "step." prefix is MANDATORY
        - WRONG: {{step_1_read_csv.output}} — missing "step." prefix
        - RIGHT: {{step.step_1_read_csv.output}} — has "step." prefix
        - Use the EXACT step name as it appears in the depends_on list
        - Do NOT abbreviate, rename, or guess step names
        - If depends_on says [step_2_check_deps], use {{step.step_2_check_deps.output}}
        - If depends_on says [step_4_analyze_crypto, step_5_check_auth], use BOTH:
            {{step.step_4_analyze_crypto.output}}
            {{step.step_5_check_auth.output}}
        - NEVER change an existing {{step.X.output}} reference. Keep it exactly as-is
        - EVERY existing reference MUST start with "step." — fix any that don't
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
              args: ["./real-workflow-attempts/scripts/validate-yaml.py", "./real-workflow-attempts/outputs/__RUN_ID__/cleaned.yml"]
              fail_on_error: false
          - log:
              to_file_path: "./real-workflow-attempts/outputs/__RUN_ID__/logs/meta-workflow.log"
              event_fields: [step_name, shell_output]
              level: info
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/__RUN_ID__/final-workflow.yml"
          - shell:
              command: "python3"
              args: ["./real-workflow-attempts/scripts/validate-yaml.py", "./real-workflow-attempts/outputs/__RUN_ID__/final-workflow.yml"]
              fail_on_error: false
          - log:
              to_file_path: "./real-workflow-attempts/outputs/__RUN_ID__/logs/meta-workflow.log"
              event_fields: [step_name, duration_ms, token_count, shell_output]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/outputs/__RUN_ID__/logs/meta-workflow.log"
              event_fields: [step_name, error_message]
              level: error

workflow_id: data_validation_pipeline
name: "Data Validation Pipeline"
version: "2.0.0"
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
    step_1_read_csv:
      generative_entity: "${models.target-model}"
      prompt: |
        Read and display the contents of the CSV file with headers.
        Ensure that each row has the correct number of columns as specified in the header.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step1.csv"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_2_validate_columns:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_csv]
      prompt: |
        Validate the column types and constraints of the CSV file.
        Ensure that each column has the correct data type (e.g., integer, float, string).
        Check for any missing or invalid values in the columns.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step2-validation.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_3_detect_duplicates:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_csv]
      prompt: |
        Detect any duplicate rows in the CSV file.
        Provide a list of duplicate rows with their indices.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step3-duplicates.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_4_analyze_anomalies:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_csv]
      prompt: |
        Analyze the data for any anomalies or outliers.
        Provide a list of rows with unusual values and their indices.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step4-anomalies.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_5_generate_quality_scores:
      generative_entity: "${models.target-model}"
      depends_on: [step_2_validate_columns, step_3_detect_duplicates, step_4_analyze_anomalies]
      prompt: |
        Generate data quality scores based on the validation results.
        Provide a summary of the overall data quality and any specific issues found.
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step5-quality_scores.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_6_generate_quality_report:
      generative_entity: "${models.target-model}"
      depends_on: [step_2_validate_columns, step_3_detect_duplicates, step_4_analyze_anomalies, step_5_generate_quality_scores]
      prompt: |
        Generate a comprehensive quality report with actionable recommendations.
        Include the validation results, duplicate rows, anomalies, and data quality scores.
        Provide specific suggestions for improving the data quality.
      model_overrides:
        temperature: 0.5
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step6-quality_report.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]

workflow_id: data_validation_pipeline
name: "Data Validation Pipeline"
version: "2.0.0"
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
    step_1_read_csv:
      generative_entity: "${models.target-model}"
      prompt: |
        Read and display the contents of the CSV file with headers.
        Ensure that each row has the correct number of columns as specified in the header.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step1.csv"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_2_validate_columns:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_csv]
      prompt: |
        Validate the column types and constraints of the CSV file.
        Ensure that each column has the correct data type (e.g., integer, float, string).
        Check for any missing or invalid values in the columns.
        INPUT FROM PREVIOUS STEP:
        {{step.step_1_read_csv.output}}
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step2-validation.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_3_detect_duplicates:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_csv]
      prompt: |
        Detect any duplicate rows in the CSV file.
        Provide a list of duplicate rows with their indices.
        INPUT FROM PREVIOUS STEP:
        {{step.step_1_read_csv.output}}
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step3-duplicates.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_4_analyze_anomalies:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_csv]
      prompt: |
        Analyze the data for any anomalies or outliers.
        Provide a list of rows with unusual values and their indices.
        INPUT FROM PREVIOUS STEP:
        {{step.step_1_read_csv.output}}
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step4-anomalies.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_5_generate_quality_scores:
      generative_entity: "${models.target-model}"
      depends_on: [step_2_validate_columns, step_3_detect_duplicates, step_4_analyze_anomalies]
      prompt: |
        Generate data quality scores based on the validation results.
        Provide a summary of the overall data quality and any specific issues found.
        INPUT FROM PREVIOUS STEP:
        {{step.step_2_validate_columns.output}}
        INPUT FROM PREVIOUS STEP:
        {{step.step_3_detect_duplicates.output}}
        INPUT FROM PREVIOUS STEP:
        {{step.step_4_analyze_anomalies.output}}
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step5-quality_scores.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_6_generate_quality_report:
      generative_entity: "${models.target-model}"
      depends_on: [step_2_validate_columns, step_3_detect_duplicates, step_4_analyze_anomalies, step_5_generate_quality_scores]
      prompt: |
        Generate a comprehensive quality report with actionable recommendations.
        Include the validation results, duplicate rows, anomalies, and data quality scores.
        Provide specific suggestions for improving the data quality.
        INPUT FROM PREVIOUS STEP:
        {{step.step_2_validate_columns.output}}
        INPUT FROM PREVIOUS STEP:
        {{step.step_3_detect_duplicates.output}}
        INPUT FROM PREVIOUS STEP:
        {{step.step_4_analyze_anomalies.output}}
        INPUT FROM PREVIOUS STEP:
        {{step.step_5_generate_quality_scores.output}}
      model_overrides:
        temperature: 0.5
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step6-quality_report.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]

```yaml
workflow_id: data_validation_pipeline
name: "Data Validation Pipeline"
version: "2.0.0"
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
    step_1_read_csv:
      generative_entity: "${models.target-model}"
      prompt: |
        Read and display the contents of the CSV file with headers.
        Ensure that each row has the correct number of columns as specified in the header.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step1.csv"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_2_validate_columns:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_csv]
      prompt: |
        Validate the column types and constraints of the CSV file.
        Ensure that each column has the correct data type (e.g., integer, float, string).
        Check for any missing or invalid values in the columns.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step2-validation.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_3_detect_duplicates:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_csv]
      prompt: |
        Detect any duplicate rows in the CSV file.
        Provide a list of duplicate rows with their indices.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step3-duplicates.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_4_analyze_anomalies:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_csv]
      prompt: |
        Analyze the data for any anomalies or outliers.
        Provide a list of rows with unusual values and their indices.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step4-anomalies.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_5_generate_quality_scores:
      generative_entity: "${models.target-model}"
      depends_on: [step_2_validate_columns, step_3_detect_duplicates, step_4_analyze_anomalies]
      prompt: |
        Generate data quality scores based on the validation results.
        Provide a summary of the overall data quality and any specific issues found.
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step5-quality_scores.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_6_generate_quality_report:
      generative_entity: "${models.target-model}"
      depends_on: [step_2_validate_columns, step_3_detect_duplicates, step_4_analyze_anomalies, step_5_generate_quality_scores]
      prompt: |
        Generate a comprehensive quality report with actionable recommendations.
        Include the validation results, duplicate rows, anomalies, and data quality scores.
        Provide specific suggestions for improving the data quality.
      model_overrides:
        temperature: 0.5
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step6-quality_report.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
```

workflow_id: network_traffic_analysis_pipeline
name: "Network Traffic Analysis Pipeline"
version: "2.0.0"
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
    step_1_capture_traffic:
      generative_entity: "${models.target-model}"
      prompt: |
        Capture network traffic headers and classify traffic by protocol (TCP/UDP/ICMP).
        Generate a detailed report with packet details, classification, and timestamps.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step1_traffic_report.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_2_analyze_traffic:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_capture_traffic]
      prompt: |
        Analyze the captured traffic for suspicious patterns such as port scans and DDoS attempts.
        Identify unusual payload sizes and correlate with known threat signatures.
        Generate a detailed report with findings, timestamps, and potential attack timelines.
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step2_traffic_analysis_report.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_3_generate_statistics:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_capture_traffic]
      prompt: |
        Generate traffic flow statistics and anomaly scores based on the captured traffic.
        Provide a detailed report with metrics such as packet count, bandwidth usage, and potential anomalies.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step3_traffic_statistics_report.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_4_correlate_with_signatures:
      generative_entity: "${models.target-model}"
      depends_on: [step_2_analyze_traffic, step_3_generate_statistics]
      prompt: |
        Correlate the analyzed traffic patterns with known threat signatures and generate a comprehensive report.
        Include threat classifications, attack timeline reconstruction, and recommended countermeasures.
      model_overrides:
        temperature: 0.5
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step4_traffic_assessment_report.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_5_generate_final_report:
      generative_entity: "${models.target-model}"
      depends_on: [step_2_analyze_traffic, step_3_generate_statistics, step_4_correlate_with_signatures]
      prompt: |
        Generate a final comprehensive network security assessment report that integrates all previous findings.
        Include detailed threat classifications, attack timeline reconstruction, recommended countermeasures, and overall traffic analysis summary.
      model_overrides:
        temperature: 0.6
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step5_final_report.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_6_notify_team:
      generative_entity: "${models.target-model}"
      depends_on: [step_5_generate_final_report]
      prompt: |
        Notify the network security team with a summary of the findings and recommended countermeasures.
        Provide any necessary action items or follow-up steps.
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step6_notification.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]

workflow_id: network_traffic_analysis_pipeline
name: "Network Traffic Analysis Pipeline"
version: "2.0.0"
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
    step_1_capture_traffic:
      generative_entity: "${models.target-model}"
      prompt: |
        Capture network traffic headers and classify traffic by protocol (TCP/UDP/ICMP).
        Generate a detailed report with packet details, classification, and timestamps.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step1_traffic_report.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_2_analyze_traffic:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_capture_traffic]
      prompt: |
        Analyze the captured traffic for suspicious patterns such as port scans and DDoS attempts.
        Identify unusual payload sizes and correlate with known threat signatures.
        Generate a detailed report with findings, timestamps, and potential attack timelines.
        INPUT FROM PREVIOUS STEP:
        {{step_1_capture_traffic.output}}
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step2_traffic_analysis_report.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_3_generate_statistics:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_capture_traffic]
      prompt: |
        Generate traffic flow statistics and anomaly scores based on the captured traffic.
        Provide a detailed report with metrics such as packet count, bandwidth usage, and potential anomalies.
        INPUT FROM PREVIOUS STEP:
        {{step_1_capture_traffic.output}}
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step3_traffic_statistics_report.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_4_correlate_with_signatures:
      generative_entity: "${models.target-model}"
      depends_on: [step_2_analyze_traffic, step_3_generate_statistics]
      prompt: |
        Correlate the analyzed traffic patterns with known threat signatures and generate a comprehensive report.
        Include threat classifications, attack timeline reconstruction, and recommended countermeasures.
        INPUT FROM PREVIOUS STEP:
        {{step_2_analyze_traffic.output}}
        INPUT FROM PREVIOUS STEP:
        {{step_3_generate_statistics.output}}
      model_overrides:
        temperature: 0.5
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step4_traffic_assessment_report.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_5_generate_final_report:
      generative_entity: "${models.target-model}"
      depends_on: [step_2_analyze_traffic, step_3_generate_statistics, step_4_correlate_with_signatures]
      prompt: |
        Generate a final comprehensive network security assessment report that integrates all previous findings.
        Include detailed threat classifications, attack timeline reconstruction, recommended countermeasures, and overall traffic analysis summary.
        INPUT FROM PREVIOUS STEP:
        {{step_2_analyze_traffic.output}}
        INPUT FROM PREVIOUS STEP:
        {{step_3_generate_statistics.output}}
        INPUT FROM PREVIOUS STEP:
        {{step_4_correlate_with_signatures.output}}
      model_overrides:
        temperature: 0.6
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step5_final_report.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_6_notify_team:
      generative_entity: "${models.target-model}"
      depends_on: [step_5_generate_final_report]
      prompt: |
        Notify the network security team with a summary of the findings and recommended countermeasures.
        Provide any necessary action items or follow-up steps.
        INPUT FROM PREVIOUS STEP:
        {{step_5_generate_final_report.output}}
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step6_notification.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]

```yaml
workflow_id: network_traffic_analysis_pipeline
name: "Network Traffic Analysis Pipeline"
version: "2.0.0"
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
    step_1_capture_traffic:
      generative_entity: "${models.target-model}"
      prompt: |
        Capture network traffic headers and classify traffic by protocol (TCP/UDP/ICMP).
        Generate a detailed report with packet details, classification, and timestamps.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step1_traffic_report.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_2_analyze_traffic:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_capture_traffic]
      prompt: |
        Analyze the captured traffic for suspicious patterns such as port scans and DDoS attempts.
        Identify unusual payload sizes and correlate with known threat signatures.
        Generate a detailed report with findings, timestamps, and potential attack timelines.
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step2_traffic_analysis_report.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_3_generate_statistics:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_capture_traffic]
      prompt: |
        Generate traffic flow statistics and anomaly scores based on the captured traffic.
        Provide a detailed report with metrics such as packet count, bandwidth usage, and potential anomalies.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step3_traffic_statistics_report.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_4_correlate_with_signatures:
      generative_entity: "${models.target-model}"
      depends_on: [step_2_analyze_traffic, step_3_generate_statistics]
      prompt: |
        Correlate the analyzed traffic patterns with known threat signatures and generate a comprehensive report.
        Include threat classifications, attack timeline reconstruction, and recommended countermeasures.
      model_overrides:
        temperature: 0.5
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step4_traffic_assessment_report.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_5_generate_final_report:
      generative_entity: "${models.target-model}"
      depends_on: [step_2_analyze_traffic, step_3_generate_statistics, step_4_correlate_with_signatures]
      prompt: |
        Generate a final comprehensive network security assessment report that integrates all previous findings.
        Include detailed threat classifications, attack timeline reconstruction, recommended countermeasures, and overall traffic analysis summary.
      model_overrides:
        temperature: 0.6
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step5_final_report.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
    step_6_notify_team:
      generative_entity: "${models.target-model}"
      depends_on: [step_5_generate_final_report]
      prompt: |
        Notify the network security team with a summary of the findings and recommended countermeasures.
        Provide any necessary action items or follow-up steps.
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step6_notification.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
```

workflow_id: api_contract_testing_pipeline
name: "API Contract Testing Pipeline"
description: "Automated API contract testing pipeline for OpenAPI/Swagger specifications"
version: "2.0.0"
author: "Whitt Execution Engine"
tags:
  - api-contract-testing
  - openapi-swagger
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
    step_1_read_specification:
      generative_entity: "${models.target-model}"
      prompt: |
        Read and display the complete contents of the OpenAPI/Swagger specification file provided.
        Output every line with line numbers prefixed.
        Preserve all whitespace, comments, and formatting exactly.
        If the file is very long, output the first 500 lines.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step1-specification.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_2_extract_endpoints:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_specification]
      prompt: |
        Extract all endpoint definitions from the following OpenAPI/Swagger specification file.
        Output each endpoint with its path, method, and summary.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step2-endpoints.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_3_extract_schemas:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_specification]
      prompt: |
        Extract all expected response schemas from the following OpenAPI/Swagger specification file.
        Output each schema with its type and any required fields.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step3-schemas.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_4_generate_test_cases:
      generative_entity: "${models.target-model}"
      depends_on: [step_2_extract_endpoints, step_3_extract_schemas]
      prompt: |
        Generate test cases for each endpoint in the following OpenAPI/Swagger specification file.
        Include happy path, error responses, edge cases (empty payloads, oversized inputs, invalid types),
        and prioritize them based on importance.
        ENDPOINT DEFINITIONS:
        {{step.step_2_extract_endpoints.output}}
        EXPECTED RESPONSE SCHEMAS:
        {{step.step_3_extract_schemas.output}}
        Output each test case with the following format:
        - Endpoint Path: /example-endpoint
        - Method: GET/POST/etc.
        - Test Case Description: Happy path, Error response, Edge case 1, etc.
        - Expected Response Schema: {type: string, required: [field1, field2]}
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step4-test-cases.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_5_validate_responses:
      generative_entity: "${models.target-model}"
      depends_on: [step_4_generate_test_cases]
      prompt: |
        Validate the actual API responses against the expected schemas for each test case in the following file.
        Output a numbered list of failed assertions with file location, severity (critical/warning/info),
        and a brief explanation of each failure.
        TEST CASES TO VALIDATE:
        {{step.step_4_test_cases.output}}
        Expected Response Schemas:
        {{step.step_3_extract_schemas.output}}
        Actual API Responses: (This should be provided by the API testing tool or framework)
        (Assuming this is a placeholder for actual responses, replace with actual data)
        Output each failure as: [SEVERITY] File:Line - Description
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step5-validation-report.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_6_check_slas:
      generative_entity: "${models.target-model}"
      depends_on: [step_4_generate_test_cases]
      prompt: |
        Check the HTTP status codes and response times for each test case in the following file against SLA thresholds.
        Output a numbered list of failed assertions with file location, severity (critical/warning/info),
        and a brief explanation of each failure.
        TEST CASES TO CHECK:
        {{step.step_4_test_cases.output}}
        SLA Thresholds: (This should be provided by the API owner or team)
        - HTTP Status Code: 200-299
        - Response Time: <100ms
        Output each failure as: [SEVERITY] File:Line - Description
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step6-sla-report.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_7_generate_compliance_report:
      generative_entity: "${models.target-model}"
      depends_on: [step_5_validate_responses, step_6_check_slas]
      prompt: |
        Compile a comprehensive API compliance report from the validation and SLA results below.
        VALIDATION RESULTS:
        {{step.step_5_validation_report.output}}
        SLA RESULTS:
        {{step.step_6_sla_report.output}}
        Create a markdown report with:
        1. Executive summary with total failure count by severity
        2. Critical failures section (must fix before deployment)
        3. Warning failures section (should fix)
        4. Info failures section (nice to fix)
        5. Prioritized action items list
        Format each failure as: [SEVERITY] File:Line - Description
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step7-compliance-report.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]

workflow_id: api_contract_testing_pipeline
name: "API Contract Testing Pipeline"
description: "Automated API contract testing pipeline for OpenAPI/Swagger specifications"
version: "2.0.0"
author: "Whitt Execution Engine"
tags:
  - api-contract-testing
  - openapi-swagger
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
    step_1_read_specification:
      generative_entity: "${models.target-model}"
      prompt: |
        Read and display the complete contents of the OpenAPI/Swagger specification file provided.
        Output every line with line numbers prefixed.
        Preserve all whitespace, comments, and formatting exactly.
        If the file is very long, output the first 500 lines.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step1-specification.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_2_extract_endpoints:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_specification]
      prompt: |
        Extract all endpoint definitions from the following OpenAPI/Swagger specification file.
        Output each endpoint with its path, method, and summary.
        INPUT FROM PREVIOUS STEP:
        {{step.step_1_read_specification.output}}
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step2-endpoints.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_3_extract_schemas:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_specification]
      prompt: |
        Extract all expected response schemas from the following OpenAPI/Swagger specification file.
        Output each schema with its type and any required fields.
        INPUT FROM PREVIOUS STEP:
        {{step.step_1_read_specification.output}}
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step3-schemas.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_4_generate_test_cases:
      generative_entity: "${models.target-model}"
      depends_on: [step_2_extract_endpoints, step_3_extract_schemas]
      prompt: |
        Generate test cases for each endpoint in the following OpenAPI/Swagger specification file.
        Include happy path, error responses, edge cases (empty payloads, oversized inputs, invalid types),
        and prioritize them based on importance.
        ENDPOINT DEFINITIONS:
        {{step.step_2_extract_endpoints.output}}
        EXPECTED RESPONSE SCHEMAS:
        {{step.step_3_extract_schemas.output}}
        Output each test case with the following format:
        - Endpoint Path: /example-endpoint
        - Method: GET/POST/etc.
        - Test Case Description: Happy path, Error response, Edge case 1, etc.
        - Expected Response Schema: {type: string, required: [field1, field2]}
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step4-test-cases.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_5_validate_responses:
      generative_entity: "${models.target-model}"
      depends_on: [step_4_generate_test_cases]
      prompt: |
        Validate the actual API responses against the expected schemas for each test case in the following file.
        Output a numbered list of failed assertions with file location, severity (critical/warning/info),
        and a brief explanation of each failure.
        TEST CASES TO VALIDATE:
        {{step.step_4_generate_test_cases.output}}
        Expected Response Schemas:
        {{step.step_3_extract_schemas.output}}
        Actual API Responses: (This should be provided by the API testing tool or framework)
        (Assuming this is a placeholder for actual responses, replace with actual data)
        Output each failure as: [SEVERITY] File:Line - Description
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step5-validation-report.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_6_check_slas:
      generative_entity: "${models.target-model}"
      depends_on: [step_4_generate_test_cases]
      prompt: |
        Check the HTTP status codes and response times for each test case in the following file against SLA thresholds.
        Output a numbered list of failed assertions with file location, severity (critical/warning/info),
        and a brief explanation of each failure.
        TEST CASES TO CHECK:
        {{step.step_4_generate_test_cases.output}}
        SLA Thresholds: 
        - HTTP Status Code: 200-299
        - Response Time: <100ms
        Output each failure as: [SEVERITY] File:Line - Description
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step6-sla-report.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_7_generate_compliance_report:
      generative_entity: "${models.target-model}"
      depends_on: [step_5_validate_responses, step_6_check_slas]
      prompt: |
        Compile a comprehensive API compliance report from the validation and SLA results below.
        VALIDATION RESULTS:
        {{step.step_5_validation_report.output}}
        SLA RESULTS:
        {{step.step_6_sla_report.output}}
        Create a markdown report with:
        1. Executive summary with total failure count by severity
        2. Critical failures section (must fix before deployment)
        3. Warning failures section (should fix)
        4. Info failures section (nice to fix)
        5. Prioritized action items list
        Format each failure as: [SEVERITY] File:Line - Description
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step7-compliance-report.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]

```yaml
workflow_id: api_contract_testing_pipeline
name: "API Contract Testing Pipeline"
description: "Automated API contract testing pipeline for OpenAPI/Swagger specifications"
version: "2.0.0"
author: "Whitt Execution Engine"
tags: [api-contract-testing, openapi-swagger]
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
    step_1_read_specification:
      generative_entity: "${models.target-model}"
      prompt: |
        Read and display the complete contents of the OpenAPI/Swagger specification file provided.
        Output every line with line numbers prefixed.
        Preserve all whitespace, comments, and formatting exactly.
        If the file is very long, output the first 500 lines.
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step1-specification.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_2_extract_endpoints:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_specification]
      prompt: |
        Extract all endpoint definitions from the following OpenAPI/Swagger specification file.
        Output each endpoint with its path, method, and summary.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step2-endpoints.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_3_extract_schemas:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_read_specification]
      prompt: |
        Extract all expected response schemas from the following OpenAPI/Swagger specification file.
        Output each schema with its type and any required fields.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step3-schemas.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_4_generate_test_cases:
      generative_entity: "${models.target-model}"
      depends_on: [step_2_extract_endpoints, step_3_extract_schemas]
      prompt: |
        Generate test cases for each endpoint in the following OpenAPI/Swagger specification file.
        Include happy path, error responses, edge cases (empty payloads, oversized inputs, invalid types),
        and prioritize them based on importance.
        ENDPOINT DEFINITIONS:
        {{step.step_2_extract_endpoints.output}}
        EXPECTED RESPONSE SCHEMAS:
        {{step.step_3_extract_schemas.output}}
        Output each test case with the following format:
        - Endpoint Path: /example-endpoint
        - Method: GET/POST/etc.
        - Test Case Description: Happy path, Error response, Edge case 1, etc.
        - Expected Response Schema: {type: string, required: [field1, field2]}
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step4-test-cases.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_5_validate_responses:
      generative_entity: "${models.target-model}"
      depends_on: [step_4_generate_test_cases]
      prompt: |
        Validate the actual API responses against the expected schemas for each test case in the following file.
        Output a numbered list of failed assertions with file location, severity (critical/warning/info),
        and a brief explanation of each failure.
        TEST CASES TO VALIDATE:
        {{step.step_4_test_cases.output}}
        Expected Response Schemas:
        {{step.step_3_extract_schemas.output}}
        Actual API Responses: (This should be provided by the API testing tool or framework)
        (Assuming this is a placeholder for actual responses, replace with actual data)
        Output each failure as: [SEVERITY] File:Line - Description
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step5-validation-report.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_6_check_slas:
      generative_entity: "${models.target-model}"
      depends_on: [step_4_generate_test_cases]
      prompt: |
        Check the HTTP status codes and response times for each test case in the following file against SLA thresholds.
        Output a numbered list of failed assertions with file location, severity (critical/warning/info),
        and a brief explanation of each failure.
        TEST CASES TO CHECK:
        {{step.step_4_test_cases.output}}
        SLA Thresholds: (This should be provided by the API owner or team)
        - HTTP Status Code: 200-299
        - Response Time: <100ms
        Output each failure as: [SEVERITY] File:Line - Description
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step6-sla-report.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_7_generate_compliance_report:
      generative_entity: "${models.target-model}"
      depends_on: [step_5_validate_responses, step_6_check_slas]
      prompt: |
        Compile a comprehensive API compliance report from the validation and SLA results below.
        VALIDATION RESULTS:
        {{step.step_5_validation_report.output}}
        SLA RESULTS:
        {{step.step_6_sla_report.output}}
        Create a markdown report with:
        1. Executive summary with total failure count by severity
        2. Critical failures section (must fix before deployment)
        3. Warning failures section (should fix)
        4. Info failures section (nice to fix)
        5. Prioritized action items list
        Format each failure as: [SEVERITY] File:Line - Description
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step7-compliance-report.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
```

workflow_id: ml_model_evaluation_pipeline
name: "Machine Learning Model Evaluation Pipeline"
description: "Automated evaluation of machine learning models with fairness auditing and deployment readiness assessment"
version: "2.0.0"
author: "Qwen AI"
tags:
  - ml-model-evaluation
  - fairness-auditing
  - deployment-readiness
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
    hosting:
      gpu_layers: 999
models:
  "trained-model":
    name: "Qwen2.5-Coder-3B-Instruct-Q8_0"
    host:
      type: llama_cpp_with_vulkan
workflow_execution_strategy:
  memory:
    model_lifecycle:
      unload_unused: true
agentic_workflow:
  steps:
    step_1_load_model_artifacts:
      generative_entity: "${models.trained-model}"
      prompt: |
        Load and display the trained machine learning model artifacts.
        Output detailed information about the model architecture, training parameters,
        and any relevant metadata.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step1-model-artifacts.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_2_run_inference:
      generative_entity: "${models.trained-model}"
      depends_on: [step_1_load_model_artifacts]
      prompt: |
        Run inference on a test dataset using the loaded machine learning model.
        Output detailed results including predictions and any relevant metrics.
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step2-inference-results.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_3_compute_metrics:
      generative_entity: "${models.trained-model}"
      depends_on: [step_2_run_inference]
      prompt: |
        Compute classification metrics for the inference results.
        Calculate precision, recall, F1-score, and AUC-ROC.
        Output detailed statistics for each metric.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step3-metrics.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_4_confusion_matrix:
      generative_entity: "${models.trained-model}"
      depends_on: [step_2_run_inference]
      prompt: |
        Generate a confusion matrix for the inference results.
        Output detailed information about true positives, false negatives,
        true negatives, and false positives.
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step4-confusion-matrix.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_5_misclassification_patterns:
      generative_entity: "${models.trained-model}"
      depends_on: [step_2_run_inference]
      prompt: |
        Identify misclassification patterns by class in the inference results.
        Output detailed information about each class's misclassified instances,
        including predicted and actual labels.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step5-misclassification-patterns.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_6_fairness_audit:
      generative_entity: "${models.trained-model}"
      depends_on: [step_2_run_inference]
      prompt: |
        Perform fairness auditing on the model's performance across different demographic groups.
        Identify any biases or disparities in predictions and output detailed analysis.
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step6-fairness-audit.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_7_generate_report:
      generative_entity: "${models.trained-model}"
      depends_on: [step_3_compute_metrics, step_4_confusion_matrix, step_5_misclassification_patterns, step_6_fairness_audit]
      prompt: |
        Generate a comprehensive model evaluation report with bias indicators and deployment readiness assessment.
        Include:
        - Detailed metrics from step 3
        - Confusion matrix from step 4
        - Misclassification patterns from step 5
        - Fairness audit results from step 6
        - Deployment readiness assessment based on model performance and fairness
        OUTPUT THE REPORT IN MARKDOWN FORMAT WITH:
        - Executive summary
        - Detailed metrics section
        - Confusion matrix analysis
        - Misclassification pattern identification
        - Fairness audit findings
        - Deployment readiness evaluation
      model_overrides:
        temperature: 0.5
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

workflow_id: ml_model_evaluation_pipeline
name: "Machine Learning Model Evaluation Pipeline"
description: "Automated evaluation of machine learning models with fairness auditing and deployment readiness assessment"
version: "2.0.0"
author: "Qwen AI"
tags:
  - ml-model-evaluation
  - fairness-auditing
  - deployment-readiness
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
    hosting:
      gpu_layers: 999
models:
  "trained-model":
    name: "Qwen2.5-Coder-3B-Instruct-Q8_0"
    host:
      type: llama_cpp_with_vulkan
workflow_execution_strategy:
  memory:
    model_lifecycle:
      unload_unused: true
agentic_workflow:
  steps:
    step_1_load_model_artifacts:
      generative_entity: "${models.trained-model}"
      prompt: |
        Load and display the trained machine learning model artifacts.
        Output detailed information about the model architecture, training parameters,
        and any relevant metadata.
        INPUT FROM PREVIOUS STEP:
        {{step.step_2_run_inference.output}}
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step1-model-artifacts.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_2_run_inference:
      generative_entity: "${models.trained-model}"
      depends_on: [step_1_load_model_artifacts]
      prompt: |
        Run inference on a test dataset using the loaded machine learning model.
        Output detailed results including predictions and any relevant metrics.
        INPUT FROM PREVIOUS STEP:
        {{step.step_1_load_model_artifacts.output}}
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step2-inference-results.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_3_compute_metrics:
      generative_entity: "${models.trained-model}"
      depends_on: [step_2_run_inference]
      prompt: |
        Compute classification metrics for the inference results.
        Calculate precision, recall, F1-score, and AUC-ROC.
        Output detailed statistics for each metric.
        INPUT FROM PREVIOUS STEP:
        {{step.step_2_run_inference.output}}
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step3-metrics.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_4_confusion_matrix:
      generative_entity: "${models.trained-model}"
      depends_on: [step_2_run_inference]
      prompt: |
        Generate a confusion matrix for the inference results.
        Output detailed information about true positives, false negatives,
        true negatives, and false positives.
        INPUT FROM PREVIOUS STEP:
        {{step.step_2_run_inference.output}}
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step4-confusion-matrix.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_5_misclassification_patterns:
      generative_entity: "${models.trained-model}"
      depends_on: [step_2_run_inference]
      prompt: |
        Identify misclassification patterns by class in the inference results.
        Output detailed information about each class's misclassified instances,
        including predicted and actual labels.
        INPUT FROM PREVIOUS STEP:
        {{step.step_2_run_inference.output}}
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step5-misclassification-patterns.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_6_fairness_audit:
      generative_entity: "${models.trained-model}"
      depends_on: [step_2_run_inference]
      prompt: |
        Perform fairness auditing on the model's performance across different demographic groups.
        Identify any biases or disparities in predictions and output detailed analysis.
        INPUT FROM PREVIOUS STEP:
        {{step.step_2_run_inference.output}}
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step6-fairness-audit.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_7_generate_report:
      generative_entity: "${models.trained-model}"
      depends_on: [step_3_compute_metrics, step_4_confusion_matrix, step_5_misclassification_patterns, step_6_fairness_audit]
      prompt: |
        Generate a comprehensive model evaluation report with bias indicators and deployment readiness assessment.
        Include:
        - Detailed metrics from step 3
        - Confusion matrix from step 4
        - Misclassification patterns from step 5
        - Fairness audit results from step 6
        - Deployment readiness assessment based on model performance and fairness
        OUTPUT THE REPORT IN MARKDOWN FORMAT WITH:
        - Executive summary
        - Detailed metrics section
        - Confusion matrix analysis
        - Misclassification pattern identification
        - Fairness audit findings
        - Deployment readiness evaluation
        INPUT FROM PREVIOUS STEP:
        {{step.step_3_compute_metrics.output}}
        INPUT FROM PREVIOUS STEP:
        {{step.step_4_confusion_matrix.output}}
        INPUT FROM PREVIOUS STEP:
        {{step.step_5_misclassification_patterns.output}}
        INPUT FROM PREVIOUS STEP:
        {{step.step_6_fairness_audit.output}}
      model_overrides:
        temperature: 0.5
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
workflow_id: ml_model_evaluation_pipeline
name: "Machine Learning Model Evaluation Pipeline"
description: "Automated evaluation of machine learning models with fairness auditing and deployment readiness assessment"
version: "2.0.0"
author: "Qwen AI"
tags: [ml-model-evaluation, fairness-auditing, deployment-readiness]
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
    hosting:
      gpu_layers: 999
models:
  "trained-model":
    name: "Qwen2.5-Coder-3B-Instruct-Q8_0"
    host:
      type: llama_cpp_with_vulkan
workflow_execution_strategy:
  memory:
    model_lifecycle:
      unload_unused: true
agentic_workflow:
  steps:
    step_1_load_model_artifacts:
      generative_entity: "${models.trained-model}"
      prompt: |
        Load and display the trained machine learning model artifacts.
        Output detailed information about the model architecture, training parameters,
        and any relevant metadata.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step1-model-artifacts.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_2_run_inference:
      generative_entity: "${models.trained-model}"
      depends_on: [step_1_load_model_artifacts]
      prompt: |
        Run inference on a test dataset using the loaded machine learning model.
        Output detailed results including predictions and any relevant metrics.
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step2-inference-results.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_3_compute_metrics:
      generative_entity: "${models.trained-model}"
      depends_on: [step_2_run_inference]
      prompt: |
        Compute classification metrics for the inference results.
        Calculate precision, recall, F1-score, and AUC-ROC.
        Output detailed statistics for each metric.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step3-metrics.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_4_confusion_matrix:
      generative_entity: "${models.trained-model}"
      depends_on: [step_2_run_inference]
      prompt: |
        Generate a confusion matrix for the inference results.
        Output detailed information about true positives, false negatives,
        true negatives, and false positives.
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step4-confusion-matrix.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_5_misclassification_patterns:
      generative_entity: "${models.trained-model}"
      depends_on: [step_2_run_inference]
      prompt: |
        Identify misclassification patterns by class in the inference results.
        Output detailed information about each class's misclassified instances,
        including predicted and actual labels.
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step5-misclassification-patterns.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_6_fairness_audit:
      generative_entity: "${models.trained-model}"
      depends_on: [step_2_run_inference]
      prompt: |
        Perform fairness auditing on the model's performance across different demographic groups.
        Identify any biases or disparities in predictions and output detailed analysis.
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step6-fairness-audit.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_7_generate_report:
      generative_entity: "${models.trained-model}"
      depends_on: [step_3_compute_metrics, step_4_confusion_matrix, step_5_misclassification_patterns, step_6_fairness_audit]
      prompt: |
        Generate a comprehensive model evaluation report with bias indicators and deployment readiness assessment.
        Include:
        - Detailed metrics from step 3
        - Confusion matrix from step 4
        - Misclassification patterns from step 5
        - Fairness audit results from step 6
        - Deployment readiness assessment based on model performance and fairness
        OUTPUT THE REPORT IN MARKDOWN FORMAT WITH:
        - Executive summary
        - Detailed metrics section
        - Confusion matrix analysis
        - Misclassification pattern identification
        - Fairness audit findings
        - Deployment readiness evaluation
      model_overrides:
        temperature: 0.5
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

workflow_id: documentation_generation_pipeline
name: "Documentation Generation Pipeline"
description: "Automated documentation generation pipeline for source code"
version: "2.0.0"
author: "Whitt Execution Engine"
tags:
  - documentation
  - code-analysis
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
    step_1_scan_source_code:
      generative_entity: "${models.target-model}"
      prompt: |
        Scan the source code directories for public APIs and their docstrings.
        Identify undocumented public APIs and provide a list of functions with missing documentation.
        Include line numbers in the output to facilitate easy reference.
        SOURCE CODE DIRECTORIES TO SCAN:
        /path/to/source/code/directory1
        /path/to/source/code/directory2
        OUTPUT FORMAT:
        - Function Name: Line Number(s) where documented
        - Function Name: Line Number(s) where undocumented
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step1-undocumented-functions.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_2_extract_docstrings:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_scan_source_code]
      prompt: |
        Extract and display the docstrings for all documented functions from the source code.
        Include line numbers in the output to facilitate easy reference.
        DOCUMENTED FUNCTIONS FROM SOURCE CODE:
        {{step.step_1_scan_source_code.output}}
        OUTPUT FORMAT:
        - Function Name: Line Number(s) where documented
        - Docstring content
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step2-extracted-docstrings.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_3_generate_api_reference:
      generative_entity: "${models.target-model}"
      depends_on: [step_2_extract_docstrings]
      prompt: |
        Generate API reference documentation in markdown format from the extracted docstrings.
        Include function signatures and detailed descriptions for each documented function.
        EXTRACTED DOCSTRINGS FROM SOURCE CODE:
        {{step.step_2_extract_docstrings.output}}
        OUTPUT FORMAT:
        # API Reference
        ## Functions
        - **Function Name**: Description of the function
          - **Parameters**:
            - Parameter 1: Description
            - Parameter 2: Description
          - **Returns**: Description
          - **Example Usage**:
            ```python
            # Example usage of the function
            result = function_name(param1, param2)
            print(result)
            ```
      model_overrides:
        temperature: 0.5
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step3-api-reference.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_4_create_usage_examples:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_scan_source_code]
      prompt: |
        Create usage examples from test files for all documented functions.
        Include line numbers in the output to facilitate easy reference.
        DOCUMENTED FUNCTIONS FROM SOURCE CODE:
        {{step.step_1_scan_source_code.output}}
        OUTPUT FORMAT:
        - Function Name: Line Number(s) where documented
        - Usage Example
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step4-usage-examples.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_5_cross_reference_modules:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_scan_source_code]
      prompt: |
        Cross-reference related modules and dependencies for all documented functions.
        Provide a list of modules or packages that each function depends on.
        DOCUMENTED FUNCTIONS FROM SOURCE CODE:
        {{step.step_1_scan_source_code.output}}
        OUTPUT FORMAT:
        - Function Name: List of dependent modules/packages
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step5-cross-reference.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_6_generate_documentation_site:
      generative_entity: "${models.target-model}"
      depends_on: [step_3_generate_api_reference, step_4_create_usage_examples, step_5_cross_reference_modules]
      prompt: |
        Generate a complete documentation site structure with navigation indices and search metadata.
        Include the API reference, usage examples, and cross-reference information.
        API REFERENCE DOCUMENTATION:
        {{step.step_3_generate_api_reference.output}}
        USAGE EXAMPLES:
        {{step.step_4_create_usage_examples.output}}
        CROSS-REFERENCE INFORMATION:
        {{step.step_5_cross_reference.txt}}
        OUTPUT FORMAT:
        # Documentation Site
        ## Navigation Indices
        - [API Reference](#api-reference)
        - [Usage Examples](#usage-examples)
        - [Cross-Reference Information](#cross-reference-information)
        ### API Reference
        {{step.step_3_generate_api_reference.output}}
        ### Usage Examples
        {{step.step_4_create_usage_examples.output}}
        ### Cross-Reference Information
        {{step.step_5_cross_reference.txt}}
      model_overrides:
        temperature: 0.6
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step6-documentation-site.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]

workflow_id: documentation_generation_pipeline
name: "Documentation Generation Pipeline"
description: "Automated documentation generation pipeline for source code"
version: "2.0.0"
author: "Whitt Execution Engine"
tags:
  - documentation
  - code-analysis
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
    step_1_scan_source_code:
      generative_entity: "${models.target-model}"
      prompt: |
        Scan the source code directories for public APIs and their docstrings.
        Identify undocumented public APIs and provide a list of functions with missing documentation.
        Include line numbers in the output to facilitate easy reference.
        SOURCE CODE DIRECTORIES TO SCAN:
        /path/to/source/code/directory1
        /path/to/source/code/directory2
        OUTPUT FORMAT:
        - Function Name: Line Number(s) where documented
        - Function Name: Line Number(s) where undocumented
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step1-undocumented-functions.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_2_extract_docstrings:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_scan_source_code]
      prompt: |
        Extract and display the docstrings for all documented functions from the source code.
        Include line numbers in the output to facilitate easy reference.
        INPUT FROM PREVIOUS STEP:
        {{step.step_1_scan_source_code.output}}
        DOCUMENTED FUNCTIONS FROM SOURCE CODE:
        {{step.step_1_scan_source_code.output}}
        OUTPUT FORMAT:
        - Function Name: Line Number(s) where documented
        - Docstring content
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step2-extracted-docstrings.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_3_generate_api_reference:
      generative_entity: "${models.target-model}"
      depends_on: [step_2_extract_docstrings]
      prompt: |
        Generate API reference documentation in markdown format from the extracted docstrings.
        Include function signatures and detailed descriptions for each documented function.
        INPUT FROM PREVIOUS STEP:
        {{step.step_2_extract_docstrings.output}}
        EXTRACTED DOCSTRINGS FROM SOURCE CODE:
        {{step.step_2_extract_docstrings.output}}
        OUTPUT FORMAT:
        # API Reference
        ## Functions
        - **Function Name**: Description of the function
          - **Parameters**:
            - Parameter 1: Description
            - Parameter 2: Description
          - **Returns**: Description
          - **Example Usage**:
            ```python
            # Example usage of the function
            result = function_name(param1, param2)
            print(result)
            ```
      model_overrides:
        temperature: 0.5
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step3-api-reference.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_4_create_usage_examples:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_scan_source_code]
      prompt: |
        Create usage examples from test files for all documented functions.
        Include line numbers in the output to facilitate easy reference.
        INPUT FROM PREVIOUS STEP:
        {{step.step_1_scan_source_code.output}}
        DOCUMENTED FUNCTIONS FROM SOURCE CODE:
        {{step.step_1_scan_source_code.output}}
        OUTPUT FORMAT:
        - Function Name: Line Number(s) where documented
        - Usage Example
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step4-usage-examples.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_5_cross_reference_modules:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_scan_source_code]
      prompt: |
        Cross-reference related modules and dependencies for all documented functions.
        Provide a list of modules or packages that each function depends on.
        INPUT FROM PREVIOUS STEP:
        {{step.step_1_scan_source_code.output}}
        DOCUMENTED FUNCTIONS FROM SOURCE CODE:
        {{step.step_1_scan_source_code.output}}
        OUTPUT FORMAT:
        - Function Name: List of dependent modules/packages
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step5-cross-reference.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_6_generate_documentation_site:
      generative_entity: "${models.target-model}"
      depends_on: [step_3_generate_api_reference, step_4_create_usage_examples, step_5_cross_reference_modules]
      prompt: |
        Generate a complete documentation site structure with navigation indices and search metadata.
        Include the API reference, usage examples, and cross-reference information.
        INPUT FROM PREVIOUS STEP:
        {{step.step_3_generate_api_reference.output}}
        INPUT FROM PREVIOUS STEP:
        {{step.step_4_create_usage_examples.output}}
        INPUT FROM PREVIOUS STEP:
        {{step.step_5_cross_reference_modules.output}}
        API REFERENCE DOCUMENTATION:
        {{step.step_3_generate_api_reference.output}}
        USAGE EXAMPLES:
        {{step.step_4_create_usage_examples.output}}
        CROSS-REFERENCE INFORMATION:
        {{step.step_5_cross_reference_modules.output}}
        OUTPUT FORMAT:
        # Documentation Site
        ## Navigation Indices
        - [API Reference](#api-reference)
        - [Usage Examples](#usage-examples)
        - [Cross-Reference Information](#cross-reference-information)
        ### API Reference
        {{step.step_3_generate_api_reference.output}}
        ### Usage Examples
        {{step.step_4_create_usage_examples.output}}
        ### Cross-Reference Information
        {{step.step_5_cross_reference_modules.output}}
      model_overrides:
        temperature: 0.6
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step6-documentation-site.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]

```yaml
workflow_id: documentation_generation_pipeline
name: "Documentation Generation Pipeline"
description: "Automated documentation generation pipeline for source code"
version: "2.0.0"
author: "Whitt Execution Engine"
tags: [documentation, code-analysis]
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
    step_1_scan_source_code:
      generative_entity: "${models.target-model}"
      prompt: |
        Scan the source code directories for public APIs and their docstrings.
        Identify undocumented public APIs and provide a list of functions with missing documentation.
        Include line numbers in the output to facilitate easy reference.
        SOURCE CODE DIRECTORIES TO SCAN:
        /path/to/source/code/directory1
        /path/to/source/code/directory2
        OUTPUT FORMAT:
        - Function Name: Line Number(s) where documented
        - Function Name: Line Number(s) where undocumented
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step1-undocumented-functions.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_2_extract_docstrings:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_scan_source_code]
      prompt: |
        Extract and display the docstrings for all documented functions from the source code.
        Include line numbers in the output to facilitate easy reference.
        DOCUMENTED FUNCTIONS FROM SOURCE CODE:
        {{step.step_1_scan_source_code.output}}
        OUTPUT FORMAT:
        - Function Name: Line Number(s) where documented
        - Docstring content
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step2-extracted-docstrings.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_3_generate_api_reference:
      generative_entity: "${models.target-model}"
      depends_on: [step_2_extract_docstrings]
      prompt: |
        Generate API reference documentation in markdown format from the extracted docstrings.
        Include function signatures and detailed descriptions for each documented function.
        EXTRACTED DOCSTRINGS FROM SOURCE CODE:
        {{step.step_2_extract_docstrings.output}}
        OUTPUT FORMAT:
        # API Reference
        ## Functions
        - **Function Name**: Description of the function
          - **Parameters**:
            - Parameter 1: Description
            - Parameter 2: Description
          - **Returns**: Description
          - **Example Usage**:
            ```python
            # Example usage of the function
            result = function_name(param1, param2)
            print(result)
            ```
      model_overrides:
        temperature: 0.5
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step3-api-reference.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_4_create_usage_examples:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_scan_source_code]
      prompt: |
        Create usage examples from test files for all documented functions.
        Include line numbers in the output to facilitate easy reference.
        DOCUMENTED FUNCTIONS FROM SOURCE CODE:
        {{step.step_1_scan_source_code.output}}
        OUTPUT FORMAT:
        - Function Name: Line Number(s) where documented
        - Usage Example
      model_overrides:
        temperature: 0.4
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step4-usage-examples.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_5_cross_reference_modules:
      generative_entity: "${models.target-model}"
      depends_on: [step_1_scan_source_code]
      prompt: |
        Cross-reference related modules and dependencies for all documented functions.
        Provide a list of modules or packages that each function depends on.
        DOCUMENTED FUNCTIONS FROM SOURCE CODE:
        {{step.step_1_scan_source_code.output}}
        OUTPUT FORMAT:
        - Function Name: List of dependent modules/packages
      model_overrides:
        temperature: 0.3
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step5-cross-reference.txt"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
    step_6_generate_documentation_site:
      generative_entity: "${models.target-model}"
      depends_on: [step_3_generate_api_reference, step_4_create_usage_examples, step_5_cross_reference_modules]
      prompt: |
        Generate a complete documentation site structure with navigation indices and search metadata.
        Include the API reference, usage examples, and cross-reference information.
        API REFERENCE DOCUMENTATION:
        {{step.step_3_generate_api_reference.output}}
        USAGE EXAMPLES:
        {{step.step_4_create_usage_examples.output}}
        CROSS-REFERENCE INFORMATION:
        {{step.step_5_cross_reference.txt}}
        OUTPUT FORMAT:
        # Documentation Site
        ## Navigation Indices
        - [API Reference](#api-reference)
        - [Usage Examples](#usage-examples)
        - [Cross-Reference Information](#cross-reference-information)
        ### API Reference
        {{step.step_3_generate_api_reference.output}}
        ### Usage Examples
        {{step.step_4_create_usage_examples.output}}
        ### Cross-Reference Information
        {{step.step_5_cross_reference.txt}}
      model_overrides:
        temperature: 0.6
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./outputs/step6-documentation-site.md"
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./logs/workflow.log"
              event_fields: [step_name, error_message]
```