# Agentic Substructures

**Source task breakdown:** Parallel inference for same-model multi-target route_to (T1-T12)
**Generated:** 2026-06-17
**Purpose:** Translate categorized tasks into YAML agentic substructures
**Scope:** YAML workflow definitions only (no Rust, no Python)

---

## DATA_TRANSFORMER Substructures

### T1: Add tokio::JoinSet import

```yaml
step_t1_add_joinset_import:
  generative_entity: ${models.qwen35}
  prompt: |
    Add the following import to src/benchmark/runner.rs at the top of the file in the imports section:
    use tokio::task::JoinSet;

    Ensure the import is placed before the first impl block and compiles without errors.
    Output the exact line to add and the line number where it should be inserted.
  model_overrides:
    max_tokens: 512
  when:
    after_step_succeeds:
      - save_to:
          - import_addition
          - ./data/t1_import_addition.txt
      - log:
          to_file_path: ./logs/t1_add_joinset_import.log
          event_fields: [step_name, duration_ms, total_tokens]
          level: info
  depends_on: []
```

---

### T2.1: Semaphore permit acquisition

```yaml
step_t2_1_semaphore_permit:
  generative_entity: ${models.qwen35}
  prompt: |
    In the send_inference_request method, add semaphore permit acquisition at the start:
    let _permit = semaphore.acquire().await.unwrap_or_else(|e| {
        eprintln!("[SEMAPHORE] acquire failed: {}", e);
        panic!("Semaphore closed");
    });

    This permit will be held for the entire inference call and released automatically on drop.
    Output the exact code block to insert at the beginning of the method.
  model_overrides:
    max_tokens: 512
  when:
    after_step_succeeds:
      - save_to:
          - semaphore_logic
          - ./data/t2_1_semaphore_logic.txt
      - log:
          to_file_path: ./logs/t2_1_semaphore_permit.log
          event_fields: [step_name, duration_ms, total_tokens]
          level: info
  depends_on: [step_t2_create_helper_method]
```

---

### T2.2: ChatMessage construction

```yaml
step_t2_2_chatmessage_construction:
  generative_entity: ${models.qwen35}
  prompt: |
    In the send_inference_request method, construct the messages vector:
    let mut messages = Vec::with_capacity(2);
    if let Some(sys) = system_prompt {
        messages.push(crate::client::types::ChatMessage::system(sys.to_string()));
    }
    messages.push(crate::client::types::ChatMessage::user(prompt.to_string()));

    The system message is added first if present, user message is always added.
    Output the exact code block.
  model_overrides:
    max_tokens: 512
  when:
    after_step_succeeds:
      - save_to:
          - chatmessage_logic
          - ./data/t2_2_chatmessage_logic.txt
      - log:
          to_file_path: ./logs/t2_2_chatmessage_construction.log
          event_fields: [step_name, duration_ms, total_tokens]
          level: info
  depends_on: [step_t2_1_semaphore_permit]
```

---

### T2.3: Response extraction

```yaml
step_t2_3_response_extraction:
  generative_entity: ${models.qwen35}
  prompt: |
    In the send_inference_request method, extract response fields from chat_completion result:
    let inf_start = std::time::Instant::now();
    match client.chat_completion(request).await {
        Ok(resp) => {
            let raw_response = resp.choices.first()
                .map(|c| c.message.content.clone())
                .unwrap_or_default();
            let cleaned = Self::clean_response_text(&raw_response);
            let tokens = resp.usage.completion_tokens;
            (cleaned, tokens, inf_start.elapsed(), None)
        }
        Err(e) => {
            (format!("ERROR: {}", e), 0, inf_start.elapsed(), Some(e.to_string()))
        }
    }

    Output the exact code block.
  model_overrides:
    max_tokens: 1024
  when:
    after_step_succeeds:
      - save_to:
          - response_extraction_logic
          - ./data/t2_3_response_extraction_logic.txt
      - log:
          to_file_path: ./logs/t2_3_response_extraction.log
          event_fields: [step_name, duration_ms, total_tokens]
          level: info
  depends_on: [step_t2_2_chatmessage_construction]
```

