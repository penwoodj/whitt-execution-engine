use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::Write;
use tracing::info;

#[derive(Debug, Clone)]
pub enum HookAction {
    Log { to_file_path: String, event_fields: Vec<String> },
    AppendTo { path: String },
    SaveTo { key: String, path: Option<String> },
    Bookmark,
}

#[derive(Debug, Clone)]
pub struct LoopHook {
    pub actions: Vec<HookAction>,
}

impl LoopHook {
    pub fn execute(&self, context: &HookContext) -> Result<(), anyhow::Error> {
        for action in &self.actions {
            match action {
                HookAction::Log { to_file_path, event_fields } => {
                    let line = event_fields.iter()
                        .map(|f| context.get_field(f))
                        .collect::<Vec<_>>()
                        .join(" | ");
                    
                    if let Some(parent) = PathBuf::from(to_file_path).parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    
                    let mut file = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(to_file_path)?;
                    writeln!(file, "{}", line)?;
                    info!("Loop hook logged to {}", to_file_path);
                }
                HookAction::AppendTo { path } => {
                    if let Some(parent) = PathBuf::from(path).parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    let mut file = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(path)?;
                    writeln!(file, "{}", context.output.as_deref().unwrap_or(""))?;
                }
                HookAction::SaveTo { key, path } => {
                    info!("SaveTo hook: key={}", key);
                    if let Some(p) = path {
                        if let Some(parent) = PathBuf::from(p).parent() {
                            std::fs::create_dir_all(parent)?;
                        }
                        std::fs::write(p, context.output.as_deref().unwrap_or(""))?;
                    }
                }
                HookAction::Bookmark => {
                    info!("Bookmark hook at iteration {}", context.iteration);
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct HookContext {
    pub step_name: String,
    pub iteration: usize,
    pub output: Option<String>,
    pub error_message: Option<String>,
    pub loop_type: String,
    pub model_size: Option<String>,
    pub model_family: Option<String>,
    pub model_quantization: Option<String>,
}

impl HookContext {
    fn get_field(&self, field: &str) -> String {
        match field {
            "step_name" => self.step_name.clone(),
            "loop_iteration" | "iteration" => self.iteration.to_string(),
            "loop_type" => self.loop_type.clone(),
            "error_message" => self.error_message.clone().unwrap_or_default(),
            "output" => self.output.clone().unwrap_or_default(),
            _ => format!("unknown_field:{}", field),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_hook_writes_to_file() {
        let temp_dir = tempfile::tempdir().expect("create temp dir");
        let log_path = temp_dir.path().join("test.log").to_string_lossy().to_string();
        
        let hook = LoopHook {
            actions: vec![
                HookAction::Log {
                    to_file_path: log_path.clone(),
                    event_fields: vec!["step_name".to_string(), "iteration".to_string()],
                },
            ],
        };

        let context = HookContext {
            step_name: "test_step".to_string(),
            iteration: 5,
            output: Some("test output".to_string()),
            error_message: None,
            loop_type: "count".to_string(),
            model_size: None,
            model_family: None,
            model_quantization: None,
        };

        hook.execute(&context).expect("execute hook");

        let content = std::fs::read_to_string(&log_path).expect("read log");
        assert!(content.contains("test_step"));
        assert!(content.contains("5"));
    }

    #[test]
    fn appendto_hook_appends_content() {
        let temp_dir = tempfile::tempdir().expect("create temp dir");
        let file_path = temp_dir.path().join("output.txt").to_string_lossy().to_string();
        
        std::fs::write(&file_path, "initial\n").expect("write initial");

        let hook = LoopHook {
            actions: vec![
                HookAction::AppendTo {
                    path: file_path.clone(),
                },
            ],
        };

        let context = HookContext {
            step_name: "test_step".to_string(),
            iteration: 0,
            output: Some("appended".to_string()),
            error_message: None,
            loop_type: "count".to_string(),
            model_size: None,
            model_family: None,
            model_quantization: None,
        };

        hook.execute(&context).expect("execute hook");

        let content = std::fs::read_to_string(&file_path).expect("read file");
        assert_eq!(content, "initial\nappended\n");
    }

    #[test]
    fn hookcontext_get_field_returns_correct_values() {
        let context = HookContext {
            step_name: "my_step".to_string(),
            iteration: 42,
            output: Some("my_output".to_string()),
            error_message: Some("my_error".to_string()),
            loop_type: "validation".to_string(),
            model_size: None,
            model_family: None,
            model_quantization: None,
        };

        assert_eq!(context.get_field("step_name"), "my_step");
        assert_eq!(context.get_field("iteration"), "42");
        assert_eq!(context.get_field("loop_iteration"), "42");
        assert_eq!(context.get_field("loop_type"), "validation");
        assert_eq!(context.get_field("error_message"), "my_error");
        assert_eq!(context.get_field("output"), "my_output");
        assert_eq!(context.get_field("unknown"), "unknown_field:unknown");
    }

    #[test]
    fn save_to_hook_writes_to_file() {
        let temp_dir = tempfile::tempdir().expect("create temp dir");
        let save_path = temp_dir.path().join("saved.txt").to_string_lossy().to_string();
        
        let hook = LoopHook {
            actions: vec![
                HookAction::SaveTo {
                    key: "result".to_string(),
                    path: Some(save_path.clone()),
                },
            ],
        };

        let context = HookContext {
            step_name: "save_test".to_string(),
            iteration: 0,
            output: Some("saved content".to_string()),
            error_message: None,
            loop_type: "count".to_string(),
            model_size: None,
            model_family: None,
            model_quantization: None,
        };

        hook.execute(&context).expect("execute hook");

        let content = std::fs::read_to_string(&save_path).expect("read saved file");
        assert_eq!(content, "saved content");
    }
}
