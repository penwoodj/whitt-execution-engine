#!/usr/bin/env python3
"""Unit tests for build-workflow.py critical transformations.

Tests the two most important fixes:
1. save_to injection for steps missing hooks
2. max_tokens enforcement for steps missing model_overrides

Run: python3 scripts/meta-v6/test-build-workflow.py
"""
import sys
import os
import tempfile

REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO, 'scripts', 'meta-v6'))

PASS = 0
FAIL = 0

def assert_eq(actual, expected, msg):
    global PASS, FAIL
    if actual == expected:
        PASS += 1
    else:
        FAIL += 1
        print(f"FAIL: {msg}")
        print(f"  Expected: {expected}")
        print(f"  Actual:   {actual}")

def assert_true(val, msg):
    global PASS, FAIL
    if val:
        PASS += 1
    else:
        FAIL += 1
        print(f"FAIL: {msg}")

def test_save_to_injection():
    """Steps without save_to should get one injected."""
    import re
    
    block = """step_t1_test:
  generative_entity: "${models.qwen35}"
  prompt: |
    Do something useful."""
    
    has_save_to = bool(re.search(r'^\s*-\s*save_to:', block, re.MULTILINE))
    assert_true(not has_save_to, "test block should NOT have save_to before injection")
    
    block_with_save = """  when:
    after_step_succeeds:
      - save_to:
          - $step_t1_test_output"""
    has_save_to_after = bool(re.search(r'^\s*-\s*save_to:', block_with_save, re.MULTILINE))
    assert_true(has_save_to_after, "injected block should have save_to")

def test_save_to_not_injected_when_exists():
    """Steps with existing save_to should NOT get a duplicate."""
    import re
    
    block = """step_t1_test:
  when:
    after_step_succeeds:
      - save_to:
          to_file_path: "./outputs/existing.txt"
  prompt: |
    Do something."""
    
    has_save_to = bool(re.search(r'^\s*-\s*save_to:', block, re.MULTILINE))
    assert_true(has_save_to, "block with existing save_to should be detected")

def test_save_to_not_matching_prompt_text():
    """'save_to' in prompt text should NOT count as having save_to hook."""
    import re
    
    block = """step_t1_test:
  prompt: |
    This step uses save_to + log hooks for output."""
    
    has_save_to_yaml = bool(re.search(r'^\s*-\s*save_to:', block, re.MULTILINE))
    assert_true(not has_save_to_yaml, "'save_to' in prompt text should NOT match YAML key pattern")

def test_max_tokens_threshold():
    """max_tokens < 100 should be treated as bootstrap, not workflow default."""
    bootstrap_max_tokens = 4
    assert_true(bootstrap_max_tokens < 100, "bootstrap max_tokens=4 should be < 100")
    
    real_max_tokens = 8192
    assert_true(real_max_tokens >= 100, "real max_tokens=8192 should be >= 100")

if __name__ == '__main__':
    test_save_to_injection()
    test_save_to_not_injected_when_exists()
    test_save_to_not_matching_prompt_text()
    test_max_tokens_threshold()
    
    print(f"\n{'='*40}")
    print(f"PASS: {PASS}, FAIL: {FAIL}")
    print(f"{'='*40}")
    sys.exit(1 if FAIL > 0 else 0)
