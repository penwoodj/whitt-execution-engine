<!--
Source Session: ses_2e3402770ffeh0v3kQs9xezPZu
Part ID: prt_d5549fbb6001HLBYfv6BywcLdd
Character Count: 15505
Extracted: 2026-06-17T07:50:39Z
-->


I get this what I run it with the full path
asset not found: index.html


And this is what I get when going to port 1420 when running locally:

[plugin:vite:import-analysis] Failed to resolve import "lucide-react" from "src/components/CacheStatistics.tsx". Does the file exist?
/home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/src/components/CacheStatistics.tsx:3:58
18 |  import React, { useState, useEffect, useCallback, useRef } from "react";
19 |  import { invoke } from "@tauri-apps/api/core";
20 |  import { Trash2, Download, RefreshCw, BarChart3, X } from "lucide-react";
   |                                                             ^
21 |  const HISTORY_KEY = "cache_stats_history";
22 |  const ACTIVITY_KEY = "cache_activity_history";
    at TransformPluginContext._formatError (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:49258:41)
    at TransformPluginContext.error (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:49253:16)
    at normalizeUrl (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:64307:23)
    at process.processTicksAndRejections (node:internal/process/task_queues:104:5)
    at async file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:64439:39
    at async Promise.all (index 5)
    at async TransformPluginContext.transform (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:64366:7)
    at async PluginContainer.transform (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:49099:18)
    at async loadAndTransform (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:51978:27)
    at async viteTransformMiddleware (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:62106:24
Click outside, press Esc key, or fix the code to dismiss.
You can also disable this overlay by setting server.hmr.overlay to false in vite.config.ts.

and here is the terminal :

❯ ./folder-summary-visualizer/src-tauri/target/release/app
fish: Unknown command: ./folder-summary-visualizer/src-tauri/target/release/app

~
❯ sh ./folder-summary-visualizer/src-tauri/target/release/app
sh: ./folder-summary-visualizer/src-tauri/target/release/app: No such file or directory

~
❯ /home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/src-tauri/target/release/app

~ 1m 20s
❯ cd ~/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/

~/c/A/y/folder-summary-visualizer develop ≡
❯ npm run tauri:dev

> folder-summary-visualizer@0.1.0 tauri:dev
> tauri dev

     Running BeforeDevCommand (`npm run dev`)

> folder-summary-visualizer@0.1.0 dev
> vite


  VITE v5.4.21  ready in 210 ms

  ➜  Local:   http://localhost:1420/
  ➜  Network: http://192.168.1.137:1420/
        Warn Waiting for your frontend dev server to start on http://localhost:3000/...
Error: The following dependencies are imported but could not be resolved:

  lucide-react (imported by /home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/src/components/SettingsDialog.tsx)

Are they installed?
    at file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:50669:15
    at process.processTicksAndRejections (node:internal/process/task_queues:104:5)
    at async file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:50174:26
        Warn Waiting for your frontend dev server to start on http://localhost:3000/...
        Warn Waiting for your frontend dev server to start on http://localhost:3000/...
4:37:41 PM [vite] Pre-transform error: Failed to resolve import "lucide-react" from "src/components/SettingsDialog.tsx". Does the file exist?
4:37:41 PM [vite] Pre-transform error: Failed to resolve import "lucide-react" from "src/components/ThemeToggle.tsx". Does the file exist?
4:37:41 PM [vite] Pre-transform error: Failed to resolve import "lucide-react" from "src/components/CacheStatistics.tsx". Does the file exist?
4:37:41 PM [vite] Pre-transform error: Failed to resolve import "lucide-react" from "src/components/LogViewer.tsx". Does the file exist?
4:37:41 PM [vite] Internal server error: Failed to resolve import "lucide-react" from "src/components/SettingsDialog.tsx". Does the file exist?
  Plugin: vite:import-analysis
  File: /home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/src/components/SettingsDialog.tsx:14:115
  23 |    formatFrequency
  24 |  } from "../types";
  25 |  import { Cpu, Settings2, Palette, Save, Download, Upload, Eye, EyeOff, RefreshCw, AlertTriangle } from "lucide-react";
     |                                                                                                          ^
  26 |  const TABS = [
  27 |    { id: "llm", label: "LLM Settings", icon: /* @__PURE__ */ jsxDEV("span", { className: "text-lg", children: "🤖" }, void 0, false, {
      at TransformPluginContext._formatError (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:49258:41)
      at TransformPluginContext.error (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:49253:16)
      at normalizeUrl (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:64307:23)
      at process.processTicksAndRejections (node:internal/process/task_queues:104:5)
      at async file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:64439:39
      at async Promise.all (index 6)
      at async TransformPluginContext.transform (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:64366:7)
      at async PluginContainer.transform (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:49099:18)
      at async loadAndTransform (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:51978:27)
      at async viteTransformMiddleware (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:62106:24)
4:37:41 PM [vite] Internal server error: Failed to resolve import "lucide-react" from "src/components/LogViewer.tsx". Does the file exist?
  Plugin: vite:import-analysis
  File: /home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/src/components/LogViewer.tsx:3:95
  18 |  import { useState, useEffect, useCallback } from "react";
  19 |  import { invoke } from "@tauri-apps/api/core";
  20 |  import { Download, RefreshCw, Trash2, Search, Filter, FileText, X, CheckCircle, XCircle } from "lucide-react";
     |                                                                                                  ^
  21 |  const LOG_LEVELS = [
  22 |    { value: "all", label: "All Levels", color: "text-gray-400" },
      at TransformPluginContext._formatError (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:49258:41)
      at TransformPluginContext.error (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:49253:16)
      at normalizeUrl (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:64307:23)
      at process.processTicksAndRejections (node:internal/process/task_queues:104:5)
      at async file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:64439:39
      at async Promise.all (index 5)
      at async TransformPluginContext.transform (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:64366:7)
      at async PluginContainer.transform (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:49099:18)
      at async loadAndTransform (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:51978:27)
      at async viteTransformMiddleware (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:62106:24)
4:37:41 PM [vite] Internal server error: Failed to resolve import "lucide-react" from "src/components/ThemeToggle.tsx". Does the file exist?
  Plugin: vite:import-analysis
  File: /home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/src/components/ThemeToggle.tsx:2:35
  16 |  }
  17 |  var _s = $RefreshSig$();
  18 |  import { Moon, Sun, Monitor } from "lucide-react";
     |                                      ^
  19 |  import { useTheme } from "../contexts/ThemeProvider";
  20 |  export function ThemeToggle() {
      at TransformPluginContext._formatError (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:49258:41)
      at TransformPluginContext.error (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:49253:16)
      at normalizeUrl (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:64307:23)
      at process.processTicksAndRejections (node:internal/process/task_queues:104:5)
      at async file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:64439:39
      at async Promise.all (index 3)
      at async TransformPluginContext.transform (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:64366:7)
      at async PluginContainer.transform (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:49099:18)
      at async loadAndTransform (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:51978:27)
      at async viteTransformMiddleware (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:62106:24)
4:37:41 PM [vite] Internal server error: Failed to resolve import "lucide-react" from "src/components/CacheStatistics.tsx". Does the file exist?
  Plugin: vite:import-analysis
  File: /home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/src/components/CacheStatistics.tsx:3:58
  18 |  import React, { useState, useEffect, useCallback, useRef } from "react";
  19 |  import { invoke } from "@tauri-apps/api/core";
  20 |  import { Trash2, Download, RefreshCw, BarChart3, X } from "lucide-react";
     |                                                             ^
  21 |  const HISTORY_KEY = "cache_stats_history";
  22 |  const ACTIVITY_KEY = "cache_activity_history";
      at TransformPluginContext._formatError (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:49258:41)
      at TransformPluginContext.error (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:49253:16)
      at normalizeUrl (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:64307:23)
      at process.processTicksAndRejections (node:internal/process/task_queues:104:5)
      at async file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:64439:39
      at async Promise.all (index 5)
      at async TransformPluginContext.transform (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:64366:7)
      at async PluginContainer.transform (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:49099:18)
      at async loadAndTransform (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:51978:27)
      at async viteTransformMiddleware (file:///home/jon/code/AgentQueue/yaml-to-local-rust-agentsdk/folder-summary-visualizer/node_modules/vite/dist/node/chunks/dep-BK3b2jBa.js:62106:24)
        Warn Waiting for your frontend dev server to start on http://localhost:3000/...
        Warn Waiting for your frontend dev server to start on http://localhost:3000/...
        Warn Waiting for your frontend dev server to start on http://localhost:3000/...
        Warn Waiting for your frontend dev server to start on http://localhost:3000/...
        Warn Waiting for your frontend dev server to start on http://localhost:3000/...
        Warn Waiting for your frontend dev server to start on http://localhost:3000/...
        Warn Waiting for your frontend dev server to start on http://localhost:3000/...
        Warn Waiting for your frontend dev server to start on http://localhost:3000/...
        Warn Waiting for your frontend dev server to start on http://localhost:3000/...
        Warn Waiting for your frontend dev server to start on http://localhost:3000/...
        Warn Waiting for your frontend dev server to start on http://localhost:3000/...
        Warn Waiting for your frontend dev server to start on http://localhost:3000/...
        Warn Waiting for your frontend dev server to start on http://localhost:3000/...
        Warn Waiting for your frontend dev server to start on http://localhost:3000/...
        Warn Waiting for your frontend dev server to start on http://localhost:3000/...
        Warn Waiting for your frontend dev server to start on http://localhost:3000/...
        Warn Waiting for your frontend dev server to start on http://localhost:3000/...
        Warn Waiting for your frontend dev server to start on http://localhost:3000/...
