// Integration test for 3-model validation loop workflow YAML
// Validates YAML structure, GWT routing, and multi-model configuration

use whitt_execution_engine::config::unified::UnifiedConfig;

const WORKFLOW_PATH: &str = "docs/benchmarks/workflows/three-model-validation-loop.yml";

#[test]
fn given_workflow_yaml_when_parsed_then_validates_successfully() {
    let config = UnifiedConfig::from_file(std::path::Path::new(WORKFLOW_PATH))
        .expect("YAML should parse and validate");

    // Verify top-level fields
    assert_eq!(config.workflow_id, Some("three_model_validation_loop".into()));
    assert_eq!(config.name, Some("3-Model Validation Loop".into()));
    assert_eq!(config.schema_version, "2.0.0");

    // Verify schema validation passes
    config.validate_schema_version().expect("schema version valid");
    config.validate_min_schema_version().expect("min schema version valid");
}

#[test]
fn given_workflow_yaml_when_models_section_parsed_then_contains_3_models() {
    let config = UnifiedConfig::from_file(std::path::Path::new(WORKFLOW_PATH))
        .expect("YAML should parse");

    // Verify 3 models defined
    assert!(config.models.models.contains_key("ministral-3b"));
    assert!(config.models.models.contains_key("qwen3-4b"));
    assert!(config.models.models.contains_key("qwen2.5-coder-3b"));

    // Verify each model has correct host type
    for model_name in ["ministral-3b", "qwen3-4b", "qwen2.5-coder-3b"] {
        let model = &config.models.models[model_name];
        assert_eq!(model.host.r#type, "llama_cpp_with_vulkan");
    }
}

#[test]
fn given_workflow_yaml_when_provider_parsed_then_uses_llama_cpp_with_vulkan() {
    let config = UnifiedConfig::from_file(std::path::Path::new(WORKFLOW_PATH))
        .expect("YAML should parse");

    // Verify provider key is llama_cpp_with_vulkan
    assert!(config.providers.providers.contains_key("llama_cpp_with_vulkan"));

    // Verify provider config
    let provider = &config.providers.providers["llama_cpp_with_vulkan"];
    if let Some(config) = &provider.config {
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 8080);
    }
}

#[test]
fn given_workflow_yaml_when_agentic_workflow_parsed_then_contains_4_steps() {
    let config = UnifiedConfig::from_file(std::path::Path::new(WORKFLOW_PATH))
        .expect("YAML should parse");

    // Get agentic_workflow as raw value
    let agentic_workflow = config.agentic_workflow
        .expect("agentic_workflow should exist");

    let workflow_map = agentic_workflow.as_object()
        .expect("agentic_workflow should be an object");

    // Verify steps section exists
    assert!(workflow_map.contains_key("steps"));

    let steps = workflow_map["steps"].as_object()
        .expect("steps should be an object");

    // Verify all 4 steps exist
    assert!(steps.contains_key("analyze_code"));
    assert!(steps.contains_key("enhance_code"));
    assert!(steps.contains_key("validate_output"));
    assert!(steps.contains_key("save_and_complete"));

    // Verify exactly 4 steps
    assert_eq!(steps.len(), 4);
}

#[test]
fn given_workflow_yaml_when_steps_parsed_then_have_correct_generative_entities() {
    let config = UnifiedConfig::from_file(std::path::Path::new(WORKFLOW_PATH))
        .expect("YAML should parse");

    let agentic_workflow = config.agentic_workflow
        .expect("agentic_workflow should exist");

    let workflow_map = agentic_workflow.as_object().unwrap();
    let steps = workflow_map["steps"].as_object().unwrap();

    // Helper to extract generative_entity from step
    let get_generative_entity = |step_name: &str| -> String {
        let step = &steps[step_name];
        step.get("generative_entity")
            .and_then(|v| v.as_str())
            .expect(&format!("Step {} should have generative_entity", step_name))
            .to_string()
    };

    // Verify each step uses correct model
    assert_eq!(get_generative_entity("analyze_code"), "${models.ministral-3b}");
    assert_eq!(get_generative_entity("enhance_code"), "${models.qwen3-4b}");
    assert_eq!(get_generative_entity("validate_output"), "${models.qwen2.5-coder-3b}");
    assert_eq!(get_generative_entity("save_and_complete"), "${models.ministral-3b}");
}

#[test]
fn given_workflow_yaml_when_steps_parsed_then_dependencies_correct() {
    let config = UnifiedConfig::from_file(std::path::Path::new(WORKFLOW_PATH))
        .expect("YAML should parse");

    let agentic_workflow = config.agentic_workflow.unwrap();
    let workflow_map = agentic_workflow.as_object().unwrap();
    let steps = workflow_map["steps"].as_object().unwrap();

    // analyze_code has no dependencies
    let analyze_code = &steps["analyze_code"];
    assert_eq!(analyze_code.get("depends_on"), None);

    // enhance_code depends on analyze_code
    let enhance_code = &steps["enhance_code"];
    let depends_on = enhance_code.get("depends_on")
        .and_then(|v| v.as_array())
        .expect("enhance_code should have depends_on array");
    assert_eq!(depends_on.len(), 1);
    assert_eq!(depends_on[0].as_str(), Some("analyze_code"));

    // validate_output depends on enhance_code
    let validate_output = &steps["validate_output"];
    let depends_on = validate_output.get("depends_on")
        .and_then(|v| v.as_array())
        .expect("validate_output should have depends_on array");
    assert_eq!(depends_on.len(), 1);
    assert_eq!(depends_on[0].as_str(), Some("enhance_code"));

    // save_and_complete has no dependencies (reached via GWT routing)
    let save_and_complete = &steps["save_and_complete"];
    assert_eq!(save_and_complete.get("depends_on"), None);
}

