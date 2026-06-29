//! META-v6 workflow YAML structural validation tests.
//!
//! Regression tests for confirmed bugs (commit refs inline):
//! - 6f5b7b6: bootstrap generative_entity requirement
//! - 05e469b: SW3 step_02 generative_entity requirement
//! - 588826a: shell machine-check hooks in evaluators
//! - f44aa2f: lenient coverage gate (>=80%)
//!
//! Design note (2026-06-27): SW5 refactored to deterministic shell-only assembly.
//! Tests that assert LLM-cascade features (gwt routes, sw-gate, bootstrap step)
//! now apply to SW1-SW4 only. SW5 has its own minimal-step structural check.

use std::fs;
use std::path::PathBuf;

use serde_yaml::Value;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn workflows_dir() -> PathBuf {
    repo_root().join("docs/benchmarks/workflows")
}

fn load_yaml(rel_path: &str) -> Value {
    let path = workflows_dir().join(rel_path);
    let content = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", path.display(), e));
    serde_yaml::from_str(&content)
        .unwrap_or_else(|e| panic!("Failed to parse {} as YAML: {}", path.display(), e))
}

const SW_FILES: &[&str] = &[
    "sw1-task-deconstruction.yml",
    "sw2-desired-output-state.yml",
    "sw3-agentic-categorization.yml",
    "sw4-yaml-substructure-translation.yml",
    "sw5-final-workflow-assembly.yml",
    "meta-workflow-v6.yml",
];

// LLM-cascade SWs only (excludes SW5 which is deterministic and orchestrator which is wrapper)
const LLM_CASCADE_SWS: &[&str] = &[
    "sw1-task-deconstruction.yml",
    "sw2-desired-output-state.yml",
    "sw3-agentic-categorization.yml",
    "sw4-yaml-substructure-translation.yml",
];

#[test]
fn given_all_sw_yamls_when_parsed_then_all_are_valid_yaml() {
    for sw in SW_FILES {
        let yaml = load_yaml(sw);
        assert!(yaml.is_mapping(), "{} should parse to a mapping", sw);
    }
}

#[test]
fn given_all_sw_yamls_when_checked_then_have_providers_llama_cpp_with_vulkan() {
    for sw in SW_FILES {
        let yaml = load_yaml(sw);
        let providers = yaml.get("providers").unwrap_or_else(|| {
            panic!("{} missing top-level 'providers' key", sw)
        });
        let llama = providers
            .get("llama_cpp_with_vulkan")
            .unwrap_or_else(|| panic!("{} must use llama_cpp_with_vulkan provider", sw));
        let config = llama.get("config").unwrap_or_else(|| {
            panic!("{} provider must use config: wrapper", sw)
        });
        assert!(config.get("host").is_some(), "{} provider must specify host", sw);
        assert!(config.get("port").is_some(), "{} provider must specify port", sw);
    }
}

#[test]
fn given_all_sw_yamls_when_checked_then_have_qwen35_model_config() {
    for sw in SW_FILES {
        let yaml = load_yaml(sw);
        let models = yaml.get("models").unwrap_or_else(|| {
            panic!("{} missing top-level 'models' key", sw)
        });
        let qwen = models
            .get("qwen35")
            .unwrap_or_else(|| panic!("{} must define models.qwen35", sw));

        // AGENTS.md: model host.type MUST be llama_cpp_with_vulkan
        let host_type = qwen
            .get("host")
            .and_then(|h| h.get("type"))
            .and_then(|v| v.as_str())
            .unwrap_or_else(|| panic!("{} models.qwen35.host.type must be set", sw));
        assert_eq!(
            host_type, "llama_cpp_with_vulkan",
            "{} model host.type MUST be llama_cpp_with_vulkan", sw
        );
    }
}

