#!/usr/bin/env python3
"""Deterministic Rust repair: fix common LLM mistakes."""
import sys, re, subprocess, tempfile, os, shutil

def main():
    if len(sys.argv) < 3:
        print("usage: repair-rust.py <input.rs> <output.rs> [--check-crate <dir>]", file=sys.stderr)
        sys.exit(2)
    src_path = sys.argv[1]
    out_path = sys.argv[2]
    check_crate = None
    if "--check-crate" in sys.argv:
        i = sys.argv.index("--check-crate")
        check_crate = sys.argv[i+1]

    with open(src_path) as f:
        src = f.read()
    original = src

    # Repair 1: bail! used without import
    if "bail!" in src and "use anyhow::bail" not in src and "anyhow::bail!" not in src:
        def add_bail(m):
            inner = m.group(1)
            if "bail" in inner:
                return m.group(0)
            return "use anyhow::{bail, " + inner + "};"
        new_src = re.sub(r'use anyhow::\{([^}]+)\};', add_bail, src, count=1)
        if new_src == src:
            new_src = src.replace(
                "use anyhow::{Context, Result};",
                "use anyhow::{bail, Context, Result};"
            )
        src = new_src

    # Repair 2: `let stream = response.bytes_stream()` needs `mut`
    src = re.sub(
        r'(\s+let\s+)stream(\s*=\s*response\.bytes_stream\(\))',
        r'\1mut stream\2',
        src
    )

    # Repair 3: u64 % usize type mismatch → cast usize
    src = re.sub(
        r'(\b[a-z_][a-z0-9_]*)\s*%\s*(CHUNK_SIZE|chunk_size)\s*==\s*0',
        r'\1 % (\2 as u64) == 0',
        src
    )

    # Repair 4: format!() arg count mismatch (only fix if extra args clearly present)
    # Conservative: only handle simple case where extra trailing comma-arg exists
    # Skipped by default — requires per-case analysis. Enable only when needed.

    # Repair 5: unused PathBuf import — drop if not used elsewhere
    if "PathBuf" not in re.sub(r'^use\s+std::path::\{Path,\s*PathBuf\};', '', src, flags=re.MULTILINE):
        src = src.replace("use std::path::{Path, PathBuf};", "use std::path::Path;")

    changed = src != original

    with open(out_path, 'w') as f:
        f.write(src)

    if check_crate:
        # Copy to crate, run cargo check, restore
        target = os.path.join(check_crate, "src/client/model_download.rs")
        backup = target + ".bak"
        shutil.copy(target, backup)
        shutil.copy(out_path, target)
        try:
            result = subprocess.run(
                ["cargo", "check", "--release", "--features", "client"],
                cwd=check_crate, capture_output=True, text=True, timeout=120
            )
            errors = [l for l in result.stdout.split("\n") + result.stderr.split("\n") if l.startswith("error")]
            print(f"cargo check after repair: {len(errors)} errors", file=sys.stderr)
            for e in errors[:10]:
                print(f"  {e}", file=sys.stderr)
        finally:
            shutil.copy(backup, target)
            os.remove(backup)

    sys.exit(0 if changed else 1)

if __name__ == "__main__":
    main()