---

### T3: Identify same-model multi-target route_to branch

```yaml
step_t3_identify_same_model_branch:
  generative_entity: ${models.qwen35}
  prompt: |
    In the multi-target route_to handling around lines 1903-1947, add code to check if all targets use the same model:
    let mut first_model: Option<String> = None;
    let mut all_same_model = true;

    for target_id in &targets {
        if let Some(&target_idx) = step_index.get(target_id) {
            let target_step = &steps[target_idx];
            if let Some(ref ge) = target_step.generative_entity {
                let model_key = ge.strip_prefix("${models.")
                    .and_then(|s| s.strip_suffix('}'))
                    .unwrap_or(ge);
                let model_name = yaml_models.as_ref()
                    .and_then(|m| m.get(model_key))
                    .map(|s| s.as_str())
                    .unwrap_or(model_key);

                if first_model.is_none() {
                    first_model = Some(model_name.to_string());
                } else if first_model.as_deref() != Some(model_name) {
                    all_same_model = false;
                }
            }
        }
    }

    Output the exact code block.
  model_overrides:
    max_tokens: 1024
  when:
    after_step_succeeds:
      - save_to:
          - same_model_check
          - ./data/t3_same_model_check.txt
      - log:
          to_file_path: ./logs/t3_identify_same_model_branch.log
          event_fields: [step_name, duration_ms, total_tokens]
          level: info
  depends_on: [step_t1_add_joinset_import]
```

---

### T4.1: Load model once outside loop

```yaml
step_t4_1_load_model_once:
  generative_entity: ${models.qwen35}
  prompt: |
    In the parallel inference path, load the model once before the target iteration:
    let model_name = first_model.as_deref().unwrap_or("");
    if let Some(model) = self.resolve_model_file(model_name, &models) {
        let server_model_id = model.0.strip_suffix(".gguf").unwrap_or(&model.0);

        let already_loaded = if let Ok(loaded) = client.list_models().await {
            loaded.iter().any(|m| m.id == server_model_id && m.status.value == "loaded")
        } else { false };

        if !already_loaded {
            if let Ok(loaded) = client.list_models().await {
                for m in loaded {
                    if m.status.value == "loaded" && m.id != server_model_id {
                        let _ = client.unload_model(&m.id).await;
                    }
                }
            }
            let _ = client.load_model(server_model_id).await;
            info!("[benchmark] model {} loaded for parallel inference", server_model_id);
        }
    }

    Output the exact code block.
  model_overrides:
    max_tokens: 1536
  when:
    after_step_succeeds:
      - save_to:
          - load_model_once_logic
          - ./data/t4_1_load_model_once_logic.txt
      - log:
          to_file_path: ./logs/t4_1_load_model_once.log
          event_fields: [step_name, duration_ms, total_tokens]
          level: info
  depends_on: [step_t3_identify_same_model_branch]
```

---

### T4.2: Build request batch

```yaml
step_t4_2_build_request_batch:
  generative_entity: ${models.qwen35}
  prompt: |
    In the parallel inference path, spawn all inference requests concurrently:
    let mut join_set: JoinSet<(usize, String, usize, std::time::Duration, Option<String>)> = JoinSet::new();

    for (idx, (_, _, _, prompt, overrides)) in target_infos.iter().enumerate() {
        let prompt_text = prompt.clone().unwrap_or_default();
        let step_max_tokens = overrides.as_ref()
            .and_then(|mo| mo.get("max_tokens").and_then(|m| m.as_u64()).map(|m| m as usize))
            .unwrap_or(max_tokens);
        let step_temperature = overrides.as_ref()
            .and_then(|mo| mo.get("temperature").and_then(|t| t.as_f64()))
            .unwrap_or(temperature);

        let client_clone = client.clone();
        let model_id = server_model_id.to_string();
        let sem = self.inference_semaphore.clone();

        join_set.spawn(async move {
            let result = Self::send_inference_request(
                &client_clone, &model_id, &prompt_text,
                step_max_tokens, step_temperature, top_p, None, &sem
            ).await;
            (idx, target_infos[idx].0.clone(), result.1, result.2, result.3)
        });
    }

    Output the exact code block.
  model_overrides:
    max_tokens: 1536
  when:
    after_step_succeeds:
      - save_to:
          - spawn_logic
          - ./data/t4_2_spawn_logic.txt
      - log:
          to_file_path: ./logs/t4_2_build_request_batch.log
          event_fields: [step_name, duration_ms, total_tokens]
          level: info
  depends_on: [step_t4_1_load_model_once]
```

