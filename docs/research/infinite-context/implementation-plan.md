# Implementation Plan: Infinite Context for Whitt Execution Engine SDK

## Executive Summary

**Objective**: Integrate infinite context capabilities (RLM, RAG, Infini-Attention) into whitt-execution-engine SDK execution engine

**Timeline**: 3-phase implementation (6-8 weeks total)

**Primary Strategy**: RAG-first, RLM for complex synthesis

**Dependencies**: None (pure Rust integration)

## Current Architecture Analysis

### Existing Execution Engine

The whitt-execution-engine SDK currently has:

```rust
// Existing components
ExecutionEngine {
    models: ModelRegistry,
    orchestrator: WorkflowOrchestrator,
    scheduler: TaskScheduler,
    // Context handling: Currently limited to prompt tokens only
}

// Current context approach
struct TaskContext {
    prompt_tokens: usize,
    max_tokens: usize,  // Fixed limit (e.g., 128K)
    // No external memory, no compression
}
```

### Gaps Identified

1. **No external memory**: All context must fit in prompt
2. **No retrieval mechanism**: Can't access knowledge base
3. **No compression**: Large documents truncated or summarized
4. **No strategy selection**: Always uses same approach
5. **No optimization**: No token budgeting or smart chunking

## Proposed Architecture

### New Components

```
┌─────────────────────────────────────────────────────────┐
│         Execution Engine (Enhanced)                │
│                                                      │
│  ┌──────────────┐  ┌──────────────────┐    │
│  │ Context      │  │ Strategy         │    │
│  │ Orchestrator │──→│ Selector         │    │
│  └──────┬───────┘  └────────┬─────────┘    │
│         │                    │                │
│         ↓                    ↓                │
│  ┌──────────────┐  ┌──────────────────┐    │
│  │ RAG Backend  │  │ RLM Backend      │    │
│  │ (Vector DB)  │  │ (REPL + Sub-LLMs) │ │
│  └──────┬───────┘  └────────┬─────────┘    │
│         │                    │                │
│         └────────────────────┘                │
│                    │                         │
│                    ↓                         │
│           ┌──────────────────┐               │
│           │ LLM Backend     │               │
│           │ (Ollama/llama)│              │
│           └──────────────────┘               │
│                                                      │
└─────────────────────────────────────────────────────────┘
```

## Phase 1: Core Infrastructure (Weeks 1-2)

### 1.1 Add Context Orchestrator

**File**: `src/context/orchestrator.rs`

```rust
pub struct ContextOrchestrator {
    // Workspace configuration (backend availability)
    workspace_config: WorkspaceConfig,
    
    // Workflow-level overrides (optional, per-workflow settings)
    workflow_config: Option<WorkflowContextConfig>,
    
    // Backends (initialized based on availability)
    rlm_backend: Option<RLMBackend>,
    rag_backend: Option<RAGBackend>,
    llm_backend: Box<dyn LLMAdapter>,
}

// Schema: Under workflow.execution.context_orchestration
#[derive(Debug, Clone, Deserialize)]
pub struct WorkflowContextConfig {
    // Strategy thresholds
    pub rlm_threshold: usize,     // Use RLM if context > N tokens
    pub rag_threshold: usize,     // Use RAG if context > M tokens
    
    // Default strategy selection
    pub default_strategy: StrategyType,  // auto|direct|rag|rlm|hybrid
    
    // Strategy-specific settings
    pub rag_config: Option<RAGConfig>,
    pub rlm_config: Option<RLMConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum StrategyType {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "direct")]
    Direct,
    #[serde(rename = "rag")]
    Rag,
    #[serde(rename = "rlm")]
    Rlm,
    #[serde(rename = "hybrid")]
    Hybrid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RAGConfig {
    pub top_k: usize,
    pub score_threshold: Option<f32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RLMConfig {
    pub depth: usize,
}

// Schema: Under workspace.backends
#[derive(Debug, Clone, Deserialize)]
pub struct WorkspaceConfig {
    pub backends: BackendsConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BackendsConfig {
    pub ollama: Option<OllamaConfig>,
    pub llama_cpp: Option<LlamaCppConfig>,
    pub vector_store: Option<VectorStoreConfig>,
    pub rlm: Option<RLMBackendConfig>,
}

// No "enabled: true" flags - presence determines availability
```

