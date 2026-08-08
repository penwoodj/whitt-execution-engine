use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::collections::HashMap;
use whitt_execution_engine::error::Error;
use whitt_execution_engine::model::{
    interpolation::TemplateInterpolator,
    registry::{ModelRegistry, ModelLifecycle},
    resource::ResourceManager,
    schema::{ModelHost, ModelSpec, ModelsConfig},
};

// ============================================================================
// Resource Allocation Benchmarks
// ============================================================================

fn bench_resource_manager_new(c: &mut Criterion) {
    c.bench_function("resource_manager_new", |b| {
        b.iter(|| {
            let _ = ResourceManager::new(black_box(16000), black_box(24000));
        });
    });
}

fn bench_can_allocate(c: &mut Criterion) {
    let manager = ResourceManager::new(16000, 24000);
    let sizes = [100, 1024, 4096, 8192, 16384]; // MB

    let mut group = c.benchmark_group("can_allocate");
    for size in sizes {
        group.throughput(Throughput::Bytes((size * 1024 * 1024) as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| manager.can_allocate(black_box(size), black_box(size)));
        });
    }
    group.finish();
}

fn bench_allocate(c: &mut Criterion) {
    let sizes = [100, 1024, 4096, 8192, 16384]; // MB

    let mut group = c.benchmark_group("allocate");
    for size in sizes {
        group.throughput(Throughput::Bytes((size * 1024 * 1024) as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut manager = ResourceManager::new(16384, 32768);
            b.iter(|| {
                let _ = manager.allocate(black_box("model"), black_box(size), black_box(size / 2));
            });
        });
    }
    group.finish();
}

fn bench_deallocate(c: &mut Criterion) {
    let sizes = [100, 1024, 4096, 8192, 16384]; // MB

    let mut group = c.benchmark_group("deallocate");
    for size in sizes {
        group.throughput(Throughput::Bytes((size * 1024 * 1024) as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut manager = ResourceManager::new(16384, 32768);
            manager.allocate("model", size, size / 2).unwrap();
            b.iter(|| {
                manager.deallocate(black_box("model"), black_box(size), black_box(size / 2));
            });
        });
    }
    group.finish();
}

fn bench_allocation_cycle(c: &mut Criterion) {
    let mut group = c.benchmark_group("allocation_cycle");

    for cycle_count in [1, 10, 100, 1000] {
        group.bench_with_input(BenchmarkId::from_parameter(cycle_count), &cycle_count, |b, &cycle_count| {
            b.iter(|| {
                let mut manager = ResourceManager::new(32768, 65536);
                for i in 0..cycle_count {
                    let name = format!("model_{}", i % 10); // Recycle 10 model names
                    if manager.can_allocate(512, 256) {
                        let _ = manager.allocate(&name, 512, 256);
                        if i > 0 && i % 5 == 0 {
                            manager.deallocate(&name, 512, 256);
                        }
                    }
                }
            });
        });
    }
    group.finish();
}

// ============================================================================
// Template Interpolation Benchmarks
// ============================================================================

fn bench_interpolator_new(c: &mut Criterion) {
    c.bench_function("interpolator_new", |b| {
        b.iter(|| {
            let _ = TemplateInterpolator::new();
        });
    });
}

fn bench_is_structural_reference(c: &mut Criterion) {
    let mut group = c.benchmark_group("is_structural_reference");
    group.bench_function("no_reference", |b| {
        b.iter(|| {
            TemplateInterpolator::is_structural_reference(black_box("plain text without references"));
        });
    });

    group.bench_function("single_reference", |b| {
        b.iter(|| {
            TemplateInterpolator::is_structural_reference(black_box("${models.primary}"));
        });
    });

    group.bench_function("multiple_references", |b| {
        b.iter(|| {
            TemplateInterpolator::is_structural_reference(black_box(
                "Use ${models.primary} with ${workspace.path} and ${workflow.timeout}",
            ));
        });
    });

    group.finish();
}