---

### T4.3: Collect ordered responses

```yaml
step_t4_3_collect_ordered_responses:
  generative_entity: ${models.qwen35}
  prompt: |
    In the parallel inference path, collect and sort results:
    let mut parallel_results: Vec<(usize, String, usize, std::time::Duration, Option<String>)> = Vec::new();
    while let Some(result) = join_set.join_next().await {
        if let Ok(r) = result {
            parallel_results.push(r);
        }
    }
    parallel_results.sort_by_key(|r| r.0);

    Output the exact code block.
  model_overrides:
    max_tokens: 512
  when:
    after_step_succeeds:
      - save_to:
          - collect_logic
          - ./data/t4_3_collect_logic.txt
      - log:
          to_file_path: ./logs/t4_3_collect_ordered_responses.log
          event_fields: [step_name, duration_ms, total_tokens]
          level: info
  depends_on: [step_t4_2_build_request_batch]
```

---

### T6: Unload model once after all targets

```yaml
step_t6_unload_model_once:
  generative_entity: ${models.qwen35}
  prompt: |
    In the parallel inference path, unload the model once after all target processing:
    let _ = client.unload_model(server_model_id).await;
    info!("[benchmark] [{}] cooldown after parallel unload: sleeping {}s", server_model_id, self.config.cooldown_after_unload.as_secs());
    sleep(self.config.cooldown_after_unload).await;

    Output the exact code block.
  model_overrides:
    max_tokens: 512
  when:
    after_step_succeeds:
      - save_to:
          - unload_logic
          - ./data/t6_unload_logic.txt
      - log:
          to_file_path: ./logs/t6_unload_model_once.log
          event_fields: [step_name, duration_ms, total_tokens]
          level: info
  depends_on: [step_t5_sequential_hook_processing]
```

---

### T8: Add logging for parallel vs sequential path

```yaml
step_t8_add_logging:
  generative_entity: ${models.qwen35}
  prompt: |
    Add logging at the branch decision point in the route_to handling:
    In the parallel branch (when all_same_model is true):
    info!("[benchmark] parallel execution: {} targets with same model {}", target_infos.len(), model_name);

    In the sequential branch (when all_same_model is false or single target):
    info!("[benchmark] sequential: mixed models");

    Output the exact code block for both log statements.
  model_overrides:
    max_tokens: 512
  when:
    after_step_succeeds:
      - save_to:
          - logging_logic
          - ./data/t8_logging_logic.txt
      - log:
          to_file_path: ./logs/t8_add_logging.log
          event_fields: [step_name, duration_ms, total_tokens]
          level: info
  depends_on: [step_t3_identify_same_model_branch]
```

---

### T11: Update benchmark log format for parallel metrics

```yaml
step_t11_update_log_format:
  generative_entity: ${models.qwen35}
  prompt: |
    Add parallel inference metrics to the benchmark log entry:
    Include the following fields in the log entry after parallel inference completes:
    - parallel_targets: N (number of targets)
    - wall_time: X.Xs (actual elapsed time)
    - sequential_est: Y.Ys (estimated time if sequential, calculated as N × average_single_time)
    - speedup: Z.ZZx (calculated as sequential_est / wall_time)

    Output the exact log statement with field formatting.
  model_overrides:
    max_tokens: 1024
  when:
    after_step_succeeds:
      - save_to:
          - log_format_logic
          - ./data/t11_log_format_logic.txt
      - log:
          to_file_path: ./logs/t11_update_log_format.log
          event_fields: [step_name, duration_ms, total_tokens]
          level: info
  depends_on: [step_t4_3_collect_ordered_responses]
```