**Implementation Steps**:

```rust
impl ContextOrchestrator {
    pub fn select_strategy(&self, task: &Task) -> Strategy {
        let context_size = self.estimate_context_size(&task.context);

        match task.task_type {
            TaskType::QuestionAnswering => {
                if context_size < self.config.rlm_threshold {
                    Strategy::Direct
                } else if context_size < self.config.rag_threshold {
                    Strategy::RAG { top_k: 5 }
                } else {
                    Strategy::RLM { depth: 1 }
                }
            },
            TaskType::DocumentAnalysis => {
                // Always use RLM for large documents
                if context_size > self.config.rlm_threshold {
                    Strategy::RLM { depth: 1 }
                } else {
                    Strategy::RAG { top_k: 10 }
                }
            },
            TaskType::CodeComprehension => {
                // Hybrid: RAG for API docs, RLM for implementation
                if self.has_code_docs(&task) {
                    Strategy::Hybrid
                } else {
                    Strategy::RAG { top_k: 7 }
                }
            },
            _ => Strategy::Direct,
        }
    }
}
```

### 1.2 Implement RAG Backend

**File**: `src/context/rag_backend.rs`

```rust
pub struct RAGBackend {
    embedder: Box<dyn EmbeddingProvider>,
    vector_store: Box<dyn VectorStore>,
    llm: Box<dyn LLMAdapter>,
    config: RAGConfig,
}

pub trait EmbeddingProvider {
    async fn embed(&self, text: &str) -> Result<Vec<f32>>;
    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>>;
}

pub trait VectorStore {
    async fn add(&mut self, chunks: &[Chunk]) -> Result<()>;
    async fn query(&self, embedding: &[f32], top_k: usize) -> Result<Vec<RetrievalResult>>;
}

#[derive(Debug)]
pub struct Chunk {
    pub id: String,
    pub text: String,
    pub embedding: Option<Vec<f32>>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug)]
pub struct RetrievalResult {
    pub chunk_id: String,
    pub text: String,
    pub metadata: HashMap<String, String>,
    pub score: f32,
}
```

**Ollama Embeddings Implementation**:

```rust
pub struct OllamaEmbeddings {
    client: reqwest::Client,
    endpoint: String,
    model: String,
}

impl EmbeddingProvider for OllamaEmbeddings {
    async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let response = self.client
            .post(format!("{}/api/embeddings", self.endpoint))
            .json(&json!({
                "model": self.model,
                "prompt": text
            }))
            .send()
            .await?;

        let data: OllamaEmbeddingResponse = response.json().await?;
        Ok(data.embedding)
    }

    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        // Process in parallel for speed
        let futures: Vec<_> = texts
            .iter()
            .map(|text| self.embed(text))
            .collect();

        let results = futures::future::join_all(futures).await;
        results.into_iter().collect()
    }
}
```

### 1.3 Implement ChromaDB Vector Store

**File**: `src/context/vector_store/chroma.rs`

```rust
use chroma::Client;

pub struct ChromaVectorStore {
    client: Client,
    collection_name: String,
}

impl VectorStore for ChromaVectorStore {
    async fn add(&mut self, chunks: &[Chunk]) -> Result<()> {
        let collection = self.client.get_collection(&self.collection_name).await?;

        for chunk in chunks {
            collection.add(
                vec![chunk.id.clone()],
                chunk.embedding.clone().unwrap_or_default(),
                None,
                vec![chunk.text.clone()],
                Some(chunk.metadata.clone()),
            ).await?;
        }

        Ok(())
    }

    async fn query(&self, embedding: &[f32], top_k: usize) -> Result<Vec<RetrievalResult>> {
        let collection = self.client.get_collection(&self.collection_name).await?;

        let results = collection
            .query(
                Some(vec![embedding.to_vec()]),
                Some(top_k),
                None,
                None,
            )
            .await?;

        let retrieval_results = results
            .documents
            .unwrap_or_default()
            .into_iter()
            .zip(results.metadatas.unwrap_or_default())
            .zip(results.distances.unwrap_or_default())
            .map(|((text, metadata), score)| RetrievalResult {
                chunk_id: metadata.get("id").unwrap_or(&String::new()).clone(),
                text,
                metadata,
                score,
            })
            .collect();

        Ok(retrieval_results)
    }
}
```