fn bench_resolve_structural(c: &mut Criterion) {
    let interpolator = TemplateInterpolator::new();
    let mut models = HashMap::new();
    models.insert("primary".to_string(), "llama-3.2-3b".to_string());
    models.insert("secondary".to_string(), "llama3.2".to_string());
    models.insert("validator".to_string(), "mistral-7b".to_string());

    let mut group = c.benchmark_group("resolve_structural");
    group.bench_function("no_placeholder", |b| {
        b.iter(|| {
            interpolator.resolve_structural(black_box("plain text"), black_box(&models)).unwrap();
        });
    });

    group.bench_function("one_placeholder", |b| {
        b.iter(|| {
            interpolator
                .resolve_structural(black_box("${models.primary}"), black_box(&models))
                .unwrap();
        });
    });

    group.bench_function("five_placeholders", |b| {
        b.iter(|| {
            interpolator
                .resolve_structural(
                    black_box("${models.primary} ${models.secondary} ${models.primary} ${models.secondary} ${models.validator}"),
                    black_box(&models)
                )
                .unwrap();
        });
    });

    group.bench_function("ten_placeholders", |b| {
        b.iter(|| {
            interpolator
                .resolve_structural(
                    black_box("${models.primary} ${models.secondary} ${models.primary} ${models.secondary} ${models.validator} ${models.primary} ${models.secondary} ${models.primary} ${models.secondary} ${models.validator}"),
                    black_box(&models)
                )
                .unwrap();
        });
    });

    group.finish();
}

fn bench_resolve_runtime(c: &mut Criterion) {
    let interpolator = TemplateInterpolator::new();
    let mut context = HashMap::new();
    context.insert("name".to_string(), "value".to_string());
    context.insert("step".to_string(), "analyze".to_string());
    context.insert("output".to_string(), "result".to_string());
    context.insert("iteration".to_string(), "5".to_string());
    context.insert("status".to_string(), "success".to_string());

    let mut group = c.benchmark_group("resolve_runtime");
    group.bench_function("no_jinja", |b| {
        b.iter(|| {
            interpolator.resolve_runtime(black_box("plain text"), black_box(&context)).unwrap();
        });
    });

    group.bench_function("one_variable", |b| {
        b.iter(|| {
            interpolator.resolve_runtime(black_box("{{name}}"), black_box(&context)).unwrap();
        });
    });

    group.bench_function("five_variables", |b| {
        b.iter(|| {
            interpolator
                .resolve_runtime(
                    black_box("{{name}} {{step}} {{output}} {{iteration}} {{status}}"),
                    black_box(&context)
                )
                .unwrap();
        });
    });

    group.bench_function("ten_variables", |b| {
        b.iter(|| {
            interpolator
                .resolve_runtime(
                    black_box("{{name}} {{step}} {{output}} {{iteration}} {{status}} {{name}} {{step}} {{output}} {{iteration}} {{status}}"),
                    black_box(&context)
                )
                .unwrap();
        });
    });

    group.finish();
}

// ============================================================================
// Error Construction Benchmarks
// ============================================================================

fn bench_error_yaml_parse(c: &mut Criterion) {
    c.bench_function("error_yaml_parse", |b| {
        b.iter(|| {
            let yaml: Result<(), _> = serde_saphyr::from_str("invalid: yaml: [unclosed");
            let err = if let Err(e) = yaml {
                Error::YamlParse(e)
            } else {
                unreachable!()
            };
            let _ = format!("{}", err);
        });
    });
}

fn bench_error_validation(c: &mut Criterion) {
    c.bench_function("error_validation", |b| {
        b.iter(|| {
            let err = Error::validation(black_box("validation failed"));
            let _ = format!("{}", err);
        });
    });
}

fn bench_error_model(c: &mut Criterion) {
    c.bench_function("error_model", |b| {
        b.iter(|| {
            let err = Error::model(black_box("model error occurred"));
            let _ = format!("{}", err);
        });
    });
}

fn bench_error_model_not_found(c: &mut Criterion) {
    c.bench_function("error_model_not_found", |b| {
        b.iter(|| {
            let err = Error::model_not_found(black_box("unknown-model"));
            let _ = format!("{}", err);
        });
    });
}

fn bench_error_model_load_failed(c: &mut Criterion) {
    c.bench_function("error_model_load_failed", |b| {
        b.iter(|| {
            let err = Error::model_load_failed(black_box("test-model"), black_box("out of memory"));
            let _ = format!("{}", err);
        });
    });
}

fn bench_error_config(c: &mut Criterion) {
    c.bench_function("error_config", |b| {
        b.iter(|| {
            let err = Error::config(black_box("config error"));
            let _ = format!("{}", err);
        });
    });
}

fn bench_error_agent_definition(c: &mut Criterion) {
    c.bench_function("error_agent_definition", |b| {
        b.iter(|| {
            let err = Error::agent_definition(black_box("agent definition error"));
            let _ = format!("{}", err);
        });
    });
}

