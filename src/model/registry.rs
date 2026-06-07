use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::model::schema::{ModelsConfig, ModelSpec};

/// Lifecycle states for a model instance.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum ModelLifecycle {
    Unloaded,
    Loading,
    Loaded,
    Active,
    Unloading,
    Error(String),
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

    pub fn list_models(&self) -> Vec<(String, ModelSpec, ModelLifecycle)> {
        self.models
            .iter()
            .map(|(name, spec)| (name.clone(), spec.clone(), self.states.get(name).unwrap_or(&ModelLifecycle::Unloaded).clone()))
            .collect()
    }

    pub fn get_state(&self, name: &str) -> Option<ModelLifecycle> {
        self.states.get(name).cloned()
    }

    pub fn set_state(&mut self, name: &str, state: ModelLifecycle) {
        if self.states.contains_key(name) {
            let old_state = self.states.get(name).cloned();

            if !Self::is_valid_transition(old_state.clone(), state.clone()) {
                tracing::warn!(
                    model = %name,
                    old_state = ?old_state,
                    new_state = ?state,
                    "Invalid state transition"
                );
                return;
            }

            self.states.insert(name.to_string(), state.clone());
            tracing::debug!(
                model = %name,
                old_state = ?old_state,
                new_state = ?state,
                "Model state transition"
            );
        }
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

    pub fn load_model(&mut self, name: &str) -> Result<(), String> {
        let current_state = self
            .get_state(name)
            .ok_or_else(|| format!("Model '{}' not found", name))?;

        match current_state {
            ModelLifecycle::Unloaded | ModelLifecycle::Loading => {
                self.set_state(name, ModelLifecycle::Loading);
                Ok(())
            }
            ModelLifecycle::Error(_) => {
                self.set_state(name, ModelLifecycle::Unloading);
                Ok(())
            }
            ModelLifecycle::Loaded | ModelLifecycle::Active => {
                Err(format!("Model '{}' is already loaded", name))
            }
            ModelLifecycle::Unloading => {
                Err(format!("Model '{}' is transitioning", name))
            }
        }
    }

    pub fn unload_model(&mut self, name: &str) -> Result<(), String> {
        let current_state = self
            .get_state(name)
            .ok_or_else(|| format!("Model '{}' not found", name))?;

        match current_state {
            ModelLifecycle::Loaded | ModelLifecycle::Active | ModelLifecycle::Unloading => {
                self.set_state(name, ModelLifecycle::Unloading);
                Ok(())
            }
            ModelLifecycle::Error(_) => {
                self.set_state(name, ModelLifecycle::Unloading);
                Ok(())
            }
            ModelLifecycle::Unloaded => {
                Err(format!("Model '{}' is already unloaded", name))
            }
            ModelLifecycle::Loading => {
                Err(format!("Model '{}' is still loading", name))
            }
        }
    }

    pub fn is_valid_transition(from: Option<ModelLifecycle>, to: ModelLifecycle) -> bool {
        matches!(
            (from, to),
            (None | Some(ModelLifecycle::Unloaded), ModelLifecycle::Loading)
                | (None | Some(ModelLifecycle::Unloaded), ModelLifecycle::Loaded)
                | (Some(ModelLifecycle::Loading), ModelLifecycle::Error(_))
                | (Some(ModelLifecycle::Loaded), ModelLifecycle::Active)
                | (Some(ModelLifecycle::Loaded), ModelLifecycle::Unloading)
                | (Some(ModelLifecycle::Loaded), ModelLifecycle::Error(_))
                | (Some(ModelLifecycle::Active), ModelLifecycle::Loaded)
                | (Some(ModelLifecycle::Active), ModelLifecycle::Unloading)
                | (Some(ModelLifecycle::Unloading), ModelLifecycle::Unloaded)
                | (Some(ModelLifecycle::Unloading), ModelLifecycle::Error(_))
                | (Some(ModelLifecycle::Error(_)), ModelLifecycle::Unloaded)
                | (Some(ModelLifecycle::Error(_)), ModelLifecycle::Loading)
                | (Some(ModelLifecycle::Error(_)), ModelLifecycle::Loaded)
                | (Some(ModelLifecycle::Error(_)), ModelLifecycle::Active)
                | (Some(ModelLifecycle::Error(_)), ModelLifecycle::Unloading)
        )
    }
}