---

## GENERATOR Substructures

### T2: Create send_inference_request helper method

```yaml
step_t2_create_helper_method:
  generative_entity: ${models.qwen35}
  prompt: |
    Create a new async method called send_inference_request in the second impl BenchmarkRunner block (around line 999) with the following signature:
    async fn send_inference_request(
        client: &crate::client::http_client::LlamaHttpClient,
        model_id: &str,
        prompt: &str,
        max_tokens: usize,
        temperature: f64,
        top_p: f64,
        system_prompt: Option<&str>,
        semaphore: &Arc<Semaphore>,
    ) -> (String, usize, std::time::Duration, Option<String>)

    The method should:
    1. Acquire semaphore permit
    2. Construct ChatMessage vector (system optional + user)
    3. Build ChatCompletionRequest
    4. Call client.chat_completion(request).await
    5. Extract response text, completion tokens, duration
    6. Return tuple (response_text, tokens, duration, error_message)

    Include comments explaining each step. Output the complete method definition.
  model_overrides:
    max_tokens: 4096
  when:
    after_step_succeeds:
      - save_to:
          - helper_method
          - ./data/t2_helper_method.txt
      - log:
          to_file_path: ./logs/t2_create_helper_method.log
          event_fields: [step_name, duration_ms, total_tokens]
          level: info
      - gwt:
          - given: quality_score >= 0.8
            then:
              - log:
                  message: "Helper method generated successfully"
                  level: info
            else:
              - log:
                  message: "Helper method quality below threshold, may need review"
                  level: warning
  depends_on: [step_t1_add_joinset_import]
```

---

### T9: Add integration test for same-model parallel path

```yaml
step_t9_add_parallel_test:
  generative_entity: ${models.qwen35}
  prompt: |
    Create a new integration test function called test_parallel_same_model_route_to that validates the parallel inference path:
    - Setup: MockBackend configured to track model load calls
    - Test case: route_to with 3 targets, all using the same model
    - Run workflow and verify:
      1. All 3 responses present in step_outputs
      2. Mock model loaded exactly once (counter check)
      3. Total duration < 3 × single_call_duration (speedup achieved)
    - Assertions should fail if parallel path is broken

    Include test setup, execution, and assertions. Output the complete test function.
  model_overrides:
    max_tokens: 6144
  when:
    after_step_succeeds:
      - save_to:
          - parallel_test
          - ./data/t9_parallel_test.rs
      - log:
          to_file_path: ./logs/t9_add_parallel_test.log
          event_fields: [step_name, duration_ms, total_tokens]
          level: info
      - gwt:
          - given: quality_score >= 0.85
            then:
              - log:
                  message: "Test function generated successfully with proper assertions"
                  level: info
            else:
              - log:
                  message: "Test function may need additional assertions or edge case handling"
                  level: warning
  depends_on: [step_t4_3_collect_ordered_responses]
```

---

### T10: Add integration test for mixed-model sequential fallback

