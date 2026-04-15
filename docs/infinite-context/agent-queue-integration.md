# Integration with Agent-Queue

## Executive Summary

**Objective**: Integrate infinite context capabilities into agent-queue architecture

**Key Insight**: agent-queue already orchestrates multi-step workflows; infinite context extends this by adding context-aware strategy selection

**Approach**: Layer infinite context over existing agent-queue as a context-aware middleware

## Current Agent-Queue Architecture

### Existing Flow

```rust
// Current agent-queue execution
AgentQueue {
    agents: Vec<Box<dyn Agent>>,
    scheduler: TaskScheduler,
}

impl AgentQueue {
    async fn process_task(&mut self, task: Task) -> Result<Output> {
        // 1. Select agent
        let agent = self.select_agent(&task);

        // 2. Execute agent (direct LLM call)
        let output = agent.execute(task).await?;

        // 3. Return output
        Ok(output)
    }
}

// Problem: All context must fit in single LLM call
// No retrieval, no compression, no strategy selection
```

### Limitations

1. **Fixed context window**: Agent prompts limited to model's context
2. **No retrieval**: Can't access knowledge base
3. **No optimization**: Always same approach regardless of task size
4. **No memory**: Each agent call starts fresh

## Proposed Integration

### Layered Architecture

```
┌─────────────────────────────────────────────────────┐
│                 Client Request                │
└──────────────────┬──────────────────────────┘
                   │
                   ↓
┌─────────────────────────────────────────────────────┐
│            Context Orchestrator                │
│  - Estimate context size                    │
│  - Select strategy (RAG/RLM/Direct)      │
│  - Prepare context for agent              │
└──────────────────┬──────────────────────────┘
                   │
                   ↓
┌─────────────────────────────────────────────────────┐
│                Agent-Queue                   │
│  - Select agent based on task              │
│  - Execute agent with enhanced context     │
│  - Handle multi-agent workflows          │
└──────────────────┬──────────────────────────┘
                   │
                   ↓
┌─────────────────────────────────────────────────────┐
│             LLM Backend (Ollama)             │
└─────────────────────────────────────────────────────┘
```

