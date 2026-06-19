<!--
Source Session: ses_21eda916dffexLBSamby9C941e
Message Length: 91312 characters
YAML Sections: 8
Embedded Prompts: 73
Agentic Keywords: 7
Complexity: HIGH
Source Files: benchmark-15-models.yml, benchmark-3-models.yml, benchmark-5-models.yml
-->

# ═══════════════════════════════════════════════════════════════════
# ADR Generation Benchmark — 15 Models
# Schema-compliant per docs/schema/unified-workflow-schema.yml v2.0
# Each model = separate step with generative_entity reference
# ═══════════════════════════════════════════════════════════════════
# ── PROVIDERS ───────────────────────────────────────────────────
  llama_cpp_with_vulkan:                               # schema line 28
    config:                                             # schema line 29
      host: localhost                                   # schema line 30
      port: 8080                                        # schema line 31
# ── MODELS (each individually listed) ──────────────────────────
# Model name field used to match GGUF file in models-dir
  "qwen-05b-q4":                                       # schema line 68
    name: "Qwen2.5-0.5B-Instruct-Q4_K_M"
    host:                                               # schema line 70
      type: llama_cpp_with_vulkan                       # schema line 71
  "qwen-05b-q5":
    name: "Qwen2.5-0.5B-Instruct-Q5_K_M"
  "qwen-05b-q8":
    name: "Qwen2.5-0.5B-Instruct-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "qwen-15b-q4":
    name: "Qwen2.5-1.5B-Instruct-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "qwen-15b-q5":
    name: "Qwen2.5-1.5B-Instruct-Q5_K_M"
    host:
      type: llama_cpp_with_vulkan
  "qwen-15b-q8":
    name: "Qwen2.5-1.5B-Instruct-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "qwen-3b-q4":
    name: "Qwen2.5-3B-Instruct-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "qwen-3b-q5":
    name: "Qwen2.5-3B-Instruct-Q5_K_M"
    host:
      type: llama_cpp_with_vulkan
  "qwen-3b-q8":
    name: "Qwen2.5-3B-Instruct-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "qwen-7b-q4":
    name: "Qwen2.5-7B-Instruct-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "qwen-7b-q5":
    name: "Qwen2.5-7B-Instruct-Q5_K_M"
    host:
      type: llama_cpp_with_vulkan
  "qwen-7b-q8":
    name: "Qwen2.5-7B-Instruct-Q8_0"
    host:
      type: llama_cpp_with_vulkan
  "qwen-14b-q4":
    name: "Qwen2.5-14B-Instruct-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "qwen-14b-q5":
    name: "Qwen2.5-14B-Instruct-Q5_K_M"
    host:
      type: llama_cpp_with_vulkan
  "qwen-14b-q8":
    name: "Qwen2.5-14B-Instruct-Q8_0"
    host:
      type: llama_cpp_with_vulkan
# ── WORKFLOW EXECUTION STRATEGY ────────────────────────────────
workflow_execution_strategy:                             # schema line 503
      unload_unused: true                               # schema line 521
