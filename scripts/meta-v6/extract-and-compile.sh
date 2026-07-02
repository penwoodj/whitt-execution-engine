#!/bin/bash
# scripts/meta-v6/extract-and-compile.sh
# Extract code blocks from deliverable files and attempt compilation.
#
# Scans all step_*.{txt,md,rs,ts,tsx,json} files in <exec-dir>/output for:
#   - Rust code blocks (```rust ... ```)
#   - TypeScript/TSX code blocks (```typescript, ```tsx)
# Extracts to /tmp, runs cargo check (for Rust) or tsc --noEmit (for TS).
# Captures errors + exit codes.
#
# Outputs JSON to stdout.
#
# Usage: extract-and-compile.sh <exec-dir> [scratch-dir]

set -uo pipefail

EXEC_DIR="${1:?usage: $0 <exec-dir> [scratch-dir]}"
SCRATCH="${2:-/tmp/compile-check-$$}"
mkdir -p "$SCRATCH"

OUTPUT_DIR="${EXEC_DIR}/output"
[ ! -d "$OUTPUT_DIR" ] && OUTPUT_DIR="${EXEC_DIR}/outputs/output"
[ ! -d "$OUTPUT_DIR" ] && OUTPUT_DIR="${EXEC_DIR}/logs"
[ ! -d "$OUTPUT_DIR" ] && OUTPUT_DIR="${EXEC_DIR}"

DELIVERABLES_DIR="${EXEC_DIR}/deliverables"
[ ! -d "$DELIVERABLES_DIR" ] && DELIVERABLES_DIR="${EXEC_DIR}/../deliverables"

if [ ! -d "$OUTPUT_DIR" ] && [ ! -d "$DELIVERABLES_DIR" ]; then
  echo "{\"error\":\"no output or deliverables dir\",\"rust\":null,\"typescript\":null}"
  exit 0
fi

RUST_DIR="${SCRATCH}/rust-check"
TS_DIR="${SCRATCH}/ts-check"
mkdir -p "$RUST_DIR/src" "$TS_DIR"

# Initialize Rust project skeleton (do not pollute repo Cargo.toml)
cat > "${RUST_DIR}/Cargo.toml" <<'EOF'
[package]
name = "compile-check"
version = "0.0.0"
edition = "2021"

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"
tokio = { version = "1", features = ["full"] }
anyhow = "1"
thiserror = "1"
tracing = "0.1"
EOF

# Initialize TypeScript config
cat > "${TS_DIR}/tsconfig.json" <<'EOF'
{
  "compilerOptions": {
    "target": "ES2022",
    "module": "ESNext",
    "moduleResolution": "Bundler",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "noEmit": true,
    "lib": ["ES2022", "DOM"]
  }
}
EOF

# Build list of files to scan: step_* in OUTPUT_DIR + deliverables
SCAN_FILES=()
shopt -s nullglob
for f in "$OUTPUT_DIR"/step_*.txt "$OUTPUT_DIR"/step_*.md "$OUTPUT_DIR"/step_*.rs \
         "$OUTPUT_DIR"/step_*.log; do
  SCAN_FILES+=("$f")
