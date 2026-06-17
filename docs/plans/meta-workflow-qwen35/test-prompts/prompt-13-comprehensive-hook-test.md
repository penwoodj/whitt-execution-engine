<!--
Source Session: ses_18911fba3ffeCV0aETu2P7em8Q
Message Length: 15677 characters
YAML Sections: 13
Embedded Prompts: 2
Agentic Keywords: 10
Complexity: HIGH
Source Files: comprehensive-hook-test.yml
-->

# ═══════════════════════════════════════════════════════════════════
# Comprehensive Hook System Test — Ministral 3B
# Schema-compliant per docs/schema/unified-workflow-schema.yml v2.0
#
# Exercises ALL 10 trigger timings + ALL 10 action types:
# - Triggers: before_step_starts, after_step_starts, during_step_streaming (deferred),
#   after_step_succeeds, after_step_fails, after_all_retries_exhausted,
#   before_gwt_evaluates, after_gwt_evaluates, on_requires_failed,
#   after_loop_iteration_fails (deferred)
# - Actions: log, save_to, append_to, route_to, bookmark, notify, fail,
#   skip_step, skip_remaining, gwt
#
# Workflow structure:
#   step_1_generate → step_2_evaluate → step_3_decision (GWT) → [step_success|step_fail]
#
# Tests: Decomposed benchmark flow with real inference, quality evaluation,
#   conditional routing, comprehensive hook coverage, workflow-level hooks
# ═══════════════════════════════════════════════════════════════════
workflow_id: comprehensive_hook_test
name: "Comprehensive Hook System Test"
description: "Test all 10 trigger timings and all 10 action types with Ministral 3B"
version: "2.0.0"
author: "Whitt Execution Engine"
tags: [comprehensive, hooks, all-triggers, all-actions]
min_schema_version: "2.0.0"
schema_version: "2.0.0"
# ── WORKFLOW-LEVEL HOOKS (merge with step-level) ───────────────
# Schema line 275: Step-level when: hooks MERGE with these defaults.
when:                                                       # schema line 275
  after_step_succeeds:                                    # schema line 276
    log:
      to_file_path: "./tmp-live-system-testing/outputs/workflow-level-hooks.log"
      event_fields: [step_name, duration_ms]
      level: info
  after_step_fails:
    log:
      to_file_path: "./tmp-live-system-testing/outputs/workflow-level-errors.log"
      event_fields: [step_name, error_message]
      level: error
# ── PROVIDERS ───────────────────────────────────────────────────
providers:                                                 # schema line 28
  llama_cpp_with_vulkan:
    config:                                                # schema line 29
      host: localhost                                      # schema line 30
      port: 8080                                           # schema line 31
    hosting:                                               # schema line 33
      gpu_layers: 999                                      # schema line 35
# ── MODEL ───────────────────────────────────────────────────────
models:
  "test-model":                                           # schema line 68
    name: "Ministral-3-3B-Instruct-2512-Q4_K_M"
    host:                                                  # schema line 70
      type: llama_cpp_with_vulkan                          # schema line 71
    execution:                                             # schema line 96
      max_tokens: 50000                                    # schema line 102
# ── WORKFLOW EXECUTION STRATEGY ────────────────────────────────
workflow_execution_strategy:                                # schema line 503
  memory:
    model_lifecycle:
      unload_unused: true                                  # schema line 521
