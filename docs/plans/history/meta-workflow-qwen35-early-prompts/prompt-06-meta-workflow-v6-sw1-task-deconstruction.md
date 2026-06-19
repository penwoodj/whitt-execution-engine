<!--
Source Session: ses_13d246579ffeoYfYtn38hAEldq
Message Length: 68401 characters
YAML Sections: 60
Embedded Prompts: 44
Agentic Keywords: 18
Complexity: HIGH
Source Files: meta-workflow-v6.yml, sw1-task-deconstruction.yml, sw2-desired-output-state.yml
-->

workflow_id: meta_workflow_v6
name: "Meta Workflow Generator v6"
description: "Orchestrates SW1-SW5 to generate an executable agentic workflow YAML from a high-complexity prompt."
version: "6.0.0"
author: "Whitt Execution Engine"
tags: [meta-workflow, orchestrator, shell-based]
schema_version: "2.0.0"
min_schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
models:
  "qwen35":
    name: "Qwen3-5-9B-Q4_K_M.gguf"
    host:
      type: llama_cpp_with_vulkan
workflow_execution_strategy:
  memory:
    model_lifecycle:
      unload_unused: false
agentic_workflow:
  when:
    after_workflow:
      - log:
          to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
          event_fields: [workflow_id, total_steps, succeeded, failed]
          level: info
  steps:
    step_00_bootstrap:
      generative_entity: "${models.qwen35}"
      prompt: |
        Bootstrapping META-v6 orchestrator. The pipeline will run SW1→SW2→SW3→SW4→SW5.
        Output "READY".
      model_overrides:
        temperature: 0.0
        max_tokens: 10
      when:
        before_step_starts:
          - shell:
              command: "mkdir -p ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/{meta,logs,input,sw1,sw2,sw3,sw4,sw5} && echo 'META-v6 bootstrap complete' > ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/meta/00-bootstrap.txt && cat ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/prompt.txt | wc -w"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true
        after_step_succeeds:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
              event_fields: [step_name, duration_ms]
              level: info
    step_01_run_sw1_task_deconstruction:
      generative_entity: "${models.qwen35}"
      depends_on: [step_00_bootstrap]
      prompt: |
        Running SW1 (task-deconstruction). This step waits for SW1 to complete.
        Output "SW1_DISPATCHED".
      model_overrides:
        temperature: 0.0
        max_tokens: 10
      when:
        before_step_starts:
          - shell:
              command: "SW1_RUN_ID=meta-__RUN_ID__-sw1-$(date +%Y%m%d-%H%M%S) && mkdir -p ./docs/benchmarks/outputs/meta-workflow/${SW1_RUN_ID}/{sw1,logs,input} && cp ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/prompt.txt ./docs/benchmarks/outputs/meta-workflow/${SW1_RUN_ID}/input/prompt.txt && sed \"s/__RUN_ID__/${SW1_RUN_ID}/g\" ./docs/benchmarks/workflows/sw1-task-deconstruction.yml > ./docs/benchmarks/outputs/meta-workflow/${SW1_RUN_ID}/sw1-runtime.yml && ./target/release/whitt benchmark --workflow ./docs/benchmarks/outputs/meta-workflow/${SW1_RUN_ID}/sw1-runtime.yml --output-dir ./docs/benchmarks/outputs/meta-workflow/${SW1_RUN_ID} --models-dir /home/jon/code/whitt-execution-engine/models --filter-name 'Qwen3-5-9B' --load-timeout 300 > ./docs/benchmarks/outputs/meta-workflow/${SW1_RUN_ID}/benchmark.log 2>&1 && cp ./docs/benchmarks/outputs/meta-workflow/${SW1_RUN_ID}/sw1/tasks.md ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/tasks.md && echo \"SW1_DONE: ${SW1_RUN_ID}\""
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true
        after_step_succeeds:
          - shell:
              command: "ls -la ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/tasks.md && wc -l ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/tasks.md"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: false
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
              event_fields: [step_name, duration_ms]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
              event_fields: [step_name, error_message]
              level: error
    step_02_run_sw2_desired_output_state:
      generative_entity: "${models.qwen35}"
      depends_on: [step_01_run_sw1_task_deconstruction]
      prompt: |
        Running SW2 (desired-output-state). This step waits for SW2 to complete.
        Output "SW2_DISPATCHED".
      model_overrides:
        temperature: 0.0
        max_tokens: 10
      when:
        before_step_starts:
          - shell:
              command: "SW2_RUN_ID=meta-__RUN_ID__-sw2-$(date +%Y%m%d-%H%M%S) && mkdir -p ./docs/benchmarks/outputs/meta-workflow/${SW2_RUN_ID}/{sw2,logs,input} && cp ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/tasks.md ./docs/benchmarks/outputs/meta-workflow/${SW2_RUN_ID}/input/tasks.md && sed \"s/__RUN_ID__/${SW2_RUN_ID}/g\" ./docs/benchmarks/workflows/sw2-desired-output-state.yml > ./docs/benchmarks/outputs/meta-workflow/${SW2_RUN_ID}/sw2-runtime.yml && ./target/release/whitt benchmark --workflow ./docs/benchmarks/outputs/meta-workflow/${SW2_RUN_ID}/sw2-runtime.yml --output-dir ./docs/benchmarks/outputs/meta-workflow/${SW2_RUN_ID} --models-dir /home/jon/code/whitt-execution-engine/models --filter-name 'Qwen3-5-9B' --load-timeout 300 > ./docs/benchmarks/outputs/meta-workflow/${SW2_RUN_ID}/benchmark.log 2>&1 && cp ./docs/benchmarks/outputs/meta-workflow/${SW2_RUN_ID}/sw2/outputs.md ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/outputs.md && echo \"SW2_DONE: ${SW2_RUN_ID}\""
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true
        after_step_succeeds:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
              event_fields: [step_name, duration_ms]
              level: info
    step_03_run_sw3_agentic_categorization:
      generative_entity: "${models.qwen35}"
      depends_on: [step_02_run_sw2_desired_output_state]
      prompt: |
        Running SW3 (agentic-categorization). This step waits for SW3 to complete.
        Output "SW3_DISPATCHED".
      model_overrides:
        temperature: 0.0
        max_tokens: 10
      when:
        before_step_starts:
          - shell:
              command: "SW3_RUN_ID=meta-__RUN_ID__-sw3-$(date +%Y%m%d-%H%M%S) && mkdir -p ./docs/benchmarks/outputs/meta-workflow/${SW3_RUN_ID}/{sw3,logs,input} && cp ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/outputs.md ./docs/benchmarks/outputs/meta-workflow/${SW3_RUN_ID}/input/outputs.md && sed \"s/__RUN_ID__/${SW3_RUN_ID}/g\" ./docs/benchmarks/workflows/sw3-agentic-categorization.yml > ./docs/benchmarks/outputs/meta-workflow/${SW3_RUN_ID}/sw3-runtime.yml && ./target/release/whitt benchmark --workflow ./docs/benchmarks/outputs/meta-workflow/${SW3_RUN_ID}/sw3-runtime.yml --output-dir ./docs/benchmarks/outputs/meta-workflow/${SW3_RUN_ID} --models-dir /home/jon/code/whitt-execution-engine/models --filter-name 'Qwen3-5-9B' --load-timeout 300 > ./docs/benchmarks/outputs/meta-workflow/${SW3_RUN_ID}/benchmark.log 2>&1 && cp ./docs/benchmarks/outputs/meta-workflow/${SW3_RUN_ID}/sw3/categories.md ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/categories.md && echo \"SW3_DONE: ${SW3_RUN_ID}\""
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true
        after_step_succeeds:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
              event_fields: [step_name, duration_ms]
              level: info
    step_04_run_sw4_yaml_substructure_translation:
      generative_entity: "${models.qwen35}"
      depends_on: [step_03_run_sw3_agentic_categorization]
      prompt: |
        Running SW4 (yaml-substructure-translation). This step waits for SW4 to complete.
        Output "SW4_DISPATCHED".
      model_overrides:
        temperature: 0.0
        max_tokens: 10
      when:
        before_step_starts:
          - shell:
              command: "SW4_RUN_ID=meta-__RUN_ID__-sw4-$(date +%Y%m%d-%H%M%S) && mkdir -p ./docs/benchmarks/outputs/meta-workflow/${SW4_RUN_ID}/{sw4,logs,input} && cp ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/categories.md ./docs/benchmarks/outputs/meta-workflow/${SW4_RUN_ID}/input/categories.md && sed \"s/__RUN_ID__/${SW4_RUN_ID}/g\" ./docs/benchmarks/workflows/sw4-yaml-substructure-translation.yml > ./docs/benchmarks/outputs/meta-workflow/${SW4_RUN_ID}/sw4-runtime.yml && ./target/release/whitt benchmark --workflow ./docs/benchmarks/outputs/meta-workflow/${SW4_RUN_ID}/sw4-runtime.yml --output-dir ./docs/benchmarks/outputs/meta-workflow/${SW4_RUN_ID} --models-dir /home/jon/code/whitt-execution-engine/models --filter-name 'Qwen3-5-9B' --load-timeout 300 > ./docs/benchmarks/outputs/meta-workflow/${SW4_RUN_ID}/benchmark.log 2>&1 && cp ./docs/benchmarks/outputs/meta-workflow/${SW4_RUN_ID}/sw4/structs.md ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/structs.md && echo \"SW4_DONE: ${SW4_RUN_ID}\""
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true
        after_step_succeeds:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
              event_fields: [step_name, duration_ms]
              level: info
    step_05_run_sw5_final_assembly:
      generative_entity: "${models.qwen35}"
      depends_on: [step_04_run_sw4_yaml_substructure_translation]
      prompt: |
        Running SW5 (final-workflow-assembly). This step waits for SW5 to complete.
        Output "SW5_DISPATCHED".
      model_overrides:
        temperature: 0.0
        max_tokens: 10
      when:
        before_step_starts:
          - shell:
              command: "SW5_RUN_ID=meta-__RUN_ID__-sw5-$(date +%Y%m%d-%H%M%S) && mkdir -p ./docs/benchmarks/outputs/meta-workflow/${SW5_RUN_ID}/{sw5,logs,input} && cp ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/structs.md ./docs/benchmarks/outputs/meta-workflow/${SW5_RUN_ID}/input/structs.md && sed \"s/__RUN_ID__/${SW5_RUN_ID}/g\" ./docs/benchmarks/workflows/sw5-final-workflow-assembly.yml > ./docs/benchmarks/outputs/meta-workflow/${SW5_RUN_ID}/sw5-runtime.yml && ./target/release/whitt benchmark --workflow ./docs/benchmarks/outputs/meta-workflow/${SW5_RUN_ID}/sw5-runtime.yml --output-dir ./docs/benchmarks/outputs/meta-workflow/${SW5_RUN_ID} --models-dir /home/jon/code/whitt-execution-engine/models --filter-name 'Qwen3-5-9B' --load-timeout 300 > ./docs/benchmarks/outputs/meta-workflow/${SW5_RUN_ID}/benchmark.log 2>&1 && cp ./docs/benchmarks/outputs/meta-workflow/${SW5_RUN_ID}/sw5/workflow.yml ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/meta/generated-workflow.yml && echo \"SW5_DONE: ${SW5_RUN_ID}\""
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true
        after_step_succeeds:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
              event_fields: [step_name, duration_ms]
              level: info
    step_06_final_validation:
      generative_entity: "${models.qwen35}"
      depends_on: [step_05_run_sw5_final_assembly]
      prompt: |
        Pipeline complete. Validating final output.
        Generated workflow:
        {{bookmarks.shell_output.stdout}}
        Check:
        1. workflow.yml exists and is non-empty
        2. YAML is valid
        3. Contains steps
        Output VALID or INVALID with reason.
      model_overrides:
        temperature: 0.0
        max_tokens: 200
      when:
        before_step_starts:
          - shell:
              command: "ls -la ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/meta/generated-workflow.yml && head -20 ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/meta/generated-workflow.yml && echo '---line count---' && wc -l ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/meta/generated-workflow.yml"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: false
        after_step_succeeds:
          - save_to:
              - meta_validation
              - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/meta/validation.txt"
          - bookmark: true
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
              event_fields: [step_name, duration_ms, total_tokens]
              level: info

         For EVERY task scoring >= 5 points, output a subtask expansion block.
         For tasks scoring 1-3 points, output NO_EXPANSION_NEEDED.
         Output format (one block per task):
         ## T<id> - <name> (<points>pts)
         EXPANSION: <SUBTASKS | NO_EXPANSION_NEEDED>
         - T<id>.1 - <subtask_name> (1pts): <action under 100 chars> | Given: <precondition> When: <trigger> Then: <result>
         - T<id>.2 - <subtask_name> (2pts): <action under 100 chars> | Given: <precondition> When: <trigger> Then: <result>
         Rules (STRICT — violations cause rejection):
         - Subtask points MUST be one of: 1, 2, or 3 — NEVER 5, 8, or 13
         - Each parent task scoring >= 5 MUST have at least 2 subtasks
         - Each subtask MUST have inline GWT (Given/When/Then) separated by `|`
         - Subtask actions < 100 chars
         - Sum of subtask points should be roughly equal to parent task points
         - Output one block per task from the task list
         Output ONLY the expansion blocks. No other text.
        ORIGINAL PROMPT:
        {{bookmarks.shell_output.stdout}}
        Assemble a SINGLE markdown document in this EXACT format. Use the ORIGINAL PROMPT shown above (first 200 chars) as "Source prompt" — do NOT hallucinate.
        **Source prompt:** (first 200 chars of the ORIGINAL PROMPT shown above)
        **Maximum leaf complexity:** <X> pts (MUST be <= 3)
        **Why:** <ONE sentence justifying the points — reference the complexity driver>
          - **Given:** <preconditions from prior task or environment>
          - **When:** <action this task performs>
          - **Then:** <observable, verifiable outcome>
        **Story points:** <Y> (MUST be 1, 2, or 3)
        **Why:** <ONE sentence justifying>
        **GWT:**
          - **Given:** <preconditions>
          - **When:** <trigger>
          - **Then:** <observable result>
        (continue for ALL tasks)
        | 8 | 0 |
        | 13 | 0 |
        Rules (STRICT):
        - Every leaf task has full GWT criteria (Given + When + Then, each non-empty)
        - Every subtask MUST have a "Why" field
        - "Why" justifies the story point value with a complexity driver
        - Total story points = sum of all LEAF task points
        - Maximum leaf complexity MUST be <= 3 (any leaf >= 5 is a violation)
        max_tokens: 3000
        before_step_starts:
          - shell:
              command: "cat ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/prompt.txt"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true