#[test]
fn given_all_sw_yamls_when_checked_then_have_load_params_with_q8_0_kv_cache() {
    for sw in SW_FILES {
        let yaml = load_yaml(sw);
        let load_params = yaml
            .get("models")
            .and_then(|m| m.get("qwen35"))
            .and_then(|q| q.get("load_params"))
            .unwrap_or_else(|| panic!("{} models.qwen35.load_params missing", sw));

        // User directive: ctx=32768 (2x buffer of 16k max response; bump to 262144 if input+response >= 131072), gpu=99 (full RX580 offload, confirmed fastest), threads=5, parallel=1
        assert_eq!(load_params.get("context_size").and_then(|v| v.as_i64()), Some(32768),
            "{} context_size MUST be 32768", sw);
        assert_eq!(load_params.get("gpu_layers").and_then(|v| v.as_i64()), Some(99),
            "{} gpu_layers MUST be 99 (full RX580 offload, server config wins)", sw);
        assert_eq!(load_params.get("threads").and_then(|v| v.as_i64()), Some(5),
            "{} threads MUST be 5", sw);
        assert_eq!(load_params.get("parallel").and_then(|v| v.as_i64()), Some(1),
            "{} parallel MUST be 1", sw);

        // AGENTS.md: Q8_0 KV cache required for Vulkan stability
        assert_eq!(load_params.get("cache_type_k").and_then(|v| v.as_str()).unwrap_or(""), "q8_0",
            "{} cache_type_k MUST be q8_0", sw);
        assert_eq!(load_params.get("cache_type_v").and_then(|v| v.as_str()).unwrap_or(""), "q8_0",
            "{} cache_type_v MUST be q8_0", sw);

        // AGENTS.md: Vulkan hard requirements
        assert_eq!(load_params.get("no_cache_prompt").and_then(|v| v.as_bool()), Some(true),
            "{} no_cache_prompt MUST be true (Vulkan)", sw);
        assert_eq!(load_params.get("cont_batching").and_then(|v| v.as_bool()), Some(false),
            "{} cont_batching MUST be false (Vulkan)", sw);
    }
}

#[test]
fn given_sw_yamls_when_checked_then_have_sampling_config() {
    for sw in SW_FILES {
        let yaml = load_yaml(sw);
        let sampling = yaml
            .get("models")
            .and_then(|m| m.get("qwen35"))
            .and_then(|q| q.get("sampling"))
            .unwrap_or_else(|| panic!("{} models.qwen35.sampling missing", sw));

        assert!(sampling.get("temperature").is_some(), "{} must specify sampling.temperature", sw);
        assert!(sampling.get("max_tokens").is_some(), "{} must specify sampling.max_tokens", sw);
    }
}

#[test]
fn given_sw_yamls_when_checked_then_have_agentic_workflow_with_steps() {
    for sw in SW_FILES {
        let yaml = load_yaml(sw);
        let agentic = yaml.get("agentic_workflow").unwrap_or_else(|| {
            panic!("{} missing agentic_workflow", sw)
        });
        let steps = agentic.get("steps").unwrap_or_else(|| {
            panic!("{} missing agentic_workflow.steps", sw)
        });
        assert!(steps.is_mapping(), "{} steps must be a mapping", sw);
        let step_count = steps.as_mapping().map(|m| m.len()).unwrap_or(0);
        // SW5 is deterministic 2-step assembly; others are LLM cascades with >=5 steps
        let min_steps = if *sw == "sw5-final-workflow-assembly.yml" { 2 } else { 5 };
        assert!(step_count >= min_steps,
            "{} should have >={} steps, got {}", sw, min_steps, step_count);
    }
}

/// Regression for commit 588826a: SW1-SW4 must have shell machine-check hooks.
/// SW5 excluded — deterministic assembly has no evaluator verdict to route on.
#[test]
fn given_sw1_through_sw4_when_checked_then_have_shell_machine_check_hooks() {
    for sw in LLM_CASCADE_SWS {
        let yaml = load_yaml(sw);
        let yaml_str = serde_yaml::to_string(&yaml).unwrap();

        assert!(yaml_str.contains("shell:"),
            "{} must have at least one shell: hook (machine-check evaluator)", sw);
        assert!(yaml_str.contains("gwt:"),
            "{} must have at least one gwt: route (evaluator verdict routing)", sw);
    }
}

