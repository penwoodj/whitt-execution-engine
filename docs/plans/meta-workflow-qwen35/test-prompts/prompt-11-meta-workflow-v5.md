<!--
Source Session: ses_21eda916dffexLBSamby9C941e
Message Length: 18572 characters
YAML Sections: 12
Embedded Prompts: 9
Agentic Keywords: 10
Complexity: HIGH
Source Files: meta-workflow-v5.yml
-->

workflow_id: meta_workflow_generator_v5
name: "Meta-Workflow Generator v5"
description: "Context-loaded, self-reviewing meta-workflow with iterative refinement for high-quality workflow generation."
version: "1.0.0"
author: "Whitt Execution Engine"
tags: [meta-workflow, yaml-generation, context-loaded, self-review, iterative]
schema_version: "2.0.0"
min_schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
models:
  "planner":
    name: "Qwen3-4B-Instruct-2507-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "coder":
    name: "Qwen2.5-Coder-3B-Instruct-Q8_0"
    host:
      type: llama_cpp_with_vulkan
workflow_execution_strategy:
  memory:
    model_lifecycle:
      unload_unused: true
agentic_workflow:
  when:
    after_workflow:
      - log:
          event_fields: [workflow_id, total_steps, succeeded, failed]
  steps:
    # ============================================================
    # PHASE 1: CONTEXT LOADING
    # Shell hooks inject schema and example into bookmark context
    # ============================================================
    step_01_load_schema:
      generative_entity: "${models.planner}"
      prompt: |
        You are a workflow schema loader. Read the schema excerpt below and confirm you understand it.
        SCHEMA EXCERPT:
        {{bookmarks.schema_context.stdout}}
        Confirm by listing the top-level keys you see. Be brief.
      model_overrides:
        temperature: 0.1
        max_tokens: 200
      when:
        before_step_starts:
          - shell:
              command: "head -150 docs/schema/unified-workflow-schema.yml"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true
          - bookmark:
              detailed:
                path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/01-schema-context.txt"
        after_step_succeeds:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta-v5.log"
              event_fields: [step_name, duration_ms]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta-v5.log"
              event_fields: [step_name, error_message]
              level: error
    step_02_load_example:
      generative_entity: "${models.planner}"
      depends_on: [step_01_load_schema]
      prompt: |
        You are studying an example workflow. Read it carefully.
        EXAMPLE WORKFLOW:
        {{bookmarks.example_context.stdout}}
        This is a CORRECT workflow. Note the structure:
        - providers with llama_cpp_with_vulkan
        - models with worker model
        - agentic_workflow.steps with when: hooks on EVERY step
        - before_step_starts with shell for file-reading steps
        - after_step_succeeds with save_to and log on EVERY step
        - after_step_fails with log on EVERY step
        Confirm you understand the pattern. Be brief.
      model_overrides:
        temperature: 0.1
        max_tokens: 200
      when:
        before_step_starts:
          - shell:
              command: "cat docs/benchmarks/workflows/live-test-ministral-3b.yml"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true
          - bookmark:
              detailed:
                path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/02-example-context.txt"
        after_step_succeeds:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta-v5.log"
              event_fields: [step_name, duration_ms]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta-v5.log"
              event_fields: [step_name, error_message]
              level: error
    # ============================================================
    # PHASE 2: ANALYSIS AND PLANNING
    # ============================================================
    step_03_classify_and_decompose:
      generative_entity: "${models.planner}"
      depends_on: [step_02_load_example]
      prompt: |
        You are a workflow architect. Analyze the task and produce a complete step plan.
        TASK: __TASK_PLACEHOLDER__
        First, classify complexity:
        SIMPLE = read file, run command, basic transform (2-3 steps)
        MEDIUM = multiple files, filtering, aggregation, formatting (4-6 steps)
        COMPLEX = multi-phase pipeline, cross-references, parallel branches (7+ steps)
        Then, break the task into atomic actions:
        - Each action = ONE operation (read file via shell, transform via LLM, format via LLM, save via hook)
        - File reading = shell step with before_step_starts shell action
        - All other actions = LLM steps (no before_step_starts shell action)
        - Use as few steps as possible — combine related operations
        For EACH action, output EXACTLY:
        STEP_NAME: step_XX_<descriptive_name> | DEPENDS: <step_name or "none"> | TYPE: <shell|llm> | SHELL_CMD: <command or "none"> | GOAL: <one sentence what this step does>
        Output ONLY step definitions. No other text.
      model_overrides:
        temperature: 0.3
        max_tokens: 2000
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/03-step-plan.txt"
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta-v5.log"
              event_fields: [step_name, duration_ms, token_count]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta-v5.log"
              event_fields: [step_name, error_message]
              level: error
    # ============================================================
    # PHASE 3: PROMPT ENGINEERING
    # ============================================================
    step_04_write_prompts:
      generative_entity: "${models.planner}"
      depends_on: [step_03_classify_and_decompose]
      prompt: |
        You are a prompt engineer for agentic workflow steps. Write detailed prompts for EACH step.
        ORIGINAL TASK: __TASK_PLACEHOLDER__
        STEP PLAN:
        {{step.step_03_classify_and_decompose.output}}
        REFERENCE EXAMPLE (correct prompt style):
        {{bookmarks.example_context.stdout}}
        For EACH step, write a prompt block:
        --- STEP: step_XX_name ---
        Given the <context from previous step or task>, <specific action to perform>.
        <Detailed instructions for what to do with the input data>
        <Exact output format specification>
        <Example of expected output>
        Output ONLY the result. No explanations. No process descriptions.
        --- END ---
        CRITICAL RULES:
        1. TYPE: shell steps MUST include: "Here is the data:\n{{bookmarks.shell_output.stdout}}"
        2. TYPE: llm steps that depend on previous steps MUST include: "{{step.PREVIOUS_STEP.output}}"
        3. EVERY prompt MUST end with: "Output ONLY the result. No explanations. No process descriptions."
        4. Include a CONCRETE output format example in EVERY prompt
        5. Be specific about what data to extract/transform/compute
        6. Use phrases: "Given the <source>, <action>", "From the <data>, output <format>"
        Output ALL step prompts separated by --- STEP: and --- END --- markers.
      model_overrides:
        temperature: 0.3
        max_tokens: 4000
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/04-prompts.txt"
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta-v5.log"
              event_fields: [step_name, duration_ms, token_count]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta-v5.log"
              event_fields: [step_name, error_message]
              level: error
    # ============================================================
    # PHASE 4: YAML ASSEMBLY
    # ============================================================
    step_05_assemble_yaml:
      generative_entity: "${models.coder}"
      depends_on: [step_03_classify_and_decompose, step_04_write_prompts]
      prompt: |
        You are a YAML code generator. Assemble a complete workflow YAML file.
        STEP PLAN:
        {{step.step_03_classify_and_decompose.output}}
        STEP PROMPTS:
        {{step.step_04_write_prompts.output}}
        OUTPUT THE COMPLETE YAML. First line MUST be "workflow_id:". No markdown fences.
        EXACT HEADER (copy verbatim, change only workflow_id and name):
        workflow_id: generated-workflow
        name: "Generated Workflow"
        description: "Auto-generated workflow"
        version: "1.0.0"
        schema_version: "2.0.0"
        min_schema_version: "2.0.0"
        providers:
          llama_cpp_with_vulkan:
            config:
              host: localhost
              port: 8080
        models:
          "worker":
            name: "Qwen2.5-Coder-3B-Instruct-Q8_0.gguf"
            host:
              type: llama_cpp_with_vulkan
        workflow_execution_strategy:
          memory:
            model_lifecycle:
              unload_unused: true
        agentic_workflow:
          steps:
        EXACT STEP TEMPLATE (copy for EVERY step):
            step_XX_name:
              generative_entity: "${models.worker}"
              depends_on: [step_YY_prev]
              prompt: |
                <prompt from STEP PROMPTS>
              model_overrides:
                temperature: 0.3
                max_tokens: 50000
              when:
                after_step_succeeds:
                  - save_to: "./docs/benchmarks/outputs/output/step_XX_name-output.txt"
                  - log:
                      to_file_path: "./docs/benchmarks/outputs/logs/workflow.log"
                      event_fields: [step_name, duration_ms, token_count]
                      level: info
                after_step_fails:
                  - log:
                      to_file_path: "./docs/benchmarks/outputs/logs/workflow.log"
                      event_fields: [step_name, error_message]
                      level: error
        FOR SHELL STEPS ONLY, add before_step_starts INSIDE when: block (before after_step_succeeds):
              when:
                before_step_starts:
                  - shell:
                      command: "ACTUAL_COMMAND"
                      args: []
                      working_dir: "/home/jon/code/whitt-execution-engine"
                      fail_on_error: false
                after_step_succeeds:
                  ...same as above...
        RULES:
        1. First step has NO depends_on field at all (remove it entirely)
        2. TYPE: shell steps → include before_step_starts with shell action
        3. TYPE: llm steps → NO before_step_starts block at all
        4. EVERY step has after_step_succeeds with save_to AND log
        5. EVERY step has after_step_fails with log
        6. All output paths use ./docs/benchmarks/outputs/output/ or ./docs/benchmarks/outputs/logs/
        7. No markdown fences (no ```)
        8. File starts with "workflow_id:" on line 1
        9. Step names use zero-padding: step_01, step_02
        10. Prompts use pipe | syntax
        11. Remove --- STEP: and --- END --- markers from prompts
        Output ONLY raw YAML. First line: workflow_id:
      model_overrides:
        temperature: 0.05
        max_tokens: 10000
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/05-assembled.yml"
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta-v5.log"
              event_fields: [step_name, duration_ms, token_count]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta-v5.log"
              event_fields: [step_name, error_message]
              level: error
    # ============================================================
    # PHASE 5: SELF-REVIEW (NEW IN V5)
    # The coder reviews its own output against a checklist
    # ============================================================
    step_06_self_review:
      generative_entity: "${models.coder}"
      depends_on: [step_05_assemble_yaml]
      prompt: |
        You are a YAML quality reviewer. Review the following generated workflow YAML.
        GENERATED YAML:
        {{step.step_05_assemble_yaml.output}}
        Check EACH of these criteria. For each, write PASS or FAIL with reason:
        1. YAML SYNTAX: Does it parse as valid YAML? (correct indentation, no tab characters)
        2. HEADER COMPLETE: Has workflow_id, name, schema_version, providers, models, agentic_workflow?
        3. PROVIDERS CORRECT: providers.llama_cpp_with_vulkan.config.host and port exist?
        4. MODELS CORRECT: models has "worker" with name ending in .gguf and host.type?
        5. STEPS UNDER agentic_workflow.steps: All steps nested correctly (not top-level)?
        6. FIRST STEP: First step has NO depends_on field?
        7. DEPENDENCY CHAIN: Each step (except first) depends on a previous step that exists?
        8. TEMPLATE REFERENCES: Steps with depends_on reference previous output via {{step.X.output}}?
        9. EVERY STEP HAS when: block with after_step_succeeds (save_to + log)?
        10. EVERY STEP HAS when: block with after_step_fails (log)?
        11. SHELL STEPS ONLY have before_step_starts with shell action?
        12. LLM STEPS have NO before_step_starts block?
        13. NO markdown fences (no ```) anywhere?
        14. File starts with "workflow_id:" on line 1?
        15. Every prompt ends with "Output ONLY the result" directive?
        16. save_to paths use ./docs/benchmarks/outputs/output/ format?
        17. log paths use ./docs/benchmarks/outputs/logs/ format?
        18. No duplicate step names?
        19. No empty depends_on arrays (remove field entirely for first step)?
        After reviewing, list ONLY the issues found. If all pass, output "ALL CHECKS PASSED".
        Format each issue as:
        ISSUE: <check_number> - <description of what's wrong>
        FIX: <what needs to change>
      model_overrides:
        temperature: 0.1
        max_tokens: 2000
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/06-review.txt"
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta-v5.log"
              event_fields: [step_name, duration_ms, token_count]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta-v5.log"
              event_fields: [step_name, error_message]
              level: error
    # ============================================================
    # PHASE 6: FIX ISSUES (NEW IN V5)
    # Apply corrections from self-review
    # ============================================================
    step_07_fix_issues:
      generative_entity: "${models.coder}"
      depends_on: [step_06_self_review, step_05_assemble_yaml]
      prompt: |
        You are a YAML fix specialist. Apply the fixes identified in the review.
        ORIGINAL YAML:
        {{step.step_05_assemble_yaml.output}}
        REVIEW FINDINGS:
        {{step.step_06_self_review.output}}
        If review says "ALL CHECKS PASSED", output the original YAML unchanged.
        Otherwise, apply EVERY fix listed in the review.
        CRITICAL: Output the COMPLETE fixed YAML. Do NOT output partial YAML or diffs.
        First line MUST be: workflow_id:
        No markdown fences.
        Output ONLY the corrected raw YAML.
      model_overrides:
        temperature: 0.05
        max_tokens: 10000
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/07-fixed.yml"
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta-v5.log"
              event_fields: [step_name, duration_ms, token_count]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta-v5.log"
              event_fields: [step_name, error_message]
              level: error
    # ============================================================
    # PHASE 7: FINAL CLEANUP AND VALIDATION
    # ============================================================
    step_08_final_validate:
      generative_entity: "${models.coder}"
      depends_on: [step_07_fix_issues]
      prompt: |
        You are a final YAML cleaner. Ensure the workflow is production-ready.
        INPUT YAML:
        {{step.step_07_fix_issues.output}}
        Perform these final cleanup steps:
        1. Remove any remaining --- STEP: or --- END --- markers
        2. Remove any "Here is the data:" prefixes that are NOT in shell-type steps
        3. Ensure consistent 2-space indentation throughout
        4. Remove any trailing whitespace
        5. Verify no tab characters (replace with spaces)
        6. Remove any "Output ONLY the result" lines that appear more than once per prompt
        7. Ensure file starts with "workflow_id:" on line 1
        Output ONLY the cleaned YAML. First line: workflow_id:
        No markdown fences. No explanations.
      model_overrides:
        temperature: 0.05
        max_tokens: 10000
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/final-workflow.yml"
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta-v5.log"
              event_fields: [step_name, duration_ms, token_count]
              level: info
          - bookmark: true
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta-v5.log"
              event_fields: [step_name, error_message]
              level: error