workflow_id: sw2_desired_output_state
name: "SW2 Desired Output State"
description: "Translates task breakdown into desired output state per task — what the system looks like after each task completes."
version: "1.0.0"
author: "Whitt Execution Engine"
tags: [sub-workflow, desired-output-state, evaluation-criteria]
schema_version: "2.0.0"
min_schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
models:
  "qwen35":
    name: "Qwen3-5-9B-Q4_K_M.gguf"
    host:
      type: llama_cpp_with_vulkan
workflow_execution_strategy:
  memory:
    model_lifecycle:
      unload_unused: false
agentic_workflow:
  when:
    after_workflow:
      - log:
          to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw2.log"
          event_fields: [workflow_id, total_steps, succeeded, failed]
          level: info
  steps:
    step_00_bootstrap:
      generative_entity: "${models.qwen35}"
      prompt: |
        Bootstrapping SW2 (desired-output-state). Output "READY".
      model_overrides:
        temperature: 0.0
        max_tokens: 10
      when:
        before_step_starts:
          - shell:
              command: "mkdir -p ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/{sw2,logs,input} && cat ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/tasks.md | head -100"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true
          - bookmark:
              path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw2/00-bootstrap.txt"
        after_step_succeeds:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw2.log"
              event_fields: [step_name, duration_ms]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw2.log"
              event_fields: [step_name, error_message]
              level: error
    step_01_extract_task_index:
      generative_entity: "${models.qwen35}"
      depends_on: [step_00_bootstrap]
      prompt: |
        You are extracting the task index from a task breakdown document.
        TASK BREAKDOWN:
        {{bookmarks.shell_output.stdout}}
        Extract a FLAT list of all tasks AND subtasks in this exact format (one per line):
        T1 - <name> (<points>pts) [PARENT]
        T1.1 - <name> (<points>pts) [LEAF]
        T1.2 - <name> (<points>pts) [LEAF]
        T2 - <name> (<points>pts) [PARENT]
        ...
        Rules:
        - Include BOTH parents (T1, T2, ...) AND leaves (T1.1, T1.2, ...)
        - Mark parent tasks with [PARENT] and leaf tasks with [LEAF]
        - Preserve the original names and points
        - Output ONLY the flat list. No preamble.
      model_overrides:
        temperature: 0.1
        max_tokens: 1500
      when:
        before_step_starts:
          - shell:
              command: "cat ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/tasks.md"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true
        after_step_succeeds:
          - save_to:
              - sw2_task_index
              - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw2/01-task-index.txt"
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw2.log"
              event_fields: [step_name, duration_ms, total_tokens]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw2.log"
              event_fields: [step_name, error_message]
              level: error
    step_02_plan_chunks:
      generative_entity: "${models.qwen35}"
      depends_on: [step_01_extract_task_index]
      prompt: |
        You are planning how to process tasks in chunks for output-state generation.
        TASK INDEX:
        {{step.step_01_extract_task_index.output}}
        Group tasks into chunks of 3-5 leaf tasks each. Each chunk should represent
        a coherent unit of work (e.g., all subtasks of one parent, or related validation steps).
        Output format:
        CHUNK 1 (complexity: <sum of points>pts):
        - T1.1 - <name>
        - T1.2 - <name>
        - T1.3 - <name>
        CHUNK 2 (complexity: <sum of points>pts):
        - T2.1 - <name>
        ...
        Rules:
        - Each chunk has 3-5 LEAF tasks (never parent tasks)
        - Sum the points for each chunk
        - Order chunks by task ID
        - Output ONLY the chunk plan. No preamble.
      model_overrides:
        temperature: 0.2
        max_tokens: 1200
      when:
        after_step_succeeds:
          - save_to:
              - sw2_chunk_plan
              - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw2/02-chunk-plan.txt"
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw2.log"
              event_fields: [step_name, duration_ms, total_tokens]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw2.log"
              event_fields: [step_name, error_message]
              level: error
    step_03_generate_outputs:
      generative_entity: "${models.qwen35}"
      depends_on: [step_02_plan_chunks]
      prompt: |
        You are generating the desired output state for each task.
        ORIGINAL TASK BREAKDOWN:
        {{bookmarks.shell_output.stdout}}
        TASK INDEX:
        {{step.step_01_extract_task_index.output}}
        CHUNK PLAN:
        {{step.step_02_plan_chunks.output}}
        For EACH task (parent AND leaf), produce a desired output state entry.
        ## What is "Desired Output State"?
        The desired output state describes what is TRUE about the system AFTER the task completes.
        It is NOT the action — it is the RESULT. Think of it as the post-condition.
        Output format (one block per task):
        ### T<id> - <name>
        **Desired state:** <one sentence describing observable end-state>
        **Artifacts produced:** <files, data structures, or values created>
        **Acceptance criteria:**
          - <criterion 1 — observable and testable>
          - <criterion 2>
          - <criterion 3>
        **Evaluation method:** <how to verify — unit test, integration test, manual inspection, file check>
        **Downstream dependencies:** <which tasks consume this output>
        **Failure indicators:**
          - <what failure looks like>
          - <common pitfalls>
        Rules (STRICT):
        - One block per task (both parents and leaves)
        - "Desired state" must be OBSERVABLE (file exists, value matches, count == N)
        - "Acceptance criteria" must be TESTABLE (avoid vague terms)
        - "Evaluation method" must be specific (test name, command, or check)
        - Cover EVERY task from the task index — none skipped
        - Keep each block under 200 words
        Output ONLY the task output-state blocks. No preamble.
      model_overrides:
        temperature: 0.2
        max_tokens: 3500
      when:
        before_step_starts:
          - shell:
              command: "cat ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/tasks.md"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true
        after_step_succeeds:
          - save_to:
              - sw2_outputs
              - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw2/03-outputs.md"
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw2.log"
              event_fields: [step_name, duration_ms, total_tokens]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw2.log"
              event_fields: [step_name, error_message]
              level: error
    step_04_evaluate:
      generative_entity: "${models.qwen35}"
      depends_on: [step_03_generate_outputs]
      prompt: |
        You are a quality reviewer for desired output state specifications.
        TASK INDEX:
        {{step.step_01_extract_task_index.output}}
        OUTPUT STATE BLOCKS:
        {{step.step_03_generate_outputs.output}}
        Evaluate against EACH criterion. Output PASS or FAIL with one-sentence reason:
        1. COVERAGE: Every task from the index has an output-state block
        2. OBSERVABLE: Every "Desired state" describes something measurable
        3. TESTABLE: Every "Acceptance criteria" can be verified by a test or check
        4. ARTIFACTS: Every block specifies concrete artifacts (file, value, structure)
        5. DEPENDENCIES: Every block lists downstream consumers
        6. FAILURE_MODES: Every block lists failure indicators
        7. CONCISE: No block exceeds 200 words
        End with EXACTLY one of these on the last line:
        VERDICT: PASS
        VERDICT: FAIL
        Output ONLY the evaluation. No preamble.
      model_overrides:
        temperature: 0.1
        max_tokens: 1000
      when:
        after_step_succeeds:
          - save_to:
              - sw2_eval
              - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw2/04-eval.txt"
          - gwt:
              - given: "{{step.step_04_evaluate.output}} contains 'VERDICT: PASS'"
                then: { route_to: step_06_assemble_final }
              - given: "true"
                then: { route_to: step_05_fix }
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw2.log"
              event_fields: [step_name, duration_ms, total_tokens]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw2.log"
              event_fields: [step_name, error_message]
              level: error
    step_05_fix:
      generative_entity: "${models.qwen35}"
      depends_on: [step_04_evaluate]
      prompt: |
        You are fixing issues in desired output state specifications.
        TASK INDEX:
        {{step.step_01_extract_task_index.output}}
        CURRENT OUTPUT STATE BLOCKS:
        {{step.step_03_generate_outputs.output}}
        EVALUATION FAILURES:
        {{step.step_04_evaluate.output}}
        Address every FAIL. Output the CORRECTED output state blocks (same format as input).
        Common fixes:
        - If COVERAGE fails: add missing task blocks
        - If OBSERVABLE fails: rewrite "Desired state" to be measurable
        - If TESTABLE fails: rewrite criteria with specific checks
        - If ARTIFACTS fails: name concrete files/values
        - If DEPENDENCIES fails: list downstream consumers
        - If FAILURE_MODES fails: add failure indicators
        Output ONLY the corrected blocks (no === HEADERS ===). No preamble.
      model_overrides:
        temperature: 0.2
        max_tokens: 3500
      when:
        after_step_succeeds:
          - save_to:
              - sw2_corrected
              - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw2/05-corrected.md"
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw2.log"
              event_fields: [step_name, duration_ms, total_tokens]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw2.log"
              event_fields: [step_name, error_message]
              level: error
    step_06_assemble_final:
      generative_entity: "${models.qwen35}"
      depends_on: [step_04_evaluate]
      prompt: |
        You are assembling the final desired output state document.
        TASK BREAKDOWN (source):
        {{bookmarks.shell_output.stdout}}
        OUTPUT STATE BLOCKS (initial):
        {{step.step_03_generate_outputs.output}}
        CORRECTED VERSION (if any):
        {{step.step_05_fix.output}}
        EVALUATION RESULT:
        {{step.step_04_evaluate.output}}
        Assemble a SINGLE markdown document in this EXACT format:
        # Desired Output State
        **Source task breakdown:** <first 150 chars of task breakdown>
        **Total tasks covered:** <N>
        **Total leaves covered:** <M>
        **Evaluation verdict:** <PASS or FAIL from evaluation>
        ---
        ## T1 - <name>
        **Desired state:** <observable end-state>
        **Artifacts produced:** <files/values created>
        **Acceptance criteria:**
          - <testable criterion>
          - <testable criterion>
        **Evaluation method:** <how to verify>
        **Downstream dependencies:** <consumers>
        **Failure indicators:**
          - <what failure looks like>
        ### T1.1 - <subtask_name>
        (same block format, indented under parent)
        ---
        ## T2 - <name>
        ...
        (continue for ALL tasks)
        ---
        ## Coverage Summary
        | Task | Has Output State | Acceptance Criteria Count | Verifiable |
        |------|------------------|---------------------------|------------|
        | T1 | ✅ | 3 | Yes |
        | T1.1 | ✅ | 2 | Yes |
        ...
        Rules (STRICT):
        - Cover EVERY task from the task breakdown
        - Every "Desired state" must be OBSERVABLE
        - Every "Acceptance criteria" must be TESTABLE
        - Use the CORRECTED version if available, else initial
        - Maximum 200 words per block
        Output ONLY the final markdown document.
      model_overrides:
        temperature: 0.1
        max_tokens: 4000
      when:
        before_step_starts:
          - shell:
              command: "cat ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/tasks.md | head -50"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true
        after_step_succeeds:
          - save_to:
              - sw2_final
              - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw2/outputs.md"
          - bookmark: true
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw2.log"
              event_fields: [step_name, duration_ms, total_tokens]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw2.log"
              event_fields: [step_name, error_message]
              level: error

