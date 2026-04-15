# Integration Schema for Infinite Context

## Executive Summary

**Purpose**: Define schema for integrating infinite context techniques into execution engine
**Approaches**: RLM, Infini-Attention, MELODI, RAG
**Stack**: Rust-based execution engine with Ollama/llama.cpp backends

## Architecture Overview

```
┌────────────────────────────────────────────────────────────┐
│              User Query / Task                     │
└────────────────────────┬───────────────────────────┘
                     │
                     ↓
┌────────────────────────────────────────────────────┐
│        Context Orchestrator                    │
│  - Selects strategy based on context size    │
│  - Manages token budgets                 │
│  - Routes to appropriate backend            │
└─────────────┬────────────┬────────────────┘
              │            │
              ↓            ↓
    ┌──────────────┐  ┌────────────────────┐
    │ RLM Backend  │  │ Vector RAG Backend │
    └──────┬───────┘  └────────┬───────────┘
           │                │
           │                ↓
           │      ┌────────────────────────┐
           │      │ Ollama/llama.cpp    │
           │      │ Local LLM Backend  │
           │      └─────────┬────────────┘
           │                │
           └────────────────┴
                     │
                     ↓
          ┌────────────────────────┐
          │ Final Response         │
          └────────────────────────┘
```

## Core Components

### 1. Context Orchestrator

```rust
pub struct ContextOrchestrator {
    config: OrchestratorConfig,
    rlm_backend: Option<RLMBackend>,
    rag_backend: Option<RAGBackend>,
    llm_backend: LLMBBackend,
}

pub struct OrchestratorConfig {
    // Strategy selection thresholds
    rlm_threshold: usize,  // Use RLM if context > N tokens
    rag_threshold: usize,  // Use RAG if context > M tokens
    max_context: usize,    // Maximum context before forced split

    // Performance settings
    max_depth: usize,        // RLM max recursion depth
    token_budget: usize,     // Total tokens allowed
    parallel_subcalls: bool,  // Parallel vs sequential sub-LLMs

    // Backend selection
    preferred_backend: BackendType,
    fallback_backend: BackendType,
}

pub enum BackendType {
    RLM,
    RAG,
    DirectLLM,
    Hybrid,  // RLM + RAG combined
}
```

### 2. RLM Backend

```rust
pub struct RLMBackend {
    env: REPLEnvironment,
    llm_adapter: Box<dyn LLMAdapter>,
    config: RLMConfig,
}

pub struct RLMConfig {
    // REPL settings
    repl_type: REPLType,  // Local, Docker, WASM
    max_variables: usize,
    timeout_ms: u64,

    // RLM-specific
    max_depth: usize,
    system_prompt: String,
    tools: Vec<Tool>,
}

pub enum REPLType {
    Local,        // Python REPL on host
    Docker,       // Containerized Python
    WASM,         // WebAssembly sandbox
    Restricted,   // RestrictedPython library
}

pub struct Tool {
    name: String,
    description: String,
    function: fn(&mut REPLEnvironment, Vec<String>) -> Result<String>,
}
```

### 3. RAG Backend

```rust
pub struct RAGBackend {
    embedder: Box<dyn EmbeddingProvider>,
    vector_store: Box<dyn VectorStore>,
    llm: LLMBBackend,
    config: RAGConfig,
}

pub struct RAGConfig {
    // Chunking
    chunk_size: usize,
    overlap: usize,
    strategy: ChunkStrategy,

    // Retrieval
    top_k: usize,
    score_threshold: f32,
    hybrid_search: bool,

    // Embedding
    embedding_model: String,
    batch_size: usize,

    // Context construction
    max_context_tokens: usize,
    prompt_template: String,
}

pub enum ChunkStrategy {
    FixedSize,    // Fixed token count
    Semantic,     // Paragraph/sentence boundaries
    Recursive,     // Hierarchical chunking
    Hybrid,        // Combination of strategies
}
```

### 4. LLM Backend Abstraction