/// Thread-safe wrapper for ModelRegistry.
pub struct ThreadSafeModelRegistry {
    inner: Arc<RwLock<ModelRegistry>>,
}

impl ThreadSafeModelRegistry {
    pub fn new(config: ModelsConfig) -> Self {
        let mut default_registry = ModelRegistry {
            models: std::collections::HashMap::new(),
            states: std::collections::HashMap::new(),
        };

        for (key, spec) in config.models.iter() {
            default_registry.models.insert(key.clone(), spec.clone());
            default_registry.states.insert(key.clone(), ModelLifecycle::Unloaded);
        }

        Self {
            inner: Arc::new(RwLock::new(default_registry)),
        }
    }

    pub fn get(&self, name: &str) -> Option<ModelSpec> {
        let registry = self.inner.read().ok()?;
        registry.get(name).cloned()
    }

    pub fn list_models(&self) -> Vec<(String, ModelSpec, ModelLifecycle)> {
        let registry = self.inner.read().unwrap();
        registry
            .list_models()
            .into_iter()
            .map(|(name, spec, state)| (name.clone(), spec.clone(), state.clone()))
            .collect()
    }

    pub fn get_state(&self, name: &str) -> Option<ModelLifecycle> {
        let registry = self.inner.read().ok()?;
        registry.get_state(name)
    }

    pub fn resolve_reference(&self, reference: &str) -> Option<ModelSpec> {
        let registry = self.inner.read().ok()?;
        registry.resolve_reference(reference).cloned()
    }

    pub fn model_names(&self) -> Vec<String> {
        let registry = self.inner.read().unwrap();
        registry.model_names()
    }

    pub fn contains(&self, name: &str) -> bool {
        let registry = self.inner.read().unwrap();
        registry.contains(name)
    }

    pub fn load_model(&self, _name: &str) -> Result<(), String> {
        Err("load_model not implemented for thread-safe registry".to_string())
    }

    pub fn unload_model(&self, name: &str) -> Result<(), String> {
        self.with_inner(|registry| registry.unload_model(name))
            .ok_or_else(|| "Registry lock failed".to_string())?
    }

    pub fn set_state(&self, name: &str, state: ModelLifecycle) -> Result<(), String> {
        self.with_inner(|registry| {
            registry.set_state(name, state);
            Ok(())
        })
        .ok_or_else(|| "Registry lock failed".to_string())?
    }

