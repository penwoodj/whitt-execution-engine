#!/usr/bin/env bash
# setup-desktop-commander.sh
# Installs @wonderwhy-er/desktop-commander globally and configures it
# in ~/.claude.json for Claude Code / Cowork.
# Run once from your terminal: bash setup-desktop-commander.sh

set -e

echo "==> Installing @wonderwhy-er/desktop-commander globally..."
npm install -g --ignore-scripts @wonderwhy-er/desktop-commander

NPM_GLOBAL=$(npm root -g)
INDEX_PATH="${NPM_GLOBAL}/@wonderwhy-er/desktop-commander/dist/index.js"

if [ ! -f "$INDEX_PATH" ]; then
  echo "ERROR: Could not find installed package at $INDEX_PATH"
  exit 1
fi

echo "==> Found package at: $INDEX_PATH"

CLAUDE_JSON="$HOME/.claude.json"

# Build the MCP server entry
MCP_ENTRY=$(cat <<EOF
{
  "command": "node",
  "args": ["$INDEX_PATH"]
}
EOF
)

# If ~/.claude.json doesn't exist, create a minimal one
if [ ! -f "$CLAUDE_JSON" ]; then
  echo "==> Creating $CLAUDE_JSON"
  echo '{"mcpServers":{}}' > "$CLAUDE_JSON"
fi

echo "==> Patching $CLAUDE_JSON ..."

# Use node to safely merge into the existing JSON (handles any existing config)
node - "$CLAUDE_JSON" "$INDEX_PATH" <<'NODEJS'
const fs = require('fs');
const [,, configPath, indexPath] = process.argv;

let config = {};
try {
  config = JSON.parse(fs.readFileSync(configPath, 'utf8'));
} catch (e) {
  console.log('  (creating fresh config)');
}

if (!config.mcpServers) config.mcpServers = {};

// Remove old key if present
delete config.mcpServers['desktopCommander'];

config.mcpServers['desktop-commander'] = {
  command: 'node',
  args: [indexPath]
};

fs.writeFileSync(configPath, JSON.stringify(config, null, 2), 'utf8');
console.log('  desktop-commander entry written.');
NODEJS

echo ""
echo "==> Done! desktop-commander is configured at:"
echo "    $CLAUDE_JSON"
echo ""
echo "==> Entry written:"
node -e "const c=require(process.env.HOME+'/.claude.json'); console.log(JSON.stringify(c.mcpServers['desktop-commander'], null, 2));"
echo ""
echo "Restart Claude Code / Cowork for the MCP server to connect."
