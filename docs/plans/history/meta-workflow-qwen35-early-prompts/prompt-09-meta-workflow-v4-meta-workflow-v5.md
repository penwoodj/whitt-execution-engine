<!--
Source Session: ses_13d246579ffeoYfYtn38hAEldq
Message Length: 32270 characters
YAML Sections: 24
Embedded Prompts: 17
Agentic Keywords: 11
Complexity: HIGH
Source Files: meta-workflow-v4.yml, meta-workflow-v5.yml
-->

workflow_id: meta_workflow_generator_v4
name: "Meta-Workflow Generator v4"
description: "Complexity-aware decomposed meta-workflow with 128K context models, sequential model switching, iterative validation."
version: "12.0.0"
author: "Whitt Execution Engine"
tags: [meta-workflow, yaml-generation, complexity-aware, 128k-context]
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
    name: "granite-3b-code-instruct-128k.i1-Q6_K"
    host:
      type: llama_cpp_with_vulkan
  "reasoner":
    name: "Phi-4-mini-reasoning-Q4_K_M"
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
    step_01_classify:
      generative_entity: "${models.planner}"
      prompt: |
        Rate this task's complexity for workflow generation.
        TASK: __TASK_PLACEHOLDER__
        SIMPLE = read a file, run one command, basic transform (2-3 steps)
        MEDIUM = multiple files, filtering, aggregation, formatting (4-6 steps)
        COMPLEX = multi-phase pipeline, cross-references, parallel branches (7+ steps)
        Answer with exactly one word: SIMPLE, MEDIUM, or COMPLEX
        No other text. No reasoning. No explanation.
      model_overrides:
        temperature: 0.1
        max_tokens: 50
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/01-complexity.txt"
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
              event_fields: [step_name, duration_ms, token_count]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
              event_fields: [step_name, error_message]
              level: error
    step_02_decompose:
      generative_entity: "${models.planner}"
      depends_on: [step_01_classify]
      prompt: |
        Break this task into the MINIMUM number of atomic actions needed.
        TASK: __TASK_PLACEHOLDER__
        Complexity: {{step.step_01_classify.output}}
        Rules:
        - SIMPLE tasks: 2-3 actions total
        - MEDIUM tasks: 3-5 actions total
        - COMPLEX tasks: 5-8 actions total
        - Each action = ONE operation (read file, transform text, compute value, format output, save result)
        - DO NOT split natural single operations into multiple steps
        - A "read file" action includes extraction — do not add separate "extract" steps
        - A "format output" action includes writing — do not add separate "save" steps
        - Shell commands for file reading count as ONE action, not two
        Format each action as a numbered item:
        N. <verb> <object> [using <method>]
        Example for "Read /tmp/data.csv and count rows":
        1. Read /tmp/data.csv using cat command
        2. Count the number of data rows
        3. Output the count as plain number
        Output ONLY numbered actions. No other text.
      model_overrides:
        temperature: 0.3
        max_tokens: 2000
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/02-actions.txt"
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
              event_fields: [step_name, duration_ms, token_count]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
              event_fields: [step_name, error_message]
              level: error
    step_03_plan_steps:
      generative_entity: "${models.planner}"
      depends_on: [step_02_decompose]
      prompt: |
        You are a workflow architect. Convert actions into a YAML workflow step plan.
        COMPLEXITY: {{step.step_01_classify.output}}
        ACTIONS:
        {{step.step_02_decompose.output}}
        For each action, output EXACTLY one line:
        STEP_NAME: step_XX_<name> | DEPENDS: <comma-separated step names or "none"> | TYPE: <shell|llm> | SHELL_CMD: <command if shell else "none">
        Rules:
        - Step names: lowercase, underscores, zero-padded (step_01, step_02, step_10)
        - First step depends on "none"
        - File read operations = TYPE: shell with actual command (e.g., "cat /tmp/data.csv")
        - All other actions = TYPE: llm with SHELL_CMD: none
        - Sequential chain: each step depends on previous
        - For COMPLEX: phases map to dependency groups (Phase 1 steps → Phase 2 depends on last Phase 1 step)
        Output ONLY step definitions, one per line. No other text.
      model_overrides:
        temperature: 0.2
        max_tokens: 2000
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/03-plan.txt"
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
              event_fields: [step_name, duration_ms, token_count]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
              event_fields: [step_name, error_message]
              level: error
    step_04_write_prompts:
      generative_entity: "${models.planner}"
      depends_on: [step_03_plan_steps, step_02_decompose]
      prompt: |
        You are a prompt writer for agentic workflow steps. Write a detailed prompt for EACH step.
        ORIGINAL TASK: __TASK_PLACEHOLDER__
        ACTIONS:
        {{step.step_02_decompose.output}}
        STEP PLAN:
        {{step.step_03_plan_steps.output}}
        For EACH step, write a prompt block:
        --- STEP: step_XX_name ---
        <detailed prompt for this step>
        --- END ---
        CRITICAL RULES:
        1. Every prompt MUST be 5-10 lines with specific instructions
        2. Steps with depends_on MUST reference previous output: {{step.PREVIOUS.output}}
        3. TYPE: shell steps MUST include: "Here is the data:\n{{bookmarks.shell_output.stdout}}"
        4. TYPE: llm steps MUST reference previous step outputs via {{step.X.output}}
        5. EVERY prompt MUST end with: "Output ONLY the result. No explanations. No process descriptions."
        6. Include concrete output format example in EVERY prompt
        7. Use action phrases: "Given the data, extract...", "From the text, output..."
        8. Specify exact output format: "Output a numbered list", "Output JSON: {key: value}", etc.
        Output ALL step prompts. No other text.
      model_overrides:
        temperature: 0.3
        max_tokens: 4000
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/04-prompts.txt"
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
              event_fields: [step_name, duration_ms, token_count]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
              event_fields: [step_name, error_message]
              level: error
    step_05_assemble:
      generative_entity: "${models.coder}"
      depends_on: [step_03_plan_steps, step_04_write_prompts]
      prompt: |
        You are a YAML assembler. Combine step plan and prompts into ONE workflow YAML file.
        STEP PLAN:
        {{step.step_03_plan_steps.output}}
        PROMPTS:
        {{step.step_04_write_prompts.output}}
        CRITICAL: Copy the EXACT hook pattern for EVERY step. Do NOT skip when: blocks.
        Do NOT explain the YAML. Do NOT describe what you are doing. Output ONLY raw YAML.
        First line MUST be: workflow_id:
        STEP TEMPLATE (copy for EVERY step, change marked parts):
          step_XX_name:
            generative_entity: "${models.worker}"
            depends_on: [step_YY_prev]
            prompt: |
              <prompt from PROMPTS section>
            model_overrides:
              temperature: 0.3
              max_tokens: 50000
            when:
              before_step_starts:
                - shell:
                    command: "SHELL_CMD"
                    args: []
                    working_dir: "/home/jon/code/whitt-execution-engine"
                    fail_on_error: false
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
        HEADER:
         workflow_id: <descriptive_id>
         name: "<Name>"
         description: "<one sentence>"
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
            <ALL STEPS HERE indented 4 spaces>
        RULES:
        1. Start with header above
        2. ALL steps under agentic_workflow: steps: at 4-space indent
        3. Copy when: blocks EXACTLY — never skip them
        4. TYPE: shell → keep before_step_starts, set command and working_dir
        5. TYPE: llm → REMOVE entire before_step_starts block
        6. No markdown fences anywhere
        7. Step names zero-padded: step_01, step_02, ..., step_10
        8. Prompts use pipe | syntax
        9. File starts with "workflow_id:" on line 1
        10. Shell commands MUST use absolute paths (e.g. /etc/hostname not hostname)
        11. working_dir MUST be /home/jon/code/whitt-execution-engine for ALL shell steps
        Output ONLY raw YAML. First line: workflow_id:
      model_overrides:
        temperature: 0.05
        max_tokens: 8000
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/05-assembled.yml"
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
              event_fields: [step_name, duration_ms, token_count]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
              event_fields: [step_name, error_message]
              level: error
    step_06_validate:
      generative_entity: "${models.coder}"
      depends_on: [step_05_assemble]
      prompt: |
        You are a YAML workflow validator and repair specialist. Fix ALL issues.
        INPUT:
        {{step.step_05_assemble.output}}
        CHECKS (fix EVERY issue):
        1. YAML parses correctly
        2. Has: workflow_id, name, providers, models, agentic_workflow with steps
        3. Steps nested under agentic_workflow: steps: (NOT top-level)
        4. Every step has: generative_entity, prompt, model_overrides, when
        5. Every step except first has depends_on
        6. Every step with depends_on has {{step.X.output}} in prompt
        7. Provider: providers: llama_cpp_with_vulkan: config: host: port:
        8. Model: models: "worker": name: host: type:
        9. EVERY step has when: with after_step_succeeds (save_to + log)
        10. EVERY step has when: with after_step_fails (log)
        11. Shell steps have before_step_starts with shell action
        12. LLM steps do NOT have before_step_starts
        13. No markdown fences (no ```)
        14. File starts with "workflow_id:" on line 1
        15. Step names zero-padded if >9 steps
        16. Prompts use pipe | syntax
        17. Every prompt ends with "Output ONLY the result" directive
        18. Every prompt includes output format example
        19. No hosting: gpu_layers field (remove if present)
        Output ONLY the fixed YAML. No fences. No explanations.
      model_overrides:
        temperature: 0.05
        max_tokens: 8000
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/final-workflow.yml"
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
              event_fields: [step_name, duration_ms, token_count]
              level: info
          - bookmark: true
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
              event_fields: [step_name, error_message]
              level: error

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
    name: "SmolLM3-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "coder":
    name: "granite-3b-code-instruct-128k.i1-Q6_K"
    host:
      type: llama_cpp_with_vulkan
  "reasoner":
    name: "Phi-4-mini-instruct-Q6_K"
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
      generative_entity: "${models.reasoner}"
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
        For each step in the STEP PLAN, create a YAML step block. You MUST copy the actual prompt text from the "STEP PROMPTS" section above into each step's prompt: | field. Do NOT write placeholder text like "<prompt from STEP PROMPTS>". Use the ACTUAL prompt content from the STEP PROMPTS section.
        Each step MUST follow this exact structure:
            step_XX_name:
              generative_entity: "${models.worker}"
              depends_on: [step_YY_prev]
              prompt: |
                <ACTUAL PROMPT TEXT FROM STEP PROMPTS SECTION - copy it verbatim>
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
        FOR SHELL STEPS (TYPE: shell in STEP PLAN), add before_step_starts INSIDE when: block:
              when:
                before_step_starts:
                  - shell:
                      command: "THE ACTUAL SHELL COMMAND FROM STEP PLAN"
                      args: []
                      working_dir: "/home/jon/code/whitt-execution-engine"
                      fail_on_error: false
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
        CRITICAL RULES:
        1. First step has NO depends_on field at all (remove it entirely)
        2. TYPE: shell steps → include before_step_starts with shell action using the ACTUAL command from STEP PLAN
        3. TYPE: llm steps → NO before_step_starts block at all
        4. EVERY step has after_step_succeeds with save_to AND log
        5. EVERY step has after_step_fails with log
        6. All output paths use ./docs/benchmarks/outputs/output/ or ./docs/benchmarks/outputs/logs/
        7. ABSOLUTELY NO markdown fences (no triple backticks ```)
        8. File starts with "workflow_id:" on line 1 (not with backticks)
        9. File ends with the last step's closing line (not with backticks)
        10. Step names use zero-padding: step_01, step_02
        11. Prompts use pipe | syntax
        12. Remove --- STEP: and --- END --- markers from prompts
        13. Copy ACTUAL prompt text from STEP PROMPTS section - NEVER use placeholder text
        14. Every prompt MUST contain actual instructions, not angle-bracket placeholders
        Output ONLY raw YAML. First line: workflow_id:
        Last line: the closing of the last step. NO backticks. NO markdown.
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
    # PHASE 5: DETERMINISTIC VALIDATION (shell-based)
    # ============================================================
    step_06_validate_yaml:
      generative_entity: "${models.coder}"
      depends_on: [step_05_assemble_yaml]
      prompt: |
        You have assembled a workflow YAML. A validation script has already run on it. Read the validation result below.
        VALIDATION RESULT:
        {{bookmarks.validation_result.stdout}}
        If the validation result contains "VALID:" then the YAML passed validation. Output the EXACT text "VALIDATION_PASSED" and nothing else.
        If the validation result contains "ERROR:" or "FATAL:" then list each error exactly as shown. Output ONLY the error messages, one per line.
      model_overrides:
        temperature: 0.1
        max_tokens: 500
      when:
        before_step_starts:
          - shell:
              command: "python3 scripts/validate-yaml.py docs/benchmarks/outputs/meta-workflow/__RUN_ID__/05-assembled.yml 2>&1 || true"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: false
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/06-validation.txt"
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
    # PHASE 6: DETERMINISTIC POST-PROCESSING (SHELL-BASED)
    # Fixes fences, indentation, placeholders programmatically
    # ============================================================
    step_07_post_process:
      generative_entity: "${models.coder}"
      depends_on: [step_06_validate_yaml, step_05_assemble_yaml]
      prompt: |
        The YAML file has been post-processed by a deterministic fix script. Confirm the fix was applied by outputting "POST_PROCESSING_COMPLETE".
      model_overrides:
        temperature: 0.0
        max_tokens: 50
      when:
        before_step_starts:
          - shell:
              command: "cp docs/benchmarks/outputs/meta-workflow/__RUN_ID__/05-assembled.yml docs/benchmarks/outputs/meta-workflow/__RUN_ID__/07-fixed.yml && python3 scripts/fix-generated-yaml.py docs/benchmarks/outputs/meta-workflow/__RUN_ID__/07-fixed.yml docs/benchmarks/outputs/meta-workflow/__RUN_ID__/03-step-plan.txt 2>&1 || true"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: false
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
    # PHASE 7: FINAL SAVE (shell-based, no LLM re-processing)
    # ============================================================
    step_08_final_save:
      generative_entity: "${models.coder}"
      depends_on: [step_07_post_process]
      prompt: |
        Confirm the final workflow file is ready. Output "FINAL_WORKFLOW_READY".
      model_overrides:
        temperature: 0.0
        max_tokens: 50
      when:
        before_step_starts:
          - shell:
              command: "python3 scripts/validate-yaml.py docs/benchmarks/outputs/meta-workflow/__RUN_ID__/07-fixed.yml 2>&1 && cp docs/benchmarks/outputs/meta-workflow/__RUN_ID__/07-fixed.yml docs/benchmarks/outputs/meta-workflow/__RUN_ID__/final-workflow.yml || echo 'validation-failed-copying-anyway' && cp docs/benchmarks/outputs/meta-workflow/__RUN_ID__/07-fixed.yml docs/benchmarks/outputs/meta-workflow/__RUN_ID__/final-workflow.yml"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: false
        after_step_succeeds:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta-v5.log"
              event_fields: [step_name, duration_ms]
              level: info
          - bookmark: true
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta-v5.log"
              event_fields: [step_name, error_message]
              level: error