### 1.4 Document Ingestion Pipeline

**File**: `src/context/ingestion.rs`

```rust
pub struct DocumentIngester {
    chunker: Box<dyn Chunker>,
    embedder: Box<dyn EmbeddingProvider>,
    vector_store: Box<dyn VectorStore>,
}

pub trait Chunker {
    fn chunk(&self, text: &str) -> Result<Vec<String>>;
}

pub struct FixedSizeChunker {
    size: usize,
    overlap: usize,
}

impl Chunker for FixedSizeChunker {
    fn chunk(&self, text: &str) -> Result<Vec<String>> {
        let mut chunks = Vec::new();
        let chars: Vec<char> = text.chars().collect();

        for i in (0..chars.len()).step_by(self.size - self.overlap) {
            let end = (i + self.size).min(chars.len());
            let chunk: String = chars[i..end].iter().collect();
            chunks.push(chunk);
        }

        Ok(chunks)
    }
}

impl DocumentIngester {
    pub async fn ingest_document(&mut self, filepath: &Path) -> Result<usize> {
        let content = fs::read_to_string(filepath)?;
        let chunks = self.chunker.chunk(&content)?;

        // Generate embeddings
        let embeddings = self.embedder.embed_batch(&chunks).await?;

        // Create chunk objects
        let chunk_objects: Vec<Chunk> = chunks
            .into_iter()
            .zip(embeddings)
            .enumerate()
            .map(|(i, (text, embedding))| Chunk {
                id: format!("{}_{}", filepath.display(), i),
                text,
                embedding: Some(embedding),
                metadata: {
                    let mut map = HashMap::new();
                    map.insert("source".to_string(), filepath.display().to_string());
                    map.insert("chunk_index".to_string(), i.to_string());
                    map
                },
            })
            .collect();

        // Add to vector store
        self.vector_store.add(&chunk_objects).await?;

        Ok(chunk_objects.len())
    }
}
```

## Phase 2: RLM Integration (Weeks 3-5)

### 2.1 Implement RLM Backend

**File**: `src/context/rlm_backend.rs`

```rust
pub struct RLMBackend {
    env: REPLEnvironment,
    llm: Box<dyn LLMAdapter>,
    config: RLMConfig,
    system_prompt: String,
}

#[derive(Debug, Clone)]
pub struct RLMConfig {
    pub max_depth: usize,
    pub token_budget: usize,
    pub timeout_ms: u64,
    pub repl_type: REPLType,
}

#[derive(Debug, Clone)]
pub enum REPLType {
    Local,        // Python REPL on host
    Docker,       // Containerized Python
    Restricted,   // RestrictedPython library
}
```

**RQLM System Prompt** (from MIT paper):

```rust
const RLM_SYSTEM_PROMPT: &str = r#"
You are operating in a Python REPL environment with access to these tools:

- peek(range_start, range_end): Read a slice of the 'context' variable
- chunk(delimiter): Split the 'context' variable into sections
- llm_query(query, context_slice): Call a sub-language model with focused context
- FINAL(result): Return your final answer and stop recursion

The full context is stored in a variable named 'context'. DO NOT include the entire
context in your prompt. Your job is to decompose the task, retrieve relevant context
using the tools above, and then call sub-LLMs for focused reasoning.

Guidelines:
- Start with planning: What chunks do I need to examine?
- Use peek() to examine small relevant sections (1K-10K tokens)
- Use chunk() to identify document structure
- Call llm_query() for each chunk that needs analysis
- Be selective: Don't analyze chunks that aren't relevant to the query
- Synthesize: Combine sub-results into a coherent final answer
"#;
```

### 2.2 REPL Environment

**File**: `src/context/repl.rs`

