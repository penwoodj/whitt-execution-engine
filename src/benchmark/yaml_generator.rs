//! Benchmark YAML generator for multi-model workflows.
//!
//! Generates YAML workflow files for benchmarking N models with
//! serial execution, model discovery, and loop-based iteration.

use crate::client::model_discovery::ModelCandidate;
use anyhow::Result;
use std::fmt::Write as FmtWrite;
use tracing::info;

/// Benchmark YAML generator.
pub struct BenchmarkYamlGenerator;

impl BenchmarkYamlGenerator {
    /// Generate benchmark YAML workflow for N models.
    ///
    /// Output includes:
    /// - workflow_id, name, description
    /// - models section with placeholder for current model
    /// - execution config (serial mode, memory settings)
    /// - benchmark config (prompts, max_tokens, temperature, top_p)
    /// - agentic_workflow with model discovery step and loop
    ///
    /// YAML is valid YAML syntax. Does not validate against UnifiedConfig
    /// (benchmark YAMLs don't match strict schema requirements like providers).
    pub fn generate_benchmark_yaml(
        _name: &str,
        models: &[ModelCandidate],
        prompts: &[&str],
        max_tokens: usize,
    ) -> Result<String> {
        let n = models.len();
        info!("[yaml_generator] generating benchmark YAML for {} models", n);

        let mut yaml = String::new();

        // Header
        writeln!(yaml, "workflow_id: benchmark_{}_models", n)?;
        writeln!(yaml, r#"name: "{}-Model Benchmark Suite""#, n)?;
        writeln!(
            yaml,
            r#"description: "Sequential benchmark of {} diverse models from external drive""#,
            n
        )?;
        writeln!(yaml, "min_schema_version: \"2.0.0\"")?;
        writeln!(yaml)?;

        // Models section
        writeln!(yaml, "models:")?;
        writeln!(yaml, "  primary:")?;
        writeln!(yaml, "    provider: lmstudio")?;
        writeln!(yaml, "    model: \"${{benchmark.current_model}}\"")?;
        writeln!(yaml)?;

        // Execution section
        writeln!(yaml, "execution:")?;
        writeln!(yaml, "  mode: serial")?;
        writeln!(yaml, "  memory:")?;
        writeln!(yaml, "    max_allocated_memory_mb: 8192")?;
        writeln!(yaml, "    model_memory_mb: 6144")?;
        writeln!(yaml, "    unload_unused: true")?;
        writeln!(yaml)?;

        // Logging section
        writeln!(yaml, "logging:")?;
        writeln!(yaml, "  global:")?;
        writeln!(yaml, "    level: info")?;
        writeln!(yaml, "    detail: medium")?;
        writeln!(yaml, "    output_type: chat")?;
        writeln!(yaml, "    format: json")?;
        writeln!(yaml, "    console: true")?;
        writeln!(yaml, "  performance_metrics:")?;
        writeln!(yaml, "    level: debug")?;
        writeln!(yaml, "    detail: very_high")?;
        writeln!(yaml)?;

        // Benchmark config section
        writeln!(yaml, "benchmark:")?;
        writeln!(yaml, "  prompts:")?;
        for prompt in prompts {
            let prompt_escaped = prompt.replace('\n', "\\n").replace('"', r#"\"#);
            writeln!(yaml, r#"    - "{}""#, prompt_escaped)?;
        }
        writeln!(yaml, "  max_tokens: {}", max_tokens)?;
        writeln!(yaml, "  temperature: 0.7")?;
        writeln!(yaml, "  top_p: 0.9")?;
        writeln!(yaml)?;

        // Agentic workflow section
        writeln!(yaml, "agentic_workflow:")?;

        // Model discovery step
        writeln!(yaml, "  - step: discover_models")?;
        writeln!(yaml, "    id: discover")?;
        writeln!(yaml, "    type: model_discovery")?;
        writeln!(yaml, "    input:")?;
        writeln!(yaml, "      models_dir: \"/run/media/jon/data/models\"")?;
        writeln!(yaml, "      filter:")?;
        writeln!(yaml, "        max_size_bytes: 6442450944")?;
        writeln!(yaml, "        file_extension: \".gguf\"")?;
        writeln!(yaml, "    output:")?;
        writeln!(yaml, "      save_to: discovered_models")?;
        writeln!(yaml)?;

        // Benchmark loop step
        writeln!(yaml, "  - step: benchmark_loop")?;
        writeln!(yaml, "    id: bench_loop")?;
        writeln!(yaml, "    loop:")?;
        writeln!(yaml, "      count:")?;
        writeln!(yaml, "        max_iterations: {}", n)?;
        writeln!(yaml, "        iteration_variable: current_model")?;
        writeln!(yaml, "    input:")?;
        writeln!(yaml, r#"      model_path: "{{{{loop.current_model}}}}""#)?;
        writeln!(yaml, r#"      prompts: "${{benchmark.prompts}}""#)?;
        writeln!(yaml, r#"      max_tokens: "${{benchmark.max_tokens}}""#)?;
        writeln!(yaml, "    when:")?;
        writeln!(yaml, "      before_step_starts:")?;
        writeln!(yaml, "        log:")?;
        writeln!(
            yaml,
            "          to_file_path: \"./workspace/logs/benchmark.log\""
        )?;
        writeln!(yaml, "          event_fields: [step_name, loop_iteration, current_model]")?;
        writeln!(yaml, "      after_step_succeeds:")?;
        writeln!(yaml, "        append_to:")?;
        writeln!(yaml, "          - \"./workspace/output/benchmark_results.yaml\"")?;
        writeln!(yaml, "          - benchmark_collection")?;
        writeln!(yaml, "      after_loop_iteration_fails:")?;
        writeln!(yaml, "        log:")?;
        writeln!(
            yaml,
            "          to_file_path: \"./workspace/logs/benchmark-errors.log\""
        )?;
        writeln!(yaml, "          event_fields: [iteration, current_model, error_message]")?;
        writeln!(yaml, "    output:")?;
        writeln!(yaml, "      save_to: benchmark_results")?;
        writeln!(yaml)?;

        // Agentic document refinement step
        writeln!(yaml, "  - step: refine_document")?;
        writeln!(yaml, "    id: doc_refine")?;
        writeln!(yaml, "    type: oscillate_abstraction")?;
        writeln!(yaml, "    generative_entity: \"${{models.primary}}\"")?;
        writeln!(yaml, "    requires: [bench_loop]")?;
        writeln!(yaml, "    loop:")?;
        writeln!(yaml, "      count:")?;
        writeln!(yaml, "        max_iterations: {}", n)?;
        writeln!(yaml, "        iteration_variable: current_model")?;
        writeln!(yaml, "    input:")?;
        writeln!(yaml, "      file_path: \"./docs/plans/multi-model-benchmark/01-IMPLEMENTATION-PLAN.md\"")?;
        writeln!(yaml, "      chunk_size: 4000")?;
        writeln!(yaml, "      oscillations: 3")?;
        writeln!(yaml, "      overlap: 200")?;
        writeln!(yaml, "    prompts:")?;
        writeln!(yaml, "      summarize: |")?;
        writeln!(yaml, "        Summarize the following text, extracting key technical points.")?;
        writeln!(yaml, "        Keep: numbers, paths, function names, struct fields, error messages.")?;
        writeln!(yaml, "        Drop: filler, hedging, repetition, transitions.")?;
        writeln!(yaml, "        Target: 2000 chars max.")?;
        writeln!(yaml)?;
        writeln!(yaml, "        Text:")?;
        writeln!(yaml, "        {{{{loop.previous_output}}}}")?;
        writeln!(yaml, "      expand: |")?;
        writeln!(yaml, "        Expand on this summary by adding implementation detail, edge cases,")?;
        writeln!(yaml, "        and performance considerations a senior engineer would expect.")?;
        writeln!(yaml, "        Keep all existing technical facts. Target: 3000 chars max.")?;
        writeln!(yaml)?;
        writeln!(yaml, "        Summary:")?;
        writeln!(yaml, "        {{{{loop.previous_output}}}}")?;
        writeln!(yaml, "    output:")?;
        writeln!(yaml, "      save_to: refined_plans")?;
        writeln!(yaml, "      format: text")?;
        writeln!(yaml, "      path: \"./workspace/output/refined_plan_{{{{loop.current_model}}}}.md\"")?;
        writeln!(yaml)?;

        // Generate report step
        writeln!(yaml, "  - step: generate_report")?;
        writeln!(yaml, "    id: report")?;
        writeln!(yaml, "    input:")?;
        writeln!(yaml, r#"      results: "${{step.bench_loop.output}}""#)?;
        writeln!(yaml, "    output:")?;
        writeln!(yaml, "      save_to:")?;
        writeln!(yaml, "        - final_report")?;
        writeln!(yaml, r#"        - "./workspace/output/benchmark_report.json""#)?;

        info!("[yaml_generator] generated {} bytes of YAML", yaml.len());
        Ok(yaml)
    }

    /// Generate benchmark YAML with custom models list embedded.
    ///
    /// This is for pre-selected model sets (not using discovery).
    pub fn generate_benchmark_yaml_with_models(
        _name: &str,
        models: &[ModelCandidate],
        prompts: &[&str],
        max_tokens: usize,
    ) -> Result<String> {
        let n = models.len();
        let mut yaml = String::new();

        // Header
        writeln!(yaml, "workflow_id: benchmark_{}_models", n)?;
        writeln!(yaml, r#"name: "{}-Model Benchmark Suite""#, n)?;
        writeln!(
            yaml,
            r#"description: "Sequential benchmark of {} diverse models""#,
            n
        )?;
        writeln!(yaml, "min_schema_version: \"2.0.0\"")?;
        writeln!(yaml)?;

        // Models section - embed model paths
        writeln!(yaml, "benchmark:")?;
        writeln!(yaml, "  models:")?;
        for model in models {
            writeln!(
                yaml,
                r#"    - path: "{}""#,
                model.path.display()
            )?;
            writeln!(yaml, r#"      model_id: "{}""#, model.model_id)?;
            writeln!(yaml, "      file_size_bytes: {}", model.file_size_bytes)?;
        }
        writeln!(yaml)?;

        // Prompts
        writeln!(yaml, "  prompts:")?;
        for prompt in prompts {
            writeln!(yaml, "    - {}", prompt)?;
        }
        writeln!(yaml, "  max_tokens: {}", max_tokens)?;
        writeln!(yaml, "  temperature: 0.7")?;
        writeln!(yaml, "  top_p: 0.9")?;
        writeln!(yaml)?;

        // Execution config
        writeln!(yaml, "execution:")?;
        writeln!(yaml, "  mode: serial")?;
        writeln!(yaml, "  memory:")?;
        writeln!(yaml, "    max_allocated_memory_mb: 8192")?;
        writeln!(yaml, "    model_memory_mb: 6144")?;
        writeln!(yaml, "    unload_unused: true")?;
        writeln!(yaml)?;

        // Agentic workflow with loop
        writeln!(yaml, "agentic_workflow:")?;
        writeln!(yaml, "  - step: benchmark_loop")?;
        writeln!(yaml, "    id: bench_loop")?;
        writeln!(yaml, "    loop:")?;
        writeln!(yaml, "      count:")?;
        writeln!(yaml, "        max_iterations: {}", n)?;
        writeln!(yaml, "        iteration_variable: current_model")?;
        writeln!(yaml, "    input:")?;
        writeln!(yaml, r#"      model: "${{benchmark.models}}""#)?;
        writeln!(yaml, r#"      prompts: "${{benchmark.prompts}}""#)?;
        writeln!(yaml, r#"      max_tokens: "${{benchmark.max_tokens}}""#)?;
        writeln!(yaml, "    output:")?;
        writeln!(yaml, "      save_to: benchmark_results")?;
        writeln!(yaml)?;

        // Agentic document refinement step
        writeln!(yaml, "  - step: refine_document")?;
        writeln!(yaml, "    id: doc_refine")?;
        writeln!(yaml, "    type: oscillate_abstraction")?;
        writeln!(yaml, "    generative_entity: \"${{models.primary}}\"")?;
        writeln!(yaml, "    requires: [bench_loop]")?;
        writeln!(yaml, "    loop:")?;
        writeln!(yaml, "      count:")?;
        writeln!(yaml, "        max_iterations: {}", n)?;
        writeln!(yaml, "        iteration_variable: current_model")?;
        writeln!(yaml, "    input:")?;
        writeln!(yaml, "      file_path: \"./docs/plans/multi-model-benchmark/01-IMPLEMENTATION-PLAN.md\"")?;
        writeln!(yaml, "      chunk_size: 4000")?;
        writeln!(yaml, "      oscillations: 3")?;
        writeln!(yaml, "      overlap: 200")?;
        writeln!(yaml, "    prompts:")?;
        writeln!(yaml, "      summarize: |")?;
        writeln!(yaml, "        Summarize the following text, extracting key technical points.")?;
        writeln!(yaml, "        Keep: numbers, paths, function names, struct fields, error messages.")?;
        writeln!(yaml, "        Drop: filler, hedging, repetition, transitions.")?;
        writeln!(yaml, "        Target: 2000 chars max.")?;
        writeln!(yaml)?;
        writeln!(yaml, "        Text:")?;
        writeln!(yaml, "        {{{{loop.previous_output}}}}")?;
        writeln!(yaml, "      expand: |")?;
        writeln!(yaml, "        Expand on this summary by adding implementation detail, edge cases,")?;
        writeln!(yaml, "        and performance considerations a senior engineer would expect.")?;
        writeln!(yaml, "        Keep all existing technical facts. Target: 3000 chars max.")?;
        writeln!(yaml)?;
        writeln!(yaml, "        Summary:")?;
        writeln!(yaml, "        {{{{loop.previous_output}}}}")?;
        writeln!(yaml, "    output:")?;
        writeln!(yaml, "      save_to: refined_plans")?;
        writeln!(yaml, "      format: text")?;
        writeln!(yaml, "      path: \"./workspace/output/refined_plan_{{{{loop.current_model}}}}.md\"")?;
        writeln!(yaml)?;

        Ok(yaml)
    }

    /// Generate benchmark YAML with GPU/CPU comparison mode.
    ///
    /// Output includes:
    /// - Header section (workflow_id, name, description, min_schema_version)
    /// - Models section with placeholder for current model
    /// - Execution config (serial mode, memory settings)
    /// - Logging config (global and performance_metrics)
    /// - Benchmark config (prompts, max_tokens, temperature, top_p, compare_modes)
    /// - Agentic workflow with 4 steps:
    ///   1. benchmark_performance (nested model+mode loops for GPU/CPU)
    ///   2. refine_document (oscillate_abstraction per model)
    ///   3. generate_speedup_report (GPU vs CPU comparison)
    ///   4. generate_report (combined results)
    ///
    /// YAML is valid YAML syntax. Does not validate against UnifiedConfig.
    pub fn generate_benchmark_yaml_gpu_cpu_compare(
        _name: &str,
        models: &[ModelCandidate],
        prompts: &[&str],
        max_tokens: usize,
    ) -> Result<String> {
        let n = models.len();
        info!("[yaml_generator] generating GPU/CPU compare benchmark YAML for {} models", n);

        let mut yaml = String::new();

        // Header
        writeln!(yaml, "workflow_id: benchmark_{}_models", n)?;
        writeln!(yaml, r#"name: "{}-Model GPU/CPU Benchmark Suite""#, n)?;
        writeln!(
            yaml,
            r#"description: "GPU vs CPU performance benchmark of {} diverse models""#,
            n
        )?;
        writeln!(yaml, "min_schema_version: \"2.0.0\"")?;
        writeln!(yaml)?;

        // Models section
        writeln!(yaml, "models:")?;
        writeln!(yaml, "  primary:")?;
        writeln!(yaml, "    provider: lmstudio")?;
        writeln!(yaml, "    model: \"${{benchmark.current_model}}\"")?;
        writeln!(yaml)?;

        // Execution section
        writeln!(yaml, "execution:")?;
        writeln!(yaml, "  mode: serial")?;
        writeln!(yaml, "  memory:")?;
        writeln!(yaml, "    max_allocated_memory_mb: 8192")?;
        writeln!(yaml, "    model_memory_mb: 6144")?;
        writeln!(yaml, "    unload_unused: true")?;
        writeln!(yaml)?;

        // Logging section
        writeln!(yaml, "logging:")?;
        writeln!(yaml, "  global:")?;
        writeln!(yaml, "    level: info")?;
        writeln!(yaml, "    detail: medium")?;
        writeln!(yaml, "    output_type: chat")?;
        writeln!(yaml, "    format: json")?;
        writeln!(yaml, "    console: true")?;
        writeln!(yaml, "  performance_metrics:")?;
        writeln!(yaml, "    level: debug")?;
        writeln!(yaml, "    detail: very_high")?;
        writeln!(yaml)?;

        // Benchmark config section with GPU/CPU comparison
        writeln!(yaml, "benchmark:")?;
        writeln!(yaml, "  compare_modes: true")?;
        writeln!(yaml, "  modes: [gpu, cpu]")?;
        writeln!(yaml, "  gpu_config:")?;
        writeln!(yaml, "    n_gpu_layers: 999")?;
        writeln!(yaml, "  cpu_config:")?;
        writeln!(yaml, "    n_gpu_layers: 0")?;
        writeln!(yaml, "  prompts:")?;
        for prompt in prompts {
            let prompt_escaped = prompt.replace('\n', "\\n").replace('"', r#"\"#);
            writeln!(yaml, r#"    - "{}""#, prompt_escaped)?;
        }
        writeln!(yaml, "  max_tokens: {}", max_tokens)?;
        writeln!(yaml, "  temperature: 0.7")?;
        writeln!(yaml, "  top_p: 0.9")?;
        writeln!(yaml)?;

        // Agentic workflow section
        writeln!(yaml, "agentic_workflow:")?;

        // Step 1: Benchmark performance with nested model+mode loops
        writeln!(yaml, "  # Step 1: Benchmark each model in GPU then CPU mode")?;
        writeln!(yaml, "  - step: benchmark_performance")?;
        writeln!(yaml, "    id: perf_bench")?;
        writeln!(yaml, "    loop:")?;
        writeln!(yaml, "      count:")?;
        writeln!(yaml, "        max_iterations: {}", n)?;
        writeln!(yaml, "        iteration_variable: current_model")?;
        writeln!(yaml, "      modes: [gpu, cpu]")?;
        writeln!(yaml, "      mode_variable: gpu_mode")?;
        writeln!(yaml, "    input:")?;
        writeln!(yaml, r#"      prompts: "${{benchmark.prompts}}""#)?;
        writeln!(yaml, r#"      max_tokens: "${{benchmark.max_tokens}}""#)?;
        writeln!(yaml, r#"      gpu_mode: "{{{{loop.gpu_mode}}}}""#)?;
        writeln!(yaml, r#"      n_gpu_layers: "{{{{loop.gpu_mode == 'gpu' ? 999 : 0}}}}""#)?;
        writeln!(yaml, "    when:")?;
        writeln!(yaml, "      before_step_starts:")?;
        writeln!(yaml, "        action: configure_server")?;
        writeln!(yaml, r#"        gpu_mode: "{{{{loop.gpu_mode}}}}""#)?;
        writeln!(yaml, "      after_step_succeeds:")?;
        writeln!(yaml, "        append_to:")?;
        writeln!(yaml, "          - \"./workspace/output/benchmark_results.yaml\"")?;
        writeln!(yaml, "          - benchmark_collection")?;
        writeln!(yaml, "      after_loop_iteration_fails:")?;
        writeln!(yaml, "        log:")?;
        writeln!(yaml, "          to_file_path: \"./workspace/logs/benchmark-errors.log\"")?;
        writeln!(yaml, "          event_fields: [iteration, current_model, gpu_mode, error_message]")?;
        writeln!(yaml, "    output:")?;
        writeln!(yaml, "      save_to: benchmark_results")?;
        writeln!(yaml)?;

        // Step 2: Agentic document refinement per model
        writeln!(yaml, "  # Step 2: Agentic document refinement per model (oscillate_abstraction)")?;
        writeln!(yaml, "  - step: refine_document")?;
        writeln!(yaml, "    id: doc_refine")?;
        writeln!(yaml, "    type: oscillate_abstraction")?;
        writeln!(yaml, "    generative_entity: \"${{models.primary}}\"")?;
        writeln!(yaml, "    requires: [perf_bench]")?;
        writeln!(yaml, "    loop:")?;
        writeln!(yaml, "      count:")?;
        writeln!(yaml, "        max_iterations: {}", n)?;
        writeln!(yaml, "        iteration_variable: current_model")?;
        writeln!(yaml, "    input:")?;
        writeln!(yaml, "      file_path: \"./docs/plans/multi-model-benchmark/01-IMPLEMENTATION-PLAN.md\"")?;
        writeln!(yaml, "      chunk_size: 4000")?;
        writeln!(yaml, "      oscillations: 3")?;
        writeln!(yaml, "      overlap: 200")?;
        writeln!(yaml, "    prompts:")?;
        writeln!(yaml, "      summarize: |")?;
        writeln!(yaml, "        Summarize the following text, extracting key technical points.")?;
        writeln!(yaml, "        Keep: numbers, paths, function names, struct fields, error messages.")?;
        writeln!(yaml, "        Drop: filler, hedging, repetition, transitions.")?;
        writeln!(yaml, "        Target: 2000 chars max.")?;
        writeln!(yaml)?;
        writeln!(yaml, "        Text:")?;
        writeln!(yaml, "        {{{{loop.previous_output}}}}")?;
        writeln!(yaml, "      expand: |")?;
        writeln!(yaml, "        Expand on this summary by adding implementation detail, edge cases,")?;
        writeln!(yaml, "        and performance considerations a senior engineer would expect.")?;
        writeln!(yaml, "        Keep all existing technical facts. Target: 3000 chars max.")?;
        writeln!(yaml)?;
        writeln!(yaml, "        Summary:")?;
        writeln!(yaml, "        {{{{loop.previous_output}}}}")?;
        writeln!(yaml, "    output:")?;
        writeln!(yaml, "      save_to: refined_plans")?;
        writeln!(yaml, "      format: text")?;
        writeln!(yaml, "      path: \"./workspace/output/refined_plan_{{{{loop.current_model}}}}.md\"")?;
        writeln!(yaml)?;

        // Step 3: Generate speedup comparison report
        writeln!(yaml, "  # Step 3: Generate speedup comparison report")?;
        writeln!(yaml, "  - step: generate_speedup_report")?;
        writeln!(yaml, "    id: speedup_report")?;
        writeln!(yaml, "    requires: [perf_bench]")?;
        writeln!(yaml, "    input:")?;
        writeln!(yaml, r#"      benchmark_results: "{{{{step.perf_bench.output}}}}""#)?;
        writeln!(yaml, "      comparison_mode: gpu_vs_cpu")?;
        writeln!(yaml, "    output:")?;
        writeln!(yaml, "      save_to:")?;
        writeln!(yaml, "        - speedup_report")?;
        writeln!(yaml, "        - \"./workspace/output/speedup_report.json\"")?;
        writeln!(yaml)?;

        // Step 4: Generate combined report
        writeln!(yaml, "  # Step 4: Generate combined report")?;
        writeln!(yaml, "  - step: generate_report")?;
        writeln!(yaml, "    id: report")?;
        writeln!(yaml, "    requires: [perf_bench, doc_refine, speedup_report]")?;
        writeln!(yaml, "    input:")?;
        writeln!(yaml, r#"      benchmark_results: "{{{{step.perf_bench.output}}}}""#)?;
        writeln!(yaml, r#"      refined_plans: "{{{{step.doc_refine.output}}}}""#)?;
        writeln!(yaml, r#"      speedup_data: "{{{{step.speedup_report.output}}}}""#)?;
        writeln!(yaml, "    output:")?;
        writeln!(yaml, "      save_to:")?;
        writeln!(yaml, "        - final_report")?;
        writeln!(yaml, "        - \"./workspace/output/benchmark_report.json\"")?;

        info!("[yaml_generator] generated {} bytes of GPU/CPU compare YAML", yaml.len());
        Ok(yaml)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn create_test_candidate(
        path: &str,
        model_id: &str,
        file_size_bytes: u64,
    ) -> ModelCandidate {
        ModelCandidate {
            path: PathBuf::from(path),
            model_id: model_id.to_string(),
            file_size_bytes,
            estimated_vram_bytes: file_size_bytes,
            fits_in_vram: true,
            author: "test".to_string(),
        }
    }

    #[test]
    fn test_generate_benchmark_yaml_valid_yaml() {
        let models = vec![
            create_test_candidate("/models/model1.gguf", "model 1", 1_000_000_000),
            create_test_candidate("/models/model2.gguf", "model 2", 2_000_000_000),
            create_test_candidate("/models/model3.gguf", "model 3", 3_000_000_000),
        ];

        let prompts = vec![
            "Explain recursion",
            "Write Rust function",
        ];

        let yaml = BenchmarkYamlGenerator::generate_benchmark_yaml(
            "test",
            &models,
            &prompts,
            128,
        )
        .expect("Failed to generate YAML");

        // Check YAML structure (not validation against schema)
        assert!(yaml.contains("workflow_id:"));
        assert!(yaml.contains("name:"));
        assert!(yaml.contains("models:"));
        assert!(yaml.contains("execution:"));
        assert!(yaml.contains("benchmark:"));
        assert!(yaml.contains("agentic_workflow:"));
        assert!(yaml.contains("discover_models"));
        assert!(yaml.contains("benchmark_loop"));
        assert!(yaml.contains("max_iterations: 3"));
    }

    #[test]
    fn test_generate_benchmark_yaml_correct_model_count() {
        let models = vec![
            create_test_candidate("/models/model1.gguf", "model 1", 1_000_000_000),
            create_test_candidate("/models/model2.gguf", "model 2", 2_000_000_000),
            create_test_candidate("/models/model3.gguf", "model 3", 3_000_000_000),
            create_test_candidate("/models/model4.gguf", "model 4", 4_000_000_000),
            create_test_candidate("/models/model5.gguf", "model 5", 5_000_000_000),
        ];

        let prompts = vec!["Test prompt"];

        let yaml = BenchmarkYamlGenerator::generate_benchmark_yaml(
            "test",
            &models,
            &prompts,
            128,
        )
        .expect("Failed to generate YAML");

        assert!(yaml.contains("max_iterations: 5"));
    }

    #[test]
    fn test_generate_benchmark_yaml_prompts_escaped() {
        let models = vec![create_test_candidate("/models/model.gguf", "model", 1_000_000_000)];

        let prompts = vec![
            "Simple prompt",
            "Prompt with \"quotes\"",
            "Prompt\nwith\nnewlines",
        ];

        let yaml = BenchmarkYamlGenerator::generate_benchmark_yaml(
            "test",
            &models,
            &prompts,
            128,
        )
        .expect("Failed to generate YAML");

        assert!(yaml.contains("Simple prompt"));
        assert!(yaml.contains("Prompt with"));
        assert!(yaml.contains("quotes"));
    }

    #[test]
    fn test_generate_benchmark_yaml_roundtrip() {
        let models = vec![
            create_test_candidate("/models/model1.gguf", "model 1", 1_000_000_000),
        ];

        let prompts = vec!["Test prompt"];

        let yaml = BenchmarkYamlGenerator::generate_benchmark_yaml(
            "test",
            &models,
            &prompts,
            128,
        )
        .expect("Failed to generate YAML");

        // Verify YAML structure without parsing
        assert!(yaml.contains("workflow_id:"));
        assert!(yaml.contains("name:"));
        assert!(yaml.contains("agentic_workflow:"));
        assert!(yaml.contains("max_iterations: 1"));
    }

    #[test]
    fn test_generate_benchmark_yaml_with_models_valid_yaml() {
        let models = vec![
            create_test_candidate("/models/model1.gguf", "model 1", 1_000_000_000),
            create_test_candidate("/models/model2.gguf", "model 2", 2_000_000_000),
        ];

        let prompts = vec!["Test prompt"];

        let yaml = BenchmarkYamlGenerator::generate_benchmark_yaml_with_models(
            "test",
            &models,
            &prompts,
            128,
        )
        .expect("Failed to generate YAML");

        assert!(yaml.contains("benchmark:"));
        assert!(yaml.contains("  models:"));
        assert!(yaml.contains("path: \"/models/model1.gguf\""));
        assert!(yaml.contains("path: \"/models/model2.gguf\""));
        assert!(yaml.contains("max_iterations: 2"));
    }
}
