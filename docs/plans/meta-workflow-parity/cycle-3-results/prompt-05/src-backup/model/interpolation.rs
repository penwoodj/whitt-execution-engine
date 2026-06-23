use minijinja::Environment;
use std::collections::HashMap;

/// Template interpolator using minijinja for resolving references.
///
/// Supports two reference types:
/// - `${models.xxx}` - Structural references resolved at parse time
/// - `{{step.xxx.output}}` - Runtime references resolved at execution time
pub struct TemplateInterpolator {
    env: Environment<'static>,
}

impl TemplateInterpolator {
    pub fn new() -> Self {
        let mut env = Environment::new();

        env.set_debug(true);

        tracing::debug!("TemplateInterpolator initialized");

        Self { env }
    }

    pub fn resolve_structural(&self, template: &str, models: &HashMap<String, String>) -> Result<String, anyhow::Error> {
        if !Self::is_structural_reference(template) {
            return Ok(template.to_string());
        }

        tracing::debug!(
            template = %template,
            model_count = models.len(),
            "Resolving structural template"
        );

        let mut result = template.to_string();

        for (key, value) in models {
            let placeholder = format!("${{models.{}}}", key);
            result = result.replace(&placeholder, value);
        }

        Ok(result)
    }

    pub fn resolve_runtime(&self, template: &str, context: &HashMap<String, String>) -> Result<String, anyhow::Error> {
        if !Self::is_runtime_reference(template) {
            return Ok(template.to_string());
        }

        tracing::debug!(
            template = %template,
            context_count = context.len(),
            "Resolving runtime template"
        );

        let template_str = self
            .env
            .template_from_str(template)
            .map_err(|e| anyhow::anyhow!("Template parsing error: {}", e))?;

        let rendered = template_str
            .render(context)
            .map_err(|e| anyhow::anyhow!("Template rendering error: {}", e))?;

        Ok(rendered)
    }

    pub fn is_structural_reference(s: &str) -> bool {
        s.contains("${models.") || s.contains("${workflow.") || s.contains("${workspace.")
    }

    pub fn is_runtime_reference(s: &str) -> bool {
        s.contains("{{")
    }
}

impl Default for TemplateInterpolator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_structural_reference_detects_models_reference() {
        assert!(TemplateInterpolator::is_structural_reference("${models.my-model}"));
        assert!(TemplateInterpolator::is_structural_reference("Use ${models.primary} for this task"));
    }

    #[test]
    fn is_structural_reference_detects_workflow_reference() {
        assert!(TemplateInterpolator::is_structural_reference("${workflow.timeout}"));
    }

    #[test]
    fn is_structural_reference_detects_workspace_reference() {
        assert!(TemplateInterpolator::is_structural_reference("${workspace.path}"));
    }

    #[test]
    fn is_structural_reference_returns_false_for_no_reference() {
        assert!(!TemplateInterpolator::is_structural_reference("plain text"));
        assert!(!TemplateInterpolator::is_structural_reference("Use step.output for this"));
    }

    #[test]
    fn is_runtime_reference_detects_jinja_syntax() {
        assert!(TemplateInterpolator::is_runtime_reference("{{step.xxx.output}}"));
        assert!(TemplateInterpolator::is_runtime_reference("Value: {{inputs.target}}"));
    }

    #[test]
    fn is_runtime_reference_returns_false_for_no_jinja_syntax() {
        assert!(!TemplateInterpolator::is_runtime_reference("${models.xxx}"));
        assert!(!TemplateInterpolator::is_runtime_reference("plain text"));
    }

    #[test]
    fn resolve_structural_replaces_model_references() {
        let interpolator = TemplateInterpolator::new();
        let mut models = HashMap::new();
        models.insert("primary".to_string(), "llama-3.2-3b".to_string());

        let result = interpolator
            .resolve_structural("${models.primary}", &models)
            .unwrap();

        assert_eq!(result, "llama-3.2-3b");
    }

    #[test]
    fn resolve_structural_replaces_multiple_references() {
        let interpolator = TemplateInterpolator::new();
        let mut models = HashMap::new();
        models.insert("primary".to_string(), "llama-3.2-3b".to_string());
        models.insert("secondary".to_string(), "llama3.2".to_string());

        let result = interpolator
            .resolve_structural("Primary: ${models.primary}, Secondary: ${models.secondary}", &models)
            .unwrap();

        assert_eq!(result, "Primary: llama-3.2-3b, Secondary: llama3.2");
    }

    #[test]
    fn resolve_structural_returns_original_if_no_match() {
        let interpolator = TemplateInterpolator::new();
        let models = HashMap::new();

        let result = interpolator
            .resolve_structural("${models.unknown}", &models)
            .unwrap();

        assert_eq!(result, "${models.unknown}");
    }

    #[test]
    fn resolve_structural_returns_non_reference_text() {
        let interpolator = TemplateInterpolator::new();
        let models = HashMap::new();

        let result = interpolator
            .resolve_structural("This is plain text", &models)
            .unwrap();

        assert_eq!(result, "This is plain text");
    }

    #[test]
    fn resolve_runtime_replaces_jinja_variables() {
        let interpolator = TemplateInterpolator::new();
        let mut context = HashMap::new();
        context.insert("name".to_string(), "test-value".to_string());

        let result = interpolator
            .resolve_runtime("{{name}}", &context)
            .unwrap();

        assert_eq!(result, "test-value");
    }

    #[test]
    fn resolve_runtime_handles_nested_variables() {
        let interpolator = TemplateInterpolator::new();
        let mut context = HashMap::new();
        context.insert("step".to_string(), "analyze_code".to_string());
        context.insert("output".to_string(), "result".to_string());

        let result = interpolator
            .resolve_runtime("{{step}}-{{output}}", &context)
            .unwrap();

        assert_eq!(result, "analyze_code-result");
    }

    #[test]
    fn resolve_runtime_returns_empty_for_missing_variable() {
        let interpolator = TemplateInterpolator::new();
        let context = HashMap::new();

        let result = interpolator.resolve_runtime("{{missing_var}}", &context).unwrap();

        assert_eq!(result, "");
    }

    #[test]
    fn resolve_runtime_returns_non_template_text() {
        let interpolator = TemplateInterpolator::new();
        let context = HashMap::new();

        let result = interpolator
            .resolve_runtime("Plain text", &context)
            .unwrap();

        assert_eq!(result, "Plain text");
    }
}