```rust
pub struct REPLEnvironment {
    python: PythonInterpreter,
    variables: HashMap<String, VariableValue>,
    history: Vec<ExecutionResult>,
}

pub enum VariableValue {
    String(String),
    Number(f64),
    List(Vec<VariableValue>),
}

pub struct PythonInterpreter {
    process: Option<Child>,
}

impl REPLEnvironment {
    pub fn new() -> Result<Self> {
        let python = PythonInterpreter::new()?;
        Ok(REPLEnvironment {
            python,
            variables: HashMap::new(),
            history: Vec::new(),
        })
    }

    pub fn set_variable(&mut self, name: &str, value: VariableValue) {
        self.variables.insert(name.to_string(), value);
    }

    pub fn peek(&self, var_name: &str, range_start: usize, range_end: usize) -> Result<String> {
        match self.variables.get(var_name) {
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
        match self.variables.get(var_name) {
            Some(VariableValue::String(text)) => {
                Ok(text.split(delimiter).map(|s| s.to_string()).collect())
            },
            _ => Err(Error::InvalidType),
        }
    }

    pub async fn execute(&mut self, code: &str) -> Result<String> {
        let output = self.python.execute(code).await?;

        self.history.push(ExecutionResult {
            code: code.to_string(),
            output: output.clone(),
            timestamp: SystemTime::now(),
        });

        Ok(output)
    }
}
```

### 2.3 RLM Execution Flow

```rust
impl RLMBackend {
    pub async fn execute(&mut self, task: Task) -> Result<TaskResult> {
        // 1. Load context into REPL
        self.env.set_variable("context", VariableValue::String(task.context_source));

        // 2. Generate RLM prompt
        let prompt = format!(
            "{}\n\nTask: {}",
            self.system_prompt,
            task.query
        );

        // 3. Execute with depth=1 (CRITICAL)
        let (answer, stats) = self.execute_with_depth(prompt, 1).await?;

        Ok(TaskResult {
            task_id: task.id,
            answer,
            strategy: Strategy::RLM {
                depth: 1,
                subcalls: stats.sub_llm_calls,
            },
            stats,
            sources: vec![],
        })
    }

    async fn execute_with_depth(
        &mut self,
        prompt: String,
        depth: usize,
    ) -> Result<(String, ExecutionStats)> {
        let mut stats = ExecutionStats::default();
        stats.total_tokens = self.llm.token_count(&prompt);
        stats.llm_tokens += stats.total_tokens;
        stats.sub_llm_calls += 1;

        // Generate code
        let code_response = self.llm.complete(&prompt).await?;
        stats.total_tokens += self.llm.token_count(&code_response);

        // Check for FINAL call
        if code_response.contains("FINAL(") {
            // Extract and return final answer
            let answer = extract_final_answer(&code_response)?;
            Ok((answer, stats))
        } else {
            // Execute code in REPL
            let output = self.env.execute(&code_response).await?;

            // Parse for llm_query calls and execute sub-LLMs
            let sub_queries = parse_llm_queries(&code_response);
            let mut sub_results = Vec::new();

            for query in sub_queries {
                let sub_answer = self.llm.complete(&query).await?;
                stats.total_tokens += self.llm.token_count(&sub_answer);
                stats.llm_tokens += self.llm.token_count(&sub_answer);
                stats.sub_llm_calls += 1;
                sub_results.push(sub_answer);
            }

            // Continue recursion if depth > 0
            if depth > 0 {
                let updated_code = replace_llm_queries(&code_response, &sub_results);
                self.execute_with_depth(updated_code, depth - 1).await
            } else {
                Err(Error::MaxDepthExceeded)
            }
        }
    }
}
```

### 2.4 Token Budgeting

```rust
pub struct TokenBudget {
    total: usize,
    used: usize,
}

impl TokenBudget {
    pub fn new(total: usize) -> Self {
        TokenBudget { total, used: 0 }
    }

    pub fn check(&self, estimated: usize) -> bool {
        self.used + estimated <= self.total
    }

    pub fn reserve(&mut self, amount: usize) -> Result<()> {
        if self.used + amount > self.total {
            return Err(Error::TokenBudgetExceeded {
                budget: self.total,
                used: self.used + amount,
            });
        }
        self.used += amount;
        Ok(())
    }
}

// Usage in RLM
impl RLMBackend {
    async fn execute_with_depth(&mut self, prompt: String, depth: usize) -> Result<...> {
        if !self.budget.check(self.llm.token_count(&prompt)) {
            return Err(Error::TokenBudgetExceeded { ... });
        }

        self.budget.reserve(self.llm.token_count(&prompt))?;

        // ... rest of execution
    }
}
```

## Phase 3: Integration (Weeks 6-8)

### 3.1 Enhance Execution Engine

