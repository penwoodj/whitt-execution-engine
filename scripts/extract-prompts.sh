#!/usr/bin/env bash
# Extract natural language user prompts from OpenCode session DB.
# Filters out: system messages, file pastes, logs, YAML content, short messages.
#
# Usage: bash scripts/extract-prompts.sh [OUTPUT_DIR] [MIN_LENGTH]
# Default output: docs/plans/meta-workflow-qwen35/test-prompts/real/
# Default min length: 2000 chars

set -euo pipefail

DB="${OPENCODE_DB:-$HOME/.local/share/opencode/opencode.db}"
OUTPUT_DIR="${1:-docs/plans/meta-workflow-qwen35/test-prompts/real}"
MIN_LENGTH="${2:-2000}"

mkdir -p "$OUTPUT_DIR"

echo "Extracting prompts from $DB → $OUTPUT_DIR (min $MIN_LENGTH chars)"

# Query: text-type parts from user messages, filtered for natural language
# Excludes: YAML content, file pastes, logs, system reminders, mode headers, compilation output
QUERY="
SELECT DISTINCT
  p.session_id,
  p.id,
  length(p.data) as data_len,
  json_extract(p.data, '\$.text') as text_content
FROM part p
JOIN message m ON p.message_id = m.id
WHERE m.data LIKE '%\"role\":\"user\"%'
  AND p.data LIKE '%\"type\":\"text\"%'
  AND length(p.data) >= ${MIN_LENGTH}
  AND json_extract(p.data, '\$.text') IS NOT NULL
  -- Exclude YAML workflow content
  AND json_extract(p.data, '\$.text') NOT LIKE '%workflow_id:%'
  AND json_extract(p.data, '\$.text') NOT LIKE '%providers:%'
  AND json_extract(p.data, '\$.text') NOT LIKE '%agentic_workflow:%'
  -- Exclude file pastes
  AND json_extract(p.data, '\$.text') NOT LIKE '<path>%'
  AND json_extract(p.data, '\$.text') NOT LIKE '<command-instruction>%'
  AND json_extract(p.data, '\$.text') NOT LIKE '<auto-slash-command>%'
  -- Exclude system content
  AND json_extract(p.data, '\$.text') NOT LIKE '%<system-reminder>%'
  AND json_extract(p.data, '\$.text') NOT LIKE '%[SYSTEM DIRECTIVE%'
  -- Exclude mode headers
  AND json_extract(p.data, '\$.text') NOT LIKE '[search-mode]%'
  AND json_extract(p.data, '\$.text') NOT LIKE '[analyze-mode]%'
  -- Exclude logs/compilation
  AND json_extract(p.data, '\$.text') NOT LIKE '   Compiling%'
  AND json_extract(p.data, '\$.text') NOT LIKE '%whitt-llama-server%'
  AND json_extract(p.data, '\$.text') NOT LIKE '%docker compose%'
  -- Exclude diffs/patches
  AND json_extract(p.data, '\$.text') NOT LIKE '%Index:%'
  AND json_extract(p.data, '\$.text') NOT LIKE '%.patch%'
ORDER BY data_len DESC
LIMIT 50;
"

COUNT=0
while IFS='|' read -r session_id part_id data_len text_preview; do
    COUNT=$((COUNT + 1))
    # Extract full text
    FULL_TEXT=$(sqlite3 "$DB" "SELECT json_extract(data, '\$.text') FROM part WHERE id='$part_id';")

    # Generate descriptive filename from first 50 chars
    DESC=$(echo "$FULL_TEXT" | head -c 100 | tr -dc 'a-zA-Z0-9 -' | tr ' ' '-' | tr '[:upper:]' '[:lower:]' | head -c 50)

    FILENAME=$(printf "prompt-%02d-%s.md" "$COUNT" "$DESC")
    FILEPATH="$OUTPUT_DIR/$FILENAME"

    # Write prompt with metadata header
    {
        echo "<!--"
        echo "Source Session: $session_id"
        echo "Part ID: $part_id"
        echo "Character Count: $data_len"
        echo "Extracted: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
        echo "-->"
        echo ""
        echo "$FULL_TEXT"
    } > "$FILEPATH"

    echo "  [$COUNT] $FILENAME (${data_len} chars) from $session_id"
done < <(sqlite3 "$DB" "$QUERY" 2>/dev/null)

echo ""
echo "Extracted $COUNT prompts to $OUTPUT_DIR"

# Create index
{
    echo "# Extracted Prompt Dataset"
    echo ""
    echo "Extracted from OpenCode session database on $(date -u +%Y-%m-%d)"
    echo ""
    echo "| # | File | Chars | Session |"
    echo "|---|------|-------|---------|"
    ls -1 "$OUTPUT_DIR"/prompt-*.md 2>/dev/null | sort | while read -r f; do
        BASENAME=$(basename "$f")
        CHARS=$(wc -c < "$f")
        SESSION=$(grep -oP 'Source Session: \K.*' "$f" | head -1)
        NUM=$(echo "$BASENAME" | grep -oP 'prompt-\K[0-9]+')
        echo "| $NUM | $BASENAME | $CHARS | $SESSION |"
    done
} > "$OUTPUT_DIR/INDEX.md"

echo "Index written to $OUTPUT_DIR/INDEX.md"
