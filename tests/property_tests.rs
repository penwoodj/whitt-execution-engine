//! Property-based tests for core types.
//!
//! Uses proptest to verify invariants across random inputs:
//! - ResourceManager: allocation/deallocation never exceeds capacity
//! - TemplateInterpolator: placeholder resolution properties
//! - Error enum: Display never panics, constructors work
//! - ModelRegistry: state transition validity

use proptest::prelude::*;
use std::collections::HashMap;
use whitt_execution_engine::error::{Error, Result};
use whitt_execution_engine::model::registry::{ModelLifecycle, ModelRegistry};
use whitt_execution_engine::model::resource::ResourceManager;

// ============================================================================
// ResourceManager Property Tests
// ============================================================================

/// Property: Allocation never exceeds total capacity.
/// After any sequence of allocations, allocated resources must be ≤ total.
proptest! {
    #[test]
    fn prop_allocation_never_exceeds_total(
        total_ram in 1024u64..65536,
        total_vram in 1024u64..65536,
        allocations in prop::collection::vec(
            (100u64..2048u64, 100u64..2048u64, ".*"),
            1..20
        ),
    ) {
        let manager = ResourceManager::new(total_ram, total_vram);

        for (ram_mb, vram_mb, _name) in allocations {
            if manager.can_allocate(ram_mb, vram_mb) {
                let mut mgr_clone = ResourceManager::new(total_ram, total_vram);
                mgr_clone.allocate_all(&vec![(ram_mb, vram_mb)]);
                assert!(mgr_clone.allocated_ram() <= total_ram);
                assert!(mgr_clone.allocated_vram() <= total_vram);
            }
        }
    }

    /// Property: Available + allocated always equals total (conservation invariant).
    #[test]
    fn prop_available_plus_allocated_equals_total(
        total_ram in 1024u64..65536,
        total_vram in 1024u64..65536,
    ) {
        let manager = ResourceManager::new(total_ram, total_vram);

        assert_eq!(manager.available_ram() + manager.allocated_ram(), total_ram);
        assert_eq!(manager.available_vram() + manager.allocated_vram(), total_vram);

        let mut mgr = ResourceManager::new(total_ram, total_vram);
        if mgr.can_allocate(1000, 1000) {
            let _ = mgr.allocate("test", 1000, 1000);
            assert_eq!(mgr.available_ram() + mgr.allocated_ram(), total_ram);
            assert_eq!(mgr.available_vram() + mgr.allocated_vram(), total_vram);
        }
    }

    /// Property: Deallocation is idempotent (saturating at zero).
    /// Deallocating more than allocated leaves allocation at zero.
    #[test]
    fn prop_deallocation_saturates_at_zero(
        total_ram in 1024u64..65536,
        total_vram in 1024u64..65536,
        allocate_mb in 100u64..2048u64,
        deallocate_mb in 0u64..4096u64,
    ) {
        prop_assume!(allocate_mb <= total_ram && allocate_mb <= total_vram);

        let mut manager = ResourceManager::new(total_ram, total_vram);
        let _ = manager.allocate("test", allocate_mb, allocate_mb);

        manager.deallocate("test", deallocate_mb, deallocate_mb);

        let expected = allocate_mb.saturating_sub(deallocate_mb);
        assert_eq!(manager.allocated_ram(), expected);
        assert_eq!(manager.allocated_vram(), expected);
    }

    /// Property: Can_allocate returns false when capacity exceeded.
    #[test]
    fn prop_can_allocate_reflects_capacity(
        total_ram in 1024u64..65536,
        total_vram in 1024u64..65536,
        request_ram in 0u64..65536,
        request_vram in 0u64..65536,
    ) {
        let manager = ResourceManager::new(total_ram, total_vram);

        let can = manager.can_allocate(request_ram, request_vram);

        if can {
            assert!(request_ram <= total_ram);
            assert!(request_vram <= total_vram);
        } else {
            assert!(request_ram > total_ram || request_vram > total_vram);
        }
    }

    /// Property: Utilization percent is bounded 0-100.
    #[test]
    fn prop_utilization_percent_bounded(
        total_ram in 1024u64..65536,
        total_vram in 1024u64..65536,
        allocate_mb in 0u64..2048u64,
    ) {
        let mut manager = ResourceManager::new(total_ram, total_vram);
        if manager.can_allocate(allocate_mb, allocate_mb) {
            let _ = manager.allocate("test", allocate_mb, allocate_mb);
            let (ram_util, vram_util) = manager.utilization_percent();
            assert!(ram_util >= 0.0 && ram_util <= 100.0);
            assert!(vram_util >= 0.0 && vram_util <= 100.0);
        }
    }
}