**File**: `src/engine/execution.rs`

```rust
pub struct ExecutionEngine {
    models: ModelRegistry,
    orchestrator: WorkflowOrchestrator,
    context_orchestrator: ContextOrchestrator,  // NEW
    scheduler: TaskScheduler,
}

impl ExecutionEngine {
    pub async fn execute_task(&mut self, task: Task) -> Result<TaskResult> {
        // 1. Check if context needs special handling
        let strategy = self.context_orchestrator.select_strategy(&task);

        match strategy {
            Strategy::Direct => {
                // Existing direct execution
                self.orchestrator.execute_workflow(task).await
            },
            Strategy::RAG { top_k } => {
                // NEW: Use RAG backend
                self.execute_with_rag(task, top_k).await
            },
            Strategy::RLM { depth } => {
                // NEW: Use RLM backend
                self.execute_with_rlm(task, depth).await
            },
            Strategy::Hybrid => {
                // NEW: Use both RAG and RLM
                self.execute_hybrid(task).await
            },
        }
    }

    async fn execute_with_rag(&mut self, task: Task, top_k: usize) -> Result<TaskResult> {
        let rag_backend = self.context_orchestrator.rag_backend.as_ref()
            .ok_or(Error::BackendNotInitialized)?;

        // 1. Embed query
        let query_embedding = rag_backend.embedder.embed(&task.query).await?;

        // 2. Retrieve relevant chunks
        let retrieved = rag_backend.vector_store.query(&query_embedding, top_k).await?;

        // 3. Build context from retrieved chunks
        let context = retrieved
            .iter()
            .map(|r| format!("Source: {}\n{}\n", r.chunk_id, r.text))
            .collect::<Vec<_>>()
            .join("\n\n---\n\n");

        // 4. Execute with retrieved context
        let enhanced_task = Task {
            context_source: ContextSource::Direct(context),
            ..task
        };

        self.orchestrator.execute_workflow(enhanced_task).await
    }

    async fn execute_with_rlm(&mut self, task: Task, depth: usize) -> Result<TaskResult> {
        let rlm_backend = self.context_orchestrator.rlm_backend.as_ref()
            .ok_or(Error::BackendNotInitialized)?;

        // Use RLM backend directly
        rlm_backend.execute(task).await
    }

    async fn execute_hybrid(&mut self, task: Task) -> Result<TaskResult> {
        // 1. Use RAG to retrieve relevant context
        let rag_backend = self.context_orchestrator.rag_backend.as_ref()
            .ok_or(Error::BackendNotInitialized)?;

        let query_embedding = rag_backend.embedder.embed(&task.query).await?;
        let retrieved = rag_backend.vector_store.query(&query_embedding, 5).await?;

        // 2. Use RLM for complex synthesis with retrieved context
        let rlm_backend = self.context_orchestrator.rlm_backend.as_ref()
            .ok_or(Error::BackendNotInitialized)?;

        let context_from_rag = retrieved
            .iter()
            .map(|r| r.text.clone())
            .collect::<Vec<_>>()
            .join("\n\n");

        let enhanced_task = Task {
            context_source: ContextSource::Direct(context_from_rag),
            ..task
        };

        rlm_backend.execute(enhanced_task).await
    }
}
```

### 3.2 Configuration Schema

**Schema Placement**: Context orchestration under `workflow.execution`, backends under `workspace`

**Workspace Configuration** (`workspace.yaml`):

```yaml
workspace:
  # Backend availability (presence = enabled, no "enabled" flags)
  backends:
    ollama:
      endpoint: http://localhost:11434
      default_model: llama3.2
      timeout_ms: 30000
      max_retries: 3
    
    llama_cpp:
      library_path: /usr/local/lib/libllama.so
      model_path: /models/llama-3.2-q4.gguf
      num_gpu_layers: 33
      context_size: 16384
      gpu_backend: Vulkan  # Vulkan|CUDA|ROCm|Metal
    
    vector_store:
      type: chroma  # chroma|faiss|weaviate
      chroma:
        path: ./chroma_db
        collection_name: documents
      faiss:
        dimension: 768
        index_type: flat  # flat|ivf|hnsw
    
    rlm:
      max_depth: 1  # CRITICAL: Always 1
      token_budget: 1000000  # 1M token limit
      repl_type: local  # local|docker|restricted
      timeout_ms: 60000
```

