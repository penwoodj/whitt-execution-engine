from pathlib import Path
import sys

import pytest


LIVE_SCRIPTS = Path(__file__).resolve().parents[1] / "scripts" / "live"
sys.path.insert(0, str(LIVE_SCRIPTS))

from lib_live import worker_prompt


@pytest.fixture
def f4_case() -> dict[str, object]:
    return {
        "task": {
            "prompt": "Analyze supplied evidence without inventing facts.",
            "expected_schema": {
                "required": [
                    "computed_stats",
                    "judgements",
                    "consistency_checks",
                    "verdict",
                ],
                "properties": {
                    "computed_stats": {"type": "string"},
                    "judgements": {"type": "string"},
                    "consistency_checks": {"type": "string"},
                    "verdict": {"type": "string"},
                },
                "optional": ["notes"],
            },
        },
    }


def test_worker_prompt_when_first_round_then_frontloads_scalar_json_contract(
    f4_case: dict[str, object], tmp_path: Path
) -> None:
    # Given: a long-form analysis case whose schema allows only top-level strings.
    # When: the first live worker prompt is generated.
    prompt = worker_prompt(f4_case, 1, tmp_path)

    # Then: output constraints appear before task prose and prohibit nested JSON.
    task_offset = prompt.index("Analyze supplied evidence")
    assert prompt.index("Every top-level value must be a JSON string") < task_offset
    assert "Do not emit nested objects, arrays, numbers, booleans, or null" in prompt
    assert "Keep all four string values concise enough to finish the response" in prompt
    assert "Start directly with { and end directly with }" in prompt
    assert "Use standard JSON double-quoted string values only" in prompt
    assert "Never use Markdown, code fences, triple quotes, or line breaks inside values" in prompt
    assert "Each value must be one concise single-line plain sentence" in prompt
    assert "Each string value must be concise plain prose, never JSON or escaped JSON" in prompt
    assert "Never place {, }, [, or ] inside any string value" in prompt


def test_worker_prompt_when_retry_then_frontloads_scalar_json_contract(
    f4_case: dict[str, object], tmp_path: Path
) -> None:
    # Given: same scalar-only case after a failed first attempt.
    # When: retry prompt is generated without a heal artifact.
    prompt = worker_prompt(f4_case, 2, tmp_path)

    # Then: scalar output constraints still precede task prose.
    assert prompt.index("Every top-level value must be a JSON string") < prompt.index(
        "Analyze supplied evidence"
    )
    output_anchor = prompt.index("OUTPUT ANCHORING")
    assert prompt.rindex("Start directly with { and end directly with }") > output_anchor
    assert (
        prompt.rindex("Never use Markdown, code fences, triple quotes, or line breaks")
        > output_anchor
    )