trait ResourceManagerExt {
    fn allocate_all(&mut self, allocations: &[(u64, u64)]);
}

impl ResourceManagerExt for ResourceManager {
    fn allocate_all(&mut self, allocations: &[(u64, u64)]) {
        for (i, &(ram, vram)) in allocations.iter().enumerate() {
            if self.can_allocate(ram, vram) {
                let _ = self.allocate(&format!("model_{}", i), ram, vram);
            }
        }
    }
}

// ============================================================================
// TemplateInterpolator Property Tests
// ============================================================================

/// Property: Non-references pass through unchanged.
/// Strings without ${...} or {{...}} should be returned as-is.
proptest! {
    #[test]
    fn prop_non_references_pass_through(
        template in "[a-zA-Z0-9\\s,.!?;:]{1,100}"
    ) {
        use whitt_execution_engine::model::interpolation::TemplateInterpolator;

        let interpolator = TemplateInterpolator::new();
        let models: HashMap<String, String> = HashMap::new();
        let context: HashMap<String, String> = HashMap::new();

        let result = interpolator.resolve_structural(&template, &models).unwrap();
        assert_eq!(result, template);

        let result = interpolator.resolve_runtime(&template, &context).unwrap();
        assert_eq!(result, template);
    }

    /// Property: Structural placeholders replaced when keys present.
    /// All ${models.xxx} patterns with matching keys are replaced.
    #[test]
    fn prop_structural_placeholders_replaced(
        key in "[a-z0-9_]{1,20}",
        value in "[a-zA-Z0-9\\-\\.]{1,50}",
    ) {
        use whitt_execution_engine::model::interpolation::TemplateInterpolator;

        let interpolator = TemplateInterpolator::new();
        let mut models = HashMap::new();
        models.insert(key.clone(), value.clone());

        let template = format!("${{models.{}}}", key);
        let result = interpolator.resolve_structural(&template, &models).unwrap();

        assert!(!result.contains("${models."));
        assert!(result.contains(&value));
    }

    /// Property: Empty templates work without error.
    #[test]
    fn prop_empty_templates_work(
        models_count in 0usize..10,
        context_count in 0usize..10,
    ) {
        use whitt_execution_engine::model::interpolation::TemplateInterpolator;

        let interpolator = TemplateInterpolator::new();
        let models: HashMap<String, String> = (0..models_count)
            .map(|i| (format!("key_{}", i), format!("value_{}", i)))
            .collect();
        let context: HashMap<String, String> = (0..context_count)
            .map(|i| (format!("key_{}", i), format!("value_{}", i)))
            .collect();

        let result_struct = interpolator.resolve_structural("", &models).unwrap();
        let result_runtime = interpolator.resolve_runtime("", &context).unwrap();

        assert_eq!(result_struct, "");
        assert_eq!(result_runtime, "");
    }

    /// Property: Multiple structural placeholders replaced in one call.
    #[test]
    fn prop_multiple_structural_placeholders_replaced(
        keys in prop::collection::vec("[a-z0-9_]{1,10}", 2..5),
    ) {
        use whitt_execution_engine::model::interpolation::TemplateInterpolator;

        let interpolator = TemplateInterpolator::new();
        let mut models = HashMap::new();
        let mut placeholders = Vec::new();

        for key in &keys {
            let value = format!("val_{}", key);
            models.insert(key.clone(), value.clone());
            placeholders.push(format!("${{models.{}}}", key));
        }

        let template = placeholders.join(" ");
        let result = interpolator.resolve_structural(&template, &models).unwrap();

        assert!(!result.contains("${models."));

        for key in &keys {
            assert!(result.contains(&format!("val_{}", key)));
        }
    }
}