**Workflow Configuration** (`workflow.yaml`):

```yaml
workflow:
  execution:
    mode: parallel|serial|hybrid
    
    # Context orchestration (under execution, not top-level)
    context_orchestration:
      # Strategy thresholds
      rlm_threshold: 100000      # 100K tokens → use RLM
      rag_threshold: 50000       # 50K tokens → use RAG
      
      # Default strategy
      default_strategy: auto       # auto|direct|rag|rlm|hybrid
      
      # Strategy-specific settings (optional)
      rag_config:
        top_k: 5
        score_threshold: 0.3
      
      rlm_config:
        depth: 1  # Should always be 1
```

**Key Differences**:
- **Schema placement**: `context_orchestration` under `workflow.execution`, not top-level
- **Backend availability**: `workspace.backends`, no `enabled: true` flags
- **Presence-based**: Backend available if present in workspace config
- **Fallback behavior**: Orchestrator falls back to direct if requested backend unavailable

### 3.3 CLI Extensions

**File**: `src/cli/context_commands.rs`

```rust
use clap::{Subcommand, Args};

#[derive(Subcommand)]
pub enum ContextCommand {
    /// Index documents into vector store
    Index {
        #[arg(short, long)]
        path: PathBuf,

        #[arg(long, default_value = "512")]
        chunk_size: usize,

        #[arg(long, default_value = "64")]
        overlap: usize,
    },

    /// Query the knowledge base
    Query {
        #[arg(short, long)]
        question: String,

        #[arg(long, default_value = "5")]
        top_k: usize,
    },

    /// Execute task with RLM
    Rlm {
        #[arg(short, long)]
        prompt: String,

        #[arg(short, long)]
        context: PathBuf,

        #[arg(long, default_value = "1")]
        depth: usize,
    },
}

pub async fn handle_context_command(command: ContextCommand) -> Result<()> {
    match command {
        ContextCommand::Index { path, chunk_size, overlap } => {
            index_documents(path, chunk_size, overlap).await?;
            println!("✓ Indexed documents");
        },
        ContextCommand::Query { question, top_k } => {
            let answer = query_knowledge_base(&question, top_k).await?;
            println!("Answer: {}", answer);
        },
        ContextCommand::Rlm { prompt, context, depth } => {
            let result = execute_rlm_task(&prompt, &context, depth).await?;
            println!("RLM Result:\n{}", result);
        },
    }

    Ok(())
}
```

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rag_retrieval() {
        let vector_store = create_test_vector_store();
        let embedder = MockEmbeddings::new();

        // Add test chunks
        let chunks = vec![
            Chunk {
                id: "chunk1".to_string(),
                text: "Test document content".to_string(),
                ..Default::default()
            },
        ];

        vector_store.add(&chunks).await.unwrap();

        // Query
        let embedding = embedder.embed("test query").await.unwrap();
        let results = vector_store.query(&embedding, 5).await.unwrap();

        assert_eq!(results.len(), 1);
        assert!(results[0].score > 0.5);
    }

    #[tokio::test]
    async fn test_rlm_depth_control() {
        let rlm = create_test_rlm_backend();
        let task = create_test_task();

        let result = rlm.execute(task).await.unwrap();

        // Verify depth=1 (max subcalls should be small)
        assert!(result.stats.sub_llm_calls < 10);
    }

    #[tokio::test]
    async fn test_token_budget() {
        let budget = TokenBudget::new(1000);

        assert!(budget.check(500));
        budget.reserve(500).unwrap();

        assert!(!budget.check(600));
        assert!(budget.reserve(600).is_err());
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_full_rag_pipeline() {
    // 1. Index documents
    let mut ingester = create_test_ingester();
    ingester.ingest_document(Path::new("test.txt")).await.unwrap();

    // 2. Query
    let engine = create_test_engine();
    let task = Task {
        id: "test1".to_string(),
        task_type: TaskType::QuestionAnswering,
        query: "What is the main topic?".to_string(),
        context_source: ContextSource::Direct("".to_string()),
        config: TaskConfig::default(),
    };

    let result = engine.execute_task(task).await.unwrap();

    assert!(!result.answer.is_empty());
    assert!(result.sources.len() > 0);
}

#[tokio::test]
async fn test_strategy_selection() {
    let orchestrator = create_test_orchestrator();

    // Small context → Direct
    let small_task = create_test_task_with_context(1000);
    let strategy1 = orchestrator.select_strategy(&small_task);
    assert!(matches!(strategy1, Strategy::Direct));

    // Medium context → RAG
    let medium_task = create_test_task_with_context(50000);
    let strategy2 = orchestrator.select_strategy(&medium_task);
    assert!(matches!(strategy2, Strategy::RAG { .. }));

    // Large context → RLM
    let large_task = create_test_task_with_context(200000);
    let strategy3 = orchestrator.select_strategy(&large_task);
    assert!(matches!(strategy3, Strategy::RLM { .. }));
}
```

### Performance Benchmarks

```rust
#[cfg(test)]
mod benchmarks {
    use super::*;

    #[tokio::test]
    async fn benchmark_rag_latency() {
        let engine = create_test_engine();
        let task = create_test_task_with_context(100000);

        let start = Instant::now();
        let result = engine.execute_task(task).await.unwrap();
        let latency = start.elapsed();

        println!("RAG latency: {:?}", latency);
        assert!(latency < Duration::from_secs(1));
    }

    #[tokio::test]
    async fn benchmark_rlm_latency() {
        let engine = create_test_engine();
        let task = create_test_task_with_context(500000);

        let start = Instant::now();
        let result = engine.execute_task(task).await.unwrap();
        let latency = start.elapsed();

        println!("RLM latency: {:?}", latency);
        assert!(latency < Duration::from_secs(60)); // Allow 1 minute
    }

    #[tokio::test]
    async fn benchmark_memory_usage() {
        let engine = create_test_engine();
        let task = create_test_task_with_context(1000000);

        let memory_before = get_memory_usage();
        let _result = engine.execute_task(task).await.unwrap();
        let memory_after = get_memory_usage();

        let memory_used = memory_after - memory_before;
        println!("Memory used: {} MB", memory_used / (1024 * 1024));
        assert!(memory_used < 2_000_000_000); // Less than 2GB
    }
}
```

## Deployment Checklist

### Development Environment
- [ ] Install Rust toolchain
- [ ] Install Ollama (`curl -fsSL https://ollama.ai/install.sh | sh`)
- [ ] Install llama.cpp with Vulkan support
- [ ] Install ChromaDB (`pip install chromadb`)
- [ ] Clone whitt-execution-engine SDK

### Configuration
- [ ] Set up `config/context_engine.yaml`
- [ ] Configure Ollama endpoint
- [ ] Select appropriate thresholds
- [ ] Set token budgets
- [ ] Choose vector store type

### Testing
- [ ] Run unit tests (`cargo test`)
- [ ] Run integration tests (`cargo test --test integration`)
- [ ] Run benchmarks (`cargo test --release --test benchmarks`)
- [ ] Manual testing with sample documents
- [ ] Load testing with concurrent queries

### Production
- [ ] Configure monitoring and metrics
- [ ] Set up logging (Rust log crate)
- [ ] Configure token budgets and cost tracking
- [ ] Set up vector store persistence
- [ ] Configure backup and recovery
- [ ] Document configuration and procedures

## Timeline Summary

| Week | Milestone | Deliverables |
|------|-----------|--------------|
| 1-2 | Core Infrastructure | Context orchestrator, RAG backend, vector store |
| 3-4 | RLM Backend | REPL environment, RLM execution flow, token budgeting |
| 5-6 | Integration | Enhanced execution engine, configuration schema, CLI |
| 7-8 | Testing & Polish | Unit tests, integration tests, benchmarks, documentation |

## Success Criteria

- [ ] RAG backend successfully indexes and retrieves documents
- [ ] RLM backend executes with depth=1 without timeout
- [ ] Context orchestrator correctly selects strategies
- [ ] Execution engine integrates both backends seamlessly
- [ ] Token budgeting prevents runaway costs
- [ ] Latency meets targets (RAG < 1s, RLM < 60s)
- [ ] Memory usage stays within limits (< 2GB for 1M tokens)
- [ ] All tests pass with > 90% coverage
- [ ] Documentation complete with examples

---

*Last updated: April 13, 2026*
