<!--
Source Session: ses_18911fba3ffeCV0aETu2P7em8Q
Message Length: 13991 characters
YAML Sections: 18
Embedded Prompts: 12
Agentic Keywords: 7
Complexity: HIGH
Source Files: test-parallel-same-model.yml, test-resource-guard.yml
-->

# ═══════════════════════════════════════════════════════════════════
# Parallel Execution Test — Same Model Concurrent Inference
# Schema-compliant per docs/schema/unified-workflow-schema.yml v2.0
#
# Tests parallel step execution via route_to with multiple targets.
# Step 1 generates a topic, then route_to triggers 3 parallel analyses
# of the SAME topic using the SAME model concurrently.
#
# Architecture:
#   step_1_generate_topic → route_to [step_2a, step_2b, step_2c]
#   step_2a, step_2b, step_2c all use same model → parallel execution
#   step_3_merge → collects all results, produces summary
#
# Expected behavior:
#   - Steps 2a/2b/2c should execute concurrently (if engine supports)
#   - OR sequentially (if engine doesn't support parallel yet)
#   - Either way, ALL targets must execute (not just first)
#   - step_3_merge gets all 3 outputs via template interpolation
# ═══════════════════════════════════════════════════════════════════
workflow_id: test_parallel_same_model
name: "Parallel Execution Test - Same Model Concurrent Inference"
description: "Test parallel step execution with route_to multiple targets using a single model"
version: "2.0.0"
author: "Whitt Execution Engine"
tags: [test, parallel, concurrent]
# ── PROVIDERS ───────────────────────────────────────────────────
providers:
  llama_cpp_with_vulkan:                               # schema line 28
    config:                                             # schema line 29
      host: localhost                                   # schema line 30
      port: 8080                                        # schema line 31
    hosting:                                            # schema line 33
      gpu_layers: 999                                   # schema line 35
# ── MODELS ──────────────────────────────────────────────────────
models:
  "target-model":
    name: "Qwen3-4B-Instruct-2507-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan                       # schema line 71
      connection_settings: {}
    ram_allocation:                                     # schema line 75
      strategy: dynamic                                 # schema line 76
    execution:                                          # schema line 96
      timeout:
        load_into_memory: 30s
        time_to_first_response: 10s
        total_time_to_response: 120s
      max_turns: 3
      stop_on_tool_failure: false
      accumulate_tool_results: false
    tools:
      default_permissions:
        web_access: false
        file_read: true
        file_write: true
        shell_exec: false
