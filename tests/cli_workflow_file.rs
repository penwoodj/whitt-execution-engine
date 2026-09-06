//! CLI regression: missing `--workflow` file must hard-error at the
//! argument-dispatch layer BEFORE any server contact or model discovery.
//!
//! Original failure mode (experiments/reasoning-enhancer-plus/docs/07-TRACKING.md:331-332):
//! "missing --workflow file silently falls back to discovery benchmark
//!  (flan-t5 loaded by accident) — should hard-error."
//! Bit v14c and the overcontext run; wasted GPU hours loading a random model.
//!
//! `--url http://127.0.0.1:9` (discard port, connection refused instantly)
//! guards the RED phase: without the fix, the binary would proceed to
//! preflight and fail with a network error that does NOT mention the
//! workflow file, failing the message assertion. With the fix, the CLI
//! hard-errors before any network I/O.

use std::process::Command;

#[test]
fn given_benchmark_with_missing_workflow_file_when_run_then_hard_error() {
    let exe = env!("CARGO_BIN_EXE_whitt");
    let missing = std::env::temp_dir().join("whitt-missing-workflow-definitely-absent.yml");
    let _ = std::fs::remove_file(&missing); // ensure absent

    let output = Command::new(exe)
        .args([
            "benchmark",
            "--url",
            "http://127.0.0.1:9",
            "--workflow",
            missing.to_str().unwrap(),
        ])
        .output()
        .expect("failed to spawn whitt binary");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{}{}", stdout, stderr);

    assert!(
        !output.status.success(),
        "missing --workflow file must exit non-zero, got {:?}. stdout={} stderr={}",
        output.status.code(),
        stdout,
        stderr
    );
    assert!(
        combined.contains("whitt-missing-workflow-definitely-absent.yml"),
        "error must name the missing workflow file (no silent discovery fallback). output={}",
        combined
    );
}