fn bench_error_tool_definition(c: &mut Criterion) {
    c.bench_function("error_tool_definition", |b| {
        b.iter(|| {
            let err = Error::tool_definition(black_box("tool definition error"));
            let _ = format!("{}", err);
        });
    });
}

fn bench_error_workflow_definition(c: &mut Criterion) {
    c.bench_function("error_workflow_definition", |b| {
        b.iter(|| {
            let err = Error::workflow_definition(black_box("workflow definition error"));
            let _ = format!("{}", err);
        });
    });
}

fn bench_error_missing_field(c: &mut Criterion) {
    c.bench_function("error_missing_field", |b| {
        b.iter(|| {
            let err = Error::missing_field(black_box("required_field"));
            let _ = format!("{}", err);
        });
    });
}

fn bench_error_invalid_value(c: &mut Criterion) {
    c.bench_function("error_invalid_value", |b| {
        b.iter(|| {
            let err = Error::invalid_value(black_box("field_name"), black_box("value out of range"));
            let _ = format!("{}", err);
        });
    });
}

fn bench_error_cli(c: &mut Criterion) {
    c.bench_function("error_cli", |b| {
        b.iter(|| {
            let err = Error::cli(black_box("CLI error"));
            let _ = format!("{}", err);
        });
    });
}

fn bench_error_execution(c: &mut Criterion) {
    c.bench_function("error_execution", |b| {
        b.iter(|| {
            let err = Error::execution(black_box("execution failed"));
            let _ = format!("{}", err);
        });
    });
}

fn bench_error_benchmark(c: &mut Criterion) {
    c.bench_function("error_benchmark", |b| {
        b.iter(|| {
            let err = Error::benchmark(black_box("benchmark error"));
            let _ = format!("{}", err);
        });
    });
}

fn bench_error_metrics_collection(c: &mut Criterion) {
    c.bench_function("error_metrics_collection", |b| {
        b.iter(|| {
            let err = Error::metrics_collection(black_box("collection failed"));
            let _ = format!("{}", err);
        });
    });
}

fn bench_error_memory(c: &mut Criterion) {
    c.bench_function("error_memory", |b| {
        b.iter(|| {
            let err = Error::memory(black_box("out of memory"));
            let _ = format!("{}", err);
        });
    });
}

fn bench_error_schedule(c: &mut Criterion) {
    c.bench_function("error_schedule", |b| {
        b.iter(|| {
            let err = Error::schedule(black_box("schedule error"));
            let _ = format!("{}", err);
        });
    });
}

fn bench_error_metric(c: &mut Criterion) {
    c.bench_function("error_metric", |b| {
        b.iter(|| {
            let err = Error::metric(black_box("metric error"));
            let _ = format!("{}", err);
        });
    });
}

// ============================================================================
// Model Registry Benchmarks
// ============================================================================

fn create_test_models_config(count: usize) -> ModelsConfig {
    let mut models = HashMap::new();
    for i in 0..count {
        models.insert(
            format!("model-{}", i),
            ModelSpec {
                name: format!("Model {}", i),
                host: ModelHost {
                    r#type: "llama_cpp_with_vulkan".to_string(),
                    connection_settings: HashMap::new(),
                },
                ram_allocation: Default::default(),
                max_allowed: Default::default(),
                min_allowed: Default::default(),
                model_memory: Default::default(),
                execution: Default::default(),
                thinking: Default::default(),
                tools: Default::default(),
                load_params: Default::default(),
                sampling: Default::default(),
                guardrails: Default::default(),
            },
        );
    }
    ModelsConfig {
        global_config_path: "./configs/models".to_string(),
        default_router: "automatic".to_string(),
        models,
    }
}

fn bench_registry_new(c: &mut Criterion) {
    let model_counts = [1, 5, 10, 20, 50];

    let mut group = c.benchmark_group("registry_new");
    for count in model_counts {
        group.throughput(Throughput::Elements(count as u64));
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            b.iter(|| {
                let config = create_test_models_config(black_box(count));
                let _ = ModelRegistry::new(black_box(config));
            });
        });
    }
    group.finish();
}

fn bench_registry_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("registry_get");

    for count in [1, 5, 10, 20, 50] {
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            let config = create_test_models_config(count);
            let registry = ModelRegistry::new(config);
            b.iter(|| {
                registry.get(black_box("model-0"));
            });
        });
    }
    group.finish();
}