```rust
pub trait LLMAdapter {
    async fn complete(&self, prompt: &str) -> Result<String>;

    async fn complete_stream(&self, prompt: &str) -> Result<Pin<Box<dyn Stream<Item = String>>>>;

    fn token_count(&self, text: &str) -> usize;

    async fn embed(&self, text: &str) -> Result<Vec<f32>>;
}

pub struct OllamaBackend {
    endpoint: String,  // http://localhost:11434
    model: String,
}

impl LLMAdapter for OllamaBackend {
    async fn complete(&self, prompt: &str) -> Result<String> {
        let response = reqwest::Client::new()
            .post(format!("{}/api/generate", self.endpoint))
            .json(&json!({
                "model": self.model,
                "prompt": prompt,
                "stream": false
            }))
            .send()
            .await?;

        Ok(response.json::<OllamaResponse>()?.response)
    }

    async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let response = reqwest::Client::new()
            .post(format!("{}/api/embed", self.endpoint))
            .json(&json!({
                "model": self.model,
                "input": text
            }))
            .send()
            .await?;

        Ok(response.json::<OllamaEmbeddingResponse>()?.embedding)
    }
}

pub struct LlamaCppBackend {
    library_path: PathBuf,
    model_path: PathBuf,
}

// Similar implementation using llama.cpp C API
```

### 5. REPL Environment

```rust
pub struct REPLEnvironment {
    variables: HashMap<String, VariableValue>,
    history: Vec<ExecutionResult>,
    config: REPLConfig,
}

pub enum VariableValue {
    String(String),
    Number(f64),
    List(Vec<VariableValue>),
}

impl REPLEnvironment {
    pub fn set_variable(&mut self, name: &str, value: VariableValue) {
        self.variables.insert(name.to_string(), value);
    }

    pub fn get_variable(&self, name: &str) -> Option<&VariableValue> {
        self.variables.get(name)
    }

    pub fn peek(&self, var_name: &str, range_start: usize, range_end: usize) -> Result<String> {
        match self.get_variable(var_name) {
            Some(VariableValue::String(text)) => {
                let chars: Vec<char> = text.chars().collect();
                if range_start >= chars.len() || range_end > chars.len() {
                    return Err(Error::IndexOutOfBounds);
                }
                let slice: String = chars[range_start..range_end].iter().collect();
                Ok(slice)
            },
            _ => Err(Error::InvalidType),
        }
    }

    pub fn chunk(&self, var_name: &str, delimiter: &str) -> Result<Vec<String>> {
        match self.get_variable(var_name) {
            Some(VariableValue::String(text)) => {
                Ok(text.split(delimiter).map(|s| s.to_string()).collect())
            },
            _ => Err(Error::InvalidType),
        }
    }

    pub fn execute(&mut self, code: &str) -> Result<ExecutionResult> {
        // Execute Python code
        let output = python::execute(code)?;

        // Update history
        self.history.push(ExecutionResult {
            code: code.to_string(),
            output: output.clone(),
            timestamp: SystemTime::now(),
        });

        Ok(ExecutionResult {
            code: code.to_string(),
            output,
        })
    }
}
```

## Schema Definitions

### Task Schema

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub task_type: TaskType,
    pub query: String,
    pub context_source: ContextSource,
    pub config: TaskConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    QuestionAnswering,
    DocumentAnalysis,
    CodeComprehension,
    Summarization,
    CreativeWriting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContextSource {
    Direct(String),           // Direct text
    File(PathBuf),           // Read from file
    URL(String),             // Fetch from URL
    VectorStore(Vec<String>), // Retrieve from RAG
    Environment(String),    // From REPL variable
}
```

### Result Schema

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: String,
    pub answer: String,
    pub strategy: Strategy,
    pub stats: ExecutionStats,
    pub sources: Vec<Source>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Strategy {
    DirectLLM,
    RLM { depth: usize, subcalls: usize },
    RAG { retrieved_chunks: usize },
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStats {
    pub total_tokens: usize,
    pub llm_tokens: usize,
    pub sub_llm_calls: usize,
    pub latency_ms: u64,
    pub memory_peak_mb: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub id: String,
    pub type: SourceType,
    pub relevance_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceType {
    ContextChunk,
    FileDocument,
    CodeSnippet,
    RetrievedDocument,
}
```