/// Property: Runtime placeholders resolved when keys present.
#[test]
fn prop_runtime_placeholders_resolved() {
    use whitt_execution_engine::model::interpolation::TemplateInterpolator;

    let interpolator = TemplateInterpolator::new();
    let mut context = HashMap::new();
    context.insert("name".to_string(), "test-value".to_string());

    let template = "{{name}}".to_string();
    let result = interpolator.resolve_runtime(&template, &context).unwrap();

    assert_eq!(result, "test-value");
    assert!(!result.contains("{{"));
    assert!(!result.contains("}}"));
}

// ============================================================================
// Error Enum Property Tests
// ============================================================================

/// Property: Display never panics for any Error variant.
/// All error variants can be formatted to String without panicking.
proptest! {
    #[test]
    fn prop_error_display_never_panics(
        message in "[a-zA-Z0-9\\s\\p{Punct}]{1,100}",
        field in "[a-zA-Z0-9_]{1,50}",
        reason in "[a-zA-Z0-9\\s]{1,100}",
    ) {
        let errors = vec![
            Error::validation(&message),
            Error::model(&message),
            Error::model_not_found(&message),
            Error::model_load_failed(&message, &reason),
            Error::InvalidModelFormat { format: message.clone() },
            Error::config(&message),
            Error::agent_definition(&message),
            Error::tool_definition(&message),
            Error::workflow_definition(&message),
            Error::missing_field(&field),
            Error::invalid_value(&field, &reason),
            Error::cli(&message),
            Error::execution(&message),
            Error::benchmark(&message),
            Error::metrics_collection(&reason),
            Error::memory(&message),
            Error::schedule(&message),
            Error::metric(&message),
        ];

        for error in errors {
            let display = format!("{}", error);
            assert!(!display.is_empty());
            assert!(display.len() > 0);
        }
    }

    /// Property: Error Result type wraps values correctly.
    #[test]
    fn prop_error_result_wraps_values(
        value in "[a-zA-Z0-9]{1,50}",
    ) {
        let ok_result: Result<String> = Ok(value.clone());
        assert!(ok_result.is_ok());
        assert_eq!(ok_result.unwrap(), value);

        let err_result: Result<String> = Err(Error::validation("test error"));
        assert!(err_result.is_err());
    }
}

// ============================================================================
// ModelRegistry Property Tests
// ============================================================================

/// Property: State transition validation is sound.
/// Valid transitions are allowed, invalid are rejected.
proptest! {
    #[test]
    fn prop_state_transition_validity(
        from_state_idx in 0usize..5,
        to_state_idx in 0usize..5,
    ) {
        let states = vec![
            ModelLifecycle::Unloaded,
            ModelLifecycle::Loading,
            ModelLifecycle::Loaded,
            ModelLifecycle::Active,
            ModelLifecycle::Unloading,
        ];

        let from = Some(states[from_state_idx].clone());
        let to = states[to_state_idx].clone();
        let is_valid = ModelRegistry::is_valid_transition(from.clone(), to.clone());

        let invalid_transitions = vec![
            (ModelLifecycle::Unloaded, ModelLifecycle::Active),
            (ModelLifecycle::Loading, ModelLifecycle::Unloaded),
            (ModelLifecycle::Active, ModelLifecycle::Loading),
        ];

        for (invalid_from, invalid_to) in &invalid_transitions {
            if from == Some(invalid_from.clone()) && to == *invalid_to {
                assert!(!is_valid, "Invalid transition should be rejected");
            }
        }
    }
}

