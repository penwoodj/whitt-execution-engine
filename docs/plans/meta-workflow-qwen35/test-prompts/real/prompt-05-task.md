<!--
Source Session: ses_17afcbfa5ffeVJykHdEoN98WDQ
Part ID: prt_e8503412f002WOXLahfmzowMZf
Character Count: 16684
Extracted: 2026-06-17T07:50:39Z
-->

## TASK

Add parallel JoinSet execution to 3 sequential route_to locations in `src/benchmark/runner.rs`. Location at L1858 already has the parallel pattern — replicate it at L2074, L2153, L2241.

## EXPECTED OUTCOME

All 4 route_to multi-target locations use the same parallel execution pattern:
- Check if all targets use the same model
- If yes and len > 1: load model once, spawn JoinSet tasks via `send_inference_request()`, collect results in parallel
- If no or len == 1: fall through to sequential `execute_workflow_step()` 

After: `cargo test --all-features` passes, `cargo clippy --all-features -- -W clippy::all` clean, `cargo build --release --all-features` clean.

## REQUIRED TOOLS

Read, Edit, Bash (for cargo test/clippy/build)

## MUST DO

### Step 1: Extract a helper method

Create a new method on `BenchmarkRunner` (find it near L1004 or near other methods). The method should encapsulate the parallel execution logic. Here's the signature and logic:

```rust
/// Execute route_to targets, using parallel JoinSet when all targets share the same model.
/// Returns (updated step_outputs HashMap, last_target_idx).
async fn execute_route_to_targets(
    &self,
    targets: &[String],
    steps: &[WorkflowStep],
    step_index: &std::collections::HashMap<String, usize>,
    client: &LlamaHttpClient,
    yaml_models: &Option<serde_json::Map<String, serde_json::Value>>,
    models: &std::collections::HashMap<String, String>,
    max_tokens: usize,
    temperature: f64,
    top_p: f64,
    step_outputs: &mut std::collections::HashMap<String, String>,
) -> usize {
```

**Use `&self`, NOT `&mut self`** — this method only reads from self and mutates step_outputs via the passed-in mutable reference. This is critical because spawned tasks can't hold &mut self.

The method should:
1. If targets.len() == 1: just return the target_idx (caller handles single-target routing)
2. Log `route_to has N targets, executing all`
3. Collect target_infos: for each target, resolve model_name from generative_entity, resolve prompt via `Self::resolve_step_output_templates`, get model_overrides
4. Check if all targets use the same model (`all_same_model` check)
5. If `all_same_model && target_infos.len() > 1`:
   a. Resolve model file via `self.resolve_model_file(model_name, models)` — if None, fall through to sequential
   b. Strip .gguf suffix for server_model_id
   c. Check if model already loaded via `client.list_models()`, load if needed, unload others
   d. Create `JoinSet<(usize, String, usize, std::time::Duration, Option<String>)>`
   e. For each target_info, spawn a task calling `Self::send_inference_request` (note: this is a static-like method, not &self)
   f. Collect results from JoinSet, sort by idx
   g. Update step_outputs with output_text for each target
   h. Unload model, sleep cooldown
   i. Return last_target_idx
6. If not parallel: execute sequentially using `self.execute_workflow_step()` — BUT this needs `&mut self` which we don't have. So for the sequential fallback, DON'T call execute_workflow_step. Instead, also use `Self::send_inference_request` for each target sequentially. This avoids the &mut self issue.

Wait — there's a problem. The sequential fallback in the existing code calls `self.execute_workflow_step()` which needs `&mut self` (for hooks). And the parallel path uses `Self::send_inference_request()` which is fine with `&self`.

**Solution**: Split the method into TWO:
- `execute_route_to_targets_parallel` — only handles the parallel case (same model). Returns `Option<usize>` (Some if parallel was used, None if not applicable).
- The caller checks the return. If Some → use parallel results. If None → fall through to existing sequential code (which calls `execute_workflow_step` with &mut self).

This way:
- Parallel path: extracted method with `&self` uses `send_inference_request`
- Sequential fallback: stays inline in the caller, uses `execute_workflow_step` with `&mut self` as before

### Step 2: Implement the method