fn bench_registry_set_state(c: &mut Criterion) {
    let states = [
        ModelLifecycle::Unloaded,
        ModelLifecycle::Loading,
        ModelLifecycle::Loaded,
        ModelLifecycle::Active,
        ModelLifecycle::Unloading,
    ];

    let mut group = c.benchmark_group("registry_set_state");
    for state in &states {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{:?}", state)),
            state,
            |b, state| {
                let config = create_test_models_config(10);
                let mut registry = ModelRegistry::new(config);
                b.iter(|| {
                    registry.set_state(black_box("model-0"), black_box(state.clone()));
                });
            },
        );
    }
    group.finish();
}

fn bench_registry_load_model(c: &mut Criterion) {
    let mut group = c.benchmark_group("registry_load_model");

    for count in [1, 5, 10, 20] {
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            let config = create_test_models_config(count);
            let mut registry = ModelRegistry::new(config);
            b.iter(|| {
                let _ = registry.load_model(black_box("model-0"));
                // Reset state for next iteration
                registry.set_state("model-0", ModelLifecycle::Unloaded);
            });
        });
    }
    group.finish();
}

fn bench_registry_unload_model(c: &mut Criterion) {
    let mut group = c.benchmark_group("registry_unload_model");

    for count in [1, 5, 10, 20] {
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            let config = create_test_models_config(count);
            let mut registry = ModelRegistry::new(config);
            registry.set_state("model-0", ModelLifecycle::Loaded);
            b.iter(|| {
                let _ = registry.unload_model(black_box("model-0"));
                // Reset state for next iteration
                registry.set_state("model-0", ModelLifecycle::Loaded);
            });
        });
    }
    group.finish();
}

fn bench_registry_resolve_reference(c: &mut Criterion) {
    let mut group = c.benchmark_group("registry_resolve_reference");

    for count in [1, 5, 10, 20] {
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            let config = create_test_models_config(count);
            let registry = ModelRegistry::new(config);
            b.iter(|| {
                registry.resolve_reference(black_box("${models.model-0}"));
            });
        });
    }
    group.finish();
}

fn bench_registry_state_transition(c: &mut Criterion) {
    let transitions = vec![
        (ModelLifecycle::Unloaded, ModelLifecycle::Loading),
        (ModelLifecycle::Loading, ModelLifecycle::Loaded),
        (ModelLifecycle::Loaded, ModelLifecycle::Active),
        (ModelLifecycle::Active, ModelLifecycle::Loaded),
        (ModelLifecycle::Loaded, ModelLifecycle::Unloading),
        (ModelLifecycle::Unloading, ModelLifecycle::Unloaded),
    ];

    let mut group = c.benchmark_group("registry_state_transition");
    for (from, to) in &transitions {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{:?} -> {:?}", from, to)),
            &(from.clone(), to.clone()),
            |b, (from, to)| {
                let config = create_test_models_config(10);
                let mut registry = ModelRegistry::new(config);
                registry.set_state("model-0", from.clone());
                b.iter(|| {
                    registry.set_state(black_box("model-0"), black_box(to.clone()));
                });
            },
        );
    }
    group.finish();
}

// ============================================================================
// Criterion Configuration
// ============================================================================

criterion_group!(
    benches,
    // Resource benchmarks
    bench_resource_manager_new,
    bench_can_allocate,
    bench_allocate,
    bench_deallocate,
    bench_allocation_cycle,
    // Interpolation benchmarks
    bench_interpolator_new,
    bench_is_structural_reference,
    bench_resolve_structural,
    bench_resolve_runtime,
    // Error benchmarks
    bench_error_yaml_parse,
    bench_error_validation,
    bench_error_model,
    bench_error_model_not_found,
    bench_error_model_load_failed,
    bench_error_config,
    bench_error_agent_definition,
    bench_error_tool_definition,
    bench_error_workflow_definition,
    bench_error_missing_field,
    bench_error_invalid_value,
    bench_error_cli,
    bench_error_execution,
    bench_error_benchmark,
    bench_error_metrics_collection,
    bench_error_memory,
    bench_error_schedule,
    bench_error_metric,
    // Registry benchmarks
    bench_registry_new,
    bench_registry_get,
    bench_registry_set_state,
    bench_registry_load_model,
    bench_registry_unload_model,
    bench_registry_resolve_reference,
    bench_registry_state_transition,
);

criterion_main!(benches);