# ── AGENTIC WORKFLOW ───────────────────────────────────────────
# One step per model. Each step references its model via
# generative_entity. Hooks drive output (save_to, log).
# No loop.count for model iteration — each model is a separate step.
# No input: key — model_overrides for per-model config.
agentic_workflow:                                        # schema line 196
  steps:                                                # schema line 293
    # ── Step 1: Qwen2.5-0.5B-Q4 ───────────────────────────────
    benchmark_qwen_05b_q4:
      generative_entity: "${models.qwen-05b-q4}"        # schema line 297
      prompt: |                                         # schema line 298
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:                                  # schema line 303
        max_tokens: 4096
      when:                                             # schema line 314
        after_step_succeeds:                            # schema line 323
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-0.5B-Instruct-Q4_K_M.json"  # schema line 476
          - log:                                        # schema line 275
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:                               # schema line 328
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 2: Qwen2.5-0.5B-Q5 ───────────────────────────────
    benchmark_qwen_05b_q5:
      generative_entity: "${models.qwen-05b-q5}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-0.5B-Instruct-Q5_K_M.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 3: Qwen2.5-0.5B-Q8 ───────────────────────────────
    benchmark_qwen_05b_q8:
      generative_entity: "${models.qwen-05b-q8}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-0.5B-Instruct-Q8_0.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 4: Qwen2.5-1.5B-Q4 ───────────────────────────────
    benchmark_qwen_15b_q4:
      generative_entity: "${models.qwen-15b-q4}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-1.5B-Instruct-Q4_K_M.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 5: Qwen2.5-1.5B-Q5 ───────────────────────────────
    benchmark_qwen_15b_q5:
      generative_entity: "${models.qwen-15b-q5}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-1.5B-Instruct-Q5_K_M.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 6: Qwen2.5-1.5B-Q8 ───────────────────────────────
    benchmark_qwen_15b_q8:
      generative_entity: "${models.qwen-15b-q8}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-1.5B-Instruct-Q8_0.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 7: Qwen2.5-3B-Q4 ────────────────────────────────
    benchmark_qwen_3b_q4:
      generative_entity: "${models.qwen-3b-q4}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-3B-Instruct-Q4_K_M.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 8: Qwen2.5-3B-Q5 ────────────────────────────────
    benchmark_qwen_3b_q5:
      generative_entity: "${models.qwen-3b-q5}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-3B-Instruct-Q5_K_M.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 9: Qwen2.5-3B-Q8 ────────────────────────────────
    benchmark_qwen_3b_q8:
      generative_entity: "${models.qwen-3b-q8}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-3B-Instruct-Q8_0.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 10: Qwen2.5-7B-Q4 ───────────────────────────────
    benchmark_qwen_7b_q4:
      generative_entity: "${models.qwen-7b-q4}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-7B-Instruct-Q4_K_M.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 11: Qwen2.5-7B-Q5 ───────────────────────────────
    benchmark_qwen_7b_q5:
      generative_entity: "${models.qwen-7b-q5}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-7B-Instruct-Q5_K_M.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 12: Qwen2.5-7B-Q8 ───────────────────────────────
    benchmark_qwen_7b_q8:
      generative_entity: "${models.qwen-7b-q8}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-7B-Instruct-Q8_0.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 13: Qwen2.5-14B-Q4 ──────────────────────────────
    benchmark_qwen_14b_q4:
      generative_entity: "${models.qwen-14b-q4}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-14B-Instruct-Q4_K_M.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 14: Qwen2.5-14B-Q5 ──────────────────────────────
    benchmark_qwen_14b_q5:
      generative_entity: "${models.qwen-14b-q5}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-14B-Instruct-Q5_K_M.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 15: Qwen2.5-14B-Q8 ──────────────────────────────
    benchmark_qwen_14b_q8:
      generative_entity: "${models.qwen-14b-q8}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-14B-Instruct-Q8_0.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]

# ═══════════════════════════════════════════════════════════════════
# ADR Generation Benchmark — 3 Models
# Schema-compliant per docs/schema/unified-workflow-schema.yml v2.0
# Each model = separate step with generative_entity reference
# ═══════════════════════════════════════════════════════════════════
# ── PROVIDERS ───────────────────────────────────────────────────
  llama_cpp_with_vulkan:                               # schema line 28
    config:                                             # schema line 29
      host: localhost                                   # schema line 30
      port: 8080                                        # schema line 31
# ── MODELS (each individually listed) ──────────────────────────
# Model name field used to match GGUF file in models-dir
  "qwen-05b":                                           # schema line 68
    name: "Qwen2.5-0.5B-Instruct-Q4_K_M"
    host:                                               # schema line 70
      type: llama_cpp_with_vulkan                       # schema line 71
  "qwen-15b":
    name: "Qwen2.5-1.5B-Instruct-Q4_K_M"
  "qwen-3b":
    name: "Qwen2.5-3B-Instruct-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