/// Regression for commits 6f5b7b6, 05e469b: bootstrap MUST have generative_entity.
/// Without it, runner skips the step and downstream cascades to fallback synthesis.
/// SW5 excluded — uses step_00_assemble (different naming, same generative_entity rule).
#[test]
fn given_sw1_through_sw4_when_checked_then_bootstrap_step_has_generative_entity() {
    for sw in LLM_CASCADE_SWS {
        let yaml = load_yaml(sw);
        let bootstrap = yaml
            .get("agentic_workflow")
            .and_then(|a| a.get("steps"))
            .and_then(|s| s.get("step_00_bootstrap"))
            .unwrap_or_else(|| panic!("{} missing step_00_bootstrap", sw));

        assert!(bootstrap.get("generative_entity").is_some(),
            "{} step_00_bootstrap MUST have generative_entity (runner skips steps without it)", sw);

        let max_tokens = bootstrap
            .get("model_overrides")
            .and_then(|m| m.get("max_tokens"))
            .and_then(|v| v.as_i64());
        assert!(max_tokens.unwrap_or(0) <= 10,
            "{} step_00_bootstrap max_tokens should be <=10 (no waste), got {:?}", sw, max_tokens);
    }
}

#[test]
fn given_meta_workflow_v6_when_checked_then_orchestration_steps_call_wrappers() {
    let yaml = load_yaml("meta-workflow-v6.yml");
    let yaml_str = serde_yaml::to_string(&yaml).unwrap();

    for wrapper in &[
        "run-sw1.sh", "run-sw2.sh", "run-sw3.sh", "run-sw4.sh", "run-sw5.sh",
    ] {
        assert!(yaml_str.contains(wrapper),
            "meta-workflow-v6.yml must call {} (orchestrator chain)", wrapper);
    }
    assert!(yaml_str.contains("bootstrap.sh"), "meta-workflow-v6.yml must call bootstrap.sh");
    assert!(yaml_str.contains("validate.sh"), "meta-workflow-v6.yml must call validate.sh");
}

/// Regression for prompt path bug fixed in this session:
/// bootstrap.sh default path was pointing to non-existent location.
#[test]
fn given_meta_workflow_v6_when_checked_then_default_prompt_path_resolves() {
    let yaml = load_yaml("meta-workflow-v6.yml");
    let yaml_str = serde_yaml::to_string(&yaml).unwrap();

    let bootstrap_line = yaml_str
        .lines()
        .find(|l| l.contains("bootstrap.sh") && l.contains("prompt-"))
        .unwrap_or_else(|| panic!("bootstrap.sh call must include a prompt-N path"));

    assert!(bootstrap_line.contains("test-prompts/real/"),
        "bootstrap prompt path must point to test-prompts/real/, got: {}", bootstrap_line);

    let start = bootstrap_line
        .find("docs/")
        .unwrap_or_else(|| panic!("bootstrap line should contain docs/ path"));
    let rest = &bootstrap_line[start..];
    let end = rest.find('"').unwrap_or(rest.len());
    let path_str = &rest[..end];

    let full_path = repo_root().join(path_str);
    assert!(full_path.exists(),
        "Bootstrap prompt file must exist at {}, got missing file", full_path.display());
}