# ── WORKFLOW ────────────────────────────────────────────────────
agentic_workflow:
  inputs:
    topic: "microservices architecture"
  steps:
    # ── STEP 1: Generate analysis topic ──────────────────────
    step_1_generate_topic:
      generative_entity: "${models.target-model}"
      prompt: |
        Generate a specific software architecture topic for analysis.
        Return ONLY valid JSON with this structure:
        {
          "topic": "a specific architecture pattern or technology choice",
          "context": "2-3 sentence description of why this topic matters",
          "analysis_dimensions": ["security", "performance", "maintainability"]
        }
        Topic should be about: {{inputs.topic}}
      model_overrides:
        max_tokens: 500
        temperature: 0.7
      when:
        before_step_starts:
          log:
            to_file_path: "./docs/benchmarks/outputs/logs/parallel-test.log"
            event_fields: [step_name, model_name]
            level: info
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/parallel-topic.json"
          - route_to: [step_2a_security, step_2b_performance, step_2c_maintainability]
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/parallel-test.log"
              event_fields: [step_name, duration_ms, token_count]
    # ── STEP 2a: Security Analysis (parallel branch A) ──────
    step_2a_security:
      generative_entity: "${models.target-model}"
      prompt: |
        Analyze the following topic from a SECURITY perspective.
        Return ONLY valid JSON:
        {
          "dimension": "security",
          "topic": "{{step.step_1_generate_topic.output}}",
          "findings": ["finding 1", "finding 2", "finding 3"],
          "risk_level": "high|medium|low",
          "recommendations": ["rec 1", "rec 2"]
        }
        Be specific and actionable. 2-3 findings minimum.
      model_overrides:
        max_tokens: 1000
        temperature: 0.3
      when:
        before_step_starts:
          log:
            to_file_path: "./docs/benchmarks/outputs/logs/parallel-test.log"
            event_fields: [step_name, model_name]
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/parallel-security.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/parallel-test.log"
              event_fields: [step_name, duration_ms]
    # ── STEP 2b: Performance Analysis (parallel branch B) ──
    step_2b_performance:
      generative_entity: "${models.target-model}"
      prompt: |
        Analyze the following topic from a PERFORMANCE perspective.
        Return ONLY valid JSON:
        {
          "dimension": "performance",
          "topic": "{{step.step_1_generate_topic.output}}",
          "findings": ["finding 1", "finding 2", "finding 3"],
          "bottleneck_risk": "high|medium|low",
          "optimizations": ["opt 1", "opt 2"]
        }
        Be specific and actionable. 2-3 findings minimum.
      model_overrides:
        max_tokens: 1000
        temperature: 0.3
      when:
        before_step_starts:
          log:
            to_file_path: "./docs/benchmarks/outputs/logs/parallel-test.log"
            event_fields: [step_name, model_name]
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/parallel-performance.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/parallel-test.log"
              event_fields: [step_name, duration_ms]
    # ── STEP 2c: Maintainability Analysis (parallel branch C) ──
    step_2c_maintainability:
      generative_entity: "${models.target-model}"
      prompt: |
        Analyze the following topic from a MAINTAINABILITY perspective.
        Return ONLY valid JSON:
        {
          "dimension": "maintainability",
          "topic": "{{step.step_1_generate_topic.output}}",
          "findings": ["finding 1", "finding 2", "finding 3"],
          "complexity_level": "high|medium|low",
          "best_practices": ["practice 1", "practice 2"]
        }
        Be specific and actionable. 2-3 findings minimum.
      model_overrides:
        max_tokens: 1000
        temperature: 0.3
      when:
        before_step_starts:
          log:
            to_file_path: "./docs/benchmarks/outputs/logs/parallel-test.log"
            event_fields: [step_name, model_name]
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/parallel-maintainability.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/parallel-test.log"
              event_fields: [step_name, duration_ms]
    # ── STEP 3: Merge all analyses ──────────────────────────
    step_3_merge:
      generative_entity: "${models.target-model}"
      requires:
        - step_2a_security
        - step_2b_performance
        - step_2c_maintainability
      prompt: |
        You are merging three separate analyses into a comprehensive report.
        Security Analysis:
        {{step.step_2a_security.output}}
        Performance Analysis:
        {{step.step_2b_performance.output}}
        Maintainability Analysis:
        {{step.step_2c_maintainability.output}}
        Produce a unified JSON report:
        {
          "summary": {
            "topic": "the topic",
            "dimensions_analyzed": 3,
            "overall_assessment": "brief summary"
          },
          "security": { "findings_count": N, "risk_level": "..." },
          "performance": { "findings_count": N, "bottleneck_risk": "..." },
          "maintainability": { "findings_count": N, "complexity_level": "..." },
          "top_recommendations": ["rec 1", "rec 2", "rec 3"],
          "parallel_execution_verified": true
        }
      model_overrides:
        max_tokens: 2000
        temperature: 0.2
      when:
        before_step_starts:
          log:
            to_file_path: "./docs/benchmarks/outputs/logs/parallel-test.log"
            event_fields: [step_name, model_name]
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/parallel-merged-report.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/parallel-test.log"
              event_fields: [step_name, duration_ms, token_count]
              level: info

# ═══════════════════════════════════════════════════════════════════
# Resource Guard Test — Overload Protection
# Schema-compliant per docs/schema/unified-workflow-schema.yml v2.0
#
# Tests that the engine handles overload gracefully.
# Route_to triggers 5 simultaneous analyses. With semaphore=2,
# only 2 should execute concurrently, the rest queue.
#
# Expected behavior:
#   - ALL 5 targets must complete (no silent drops)
#   - Engine should NOT crash under load
#   - Steps should complete in ~3 sequential batches of 2+2+1
#   - step_6_report confirms all 5 analyses completed
# ═══════════════════════════════════════════════════════════════════
workflow_id: test_resource_guard_overload
name: "Resource Guard Test - Overload Protection"
description: "Test that engine handles 5 parallel targets gracefully with resource limits"
version: "2.0.0"
author: "Whitt Execution Engine"
tags: [test, parallel, resource-guard, negative]
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
    hosting:
      gpu_layers: 999