# ── WORKFLOW EXECUTION STRATEGY ────────────────────────────────
workflow_execution_strategy:                             # schema line 503
      unload_unused: true                               # schema line 521
# ── AGENTIC WORKFLOW ───────────────────────────────────────────
# One step per model. Each step references its model via
# generative_entity. Hooks drive output (save_to, log).
# No loop.count for model iteration — each model is a separate step.
# No input: key — model_overrides for per-model config.
agentic_workflow:                                        # schema line 196
  steps:                                                # schema line 293
    # ── Step 1: Qwen2.5-0.5B ─────────────────────────────────
    benchmark_qwen_05b:
      generative_entity: "${models.qwen-05b}"           # schema line 297
      prompt: |                                         # schema line 298
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:                                  # schema line 303
        max_tokens: 4096
      when:                                             # schema line 314
        after_step_succeeds:                            # schema line 323
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-0.5B-Instruct-Q4_K_M.json"  # schema line 476
          - log:                                        # schema line 275
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:                               # schema line 328
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 2: Qwen2.5-1.5B ─────────────────────────────────
    benchmark_qwen_15b:
      generative_entity: "${models.qwen-15b}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-1.5B-Instruct-Q4_K_M.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 3: Qwen2.5-3B ───────────────────────────────────
    benchmark_qwen_3b:
      generative_entity: "${models.qwen-3b}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-3B-Instruct-Q4_K_M.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]

# ═══════════════════════════════════════════════════════════════════
# ADR Generation Benchmark — 5 Models
# Schema-compliant per docs/schema/unified-workflow-schema.yml v2.0
# Each model = separate step with generative_entity reference
# ═══════════════════════════════════════════════════════════════════
# ── PROVIDERS ───────────────────────────────────────────────────
  llama_cpp_with_vulkan:                               # schema line 28
    config:                                             # schema line 29
      host: localhost                                   # schema line 30
      port: 8080                                        # schema line 31
# ── MODELS (each individually listed) ──────────────────────────
# Model name field used to match GGUF file in models-dir
  "qwen-05b":                                           # schema line 68
    name: "Qwen2.5-0.5B-Instruct-Q4_K_M"
    host:                                               # schema line 70
      type: llama_cpp_with_vulkan                       # schema line 71
  "qwen-15b":
    name: "Qwen2.5-1.5B-Instruct-Q4_K_M"
  "qwen-3b":
    name: "Qwen2.5-3B-Instruct-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "qwen-7b":
    name: "Qwen2.5-7B-Instruct-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  "qwen-14b":
    name: "Qwen2.5-14B-Instruct-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
# ── WORKFLOW EXECUTION STRATEGY ────────────────────────────────
workflow_execution_strategy:                             # schema line 503
      unload_unused: true                               # schema line 521
# ── AGENTIC WORKFLOW ───────────────────────────────────────────
# One step per model. Each step references its model via
# generative_entity. Hooks drive output (save_to, log).
# No loop.count for model iteration — each model is a separate step.
# No input: key — model_overrides for per-model config.
agentic_workflow:                                        # schema line 196
  steps:                                                # schema line 293
    # ── Step 1: Qwen2.5-0.5B ─────────────────────────────────
    benchmark_qwen_05b:
      generative_entity: "${models.qwen-05b}"           # schema line 297
      prompt: |                                         # schema line 298
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:                                  # schema line 303
        max_tokens: 4096
      when:                                             # schema line 314
        after_step_succeeds:                            # schema line 323
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-0.5B-Instruct-Q4_K_M.json"  # schema line 476
          - log:                                        # schema line 275
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:                               # schema line 328
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 2: Qwen2.5-1.5B ─────────────────────────────────
    benchmark_qwen_15b:
      generative_entity: "${models.qwen-15b}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-1.5B-Instruct-Q4_K_M.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 3: Qwen2.5-3B ───────────────────────────────────
    benchmark_qwen_3b:
      generative_entity: "${models.qwen-3b}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-3B-Instruct-Q4_K_M.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 4: Qwen2.5-7B ───────────────────────────────────
    benchmark_qwen_7b:
      generative_entity: "${models.qwen-7b}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-7B-Instruct-Q4_K_M.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 5: Qwen2.5-14B ──────────────────────────────────
    benchmark_qwen_14b:
      generative_entity: "${models.qwen-14b}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-14B-Instruct-Q4_K_M.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]

