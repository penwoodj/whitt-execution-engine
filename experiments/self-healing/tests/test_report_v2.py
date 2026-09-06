import importlib.util
from pathlib import Path

import yaml


EXPERIMENT_ROOT = Path(__file__).resolve().parents[1]
REPORTER_PATH = EXPERIMENT_ROOT / "scripts" / "report_v2.py"


def load_reporter() -> object:
    spec = importlib.util.spec_from_file_location("report_v2", REPORTER_PATH)
    assert spec is not None
    assert spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def test_suite_pairs_when_results_live_v2_then_discovers_root_cases(
    tmp_path: Path,
) -> None:
    # Given: a standard self-healing root with one matrix case and one run.
    case_path = tmp_path / "cases" / "v2" / "matrix" / "case-a.yml"
    case_path.parent.mkdir(parents=True)
    case_path.write_text(yaml.safe_dump({"case_id": "case-a"}))
    run_dir = tmp_path / "results" / "live-v2" / "case-a"
    run_dir.mkdir(parents=True)
    (run_dir / "outcome.json").write_text("{}")
    reporter = load_reporter()

    # When: suite pairs are collected from its results directory.
    pairs = reporter.suite_pairs(run_dir.parent)

    # Then: matching case YAML is found from experiment-root cases, not results.
    assert [(case["case_id"], directory) for case, directory in pairs] == [
        ("case-a", run_dir)
    ]