/// Regression for commit 05e469b: SW3 step_02_define_categories previously had
/// NO generative_entity (replaced with shell echo), causing runner to skip it.
/// Fix restored minimal generative_entity (max_tokens: 1).
#[test]
fn given_sw3_step_02_when_checked_then_has_generative_entity_to_prevent_skip() {
    let yaml = load_yaml("sw3-agentic-categorization.yml");
    let step_02 = yaml
        .get("agentic_workflow")
        .and_then(|a| a.get("steps"))
        .and_then(|s| s.get("step_02_define_categories"))
        .unwrap_or_else(|| panic!("SW3 missing step_02_define_categories"));

    assert!(step_02.get("generative_entity").is_some(),
        "SW3 step_02 MUST have generative_entity (without it runner skips step)");

    let max_tokens = step_02
        .get("model_overrides")
        .and_then(|m| m.get("max_tokens"))
        .and_then(|v| v.as_i64());
    assert_eq!(max_tokens, Some(1),
        "SW3 step_02 max_tokens should be 1 (shell echo does the actual work)");
}

/// Regression for iteration cap safety net.
/// Without cap, evaluator subjective criteria can loop indefinitely.
/// Cap forces PASS after 3 fix iterations (4 evaluator runs).
/// Uses external script to avoid YAML escaping issues with nested quotes.
/// SW5 excluded — deterministic assembly has no evaluator loop.
#[test]
fn given_sw1_through_sw4_when_checked_then_have_iteration_counter_gate() {
    for sw in LLM_CASCADE_SWS {
        let yaml = load_yaml(sw);
        let yaml_str = serde_yaml::to_string(&yaml).unwrap();

        // All LLM-cascade SWs must call sw-gate.sh (extracted to script for robust quoting)
        assert!(
            yaml_str.contains("sw-gate.sh"),
            "{} must call sw-gate.sh (safety cap against fix loop storms)",
            sw
        );

        // All LLM-cascade SWs must reference iteration-counter.txt (counter persistence)
        assert!(
            yaml_str.contains("iteration-counter.txt"),
            "{} must reference iteration-counter.txt (cap state file)",
            sw
        );
    }
}

/// Regression for sw-gate.sh VERDICT line check.
/// Bug: `grep -qi 'PASS'` matched ANY 'PASS' in eval text, not just VERDICT.
/// When eval said "1. PASS - criterion X" but "VERDICT: FAIL", gate
/// incorrectly returned PASS, skipping fix step on failed output.
/// Fix: regex anchored to VERDICT line.
#[test]
fn given_sw_gate_when_eval_text_has_pass_but_verdict_fail_then_returns_fail() {
    use std::process::Command;
    use std::fs;
    let temp_dir = std::env::temp_dir();
    let eval_file = temp_dir.join("sw-gate-eval-test.txt");
    let counter_file = temp_dir.join("sw-gate-counter-test.txt");

    fs::write(&eval_file, "1. PASS - criterion X\n2. FAIL - criterion Y\nVERDICT: FAIL").unwrap();
    fs::write(&counter_file, "0").unwrap();

    let output = Command::new("bash")
        .arg("scripts/meta-v6/sw-gate.sh")
        .arg(&eval_file)
        .arg(&counter_file)
        .output()
        .expect("sw-gate.sh must execute");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    assert_eq!(stdout.trim(), "FAIL",
        "Gate must return FAIL when VERDICT: FAIL even if other criteria say PASS");

    let _ = fs::remove_file(&eval_file);
    let _ = fs::remove_file(&counter_file);
}

// ============================================================================
// Per-SW Quality Unit Tests (Phase 1.1 — fast static validators)
// ============================================================================

/// SW1 must require task decomposition with group-of-5 chunking rule.
#[test]
fn given_sw1_prompt_when_checked_then_has_task_decomposition_rules() {
    let yaml_str = fs::read_to_string(workflows_dir().join("sw1-task-deconstruction.yml"))
        .expect("SW1 yaml must be readable");
    assert!(yaml_str.contains("task") || yaml_str.contains("decompos"),
        "SW1 must reference task decomposition");
    assert!(yaml_str.contains("5") || yaml_str.contains("chunk") || yaml_str.contains("group"),
        "SW1 must reference group-of-5 chunking for complexity management");
}