# ═══════════════════════════════════════════════════════════════════
# ADR Generation Benchmark — 50 Models
# Schema-compliant per docs/schema/unified-workflow-schema.yml v2.0
# Each model = separate step with generative_entity reference
# ═══════════════════════════════════════════════════════════════════
# ── PROVIDERS ───────────────────────────────────────────────────
  llama_cpp_with_vulkan:                               # schema line 28
    config:                                             # schema line 29
      host: localhost                                   # schema line 30
      port: 8080                                        # schema line 31
# ── MODELS (each individually listed) ──────────────────────────
# Model name field used to match GGUF file in models-dir
  "qwen-05b-q4_0":                                     # schema line 68
    name: "Qwen2.5-0.5B-Instruct-Q4_0.gguf"
    host:                                               # schema line 70
      type: llama_cpp_with_vulkan                       # schema line 71
  "qwen-05b-q4_k_m":
    name: "Qwen2.5-0.5B-Instruct-Q4_K_M.gguf"
  "qwen-05b-q5_0":
    name: "Qwen2.5-0.5B-Instruct-Q5_0.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-05b-q5_k_m":
    name: "Qwen2.5-0.5B-Instruct-Q5_K_M.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-05b-q6_k":
    name: "Qwen2.5-0.5B-Instruct-Q6_K.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-05b-q8_0":
    name: "Qwen2.5-0.5B-Instruct-Q8_0.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-05b-q2_k":
    name: "Qwen2.5-0.5B-Instruct-Q2_K.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-05b-q3_k_m":
    name: "Qwen2.5-0.5B-Instruct-Q3_K_M.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-05b-q3_k_s":
    name: "Qwen2.5-0.5B-Instruct-Q3_K_S.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-05b-iq3_m":
    name: "Qwen2.5-0.5B-Instruct-IQ3_M.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-15b-q4_0":
    name: "Qwen2.5-1.5B-Instruct-Q4_0.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-15b-q4_k_m":
    name: "Qwen2.5-1.5B-Instruct-Q4_K_M.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-15b-q5_0":
    name: "Qwen2.5-1.5B-Instruct-Q5_0.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-15b-q5_k_m":
    name: "Qwen2.5-1.5B-Instruct-Q5_K_M.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-15b-q6_k":
    name: "Qwen2.5-1.5B-Instruct-Q6_K.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-15b-q8_0":
    name: "Qwen2.5-1.5B-Instruct-Q8_0.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-15b-q2_k":
    name: "Qwen2.5-1.5B-Instruct-Q2_K.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-15b-q3_k_m":
    name: "Qwen2.5-1.5B-Instruct-Q3_K_M.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-15b-q3_k_s":
    name: "Qwen2.5-1.5B-Instruct-Q3_K_S.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-15b-iq3_m":
    name: "Qwen2.5-1.5B-Instruct-IQ3_M.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-3b-q4_0":
    name: "Qwen2.5-3B-Instruct-Q4_0.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-3b-q4_k_m":
    name: "Qwen2.5-3B-Instruct-Q4_K_M.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-3b-q5_0":
    name: "Qwen2.5-3B-Instruct-Q5_0.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-3b-q5_k_m":
    name: "Qwen2.5-3B-Instruct-Q5_K_M.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-3b-q6_k":
    name: "Qwen2.5-3B-Instruct-Q6_K.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-3b-q8_0":
    name: "Qwen2.5-3B-Instruct-Q8_0.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-3b-q2_k":
    name: "Qwen2.5-3B-Instruct-Q2_K.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-3b-q3_k_m":
    name: "Qwen2.5-3B-Instruct-Q3_K_M.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-3b-q3_k_s":
    name: "Qwen2.5-3B-Instruct-Q3_K_S.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-3b-iq3_m":
    name: "Qwen2.5-3B-Instruct-IQ3_M.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-7b-q4_0":
    name: "Qwen2.5-7B-Instruct-Q4_0.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-7b-q4_k_m":
    name: "Qwen2.5-7B-Instruct-Q4_K_M.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-7b-q5_0":
    name: "Qwen2.5-7B-Instruct-Q5_0.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-7b-q5_k_m":
    name: "Qwen2.5-7B-Instruct-Q5_K_M.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-7b-q6_k":
    name: "Qwen2.5-7B-Instruct-Q6_K.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-7b-q8_0":
    name: "Qwen2.5-7B-Instruct-Q8_0.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-7b-q2_k":
    name: "Qwen2.5-7B-Instruct-Q2_K.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-7b-q3_k_m":
    name: "Qwen2.5-7B-Instruct-Q3_K_M.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-7b-q3_k_s":
    name: "Qwen2.5-7B-Instruct-Q3_K_S.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-7b-iq3_m":
    name: "Qwen2.5-7B-Instruct-IQ3_M.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-14b-q4_0":
    name: "Qwen2.5-14B-Instruct-Q4_0.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-14b-q4_k_m":
    name: "Qwen2.5-14B-Instruct-Q4_K_M.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-14b-q5_0":
    name: "Qwen2.5-14B-Instruct-Q5_0.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-14b-q5_k_m":
    name: "Qwen2.5-14B-Instruct-Q5_K_M.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-14b-q6_k":
    name: "Qwen2.5-14B-Instruct-Q6_K.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-14b-q8_0":
    name: "Qwen2.5-14B-Instruct-Q8_0.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-14b-q2_k":
    name: "Qwen2.5-14B-Instruct-Q2_K.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-14b-q3_k_m":
    name: "Qwen2.5-14B-Instruct-Q3_K_M.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-14b-q3_k_s":
    name: "Qwen2.5-14B-Instruct-Q3_K_S.gguf"
    host:
      type: llama_cpp_with_vulkan
  "qwen-14b-iq3_m":
    name: "Qwen2.5-14B-Instruct-IQ3_M.gguf"
    host:
      type: llama_cpp_with_vulkan