```rust
/// Try parallel execution of route_to targets when all use the same model.
/// Returns Some(last_target_idx) if parallel was executed, None if caller should use sequential.
async fn try_execute_route_to_parallel(
    &self,
    targets: &[String],
    steps: &[WorkflowStep],
    step_index: &std::collections::HashMap<String, usize>,
    client: &LlamaHttpClient,
    yaml_models: &Option<serde_json::Map<String, serde_json::Value>>,
    models: &std::collections::HashMap<String, String>,
    max_tokens: usize,
    temperature: f64,
    top_p: f64,
    step_outputs: &mut std::collections::HashMap<String, String>,
    current_index: usize,
) -> Option<usize> {
    if targets.len() <= 1 {
        return None; // single target handled by caller
    }
    
    info!("[benchmark] route_to has {} targets, executing all", targets.len());
    
    // Collect target info
    let mut target_infos: Vec<(String, usize, String, Option<String>, Option<serde_json::Value>)> = Vec::new();
    let mut first_model: Option<String> = None;
    let mut all_same_model = true;
    
    for target_id in targets {
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
                
                let resolved_prompt = target_step.prompt.as_ref()
                    .map(|p| Self::resolve_step_output_templates(p, step_outputs));
                    
                target_infos.push((
                    target_id.clone(),
                    target_idx,
                    model_name.to_string(),
                    resolved_prompt,
                    target_step.model_overrides.clone(),
                ));
            }
        } else {
            warn!("[benchmark] route_to target '{}' not found", target_id);
        }
    }
    
    // Only parallel if all same model and more than 1 target
    if !all_same_model || target_infos.len() <= 1 {
        return None; // fall through to sequential
    }
    
    let model_name = first_model.as_deref().unwrap_or("");
    info!("[benchmark] parallel execution: {} targets with same model {}", target_infos.len(), model_name);
    
    let model = self.resolve_model_file(model_name, models)?;
    let server_model_id = model.0.strip_suffix(".gguf").unwrap_or(&model.0);
    
    // Ensure model is loaded
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
    
    // Spawn parallel tasks
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
            (idx, result.0, result.1, result.2, result.3)
        });
    }
    
    // Collect results
    let mut parallel_results: Vec<(usize, String, usize, std::time::Duration, Option<String>)> = Vec::new();
    while let Some(result) = join_set.join_next().await {
        if let Ok(r) = result {
            parallel_results.push(r);
        }
    }
    parallel_results.sort_by_key(|r| r.0);
    
    let mut last_target_idx = current_index;
    for (idx, response_text, tokens, duration, error) in &parallel_results {
        let (_target_id, target_idx, _, _, _) = &target_infos[*idx];
        let step = &steps[*target_idx];
        
        let output_text = if let Some(err) = error {
            format!("ERROR: {}", err)
        } else {
            response_text.clone()
        };
        
        step_outputs.insert(step.step_id.clone(), output_text.clone());
        last_target_idx = *target_idx;
        
        info!("[benchmark] parallel target {} completed: {} tokens in {:?}", step.step_id, tokens, duration);
    }
    
    let _ = client.unload_model(server_model_id).await;
    info!("[benchmark] [{}] cooldown after parallel unload: sleeping {}s", server_model_id, self.config.cooldown_after_unload.as_secs());
    sleep(self.config.cooldown_after_unload).await;
    
    Some(last_target_idx)
}
```

**IMPORTANT**: Place this method near the existing `send_inference_request` method (around L1004-1049). Make sure `JoinSet` and `sleep` imports are available (they should already be since L1858 uses them).

### Step 3: Replace the 3 sequential locations

At each of these 3 locations, the existing code looks like:

```rust
} else {
    info!("[benchmark] route_to has {} targets, executing all", targets.len());
    let mut last_target_idx = current_index;
    for target_id in targets {
        // ... sequential execute_workflow_step ...
    }
    current_index = last_target_idx + 1;
    continue;
}
```

Replace each with:

