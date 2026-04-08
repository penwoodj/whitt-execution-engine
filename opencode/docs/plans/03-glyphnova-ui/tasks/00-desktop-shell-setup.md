# Task 00: Desktop Shell Setup

**Estimated Time:** 5 days
**Priority:** Critical - Foundation for all UI work

**Goal:** Initialize a Tauri 2.0 desktop application with React + TypeScript frontend, configure cross-platform builds, and set up hot reload for development.

**Files:**
- Create: `agentsdk/glyphnova/src/main.rs`
- Create: `agentsdk/glyphnova/src/lib.rs`
- Create: `agentsdk/glyphnova/src/api/mod.rs`
- Create: `agentsdk/glyphnova/src-frontend/App.tsx`
- Create: `agentsdk/glyphnova/src-frontend/main.tsx`
- Create: `agentsdk/glyphnova/src-tauri/tauri.conf.json`
- Create: `agentsdk/glyphnova/src-tauri/Cargo.toml`
- Create: `agentsdk/glyphnova/package.json`
- Create: `agentsdk/glyphnova/tsconfig.json`
- Create: `agentsdk/glyphnova/vite.config.ts`
- Create: `agentsdk/glyphnova/tailwind.config.js`
- Modify: `agentsdk/Cargo.toml` (add glyphnova workspace member)
- Modify: `agentsdk/Makefile` (add UI build targets)

---

## Step 1: Initialize Tauri Project

- [ ] **Step 1.1: Create glyphnova directory structure**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk
mkdir -p glyphnova/src/api glyphnova/src-tauri glyphnova/src-frontend/{components,hooks,stores,types,utils}
```

- [ ] **Step 1.2: Initialize Tauri project**

```bash
cd glyphnova
cargo create-tauri-app --name glyphnova --template react-ts
# When prompted, select:
# - Project name: glyphnova
# - Frontend: React + TypeScript
# - Package manager: npm
# - UI template: Vanilla (we'll use shadcn/ui later)
```

Expected: Tauri creates initial project structure with React + TypeScript

- [ ] **Step 1.3: Verify Tauri project was created**

```bash
ls -la
# Should see: package.json, src-tauri/, src-frontend/, tauri.conf.json, etc.
```

Expected: Standard Tauri project structure present

---

## Step 2: Configure Tauri Backend

- [ ] **Step 2.1: Create main.rs entry point**

```rust
// agentsdk/glyphnova/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    glyphnova_lib::run()
}
```

- [ ] **Step 2.2: Create lib.rs with command handlers**

```rust
// agentsdk/glyphnova/src/lib.rs
mod api;

use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            api::api_request,
            api::get_runtime_state,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 2.3: Create API module**

```rust
// agentsdk/glyphnova/src/api/mod.rs
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiRequest {
    pub method: String,
    pub path: String,
    pub body: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub status: u16,
    pub body: String,
}

#[tauri::command]
pub async fn api_request(req: ApiRequest) -> Result<ApiResponse, String> {
    // TODO: Implement actual API call to runtime
    // For now, return a mock response
    Ok(ApiResponse {
        status: 200,
        body: r#"{"message": "API not yet connected"}"#.to_string(),
    })
}

#[tauri::command]
pub fn get_runtime_state() -> Result<String, String> {
    // TODO: Return actual runtime state
    Ok(r#"{"status": "disconnected"}"#.to_string())
}
```

- [ ] **Step 2.4: Update Cargo.toml**

```toml
# agentsdk/glyphnova/src-tauri/Cargo.toml
[package]
name = "glyphnova"
version = "0.1.0"
description = "Glyphnova UI for AgentSDK"
authors = ["you"]
license = ""
repository = ""
edition = "2021"

[build-dependencies]
tauri-build = { version = "2.0", features = [] }

[dependencies]
tauri = { version = "2.0", features = ["shell-open"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"

[features]
default = ["custom-protocol"]
custom-protocol = ["tauri/custom-protocol"]
```

- [ ] **Step 2.5: Configure tauri.conf.json**

```json
{
  "$schema": "https://schema.tauri.app/config/2.0.0",
  "productName": "Glyphnova",
  "version": "0.1.0",
  "identifier": "com.agentsdk.glyphnova",
  "build": {
    "beforeDevCommand": "npm run dev",
    "devUrl": "http://localhost:5173",
    "beforeBuildCommand": "npm run build",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "title": "Glyphnova",
        "width": 1400,
        "height": 900,
        "resizable": true,
        "fullscreen": false
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": []
  },
  "plugins": {}
}
```

