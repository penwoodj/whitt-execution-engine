<!--
Source Session: ses_17a245a9cffeWo9uVFAyuF6C9I
Message Length: 155196 characters
YAML Sections: 133
Embedded Prompts: 73
Agentic Keywords: 15
Complexity: HIGH
Source Files: attempt-1-single-pass.yml, attempt-2-example-based.yml, attempt-3-multi-phase.yml
-->

# ═══════════════════════════════════════════════════════════════════
# Meta-Workflow: Prompt → YAML Workflow Generator (Attempt 1)
# Schema-compliant per docs/schema/unified-workflow-schema.yml v2.0
#
# Takes a natural language description and generates a valid YAML
# agentic workflow using a local SLM (Qwen2.5-Coder-3B).
#
# Strategy: Single-pass generation with extensive schema context
# baked into the prompt, followed by shell validation.
#
# Steps:
#   step_1_analyze:   Identify required features from prompt
#   step_2_generate:  Generate complete YAML workflow
#   step_3_validate:  Shell action to validate output YAML
#   step_4_report:    Summarize success/failure
# ═══════════════════════════════════════════════════════════════════
workflow_id: meta_workflow_generator_v1
name: "Meta-Workflow Generator v1"
description: "Generate valid YAML agentic workflows from natural language prompts using local SLMs"
version: "2.0.0"
author: "Whitt Execution Engine"
tags: [meta-workflow, yaml-generation, code-model]
min_schema_version: "2.0.0"
schema_version: "2.0.0"
# ── PROVIDERS ───────────────────────────────────────────────────
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
    hosting:
      gpu_layers: 999
# ── MODELS ───────────────────────────────────────────────────────
models:
  "coder-model":
    name: "Qwen2.5-Coder-3B-Instruct-Q8_0"
    host:
      type: llama_cpp_with_vulkan
    execution:
      max_tokens: 50000
      timeout:
        load_into_memory: "30s"
        time_to_first_response: "10s"
        total_time_to_response: "120s"
  "reviewer-model":
    name: "Qwen3-4B-Instruct-2507-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
    execution:
      max_tokens: 50000
      timeout:
        load_into_memory: "30s"
        time_to_first_response: "10s"
        total_time_to_response: "120s"
# ── WORKFLOW EXECUTION STRATEGY ────────────────────────────────
workflow_execution_strategy:
  memory:
    model_lifecycle:
      unload_unused: true