done
if [ -d "$DELIVERABLES_DIR" ]; then
  for d in "$DELIVERABLES_DIR"/*; do
    [ -f "$d" ] && SCAN_FILES+=("$d")
  done
fi
shopt -u nullglob

# Extract all ```rust blocks
extract_blocks() {
  local file="$1"
  local lang="$2"
  python3 -c "
import re, sys
content = open('$file').read()
pattern = r'\`\`\`$lang\n(.*?)\`\`\`'
blocks = re.findall(pattern, content, re.DOTALL)
for i, b in enumerate(blocks):
    print(f'--- block {i} ---')
    print(b)
"
}

# Extract Rust code from all scan files
RUST_EXTRACTED=0
RUST_FILES_LIST=()
for f in "${SCAN_FILES[@]}"; do
  [ ! -f "$f" ] && continue
  blocks=$(extract_blocks "$f" "rust")
  if [ -n "$blocks" ]; then
    idx=0
    fname=$(basename "$f" .txt)
    fname=${fname%.md}
    fname=${fname%.rs}
    echo "$blocks" | awk -v base="$fname" '
      /^--- block / {
        if (collect != "") {
          print collect > base"_"idx".rs"
          idx++
        }
        collect = ""
        next
      }
      { collect = collect $0 "\n" }
      END {
        if (collect != "") print collect > base"_"idx".rs"
      }
    ' || true
    for gen in "${fname}"_*.rs; do
      [ -f "$gen" ] || continue
      mv "$gen" "${RUST_DIR}/src/" 2>/dev/null || true
      RUST_EXTRACTED=$((RUST_EXTRACTED+1))
      RUST_FILES_LIST+=("$(basename "$gen")")
    done
  fi
done

# Also check for raw .rs files
shopt -s nullglob
for f in "$OUTPUT_DIR"/*.rs; do
  cp "$f" "${RUST_DIR}/src/" 2>/dev/null
  RUST_EXTRACTED=$((RUST_EXTRACTED+1))
  RUST_FILES_LIST+=("$(basename "$f")")
done
shopt -u nullglob

# Concatenate all Rust blocks into a single lib.rs for cargo check
# (each block standalone won't compile due to missing crate context)
RUST_RESULT=null
if [ "$RUST_EXTRACTED" -gt 0 ]; then
  > "${RUST_DIR}/src/lib.rs"
  for rf in "${RUST_FILES_LIST[@]}"; do
    echo "// === from $rf ===" >> "${RUST_DIR}/src/lib.rs"
    cat "${RUST_DIR}/src/$rf" >> "${RUST_DIR}/src/lib.rs" 2>/dev/null
    echo "" >> "${RUST_DIR}/src/lib.rs"
  done

  # Run cargo check in isolated dir
  cd "${RUST_DIR}"
  cargo check --offline --message-format=short > "${SCRATCH}/rust-compile.log" 2>&1
  RUST_RC=$?
  cd - > /dev/null

  RUST_ERR=$(grep -E "^error" "${SCRATCH}/rust-compile.log" 2>/dev/null | head -20 | tr '\n' '|' | sed 's/|$//')
  RUST_ERR_COUNT=$(grep -cE "^error" "${SCRATCH}/rust-compile.log" 2>/dev/null)
  [ -z "$RUST_ERR_COUNT" ] && RUST_ERR_COUNT=0

  RUST_RESULT=$(cat <<EOF
{"extracted_blocks":${RUST_EXTRACTED},"compiles":$([ $RUST_RC -eq 0 ] && echo true || echo false),"error_count":${RUST_ERR_COUNT},"sample_errors":"${RUST_ERR}"}
EOF
)
fi

# Extract TypeScript code
TS_EXTRACTED=0
TS_FILES_LIST=()
for f in "${SCAN_FILES[@]}"; do
  [ ! -f "$f" ] && continue
  for lang in typescript ts tsx; do
    blocks=$(extract_blocks "$f" "$lang")
    if [ -n "$blocks" ]; then
      idx=0
      fname=$(basename "$f")
      fname=${fname%.txt}
      fname=${fname%.md}
      echo "$blocks" | awk -v base="${fname}_${lang}" '
        /^--- block / {
          if (collect != "") { print collect > base"_"idx".ts"; idx++ }
          collect = ""; next
        }
        { collect = collect $0 "\n" }
        END { if (collect != "") print collect > base"_"idx".ts" }
      ' || true
      for gen in "${fname}_${lang}"_*.ts; do
        [ -f "$gen" ] || continue
        mv "$gen" "${TS_DIR}/" 2>/dev/null || true
        TS_EXTRACTED=$((TS_EXTRACTED+1))
        TS_FILES_LIST+=("$(basename "$gen")")
      done
    fi
  done
done

TS_RESULT=null
if [ "$TS_EXTRACTED" -gt 0 ]; then
  if command -v npx >/dev/null 2>&1; then
    cd "${TS_DIR}"
    timeout 60 npx -y typescript@5 --noEmit --project tsconfig.json > "${SCRATCH}/ts-compile.log" 2>&1
    TS_RC=$?
    cd - > /dev/null
  else
    echo "npx not available" > "${SCRATCH}/ts-compile.log"
    TS_RC=127
  fi

  TS_ERR=$(grep -E "error TS" "${SCRATCH}/ts-compile.log" 2>/dev/null | head -20 | tr '\n' '|' | sed 's/|$//')
  TS_ERR_COUNT=$(grep -cE "error TS" "${SCRATCH}/ts-compile.log" 2>/dev/null)
  [ -z "$TS_ERR_COUNT" ] && TS_ERR_COUNT=0

  TS_RESULT=$(cat <<EOF
{"extracted_blocks":${TS_EXTRACTED},"compiles":$([ $TS_RC -eq 0 ] && echo true || echo false),"error_count":${TS_ERR_COUNT},"sample_errors":"${TS_ERR}"}
EOF
)
fi

# Final JSON
echo "{"
echo "  \"rust\": ${RUST_RESULT},"
echo "  \"typescript\": ${TS_RESULT}"
echo "}"

# Cleanup scratch
rm -rf "$SCRATCH"
