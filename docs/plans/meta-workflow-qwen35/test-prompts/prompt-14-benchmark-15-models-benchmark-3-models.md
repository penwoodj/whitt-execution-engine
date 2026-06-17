<!--
Source Session: ses_21eda916dffexLBSamby9C941e
Message Length: 14327 characters
YAML Sections: 96
Embedded Prompts: 8
Agentic Keywords: 9
Complexity: HIGH
Source Files: benchmark-15-models.yml, benchmark-3-models.yml, benchmark-5-models.yml
-->

workflow_id: benchmark_15_models
name: "15-Model GPU/CPU Benchmark Suite"
description: "GPU vs CPU performance benchmark of 15 diverse models"
version: "1.0.0"
author: "Whitt Execution Engine"
tags: [benchmark]
min_schema_version: "2.0.0"
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
models:
  primary:
    host:
      type: llama_cpp_with_vulkan
workflow_execution_strategy:
  load_unload: one_at_a_time
  memory:
    model_lifecycle:
      load_unload_strategy: lazy
      unload_unused: true
agentic_workflow:
  benchmark_performance:
    loop:
      count:
        max_iterations: 15
        iteration_variable: current_model
    input:
      max_tokens: 4096
    prompt: |
      Write a concise technical summary of best practices for building fault-tolerant distributed systems.
      Cover these topics with 2-3 sentences each:
      1. Circuit breaker pattern
      2. Retry with exponential backoff
      3. Bulkhead isolation
      4. Timeout management
      5. Health check monitoring
      Format as a numbered list with clear headings.
    when:
      after_step_succeeds:
        - append_to:
            - "./workspace/output/benchmark_results.yaml"
            - benchmark_collection
      after_loop_iteration_fails:
        - log:
            to_file_path: "./workspace/logs/benchmark-errors.log"
            event_fields: [iteration, current_model, error_message]

workflow_id: adr_benchmark_3_models
name: "ADR Generation Benchmark - 3 Models"
description: "Generate a fully JSON parsable ADR using 3 different models"
version: "1.0.0"
author: "Whitt Execution Engine"
tags: [benchmark, adr]
min_schema_version: "2.0.0"
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
models:
  primary:
    host:
      type: llama_cpp_with_vulkan
workflow_execution_strategy:
  load_unload: one_at_a_time
  memory:
    model_lifecycle:
      load_unload_strategy: lazy
      unload_unused: true
agentic_workflow:
  generate_adr:
    loop:
      count:
        max_iterations: 3
        iteration_variable: current_model
    input:
      max_tokens: 4096
    prompt: |
      Build me an extremely extensive fully JSON parsable ADR on a full featured TODO application in vanilla JS.
      The ADR must be valid JSON with these top-level keys:
      - "title": the ADR title
      - "status": "proposed"
      - "date": ISO 8601 date
      - "context": detailed technical context
      - "decision": the architectural decision
      - "alternatives": array of alternative approaches considered
      - "consequences": object with "positive" and "negative" arrays
      - "implementation": detailed implementation plan with code examples
      - "data_model": complete data model schema
      - "api_design": API endpoint specifications
      - "testing_strategy": testing approach and examples
      - "security_considerations": security analysis
      - "performance_considerations": performance analysis
      Make the response a single valid JSON object, no markdown fences.
    when:
      after_step_succeeds:
        - append_to:
            - "./workspace/output/adr_results.yaml"
            - adr_collection
      after_loop_iteration_fails:
        - log:
            to_file_path: "./workspace/logs/benchmark-errors.log"
            event_fields: [iteration, current_model, error_message]

workflow_id: benchmark_5_models
name: "5-Model GPU/CPU Benchmark Suite"
description: "GPU vs CPU performance benchmark of 5 diverse models"
version: "1.0.0"
author: "Whitt Execution Engine"
tags: [benchmark]
min_schema_version: "2.0.0"
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
models:
  primary:
    host:
      type: llama_cpp_with_vulkan
workflow_execution_strategy:
  load_unload: one_at_a_time
  memory:
    model_lifecycle:
      load_unload_strategy: lazy
      unload_unused: true
agentic_workflow:
  benchmark_performance:
    loop:
      count:
        max_iterations: 5
        iteration_variable: current_model
    input:
      max_tokens: 4096
    prompt: |
      Write a concise technical summary of best practices for building fault-tolerant distributed systems.
      Cover these topics with 2-3 sentences each:
      1. Circuit breaker pattern
      2. Retry with exponential backoff
      3. Bulkhead isolation
      4. Timeout management
      5. Health check monitoring
      Format as a numbered list with clear headings.
    when:
      after_step_succeeds:
        - append_to:
            - "./workspace/output/benchmark_results.yaml"
            - benchmark_collection
      after_loop_iteration_fails:
        - log:
            to_file_path: "./workspace/logs/benchmark-errors.log"
            event_fields: [iteration, current_model, error_message]