### Configuration Schema

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    pub context: ContextConfig,
    pub backends: BackendConfig,
    pub performance: PerformanceConfig,
    pub monitoring: MonitoringConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextConfig {
    pub rlm_threshold: usize,
    pub rag_threshold: usize,
    pub max_total_tokens: usize,
    pub default_strategy: BackendType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendConfig {
    pub ollama: Option<OllamaConfig>,
    pub llama_cpp: Option<LlamaCppConfig>,
    pub vector_store: Option<VectorStoreConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaConfig {
    pub endpoint: String,
    pub default_model: String,
    pub timeout_ms: u64,
    pub max_retries: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlamaCppConfig {
    pub library_path: PathBuf,
    pub model_path: PathBuf,
    pub num_gpu_layers: usize,
    pub context_size: usize,
    pub gpu_backend: GPUBackend,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GPUBackend {
    Vulkan,
    CUDA,
    ROCm,
    Metal,
}
```

## Workflow Integration

### 1. Strategy Selection

```rust
impl ContextOrchestrator {
    pub fn select_strategy(&self, task: &Task) -> Strategy {
        let context_size = self.estimate_context_size(&task.context_source);

        match task.task_type {
            TaskType::QuestionAnswering => {
                if context_size < self.config.rlm_threshold {
                    Strategy::DirectLLM
                } else if context_size < self.config.rag_threshold {
                    Strategy::RAG
                } else {
                    Strategy::RLM {
                        depth: 1,  // Start with depth=1
                        subcalls: 0,
                    }
                }
            },

            TaskType::DocumentAnalysis => {
                // Always use RLM for large documents
                if context_size > self.config.rlm_threshold {
                    Strategy::RLM { depth: 1, subcalls: 0 }
                } else {
                    Strategy::RAG
                }
            },

            TaskType::CodeComprehension => {
                // Hybrid: RAG for API docs, RLM for implementation
                if self.has_api_docs(&task) {
                    Strategy::Hybrid
                } else {
                    Strategy::RAG
                }
            },

            // ... other task types
        }
    }

    fn estimate_context_size(&self, source: &ContextSource) -> usize {
        match source {
            ContextSource::Direct(text) => token_counter::count(text),
            ContextSource::File(path) => {
                if let Ok(metadata) = fs::metadata(path) {
                    metadata.len() as usize  // Approximate bytes → tokens
                } else {
                    0
                }
            },
            ContextSource::URL(url) => {
                // Would need to fetch and estimate
                10000  // Placeholder
            },
            ContextSource::VectorStore(_) => 5000,  // Typical RAG context
            ContextSource::Environment(var_name) => {
                self.env.get_variable(var_name)
                    .map(|v| match v {
                        VariableValue::String(s) => token_counter::count(s),
                        _ => 0,
                    })
                    .unwrap_or(0)
            },
        }
    }
}
```

### 2. Execution Flow

```rust
impl ContextOrchestrator {
    pub async fn execute_task(&mut self, task: Task) -> Result<TaskResult> {
        let strategy = self.select_strategy(&task);
        let start_time = SystemTime::now();

        let (answer, stats) = match strategy {
            Strategy::DirectLLM => self.execute_direct_llm(&task).await?,
            Strategy::RLM { depth, subcalls } => {
                self.execute_rlm(&task, depth, subcalls).await?
            },
            Strategy::RAG => self.execute_rag(&task).await?,
            Strategy::Hybrid => self.execute_hybrid(&task).await?,
        };

        let latency = SystemTime::now().duration_since(start_time);

        Ok(TaskResult {
            task_id: task.id.clone(),
            answer,
            strategy,
            stats: ExecutionStats {
                total_tokens: stats.total_tokens,
                llm_tokens: stats.llm_tokens,
                sub_llm_calls: stats.sub_llm_calls,
                latency_ms: latency.as_millis(),
                memory_peak_mb: stats.memory_peak_mb,
            },
            sources: stats.sources,
        })
    }
}
```

## Error Handling

```rust
#[derive(Debug)]
pub enum EngineError {
    ContextTooLarge { size: usize, max: usize },
    Timeout { operation: String, elapsed_ms: u64 },
    BackendUnavailable { backend: String },
    TokenBudgetExceeded { budget: usize, used: usize },
    SubLLMFailure { sub_call: usize, error: String },
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineError::ContextTooLarge { size, max } => {
                write!(f, "Context too large: {} tokens (max {})", size, max)
            },
            EngineError::Timeout { operation, elapsed } => {
                write!(f, "Timeout in {}: {}ms", operation, elapsed)
            },
            // ... other variants
        }
    }
}
```

## Validation

```rust
pub struct Validator;

impl Validator {
    pub fn validate_config(&self, config: &EngineConfig) -> Result<(), Vec<Error>> {
        let mut errors = Vec::new();

        // Check thresholds
        if config.context.rlm_threshold > config.context.rag_threshold {
            errors.push(Error::InvalidThresholds);
        }

        // Check backend availability
        if config.backends.ollama.is_none() &&
           config.backends.llama_cpp.is_none() {
            errors.push(Error::NoBackendConfigured);
        }

        // Check model exists
        if let Some(llama) = &config.backends.llama_cpp {
            if !llama.model_path.exists() {
                errors.push(Error::ModelNotFound(llama.model_path.clone()));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
```

## Testing Schema

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy_selection() {
        let config = EngineConfig::default();

        // Small context → Direct LLM
        let task = Task {
            id: "test1".to_string(),
            task_type: TaskType::QuestionAnswering,
            query: "Test".to_string(),
            context_source: ContextSource::Direct("Short text".to_string()),
            config: TaskConfig::default(),
        };

        let orchestrator = ContextOrchestrator::new(config);
        let strategy = orchestrator.select_strategy(&task);

        assert!(matches!(strategy, Strategy::DirectLLM));
    }

    #[test]
    fn test_rlm_integration() {
        let config = EngineConfig {
            context: ContextConfig {
                rlm_threshold: 1000,  // 1K tokens → RLM
                ..Default::default()
            },
            ..Default::default()
        };

        let task = Task {
            id: "test2".to_string(),
            task_type: TaskType::QuestionAnswering,
            query: "Test".to_string(),
            context_source: ContextSource::Direct("A".repeat(100_000)),  // 100K tokens
            config: TaskConfig::default(),
        };

        let orchestrator = ContextOrchestrator::new(config);
        let strategy = orchestrator.select_strategy(&task);

        assert!(matches!(strategy, Strategy::RLM { depth: 1, subcalls: _ }));
    }
}
```

## Migration Path

### Phase 1: Core Infrastructure
- [ ] Implement `LLMAdapter` trait
- [ ] Implement Ollama backend
- [ ] Implement llama.cpp backend (with Vulkan support)
- [ ] Implement REPL environment (Python execution)
- [ ] Add basic configuration schema

### Phase 2: RLM Backend
- [ ] Implement `RLMBackend` struct
- [ ] Add RLM system prompt
- [ ] Implement peek/chunk tools
- [ ] Add depth control and token budgeting
- [ ] Add streaming support

### Phase 3: RAG Backend
- [ ] Implement vector store abstraction
- [ ] Add ChromaDB support
- [ ] Add FAISS support
- [ ] Implement embedding provider
- [ ] Add chunking strategies
- [ ] Implement hybrid search

### Phase 4: Orchestration
- [ ] Implement strategy selection
- [ ] Add execution flow
- [ ] Implement error handling
- [ ] Add monitoring and metrics
- [ ] Add validation

### Phase 5: Integration
- [ ] Integrate with existing execution engine
- [ ] Add migration scripts
- [ ] Update documentation
- [ ] Add examples and tutorials
- [ ] Performance testing

---

*Last updated: April 13, 2026*