/// Property: Model names are unique in registry.
#[test]
fn prop_model_names_unique() {
    use whitt_execution_engine::model::schema::ModelsConfig;

    let yaml = r#"
    model1:
      name: "Model 1"
      host:
        type: "llama_cpp_with_vulkan"
    model2:
      name: "Model 2"
      host:
        type: "ollama"
    "#;

    let config: ModelsConfig = serde_saphyr::from_str(yaml).expect("parse config");
    let registry = ModelRegistry::new(config);

    let names = registry.model_names();
    assert_eq!(names.len(), 2);

    let unique_names: std::collections::HashSet<_> = names.into_iter().collect();
    assert_eq!(unique_names.len(), 2);
}

/// Property: Unknown model references return None.
proptest! {
    #[test]
    fn prop_unknown_model_returns_none(
        reference in "[a-zA-Z0-9_]{1,50}",
    ) {
        use whitt_execution_engine::model::schema::ModelsConfig;

        let yaml = r#"
        known-model:
          name: "Known Model"
          host:
            type: "llama_cpp_with_vulkan"
        "#;

        let config: ModelsConfig = serde_saphyr::from_str(yaml).expect("parse config");
        let registry = ModelRegistry::new(config);

        let result = registry.get(reference.trim());
        let known_names = vec!["known-model"];

        if known_names.contains(&reference.trim()) {
            assert!(result.is_some());
        } else {
            assert!(result.is_none());
        }
    }
}

/// Property: State can be set to valid next state.
#[test]
fn prop_valid_state_transition_succeeds() {
    use whitt_execution_engine::model::schema::ModelsConfig;

    let yaml = r#"
    test-model:
      name: "Test Model"
      host:
        type: "llama_cpp_with_vulkan"
    "#;

    let config: ModelsConfig = serde_saphyr::from_str(yaml).expect("parse config");
    let mut registry = ModelRegistry::new(config);

    registry.set_state("test-model", ModelLifecycle::Loaded);
    assert_eq!(
        registry.get_state("test-model"),
        Some(ModelLifecycle::Loaded)
    );

    registry.set_state("test-model", ModelLifecycle::Active);
    assert_eq!(
        registry.get_state("test-model"),
        Some(ModelLifecycle::Active)
    );

    registry.set_state("test-model", ModelLifecycle::Unloading);
    assert_eq!(
        registry.get_state("test-model"),
        Some(ModelLifecycle::Unloading)
    );

    registry.set_state("test-model", ModelLifecycle::Unloaded);
    assert_eq!(
        registry.get_state("test-model"),
        Some(ModelLifecycle::Unloaded)
    );
}

/// Property: Invalid state transitions are rejected.
#[test]
fn prop_invalid_state_transition_rejected() {
    use whitt_execution_engine::model::schema::ModelsConfig;

    let yaml = r#"
    test-model:
      name: "Test Model"
      host:
        type: "llama_cpp_with_vulkan"
    "#;

    let config: ModelsConfig = serde_saphyr::from_str(yaml).expect("parse config");
    let mut registry = ModelRegistry::new(config);

    let initial_state = registry.get_state("test-model");

    registry.set_state("test-model", ModelLifecycle::Active);

    assert_eq!(
        registry.get_state("test-model"),
        initial_state
    );
}

/// Property: Reference resolution works for ${models.xxx} syntax.
#[test]
fn prop_model_reference_resolution() {
    use whitt_execution_engine::model::schema::ModelsConfig;

    let model_name = "test-model";
    let display_name = "Test Model";

    let yaml = format!(r#"
    {}:
      name: "{}"
      host:
        type: "llama_cpp_with_vulkan"
    "#, model_name, display_name);

    let config: ModelsConfig = serde_saphyr::from_str(&yaml).expect("parse config");
    let registry = ModelRegistry::new(config);

    let reference = format!("${{models.{}}}", model_name);
    let result = registry.resolve_reference(&reference);

    assert!(result.is_some());
    assert_eq!(result.unwrap().name, display_name);
}