workflow_id: benchmark_50_models
name: "50-Model GPU/CPU Benchmark Suite"
description: "GPU vs CPU performance benchmark of 50 diverse models"
version: "1.0.0"
author: "Whitt Execution Engine"
tags: [benchmark]
min_schema_version: "2.0.0"
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
models:
  primary:
    host:
      type: llama_cpp_with_vulkan
workflow_execution_strategy:
  load_unload: one_at_a_time
  memory:
    model_lifecycle:
      load_unload_strategy: lazy
      unload_unused: true
agentic_workflow:
  benchmark_performance:
    loop:
      count:
        max_iterations: 50
        iteration_variable: current_model
    input:
      max_tokens: 4096
    prompt: |
      Write a concise technical summary of best practices for building fault-tolerant distributed systems.
      Cover these topics with 2-3 sentences each:
      1. Circuit breaker pattern
      2. Retry with exponential backoff
      3. Bulkhead isolation
      4. Timeout management
      5. Health check monitoring
      Format as a numbered list with clear headings.
    when:
      after_step_succeeds:
        - append_to:
            - "./workspace/output/benchmark_results.yaml"
            - benchmark_collection
      after_loop_iteration_fails:
        - log:
            to_file_path: "./workspace/logs/benchmark-errors.log"
            event_fields: [iteration, current_model, error_message]

- model_id: "Qwen2.5-0.5B-Instruct-Q4_K_M.gguf"
  tokens_per_second: 23.766403319565335
  avg_latency_ms: 1220.2098740000001
  gpu_mode: "gpu"
  load_duration_ms: 3024
  total_duration_ms: 4761
  inference_count: 1
- model_id: "Qwen3-0.6B-Q8_0.gguf"
  tokens_per_second: 17.979013203841145
  avg_latency_ms: 11457.803477000001
  gpu_mode: "gpu"
  load_duration_ms: 6557
  total_duration_ms: 18524
  inference_count: 1
- model_id: "Qwen3-4B-Instruct-2507-Q4_K_M.gguf"
  tokens_per_second: 4.649154819433723
  avg_latency_ms: 5377.321464
  gpu_mode: "gpu"
  load_duration_ms: 20720
  total_duration_ms: 26609
  inference_count: 1
- model_id: "llama-3.2-1b-instruct-q8_0.gguf"
  tokens_per_second: 10.973389365967222
  avg_latency_ms: 4556.477341
  gpu_mode: "gpu"
  load_duration_ms: 9584
  total_duration_ms: 14655
  inference_count: 1
- model_id: "gemma-3-1B-it-QAT-Q4_0.gguf"
  tokens_per_second: 21.379238149352002
  avg_latency_ms: 1590.327951
  gpu_mode: "gpu"
  load_duration_ms: 6051
  total_duration_ms: 8152
  inference_count: 1

workflow_id: benchmark_15_models
name: "15-Model GPU/CPU Benchmark Suite"
description: "GPU vs CPU performance benchmark of 15 diverse models"
version: "1.0.0"
author: "Whitt Execution Engine"
tags: [benchmark]
min_schema_version: "2.0.0"
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
models:
  primary:
    host:
      type: llama_cpp_with_vulkan
workflow_execution_strategy:
  load_unload: one_at_a_time
  memory:
    model_lifecycle:
      load_unload_strategy: lazy
      unload_unused: true
agentic_workflow:
  benchmark_performance:
    loop:
      count:
        max_iterations: 15
        iteration_variable: current_model
    input:
      max_tokens: 4096
    prompt: |
      Write a concise technical summary of best practices for building fault-tolerant distributed systems.
      Cover these topics with 2-3 sentences each:
      1. Circuit breaker pattern
      2. Retry with exponential backoff
      3. Bulkhead isolation
      4. Timeout management
      5. Health check monitoring
      Format as a numbered list with clear headings.
    when:
      after_step_succeeds:
        - append_to:
            - "./workspace/output/benchmark_results.yaml"
            - benchmark_collection
      after_loop_iteration_fails:
        - log:
            to_file_path: "./workspace/logs/benchmark-errors.log"
            event_fields: [iteration, current_model, error_message]

workflow_id: adr_benchmark_3_models
name: "ADR Generation Benchmark - 3 Models"
description: "Generate a fully JSON parsable ADR using 3 different models"
version: "1.0.0"
author: "Whitt Execution Engine"
tags: [benchmark, adr]
min_schema_version: "2.0.0"
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
models:
  primary:
    host:
      type: llama_cpp_with_vulkan
workflow_execution_strategy:
  load_unload: one_at_a_time
  memory:
    model_lifecycle:
      load_unload_strategy: lazy
      unload_unused: true