```rust
} else {
    if let Some(last_idx) = self.try_execute_route_to_parallel(
        &targets, &steps, &step_index, &client, &yaml_models, &models,
        max_tokens, temperature, top_p, &mut step_outputs, current_index
    ).await {
        current_index = last_idx + 1;
        continue;
    }
    // Fallback: sequential execution
    let mut last_target_idx = current_index;
    for target_id in targets {
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
                if let Some(model) = self.resolve_model_file(model_name, &models) {
                    let resolved_prompt = target_step.prompt.as_ref()
                        .map(|p| Self::resolve_step_output_templates(p, &step_outputs));
                    let resolved_target = WorkflowStep {
                        step_name: target_step.step_name.clone(),
                        step_id: target_step.step_id.clone(),
                        requires: target_step.requires.clone(),
                        when: target_step.when.clone(),
                        prompt: resolved_prompt,
                        generative_entity: target_step.generative_entity.clone(),
                        model_overrides: target_step.model_overrides.clone(),
                        r#loop: target_step.r#loop.clone(),
                    };
                    let target_result = self.execute_workflow_step(&resolved_target, &client, &model, max_tokens, temperature, top_p, None).await?;
                    results.push(target_result.benchmark_result.clone());
                    if let Some(ref last) = results.last() {
                        let output_text = last.inference_results.first()
                            .map(|inf| inf.response_text.clone())
                            .unwrap_or_default();
                        step_outputs.insert(target_step.step_id.clone(), output_text);
                    }
                    last_target_idx = target_idx;
                }
            }
        } else {
            warn!("[benchmark] route_to target '{}' not found", target_id);
        }
    }
    current_index = last_target_idx + 1;
    continue;
}
```

### The 3 locations to modify:

**Location 1 (L2074)**: Inside the `step_result.route_to` handling block in the iteration loop. The `else` branch starting at line 2073.

**Location 2 (L2153)**: Inside the `after_step_fails` hook RouteTo handling. The `else` branch starting at line 2152.

**Location 3 (L2241)**: Inside the second `step_result.route_to` handling (likely in the iteration results). The `else` branch starting at line 2240.

### Step 4: Also replace Location 0 (L1858)

The existing inline JoinSet code at L1858 should ALSO be replaced with a call to `try_execute_route_to_parallel`, with the same sequential fallback. This eliminates code duplication. The current L1858 code has the parallel path but its sequential fallback (at L1978-2019) is also sequential. Replace the entire multi-target `else` block at L1857 with the same pattern.

### Step 5: Handle the `results` variable

The 3 sequential locations push to `results` (a `Vec<BenchmarkResult>`). The new parallel method doesn't do this — it only updates `step_outputs`. After the parallel path returns `Some(last_idx)`, the `results` vec doesn't get the parallel results pushed to it. This is fine for hook-driven route_to because the caller doesn't use `results` for those paths — the results are already in `step_outputs`. But verify by checking what happens with `results` after each location.

If `results` is needed, the parallel method should also return a `Vec<BenchmarkResult>` or similar. Check the context.

### Step 6: Verify compilation and tests

Run:
1. `cargo build --all-features` — must succeed
2. `cargo clippy --all-features -- -W clippy::all` — 0 warnings
3. `cargo test --all-features` — all tests pass

## MUST NOT DO

- Do NOT change the `send_inference_request` function signature
- Do NOT change the `WorkflowStep` struct
- Do NOT modify any test files
- Do NOT add new dependencies to Cargo.toml
- Do NOT remove the sequential fallback — it's needed for mixed-model route_to
- Do NOT use `&mut self` in the new method — it must be `&self` to allow JoinSet spawning
- Do NOT suppress compiler warnings with `#[allow(...)]`
- Do NOT use `unwrap()` on Results — use `?` or proper error handling
- Do NOT touch any file other than `src/benchmark/runner.rs`

## CONTEXT

- **File**: `src/benchmark/runner.rs` (4714 lines)
- **JoinSet import**: Already imported at top of file (used at L1923)
- **sleep import**: Already imported (used at L1973)
- **`send_inference_request`**: Static-like method at L1004, takes `&LlamaHttpClient, &str, &str, usize, f64, f64, Option<&str>, &Arc<Semaphore>` — no `&self` needed
- **`resolve_model_file`**: `&self` method, returns `Option<(String, String)>` — read-only, safe for `&self`
- **`inference_semaphore`**: `Arc<Semaphore>` field, safe to clone and pass to spawned tasks
- **`config.cooldown_after_unload`**: `std::time::Duration`, read-only
- **`LlamaHttpClient`**: Derives `Clone` (confirmed in prior investigation)
- **The existing `execute_parallel_steps` at L1053** is SEPARATE and SEQUENTIAL — do NOT modify it. It has different semantics (returns Vec of Results).
- **`step_outputs`**: `HashMap<String, String>` — maps step_id → output text. Used for template interpolation in subsequent steps.
- **`results`**: `Vec<WorkflowStepResult>` — collected throughout execution. Used for final benchmark report. The parallel path doesn't push to this, but the sequential fallback does.

<!-- OMO_INTERNAL_INITIATOR -->