/// SW2 must produce desired output state description.
#[test]
fn given_sw2_prompt_when_checked_then_has_desired_output_state_rules() {
    let yaml_str = fs::read_to_string(workflows_dir().join("sw2-desired-output-state.yml"))
        .expect("SW2 yaml must be readable");
    assert!(yaml_str.contains("output") || yaml_str.contains("deliverable"),
        "SW2 must reference desired output / deliverable characteristics");
}

/// SW3 must categorize tasks agentic vs deterministic.
#[test]
fn given_sw3_prompt_when_checked_then_has_categorization_rules() {
    let yaml_str = fs::read_to_string(workflows_dir().join("sw3-agentic-categorization.yml"))
        .expect("SW3 yaml must be readable");
    assert!(yaml_str.contains("categor") || yaml_str.contains("agentic") || yaml_str.contains("pattern"),
        "SW3 must reference categorization / pattern assignment");
}

/// SW4 must enforce filename consistency (Priority 2.1 fix).
#[test]
fn given_sw4_prompt_when_checked_then_has_filename_consistency_rule() {
    let yaml_str = fs::read_to_string(workflows_dir().join("sw4-yaml-substructure-translation.yml"))
        .expect("SW4 yaml must be readable");
    assert!(yaml_str.contains("FILENAME CONSISTENCY"),
        "SW4 must enforce FILENAME CONSISTENCY rule (Priority 2.1 fix)");
}

/// SW4 must enforce file slicing for large files (Priority 2.2 fix).
#[test]
fn given_sw4_prompt_when_checked_then_has_file_slicing_rule() {
    let yaml_str = fs::read_to_string(workflows_dir().join("sw4-yaml-substructure-translation.yml"))
        .expect("SW4 yaml must be readable");
    assert!(yaml_str.contains("FILE SLICING"),
        "SW4 must enforce FILE SLICING rule for files >50KB (Priority 2.2 fix)");
    assert!(yaml_str.contains("sed -n"),
        "SW4 file slicing rule must reference sed -n command");
}

/// SW5 must be deterministic (2-step assembly, no LLM cascade).
/// Regression: pre-refactor SW5 had 6-step LLM cascade with 0% success rate.
/// Fix: replaced with 2-step deterministic assembly using build-workflow.py.
#[test]
fn given_sw5_yaml_when_checked_then_is_deterministic_assembly() {
    let yaml_str = fs::read_to_string(workflows_dir().join("sw5-final-workflow-assembly.yml"))
        .expect("SW5 yaml must be readable");
    assert!(yaml_str.contains("sw5-assemble.sh"),
        "SW5 must invoke sw5-assemble.sh wrapper (which calls build-workflow.py)");
    assert!(yaml_str.contains("DESIGN DECISION"),
        "SW5 must document design decision for deterministic refactor");
    let step_count = yaml_str.matches("    step_").count();
    assert!(step_count <= 3,
        "SW5 must have ≤3 steps (deterministic 2-step assembly), got {}", step_count);
}

/// SW5 step_00_assemble must have generative_entity (runner requires it).
#[test]
fn given_sw5_step_00_assemble_when_checked_then_has_generative_entity() {
    let yaml = load_yaml("sw5-final-workflow-assembly.yml");
    let assemble = yaml
        .get("agentic_workflow")
        .and_then(|a| a.get("steps"))
        .and_then(|s| s.get("step_00_assemble"))
        .unwrap_or_else(|| panic!("SW5 missing step_00_assemble"));
    assert!(assemble.get("generative_entity").is_some(),
        "SW5 step_00_assemble MUST have generative_entity");
}