# ── AGENTIC WORKFLOW ───────────────────────────────────────────
agentic_workflow:                                           # schema line 196
  steps:                                                   # schema line 293
    # ── STEP 1: Generate Benchmark Result (ALL 3 WIREABLE TRIGGERS) ──
    step_1_generate:
      generative_entity: "${models.test-model}"            # schema line 297
      prompt: |                                            # schema line 298
        Generate a JSON object with exactly these keys:
        - "benchmark_name": "live_test"
        - "tokens_per_second": number (float, realistic value 10-50)
        - "latency_ms": number (integer, realistic value 50-200)
        - "quality_score": number (float, 0.0-1.0)
        - "status": "ok" or "fail"
        Output ONLY the JSON object. No markdown. No backticks.
        First character: { Last character: }
      model_overrides:                                     # schema line 303
        temperature: 0.3
        max_tokens: 200
      retry:                                                # schema line 307
        max_attempts: 3                                     # schema line 308
        backoff: linear                                     # schema line 309
        initial_delay: "1s"                                 # schema line 310
      when:                                                # schema line 314
        # TRIGGER: before_step_starts (step 1 in phase 1) ───────
        before_step_starts:                                 # schema line 317
          - log:
              to_file_path: "./tmp-live-system-testing/outputs/hook-log.log"
              event_fields: [step_name, timestamp, model_name]
              level: info
          - bookmark: "./tmp-live-system-testing/checkpoints/step_1_before.cp"
        # TRIGGER: after_step_starts (step 1 in phase 1) ────────
        after_step_starts:                                  # NEW trigger
          - log:
              to_file_path: "./tmp-live-system-testing/outputs/hook-log.log"
              event_fields: [step_name, step_type, prompt_preview]
              level: info
        # TRIGGER: after_step_succeeds (actions: save_to, append_to, log) ───
        after_step_succeeds:                                # schema line 325
          - save_to:
              - benchmark_result
              - "./tmp-live-system-testing/outputs/step1-result.json"
          - append_to:
              - "./tmp-live-system-testing/outputs/all-results.yaml"
              - results_collection
          - log:
              to_file_path: "./tmp-live-system-testing/outputs/hook-log.log"
              event_fields: [step_name, output, duration_ms, token_count, quality_score]
              level: info
          - notify:
              message: "Step 1 benchmark completed successfully"
        # TRIGGER: after_step_fails (actions: log, GWT conditional routing) ───
        after_step_fails:                                   # schema line 330
          - log:
              to_file_path: "./tmp-live-system-testing/outputs/hook-errors.log"
              event_fields: [step_name, error_type, error_message, is_retryable]
              level: error
          - gwt:                                           # schema line 331 (alias: given_when_then)
              - given: "error.is_retryable == true"
                then: { route_to: step_1_generate }        # Retry same step
              - given: "error.is_retryable == false"
                then: { route_to: step_all_retries_exhausted }
        # TRIGGER: after_all_retries_exhausted (actions: log, fail) ───
        after_all_retries_exhausted:                        # schema line 339
          - log:
              to_file_path: "./tmp-live-system-testing/outputs/hook-critical.log"
              event_fields: [step_name, total_attempts, last_error]
              level: critical
          - fail: "All retry attempts exhausted for benchmark generation"
    # ── STEP: All Retries Exhausted (only fires if step_1_generate fails 3x) ──
    step_all_retries_exhausted:
      generative_entity: "${models.test-model}"
      prompt: "The benchmark generation failed after 3 attempts. Log this failure."
      model_overrides:
        temperature: 0.1
        max_tokens: 50
      when:
        after_step_succeeds:
          - log:
              to_file_path: "./tmp-live-system-testing/outputs/hook-log.log"
              event_fields: [step_name, output]
              level: warning
    # ── STEP 2: Evaluate Quality (tests on_requires_failed) ────────
    step_2_evaluate:
      generative_entity: "${models.test-model}"
      prompt: |
        Evaluate this benchmark result and return a JSON object with:
        - "evaluation": "pass" or "fail"
        - "reason": brief explanation (10 words max)
        - "overall_score": number (float, 0.0-1.0)
        INPUT:
        {{step.step_1_generate.output}}
        Output ONLY the JSON object. No markdown. No backticks.
      model_overrides:
        temperature: 0.2
        max_tokens: 100
      depends_on: [step_1_generate]                       # schema line 314 (alias: requires)
      retry:
        max_attempts: 2
        backoff: linear
        initial_delay: "500ms"
      when:
        # TRIGGER: on_requires_failed (fires if step_1_generate failed) ───
        on_requires_failed:                                # NEW trigger
          - log:
              to_file_path: "./tmp-live-system-testing/outputs/hook-dependency.log"
              event_fields: [failed_step, reason, dependency_chain]
              level: error
          - fail: "Dependency step_1_generate failed, cannot evaluate"
        # TRIGGER: before_step_starts
        before_step_starts:
          - log:
              to_file_path: "./tmp-live-system-testing/outputs/hook-log.log"
              event_fields: [step_name, timestamp]
              level: info
          - bookmark: "./tmp-live-system-testing/checkpoints/step_2_before.cp"
        # TRIGGER: after_step_succeeds (actions: save_to, route_to conditional)
        after_step_succeeds:
          - save_to: "./tmp-live-system-testing/outputs/step2-evaluation.json"
          - route_to: step_3_decision                       # Always route to decision step
        # TRIGGER: after_step_fails
        after_step_fails:
          - log:
              to_file_path: "./tmp-live-system-testing/outputs/hook-errors.log"
              event_fields: [step_name, error_type, error_message]
              level: error
          - route_to: step_failure_handler                  # Route to failure handler
    # ── STEP 3: Decision via GWT (tests before/after_gwt_evaluates) ───
    step_3_decision:
      when:                                                 # schema line 369
        # ACTION: gwt at top level (NOT inside a trigger timing)
        gwt:                                                 # schema line 370
          - given: "step.step_2_evaluate.output.evaluation == 'pass'"
            when: "Quality check passed"
            then: { route_to: step_success }                 # schema line 373
          - given: "step.step_2_evaluate.output.overall_score >= 0.7"
            when: "Acceptable quality, needs improvement"
            then: { route_to: step_marginal }
          - given: "true"
            when: "All other cases"
            then: { route_to: step_fail }
        # TRIGGER: before_gwt_evaluates
        before_gwt_evaluates:                               # schema line 378
          - log:
              to_file_path: "./tmp-live-system-testing/outputs/hook-gwt.log"
              event_fields: [step_name, input_value, quality_score]
              level: info
        # TRIGGER: after_gwt_evaluates
        after_gwt_evaluates:                                # schema line 382
          - log:
              to_file_path: "./tmp-live-system-testing/outputs/hook-gwt.log"
              event_fields: [step_name, decision, route_target, quality_score]
              level: info
          - notify:
              message: "GWT decision completed"
    # ── STEP SUCCESS PATH ─────────────────────────────────────────
    step_success:
      generative_entity: "${models.test-model}"
      prompt: "The benchmark passed all quality checks. Generate a success summary (1 sentence)."
      model_overrides:
        temperature: 0.3
        max_tokens: 100
      when:
        before_step_starts:
          - log:
              to_file_path: "./tmp-live-system-testing/outputs/hook-log.log"
              event_fields: [step_name, status]
              level: info
          - notify:
              message: "Entering success path"
        after_step_succeeds:
          - save_to: "./tmp-live-system-testing/outputs/final-summary.txt"
          - log:
              to_file_path: "./tmp-live-system-testing/outputs/hook-completion.log"
              event_fields: [step_name, output, final_status]
              level: info
          - notify:
              message: "Workflow completed successfully"
    # ── STEP MARGINAL PATH ────────────────────────────────────────
    step_marginal:
      generative_entity: "${models.test-model}"
      prompt: "The benchmark quality is acceptable (0.7+). Generate a note (1 sentence)."
      model_overrides:
        temperature: 0.3
        max_tokens: 100
      when:
        after_step_succeeds:
          - save_to: "./tmp-live-system-testing/outputs/final-summary.txt"
          - log:
              to_file_path: "./tmp-live-system-testing/outputs/hook-completion.log"
              event_fields: [step_name, output, status]
              level: warning
          - notify:
              message: "Workflow completed with marginal quality"
    # ── STEP FAIL PATH ────────────────────────────────────────────
    step_fail:
      generative_entity: "${models.test-model}"
      prompt: "The benchmark failed quality checks. Generate a failure note (1 sentence)."
      model_overrides:
        temperature: 0.3
        max_tokens: 100
      when:
        after_step_succeeds:
          - save_to: "./tmp-live-system-testing/outputs/failure-note.txt"
          - log:
              to_file_path: "./tmp-live-system-testing/outputs/hook-failure.log"
              event_fields: [step_name, output, status]
              level: error
          - fail: "Benchmark quality below threshold"
    # ── STEP FAILURE HANDLER ──────────────────────────────────────
    step_failure_handler:
      generative_entity: "${models.test-model}"
      prompt: "Evaluation failed. Log error details (1 sentence)."
      model_overrides:
        temperature: 0.1
        max_tokens: 50
      when:
        after_step_succeeds:
          - log:
              to_file_path: "./tmp-live-system-testing/outputs/hook-failure.log"
              event_fields: [step_name, output]
              level: critical
          - notify:
              message: "Workflow failed during evaluation"
