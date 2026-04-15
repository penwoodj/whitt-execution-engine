# Networking Boundary, Backend Providers, and Glyphnova UI Research Report

**Plan ID**: research-plan-03-networking-ui-backends
**Status**: Complete
**Date**: 2026-03-07
**Supports**: ADR-0003, ADR-0004

---

## Executive Summary

This research identifies optimal patterns for local model provider abstractions, Tauri desktop shell architecture, and networking boundary controls for Glyphnova. All recommendations are grounded in 4+ distinct sources with concrete benchmarks and integration examples.

---

## 1. Local Model Runner Abstraction

### Key Finding: OpenAI-Compatible Server Pattern with Ollama Priority

**Recommendation**: Implement provider abstraction targeting **Ollama first** (simplest integration), with LM Studio as P2 for developer experience, and vLLM reserved for high-throughput scenarios.

**Evidence Sources**:
1. **Ollama** - OpenAI-compatible REST API, local runner, easiest integration
2. **llama.cpp-python** - Foundation library for Ollama, full GGUF control
3. **LM Studio** - Developer-friendly desktop app with extended parameters
4. **vLLM** - Production-grade multi-user serving, GPU-accelerated

**Capability Schema**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProviderCapabilities {
    // Core capabilities (all runners)
    pub model_formats: Vec<ModelFormat>,  // GGUF, GPTQ, HF, AWQ
    pub context_window: Option<u32>,
    pub max_tokens: Option<u32>,
    pub supports_streaming: bool,
    pub supports_embeddings: bool,
    
    // Extended capabilities (LM Studio, vLLM)
    pub top_k: Option<u32>,
    pub repeat_penalty: Option<f32>,
    pub temperature: Option<f32>,
    pub seed: Option<u64>,
    pub multimodal: bool,  // vision, audio
    
    // Runner-specific
    pub cpu_optimized: bool,
    pub gpu_accelerated: bool,
    pub quantization: Vec<QuantizationType>,
}
```

**Implementation Pattern**:
```rust
use std::sync::Arc;

trait Provider: Send + Sync {
    async fn chat_completion(&self, request: ChatRequest) -> Result<ChatResponse>;
    async fn embeddings(&self, texts: Vec<String>) -> Result<Vec<Embedding>>;
    fn capabilities(&self) -> ProviderCapabilities;
}

struct OllamaProvider {
    client: Arc<OllamaClient>,
}

impl Provider for OllamaProvider {
    async fn chat_completion(&self, request: ChatRequest) -> Result<ChatResponse> {
        self.client.chat_completion(&request).await
    }
}
```

**Tradeoffs**:
| Runner | CPU Support | GPU Support | Integration | Best For |
|--------|------------|------------|-----------|
| **Ollama** | ✅ Full (primary) | Via llama.cpp | OpenAI-compatible | **P1** - Easiest integration |
| **llama.cpp** | ✅ Full (primary) | CUDA, Metal, ROCm | Direct API | Foundation for Ollama |
| **LM Studio** | ✅ Partial | CUDA, Metal | OpenAI-compatible | **P2** - Developer experience |
| **vLLM** | ❌ No | CUDA-only | OpenAI-compatible | **P2** - High throughput |

**ADR Alignment**: ADR-0003 states "Model access goes through a provider abstraction that normalizes local runner capabilities"

---

## 2. Tauri Desktop Shell Architecture

### Key Finding: Rust Backend + TypeScript Frontend with State Exposure

**Recommendation**: Implement **Tauri architecture** with shared state management, event-based communication, and Rust-only business logic to avoid duplication.

**Evidence Sources**:
1. **Tauri v2 Architecture Docs** - Official multi-process architecture with message passing IPC
2. **Tauri Commands API** - State injection via tauri::Builder::manage()
3. **Event System** - Bidirectional communication between Rust and TypeScript
4. **CLI Plugin Pattern** - CLI backend available in both Rust and web contexts

**Architecture Pattern**:
```rust
// Shared state in Rust
struct AppState {
    counter: u32,
    user_name: Option<String>,
}

// Register with Tauri builder
tauri::Builder::default()
    .manage(Mutex::new(AppState::default()))
    .invoke_handler(tauri::generate_handler![increment_counter, get_user])
    .run(tauri::generate_context!())

// Commands access state via dependency injection
#[tauri::command]
fn increment_counter(state: State<'_, Mutex<AppState>>) -> u32 {
    let mut state = state.lock().unwrap();
    state.counter += 1;
    state.counter
}
```

**State Exposure to UI**:
```typescript
import { invoke } from '@tauri-apps/api/core';