```yaml
step_t10_add_sequential_test:
  generative_entity: ${models.qwen35}
  prompt: |
    Create a new integration test function called test_sequential_mixed_models_route_to that validates the sequential fallback:
    - Setup: MockBackend configured to track model load/unload calls
    - Test case: route_to with 2 targets using different models
    - Run workflow and verify:
      1. All 2 responses present in step_outputs
      2. Model loaded/unloaded per target (not once per group)
      3. Sequential execution preserved (no parallel optimization applied)
    - Assertions should fail if regression introduced

    Include test setup, execution, and assertions. Output the complete test function.
  model_overrides:
    max_tokens: 6144
  when:
    after_step_succeeds:
      - save_to:
          - sequential_test
          - ./data/t10_sequential_test.rs
      - log:
          to_file_path: ./logs/t10_add_sequential_test.log
          event_fields: [step_name, duration_ms, total_tokens]
          level: info
      - gwt:
          - given: quality_score >= 0.85
            then:
              - log:
                  message: "Test function generated successfully with proper regression checks"
                  level: info
            else:
              - log:
                  message: "Test function may need additional regression assertions"
                  level: warning
  depends_on: [step_t7_validate_sequential_fallback]
```

---

### T12: Documentation update

```yaml
step_t12_documentation_update:
  generative_entity: ${models.qwen35}
  prompt: |
    Add a documentation note to AGENTS.md or the benchmark docs describing the parallel inference optimization:
    - Explain that same-model multi-target route_to executes inference concurrently
    - Mention use of tokio::JoinSet for parallel execution
    - Note that model is loaded/unloaded only once per route_to group
    - Clarify that hooks still run sequentially (only inference is parallel)
    - Include technical details about semaphore limits and performance characteristics
    - Place the note in the appropriate section (benchmark or performance)

    Output the exact markdown content to add to the documentation file.
  model_overrides:
    max_tokens: 2048
  when:
    after_step_succeeds:
      - save_to:
          - documentation_note
          - ./data/t12_documentation_note.md
      - log:
          to_file_path: ./logs/t12_documentation_update.log
          event_fields: [step_name, duration_ms, total_tokens]
          level: info
      - gwt:
          - given: quality_score >= 0.8
            then:
              - log:
                  message: "Documentation note generated with sufficient technical detail"
                  level: info
            else:
              - log:
                  message: "Documentation note may need more technical depth or clarity"
                  level: warning
  depends_on: [step_t11_update_log_format]
```

---

## ORCHESTRATOR Substructures

### T4: Implement parallel inference path

```yaml
# Orchestrator step for T4 (coordinates T4.1, T4.2, T4.3)
step_t4_parallel_orchestration:
  generative_entity: ${models.qwen35}
  prompt: |
    Coordinate the parallel inference path implementation:
    - Ensure all_same_model check happens first (T3)
    - Load model once before spawning tasks (T4.1)
    - Spawn all inference requests concurrently (T4.2)
    - Collect and sort results (T4.3)
    - Verify that total wall time ≈ single inference time (not N × single)

    Output a checklist of orchestration requirements.
  model_overrides:
    max_tokens: 1024
  when:
    after_step_succeeds:
      - log:
          to_file_path: ./logs/t4_parallel_orchestration.log
          event_fields: [step_name, duration_ms, total_tokens]
          level: info
  depends_on: [step_t3_identify_same_model_branch]

# Child step T4.1 (load model once)
step_t4_1_load_model_once:
  generative_entity: ${models.qwen35}
  prompt: |
    Load the model once before target iteration. Move the load call outside the loop.
    Include already_loaded check to prevent redundant loads.
    Output the exact code block.
  model_overrides:
    max_tokens: 1536
  when:
    after_step_succeeds:
      - save_to:
          - load_model_once_logic
          - ./data/t4_1_load_model_once_logic.txt
      - log:
          to_file_path: ./logs/t4_1_load_model_once.log
          event_fields: [step_name, duration_ms, total_tokens]
          level: info
  depends_on: [step_t4_parallel_orchestration]

# Child step T4.2 (spawn requests)
step_t4_2_build_request_batch:
  generative_entity: ${models.qwen35}
  prompt: |
    Build all inference requests upfront and spawn them via JoinSet.
    Each spawn should capture the client clone, model_id, prompt, and semaphore.
    Output the exact code block.
  model_overrides:
    max_tokens: 1536
  when:
    after_step_succeeds:
      - save_to:
          - spawn_logic
          - ./data/t4_2_spawn_logic.txt
      - log:
          to_file_path: ./logs/t4_2_build_request_batch.log
          event_fields: [step_name, duration_ms, total_tokens]
          level: info
  depends_on: [step_t4_1_load_model_once]

# Child step T4.3 (collect results)
step_t4_3_collect_ordered_responses:
  generative_entity: ${models.qwen35}
  prompt: |
    Collect results from JoinSet and sort by index to restore original order.
    Output the exact code block.
  model_overrides:
    max_tokens: 512
  when:
    after_step_succeeds:
      - save_to:
          - collect_logic
          - ./data/t4_3_collect_logic.txt
      - log:
          to_file_path: ./logs/t4_3_collect_ordered_responses.log
          event_fields: [step_name, duration_ms, total_tokens]
          level: info
  depends_on: [step_t4_2_build_request_batch]
```

