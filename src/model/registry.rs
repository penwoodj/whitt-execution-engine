use std::collections::HashMap;
use crate::model::schema::{ModelsConfig, ModelSpec};

/// Lifecycle states for a model instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelLifecycle {
    Loaded,
    Warming,
    Active,
    Cooling,
    Unloaded,
}

/// Registry for managing model specifications and lifecycle states.
pub struct ModelRegistry {
    models: HashMap<String, ModelSpec>,
    states: HashMap<String, ModelLifecycle>,
}

impl ModelRegistry {
    pub fn new(config: ModelsConfig) -> Self {
        let models = config.models;
        let states: HashMap<String, ModelLifecycle> = models
            .keys()
            .map(|k| (k.clone(), ModelLifecycle::Unloaded))
            .collect();

        tracing::debug!(
            count = models.len(),
            "ModelRegistry initialized with {} models",
            models.len()
        );

        Self { models, states }
    }

    pub fn get(&self, name: &str) -> Option<&ModelSpec> {
        self.models.get(name)
    }

    pub fn list_models(&self) -> Vec<(String, &ModelSpec, ModelLifecycle)> {
        self.models
            .iter()
            .filter_map(|(name, spec)| {
                self.states.get(name).map(|state| (name.clone(), spec, *state))
            })
            .collect()
    }

    pub fn set_state(&mut self, name: &str, state: ModelLifecycle) {
        if self.states.contains_key(name) {
            let old_state = self.states.get(name).copied();
            self.states.insert(name.to_string(), state);
            tracing::debug!(
                model = %name,
                old_state = ?old_state,
                new_state = ?state,
                "Model state transition"
            );
        }
    }

    pub fn get_state(&self, name: &str) -> Option<ModelLifecycle> {
        self.states.get(name).copied()
    }

    pub fn resolve_reference(&self, reference: &str) -> Option<&ModelSpec> {
        let reference = reference.trim();

        if reference.starts_with("${models.") && reference.ends_with('}') {
            let model_name = &reference[9..reference.len() - 1];
            tracing::debug!(
                reference = %reference,
                model_name = %model_name,
                "Resolving model reference"
            );
            self.get(model_name)
        } else {
            None
        }
    }

    pub fn model_names(&self) -> Vec<String> {
        self.models.keys().cloned().collect()
    }

    pub fn contains(&self, name: &str) -> bool {
        self.models.contains_key(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> ModelsConfig {
        let yaml = r#"
test-model:
  name: "Test Model"
  host:
    type: "llama_cpp_with_vulkan"
other-model:
  name: "Other Model"
  host:
    type: "ollama"
"#;
        serde_saphyr::from_str(yaml).expect("parse test config")
    }

    #[test]
    fn registry_initializes_all_models_as_unloaded() {
        let config = create_test_config();
        let registry = ModelRegistry::new(config);

        assert_eq!(registry.get_state("test-model"), Some(ModelLifecycle::Unloaded));
        assert_eq!(registry.get_state("other-model"), Some(ModelLifecycle::Unloaded));
    }

    #[test]
    fn get_returns_model_spec() {
        let config = create_test_config();
        let registry = ModelRegistry::new(config);

        let spec = registry.get("test-model");
        assert!(spec.is_some());
        assert_eq!(spec.unwrap().name, "Test Model");
    }

    #[test]
    fn get_returns_none_for_unknown_model() {
        let config = create_test_config();
        let registry = ModelRegistry::new(config);

        assert!(registry.get("unknown").is_none());
    }

    #[test]
    fn list_models_returns_all_with_state() {
        let config = create_test_config();
        let registry = ModelRegistry::new(config);

        let models = registry.list_models();
        assert_eq!(models.len(), 2);

        let model_names: Vec<_> = models.iter().map(|(name, _, _)| name.clone()).collect();
        assert!(model_names.contains(&"test-model".to_string()));
        assert!(model_names.contains(&"other-model".to_string()));
    }

    #[test]
    fn set_state_transitions_model() {
        let config = create_test_config();
        let mut registry = ModelRegistry::new(config);

        registry.set_state("test-model", ModelLifecycle::Active);
        assert_eq!(registry.get_state("test-model"), Some(ModelLifecycle::Active));
    }

    #[test]
    fn set_state_ignores_unknown_model() {
        let config = create_test_config();
        let mut registry = ModelRegistry::new(config);

        registry.set_state("unknown", ModelLifecycle::Active);
        assert_eq!(registry.get_state("unknown"), None);
    }

    #[test]
    fn resolve_reference_extracts_model_name() {
        let config = create_test_config();
        let registry = ModelRegistry::new(config);

        let spec = registry.resolve_reference("${models.test-model}");
        assert!(spec.is_some());
        assert_eq!(spec.unwrap().name, "Test Model");
    }

    #[test]
    fn resolve_reference_returns_none_for_invalid_format() {
        let config = create_test_config();
        let registry = ModelRegistry::new(config);

        assert!(registry.resolve_reference("test-model").is_none());
        assert!(registry.resolve_reference("${workflow.xxx}").is_none());
        assert!(registry.resolve_reference("${models.test-model").is_none());
    }

    #[test]
    fn model_names_returns_all_keys() {
        let config = create_test_config();
        let registry = ModelRegistry::new(config);

        let names = registry.model_names();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"test-model".to_string()));
        assert!(names.contains(&"other-model".to_string()));
    }

    #[test]
    fn contains_checks_model_presence() {
        let config = create_test_config();
        let registry = ModelRegistry::new(config);

        assert!(registry.contains("test-model"));
        assert!(!registry.contains("unknown"));
    }
}