# ── WORKFLOW EXECUTION STRATEGY ────────────────────────────────
workflow_execution_strategy:                             # schema line 503
      unload_unused: true                               # schema line 521
# ── AGENTIC WORKFLOW ───────────────────────────────────────────
# One step per model. Each step references its model via
# generative_entity. Hooks drive output (save_to, log).
# No loop.count for model iteration — each model is a separate step.
# No input: key — model_overrides for per-model config.
agentic_workflow:                                        # schema line 196
  steps:                                                # schema line 293
    # ── Step 1: Qwen2.5-0.5B-Q4_0 ────────────────────────────
    benchmark_qwen_05b_q4_0:
      generative_entity: "${models.qwen-05b-q4_0}"      # schema line 297
      prompt: |                                         # schema line 298
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:                                  # schema line 303
        max_tokens: 4096
      when:                                             # schema line 314
        after_step_succeeds:                            # schema line 323
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-0.5B-Instruct-Q4_0.gguf.json"  # schema line 476
          - log:                                        # schema line 275
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:                               # schema line 328
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 2: Qwen2.5-0.5B-Q4_K_M ─────────────────────────
    benchmark_qwen_05b_q4_k_m:
      generative_entity: "${models.qwen-05b-q4_k_m}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-0.5B-Instruct-Q4_K_M.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 3: Qwen2.5-0.5B-Q5_0 ────────────────────────────
    benchmark_qwen_05b_q5_0:
      generative_entity: "${models.qwen-05b-q5_0}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-0.5B-Instruct-Q5_0.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 4: Qwen2.5-0.5B-Q5_K_M ─────────────────────────
    benchmark_qwen_05b_q5_k_m:
      generative_entity: "${models.qwen-05b-q5_k_m}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-0.5B-Instruct-Q5_K_M.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 5: Qwen2.5-0.5B-Q6_K ────────────────────────────
    benchmark_qwen_05b_q6_k:
      generative_entity: "${models.qwen-05b-q6_k}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-0.5B-Instruct-Q6_K.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 6: Qwen2.5-0.5B-Q8_0 ────────────────────────────
    benchmark_qwen_05b_q8_0:
      generative_entity: "${models.qwen-05b-q8_0}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-0.5B-Instruct-Q8_0.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 7: Qwen2.5-0.5B-Q2_K ────────────────────────────
    benchmark_qwen_05b_q2_k:
      generative_entity: "${models.qwen-05b-q2_k}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-0.5B-Instruct-Q2_K.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 8: Qwen2.5-0.5B-Q3_K_M ─────────────────────────
    benchmark_qwen_05b_q3_k_m:
      generative_entity: "${models.qwen-05b-q3_k_m}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-0.5B-Instruct-Q3_K_M.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 9: Qwen2.5-0.5B-Q3_K_S ─────────────────────────
    benchmark_qwen_05b_q3_k_s:
      generative_entity: "${models.qwen-05b-q3_k_s}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-0.5B-Instruct-Q3_K_S.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 10: Qwen2.5-0.5B-IQ3_M ─────────────────────────
    benchmark_qwen_05b_iq3_m:
      generative_entity: "${models.qwen-05b-iq3_m}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-0.5B-Instruct-IQ3_M.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 11: Qwen2.5-1.5B-Q4_0 ───────────────────────────
    benchmark_qwen_15b_q4_0:
      generative_entity: "${models.qwen-15b-q4_0}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-1.5B-Instruct-Q4_0.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 12: Qwen2.5-1.5B-Q4_K_M ─────────────────────────
    benchmark_qwen_15b_q4_k_m:
      generative_entity: "${models.qwen-15b-q4_k_m}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-1.5B-Instruct-Q4_K_M.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 13: Qwen2.5-1.5B-Q5_0 ───────────────────────────
    benchmark_qwen_15b_q5_0:
      generative_entity: "${models.qwen-15b-q5_0}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-1.5B-Instruct-Q5_0.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 14: Qwen2.5-1.5B-Q5_K_M ─────────────────────────
    benchmark_qwen_15b_q5_k_m:
      generative_entity: "${models.qwen-15b-q5_k_m}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-1.5B-Instruct-Q5_K_M.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 15: Qwen2.5-1.5B-Q6_K ───────────────────────────
    benchmark_qwen_15b_q6_k:
      generative_entity: "${models.qwen-15b-q6_k}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-1.5B-Instruct-Q6_K.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 16: Qwen2.5-1.5B-Q8_0 ───────────────────────────
    benchmark_qwen_15b_q8_0:
      generative_entity: "${models.qwen-15b-q8_0}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-1.5B-Instruct-Q8_0.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 17: Qwen2.5-1.5B-Q2_K ───────────────────────────
    benchmark_qwen_15b_q2_k:
      generative_entity: "${models.qwen-15b-q2_k}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-1.5B-Instruct-Q2_K.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 18: Qwen2.5-1.5B-Q3_K_M ─────────────────────────
    benchmark_qwen_15b_q3_k_m:
      generative_entity: "${models.qwen-15b-q3_k_m}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-1.5B-Instruct-Q3_K_M.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 19: Qwen2.5-1.5B-Q3_K_S ─────────────────────────
    benchmark_qwen_15b_q3_k_s:
      generative_entity: "${models.qwen-15b-q3_k_s}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-1.5B-Instruct-Q3_K_S.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 20: Qwen2.5-1.5B-IQ3_M ─────────────────────────
    benchmark_qwen_15b_iq3_m:
      generative_entity: "${models.qwen-15b-iq3_m}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-1.5B-Instruct-IQ3_M.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 21: Qwen2.5-3B-Q4_0 ────────────────────────────
    benchmark_qwen_3b_q4_0:
      generative_entity: "${models.qwen-3b-q4_0}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-3B-Instruct-Q4_0.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 22: Qwen2.5-3B-Q4_K_M ───────────────────────────
    benchmark_qwen_3b_q4_k_m:
      generative_entity: "${models.qwen-3b-q4_k_m}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-3B-Instruct-Q4_K_M.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 23: Qwen2.5-3B-Q5_0 ────────────────────────────
    benchmark_qwen_3b_q5_0:
      generative_entity: "${models.qwen-3b-q5_0}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-3B-Instruct-Q5_0.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 24: Qwen2.5-3B-Q5_K_M ───────────────────────────
    benchmark_qwen_3b_q5_k_m:
      generative_entity: "${models.qwen-3b-q5_k_m}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-3B-Instruct-Q5_K_M.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 25: Qwen2.5-3B-Q6_K ────────────────────────────
    benchmark_qwen_3b_q6_k:
      generative_entity: "${models.qwen-3b-q6_k}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-3B-Instruct-Q6_K.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 26: Qwen2.5-3B-Q8_0 ────────────────────────────
    benchmark_qwen_3b_q8_0:
      generative_entity: "${models.qwen-3b-q8_0}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-3B-Instruct-Q8_0.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 27: Qwen2.5-3B-Q2_K ────────────────────────────
    benchmark_qwen_3b_q2_k:
      generative_entity: "${models.qwen-3b-q2_k}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-3B-Instruct-Q2_K.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 28: Qwen2.5-3B-Q3_K_M ───────────────────────────
    benchmark_qwen_3b_q3_k_m:
      generative_entity: "${models.qwen-3b-q3_k_m}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-3B-Instruct-Q3_K_M.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 29: Qwen2.5-3B-Q3_K_S ───────────────────────────
    benchmark_qwen_3b_q3_k_s:
      generative_entity: "${models.qwen-3b-q3_k_s}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-3B-Instruct-Q3_K_S.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 30: Qwen2.5-3B-IQ3_M ───────────────────────────
    benchmark_qwen_3b_iq3_m:
      generative_entity: "${models.qwen-3b-iq3_m}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-3B-Instruct-IQ3_M.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 31: Qwen2.5-7B-Q4_0 ────────────────────────────
    benchmark_qwen_7b_q4_0:
      generative_entity: "${models.qwen-7b-q4_0}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-7B-Instruct-Q4_0.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 32: Qwen2.5-7B-Q4_K_M ───────────────────────────
    benchmark_qwen_7b_q4_k_m:
      generative_entity: "${models.qwen-7b-q4_k_m}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-7B-Instruct-Q4_K_M.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 33: Qwen2.5-7B-Q5_0 ────────────────────────────
    benchmark_qwen_7b_q5_0:
      generative_entity: "${models.qwen-7b-q5_0}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-7B-Instruct-Q5_0.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 34: Qwen2.5-7B-Q5_K_M ───────────────────────────
    benchmark_qwen_7b_q5_k_m:
      generative_entity: "${models.qwen-7b-q5_k_m}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-7B-Instruct-Q5_K_M.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 35: Qwen2.5-7B-Q6_K ────────────────────────────
    benchmark_qwen_7b_q6_k:
      generative_entity: "${models.qwen-7b-q6_k}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-7B-Instruct-Q6_K.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 36: Qwen2.5-7B-Q8_0 ────────────────────────────
    benchmark_qwen_7b_q8_0:
      generative_entity: "${models.qwen-7b-q8_0}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-7B-Instruct-Q8_0.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 37: Qwen2.5-7B-Q2_K ────────────────────────────
    benchmark_qwen_7b_q2_k:
      generative_entity: "${models.qwen-7b-q2_k}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-7B-Instruct-Q2_K.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 38: Qwen2.5-7B-Q3_K_M ───────────────────────────
    benchmark_qwen_7b_q3_k_m:
      generative_entity: "${models.qwen-7b-q3_k_m}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-7B-Instruct-Q3_K_M.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 39: Qwen2.5-7B-Q3_K_S ───────────────────────────
    benchmark_qwen_7b_q3_k_s:
      generative_entity: "${models.qwen-7b-q3_k_s}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-7B-Instruct-Q3_K_S.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 40: Qwen2.5-7B-IQ3_M ───────────────────────────
    benchmark_qwen_7b_iq3_m:
      generative_entity: "${models.qwen-7b-iq3_m}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-7B-Instruct-IQ3_M.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 41: Qwen2.5-14B-Q4_0 ───────────────────────────
    benchmark_qwen_14b_q4_0:
      generative_entity: "${models.qwen-14b-q4_0}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-14B-Instruct-Q4_0.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 42: Qwen2.5-14B-Q4_K_M ──────────────────────────
    benchmark_qwen_14b_q4_k_m:
      generative_entity: "${models.qwen-14b-q4_k_m}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-14B-Instruct-Q4_K_M.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 43: Qwen2.5-14B-Q5_0 ───────────────────────────
    benchmark_qwen_14b_q5_0:
      generative_entity: "${models.qwen-14b-q5_0}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-14B-Instruct-Q5_0.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 44: Qwen2.5-14B-Q5_K_M ──────────────────────────
    benchmark_qwen_14b_q5_k_m:
      generative_entity: "${models.qwen-14b-q5_k_m}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-14B-Instruct-Q5_K_M.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 45: Qwen2.5-14B-Q6_K ───────────────────────────
    benchmark_qwen_14b_q6_k:
      generative_entity: "${models.qwen-14b-q6_k}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-14B-Instruct-Q6_K.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 46: Qwen2.5-14B-Q8_0 ───────────────────────────
    benchmark_qwen_14b_q8_0:
      generative_entity: "${models.qwen-14b-q8_0}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-14B-Instruct-Q8_0.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 47: Qwen2.5-14B-Q2_K ───────────────────────────
    benchmark_qwen_14b_q2_k:
      generative_entity: "${models.qwen-14b-q2_k}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-14B-Instruct-Q2_K.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 48: Qwen2.5-14B-Q3_K_M ──────────────────────────
    benchmark_qwen_14b_q3_k_m:
      generative_entity: "${models.qwen-14b-q3_k_m}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-14B-Instruct-Q3_K_M.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 49: Qwen2.5-14B-Q3_K_S ──────────────────────────
    benchmark_qwen_14b_q3_k_s:
      generative_entity: "${models.qwen-14b-q3_k_s}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-14B-Instruct-Q3_K_S.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]
    # ── Step 50: Qwen2.5-14B-IQ3_M ──────────────────────────
    benchmark_qwen_14b_iq3_m:
      generative_entity: "${models.qwen-14b-iq3_m}"
      prompt: |
        Build me a JSON parsable ADR file with good schema-specific specifications for an ADR. Fill out that JSON ADR with all of the information required to build a vanilla JS TODO application. Use proper JSON schema structure - do not put everything into giant strings. The response must look like JSON and be parsable as JSON. Output only the JSON object with no markdown fences.
      model_overrides:
        max_tokens: 4096
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/Qwen2.5-14B-Instruct-IQ3_M.gguf.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, duration_ms]
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/benchmark.log"
              event_fields: [step_name, error_message]