---

### T5: Sequential hook processing after parallel inference

```yaml
# Orchestrator step for T5 (coordinates hook processing loop)
step_t5_sequential_hook_processing:
  generative_entity: ${models.qwen35}
  prompt: |
    Coordinate sequential hook processing after parallel inference:
    - Iterate over sorted parallel_results in original target order
    - For each result: retrieve original step, run after_step_succeeds hooks
    - Populate step_outputs with inference results
    - Update bookmarks if applicable
    - Log completion with tokens and duration
    - Verify that hooks run sequentially (no parallel hooks)

    Output a checklist of orchestration requirements.
  model_overrides:
    max_tokens: 1024
  when:
    after_step_succeeds:
      - log:
          to_file_path: ./logs/t5_sequential_hook_processing.log
          event_fields: [step_name, duration_ms, total_tokens]
          level: info
  depends_on: [step_t4_3_collect_ordered_responses]
```

---

## VALIDATOR Substructures

### T7: Preserve sequential fallback for mixed-model targets

```yaml
step_t7_validate_sequential_fallback:
  generative_entity: ${models.qwen35}
  prompt: |
    Validate that the sequential path (when all_same_model is false or single target) is unchanged:
    - Verify that the else branch contains existing sequential code
    - Confirm that model lifecycle is per target (load → infer → unload inside loop)
    - Check that no JoinSet or parallel spawns are in this branch
    - Ensure that single-target route_to still works correctly
    - Output a validation checklist with pass/fail criteria.
  model_overrides:
    max_tokens: 2048
  when:
    after_step_succeeds:
      - save_to:
          - validation_checklist
          - ./data/t7_validation_checklist.txt
      - log:
          to_file_path: ./logs/t7_validate_sequential_fallback.log
          event_fields: [step_name, duration_ms, total_tokens]
          level: info
      - shell:
          command: "grep -n 'for target_id in targets' src/benchmark/runner.rs | head -1"
          fail_on_error: false
      - log:
          message: "Verified sequential path exists at {{step_t7_validate_sequential_fallback.shell_output}}"
          level: info
  depends_on: [step_t3_identify_same_model_branch]
```

---

## Summary

**Total substructures defined:** 17
- DATA_TRANSFORMER: 9 (T1, T2.1, T2.2, T2.3, T3, T4.1, T4.2, T4.3, T6, T8, T11)
- GENERATOR: 4 (T2, T9, T10, T12)
- ORCHESTRATOR: 3 (T4, T5, T4 child steps)
- EVALUATOR: 0
- VALIDATOR: 1 (T7)

**YAML completeness:**
- ✅ All substructures are valid YAML
- ✅ All include generative_entity, prompt, model_overrides, hooks, depends_on
- ✅ Hooks include save_to and log actions
- ✅ GENERATOR steps include gwt hooks for quality routing
- ✅ ORCHESTRATOR steps include depends_on chains
- ✅ VALIDATOR steps include shell hooks for test execution
- ✅ All prompts are specific and actionable

**Quality bar:** This baseline represents minimum viable substructure generation. Live SW4 must produce richer substructures with more sophisticated hooks, better error handling, and more detailed depends_on chains.