#[test]
fn given_workflow_yaml_when_validate_output_step_then_contains_gwt_hooks() {
    let config = UnifiedConfig::from_file(std::path::Path::new(WORKFLOW_PATH))
        .expect("YAML should parse");

    let agentic_workflow = config.agentic_workflow.unwrap();
    let workflow_map = agentic_workflow.as_object().unwrap();
    let steps = workflow_map["steps"].as_object().unwrap();
    let validate_output = &steps["validate_output"];

    // Get when section
    let when = validate_output.get("when")
        .and_then(|v| v.as_object())
        .expect("validate_output should have when section");

    // Get after_step_succeeds hooks
    let after_succeeds = when.get("after_step_succeeds")
        .and_then(|v| v.as_array())
        .expect("validate_output should have after_step_succeeds hooks");

    // Verify first action is GWT
    let gwt_action = &after_succeeds[0];
    assert!(gwt_action.as_object().unwrap().contains_key("gwt"));

    // Get GWT clauses
    let gwt_clauses = gwt_action.get("gwt")
        .and_then(|v| v.as_array())
        .expect("GWT should be an array");

    // Verify exactly 2 GWT clauses
    assert_eq!(gwt_clauses.len(), 2);
}

#[test]
fn given_workflow_yaml_when_gwt_clauses_parsed_then_routes_correctly() {
    let config = UnifiedConfig::from_file(std::path::Path::new(WORKFLOW_PATH))
        .expect("YAML should parse");

    let agentic_workflow = config.agentic_workflow.unwrap();
    let workflow_map = agentic_workflow.as_object().unwrap();
    let steps = workflow_map["steps"].as_object().unwrap();
    let validate_output = &steps["validate_output"];

    let when = validate_output.get("when").unwrap().as_object().unwrap();
    let after_succeeds = when.get("after_step_succeeds").unwrap().as_array().unwrap();
    let gwt_action = &after_succeeds[0];
    let gwt_clauses = gwt_action.get("gwt").unwrap().as_array().unwrap();

    // First clause: "quality_score >= 0.7" → route_to: save_and_complete
    let clause1 = &gwt_clauses[0].as_object().unwrap();
    assert_eq!(clause1.get("given").unwrap().as_str(), Some("quality_score >= 0.7"));
    let then1 = clause1.get("then").unwrap().as_object().unwrap();
    let route_to1 = then1.get("route_to").unwrap().as_str();
    assert_eq!(route_to1, Some("save_and_complete"));

    // Second clause: "true" → route_to: analyze_code (loop back)
    let clause2 = &gwt_clauses[1].as_object().unwrap();
    assert_eq!(clause2.get("given").unwrap().as_str(), Some("true"));
    let then2 = clause2.get("then").unwrap().as_object().unwrap();
    let route_to2 = then2.get("route_to").unwrap().as_str();
    assert_eq!(route_to2, Some("analyze_code"));
}

#[test]
fn given_workflow_yaml_when_all_steps_parsed_then_have_after_step_succeeds_hooks() {
    let config = UnifiedConfig::from_file(std::path::Path::new(WORKFLOW_PATH))
        .expect("YAML should parse");

    let agentic_workflow = config.agentic_workflow.unwrap();
    let workflow_map = agentic_workflow.as_object().unwrap();
    let steps = workflow_map["steps"].as_object().unwrap();

    let step_names = ["analyze_code", "enhance_code", "validate_output", "save_and_complete"];

    for step_name in step_names {
        let step = &steps[step_name];
        let when = step.get("when")
            .and_then(|v| v.as_object())
            .expect(&format!("Step {} should have when section", step_name));

        assert!(
            when.contains_key("after_step_succeeds"),
            "Step {} should have after_step_succeeds hooks",
            step_name
        );
    }
}

#[test]
fn given_workflow_yaml_when_execution_strategy_parsed_then_load_unload_one_at_a_time() {
    let config = UnifiedConfig::from_file(std::path::Path::new(WORKFLOW_PATH))
        .expect("YAML should parse");

    let strategy = config.workflow_execution_strategy
        .expect("workflow_execution_strategy should exist");

    // Verify load_unload is one_at_a_time
    let load_unload = strategy.get("load_unload")
        .and_then(|v| v.as_str())
        .expect("load_unload should exist");
    assert_eq!(load_unload, "one_at_a_time");

    // Verify memory.model_lifecycle.unload_unused is true
    let memory = strategy.get("memory")
        .and_then(|v| v.as_object())
        .expect("memory section should exist");

    let model_lifecycle = memory.get("model_lifecycle")
        .and_then(|v| v.as_object())
        .expect("model_lifecycle should exist");

    let unload_unused = model_lifecycle.get("unload_unused")
        .and_then(|v| v.as_bool())
        .expect("unload_unused should exist");
    assert!(unload_unused);
}

#[test]
fn given_workflow_yaml_when_step_names_parsed_then_match_expected() {
    let config = UnifiedConfig::from_file(std::path::Path::new(WORKFLOW_PATH))
        .expect("YAML should parse");

    let agentic_workflow = config.agentic_workflow.unwrap();
    let workflow_map = agentic_workflow.as_object().unwrap();
    let steps = workflow_map["steps"].as_object().unwrap();

    let mut step_names: Vec<String> = steps.keys().cloned().collect();
    step_names.sort();

    assert_eq!(step_names, vec![
        "analyze_code",
        "enhance_code",
        "save_and_complete",
        "validate_output",
    ]);
}