# ── AGENTIC WORKFLOW ───────────────────────────────────────────
agentic_workflow:
  steps:
    # ── STEP 1: Analyze the prompt ────────────────────────────────
    step_1_analyze:
      generative_entity: "${models.coder-model}"
      prompt: |
        You are a YAML workflow architect. Analyze the following user request and output a JSON plan.
        USER REQUEST:
        "Build a workflow that reads a Python source file, analyzes it for code quality issues, suggests fixes, and generates a summary report with metrics"
        Output a JSON object with these keys:
        - "workflow_id": snake_case identifier
        - "name": short human-readable name
        - "description": one sentence
        - "num_steps": number of steps needed (3-6)
        - "steps": array of objects, each with:
          - "step_name": snake_case name
          - "purpose": what this step does (one sentence)
          - "needs_previous_output": boolean
          - "model_overrides": object with temperature and max_tokens
        - "hooks_needed": array of trigger names (e.g. "after_step_succeeds", "before_step_starts")
        - "dependencies": array of arrays showing which steps depend on which
        Output ONLY the JSON. No markdown. No backticks.
        First character: { Last character: }
      model_overrides:
        temperature: 0.3
        max_tokens: 2000
      when:
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/v1-step1-analysis.json"
          - log:
              to_file_path: "./real-workflow-attempts/logs/v1-generator.log"
              event_fields: [step_name, duration_ms, token_count]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v1-generator.log"
              event_fields: [step_name, error_message]
    # ── STEP 2: Generate the complete YAML workflow ───────────────
    step_2_generate:
      generative_entity: "${models.coder-model}"
      depends_on: [step_1_analyze]
      prompt: |
        You are a YAML workflow generator. Generate a COMPLETE, VALID YAML agentic workflow file.
        REFERENCE: The analysis plan from step 1:
        {{step.step_1_analyze.output}}
        Generate a YAML workflow with EXACTLY this structure. Follow these rules STRICTLY:
        REQUIRED TOP-LEVEL KEYS (in this order):
        1. workflow_id: snake_case string
        2. name: quoted string
        3. description: quoted string
        4. version: "2.0.0"
        5. author: "Whitt Execution Engine"
        6. tags: [list of strings]
        7. schema_version: "2.0.0"
        8. providers: mapping with one key "llama_cpp_with_vulkan" containing:
           config: {host: localhost, port: 8080}
           hosting: {gpu_layers: 999}
        9. models: mapping with key "target-model" containing:
           name: "Qwen2.5-Coder-3B-Instruct-Q8_0"
           host: {type: llama_cpp_with_vulkan}
        10. workflow_execution_strategy:
            memory: {model_lifecycle: {unload_unused: true}}
        11. agentic_workflow: mapping with key "steps" containing all steps
        STEP FORMAT (each step is a mapping):
        - step_name_here:
            generative_entity: "${models.target-model}"
            prompt: |
              <prompt text here>
            model_overrides:
              temperature: 0.3
              max_tokens: 50000
            depends_on: [previous_step_name]
            when:
              after_step_succeeds:
                - save_to: "./outputs/step-output.txt"
                - log:
                    to_file_path: "./logs/workflow.log"
                    event_fields: [step_name, duration_ms]
              after_step_fails:
                - log:
                    to_file_path: "./logs/workflow.log"
                    event_fields: [step_name, error_message]
        CRITICAL YAML RULES:
        - Use 2-space indentation consistently
        - All strings with special chars must be quoted
        - Use | for multi-line prompt strings
        - Lists use - dash notation
        - No tabs, only spaces
        - No trailing commas (YAML doesn't use them)
        - Do NOT wrap output in ```yaml``` code fences
        - Output ONLY the raw YAML, starting with workflow_id:
        Generate the complete workflow now. First line must be: workflow_id:
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/v1-generated-workflow.yml"
          - log:
              to_file_path: "./real-workflow-attempts/logs/v1-generator.log"
              event_fields: [step_name, duration_ms, token_count, json_parsable]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v1-generator.log"
              event_fields: [step_name, error_message]
    # ── STEP 3: Validate the generated YAML ───────────────────────
    step_3_validate:
      generative_entity: "${models.reviewer-model}"
      depends_on: [step_2_generate]
      prompt: |
        You are a YAML validation reviewer. The following is a generated YAML workflow. Check it for:
        1. Valid YAML syntax
        2. Required fields present (workflow_id, name, providers, models, agentic_workflow)
        3. Correct indentation
        4. No markdown artifacts (backticks, code fences)
        5. Proper use of template variables (${} and {{}})
        GENERATED YAML:
        {{step.step_2_generate.output}}
        Output a JSON object:
        - "valid": true or false
        - "errors": array of error strings (empty if valid)
        - "warnings": array of warning strings
        - "line_count": number of lines in the YAML
        Output ONLY the JSON. No markdown. No backticks.
      model_overrides:
        temperature: 0.1
        max_tokens: 2000
      when:
        before_step_starts:
          - shell:
              command: "python3"
              args: ["./real-workflow-attempts/scripts/validate-yaml.py", "./real-workflow-attempts/outputs/v1-generated-workflow.yml"]
              fail_on_error: false
          - log:
              to_file_path: "./real-workflow-attempts/logs/v1-generator.log"
              event_fields: [step_name, shell_output]
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/v1-validation-result.json"
          - log:
              to_file_path: "./real-workflow-attempts/logs/v1-generator.log"
              event_fields: [step_name, duration_ms, json_parsable]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v1-generator.log"
              event_fields: [step_name, error_message]
    # ── STEP 4: Fix any issues and produce final output ────────────
    step_4_fix:
      generative_entity: "${models.coder-model}"
      depends_on: [step_3_validate]
      prompt: |
        You are a YAML fixer. You will receive a YAML workflow that may have issues, and a validation report.
        ORIGINAL YAML:
        {{step.step_2_generate.output}}
        VALIDATION REPORT:
        {{step.step_3_validate.output}}
        Fix ALL errors listed in the validation report. Common fixes:
        1. Strip any markdown code fences (```yaml or ```)
        2. Fix indentation (use 2 spaces consistently)
        3. Ensure all required fields exist
        4. Fix YAML syntax errors
        5. Remove any trailing content after the YAML ends
        Output the COMPLETE FIXED YAML. No markdown. No backticks. No explanations.
        First line must be: workflow_id:
      model_overrides:
        temperature: 0.1
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/v1-final-workflow.yml"
          - shell:
              command: "python3"
              args: ["./real-workflow-attempts/scripts/validate-yaml.py", "./real-workflow-attempts/outputs/v1-final-workflow.yml"]
              fail_on_error: false
          - log:
              to_file_path: "./real-workflow-attempts/logs/v1-generator.log"
              event_fields: [step_name, duration_ms, shell_output]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v1-generator.log"
              event_fields: [step_name, error_message]

workflow_id: meta_workflow_generator_v2
name: "Meta-Workflow Generator v2"
description: "Generate valid YAML workflows from prompts using in-context example + local SLMs"
version: "2.0.0"
author: "Whitt Execution Engine"
tags: [meta-workflow, yaml-generation, attempt-2]
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
workflow_execution_strategy:
  memory:
    model_lifecycle:
      unload_unused: true
agentic_workflow:
  steps:
    step_1_generate:
      generative_entity: "${models.coder-model}"
      prompt: |
        You must output a valid YAML workflow file. Follow the EXAMPLE exactly.
        TASK: Create a workflow that reads a Python source file, analyzes it for code quality issues, suggests fixes, and generates a summary report with metrics.
        EXAMPLE OUTPUT FORMAT (copy this structure exactly):
        ---
        workflow_id: python_code_quality_checker
        name: "Python Code Quality Checker"
        description: "Analyze Python files for quality issues and generate fix suggestions"
        version: "2.0.0"
        author: "Whitt Execution Engine"
        tags: [python, code-quality]
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
            step_1_read_file:
              generative_entity: "${models.target-model}"
              prompt: |
                Read and display the contents of the Python source file at the given path.
                Output the complete file contents with line numbers.
              model_overrides:
                temperature: 0.3
                max_tokens: 50000
              when:
                after_step_succeeds:
                  - save_to: "./outputs/step1-file-contents.txt"
                  - log:
                      to_file_path: "./logs/workflow.log"
                      event_fields: [step_name, duration_ms]
                after_step_fails:
                  - log:
                      to_file_path: "./logs/workflow.log"
                      event_fields: [step_name, error_message]
            step_2_analyze:
              generative_entity: "${models.target-model}"
              depends_on: [step_1_read_file]
              prompt: |
                Analyze the following Python code for quality issues. Check for:
                - Code style violations (PEP 8)
                - Potential bugs or errors
                - Performance issues
                - Security vulnerabilities
                INPUT CODE:
                {{step.step_1_read_file.output}}
                Output a numbered list of issues found.
              model_overrides:
                temperature: 0.4
                max_tokens: 50000
              when:
                after_step_succeeds:
                  - save_to: "./outputs/step2-analysis.txt"
                  - log:
                      to_file_path: "./logs/workflow.log"
                      event_fields: [step_name, duration_ms]
                after_step_fails:
                  - log:
                      to_file_path: "./logs/workflow.log"
                      event_fields: [step_name, error_message]
        ---
        Now generate the COMPLETE YAML for the TASK above. Output 4 steps: read_file, analyze_quality, suggest_fixes, generate_report.
        Each step must have a real prompt (not a placeholder). Each step after the first must use depends_on and {{step.X.output}}.
        RULES:
        - Output ONLY raw YAML. NO markdown fences. NO ```yaml```. NO ``` at start or end.
        - Steps MUST use MAP notation (step_name: followed by indented properties), NOT list notation (- step_name:)
        - First line must be: workflow_id:
        - Last character must be a letter or newline, NOT a backtick
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/v2-generated-workflow.yml"
          - log:
              to_file_path: "./real-workflow-attempts/logs/v2-generator.log"
              event_fields: [step_name, duration_ms, token_count]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v2-generator.log"
              event_fields: [step_name, error_message]
    step_2_validate:
      generative_entity: "${models.coder-model}"
      depends_on: [step_1_generate]
      prompt: |
        Review this YAML workflow. Check for errors and fix them.
        GENERATED YAML:
        {{step.step_1_generate.output}}
        Output the FIXED YAML. Rules:
        - NO markdown fences (no ```yaml or ```)
        - Steps MUST use MAP notation (step_name: not - step_name:)
        - All prompts must be real text, not placeholders like <prompt text here>
        - depends_on must reference actual step names
        - Template variables must use {{step.step_name.output}} format
        Output ONLY the corrected YAML. First line: workflow_id:
      model_overrides:
        temperature: 0.1
        max_tokens: 50000
      when:
        before_step_starts:
          - shell:
              command: "python3"
              args: ["./real-workflow-attempts/scripts/validate-yaml.py", "./real-workflow-attempts/outputs/v2-generated-workflow.yml"]
              fail_on_error: false
          - log:
              to_file_path: "./real-workflow-attempts/logs/v2-generator.log"
              event_fields: [step_name, shell_output]
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/v2-final-workflow.yml"
          - shell:
              command: "python3"
              args: ["./real-workflow-attempts/scripts/validate-yaml.py", "./real-workflow-attempts/outputs/v2-final-workflow.yml"]
              fail_on_error: false
          - log:
              to_file_path: "./real-workflow-attempts/logs/v2-generator.log"
              event_fields: [step_name, duration_ms, shell_output]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v2-generator.log"
              event_fields: [step_name, error_message]

workflow_id: meta_workflow_generator_v3
name: "Meta-Workflow Generator v3 - Multi-Phase Decomposition"
description: "3-phase meta-workflow: plan → generate sections → assemble+validate. Produces complex YAML workflows from prompts."
version: "3.0.0"
author: "Whitt Execution Engine"
tags: [meta-workflow, yaml-generation, attempt-3, multi-phase]
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
    # ── PHASE 1: PLAN ──────────────────────────────────────────────
    # Generate a structured plan: step names, dependencies, prompts, hooks
    step_1_plan:
      generative_entity: "${models.coder-model}"
      prompt: |
        You are a workflow architect. Given a task description, output a structured plan for a YAML agentic workflow.
        TASK: Create a workflow that takes a Rust source file, analyzes its architecture and design patterns, identifies code smells and anti-patterns, generates refactoring suggestions with before/after code examples, validates the suggestions compile, and produces a detailed refactoring roadmap document with priority rankings.
        Output a JSON plan with this exact structure:
        {
          "workflow_id": "snake_case_id",
          "name": "Human Readable Name",
          "description": "One line description",
          "tags": ["tag1", "tag2"],
          "steps": [
            {
              "name": "step_1_name",
              "purpose": "What this step does",
              "depends_on": [],
              "prompt_summary": "Brief description of the prompt for this step",
              "temperature": 0.3,
              "hooks": ["after_step_succeeds: save_to + log", "after_step_fails: log"]
            }
          ]
        }
        RULES:
        - Generate at least 6 steps
        - Create a non-trivial dependency graph (not just a chain — have parallel branches)
        - At least one step should have 2+ dependencies
        - Include at least one quality validation step
        - Include a final assembly/report step
        - Use temperatures between 0.1 and 0.7
        - Output ONLY valid JSON, no markdown fences
      model_overrides:
        temperature: 0.2
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/v3-plan.json"
          - log:
              to_file_path: "./real-workflow-attempts/logs/v3-generator.log"
              event_fields: [step_name, duration_ms, token_count]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v3-generator.log"
              event_fields: [step_name, error_message]
    # ── PHASE 2: GENERATE FULL YAML ───────────────────────────────
    # Takes the plan and generates complete YAML using example-based prompting
    step_2_generate:
      generative_entity: "${models.coder-model}"
      depends_on: [step_1_plan]
      prompt: |
        Generate a COMPLETE YAML agentic workflow file based on the plan below.
        PLAN:
        {{step.step_1_plan.output}}
        Follow this EXAMPLE STRUCTURE exactly (map notation, real prompts, proper hooks):
        ---
        workflow_id: python_code_quality_checker
        name: "Python Code Quality Checker"
        description: "Analyze Python files for quality issues and generate fix suggestions"
        version: "2.0.0"
        author: "Whitt Execution Engine"
        tags: [python, code-quality]
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
            step_1_read_file:
              generative_entity: "${models.target-model}"
              prompt: |
                Read and display the contents of the Python source file at the given path.
                Output the complete file contents with line numbers.
              model_overrides:
                temperature: 0.3
                max_tokens: 50000
              when:
                after_step_succeeds:
                  - save_to: "./outputs/step1-file-contents.txt"
                  - log:
                      to_file_path: "./logs/workflow.log"
                      event_fields: [step_name, duration_ms]
                after_step_fails:
                  - log:
                      to_file_path: "./logs/workflow.log"
                      event_fields: [step_name, error_message]
            step_2_analyze:
              generative_entity: "${models.target-model}"
              depends_on: [step_1_read_file]
              prompt: |
                Analyze the following Python code for quality issues. Check for:
                - Code style violations (PEP 8)
                - Potential bugs or errors
                - Performance issues
                - Security vulnerabilities
                INPUT CODE:
                {{step.step_1_read_file.output}}
                Output a numbered list of issues found.
              model_overrides:
                temperature: 0.4
                max_tokens: 50000
              when:
                after_step_succeeds:
                  - save_to: "./outputs/step2-analysis.txt"
                  - log:
                      to_file_path: "./logs/workflow.log"
                      event_fields: [step_name, duration_ms]
                after_step_fails:
                  - log:
                      to_file_path: "./logs/workflow.log"
                      event_fields: [step_name, error_message]
        ---
        NOW generate the COMPLETE YAML for the plan above.
        CRITICAL RULES:
        1. Output ONLY raw YAML. NO markdown fences. NO ```yaml```. NO ``` at start or end.
        2. Steps MUST use MAP notation (step_name: followed by indented properties), NOT list notation (- step_name:)
        3. First line must be: workflow_id:
        4. Every step MUST have a REAL, DETAILED prompt (not a placeholder)
        5. Every step after the first MUST use depends_on and {{step.X.output}}
        6. Every step MUST have when: with after_step_succeeds (save_to + log) and after_step_fails (log)
        7. Every step MUST have model_overrides with temperature and max_tokens
        8. Last character must be a letter or newline, NOT a backtick
        9. Use 50000 for max_tokens on every step
        10. The prompts must be detailed and specific — at least 3 lines each
      model_overrides:
        temperature: 0.15
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/v3-generated-raw.yml"
          - log:
              to_file_path: "./real-workflow-attempts/logs/v3-generator.log"
              event_fields: [step_name, duration_ms, token_count]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v3-generator.log"
              event_fields: [step_name, error_message]
    # ── PHASE 3: CLEAN + VALIDATE + FIX ───────────────────────────
    # Strip fences, fix common issues, validate structure
    step_3_clean_validate:
      generative_entity: "${models.reviewer-model}"
      depends_on: [step_2_generate]
      prompt: |
        You are a YAML validation and cleanup specialist. Your job is to take a generated YAML workflow and output a clean, valid version.
        INPUT YAML:
        {{step.step_2_generate.output}}
        Perform these steps:
        1. Strip any markdown code fences (```yaml or ```) from start and end
        2. Verify the file starts with "workflow_id:" (no leading whitespace or other text)
        3. Verify steps use MAP notation (step_name: not - step_name:)
        4. Verify every step has: generative_entity, prompt, model_overrides, when
        5. Verify every step has depends_on (except the first step)
        6. Verify template variables use {{step.step_name.output}} format
        7. Verify prompts are real text (not placeholders like <prompt text here>)
        8. Remove any trailing text after the last YAML line
        9. Ensure proper YAML indentation (2 spaces per level)
        Output ONLY the cleaned, validated YAML. No explanations. No markdown fences.
        First line: workflow_id:
        Last line: a YAML value, not a backtick.
      model_overrides:
        temperature: 0.05
        max_tokens: 50000
      when:
        before_step_starts:
          - shell:
              command: "python3"
              args: ["./real-workflow-attempts/scripts/validate-yaml.py", "./real-workflow-attempts/outputs/v3-generated-raw.yml"]
              fail_on_error: false
          - log:
              to_file_path: "./real-workflow-attempts/logs/v3-generator.log"
              event_fields: [step_name, shell_output]
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/v3-final-workflow.yml"
          - shell:
              command: "python3"
              args: ["./real-workflow-attempts/scripts/validate-yaml.py", "./real-workflow-attempts/outputs/v3-final-workflow.yml"]
              fail_on_error: false
          - log:
              to_file_path: "./real-workflow-attempts/logs/v3-generator.log"
              event_fields: [step_name, duration_ms, shell_output]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v3-generator.log"
              event_fields: [step_name, error_message]

workflow_id: meta_workflow_generator_v4
name: "Meta-Workflow Generator v4 - Improved Example"
description: "2-step meta-workflow with interpolation example + parallel branches. Produces complex YAML workflows."
version: "4.0.0"
author: "Whitt Execution Engine"
tags: [meta-workflow, yaml-generation, attempt-4]
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
    # Single generation with comprehensive example that includes interpolation
    step_1_generate:
      generative_entity: "${models.coder-model}"
      prompt: |
        Generate a COMPLETE YAML agentic workflow file for the following task:
        TASK: Create a workflow that takes a Rust source file, analyzes its architecture and design patterns, identifies code smells and anti-patterns, generates refactoring suggestions with before/after code examples, validates the suggestions compile, and produces a detailed refactoring roadmap document with priority rankings.
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
        - step_2_static_analysis depends_on: [step_1_read_code] and uses {{step.step_1_read_code.output}} in its prompt
        - step_3_security_scan ALSO depends_on: [step_1_read_code] (PARALLEL with step_2)
        - step_4_compile_report depends_on BOTH: [step_2_static_analysis, step_3_security_scan] and uses BOTH outputs
        - Every prompt is 5+ lines with specific instructions
        - Every step after step 1 uses {{step.X.output}} template interpolation
        NOW generate a COMPLETE YAML workflow for the TASK above.
        CRITICAL RULES:
        1. Output ONLY raw YAML. NO markdown fences. NO ```yaml```. NO ``` anywhere.
        2. First line: workflow_id:  Last line: a YAML value.
        3. Steps MUST use MAP notation (step_name: with indented properties), NOT list notation
        4. Generate at least 6 steps
        5. At least 2 steps must run in PARALLEL (share the same dependency)
        6. At least 1 step must depend on 2+ other steps
        7. Every step after the first MUST have depends_on and use {{step.X.output}} in prompt
        8. Every prompt must be 5+ lines with specific detailed instructions
        9. Every step MUST have: generative_entity, prompt, model_overrides, when
        10. Every step MUST have model_overrides with temperature (0.1-0.7) and max_tokens: 50000
        11. Every step MUST have when: with after_step_succeeds (save_to + log) and after_step_fails (log)
      model_overrides:
        temperature: 0.15
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/v4-generated-raw.yml"
          - log:
              to_file_path: "./real-workflow-attempts/logs/v4-generator.log"
              event_fields: [step_name, duration_ms, token_count]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v4-generator.log"
              event_fields: [step_name, error_message]
    # ── PHASE 2: CLEAN + VALIDATE ─────────────────────────────────
    # Strip fences, fix common issues, validate structure
    step_2_clean_validate:
      generative_entity: "${models.reviewer-model}"
      depends_on: [step_1_generate]
      prompt: |
        You are a YAML validation and cleanup specialist. Your job is to take a generated YAML workflow and output a clean, valid version.
        INPUT YAML:
        {{step.step_1_generate.output}}
        Perform these steps:
        1. Strip any markdown code fences (```yaml or ```) from start and end
        2. Verify the file starts with "workflow_id:" (no leading whitespace or other text)
        3. Verify steps use MAP notation (step_name: not - step_name:)
        4. Verify every step has: generative_entity, prompt, model_overrides, when
        5. Verify every step has depends_on (except the first step)
        6. Verify template variables use {{step.step_name.output}} format in prompts
        7. Verify prompts are real text (not placeholders like <prompt text here>)
        8. Verify at least one step has 2+ items in depends_on (parallel merge)
        9. Verify at least 2 steps share the same parent dependency (parallel branches)
        10. Remove any trailing text after the last YAML line
        11. Ensure proper YAML indentation (2 spaces per level)
        Output ONLY the cleaned, validated YAML. No explanations. No markdown fences.
        First line: workflow_id:
        Last line: a YAML value, not a backtick.
      model_overrides:
        temperature: 0.05
        max_tokens: 50000
      when:
        before_step_starts:
          - shell:
              command: "python3"
              args: ["./real-workflow-attempts/scripts/validate-yaml.py", "./real-workflow-attempts/outputs/v4-generated-raw.yml"]
              fail_on_error: false
          - log:
              to_file_path: "./real-workflow-attempts/logs/v4-generator.log"
              event_fields: [step_name, shell_output]
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/v4-final-workflow.yml"
          - shell:
              command: "python3"
              args: ["./real-workflow-attempts/scripts/validate-yaml.py", "./real-workflow-attempts/outputs/v4-final-workflow.yml"]
              fail_on_error: false
          - log:
              to_file_path: "./real-workflow-attempts/logs/v4-generator.log"
              event_fields: [step_name, duration_ms, shell_output]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v4-generator.log"
              event_fields: [step_name, error_message]

workflow_id: meta_workflow_generator_v4b
name: Meta-Workflow Generator v4b - Requirements Analysis
description: 2-step meta-workflow with interpolation example + parallel branches. Produces complex YAML workflows.
version: 4.0.0
author: Whitt Execution Engine
tags:
- meta-workflow
- yaml-generation
- attempt-4b
- requirements
schema_version: 2.0.0
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
    hosting:
      gpu_layers: 999
models:
  coder-model:
    name: Qwen2.5-Coder-3B-Instruct-Q8_0
    host:
      type: llama_cpp_with_vulkan
  reviewer-model:
    name: Qwen3-4B-Instruct-2507-Q4_K_M
    host:
      type: llama_cpp_with_vulkan
workflow_execution_strategy:
  memory:
    model_lifecycle:
      unload_unused: true
agentic_workflow:
  steps:
    step_1_generate:
      generative_entity: ${models.coder-model}
      prompt: "Generate a COMPLETE YAML agentic workflow file for the following task:\n\nTASK: Create a workflow that takes a natural language requirements document, extracts functional and non-functional\
        \ requirements, identifies contradictions and ambiguities, generates user stories with acceptance criteria, creates a test plan with coverage matrix, and produces a requirements traceability matrix\
        \ linking requirements to user stories to test cases.\n\nFollow this EXAMPLE exactly. Copy its structure, notation, and style:\n\n---\nworkflow_id: code_review_pipeline\nname: \"Code Review Pipeline\"\
        \ndescription: \"Automated code review with static analysis, security scan, and final report\"\nversion: \"2.0.0\"\nauthor: \"Whitt Execution Engine\"\ntags: [code-review, security, analysis]\n\
        schema_version: \"2.0.0\"\n\nproviders:\n  llama_cpp_with_vulkan:\n    config:\n      host: localhost\n      port: 8080\n    hosting:\n      gpu_layers: 999\n\nmodels:\n  \"target-model\":\n   \
        \ name: \"Qwen2.5-Coder-3B-Instruct-Q8_0\"\n    host:\n      type: llama_cpp_with_vulkan\n\nworkflow_execution_strategy:\n  memory:\n    model_lifecycle:\n      unload_unused: true\n\nagentic_workflow:\n\
        \n  steps:\n\n    step_1_read_code:\n      generative_entity: \"${models.target-model}\"\n      prompt: |\n        Read and display the complete contents of the source file provided.\n        Output\
        \ every line with line numbers prefixed.\n        Preserve all whitespace, comments, and formatting exactly.\n        If the file is very long, output the first 500 lines.\n      model_overrides:\n\
        \        temperature: 0.2\n        max_tokens: 50000\n      when:\n        after_step_succeeds:\n          - save_to: \"./outputs/step1-source-code.txt\"\n          - log:\n              to_file_path:\
        \ \"./logs/workflow.log\"\n              event_fields: [step_name, duration_ms]\n        after_step_fails:\n          - log:\n              to_file_path: \"./logs/workflow.log\"\n              event_fields:\
        \ [step_name, error_message]\n\n    step_2_static_analysis:\n      generative_entity: \"${models.target-model}\"\n      depends_on: [step_1_read_code]\n      prompt: |\n        Perform a thorough\
        \ static analysis of the following source code.\n        Check for these specific issues:\n        - Dead code and unreachable branches\n        - Unused variables and imports\n        - Type mismatches\
        \ and potential runtime errors\n        - Performance bottlenecks (O(n\xB2) loops, unnecessary allocations)\n        - Naming convention violations\n\n        SOURCE CODE TO ANALYZE:\n        {{step.step_1_read_code.output}}\n\
        \n        Output a numbered list of issues with file location, severity (critical/warning/info),\n        and a brief explanation of each issue.\n      model_overrides:\n        temperature: 0.3\n\
        \        max_tokens: 50000\n      when:\n        after_step_succeeds:\n          - save_to: \"./outputs/step2-static-analysis.txt\"\n          - log:\n              to_file_path: \"./logs/workflow.log\"\
        \n              event_fields: [step_name, duration_ms]\n        after_step_fails:\n          - log:\n              to_file_path: \"./logs/workflow.log\"\n              event_fields: [step_name,\
        \ error_message]\n\n    step_3_security_scan:\n      generative_entity: \"${models.target-model}\"\n      depends_on: [step_1_read_code]\n      prompt: |\n        Perform a security scan of the\
        \ following source code.\n        Look for these vulnerability categories:\n        - SQL injection and command injection\n        - Cross-site scripting (XSS) vectors\n        - Insecure cryptographic\
        \ usage\n        - Hardcoded secrets or credentials\n        - Path traversal vulnerabilities\n\n        SOURCE CODE TO SCAN:\n        {{step.step_1_read_code.output}}\n\n        Output each finding\
        \ with: vulnerability type, location (line number),\n        severity (critical/high/medium/low), and remediation advice.\n      model_overrides:\n        temperature: 0.2\n        max_tokens: 50000\n\
        \      when:\n        after_step_succeeds:\n          - save_to: \"./outputs/step3-security-scan.txt\"\n          - log:\n              to_file_path: \"./logs/workflow.log\"\n              event_fields:\
        \ [step_name, duration_ms]\n        after_step_fails:\n          - log:\n              to_file_path: \"./logs/workflow.log\"\n              event_fields: [step_name, error_message]\n\n    step_4_compile_report:\n\
        \      generative_entity: \"${models.target-model}\"\n      depends_on: [step_2_static_analysis, step_3_security_scan]\n      prompt: |\n        Compile a comprehensive code review report from the\
        \ analysis results below.\n\n        STATIC ANALYSIS RESULTS:\n        {{step.step_2_static_analysis.output}}\n\n        SECURITY SCAN RESULTS:\n        {{step.step_3_security_scan.output}}\n\n\
        \        Create a markdown report with:\n        1. Executive summary with total issue count by severity\n        2. Critical issues section (must fix before merge)\n        3. Warnings section\
        \ (should fix)\n        4. Info section (nice to fix)\n        5. Prioritized action items list\n\n        Format each issue as: [SEVERITY] File:Line - Description\n      model_overrides:\n    \
        \    temperature: 0.4\n        max_tokens: 50000\n      when:\n        after_step_succeeds:\n          - save_to: \"./outputs/step4-final-report.md\"\n          - log:\n              to_file_path:\
        \ \"./logs/workflow.log\"\n              event_fields: [step_name, duration_ms]\n        after_step_fails:\n          - log:\n              to_file_path: \"./logs/workflow.log\"\n              event_fields:\
        \ [step_name, error_message]\n---\n\nIMPORTANT PATTERNS IN THE EXAMPLE ABOVE:\n- step_2_static_analysis depends_on: [step_1_read_code] and uses {{step.step_1_read_code.output}} in its prompt\n-\
        \ step_3_security_scan ALSO depends_on: [step_1_read_code] (PARALLEL with step_2)\n- step_4_compile_report depends_on BOTH: [step_2_static_analysis, step_3_security_scan] and uses BOTH outputs\n\
        - Every prompt is 5+ lines with specific instructions\n- Every step after step 1 uses {{step.X.output}} template interpolation\n\nNOW generate a COMPLETE YAML workflow for the TASK above.\n\nCRITICAL\
        \ RULES:\n1. Output ONLY raw YAML. NO markdown fences. NO ```yaml```. NO ``` anywhere.\n2. First line: workflow_id:  Last line: a YAML value.\n3. Steps MUST use MAP notation (step_name: with indented\
        \ properties), NOT list notation\n4. Generate at least 6 steps\n5. At least 2 steps must run in PARALLEL (share the same dependency)\n6. At least 1 step must depend on 2+ other steps\n7. Every step\
        \ after the first MUST have depends_on and use {{step.X.output}} in prompt\n8. Every prompt must be 5+ lines with specific detailed instructions\n9. Every step MUST have: generative_entity, prompt,\
        \ model_overrides, when\n10. Every step MUST have model_overrides with temperature (0.1-0.7) and max_tokens: 50000\n11. Every step MUST have when: with after_step_succeeds (save_to + log) and after_step_fails\
        \ (log)\n"
      model_overrides:
        temperature: 0.15
        max_tokens: 50000
      when:
        after_step_succeeds:
        - save_to: ./real-workflow-attempts/outputs/v4b-generated-raw.yml
        - log:
            to_file_path: ./real-workflow-attempts/logs/v4b-generator.log
            event_fields:
            - step_name
            - duration_ms
            - token_count
        after_step_fails:
        - log:
            to_file_path: ./real-workflow-attempts/logs/v4b-generator.log
            event_fields:
            - step_name
            - error_message
    step_2_clean_validate:
      generative_entity: ${models.reviewer-model}
      depends_on:
      - step_1_generate
      prompt: 'You are a YAML validation and cleanup specialist. Your job is to take a generated YAML workflow and output a clean, valid version.
        INPUT YAML:
        {{step.step_1_generate.output}}
        Perform these steps:
        1. Strip any markdown code fences (```yaml or ```) from start and end
        2. Verify the file starts with "workflow_id:" (no leading whitespace or other text)
        3. Verify steps use MAP notation (step_name: not - step_name:)
        4. Verify every step has: generative_entity, prompt, model_overrides, when
        5. Verify every step has depends_on (except the first step)
        6. Verify template variables use {{step.step_name.output}} format in prompts
        7. Verify prompts are real text (not placeholders like <prompt text here>)
        8. Verify at least one step has 2+ items in depends_on (parallel merge)
        9. Verify at least 2 steps share the same parent dependency (parallel branches)
        10. Remove any trailing text after the last YAML line
        11. Ensure proper YAML indentation (2 spaces per level)
        Output ONLY the cleaned, validated YAML. No explanations. No markdown fences.
        First line: workflow_id:
        Last line: a YAML value, not a backtick.
        '
      model_overrides:
        temperature: 0.05
        max_tokens: 50000
      when:
        before_step_starts:
        - shell:
            command: python3
            args:
            - ./real-workflow-attempts/scripts/validate-yaml.py
            - ./real-workflow-attempts/outputs/v4-generated-raw.yml
            fail_on_error: false
        - log:
            to_file_path: ./real-workflow-attempts/logs/v4b-generator.log
            event_fields:
            - step_name
            - shell_output
        after_step_succeeds:
        - save_to: ./real-workflow-attempts/outputs/v4b-final-workflow.yml
        - shell:
            command: python3
            args:
            - ./real-workflow-attempts/scripts/validate-yaml.py
            - ./real-workflow-attempts/outputs/v4-final-workflow.yml
            fail_on_error: false
        - log:
            to_file_path: ./real-workflow-attempts/logs/v4b-generator.log
            event_fields:
            - step_name
            - duration_ms
            - shell_output
        after_step_fails:
        - log:
            to_file_path: ./real-workflow-attempts/logs/v4b-generator.log
            event_fields:
            - step_name
            - error_message

workflow_id: meta_workflow_generator_v5
name: "Meta-Workflow Generator v5 - Robust"
description: "3-step meta-workflow: generate YAML → clean/strip fences → validate+fix. Produces complex, schema-valid YAML workflows with guaranteed template interpolation."
version: "5.0.0"
author: "Whitt Execution Engine"
tags: [meta-workflow, yaml-generation, attempt-5, robust]
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
  hardcoded_values:
    target_task: "Create a workflow that takes a database schema definition (SQL CREATE TABLE statements), analyzes the schema for normalization issues (1NF/2NF/3NF/BCNF violations), identifies missing indexes and foreign key constraints, generates optimized DDL migration scripts, validates the migrations are syntactically correct, and produces a comprehensive database optimization report with before/after schema diagrams."
  steps:
    # ── PHASE 1: GENERATE ──────────────────────────────────────────
    # Generate complete YAML workflow with comprehensive example
    step_1_generate:
      generative_entity: "${models.coder-model}"
      prompt: |
        Generate a COMPLETE YAML agentic workflow file for the following task:
        TASK: {{hardcoded_values.target_task}}
        You MUST follow this EXAMPLE exactly. Copy its structure, notation, indentation, and style precisely:
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
        CRITICAL PATTERNS you MUST replicate from the example above:
        PATTERN A: step_2_static_analysis has depends_on: [step_1_read_code] AND its prompt contains {{step.step_1_read_code.output}}
        PATTERN B: step_3_security_scan ALSO has depends_on: [step_1_read_code] — this makes step_2 and step_3 run in PARALLEL
        PATTERN C: step_4_compile_report has depends_on: [step_2_static_analysis, step_3_security_scan] AND its prompt contains BOTH {{step.step_2_static_analysis.output}} AND {{step.step_3_security_scan.output}}
        PATTERN D: Every single step after step_1 has {{step.PARENT_STEP_NAME.output}} inside its prompt text
        PATTERN E: Every prompt is 5+ lines with specific detailed instructions
        NOW generate a COMPLETE YAML workflow for the TASK above.
        ABSOLUTE REQUIREMENTS (you will FAIL if you miss any):
        1. Output ONLY raw YAML. NO markdown fences. NO ```yaml```. NO ``` anywhere.
        2. First line must be: workflow_id:
        3. Steps MUST use MAP notation (step_name: with indented properties), NOT list notation with dashes
        4. Generate at least 6 steps
        5. At least 2 steps must share the same depends_on parent (PARALLEL branches)
        6. At least 1 step must have 2+ items in depends_on (MERGE point)
        7. EVERY step after the first one MUST contain {{step.PARENT_STEP_NAME.output}} somewhere in its prompt text
        8. EVERY prompt must be 5+ lines with specific detailed instructions about what to do
        9. EVERY step MUST have these keys: generative_entity, prompt, model_overrides, when
        10. model_overrides MUST have temperature (0.1 to 0.7 only) and max_tokens: 50000
        11. when MUST have after_step_succeeds (with save_to and log) and after_step_fails (with log)
        12. Do NOT use temperature above 0.7
      model_overrides:
        temperature: 0.15
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/v5-generated-raw.yml"
          - log:
              to_file_path: "./real-workflow-attempts/logs/v5-generator.log"
              event_fields: [step_name, duration_ms, token_count]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v5-generator.log"
              event_fields: [step_name, error_message]
    # ── PHASE 2: CLEAN + STRIP FENCES ──────────────────────────────
    # Strip markdown fences, fix notation, ensure structure
    step_2_clean:
      generative_entity: "${models.reviewer-model}"
      depends_on: [step_1_generate]
      prompt: |
        You are a YAML cleanup specialist. Take the generated YAML below and output ONLY a clean version.
        INPUT:
        {{step.step_1_generate.output}}
        Perform these exact cleanups:
        1. Strip ALL markdown code fences (```yaml or ```) from start and end
        2. Strip any leading/trailing whitespace before "workflow_id:"
        3. Remove any trailing text after the last YAML line
        4. Ensure steps use MAP notation (step_name: with indented properties, NOT - step_name:)
        5. Ensure proper YAML indentation (2 spaces per level)
        6. Ensure the file starts with "workflow_id:" and ends with a YAML value
        Output ONLY the cleaned YAML. No explanations. No fences. No commentary.
        First character: 'w' (from workflow_id)
        Last character: a YAML value, NOT a backtick.
      model_overrides:
        temperature: 0.05
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/v5-cleaned.yml"
          - log:
              to_file_path: "./real-workflow-attempts/logs/v5-generator.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v5-generator.log"
              event_fields: [step_name, error_message]
    # ── PHASE 3: VALIDATE + FIX ────────────────────────────────────
    # Validate the cleaned YAML and fix any remaining issues
    step_3_validate_fix:
      generative_entity: "${models.reviewer-model}"
      depends_on: [step_2_clean]
      prompt: |
        You are a YAML workflow validator and fixer. Your job is to ensure the workflow is COMPLETE and CORRECT.
        CLEANED YAML:
        {{step.step_2_clean.output}}
        Check ALL of these requirements and FIX any that are missing or wrong:
        1. File starts with "workflow_id:" — if not, remove everything before it
        2. Has "providers:" section with llama_cpp_with_vulkan provider containing config with host and port
        3. Has "models:" section with at least one model with name and host.type
        4. Has "agentic_workflow:" with "steps:" section
        5. All steps use MAP notation (step_name: with indented properties, NOT - step_name:)
        6. Every step has: generative_entity, prompt, model_overrides, when
        7. Every step except the first has "depends_on:" with at least one parent step name
        8. CRITICAL: Every step that has depends_on MUST contain {{step.PARENT_STEP_NAME.output}} in its prompt. If any step is missing this, ADD it. Insert a line like "INPUT FROM PREVIOUS STEP:" followed by {{step.parent_step_name.output}} into the prompt.
        9. Every prompt is 5+ lines with real specific instructions (not placeholders)
        10. Every model_overrides has temperature between 0.1 and 0.7, and max_tokens: 50000
        11. Every when section has after_step_succeeds with save_to and log, and after_step_fails with log
        12. At least 2 steps share the same depends_on parent (parallel branches)
        13. At least 1 step has 2+ items in depends_on (merge point)
        After checking and fixing, output ONLY the final valid YAML.
        No explanations. No markdown fences. No commentary.
        First character: 'w' (from workflow_id).
        Last character: a YAML value, NOT a backtick.
      model_overrides:
        temperature: 0.05
        max_tokens: 50000
      when:
        before_step_starts:
          - shell:
              command: "python3"
              args: ["./real-workflow-attempts/scripts/validate-yaml.py", "./real-workflow-attempts/outputs/v5-cleaned.yml"]
              fail_on_error: false
          - log:
              to_file_path: "./real-workflow-attempts/logs/v5-generator.log"
              event_fields: [step_name, shell_output]
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/v5-final-workflow.yml"
          - shell:
              command: "python3"
              args: ["./real-workflow-attempts/scripts/validate-yaml.py", "./real-workflow-attempts/outputs/v5-final-workflow.yml"]
              fail_on_error: false
          - log:
              to_file_path: "./real-workflow-attempts/logs/v5-generator.log"
              event_fields: [step_name, duration_ms, shell_output]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v5-generator.log"
              event_fields: [step_name, error_message]

workflow_id: meta_workflow_generator_v6
name: "Meta-Workflow Generator v6 - Robust 3-Step"
description: "3-step meta-workflow: generate → clean → validate/fix. Task inlined in prompt. Produces complex YAML workflows."
version: "6.0.0"
author: "Whitt Execution Engine"
tags: [meta-workflow, yaml-generation, attempt-6]
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
    # Task is INLINED directly in prompt (no hardcoded_values)
    step_1_generate:
      generative_entity: "${models.coder-model}"
      prompt: |
        Generate a COMPLETE YAML agentic workflow file for the following task:
        TASK: Create a database schema analysis and optimization workflow that reads SQL schema definitions from multiple tables, identifies normalization violations (1NF/2NF/3NF), detects missing indexes for common query patterns, generates ALTER TABLE statements for optimization, validates the SQL syntax of generated statements, and produces a comprehensive database optimization report with before/after schema comparisons and migration steps ordered by dependency.
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
        7. Every step after the first MUST have depends_on and use {{step.X.output}} in prompt
        8. Every prompt must be 5+ lines with specific detailed instructions
        9. Every step MUST have: generative_entity, prompt, model_overrides, when
        10. Every step MUST have model_overrides with temperature (0.1-0.7) and max_tokens: 50000
        11. Every step MUST have when: with after_step_succeeds (save_to + log) and after_step_fails (log)
        12. The workflow MUST be about DATABASE SCHEMA ANALYSIS AND OPTIMIZATION, not code review
      model_overrides:
        temperature: 0.15
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/v6-generated-raw.yml"
          - log:
              to_file_path: "./real-workflow-attempts/logs/v6-generator.log"
              event_fields: [step_name, duration_ms, token_count]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v6-generator.log"
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
          - save_to: "./real-workflow-attempts/outputs/v6-cleaned.yml"
          - log:
              to_file_path: "./real-workflow-attempts/logs/v6-generator.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v6-generator.log"
              event_fields: [step_name, error_message]
    # ── PHASE 3: VALIDATE + FIX ────────────────────────────────────
    # Checks structural correctness and adds missing interpolation
    step_3_validate_fix:
      generative_entity: "${models.reviewer-model}"
      depends_on: [step_2_clean]
      prompt: |
        You are a YAML workflow validation and repair specialist. Validate the workflow below and fix any issues.
        INPUT WORKFLOW:
        {{step.step_2_clean.output}}
        CHECK AND FIX THESE ISSUES:
        1. Verify steps use MAP notation (step_name: with indented properties), NOT list notation (- step_name:)
        2. Verify every step has: generative_entity, prompt, model_overrides, when
        3. Verify every step (except first) has depends_on with at least one step name
        4. Verify template variables use {{step.step_name.output}} format in prompts
        5. If any step after the first is MISSING {{step.X.output}} in its prompt, ADD this line:
           "INPUT FROM PREVIOUS STEP:" followed by "{{step.APPROPRIATE_STEP_NAME.output}}"
           where APPROPRIATE_STEP_NAME is the first step in that step's depends_on list
        6. Verify prompts are real detailed text (not placeholders like <prompt text here>)
        7. Verify at least one step has 2+ items in depends_on (parallel merge)
        8. Verify at least 2 steps share the same parent dependency (parallel branches)
        9. Verify model_overrides has temperature (0.1-0.7) and max_tokens: 50000
        10. Verify every step has when: with after_step_succeeds (save_to + log) and after_step_fails (log)
        11. Ensure proper YAML indentation (2 spaces per level)
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
              args: ["./real-workflow-attempts/scripts/validate-yaml.py", "./real-workflow-attempts/outputs/v6-cleaned.yml"]
              fail_on_error: false
          - log:
              to_file_path: "./real-workflow-attempts/logs/v6-generator.log"
              event_fields: [step_name, shell_output]
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/v6-final-workflow.yml"
          - shell:
              command: "python3"
              args: ["./real-workflow-attempts/scripts/validate-yaml.py", "./real-workflow-attempts/outputs/v6-final-workflow.yml"]
              fail_on_error: false
          - log:
              to_file_path: "./real-workflow-attempts/logs/v6-generator.log"
              event_fields: [step_name, duration_ms, shell_output]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v6-generator.log"
              event_fields: [step_name, error_message]

workflow_id: meta_workflow_generator_v7
name: "Meta-Workflow Generator v7 - Robust Interpolation"
description: "3-step meta-workflow with strengthened validate step that ensures ALL steps have interpolation. Task inlined in prompt."
version: "7.0.0"
author: "Whitt Execution Engine"
tags: [meta-workflow, yaml-generation, attempt-7]
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
        TASK: Create an API integration testing workflow that reads OpenAPI/Swagger specification documents, generates HTTP test requests for each endpoint (including edge cases like auth failures, rate limiting, malformed payloads), executes the tests against a running API server, validates response schemas against the spec, checks response times against SLA thresholds, and produces a detailed test report with pass/fail status per endpoint, response time charts, and a summary of spec compliance coverage.
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
        12. The workflow MUST be about API INTEGRATION TESTING, not code review
      model_overrides:
        temperature: 0.15
        max_tokens: 50000
      when:
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/v7-generated-raw.yml"
          - log:
              to_file_path: "./real-workflow-attempts/logs/v7-generator.log"
              event_fields: [step_name, duration_ms, token_count]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v7-generator.log"
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
          - save_to: "./real-workflow-attempts/outputs/v7-cleaned.yml"
          - log:
              to_file_path: "./real-workflow-attempts/logs/v7-generator.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v7-generator.log"
              event_fields: [step_name, error_message]
    # ── PHASE 3: VALIDATE + FIX ────────────────────────────────────
    # Strengthened to ensure ALL steps get interpolation
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
        where DEPENDENCY_STEP_NAME is the first entry in that step's depends_on list.
        If the step depends on multiple steps, include ALL of them:
            INPUT FROM STEP A:
            {{step.step_A_name.output}}
            INPUT FROM STEP B:
            {{step.step_B_name.output}}
        Do NOT skip this check. Apply it to EVERY step that has depends_on.
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
              args: ["./real-workflow-attempts/scripts/validate-yaml.py", "./real-workflow-attempts/outputs/v7-cleaned.yml"]
              fail_on_error: false
          - log:
              to_file_path: "./real-workflow-attempts/logs/v7-generator.log"
              event_fields: [step_name, shell_output]
        after_step_succeeds:
          - save_to: "./real-workflow-attempts/outputs/v7-final-workflow.yml"
          - shell:
              command: "python3"
              args: ["./real-workflow-attempts/scripts/validate-yaml.py", "./real-workflow-attempts/outputs/v7-final-workflow.yml"]
              fail_on_error: false
          - log:
              to_file_path: "./real-workflow-attempts/logs/v7-generator.log"
              event_fields: [step_name, duration_ms, shell_output]
        after_step_fails:
          - log:
              to_file_path: "./real-workflow-attempts/logs/v7-generator.log"
              event_fields: [step_name, error_message]

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