### Enhanced Task Schema

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedTask {
    // Existing fields
    pub id: String,
    pub task_type: TaskType,
    pub query: String,
    pub agent_type: AgentType,

    // NEW: Context fields
    pub context_source: ContextSource,
    pub context_size: Option<usize>,
    pub preferred_strategy: Option<Strategy>,

    // NEW: Metadata for orchestration
    pub allow_rag: bool,
    pub allow_rlm: bool,
    pub max_tokens: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContextSource {
    Direct(String),
    File(PathBuf),
    VectorStore {
        collection: String,
        top_k: usize,
    },
    Environment(String),  // From previous agent output
    Hybrid {
        rag: bool,
        rlm: bool,
    },
}
```

## Integration Patterns

### Pattern 1: Context-Aware Agent Selection

**Scenario**: User asks "Analyze this codebase and find security vulnerabilities"

```rust
impl AgentQueue {
    async fn process_task_with_context(&mut self, task: EnhancedTask) -> Result<Output> {
        // 1. Analyze task context
        let context_size = estimate_context_size(&task.context_source);

        // 2. Select appropriate agent
        let agent = if context_size > 100_000 {
            // Large context: Use RLM-capable agent
            self.select_agent_with_capability("rlm")
        } else if context_size > 50_000 {
            // Medium context: Use RAG-capable agent
            self.select_agent_with_capability("rag")
        } else {
            // Small context: Use standard agent
            self.select_agent(&task)
        };

        // 3. Prepare enhanced context
        let enhanced_context = match task.preferred_strategy {
            Some(Strategy::RAG { top_k }) => {
                self.prepare_rag_context(&task, top_k).await?
            },
            Some(Strategy::RLM { depth }) => {
                self.prepare_rlm_context(&task, depth).await?
            },
            _ => task.context_source.clone(),
        };

        // 4. Execute agent with enhanced context
        let enhanced_task = EnhancedTask {
            context_source: enhanced_context,
            ..task
        };

        agent.execute(enhanced_task).await
    }
}
```

### Pattern 2: Agent Chaining with Context Pass-Through

**Scenario**: Multi-agent workflow where context grows

```rust
impl AgentQueue {
    async fn execute_workflow_with_context(
        &mut self,
        workflow: Workflow,
        initial_context: ContextSource,
    ) -> Result<Vec<Output>> {
        let mut results = Vec::new();
        let mut current_context = initial_context;

        for step in workflow.steps {
            // Prepare task with current context
            let task = EnhancedTask {
                id: format!("{}-{}", workflow.id, step.id),
                task_type: step.task_type.clone(),
                query: step.prompt.clone(),
                agent_type: step.agent_type.clone(),
                context_source: current_context.clone(),
                allow_rag: step.allow_rag,
                allow_rlm: step.allow_rlm,
                ..Default::default()
            };

            // Execute task
            let output = self.process_task_with_context(task).await?;
            results.push(output.clone());

            // Update context for next agent
            current_context = self.update_context(
                current_context,
                output.clone(),
                step.context_update,
            );
        }

        Ok(results)
    }

    fn update_context(
        &self,
        current_context: ContextSource,
        output: Output,
        update_policy: ContextUpdatePolicy,
    ) -> ContextSource {
        match update_policy {
            ContextUpdatePolicy::Append => {
                // Append agent output to context
                if let ContextSource::Direct(mut text) = current_context {
                    text.push_str("\n\n---\n\n");
                    text.push_str(&output.content);
                    ContextSource::Direct(text)
                } else {
                    current_context
                }
            },
            ContextUpdatePolicy::Replace => {
                // Replace with agent output
                ContextSource::Direct(output.content)
            },
            ContextUpdatePolicy::Retrieval => {
                // Use agent output as retrieval query
                ContextSource::VectorStore {
                    collection: "workflow_context".to_string(),
                    top_k: 5,
                }
            },
            ContextUpdatePolicy::Rlm => {
                // Use RLM for synthesis
                ContextSource::Direct(output.content)
            },
        }
    }
}
```

### Pattern 3: Hybrid RAG + RLM for Complex Workflows

**Scenario**: Code review workflow with both API docs (RAG) and implementation analysis (RLM)

```rust
#[derive(Debug, Clone)]
pub struct HybridAgent {
    rag_backend: RAGBackend,
    rlm_backend: RLMBackend,
    llm: Box<dyn LLMAdapter>,
}

impl Agent for HybridAgent {
    async fn execute(&self, task: EnhancedTask) -> Result<Output> {
        match (task.allow_rag, task.allow_rlm) {
            (true, true) => {
                // Hybrid: RAG for retrieval, RLM for synthesis
                self.execute_hybrid(task).await
            },
            (true, false) => {
                // RAG only
                self.execute_with_rag(task).await
            },
            (false, true) => {
                // RLM only
                self.execute_with_rlm(task).await
            },
            (false, false) => {
                // Direct LLM
                self.execute_direct(task).await
            },
        }
    }
}

impl HybridAgent {
    async fn execute_hybrid(&self, task: EnhancedTask) -> Result<Output> {
        // 1. Use RAG to retrieve relevant context
        let query_embedding = self.rag_backend.embedder.embed(&task.query).await?;
        let retrieved = self.rag_backend.vector_store.query(&query_embedding, 10).await?;

        // 2. Prepare context from retrieved documents
        let rag_context = retrieved
            .iter()
            .map(|r| r.text.clone())
            .collect::<Vec<_>>()
            .join("\n\n---\n\n");

        // 3. Use RLM for complex analysis with retrieved context
        let rlm_task = EnhancedTask {
            context_source: ContextSource::Direct(rag_context),
            ..task
        };

        self.rlm_backend.execute(rlm_task).await
    }
}
```

## Workflow Examples

### Example 1: Document Analysis with Agent-Queue

**YAML Workflow Definition**:

```yaml
workflow:
  id: doc-analysis
  name: Document Analysis with RAG
  steps:
    - id: retrieve
      agent_type: rag-capable
      prompt: "What are the key findings in this document?"
      allow_rag: true
      allow_rlm: false
      context_update: retrieval

    - id: summarize
      agent_type: rlm-capable
      prompt: "Synthesize the retrieved findings into a summary"
      allow_rag: false
      allow_rlm: true
      context_update: rlm
```

**Execution**:

```rust
async fn execute_document_analysis(
    queue: &mut AgentQueue,
    document_path: &Path,
) -> Result<String> {
    // 1. Index document (one-time setup)
    let ingester = DocumentIngester::new();
    ingester.ingest_document(document_path).await?;

    // 2. Execute workflow
    let workflow = load_workflow("doc-analysis.yaml")?;
    let context = ContextSource::VectorStore {
        collection: "documents".to_string(),
        top_k: 5,
    };

    let results = queue.execute_workflow_with_context(workflow, context).await?;

    // 3. Extract final summary
    let summary = &results.last().unwrap().content;
    Ok(summary.clone())
}
```

### Example 2: Code Review Workflow

**YAML Workflow Definition**:

```yaml
workflow:
  id: code-review
  name: Code Review with Hybrid Approach
  steps:
    - id: retrieve-api-docs
      agent_type: rag-capable
      prompt: "Retrieve API documentation for this code"
      allow_rag: true
      allow_rlm: false
      context_update: append

    - id: analyze-implementation
      agent_type: rlm-capable
      prompt: "Analyze implementation against retrieved API docs and find issues"
      allow_rag: false
      allow_rlm: true
      context_update: rlm

    - id: generate-report
      agent_type: standard
      prompt: "Generate a code review report from the analysis"
      allow_rag: false
      allow_rlm: false
      context_update: replace
```

**Execution**:

```rust
async fn execute_code_review(
    queue: &mut AgentQueue,
    codebase_path: &Path,
) -> Result<CodeReviewReport> {
    // 1. Index API docs and codebase
    let mut ingester = DocumentIngester::new();
    for doc in find_api_docs(codebase_path) {
        ingester.ingest_document(&doc).await?;
    }

    // 2. Execute workflow
    let workflow = load_workflow("code-review.yaml")?;
    let context = ContextSource::File(codebase_path.to_path_buf());

    let results = queue.execute_workflow_with_context(workflow, context).await?;

    // 3. Parse code review report
    let report: CodeReviewReport = serde_json::from_str(&results.last().unwrap().content)?;
    Ok(report)
}
```

### Example 3: Research Question with RLM

**YAML Workflow Definition**:

```yaml
workflow:
  id: research
  name: Deep Research with RLM
  steps:
    - id: initial-analysis
      agent_type: rlm-capable
      prompt: "Analyze this research corpus and extract key themes"
      allow_rag: false
      allow_rlm: true
      max_tokens: 100000
      context_update: rlm

    - id: detailed-analysis
      agent_type: rlm-capable
      prompt: "Based on initial themes, perform detailed analysis of each theme"
      allow_rag: false
      allow_rlm: true
      max_tokens: 200000
      context_update: rlm
```

**Execution**:

```rust
async fn execute_research(
    queue: &mut AgentQueue,
    research_corpus: &Path,
) -> Result<ResearchReport> {
    // 1. Load corpus into RLM environment
    let corpus_content = fs::read_to_string(research_corpus)?;

    // 2. Execute workflow
    let workflow = load_workflow("research.yaml")?;
    let context = ContextSource::Direct(corpus_content);

    let results = queue.execute_workflow_with_context(workflow, context).await?;

    // 3. Synthesize research report
    let report: ResearchReport = serde_json::from_str(&results.last().unwrap().content)?;
    Ok(report)
}
```

## Configuration Integration

### Agent-Queue Configuration

**File**: `config/agent-queue.yaml`

```yaml
agent_queue:
  # Context orchestration
  context_orchestration:
    enabled: true
    rlm_threshold: 100000
    rag_threshold: 50000
    default_strategy: auto  # auto, direct, rag, rlm, hybrid

  # Backend configuration
  backends:
    ollama:
      enabled: true
      endpoint: http://localhost:11434
      default_model: llama3.2

    vector_store:
      type: chroma
      path: ./chroma_db
      collection_name: agent_queue_context

    rlm:
      enabled: true
      max_depth: 1
      token_budget: 1000000

  # Agent capabilities
  agents:
    standard:
      type: llm
      capabilities:
        - direct_execution

    rag-capable:
      type: llm
      capabilities:
        - direct_execution
        - rag_retrieval
      default_strategy: rag

    rlm-capable:
      type: llm
      capabilities:
        - direct_execution
        - rlm_decomposition
      default_strategy: rlm

    hybrid:
      type: llm
      capabilities:
        - direct_execution
        - rag_retrieval
        - rlm_decomposition
      default_strategy: hybrid

  # Workflow configuration
  workflows:
    default_context_update: append
    max_context_size: 5000000
    enable_context_sharing: true
```

## Performance Considerations

### Latency vs Accuracy Tradeoff

```rust
impl AgentQueue {
    pub async fn optimize_for_latency(&mut self, task: EnhancedTask) -> Result<Output> {
        // Use RAG with smaller top_k for faster retrieval
        let rag_task = EnhancedTask {
            context_source: ContextSource::VectorStore {
                collection: "documents".to_string(),
                top_k: 3,  // Reduced from 5
            },
            preferred_strategy: Some(Strategy::RAG { top_k: 3 }),
            ..task
        };

        self.process_task_with_context(rag_task).await
    }

    pub async fn optimize_for_accuracy(&mut self, task: EnhancedTask) -> Result<Output> {
        // Use RLM with higher token budget for better synthesis
        let rlm_task = EnhancedTask {
            max_tokens: Some(200000),  // Increased from default
            preferred_strategy: Some(Strategy::RLM { depth: 1 }),
            ..task
        };

        self.process_task_with_context(rlm_task).await
    }
}
```

### Context Management

```rust
impl AgentQueue {
    async fn manage_context_memory(&mut self) -> Result<()> {
        // 1. Monitor context size
        let total_context_size = self.calculate_total_context_size();

        if total_context_size > MAX_CONTEXT_MEMORY {
            // 2. Prune old context
            self.prune_old_context();

            // 3. Re-index if using RAG
            if self.rag_backend.is_some() {
                self.rerank_vector_store().await?;
            }
        }

        Ok(())
    }

    fn prune_old_context(&mut self) {
        // Remove context older than 1 hour
        let cutoff = SystemTime::now() - Duration::from_secs(3600);

        self.workflow_context.retain(|entry| entry.timestamp > cutoff);
    }
}
```

## Error Handling

```rust
impl AgentQueue {
    async fn process_task_with_fallback(
        &mut self,
        task: EnhancedTask,
    ) -> Result<Output> {
        // Try preferred strategy first
        match self.process_task_with_context(task.clone()).await {
            Ok(output) => Ok(output),
            Err(Error::TokenBudgetExceeded { .. }) => {
                // Fallback: Reduce context size
                log::warn!("Token budget exceeded, reducing context");
                let reduced_task = self.reduce_context(task);
                self.process_task_with_context(reduced_task).await
            },
            Err(Error::Timeout { operation, .. }) => {
                // Fallback: Switch to faster strategy
                log::warn!("Timeout in {}, switching strategy", operation);
                let fallback_task = self.switch_to_faster_strategy(task);
                self.process_task_with_context(fallback_task).await
            },
            Err(e) => {
                // Final fallback: Direct LLM call
                log::error!("All strategies failed, using direct: {}", e);
                self.execute_direct(task).await
            },
        }
    }
}
```

## Monitoring and Metrics

```rust
#[derive(Debug, Clone, Serialize)]
pub struct ContextMetrics {
    pub tasks_processed: usize,
    pub strategy_usage: HashMap<String, usize>,
    pub average_latency_ms: u64,
    pub average_tokens_used: usize,
    pub retrieval_accuracy: f32,
}

impl AgentQueue {
    pub fn collect_metrics(&self) -> ContextMetrics {
        ContextMetrics {
            tasks_processed: self.metrics.total_tasks,
            strategy_usage: self.metrics.strategy_distribution.clone(),
            average_latency_ms: self.metrics.total_latency / self.metrics.total_tasks,
            average_tokens_used: self.metrics.total_tokens / self.metrics.total_tasks,
            retrieval_accuracy: self.metrics.retrieval_success_rate(),
        }
    }

    pub async fn report_metrics(&self) {
        let metrics = self.collect_metrics();
        let json = serde_json::to_string_pretty(&metrics).unwrap();
        log::info!("Context Metrics:\n{}", json);
    }
}
```

## Migration Path

### Step 1: Add Context Orchestrator
- Integrate `ContextOrchestrator` into `AgentQueue`
- Add `EnhancedTask` schema
- Implement strategy selection

### Step 2: Implement RAG Backend
- Add `RAGBackend` to agent-queue
- Integrate with Ollama embeddings
- Set up ChromaDB vector store

### Step 3: Implement RLM Backend
- Add `RLMBackend` to agent-queue
- Integrate with existing agents
- Implement REPL environment

### Step 4: Update Workflows
- Modify existing workflows to use enhanced context
- Add context_update policies
- Test with sample workflows

### Step 5: Deploy and Monitor
- Deploy to production
- Monitor metrics and performance
- Iterate on thresholds and strategies

---

*Last updated: April 13, 2026*