    pub fn with_inner<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut ModelRegistry) -> R,
    {
        let mut registry = self.inner.write().ok()?;
        Some(f(&mut registry))
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
        assert!(models.iter().any(|(name, _, _)| name == "test-model"));
        assert!(models.iter().any(|(name, _, _)| name == "other-model"));
    }

    #[test]
    fn model_names_returns_all_keys() {
        let config = create_test_config();
        let registry = ModelRegistry::new(config);

        let names = registry.model_names();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&String::from("test-model")));
        assert!(names.contains(&String::from("other-model")));
    }

    #[test]
    fn contains_returns_true_for_known_model() {
        let config = create_test_config();
        let registry = ModelRegistry::new(config);

        assert!(registry.contains("test-model"));
        assert!(!registry.contains("unknown"));
    }

    #[test]
    fn set_state_changes_state() {
        let config = create_test_config();
        let mut registry = ModelRegistry::new(config);

        registry.set_state("test-model", ModelLifecycle::Loaded);
        assert_eq!(registry.get_state("test-model"), Some(ModelLifecycle::Loaded));
    }

    #[test]
    fn set_state_logs_warning_for_invalid_transition() {
        let config = create_test_config();
        let mut registry = ModelRegistry::new(config);

        registry.set_state("test-model", ModelLifecycle::Unloaded);
        assert_eq!(registry.get_state("test-model"), Some(ModelLifecycle::Unloaded));

        registry.set_state("test-model", ModelLifecycle::Active);
        assert_eq!(registry.get_state("test-model"), Some(ModelLifecycle::Unloaded));
    }

    #[test]
    fn load_model_transitions_from_unloaded() {
        let config = create_test_config();
        let mut registry = ModelRegistry::new(config);

        let result = registry.load_model("test-model");
        assert!(result.is_ok());
        assert_eq!(registry.get_state("test-model"), Some(ModelLifecycle::Loading));
    }

    #[test]
    fn load_model_fails_for_already_loaded() {
        let config = create_test_config();
        let mut registry = ModelRegistry::new(config);

        registry.set_state("test-model", ModelLifecycle::Loaded);
        let result = registry.load_model("test-model");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already loaded"));
    }

    #[test]
    fn load_model_transitions_from_error() {
        let config = create_test_config();
        let mut registry = ModelRegistry::new(config);

        registry.set_state("test-model", ModelLifecycle::Error("test error".to_string()));
        let result = registry.load_model("test-model");
        assert!(result.is_ok());
        assert_eq!(registry.get_state("test-model"), Some(ModelLifecycle::Loading));
    }

    #[test]
    fn unload_model_transitions_from_loaded() {
        let config = create_test_config();
        let mut registry = ModelRegistry::new(config);

        registry.set_state("test-model", ModelLifecycle::Loaded);
        let result = registry.unload_model("test-model");
        assert!(result.is_ok());
        assert_eq!(registry.get_state("test-model"), Some(ModelLifecycle::Unloading));
    }

    #[test]
    fn unload_model_fails_for_already_unloaded() {
        let config = create_test_config();
        let mut registry = ModelRegistry::new(config);

        let result = registry.unload_model("test-model");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already unloaded"));
    }

    #[test]
    fn unload_model_transitions_from_active() {
        let config = create_test_config();
        let mut registry = ModelRegistry::new(config);

        registry.set_state("test-model", ModelLifecycle::Loaded);
        registry.set_state("test-model", ModelLifecycle::Active);
        let result = registry.unload_model("test-model");
        assert!(result.is_ok());
        assert_eq!(registry.get_state("test-model"), Some(ModelLifecycle::Unloading));
    }

    #[test]
    fn resolve_reference_works_for_valid_reference() {
        let config = create_test_config();
        let registry = ModelRegistry::new(config);

        let spec = registry.resolve_reference("${models.test-model}");
        assert!(spec.is_some());
        assert_eq!(spec.unwrap().name, "Test Model");
    }

    #[test]
    fn resolve_reference_returns_none_for_invalid_syntax() {
        let config = create_test_config();
        let registry = ModelRegistry::new(config);

        assert!(registry.resolve_reference("${workflow.xxx}").is_none());
        assert!(registry.resolve_reference("test-model").is_none());
        assert!(registry.resolve_reference("${models.unknown}").is_none());
    }

    #[test]
    fn state_transition_validation_works() {
        // Valid transitions
        assert!(ModelRegistry::is_valid_transition(None, ModelLifecycle::Loading));
        assert!(ModelRegistry::is_valid_transition(None, ModelLifecycle::Loaded));
        assert!(ModelRegistry::is_valid_transition(Some(ModelLifecycle::Loading), ModelLifecycle::Error("test".to_string())));

        assert!(ModelRegistry::is_valid_transition(Some(ModelLifecycle::Loaded), ModelLifecycle::Active));
        assert!(ModelRegistry::is_valid_transition(Some(ModelLifecycle::Loaded), ModelLifecycle::Unloading));
        assert!(ModelRegistry::is_valid_transition(Some(ModelLifecycle::Loaded), ModelLifecycle::Error("test".to_string())));

        assert!(ModelRegistry::is_valid_transition(Some(ModelLifecycle::Active), ModelLifecycle::Loaded));
        assert!(ModelRegistry::is_valid_transition(Some(ModelLifecycle::Active), ModelLifecycle::Unloading));

        assert!(ModelRegistry::is_valid_transition(Some(ModelLifecycle::Unloading), ModelLifecycle::Unloaded));
        assert!(ModelRegistry::is_valid_transition(Some(ModelLifecycle::Unloading), ModelLifecycle::Error("test".to_string())));

        // Invalid transitions
        assert!(!ModelRegistry::is_valid_transition(Some(ModelLifecycle::Unloaded), ModelLifecycle::Active));
        assert!(!ModelRegistry::is_valid_transition(Some(ModelLifecycle::Loading), ModelLifecycle::Unloaded));
        assert!(!ModelRegistry::is_valid_transition(Some(ModelLifecycle::Active), ModelLifecycle::Loading));
    }

    #[test]
    fn thread_safe_registry_new() {
        let config = create_test_config();
        let registry = ThreadSafeModelRegistry::new(config);

        assert!(registry.contains("test-model"));
        assert!(registry.contains("other-model"));
    }

    #[test]
    fn thread_safe_registry_get() {
        let config = create_test_config();
        let registry = ThreadSafeModelRegistry::new(config);

        let spec = registry.get("test-model");
        assert!(spec.is_some());
        assert_eq!(spec.unwrap().name, "Test Model");
    }

    #[test]
    fn thread_safe_registry_list_models() {
        let config = create_test_config();
        let registry = ThreadSafeModelRegistry::new(config);

        let models = registry.list_models();
        assert_eq!(models.len(), 2);
    }

    #[test]
    fn thread_safe_registry_get_state() {
        let config = create_test_config();
        let registry = ThreadSafeModelRegistry::new(config);

        let state = registry.get_state("test-model");
        assert!(state.is_some());
        assert_eq!(state.unwrap(), ModelLifecycle::Unloaded);
    }

    #[test]
    fn thread_safe_registry_model_names() {
        let config = create_test_config();
        let registry = ThreadSafeModelRegistry::new(config);

        let names = registry.model_names();
        assert_eq!(names.len(), 2);
    }

    #[test]
    fn thread_safe_registry_contains() {
        let config = create_test_config();
        let registry = ThreadSafeModelRegistry::new(config);

        assert!(registry.contains("test-model"));
        assert!(!registry.contains("unknown"));
    }

    #[test]
    fn thread_safe_registry_resolve_reference() {
        let config = create_test_config();
        let registry = ThreadSafeModelRegistry::new(config);

        let spec = registry.resolve_reference("${models.test-model}");
        assert!(spec.is_some());
        assert_eq!(spec.unwrap().name, "Test Model");
    }

    #[test]
    fn thread_safe_registry_unload() {
        let config = create_test_config();
        let registry = ThreadSafeModelRegistry::new(config);

        assert!(registry.set_state("test-model", ModelLifecycle::Loaded).is_ok());
        let result = registry.unload_model("test-model");
        assert!(result.is_ok());
    }

    #[test]
    fn thread_safe_registry_load() {
        let config = create_test_config();
        let registry = ThreadSafeModelRegistry::new(config);

        let result = registry.load_model("test-model");
        assert!(result.is_err()); // Not implemented yet
    }

    #[test]
    fn thread_safe_registry_set_state() {
        let config = create_test_config();
        let registry = ThreadSafeModelRegistry::new(config);

        let result = registry.set_state("test-model", ModelLifecycle::Loaded);
        assert!(result.is_ok());
        assert_eq!(registry.get_state("test-model"), Some(ModelLifecycle::Loaded));
    }

    #[test]
    fn thread_safe_registry_with_inner() {
        let config = create_test_config();
        let registry = ThreadSafeModelRegistry::new(config);

        let name = registry.with_inner(|r| {
            r.get("test-model").map(|spec| spec.name.clone())
        });
        assert_eq!(name, Some(Some(String::from("Test Model"))));
    }
}