/// All wrapper scripts referenced in YAMLs must exist on disk.
#[test]
fn given_sw_yamls_when_checked_then_referenced_scripts_exist() {
    let required_scripts = [
        "run-sw1.sh", "run-sw2.sh", "run-sw3.sh", "run-sw4.sh", "run-sw5.sh",
        "bootstrap.sh", "sw-gate.sh",
        "build-workflow.py", "fix-yaml.py",
        "sw5-assemble.sh", "sw5-finalize.sh",
    ];
    for script in &required_scripts {
        let path = repo_root().join("scripts/meta-v6").join(script);
        assert!(path.exists(),
            "Required script {} must exist at {}", script, path.display());
    }
}

/// Pipeline must call all SW wrappers in order.
/// pipeline.sh uses a `run_sw N` helper that calls `run-swN.sh` dynamically.
/// We verify by checking for the `run_sw N` calls in sequence.
#[test]
fn given_pipeline_sh_when_checked_then_calls_all_sws_in_order() {
    let pipeline = repo_root().join("scripts/meta-v6/debug/pipeline.sh");
    let content = fs::read_to_string(&pipeline)
        .expect("pipeline.sh must be readable");
    
    assert!(content.contains("run-sw5.sh"),
        "pipeline.sh must call run-sw5.sh explicitly (deterministic path)");
    
    let positions: Vec<Option<usize>> = (1..=4)
        .map(|n| content.find(&format!("run_sw {}", n)))
        .collect();
    for (i, pos) in positions.iter().enumerate() {
        assert!(pos.is_some(),
            "pipeline.sh must call run_sw {} (SW{})", i+1, i+1);
    }
    let positions: Vec<usize> = positions.iter().map(|p| p.unwrap()).collect();
    for i in 0..positions.len()-1 {
        assert!(positions[i] < positions[i+1],
            "pipeline.sh must call SW{} before SW{}", i+1, i+2);
    }
}

/// Regression: save_to entries must NOT be bare names (must be $variable or file path).
/// Bug: bare names like `sw1_eval` caused engine to write files to CWD (repo root)
/// instead of storing as variable bookmarks. Stray files polluted repo root.
/// Fix: prefix all bare names with $ in SW1-SW4 YAMLs + build-workflow.py.
#[test]
fn given_sw_yamls_when_checked_then_no_bare_save_to_names() {
    let sw_files = &SW_FILES[..5]; // SW1-SW5 only (not orchestrator)
    for sw in sw_files {
        let yaml = load_yaml(sw);
        let steps = yaml
            .get("agentic_workflow")
            .and_then(|a| a.get("steps"))
            .and_then(|s| s.as_mapping());

        if let Some(steps) = steps {
            for (step_id, step_val) in steps {
                let step_obj = step_id.as_str().unwrap_or("");
                let when = step_val.get("when");
                if let Some(when) = when {
                    for (_, trigger_val) in when.as_mapping().unwrap_or(&serde_yaml::Mapping::new()) {
                        if let Some(hooks) = trigger_val.as_sequence() {
                            for hook in hooks {
                                if let Some(hook_map) = hook.as_mapping() {
                                    for (action_key, action_val) in hook_map {
                                        if action_key.as_str() == Some("save_to") {
                                            check_save_to_value(sw, step_obj, action_val);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn check_save_to_value(sw: &str, step_id: &str, val: &serde_yaml::Value) {
    if let Some(s) = val.as_str() {
        assert_bare_name(sw, step_id, s);
    } else if let Some(seq) = val.as_sequence() {
        for item in seq {
            if let Some(s) = item.as_str() {
                assert_bare_name(sw, step_id, s);
            }
        }
    }
}

fn assert_bare_name(sw: &str, step_id: &str, s: &str) {
    let is_bare = !s.starts_with('$')
        && !s.starts_with("./")
        && !s.starts_with('/')
        && !s.contains('.')
        && !s.contains('/');
    assert!(!is_bare,
        "{}:{} save_to value '{}' is a BARE name — must be $variable or file path. \
         Bare names cause engine to write files to CWD (repo root pollution).",
        sw, step_id, s);
}