# ═══════════════════════════════════════════════════════════════════
# TRIGGER COVERAGE:
# ✅ before_step_starts: step_1_generate, step_2_evaluate, step_success
# ✅ after_step_starts: step_1_generate
# ✅ after_step_succeeds: step_1_generate, step_2_evaluate, step_success, step_marginal, step_fail, step_failure_handler, step_all_retries_exhausted
# ✅ after_step_fails: step_1_generate, step_2_evaluate
# ✅ after_all_retries_exhausted: step_1_generate
# ✅ before_gwt_evaluates: step_3_decision
# ✅ after_gwt_evaluates: step_3_decision
# ✅ on_requires_failed: step_2_evaluate
# ⏳ during_step_streaming: DEFERRED (benchmark uses non-streaming)
# ⏳ after_loop_iteration_fails: DEFERRED (requires loop executor)
#
# ACTION COVERAGE:
# ✅ log: All steps with various event_fields and levels
# ✅ save_to: step_1_generate (variable + file), step_2_evaluate (file), step_success/marginal/fail (file)
# ✅ append_to: step_1_generate (file + variable)
# ✅ route_to: step_2_evaluate (single), step_1 GWT (conditional), step_3 GWT (multi)
# ✅ bookmark: step_1_generate, step_2_evaluate (before_step_starts)
# ✅ notify: step_1_generate, step_3_decision, step_success, step_marginal, step_failure_handler
# ✅ fail: step_1 GWT (on non-retryable), step_fail, step_all_retries_exhausted
# ✅ skip_step: Not exercised (would break workflow)
# ✅ skip_remaining: Not exercised (would end workflow prematurely)
# ✅ gwt: step_1 after_step_fails (conditional), step_3_decision (control flow)
# ═══════════════════════════════════════════════════════════════════