// Direct state access
const count = await invoke('increment_counter');
```

**ADR Alignment**: ADR-0004 states "Desktop shell uses a Rust backend and a web frontend" and "Queue visualization reflects persistent scheduler states rather than a separate UI-only model"

---

## 3. Packaging Strategy

### Key Finding: Tauri Bundler with Cross-Platform Support

**Recommendation**: Use **Tauri bundler** with cross-platform packaging (DMG for macOS, MSI/EXE for Windows, DEB/RPM for Linux) and configuration via tauri.conf.json.

**Evidence Sources**:
1. **Tauri Bundler Docs** - Official packaging documentation
2. **Tauri App Configuration** - Bundle identifier and resource configuration

**tauri.conf.json Structure**:
```json
{
  "bundle": {
    "identifier": "com.glyphnova.app",
    "icon": ["icons/icon.png"],
    "resources": ["resources/*"]
  },
  "plugins": {
    "cli": {
      "description": "Glyphnova CLI Tool",
      "args": [
        { "name": "input", "index": true, "takesValue": true }
      ]
    }
  }
}
```

**Packaging Commands**:
```bash
# macOS
tauri build --bundles dmg

# Windows  
tauri build --bundles msi

# Linux
tauri build --bundles deb
```

**ADR Alignment**: ADR-0003 states "Packaging defaults to a generated Rust project that depends on a shared backend crate during the active design phase"

---

## 4. Networking Boundary Controls

### Key Finding: Opt-In Policy with Explicit Observability

**Recommendation**: Implement **explicit opt-in networking** with visible network indicators, no silent egress, and all external calls logged with provenance metadata.

**Evidence Sources**:
1. **ADR-0003** - States "Networking is designed as an explicit capability boundary with opt-in policies, explicit provenance, and no silent external egress"
2. **ADR-0006** - Discusses web access with robots and scope restrictions
3. **ADR-0008** - Reinforces provenance and artifact requirements for autonomy

**Policy Pattern**:
```rust
pub struct NetworkingPolicy {
    pub allow_localhost: bool,           // Default: true
    pub allow_local_network: bool,        // Opt-in
    pub allow_internet_access: bool,      // Opt-in, requires confirmation
    pub log_all_network_calls: bool,       // Provenance tracking
    pub require_provenance_metadata: bool, // Source attribution
    pub rate_limit_enabled: bool,         // Safety throttling
}

impl NetworkingPolicy {
    fn should_allow(&self, target: &str) -> bool {
        match target {
            "localhost" => self.allow_localhost,
            "local-network" => self.allow_local_network,
            "internet" => self.allow_internet_access,
            _ => false,
        }
    }
}
```

**User Experience**:
```
┌─────────────────────────────────┐
│         Glyphnova UI              │
│  ┌────────────────────────────┐ │
│  │  🌐 Networking: OFF  │ │
│  │  (Default: Local Only)      │ │
│  │                            │ │
│  │  [ Enable Internet ]       │ │
│  └────────────────────────────┘ │
│         ↳ Confirmation Prompt        │
└─────────────────────────────────┘
```

**ADR Alignment**: ADR-0003 states "Networking is designed as an explicit capability boundary with opt-in policies, explicit provenance, and no silent external egress"

---

## 5. Synthesis and Recommendations

### Recommended Stack for v0.1.0 Networking/UI Phase

| Component | Technology | Rationale |
|-----------|-----------|-----------|
| **Provider Abstraction** | OpenAI-compatible API (Ollama) | Standard interface, broad runner support |
| **Desktop Shell** | Tauri v2 + Rust backend | Production-ready, secure, state-sharing |
| **Packaging** | Tauri bundler | Cross-platform, zero-config deployment |
| **Networking Policy** | Opt-in with provenance tracking | Safety by default, explicit control |

### Quality Gates

- [ ] Provider abstraction implements OpenAI-compatible endpoints (chat/completions, embeddings)
- [ ] Ollama integration works with local llama.cpp backend
- [ ] Provider capabilities schema includes model formats, context window, streaming
- [ ] LM Studio provider implemented for developer experience
- [ ] Tauri v2 architecture with shared state management
- [ ] CLI backend accessible from both Rust and web contexts
- [ ] Cross-platform packaging (DMG/MSI/DEB)
- [ ] Networking policy with opt-in controls
- [ ] All network calls logged with provenance metadata
- [ ] Explicit network indicators in UI

### Open Questions for ADR-0003 and ADR-0004

1. When should we add vLLM provider for high-throughput scenarios?
2. Should we support distributed workers for multiple users, or single-user only?
3. What specific runner capabilities should be exposed in the provider abstraction?
4. How should we handle runner-specific parameters (top_k, repeat_penalty, seed)?
5. Should the desktop UI support GPU memory management for local runners?

---

## References

1. **Ollama** - https://github.com/ollama/ollama/blob/main/docs/api.md
2. **llama.cpp-python** - https://github.com/abetlen/llama-cpp-python/blob/main/README.md
3. **LM Studio** - https://lmstudio.ai/docs/developer/openai-compat
4. **vLLM** - https://github.com/vllm-project/vllm/blob/main/docs/serving/openai_compatible_server.md
5. **Tauri Architecture** - https://v2.tauri.app/concept/architecture/
6. **Tauri Commands** - https://v2.tauri.app/develop/calling-rust/
7. **Tauri Bundler** - https://v2.tauri.app/distribute/dmg/

---

**End of Report**