agentic_workflow:
  generate_adr:
    loop:
      count:
        max_iterations: 3
        iteration_variable: current_model
    input:
      max_tokens: 4096
    prompt: |
      Build me an extremely extensive fully JSON parsable ADR on a full featured TODO application in vanilla JS.
      The ADR must be valid JSON with these top-level keys:
      - "title": the ADR title
      - "status": "proposed"
      - "date": ISO 8601 date
      - "context": detailed technical context
      - "decision": the architectural decision
      - "alternatives": array of alternative approaches considered
      - "consequences": object with "positive" and "negative" arrays
      - "implementation": detailed implementation plan with code examples
      - "data_model": complete data model schema
      - "api_design": API endpoint specifications
      - "testing_strategy": testing approach and examples
      - "security_considerations": security analysis
      - "performance_considerations": performance analysis
      Make the response a single valid JSON object, no markdown fences.
    when:
      after_step_succeeds:
        - append_to:
            - "./workspace/output/adr_results.yaml"
            - adr_collection
      after_loop_iteration_fails:
        - log:
            to_file_path: "./workspace/logs/benchmark-errors.log"
            event_fields: [iteration, current_model, error_message]

workflow_id: benchmark_5_models
name: "5-Model GPU/CPU Benchmark Suite"
description: "GPU vs CPU performance benchmark of 5 diverse models"
version: "1.0.0"
author: "Whitt Execution Engine"
tags: [benchmark]
min_schema_version: "2.0.0"
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
models:
  primary:
    host:
      type: llama_cpp_with_vulkan
workflow_execution_strategy:
  load_unload: one_at_a_time
  memory:
    model_lifecycle:
      load_unload_strategy: lazy
      unload_unused: true
agentic_workflow:
  benchmark_performance:
    loop:
      count:
        max_iterations: 5
        iteration_variable: current_model
    input:
      max_tokens: 4096
    prompt: |
      Write a concise technical summary of best practices for building fault-tolerant distributed systems.
      Cover these topics with 2-3 sentences each:
      1. Circuit breaker pattern
      2. Retry with exponential backoff
      3. Bulkhead isolation
      4. Timeout management
      5. Health check monitoring
      Format as a numbered list with clear headings.
    when:
      after_step_succeeds:
        - append_to:
            - "./workspace/output/benchmark_results.yaml"
            - benchmark_collection
      after_loop_iteration_fails:
        - log:
            to_file_path: "./workspace/logs/benchmark-errors.log"
            event_fields: [iteration, current_model, error_message]

workflow_id: benchmark_50_models
name: "50-Model GPU/CPU Benchmark Suite"
description: "GPU vs CPU performance benchmark of 50 diverse models"
version: "1.0.0"
author: "Whitt Execution Engine"
tags: [benchmark]
min_schema_version: "2.0.0"
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
models:
  primary:
    host:
      type: llama_cpp_with_vulkan
workflow_execution_strategy:
  load_unload: one_at_a_time
  memory:
    model_lifecycle:
      load_unload_strategy: lazy
      unload_unused: true
agentic_workflow:
  benchmark_performance:
    loop:
      count:
        max_iterations: 50
        iteration_variable: current_model
    input:
      max_tokens: 4096
    prompt: |
      Write a concise technical summary of best practices for building fault-tolerant distributed systems.
      Cover these topics with 2-3 sentences each:
      1. Circuit breaker pattern
      2. Retry with exponential backoff
      3. Bulkhead isolation
      4. Timeout management
      5. Health check monitoring
      Format as a numbered list with clear headings.
    when:
      after_step_succeeds:
        - append_to:
            - "./workspace/output/benchmark_results.yaml"
            - benchmark_collection
      after_loop_iteration_fails:
        - log:
            to_file_path: "./workspace/logs/benchmark-errors.log"
            event_fields: [iteration, current_model, error_message]

- model_id: "Qwen3-4B-Instruct-2507-Q4_K_M.gguf"
  tokens_per_second: 4.66931260652818
  avg_latency_ms: 438608.457514
  gpu_mode: "gpu"
  load_duration_ms: 23693
  total_duration_ms: 462818
  inference_count: 1
- model_id: "Qwen2.5-0.5B-Instruct-Q4_K_M.gguf"
  tokens_per_second: 29.72790858681906
  avg_latency_ms: 12177.109565
  gpu_mode: "gpu"
  load_duration_ms: 2525
  total_duration_ms: 15220
  inference_count: 1
- model_id: "llama-3.2-1b-instruct-q8_0.gguf"
  tokens_per_second: 10.81954332629235
  avg_latency_ms: 92887.469433
  gpu_mode: "gpu"
  load_duration_ms: 10081
  total_duration_ms: 103481
  inference_count: 1