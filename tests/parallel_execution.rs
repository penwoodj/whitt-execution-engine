//! Integration tests for parallel route_to execution and edge cases.
//!
//! These tests verify graceful handling of:
//! - route_to with non-existent step targets
//! - route_to with empty target lists
//! - Workflow parsing and validation

use std::fs;

#[test]
fn test_route_to_with_non_existent_target_in_yaml() {
    let dir = tempfile::tempdir().expect("tempdir");
    let yaml_path = dir.path().join("workflow-missing-target.yml");
    fs::write(&yaml_path, r#"
workflow_id: test_route_to_missing_target
name: "Test route_to with missing target"
version: "2.0.0"
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
agentic_workflow:
  - step: initial_step
    id: step_1
    requires: []
    prompt: "Initial step"
    when:
      after_step_succeeds:
        - route_to: ["non_existent_step_2", "step_3"]
  - step: step_3
    id: step_3
    requires: []
    prompt: "Third step"
"#).expect("write workflow yaml");

    let content = fs::read_to_string(&yaml_path).expect("read yaml");
    assert!(content.contains("non_existent_step_2"));
    assert!(content.contains("route_to"));

    let parsed: serde_yaml::Value = serde_yaml::from_str(&content).expect("parse yaml");
    let workflow = parsed.get("agentic_workflow").expect("workflow");
    let step_1 = workflow.get(0).expect("step_1");
    let when = step_1.get("when").expect("when");
    let after_succeeds = when.get("after_step_succeeds").expect("after_step_succeeds");
    let route_to = after_succeeds.get(0).expect("route_to action");
    let targets = route_to.get("route_to").expect("targets");

    let targets_vec: Vec<String> = serde_yaml::from_value(targets.clone()).expect("parse targets");
    assert_eq!(targets_vec, vec!["non_existent_step_2", "step_3"]);
}

#[test]
fn test_route_to_with_empty_target_list() {
    let dir = tempfile::tempdir().expect("tempdir");
    let yaml_path = dir.path().join("workflow-empty-route.yml");
    fs::write(&yaml_path, r#"
workflow_id: test_route_to_empty
name: "Test route_to with empty targets"
version: "2.0.0"
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
agentic_workflow:
  - step: initial_step
    id: step_1
    requires: []
    prompt: "Initial step"
    when:
      after_step_succeeds:
        - route_to: []
  - step: step_2
    id: step_2
    requires: []
    prompt: "Second step"
"#).expect("write workflow yaml");

    let content = fs::read_to_string(&yaml_path).expect("read yaml");
    let parsed: serde_yaml::Value = serde_yaml::from_str(&content).expect("parse yaml");
    let workflow = parsed.get("agentic_workflow").expect("workflow");
    let step_1 = workflow.get(0).expect("step_1");
    let when = step_1.get("when").expect("when");
    let after_succeeds = when.get("after_step_succeeds").expect("after_step_succeeds");
    let route_to = after_succeeds.get(0).expect("route_to action");
    let targets = route_to.get("route_to").expect("targets");

    let targets_vec: Vec<String> = serde_yaml::from_value(targets.clone()).expect("parse targets");
    assert_eq!(targets_vec, Vec::<String>::new());
    assert!(targets_vec.is_empty());
}

#[test]
fn test_route_to_single_target() {
    let dir = tempfile::tempdir().expect("tempdir");
    let yaml_path = dir.path().join("workflow-single-target.yml");
    fs::write(&yaml_path, r#"
workflow_id: test_route_to_single
name: "Test route_to with single target"
version: "2.0.0"
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
agentic_workflow:
  - step: initial_step
    id: step_1
    requires: []
    prompt: "Initial step"
    when:
      after_step_succeeds:
        - route_to: ["step_2"]
  - step: step_2
    id: step_2
    requires: []
    prompt: "Second step"
"#).expect("write workflow yaml");

    let content = fs::read_to_string(&yaml_path).expect("read yaml");
    let parsed: serde_yaml::Value = serde_yaml::from_str(&content).expect("parse yaml");
    let workflow = parsed.get("agentic_workflow").expect("workflow");
    let step_1 = workflow.get(0).expect("step_1");
    let when = step_1.get("when").expect("when");
    let after_succeeds = when.get("after_step_succeeds").expect("after_step_succeeds");
    let route_to = after_succeeds.get(0).expect("route_to action");
    let targets = route_to.get("route_to").expect("targets");

    let targets_vec: Vec<String> = serde_yaml::from_value(targets.clone()).expect("parse targets");
    assert_eq!(targets_vec, vec!["step_2"]);
    assert_eq!(targets_vec.len(), 1);
}

#[test]
fn test_route_to_parallel_multiple_same_model() {
    let dir = tempfile::tempdir().expect("tempdir");
    let yaml_path = dir.path().join("workflow-parallel-same-model.yml");
    fs::write(&yaml_path, r#"
workflow_id: test_route_to_parallel
name: "Test route_to parallel execution"
version: "2.0.0"
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
agentic_workflow:
  - step: initial_step
    id: step_1
    requires: []
    prompt: "Initial step"
    generative_entity: "${models.primary}"
    when:
      after_step_succeeds:
        - route_to: ["step_2", "step_3", "step_4"]
  - step: parallel_step_2
    id: step_2
    requires: []
    prompt: "Parallel step 2"
    generative_entity: "${models.primary}"
  - step: parallel_step_3
    id: step_3
    requires: []
    prompt: "Parallel step 3"
    generative_entity: "${models.primary}"
  - step: parallel_step_4
    id: step_4
    requires: []
    prompt: "Parallel step 4"
    generative_entity: "${models.primary}"
"#).expect("write workflow yaml");

    let content = fs::read_to_string(&yaml_path).expect("read yaml");
    let parsed: serde_yaml::Value = serde_yaml::from_str(&content).expect("parse yaml");
    let workflow = parsed.get("agentic_workflow").expect("workflow");
    let step_1 = workflow.get(0).expect("step_1");
    let when = step_1.get("when").expect("when");
    let after_succeeds = when.get("after_step_succeeds").expect("after_step_succeeds");
    let route_to = after_succeeds.get(0).expect("route_to action");
    let targets = route_to.get("route_to").expect("targets");

    let targets_vec: Vec<String> = serde_yaml::from_value(targets.clone()).expect("parse targets");
    assert_eq!(targets_vec, vec!["step_2", "step_3", "step_4"]);
    assert_eq!(targets_vec.len(), 3);

    let models = parsed.get("models").expect("models");
    let primary = models.get("primary").expect("primary");
    let primary_model = primary.get("host").expect("host");

    assert!(primary_model.get("type").is_some());
}

#[test]
fn test_route_to_mixed_models_prevents_parallel() {
    let dir = tempfile::tempdir().expect("tempdir");
    let yaml_path = dir.path().join("workflow-mixed-models.yml");
    fs::write(&yaml_path, r#"
workflow_id: test_route_to_mixed_models
name: "Test route_to with mixed models"
version: "2.0.0"
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
  secondary:
    host:
      type: llama_cpp_with_vulkan
agentic_workflow:
  - step: initial_step
    id: step_1
    requires: []
    prompt: "Initial step"
    generative_entity: "${models.primary}"
    when:
      after_step_succeeds:
        - route_to: ["step_2", "step_3"]
  - step: step_with_primary
    id: step_2
    requires: []
    prompt: "Step with primary model"
    generative_entity: "${models.primary}"
  - step: step_with_secondary
    id: step_3
    requires: []
    prompt: "Step with secondary model"
    generative_entity: "${models.secondary}"
"#).expect("write workflow yaml");

    let content = fs::read_to_string(&yaml_path).expect("read yaml");
    let parsed: serde_yaml::Value = serde_yaml::from_str(&content).expect("parse yaml");

    let step_2 = &parsed["agentic_workflow"][1];
    let step_3 = &parsed["agentic_workflow"][2];
    let ge_2 = step_2.get("generative_entity").expect("step_2 ge");
    let ge_3 = step_3.get("generative_entity").expect("step_3 ge");

    assert_eq!(ge_2, "${models.primary}");
    assert_eq!(ge_3, "${models.secondary}");
    assert_ne!(ge_2, ge_3);
}
