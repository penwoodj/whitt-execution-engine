//! SW4 fast regression validation.
//!
//! Permanent fast-check test (runs on every push). Confirms the deterministic
//! SW4→SW5 assembler (scripts/meta-v6/build-workflow.py) produces:
//!   1. Valid YAML on a known sample input
//!   2. Correctly rewrites bare `cat step_*.txt` → `$WHITT_OUTPUT_DIR/outputs/`
//!   3. Correctly rewrites `cat ./outputs/step_*.txt` → `$WHITT_OUTPUT_DIR/outputs/`
//!   4. Strips unknown step-level fields (intent, fit, description at step level)
//!   5. Preserves schema-valid step fields (generative_entity, prompt, etc.)
//!
//! Regression scope: the 3 fixes committed in commits 7e4dbbe, a2dded1, 35e6abf.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn build_workflow_py() -> PathBuf {
    repo_root().join("scripts/meta-v6/build-workflow.py")
}

/// Run build-workflow.py on `input_md`, return assembled YAML.
fn assemble(input_md: &str) -> String {
    let tmp = tempfile::tempdir().expect("tempdir");
    let in_path = tmp.path().join("input.md");
    let out_path = tmp.path().join("out.yml");
    fs::write(&in_path, input_md).expect("write input");

    let status = Command::new("python3")
        .arg(build_workflow_py())
        .arg(&in_path)
        .arg(&out_path)
        .env("WHITT_META_RUN_ID", "test-meta-id-12345")
        .env("WHITT_PROMPT_FILE", "prompt-test.md")
        .env("WHITT_OUTPUT_DIR", "/tmp/whitt-test-output")
        .env("WHITT_DELIVERABLE_FILENAME", "step_final.txt")
        .output()
        .expect("run python");

    if !status.status.success() {
        panic!(
            "build-workflow.py failed: stderr={}",
            String::from_utf8_lossy(&status.stderr)
        );
    }

    fs::read_to_string(&out_path).expect("read output")
}

#[test]
fn given_sample_sw4_input_when_assembledthen_produces_valid_yaml() {
    let input = r#"# SW4 Output

```yaml
step_1_generate:
  generative_entity: "${models.qwen35}"
  prompt: |
    Generate something.
```
"#;
    let yaml_str = assemble(input);
    let parsed: serde_yaml::Value = serde_yaml::from_str(&yaml_str).unwrap_or_else(|e| {
        panic!("assembled YAML does not parse: {}\n--- output ---\n{}", e, yaml_str)
    });
    assert!(parsed.is_mapping(), "assembled YAML must be a mapping");
}

#[test]
fn given_bare_cat_in_input_when_assembled_then_rewrites_to_whitt_output_dir() {
    let input = r#"# SW4

```yaml
step_1_read:
  generative_entity: "${models.qwen35}"
  prompt: |
    Read the prior output:
    ```bash
    cat step_t1_breakdown.txt
    ```
```
"#;
    let yaml_str = assemble(input);
    assert!(
        yaml_str.contains("$WHITT_OUTPUT_DIR/outputs/step_t1_breakdown.txt"),
        "bare cat must be prefixed with $WHITT_OUTPUT_DIR/outputs/. Got:\n{}",
        yaml_str
    );
}

#[test]
fn given_dot_slash_outputs_prefix_when_assembled_then_rewrites_to_whitt_output_dir() {
    let input = r#"# SW4

```yaml
step_1_read:
  generative_entity: "${models.qwen35}"
  prompt: |
    Read:
    ```bash
    cat ./outputs/step_t2_implement.txt
    ```
```
"#;
    let yaml_str = assemble(input);
    assert!(
        !yaml_str.contains("./outputs/step_t2_implement.txt"),
        "./outputs/ prefix must be rewritten. Got:\n{}",
        yaml_str
    );
    assert!(
        yaml_str.contains("$WHITT_OUTPUT_DIR/outputs/step_t2_implement.txt"),
        "must contain rewritten path. Got:\n{}",
        yaml_str
    );
}

#[test]
fn given_unknown_step_fields_when_assembled_then_strips_them() {
    let input = r#"# SW4

```yaml
step_1_generate:
  intent: Generate code for the task
  fit: This step is needed because the user asked for code
  description: A step that generates code
  generative_entity: "${models.qwen35}"
  prompt: |
    Do something.
```
"#;
    let yaml_str = assemble(input);
    assert!(
        !yaml_str.contains("intent: Generate code"),
        "unknown field `intent` must be stripped. Got:\n{}",
        yaml_str
    );
    assert!(
        !yaml_str.contains("fit: This step"),
        "unknown field `fit` must be stripped. Got:\n{}",
        yaml_str
    );
    assert!(
        !yaml_str.contains("description: A step"),
        "unknown field `description` at step level must be stripped. Got:\n{}",
        yaml_str
    );
    assert!(
        yaml_str.contains("generative_entity:"),
        "schema-valid field `generative_entity` must be preserved. Got:\n{}", yaml_str
    );
    assert!(
        yaml_str.contains("step_1_generate:"),
        "step YAML key `step_1_generate:` must be present. Got:\n{}", yaml_str
    );
}

#[test]
fn given_schema_valid_fields_when_assembled_then_preserves_them() {
    let input = r#"# SW4

```yaml
step_1_generate:
  generative_entity: "${models.qwen35}"
  prompt: |
    Generate.
  depends_on: [bootstrap]
  retry:
    attempts: 3
    backoff_seconds: 5
```
"#;
    let yaml_str = assemble(input);
    assert!(
        yaml_str.contains("generative_entity:"),
        "schema-valid field `generative_entity` must be preserved. Got:\n{}", yaml_str
    );
    assert!(
        yaml_str.contains("step_1_generate:"),
        "step YAML key `step_1_generate:` must be present. Got:\n{}", yaml_str
    );
}

#[test]
fn given_depends_on_cat_pattern_when_assembled_then_keeps_step_ids() {
    let input = r#"# SW4

```yaml
step_1_gen:
  generative_entity: "${models.qwen35}"
  prompt: |
    Generate.

step_2_use:
  generative_entity: "${models.qwen35}"
  prompt: |
    ```bash
    cat $WHITT_OUTPUT_DIR/outputs/step_1_gen.txt
    ```
```
"#;
    let yaml_str = assemble(input);
    assert!(
        yaml_str.contains("step_1_gen"),
        "step_1_gen reference must be present. Got:\n{}", yaml_str
    );
}