- [ ] **Step 2.6: Build Tauri backend**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk/glyphnova
cargo build --manifest-path=src-tauri/Cargo.toml
```

Expected: Build succeeds without errors

---

## Step 3: Configure React Frontend

- [ ] **Step 3.1: Update package.json dependencies**

```json
{
  "name": "glyphnova-frontend",
  "version": "0.1.0",
  "private": true,
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "lint": "eslint . --ext ts,tsx --report-unused-disable-directives --max-warnings 0"
  },
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "@tauri-apps/api": "^2.0.0",
    "zustand": "^4.4.0",
    "@tanstack/react-query": "^5.0.0",
    "react-router-dom": "^6.20.0",
    "class-variance-authority": "^0.7.0",
    "clsx": "^2.0.0",
    "tailwind-merge": "^2.0.0",
    "lucide-react": "^0.300.0"
  },
  "devDependencies": {
    "@types/react": "^18.2.43",
    "@types/react-dom": "^18.2.17",
    "@typescript-eslint/eslint-plugin": "^6.14.0",
    "@typescript-eslint/parser": "^6.14.0",
    "@vitejs/plugin-react": "^4.2.1",
    "eslint": "^8.55.0",
    "eslint-plugin-react-hooks": "^4.6.0",
    "eslint-plugin-react-refresh": "^0.4.5",
    "typescript": "^5.2.2",
    "vite": "^5.0.8",
    "tailwindcss": "^3.4.0",
    "postcss": "^8.4.32",
    "autoprefixer": "^10.4.16"
  }
}
```

- [ ] **Step 3.2: Create main.tsx entry point**

```typescript
// agentsdk/glyphnova/src-frontend/main.tsx
import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './index.css'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
)
```

- [ ] **Step 3.3: Create App.tsx root component**

```typescript
// agentsdk/glyphnova/src-frontend/App.tsx
import { invoke } from '@tauri-apps/api/core'

function App() {
  const [greeting, setGreeting] = React.useState('Loading...')

  React.useEffect(() => {
    invoke('get_runtime_state')
      .then((result: any) => setGreeting(JSON.stringify(result)))
      .catch(console.error)
  }, [])

  return (
    <div className="min-h-screen bg-gray-900 text-white">
      <div className="container mx-auto p-8">
        <h1 className="text-4xl font-bold mb-4">Glyphnova</h1>
        <p className="text-lg">{greeting}</p>
      </div>
    </div>
  )
}

export default App
```

- [ ] **Step 3.4: Create basic CSS**

```css
/* agentsdk/glyphnova/src-frontend/index.css */
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  body {
    @apply bg-gray-900 text-gray-100;
  }
}
```

- [ ] **Step 3.5: Configure tsconfig.json**

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true,
    "baseUrl": ".",
    "paths": {
      "@/*": ["./src/*"]
    }
  },
  "include": ["src"],
  "references": [{ "path": "./tsconfig.node.json" }]
}
```

- [ ] **Step 3.6: Configure vite.config.ts**

```typescript
// agentsdk/glyphnova/vite.config.ts
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import path from 'path'

export default defineConfig(async () => ({
  plugins: [react()],
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    target: process.env.TAURI_PLATFORM == 'windows' ? 'chrome105' : 'safari13',
    if you don't target a browser or
    you don't support web workers explicitly add the browser entry to make rollup
    choose a name that is valid for a target browser
    target: 'chrome105',  // or: 'safari13' for macOS,
    },
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
}))
```

- [ ] **Step 3.7: Configure Tailwind CSS**

```javascript
// agentsdk/glyphnova/tailwind.config.js
/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
```

- [ ] **Step 3.8: Create postcss.config.js**

```javascript
// agentsdk/glyphnova/postcss.config.js
export default {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
}
```

- [ ] **Step 3.9: Install dependencies**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk/glyphnova
npm install
```

Expected: All npm packages install successfully

---

## Step 4: Test Tauri Application

- [ ] **Step 4.1: Run Tauri dev server**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk/glyphnova
npm run tauri dev
```

Expected:
- Tauri window opens
- Displays "Glyphnova" heading
- Shows runtime state JSON

- [ ] **Step 4.2: Verify hot reload works**

```bash
# While dev server is running, modify src-frontend/App.tsx
# Change "Glyphnova" to "Glyphnova UI"
```