models:
  "target-model":
    name: "Qwen3-4B-Instruct-2507-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
      connection_settings: {}
    ram_allocation:
      strategy: dynamic
    execution:
      timeout:
        load_into_memory: 30s
        time_to_first_response: 10s
        total_time_to_response: 120s
      max_turns: 3
      stop_on_tool_failure: false
      accumulate_tool_results: false
    tools:
      default_permissions:
        web_access: false
        file_read: true
        file_write: true
        shell_exec: false
agentic_workflow:
  inputs:
    topic: "data pipeline architecture"
  steps:
    step_1_topic:
      generative_entity: "${models.target-model}"
      prompt: |
        Generate a 2-sentence software architecture topic description.
        Return JSON: {"topic": "...", "context": "..."}
        Topic: {{inputs.topic}}
      model_overrides:
        max_tokens: 200
        temperature: 0.5
      when:
        after_step_succeeds:
          - route_to: [step_2_aspect_1, step_2_aspect_2, step_2_aspect_3, step_2_aspect_4, step_2_aspect_5]
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/resource-guard-test.log"
              event_fields: [step_name, duration_ms]
    step_2_aspect_1:
      generative_entity: "${models.target-model}"
      prompt: |
        Analyze this topic from a SCALABILITY angle in exactly 1 sentence.
        Return JSON: {"aspect": "scalability", "finding": "...", "severity": "low|medium|high"}
        Topic: {{step.step_1_topic.output}}
      model_overrides:
        max_tokens: 200
        temperature: 0.3
    step_2_aspect_2:
      generative_entity: "${models.target-model}"
      prompt: |
        Analyze this topic from a RELIABILITY angle in exactly 1 sentence.
        Return JSON: {"aspect": "reliability", "finding": "...", "severity": "low|medium|high"}
        Topic: {{step.step_1_topic.output}}
      model_overrides:
        max_tokens: 200
        temperature: 0.3
    step_2_aspect_3:
      generative_entity: "${models.target-model}"
      prompt: |
        Analyze this topic from a COST angle in exactly 1 sentence.
        Return JSON: {"aspect": "cost", "finding": "...", "severity": "low|medium|high"}
        Topic: {{step.step_1_topic.output}}
      model_overrides:
        max_tokens: 200
        temperature: 0.3
    step_2_aspect_4:
      generative_entity: "${models.target-model}"
      prompt: |
        Analyze this topic from a COMPLIANCE angle in exactly 1 sentence.
        Return JSON: {"aspect": "compliance", "finding": "...", "severity": "low|medium|high"}
        Topic: {{step.step_1_topic.output}}
      model_overrides:
        max_tokens: 200
        temperature: 0.3
    step_2_aspect_5:
      generative_entity: "${models.target-model}"
      prompt: |
        Analyze this topic from a OBSERVABILITY angle in exactly 1 sentence.
        Return JSON: {"aspect": "observability", "finding": "...", "severity": "low|medium|high"}
        Topic: {{step.step_1_topic.output}}
      model_overrides:
        max_tokens: 200
        temperature: 0.3
    step_3_report:
      generative_entity: "${models.target-model}"
      requires:
        - step_2_aspect_1
        - step_2_aspect_2
        - step_2_aspect_3
        - step_2_aspect_4
        - step_2_aspect_5
      prompt: |
        Merge these 5 analyses into a summary.
        Return JSON with: aspects_count, overall_risk, top_concern
        1: {{step.step_2_aspect_1.output}}
        2: {{step.step_2_aspect_2.output}}
        3: {{step.step_2_aspect_3.output}}
        4: {{step.step_2_aspect_4.output}}
        5: {{step.step_2_aspect_5.output}}
      model_overrides:
        max_tokens: 300
        temperature: 0.2
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/resource-guard-report.json"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/resource-guard-test.log"
              event_fields: [step_name, duration_ms, token_count]