workflow_id: sw3_agentic_categorization
name: "SW3 Agentic Categorization"
description: "Translates deconstructed tasks + desired outputs into agentic behavior categories that correlate to workflow substructures."
version: "1.0.0"
author: "Whitt Execution Engine"
tags: [sub-workflow, agentic-categorization, classification]
schema_version: "2.0.0"
min_schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
models:
  "qwen35":
    name: "Qwen3-5-9B-Q4_K_M.gguf"
    host:
      type: llama_cpp_with_vulkan
workflow_execution_strategy:
  memory:
    model_lifecycle:
      unload_unused: false
agentic_workflow:
  when:
    after_workflow:
      - log:
          to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw3.log"
          event_fields: [workflow_id, total_steps, succeeded, failed]
          level: info
  steps:
    step_00_bootstrap:
      generative_entity: "${models.qwen35}"
      prompt: |
        Bootstrapping SW3 (agentic-categorization). Output "READY".
      model_overrides:
        temperature: 0.0
        max_tokens: 10
      when:
        before_step_starts:
          - shell:
              command: "mkdir -p ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/{sw3,logs,input} && head -50 ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/outputs.md"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true
          - bookmark:
              path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw3/00-bootstrap.txt"
        after_step_succeeds:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw3.log"
              event_fields: [step_name, duration_ms]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw3.log"
              event_fields: [step_name, error_message]
              level: error
    step_01_extract_task_list:
      generative_entity: "${models.qwen35}"
      depends_on: [step_00_bootstrap]
      prompt: |
        You are extracting the task list from a desired-output-state document.
        OUTPUTS DOCUMENT:
        {{bookmarks.shell_output.stdout}}
        Extract ONLY the task IDs and names (no output-state details). One per line:
        T1 - <name>
        T1.1 - <name>
        T1.2 - <name>
        ...
        Include BOTH parents and leaves. Output ONLY the flat list.
      model_overrides:
        temperature: 0.1
        max_tokens: 1000
      when:
        before_step_starts:
          - shell:
              command: "cat ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/outputs.md"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true
        after_step_succeeds:
          - save_to:
              - sw3_task_list
              - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw3/01-task-list.txt"
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw3.log"
              event_fields: [step_name, duration_ms, total_tokens]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw3.log"
              event_fields: [step_name, error_message]
              level: error
    step_02_define_categories:
      generative_entity: "${models.qwen35}"
      depends_on: [step_01_extract_task_list]
      prompt: |
        You are defining agentic behavior categories for workflow substructures.
        These are the 6 standard agentic categories. Each maps to a workflow substructure pattern:
        1. SEQUENTIAL_PROCESSOR — Linear chain of steps, each depends on previous
           Pattern: step_a → step_b → step_c (depends_on chain)
           Use when: Tasks must execute in strict order, output feeds next input
        2. PARALLEL_FAN_OUT — One input splits to multiple independent processors
           Pattern: step_a → [step_b1, step_b2, step_b3] → step_c (aggregate)
           Use when: Independent computations on same input, merge results
        3. ITERATIVE_REFINER — Loop that improves output until quality gate passes
           Pattern: generate → evaluate → fix → evaluate (GWT gate)
           Use when: Quality criteria exist, LLM may need multiple passes
        4. CONDITIONAL_ROUTER — Branches based on runtime evaluation
           Pattern: evaluate → GWT clause → route_to A or B
           Use when: Different processing paths based on input characteristics
        5. DATA_TRANSFORMER — Pure input→output conversion, no branching
           Pattern: parse → transform → format (single step or short chain)
           Use when: Deterministic transformation, well-defined schema
        6. ACCUMULATOR — Iterates over collection, accumulating state
           Pattern: loop with iterate_values, append_to builds state
           Use when: Process N items, aggregate into single output
        Task List:
        {{step.step_01_extract_task_list.output}}
        Output ONLY this (nothing else):
        CATEGORIES DEFINED: 6
        1. SEQUENTIAL_PROCESSOR
        2. PARALLEL_FAN_OUT
        3. ITERATIVE_REFINER
        4. CONDITIONAL_ROUTER
        5. DATA_TRANSFORMER
        6. ACCUMULATOR
      model_overrides:
        temperature: 0.0
        max_tokens: 500
      when:
        after_step_succeeds:
          - save_to:
              - sw3_categories
              - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw3/02-categories.txt"
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw3.log"
              event_fields: [step_name, duration_ms, total_tokens]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw3.log"
              event_fields: [step_name, error_message]
              level: error
    step_03_categorize_tasks:
      generative_entity: "${models.qwen35}"
      depends_on: [step_02_define_categories]
      prompt: |
        You are categorizing each task into an agentic behavior pattern.
        OUTPUTS DOCUMENT (for context on each task's purpose):
        {{bookmarks.shell_output.stdout}}
        TASK LIST:
        {{step.step_01_extract_task_list.output}}
        For EACH task (parent AND leaf), output a categorization block:
        ### T<id> - <name>
        **Primary category:** <one of SEQUENTIAL_PROCESSOR, PARALLEL_FAN_OUT, ITERATIVE_REFINER, CONDITIONAL_ROUTER, DATA_TRANSFORMER, ACCUMULATOR>
        **Reasoning:** <2-3 sentences explaining why this category fits>
        **Substructure hint:** <brief description of the workflow substructure pattern>
          - e.g., "single LLM step with save_to + log hooks"
          - e.g., "loop with GWT quality gate, max 3 iterations"
          - e.g., "parallel branches with different prompts, merged via append_to"
        **Inputs needed:** <what this task consumes — prior step output, file, user param>
        **Outputs produced:** <what this task generates — file, variable, state change>
        **Evaluation scope:** <how to verify — single-step check, multi-step integration, end-to-end>
        **Iteration budget:** <1, 2, or 3 — how many LLM passes this task likely needs>
        Rules:
        - Assign EXACTLY ONE primary category per task
        - "Reasoning" must reference the task's actual output-state
        - "Iteration budget" reflects complexity (1=simple, 3=complex)
        - Cover EVERY task from the task list
        Output ONLY the categorization blocks. No preamble.
      model_overrides:
        temperature: 0.2
        max_tokens: 3000
      when:
        before_step_starts:
          - shell:
              command: "cat ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/outputs.md"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true
        after_step_succeeds:
          - save_to:
              - sw3_categorized
              - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw3/03-categorized.md"
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw3.log"
              event_fields: [step_name, duration_ms, total_tokens]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw3.log"
              event_fields: [step_name, error_message]
              level: error
    step_04_evaluate:
      generative_entity: "${models.qwen35}"
      depends_on: [step_03_categorize_tasks]
      prompt: |
        You are a quality reviewer for agentic categorization.
        TASK LIST:
        {{step.step_01_extract_task_list.output}}
        CATEGORIZATION BLOCKS:
        {{step.step_03_categorize_tasks.output}}
        Evaluate against EACH criterion. Output PASS or FAIL with reason:
        1. COVERAGE: Every task has a categorization block
        2. SINGLE_CATEGORY: Each task has exactly one primary category
        3. VALID_CATEGORY: Every category is one of the 6 defined types
        4. REASONED: Every "Reasoning" references the task's actual output-state
        5. SUBSTRUCTURE_HINT: Every block has a concrete substructure description
        6. INPUTS_OUTPUTS: Every block specifies inputs and outputs
        7. ITERATION_BUDGET: Every block has a budget in {1, 2, 3}
        End with EXACTLY one of:
        VERDICT: PASS
        VERDICT: FAIL
        Output ONLY the evaluation.
      model_overrides:
        temperature: 0.1
        max_tokens: 800
      when:
        after_step_succeeds:
          - save_to:
              - sw3_eval
              - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw3/04-eval.txt"
          - gwt:
              - given: "{{step.step_04_evaluate.output}} contains 'VERDICT: PASS'"
                then: { route_to: step_06_assemble_final }
              - given: "true"
                then: { route_to: step_05_fix }
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw3.log"
              event_fields: [step_name, duration_ms, total_tokens]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw3.log"
              event_fields: [step_name, error_message]
              level: error
    step_05_fix:
      generative_entity: "${models.qwen35}"
      depends_on: [step_04_evaluate]
      prompt: |
        You are fixing categorization issues.
        TASK LIST:
        {{step.step_01_extract_task_list.output}}
        CURRENT CATEGORIZATION:
        {{step.step_03_categorize_tasks.output}}
        EVALUATION FAILURES:
        {{step.step_04_evaluate.output}}
        Output CORRECTED categorization blocks (same format as input).
        Address every FAIL. No preamble.
      model_overrides:
        temperature: 0.2
        max_tokens: 3000
      when:
        after_step_succeeds:
          - save_to:
              - sw3_corrected
              - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw3/05-corrected.md"
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw3.log"
              event_fields: [step_name, duration_ms, total_tokens]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw3.log"
              event_fields: [step_name, error_message]
              level: error
    step_06_assemble_final:
      generative_entity: "${models.qwen35}"
      depends_on: [step_04_evaluate]
      prompt: |
        You are assembling the final agentic categorization document.
        OUTPUTS DOCUMENT (source):
        {{bookmarks.shell_output.stdout}}
        CATEGORIZATION BLOCKS (initial):
        {{step.step_03_categorize_tasks.output}}
        CORRECTED (if any):
        {{step.step_05_fix.output}}
        EVALUATION:
        {{step.step_04_evaluate.output}}
        Assemble a SINGLE markdown document:
        # Agentic Categorization
        **Source document:** <first 150 chars of outputs doc>
        **Total tasks categorized:** <N>
        **Evaluation verdict:** <PASS/FAIL>
        ---
        ## Category Summary
        | Category | Count | Avg Iteration Budget |
        |----------|-------|---------------------|
        | SEQUENTIAL_PROCESSOR | <n> | <avg> |
        | PARALLEL_FAN_OUT | <n> | <avg> |
        | ITERATIVE_REFINER | <n> | <avg> |
        | CONDITIONAL_ROUTER | <n> | <avg> |
        | DATA_TRANSFORMER | <n> | <avg> |
        | ACCUMULATOR | <n> | <avg> |
        ---
        ## T1 - <name>
        **Primary category:** <CATEGORY>
        **Reasoning:** <why>
        **Substructure hint:** <pattern>
        **Inputs:** <from where>
        **Outputs:** <to where>
        **Evaluation scope:** <how to verify>
        **Iteration budget:** <N>
        ### T1.1 - <name>
        (same format)
        ---
        (continue for ALL tasks)
        Output ONLY the final document.
      model_overrides:
        temperature: 0.1
        max_tokens: 3500
      when:
        before_step_starts:
          - shell:
              command: "cat ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/outputs.md | head -30"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true
        after_step_succeeds:
          - save_to:
              - sw3_final
              - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw3/categories.md"
          - bookmark: true
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw3.log"
              event_fields: [step_name, duration_ms, total_tokens]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw3.log"
              event_fields: [step_name, error_message]
              level: error

workflow_id: sw4_yaml_substructure_translation
name: "SW4 YAML Substructure Translation"
description: "Translates categorized tasks into individual YAML agentic substructures with intent descriptions. YAML-only code blocks in output."
version: "1.0.0"
author: "Whitt Execution Engine"
tags: [sub-workflow, yaml-translation, substructures]
schema_version: "2.0.0"
min_schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
models:
  "qwen35":
    name: "Qwen3-5-9B-Q4_K_M.gguf"
    host:
      type: llama_cpp_with_vulkan
workflow_execution_strategy:
  memory:
    model_lifecycle:
      unload_unused: false
agentic_workflow:
  when:
    after_workflow:
      - log:
          to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw4.log"
          event_fields: [workflow_id, total_steps, succeeded, failed]
          level: info
  steps:
    step_00_bootstrap:
      generative_entity: "${models.qwen35}"
      prompt: |
        Bootstrapping SW4 (yaml-substructure-translation). Output "READY".
      model_overrides:
        temperature: 0.0
        max_tokens: 10
      when:
        before_step_starts:
          - shell:
              command: "mkdir -p ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/{sw4,logs,input} && head -30 ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/categories.md"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true
          - bookmark:
              path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw4/00-bootstrap.txt"
        after_step_succeeds:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw4.log"
              event_fields: [step_name, duration_ms]
              level: info
    step_01_load_categories:
      generative_entity: "${models.qwen35}"
      depends_on: [step_00_bootstrap]
      prompt: |
        You are loading the agentic categorization for YAML translation.
        CATEGORIES DOC:
        {{bookmarks.shell_output.stdout}}
        Extract a flat list of tasks with their primary category. One per line:
        T1 - <name> | CATEGORY: <cat> | BUDGET: <n>
        T1.1 - <name> | CATEGORY: <cat> | BUDGET: <n>
        ...
        Output ONLY the flat list.
      model_overrides:
        temperature: 0.1
        max_tokens: 1000
      when:
        before_step_starts:
          - shell:
              command: "cat ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/categories.md"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true
        after_step_succeeds:
          - save_to:
              - sw4_task_cats
              - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw4/01-task-cats.txt"
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw4.log"
              event_fields: [step_name, duration_ms, total_tokens]
              level: info
    step_02_emit_substructures:
      generative_entity: "${models.qwen35}"
      depends_on: [step_01_load_categories]
      prompt: |
        You are translating categorized tasks into YAML agentic substructures.
        TASK-CATEGORY LIST:
        {{step.step_01_load_categories.output}}
        FULL CATEGORIES DOC (for reasoning/substructure hints):
        {{bookmarks.shell_output.stdout}}
        For EACH task, emit a YAML substructure snippet showing how it becomes a workflow step.
        ## Category → YAML Pattern Reference
        SEQUENTIAL_PROCESSOR:
        ```yaml
        step_<id>_<name>:
          generative_entity: "${models.qwen35}"
          depends_on: [step_<prior>]
          prompt: |
            <prompt text>
          model_overrides: {max_tokens: <N>, temperature: <T>}
          when:
            after_step_succeeds:
              - save_to: [<var>, "<path>"]
              - log: {to_file_path: "<log>", event_fields: [step_name, duration_ms]}
        ```
        ITERATIVE_REFINER:
        ```yaml
        step_<id>_<name>_generate:
          generative_entity: "${models.qwen35}"
          prompt: |
            <generation prompt>
          when:
            after_step_succeeds:
              - save_to: [<var>, "<path>"]
        step_<id>_<name>_evaluate:
          generative_entity: "${models.qwen35}"
          depends_on: [step_<id>_<name>_generate]
          prompt: |
            <eval prompt ending with VERDICT: PASS or FAIL>
          when:
            after_step_succeeds:
              - gwt:
                  - given: "contains 'VERDICT: PASS'"
                    then: { route_to: step_<id>_<name>_finalize }
                  - given: "true"
                    then: { route_to: step_<id>_<name>_fix }
        step_<id>_<name>_fix:
          generative_entity: "${models.qwen35}"
          prompt: |
            <fix prompt>
        ```
        CONDITIONAL_ROUTER:
        ```yaml
        step_<id>_<name>_route:
          generative_entity: "${models.qwen35}"
          prompt: |
            <eval prompt>
          when:
            after_step_succeeds:
              - gwt:
                  - given: "<condition>"
                    then: { route_to: [<target_a>] }
                  - given: "<condition>"
                    then: { route_to: [<target_b>] }
        ```
        DATA_TRANSFORMER:
        ```yaml
        step_<id>_<name>:
          generative_entity: "${models.qwen35}"
          prompt: |
            <transform prompt>
          when:
            after_step_succeeds:
              - save_to: [<var>, "<path>"]
        ```
        PARALLEL_FAN_OUT:
        ```yaml
        step_<id>_<name>_branch_a:
          generative_entity: "${models.qwen35}"
          prompt: |
            <branch A prompt>
        step_<id>_<name>_branch_b:
          generative_entity: "${models.qwen35}"
          prompt: |
            <branch B prompt>
        step_<id>_<name>_merge:
          depends_on: [step_<id>_<name>_branch_a, step_<id>_<name>_branch_b]
          generative_entity: "${models.qwen35}"
          prompt: |
            Merge: {{step.step_<id>_<name>_branch_a.output}} + {{step.step_<id>_<name>_branch_b.output}}
        ```
        ACCUMULATOR:
        ```yaml
        step_<id>_<name>_loop:
          generative_entity: "${models.qwen35}"
          prompt: |
            Process item: {{loop.current_item}}
          when:
            before_step_starts:
              - iterate_values:
                  current_item: [<item1>, <item2>, <item3>]
            after_step_succeeds:
              - append_to: [<accumulated_var>, "<path>"]
        ```
        For EACH task from the list, output:
        ### T<id> - <name> (<category>, budget=<n>)
        Intent: <2-3 sentences describing what this substructure accomplishes and why this pattern fits>
        ```yaml
        <YAML substructure snippet>
        ```
        Fit analysis: <how this substructure integrates with the rest of the workflow>
        Rules (STRICT):
        - Output ONLY ```yaml code blocks for the substructure snippets (no other code block types)
        - Every YAML block must be valid YAML (correct indentation, quoting)
        - Use `step_<id>_<name>` as step IDs (snake_case, no hyphens)
        - Reference models as `${models.qwen35}`
        - Reference prior outputs as `{{step.<step_id>.output}}`
        - Every step must have `generative_entity` and `prompt`
        - Keep prompts SHORT but meaningful (use <placeholder text> for actual content)
        - Cover EVERY task from the task-category list
        Output ONLY the task substructure blocks.
      model_overrides:
        temperature: 0.2
        max_tokens: 4000
      when:
        before_step_starts:
          - shell:
              command: "cat ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/categories.md"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true
        after_step_succeeds:
          - save_to:
              - sw4_substructures
              - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw4/02-substructures.md"
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw4.log"
              event_fields: [step_name, duration_ms, total_tokens]
              level: info
    step_03_evaluate:
      generative_entity: "${models.qwen35}"
      depends_on: [step_02_emit_substructures]
      prompt: |
        You are reviewing YAML substructures for correctness.
        TASK-CATEGORY LIST:
        {{step.step_01_load_categories.output}}
        SUBSTRUCTURES:
        {{step.step_02_emit_substructures.output}}
        Evaluate EACH criterion. Output PASS/FAIL with reason:
        1. COVERAGE: Every task from the list has a substructure block
        2. YAML_ONLY: All code blocks are ```yaml (no other types)
        3. VALID_YAML: Each YAML snippet is syntactically valid (correct indentation/quotes)
        4. HAS_GENERATIVE_ENTITY: Every step YAML has `generative_entity`
        5. HAS_PROMPT: Every step YAML has `prompt`
        6. CORRECT_PATTERN: The YAML matches the category pattern (e.g., ITERATIVE_REFINER has generate+evaluate+fix)
        7. INTEGRATION: Each block has "Fit analysis" describing workflow integration
        End with:
        VERDICT: PASS
        VERDICT: FAIL
        Output ONLY the evaluation.
      model_overrides:
        temperature: 0.1
        max_tokens: 800
      when:
        after_step_succeeds:
          - save_to:
              - sw4_eval
              - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw4/03-eval.txt"
          - gwt:
              - given: "{{step.step_03_evaluate.output}} contains 'VERDICT: PASS'"
                then: { route_to: step_05_assemble_final }
              - given: "true"
                then: { route_to: step_04_fix }
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw4.log"
              event_fields: [step_name, duration_ms, total_tokens]
              level: info
    step_04_fix:
      generative_entity: "${models.qwen35}"
      depends_on: [step_03_evaluate]
      prompt: |
        You are fixing YAML substructure issues.
        TASK-CATEGORY LIST:
        {{step.step_01_load_categories.output}}
        CURRENT SUBSTRUCTURES:
        {{step.step_02_emit_substructures.output}}
        EVALUATION FAILURES:
        {{step.step_03_evaluate.output}}
        Output CORRECTED substructure blocks (same format). Address every FAIL.
        Output ONLY the corrected blocks.
      model_overrides:
        temperature: 0.2
        max_tokens: 4000
      when:
        after_step_succeeds:
          - save_to:
              - sw4_corrected
              - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw4/04-corrected.md"
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw4.log"
              event_fields: [step_name, duration_ms, total_tokens]
              level: info
    step_05_assemble_final:
      generative_entity: "${models.qwen35}"
      depends_on: [step_03_evaluate]
      prompt: |
        You are assembling the final YAML substructures document.
        CATEGORIES SOURCE:
        {{bookmarks.shell_output.stdout}}
        SUBSTRUCTURES (initial):
        {{step.step_02_emit_substructures.output}}
        CORRECTED (if any):
        {{step.step_04_fix.output}}
        EVALUATION:
        {{step.step_03_evaluate.output}}
        Assemble a SINGLE markdown document:
        # YAML Substructures
        **Source:** <first 150 chars of categories doc>
        **Total substructures:** <N>
        **Verdict:** <PASS/FAIL>
        ---
        ## Pattern Summary
        | Category | Step Count | Hooks Used |
        |----------|-----------|------------|
        | SEQUENTIAL_PROCESSOR | <n> | save_to, log |
        | ITERATIVE_REFINER | <n> | gwt, route_to |
        ...
        ---
        ## T1 - <name> (<category>)
        Intent: <what this does>
        ```yaml
        <substructure YAML>
        ```
        Fit analysis: <integration description>
        ---
        (continue for ALL tasks)
        Rules:
        - ONLY ```yaml code blocks (no ```rust, ```python, etc.)
        - Cover EVERY task
        - Include intent + fit analysis for each
        Output ONLY the final document.
      model_overrides:
        temperature: 0.1
        max_tokens: 4000
      when:
        before_step_starts:
          - shell:
              command: "cat ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/categories.md | head -20"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true
        after_step_succeeds:
          - save_to:
              - sw4_final
              - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw4/structs.md"
          - bookmark: true
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw4.log"
              event_fields: [step_name, duration_ms, total_tokens]
              level: info

workflow_id: sw5_final_workflow_assembly
name: "SW5 Final Workflow Assembly"
description: "Assembles the final executable agentic workflow .yml file from substructures. Most critical step — exhaustive task coverage required."
version: "1.0.0"
author: "Whitt Execution Engine"
tags: [sub-workflow, final-assembly, yaml-generation]
schema_version: "2.0.0"
min_schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
models:
  "qwen35":
    name: "Qwen3-5-9B-Q4_K_M.gguf"
    host:
      type: llama_cpp_with_vulkan
workflow_execution_strategy:
  memory:
    model_lifecycle:
      unload_unused: false
agentic_workflow:
  when:
    after_workflow:
      - log:
          to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw5.log"
          event_fields: [workflow_id, total_steps, succeeded, failed]
          level: info
  steps:
    step_00_bootstrap:
      generative_entity: "${models.qwen35}"
      prompt: |
        Bootstrapping SW5 (final-workflow-assembly). Output "READY".
      model_overrides:
        temperature: 0.0
        max_tokens: 10
      when:
        before_step_starts:
          - shell:
              command: "mkdir -p ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/{sw5,logs,input} && head -30 ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/structs.md"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true
          - bookmark:
              path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw5/00-bootstrap.txt"
        after_step_succeeds:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw5.log"
              event_fields: [step_name, duration_ms]
              level: info
    step_01_inventory_tasks:
      generative_entity: "${models.qwen35}"
      depends_on: [step_00_bootstrap]
      prompt: |
        You are inventorying all tasks that must appear in the final workflow.
        SUBSTRUCTURES DOC:
        {{bookmarks.shell_output.stdout}}
        Extract EVERY task ID mentioned (T1, T1.1, T1.2, T2, ...). One per line:
        T1 - <name>
        T1.1 - <name>
        ...
        Also note the step_count: <N> on the last line.
        Output ONLY the inventory.
      model_overrides:
        temperature: 0.1
        max_tokens: 1000
      when:
        before_step_starts:
          - shell:
              command: "cat ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/structs.md"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true
        after_step_succeeds:
          - save_to:
              - sw5_inventory
              - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw5/01-inventory.txt"
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw5.log"
              event_fields: [step_name, duration_ms, total_tokens]
              level: info
    step_02_emit_skeleton:
      generative_entity: "${models.qwen35}"
      depends_on: [step_01_inventory_tasks]
      prompt: |
        You are emitting the workflow skeleton (header + providers + models + strategy).
        TASK INVENTORY:
        {{step.step_01_inventory_tasks.output}}
        Output ```yaml code block with ONLY the header/skeleton (no steps yet):
        ```yaml
        workflow_id: generated_workflow_<unique>
        name: "<descriptive name from task context>"
        description: "<one-line description>"
        version: "1.0.0"
        schema_version: "2.0.0"
        providers:
          llama_cpp_with_vulkan:
            config:
              host: localhost
              port: 8080
        models:
          "qwen35":
            name: "Qwen3-5-9B-Q4_K_M.gguf"
            host:
              type: llama_cpp_with_vulkan
        workflow_execution_strategy:
          memory:
            model_lifecycle:
              unload_unused: false
        agentic_workflow:
          when:
            after_workflow:
              - log:
                  to_file_path: "./outputs/workflow.log"
                  event_fields: [workflow_id, total_steps, succeeded, failed]
                  level: info
          steps:
        ```
        (steps will be added in next step)
        Output ONLY the yaml code block. No other text.
      model_overrides:
        temperature: 0.0
        max_tokens: 800
      when:
        after_step_succeeds:
          - save_to:
              - sw5_skeleton
              - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw5/02-skeleton.yml"
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw5.log"
              event_fields: [step_name, duration_ms, total_tokens]
              level: info
    step_03_assemble_steps:
      generative_entity: "${models.qwen35}"
      depends_on: [step_02_emit_skeleton]
      prompt: |
        You are assembling the complete workflow by combining skeleton + all step substructures.
        SKELETON:
        {{step.step_02_emit_skeleton.output}}
        SUBSTRUCTURES (from SW4 — these are the step definitions):
        {{bookmarks.shell_output.stdout}}
        TASK INVENTORY (verify all are covered):
        {{step.step_01_inventory_tasks.output}}
        Concatenate ALL step YAML snippets under the `steps:` key of the skeleton.
        CRITICAL RULES:
        1. Every task from the inventory MUST have a corresponding step in the output
        2. Preserve exact step IDs from the substructures (step_<id>_<name>)
        3. Maintain correct depends_on chains
        4. Keep all hooks (save_to, log, gwt, bookmark, shell) from substructures
        5. Ensure 2-space indentation throughout
        6. Output a SINGLE ```yaml code block with the COMPLETE workflow
        Output ONLY the complete workflow yaml. No explanation.
      model_overrides:
        temperature: 0.1
        max_tokens: 4500
      when:
        before_step_starts:
          - shell:
              command: "cat ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/structs.md"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true
        after_step_succeeds:
          - save_to:
              - sw5_assembled
              - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw5/03-assembled.yml"
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw5.log"
              event_fields: [step_name, duration_ms, total_tokens]
              level: info
    step_04_evaluate:
      generative_entity: "${models.qwen35}"
      depends_on: [step_03_assemble_steps]
      prompt: |
        You are exhaustively evaluating the assembled workflow.
        TASK INVENTORY (must all be covered):
        {{step.step_01_inventory_tasks.output}}
        ASSEMBLED WORKFLOW:
        {{step.step_03_assemble_steps.output}}
        Evaluate EACH criterion. Output PASS/FAIL with reason:
        1. COVERAGE_EXHAUSTIVE: Every task ID from inventory has a corresponding step in the workflow
        2. VALID_YAML: The output parses as valid YAML (correct indentation, no tabs, proper quoting)
        3. HAS_WORKFLOW_ID: workflow_id field present and non-empty
        4. HAS_PROVIDERS: providers.llama_cpp_with_vulkan present
        5. HAS_MODELS: models.qwen35 present
        6. STEPS_NON_EMPTY: At least one step under agentic_workflow.steps
        7. DEPENDS_ON_CORRECT: Each step's depends_on references exist as step IDs
        8. PROMPTS_PRESENT: Every step has a non-empty prompt
        9. HOOKS_PRESERVED: save_to, log, gwt hooks from substructures are present
        End with:
        VERDICT: PASS
        VERDICT: FAIL
        Output ONLY the evaluation.
      model_overrides:
        temperature: 0.1
        max_tokens: 1000
      when:
        after_step_succeeds:
          - save_to:
              - sw5_eval
              - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw5/04-eval.txt"
          - gwt:
              - given: "{{step.step_04_evaluate.output}} contains 'VERDICT: PASS'"
                then: { route_to: step_06_finalize }
              - given: "true"
                then: { route_to: step_05_fix }
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw5.log"
              event_fields: [step_name, duration_ms, total_tokens]
              level: info
    step_05_fix:
      generative_entity: "${models.qwen35}"
      depends_on: [step_04_evaluate]
      prompt: |
        You are fixing the assembled workflow.
        TASK INVENTORY:
        {{step.step_01_inventory_tasks.output}}
        CURRENT WORKFLOW:
        {{step.step_03_assemble_steps.output}}
        EVALUATION FAILURES:
        {{step.step_04_evaluate.output}}
        Output the COMPLETE corrected workflow YAML (same format as input — single ```yaml block).
        Address EVERY FAIL:
        - Missing tasks: add their step definitions
        - Invalid YAML: fix indentation/quotes
        - Missing hooks: add from substructures
        Output ONLY the corrected complete yaml.
      model_overrides:
        temperature: 0.1
        max_tokens: 4500
      when:
        after_step_succeeds:
          - save_to:
              - sw5_corrected
              - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw5/05-corrected.yml"
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw5.log"
              event_fields: [step_name, duration_ms, total_tokens]
              level: info
    step_06_finalize:
      generative_entity: "${models.qwen35}"
      depends_on: [step_04_evaluate]
      prompt: |
        You are finalizing the workflow file.
        ASSEMBLED (initial):
        {{step.step_03_assemble_steps.output}}
        CORRECTED (if any):
        {{step.step_05_fix.output}}
        EVALUATION:
        {{step.step_04_evaluate.output}}
        Output the FINAL workflow YAML — use corrected version if available, else initial.
        Strip any markdown formatting (no ```yaml fences). Output RAW YAML only.
        This is the final executable workflow file.
      model_overrides:
        temperature: 0.0
        max_tokens: 4500
      when:
        after_step_succeeds:
          - save_to:
              - sw5_final
              - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw5/workflow.yml"
          - bookmark: true
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw5.log"
              event_fields: [step_name, duration_ms, total_tokens]
              level: info