Expected: Window content updates automatically

- [ ] **Step 4.3: Stop dev server**

```bash
# Press Ctrl+C to stop the dev server
```

---

## Step 5: Configure Cross-Platform Builds

- [ ] **Step 5.1: Update tauri.conf.json for all platforms**

```json
// In agentsdk/glyphnova/src-tauri/tauri.conf.json
"bundle": {
  "active": true,
  "targets": "all",
  "icon": [
    "icons/32x32.png",
    "icons/128x128.png",
    "icons/128x128@2x.png",
    "icons/icon.icns",
    "icons/icon.ico"
  ]
}
```

- [ ] **Step 5.2: Add build targets to Makefile**

```makefile
# Add to agentsdk/Makefile
.PHONY: ui-dev ui-build ui-clean

ui-dev:
	@echo "Starting Glyphnova dev server..."
	cd glyphnova && npm run tauri dev

ui-build:
	@echo "Building Glyphnova for all platforms..."
	cd glyphnova && npm run tauri build

ui-build-linux:
	@echo "Building Glyphnova for Linux..."
	cd glyphnova && npm run tauri build --target x86_64-unknown-linux-gnu

ui-build-mac:
	@echo "Building Glyphnova for macOS..."
	cd glyphnova && npm run tauri build --target x86_64-apple-darwin

ui-build-windows:
	@echo "Building Glyphnova for Windows..."
	cd glyphnova && npm run tauri build --target x86_64-pc-windows-msvc

ui-clean:
	@echo "Cleaning Glyphnova build artifacts..."
	cd glyphnova && rm -rf dist src-tauri/target
```

- [ ] **Step 4.3: Test build for current platform**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk
make ui-build
```

Expected: Build completes and creates executable in `glyphnova/src-tauri/target/release/bundle/`

---

## Step 6: Add glyphnova to Workspace

- [ ] **Step 6.1: Update root Cargo.toml**

```toml
# Add to agentsdk/Cargo.toml
[workspace]
members = [
    # ... existing members ...
    "glyphnova/src-tauri",
]
```

- [ ] **Step 6.2: Verify workspace includes glyphnova**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk
cargo tree -p glyphnova
```

Expected: Shows glyphnova and its dependencies

---

## Step 7: Create Component Structure

- [ ] **Step 7.1: Create base components directory structure**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk/glyphnova/src-frontend
mkdir -p components/{layout,queue,navigation,visualization,artifacts}
```

- [ ] **Step 7.2: Create Layout components**

```typescript
// agentsdk/glyphnova/src-frontend/components/layout/Header.tsx
export function Header() {
  return (
    <header className="bg-gray-800 border-b border-gray-700 h-16 flex items-center px-6">
      <h1 className="text-xl font-bold">Glyphnova</h1>
    </header>
  )
}
```

```typescript
// agentsdk/glyphnova/src-frontend/components/layout/Sidebar.tsx
export function Sidebar() {
  return (
    <aside className="bg-gray-850 w-64 border-r border-gray-700 p-4">
      <h2 className="text-lg font-semibold mb-4">Queue</h2>
      <p className="text-gray-400">Queue loading...</p>
    </aside>
  )
}
```

```typescript
// agentsdk/glyphnova/src-frontend/components/layout/MainContent.tsx
export function MainContent() {
  return (
    <main className="flex-1 p-6">
      <p className="text-gray-400">Select an item to view details</p>
    </main>
  )
}
```

- [ ] **Step 7.3: Create index files for components**

```typescript
// agentsdk/glyphnova/src-frontend/components/layout/index.ts
export { Header } from './Header'
export { Sidebar } from './Sidebar'
export { MainContent } from './MainContent'
```

---

## Step 8: Update App with Layout

- [ ] **Step 8.1: Update App.tsx to use layout components**

```typescript
// agentsdk/glyphnova/src-frontend/App.tsx
import { Header, Sidebar, MainContent } from './components/layout'

function App() {
  return (
    <div className="min-h-screen bg-gray-900 text-gray-100 flex flex-col">
      <Header />
      <div className="flex flex-1 overflow-hidden">
        <Sidebar />
        <MainContent />
      </div>
    </div>
  )
}

export default App
```

- [ ] **Step 8.2: Test layout**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk/glyphnova
npm run tauri dev
```

Expected: Window shows header, sidebar, and main content areas

---

## Step 9: Add ESLint and Prettier

