import importlib.util
from pathlib import Path

import yaml


EXPERIMENT_ROOT = Path(__file__).resolve().parents[1]
GENERATOR_PATH = EXPERIMENT_ROOT / "scripts" / "gen-live-v2.py"
CASE_PATH = EXPERIMENT_ROOT / "cases" / "v2" / "matrix" / "px-ana-a-f4.yml"


def load_generator() -> object:
    spec = importlib.util.spec_from_file_location("gen_live_v2", GENERATOR_PATH)
    assert spec is not None
    assert spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def test_build_when_live_workflow_generated_then_declares_resource_contract() -> None:
    # Given: every live model has a stat-verified absolute GGUF source path.
    expected_paths = {
        "m_worker": "/run/media/jon/data/models/Qwen3-4B-Instruct-2507-Q4_K_M.gguf",
        "m_fast": "/run/media/jon/data/models/Ministral-3-3B-Instruct-2512-Q4_K_M.gguf",
        "m_coder": "/run/media/jon/data/models/Qwen2.5-Coder-3B-Instruct-Q8_0.gguf",
        "m_heavy": "/run/media/jon/data/models/Qwen3-8B-Q4_K_M.gguf",
        "m_judge": "/run/media/jon/data/models/Hermes-2-Pro-Mistral-7B.Q4_K_M.gguf",
    }
    generator = load_generator()

    # When: F4's real-LLM workflow is built.
    workflow = generator.build(CASE_PATH)

    # Then: every load is serial, bounded, traceable, and admission-gated.
    for alias, source_path in expected_paths.items():
        model = workflow["models"][alias]
        assert model["source_path"] == source_path
        assert model["name"] == Path(source_path).name
        assert model["load_params"] == {
            "context_size": 4096,
            "gpu_layers": 99,
            "cont_batching": False,
            "no_cache_prompt": True,
            "parallel": 1,
        }

    strategy = workflow["workflow_execution_strategy"]
    assert strategy["load_unload"] == "one_at_a_time"
    assert strategy["resource_admission"] == {
        "enforcement_policy": "block",
        "minimum_available": {"ram": "6GiB", "vram": "6GiB", "swap_free": "4GiB"},
        "model_estimate": {
            "kv_cache": "288MiB",
            "compute_buffer": "512MiB",
            "host_runtime": "700MiB",
            "expected_runtime_secs": 900,
        },
        "telemetry": {"write_profile": True},
    }


def test_build_when_live_workflow_generated_then_sources_are_router_visible() -> None:
    # Given: router model discovery scans only the configured models directory.
    registry_dir = Path("/run/media/jon/data/models")
    generator = load_generator()

    # When: the F4 real-LLM workflow is built.
    workflow = generator.build(CASE_PATH)

    # Then: every generated source is an immediate router-visible model file.
    assert all(
        Path(model["source_path"]).parent == registry_dir
        for model in workflow["models"].values()
    )


def test_render_when_resource_contract_emitted_then_cites_schema_lines() -> None:
    generator = load_generator()
    workflow = generator.build(CASE_PATH)

    rendered = generator.render_workflow(workflow)

    expected_lines = (
        "source_path: /run/media/jon/data/models/Qwen3-4B-Instruct-2507-Q4_K_M.gguf # schema line 911",
        "load_params: # schema line 75",
        "context_size: 4096 # schema line 76",
        "gpu_layers: 99 # schema line 81",
        "cont_batching: false # schema line 85",
        "no_cache_prompt: true # schema line 86",
        "parallel: 1 # schema line 87",
        "load_unload: one_at_a_time # schema line 534",
        "resource_admission: # schema line 562",
        "enforcement_policy: block # schema line 563",
        "minimum_available: # schema line 564",
        "ram: 6GiB # schema line 565",
        "vram: 6GiB # schema line 566",
        "swap_free: 4GiB # schema line 567",
        "model_estimate: # schema line 568",
        "kv_cache: 288MiB # schema line 569",
        "compute_buffer: 512MiB # schema line 570",
        "host_runtime: 700MiB # schema line 571",
        "expected_runtime_secs: 900 # schema line 572",
        "telemetry: # schema line 573",
        "write_profile: true # schema line 574",
    )

    assert all(line in rendered for line in expected_lines)
    assert yaml.safe_load(rendered) == workflow
