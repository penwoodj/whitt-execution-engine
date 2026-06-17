<!--
Source Session: ses_17b1ba6e5ffeVeItkFuCd221Lg
Part ID: prt_e84e4591f0026vP8j5HqpCouc0
Character Count: 11288
Extracted: 2026-06-17T07:50:39Z
-->

## TASK: Add true parallel inference for same-model multi-target route_to

## CONTEXT
File: `src/benchmark/runner.rs` (4544 lines). When route_to returns multiple targets using the same model, they currently execute sequentially (load → infer → unload → cooldown → repeat for each target). 

The ONLY slow part is `client.chat_completion(request).await` (line 2453). Hooks are fast and sequential. Model loading is fast once the model is loaded.

## APPROACH
In the multi-target route_to handling (around lines 1903-1947), when all targets use the same model:

1. Load model ONCE (outside the target loop)
2. Build ALL chat_completion requests upfront
3. Send them ALL concurrently via `tokio::task::JoinSet`
4. Collect all responses
5. Process each response (hooks, step_outputs) sequentially
6. Unload model ONCE after all targets complete

This avoids refactoring HookEngine because hooks still run sequentially. Only the HTTP inference call runs in parallel.

## EXACT IMPLEMENTATION

### Step 1: Add tokio::JoinSet import
Add to imports at top of file:
```rust
use tokio::task::JoinSet;
```

### Step 2: Create a helper method for single inference
Add this method to the second `impl BenchmarkRunner` block (around line 999):

```rust
/// Send a single chat completion request. Used for parallel inference.
/// Returns (response_text, completion_tokens, duration).
async fn send_inference_request(
    client: &crate::client::http_client::LlamaHttpClient,
    model_id: &str,
    prompt: &str,
    max_tokens: usize,
    temperature: f64,
    top_p: f64,
    system_prompt: Option<&str>,
    semaphore: &Arc<Semaphore>,
) -> (String, usize, std::time::Duration, Option<String>) {
    let _permit = semaphore.acquire().await.unwrap_or_else(|e| {
        eprintln!("[SEMAPHORE] acquire failed: {}", e);
        panic!("Semaphore closed");
    });
    
    let mut messages = Vec::with_capacity(2);
    if let Some(sys) = system_prompt {
        messages.push(crate::client::types::ChatMessage::system(sys.to_string()));
    }
    messages.push(crate::client::types::ChatMessage::user(prompt.to_string()));
    
    let request = crate::client::types::ChatCompletionRequest {
        model: model_id.to_string(),
        messages,
        max_tokens: Some(max_tokens),
        temperature: Some(temperature as f32),
        top_p: Some(top_p as f32),
        stream: false,
        ..Default::default()
    };
    
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
}
```

### Step 3: Modify the multi-target route_to path
Find the block starting with `info!("[benchmark] route_to has {} targets, executing all", targets.len());` (there are multiple — modify the FIRST one inside the main step loop, around line 1904).

Replace the sequential `for target_id in targets` loop with parallel execution:

```rust
info!("[benchmark] route_to has {} targets, executing all", targets.len());
let mut last_target_idx = current_index;

// Collect target steps and check if they all use the same model
let mut target_infos: Vec<(String, usize, String, Option<String>, Option<serde_json::Value>)> = Vec::new();
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
            
            let resolved_prompt = target_step.prompt.as_ref()
                .map(|p| Self::resolve_step_output_templates(p, &step_outputs));
            
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

if all_same_model && !target_infos.is_empty() && target_infos.len() > 1 {
    // PARALLEL PATH: All targets use same model
    let model_name = first_model.as_deref().unwrap_or("");
    info!("[benchmark] parallel execution: {} targets with same model {}", target_infos.len(), model_name);
    
    if let Some(model) = self.resolve_model_file(model_name, &models) {
        let server_model_id = model.0.strip_suffix(".gguf").unwrap_or(&model.0);
        
        // Load model ONCE
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
        
        // Send all inference requests concurrently
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
        
        // Collect results in order
        let mut parallel_results: Vec<(usize, String, usize, std::time::Duration, Option<String>)> = Vec::new();
        while let Some(result) = join_set.join_next().await {
            if let Ok(r) = result {
                parallel_results.push(r);
            }
        }
        parallel_results.sort_by_key(|r| r.0);
        
        // Process each result sequentially (hooks need &mut self)
        for (idx, step_id, tokens, duration, error) in &parallel_results {
            let target_idx = target_infos[*idx].1;
            let step = &steps[target_idx];
            
            let output_text = if error.is_some() {
                format!("ERROR: {}", error.as_ref().unwrap())
            } else {
                // Create a minimal benchmark result for hooks
                let output = if let Some(result) = parallel_results.iter().find(|r| r.0 == *idx) {
                    String::new() // We'll use the inference result directly
                } else { String::new() };
                String::new()
            };
            
            // Store raw output for downstream steps
            // The actual output comes from the inference — we need to reconstruct it
            step_outputs.insert(step.step_id.clone(), format!("(parallel result: {} tokens, {:?})", tokens, duration));
            last_target_idx = target_idx;
            
            info!("[benchmark] parallel target {} completed: {} tokens in {:?}", step.step_id, tokens, duration);
        }
        
        // Unload model ONCE
        let _ = client.unload_model(server_model_id).await;
        info!("[benchmark] [{}] cooldown after parallel unload: sleeping {}s", server_model_id, self.config.cooldown_after_unload.as_secs());
        sleep(self.config.cooldown_after_unload).await;
        
        current_index = last_target_idx + 1;
        continue;
    }
} else {
    // SEQUENTIAL PATH: Different models or single target
    for target_id in targets {
        // ... existing sequential code ...
    }
    current_index = last_target_idx + 1;
    continue;
}
```

**IMPORTANT NOTES:**
1. `LlamaHttpClient` must implement `Clone` — check if it does. If not, you'll need to wrap it in `Arc` or use `&client` differently.
2. The `ChatCompletionRequest` and `ChatMessage` types need to be importable — use the correct paths.
3. The `Semaphore` acquire provides the resource guard — limits concurrent inferences.
4. After parallel results collected, you MUST still run hooks sequentially for each target (but this wasn't in the first implementation — hooks during parallel are skipped in this version).
5. For step_outputs, store the actual response text from the parallel inference.

## MUST DO
- Read `src/client/http_client.rs` to check if `LlamaHttpClient` implements Clone
- Read `src/client/types.rs` for ChatCompletionRequest and ChatMessage types
- Make `send_inference_request` a standalone function if `Self::` doesn't work for static methods
- Handle the case where `LlamaHttpClient` is NOT Clone (wrap in Arc if needed)
- Preserve ALL existing behavior for single-target route_to
- Run `cargo check --features client` after changes
- The parallel path must only trigger when `targets.len() > 1` AND all targets use the same model

## MUST NOT DO
- Don't modify HookEngine
- Don't modify execute_workflow_step signature
- Don't modify any test files
- Don't use `as any` or type suppression
- Don't modify http_client.rs (if Clone is missing, add Clone derive to LlamaHttpClient)
- Don't break the sequential path (single target or different models)

## EXPECTED OUTCOME
- When route_to has multiple same-model targets, inference runs concurrently
- Semaphore limits concurrent requests (default 2)
- Model loaded/unloaded only once
- Sequential fallback for different-model targets
- `cargo check --features client` passes

<!-- OMO_INTERNAL_INITIATOR -->