- [ ] **Step 9.1: Create .eslintrc.json**

```json
{
  "root": true,
  "env": { "browser": true, "es2020": true },
  "extends": [
    "eslint:recommended",
    "plugin:@typescript-eslint/recommended",
    "plugin:react-hooks/recommended"
  ],
  "ignorePatterns": ["dist", ".eslintrc.json"],
  "parser": "@typescript-eslint/parser",
  "plugins": ["react-refresh"],
  "rules": {
    "react-refresh/only-export-components": [
      "warn",
      { "allowConstantExport": true }
    ]
  }
}
```

- [ ] **Step 9.2: Create .prettierrc**

```json
{
  "semi": true,
  "singleQuote": true,
  "tabWidth": 2,
  "trailingComma": "es5"
}
```

- [ ] **Step 9.3: Add lint script to package.json**

```json
"scripts": {
  "lint": "eslint . --ext ts,tsx --report-unused-disable-directives --max-warnings 0",
  "format": "prettier --write \"src/**/*.{ts,tsx,css}\""
}
```

- [ ] **Step 9.4: Run linter**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk/glyphnova
npm run lint
```

Expected: No linting errors

---

## Step 10: Create Documentation

- [ ] **Step 10.1: Create README.md**

```markdown
# Glyphnova - AgentSDK Desktop UI

Glyphnova is a native desktop application providing visual shell over the AgentSDK runtime.

## Development

### Prerequisites

- Node.js 18+
- Rust stable toolchain
- Tauri CLI: `cargo install tauri-cli`

### Running in Development

```bash
npm run tauri dev
```

### Building for Production

```bash
# All platforms
npm run tauri build

# Specific platform
make ui-build-linux   # Linux
make ui-build-mac      # macOS
make ui-build-windows  # Windows
```

### Project Structure

```
glyphnova/
├── src/                  # Rust backend
│   ├── main.rs          # Entry point
│   ├── lib.rs           # Tauri commands
│   └── api/             # API wrappers
├── src-frontend/         # React frontend
│   ├── components/      # React components
│   ├── hooks/           # Custom hooks
│   ├── stores/          # Zustand stores
│   └── types/           # TypeScript types
└── src-tauri/           # Tauri configuration
```

## Architecture

Glyphnova is a **projection layer** over the AgentSDK runtime:
- No business logic in UI
- All state comes from runtime API
- WebSocket for real-time updates
- REST API for queries and actions

See `plan.md` for full architecture details.
```

- [ ] **Step 10.2: Add setup instructions to main README**

```markdown
# AgentSDK

[... existing content ...]

## Desktop UI

Glyphnova provides a native desktop interface for the AgentSDK runtime.

See [glyphnova/README.md](./glyphnova/README.md) for UI development instructions.
```

---

## Step 11: Final Verification

- [ ] **Step 11.1: Run full dev build**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk/glyphnova
npm run tauri dev
```

Expected: App launches successfully with header, sidebar, main content

- [ ] **Step 11.2: Verify hot reload works**

Modify `App.tsx`, save, verify content updates automatically

- [ ] **Step 11.3: Run linter**

```bash
npm run lint
```

Expected: No linting errors

- [ ] **Step 11.4: Test production build**

```bash
npm run tauri build
```

Expected: Build completes, executable created in `src-tauri/target/release/bundle/`

---

## Step 12: Commit

- [ ] **Step 12.1: Stage and commit changes**

```bash
cd /home/jon/code/yaml-to-rust-agentsdk
git add glyphnova/ Cargo.toml Makefile README.md
git commit -m "feat(ui): initialize Tauri desktop shell with React+TypeScript

- Create Tauri project structure with React+TypeScript frontend
- Configure cross-platform builds (Linux/macOS/Windows)
- Set up hot reload for development
- Create base layout components (header, sidebar, main content)
- Add ESLint and Prettier configuration
- Integrate with cargo workspace

Task: 00-desktop-shell-setup
Part of: Phase 3 - Glyphnova UI"
```

---

## Success Criteria

- [ ] Tauri application builds and runs
- [ ] Window displays with header, sidebar, main content
- [ ] Hot reload works during development
- [ ] Production build creates cross-platform executables
- [ ] Linting passes without errors
- [ ] Documentation is complete
- [ ] Integrated with cargo workspace

---

## Next Steps

After completing this task, proceed to **Task 01: Shared Backend API** to connect the frontend to the runtime API.
