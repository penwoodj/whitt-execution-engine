# Integration with YAML-to-Rust-Agent SDK Execution Engine

## Executive Summary

**Objective**: Integrate infinite context capabilities into the whitt-execution-engine SDK's execution engine

**Current State**: SDK uses direct LLM calls with fixed context windows

**Target State**: SDK intelligently selects context strategies (RAG/RLM/Direct) based on task requirements

## Current SDK Architecture

### Execution Flow (Before)

```yaml
# workflow.yaml
workflow:
  name: Document Analysis
  steps:
    - type: llm
      model: llama3.2
      prompt: |
        Analyze this document:
        {{ document_content }}

    - type: llm
      model: llama3.2
      prompt: |
        Summarize the analysis:
        {{ previous_output }}
```

**Problem**: `document_content` must fit entirely in prompt context
**Limitation**: Documents > 32K tokens are truncated or fail

### SDK Components

```rust
// src/sdk/execution/engine.rs
pub struct ExecutionEngine {
    models: ModelRegistry,
    step_executor: StepExecutor,
    context: ExecutionContext,
}

pub struct ExecutionContext {
    variables: HashMap<String, serde_json::Value>,
    // NO external memory, NO retrieval
}

impl ExecutionEngine {
    pub async fn execute_workflow(&mut self, workflow: Workflow) -> Result<WorkflowOutput> {
        for step in workflow.steps {
            let output = self.execute_step(step).await?;
            self.context.variables.insert(step.id.clone(), output.value);
        }
        Ok(final_output)
    }

    async fn execute_step(&mut self, step: Step) -> Result<StepOutput> {
        match step.step_type {
            StepType::LLM => self.execute_llm_step(step).await,
            StepType::Code => self.execute_code_step(step).await,
            // ... other step types
        }
    }

    async fn execute_llm_step(&mut self, step: LLMStep) -> Result<StepOutput> {
        // 1. Resolve variables (NO context enhancement)
        let prompt = self.resolve_variables(&step.prompt)?;

        // 2. Direct LLM call (NO strategy selection)
        let response = self.models.call(
            step.model.clone(),
            prompt,
            step.max_tokens,
        ).await?;

        Ok(StepOutput::new(response))
    }
}
```

## Proposed SDK Enhancements

### 1. Add Context-Aware LLM Step Type

**New Step Schema**:

```yaml
# Enhanced workflow with context-aware steps
workflow:
  name: Advanced Document Analysis
  steps:
    - type: llm_context_aware
      id: analyze
      model: llama3.2
      prompt: |
        Analyze this document for key findings:
        {{ document_content }}

      # NEW: Context configuration
      context:
        source: file
        path: ./large_document.txt

        # Optional: Override default strategy
        strategy: auto  # auto, direct, rag, rlm, hybrid

        # Optional: Strategy-specific settings
        rag:
          top_k: 5
          score_threshold: 0.3

        rlm:
          max_depth: 1
          token_budget: 500000

        # Optional: Max context tokens
        max_context_tokens: 10000

    - type: llm_context_aware
      id: summarize
      model: llama3.2
      prompt: |
        Summarize the findings:
        {{ previous_output }}

      context:
        source: variable
        variable: analyze.output
        strategy: direct  # Small context, direct call
```

### 2. Context Source Schema

```rust
#[derive(Debug, Clone, Deserialize)]
pub enum ContextSource {
    /// Direct text (traditional approach)
    Direct(String),

    /// File content
    File {
        path: PathBuf,
        encoding: Option<String>,
    },

    /// URL content
    URL {
        url: String,
        headers: Option<HashMap<String, String>>,
    },

    /// Variable from previous step
    Variable {
        name: String,
    },

    /// Vector store retrieval
    VectorStore {
        collection: String,
        query: String,  # Or reference to variable
        top_k: usize,
    },

    /// Hybrid: Multiple sources combined
    Hybrid {
        sources: Vec<ContextSource>,
        merge_strategy: MergeStrategy,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub enum MergeStrategy {
    Concatenate,
    Interleave,
    PrioritizeFirst,
}
```

### 3. Enhanced Execution Engine

```rust
// src/sdk/execution/context_engine.rs
pub struct ContextEngine {
    orchestrator: ContextOrchestrator,
    rag_backend: Option<RAGBackend>,
    rlm_backend: Option<RLMBackend>,
    llm_backend: Box<dyn LLMAdapter>,
}

impl ContextEngine {
    pub async fn execute_with_context(
        &self,
        prompt: String,
        context_source: ContextSource,
        strategy: ContextStrategy,
    ) -> Result<String> {
        // 1. Resolve context source
        let context = self.resolve_context_source(&context_source).await?;

        // 2. Select strategy if auto
        let selected_strategy = match strategy {
            ContextStrategy::Auto => {
                let context_size = self.estimate_context_size(&context);
                self.orchestrator.select_strategy_for_size(context_size)
            },
            ContextStrategy::Direct => Strategy::Direct,
            ContextStrategy::RAG(config) => Strategy::RAG(config),
            ContextStrategy::RLM(config) => Strategy::RLM(config),
            ContextStrategy::Hybrid(config) => Strategy::Hybrid(config),
        };

        // 3. Execute with selected strategy
        match selected_strategy {
            Strategy::Direct => {
                self.execute_direct(prompt, context).await
            },
            Strategy::RAG(config) => {
                self.execute_with_rag(prompt, context, config).await
            },
            Strategy::RLM(config) => {
                self.execute_with_rlm(prompt, context, config).await
            },
            Strategy::Hybrid(config) => {
                self.execute_hybrid(prompt, context, config).await
            },
        }
    }

    async fn resolve_context_source(&self, source: &ContextSource) -> Result<String> {
        match source {
            ContextSource::Direct(text) => Ok(text.clone()),
            ContextSource::File { path, .. } => {
                Ok(fs::read_to_string(path)?)
            },
            ContextSource::URL { url, .. } => {
                self.fetch_url(url).await
            },
            ContextSource::Variable { name } => {
                Ok(self.variables.get(name)
                    .ok_or(Error::VariableNotFound)?
                    .as_str()
                    .unwrap()
                    .to_string())
            },
            ContextSource::VectorStore { collection, query, top_k } => {
                let embedding = self.llm_backend.embed(query).await?;
                let retrieved = self.rag_backend.as_ref()
                    .ok_or(Error::BackendNotInitialized)?
                    .vector_store
                    .query(&embedding, *top_k)
                    .await?;

                Ok(retrieved.iter()
                    .map(|r| r.text.clone())
                    .collect::<Vec<_>>()
                    .join("\n\n---\n\n"))
            },
            ContextSource::Hybrid { sources, merge_strategy } => {
                let contexts: Vec<_> = futures::future::join_all(
                    sources.iter().map(|s| self.resolve_context_source(s))
                ).await.into_iter().collect::<Result<Vec<_>>>()?;

                match merge_strategy {
                    MergeStrategy::Concatenate => Ok(contexts.join("\n\n---\n\n")),
                    MergeStrategy::Interleave => self.interleave_contexts(contexts),
                    MergeStrategy::PrioritizeFirst => contexts.first()
                        .cloned()
                        .ok_or(Error::NoContext),
                }
            },
        }
    }
}
```

### 4. Integration with ExecutionEngine

```rust
// src/sdk/execution/engine.rs (modified)
impl ExecutionEngine {
    pub fn new_with_context(config: EngineConfig) -> Result<Self> {
        let context_engine = ContextEngine::new(&config)?;
        Ok(ExecutionEngine {
            models: ModelRegistry::new(),
            step_executor: StepExecutor::new(),
            context: ExecutionContext::default(),
            context_engine: Some(context_engine),  // NEW
        })
    }

    async fn execute_step(&mut self, step: Step) -> Result<StepOutput> {
        match step.step_type {
            StepType::LLM => self.execute_llm_step(step).await,
            StepType::LLMContextAware => {  // NEW
                self.execute_llm_context_aware_step(step).await
            },
            StepType::Code => self.execute_code_step(step).await,
            // ... other step types
        }
    }

    async fn execute_llm_context_aware_step(
        &mut self,
        step: Step,
    ) -> Result<StepOutput> {
        let context_aware_step: LLMContextAwareStep = serde_json::from_value(step.data.clone())?;

        // 1. Resolve prompt with variables (traditional)
        let resolved_prompt = self.context.resolve_variables(&context_aware_step.prompt)?;

        // 2. Use context engine for enhanced execution
        let context_engine = self.context_engine.as_ref()
            .ok_or(Error::ContextEngineNotInitialized)?;

        let enhanced_response = context_engine.execute_with_context(
            resolved_prompt,
            context_aware_step.context.source.clone(),
            context_aware_step.context.strategy.clone(),
        ).await?;

        Ok(StepOutput::new(enhanced_response))
    }
}
```

## Workflow Examples

### Example 1: Large Document Processing

```yaml
# workflow: process_large_document.yaml
workflow:
  name: Process Large Document with RAG
  steps:
    - type: llm_context_aware
      id: index
      description: Index document for retrieval
      model: llama3.2
      prompt: |
        Prepare this document for analysis:
        {{ document_content }}

      context:
        source: file
        path: ./large_document.txt
        strategy: rag
        rag:
          top_k: 5
          chunk_size: 512

    - type: llm_context_aware
      id: analyze
      description: Analyze document sections
      model: llama3.2
      prompt: |
        Analyze the following document sections:
        {{ context }}

        Extract: key findings, themes, and conclusions
      context:
        source: vector_store
        collection: documents
        query: |
          What are the main themes and conclusions?
        top_k: 10
        strategy: rag
        rag:
          top_k: 10

    - type: llm_context_aware
      id: synthesize
      description: Synthesize findings
      model: llama3.2
      prompt: |
        Based on the analysis, provide a comprehensive summary:
        {{ analyze.output }}

      context:
        source: variable
        variable: analyze.output
        strategy: direct  # Small output, direct call
```

**Execution**:

```bash
# Run workflow
whitt-execution-engine execute workflow:process_large_document.yaml

# SDK automatically:
# 1. Indexes the large document into vector store
# 2. Retrieves relevant sections for analysis
# 3. Synthesizes final summary
```

### Example 2: Codebase Analysis with RLM

```yaml
# workflow: codebase_analysis.yaml
workflow:
  name: Analyze Entire Codebase with RLM
  steps:
    - type: llm_context_aware
      id: analyze_structure
      description: Analyze codebase structure
      model: qwen3.5-7b
      prompt: |
        Analyze this codebase and identify:
        - Project structure
        - Key components
        - Dependencies
        - Potential issues

        Use code analysis tools to examine:
        1. README and documentation
        2. Configuration files
        3. Main entry points
        4. Test files
      context:
        source: file
        path: ./codebase/
        strategy: rlm
        rlm:
          max_depth: 1
          token_budget: 500000
          timeout_ms: 120000

    - type: llm_context_aware
      id: generate_report
      description: Generate analysis report
      model: llama3.2
      prompt: |
        Generate a comprehensive codebase analysis report:
        {{ analyze_structure.output }}

        Include sections:
        1. Executive Summary
        2. Architecture
        3. Component Details
        4. Recommendations
      context:
        source: variable
        variable: analyze_structure.output
        strategy: direct
```

### Example 3: Hybrid RAG + RLM Workflow

```yaml
# workflow: hybrid_research.yaml
workflow:
  name: Hybrid Research with RAG and RLM
  steps:
    - type: llm_context_aware
      id: retrieve_knowledge
      description: Retrieve relevant knowledge
      model: llama3.2
      prompt: |
        Retrieve information about: {{ research_topic }}
      context:
        source: vector_store
        collection: research_papers
        query: "{{ research_topic }}"
        top_k: 5
        strategy: rag

    - type: llm_context_aware
      id: analyze_corpus
      description: Deep analysis with RLM
      model: qwen3.5-7b
      prompt: |
        Analyze this corpus in depth:
        {{ corpus_content }}

        Integrate with retrieved knowledge:
        {{ retrieve_knowledge.output }}

        Provide detailed analysis with citations
      context:
        source: hybrid
        strategy: hybrid
        hybrid:
          sources:
            - type: file
              path: ./research_corpus.txt
            - type: variable
              variable: retrieve_knowledge.output
          merge_strategy: interleave

    - type: llm_context_aware
      id: write_report
      description: Write final report
      model: llama3.2
      prompt: |
        Write a comprehensive research report:
        {{ analyze_corpus.output }}

        Structure: Introduction, Methods, Results, Discussion, Conclusion
      context:
        source: variable
        variable: analyze_corpus.output
        strategy: direct
```

## Configuration

### SDK Configuration

**File**: `config/sdk.yaml`

```yaml
sdk:
  # Execution engine
  execution_engine:
    enable_context_engine: true
    default_strategy: auto

    # Backend configuration
    backends:
      ollama:
        enabled: true
        endpoint: http://localhost:11434
        default_model: llama3.2

      vector_store:
        type: chroma
        path: ./chroma_db
        default_collection: documents

      rlm:
        enabled: true
        max_depth: 1
        token_budget: 1000000
        repl_type: local

    # Strategy thresholds
    thresholds:
      rlm: 100000      # 100K tokens → use RLM
      rag: 50000       # 50K tokens → use RAG
      max_context: 1000000  # 1M token hard limit

  # Model registry
  models:
    llama3.2:
      provider: ollama
      context_size: 128000
      max_tokens: 8192

    qwen3.5-7b:
      provider: ollama
      context_size: 32000
      max_tokens: 8192

  # Workflow settings
  workflows:
    default_max_steps: 10
    default_timeout_ms: 300000
    enable_caching: true
    cache_ttl_seconds: 3600
```

## CLI Integration

### New Commands

```bash
# Execute workflow with context
whitt-execution-engine execute \
  --workflow process_large_document.yaml \
  --context-strategy auto \
  --document ./large_document.txt

# Index documents for RAG
whitt-execution-engine index \
  --path ./documents/ \
  --chunk-size 512 \
  --overlap 64 \
  --collection documents

# Query knowledge base
whitt-execution-engine query \
  --query "What are the key findings?" \
  --collection documents \
  --top-k 5 \
  --model llama3.2

# Execute with RLM
whitt-execution-engine execute \
  --workflow codebase_analysis.yaml \
  --context-strategy rlm \
  --max-depth 1 \
  --token-budget 500000
```

### CLI Implementation

```rust
// src/cli/commands.rs
#[derive(Subcommand)]
pub enum ExecuteCommand {
    Execute {
        #[arg(short, long)]
        workflow: PathBuf,

        #[arg(long)]
        context_strategy: Option<ContextStrategy>,

        #[arg(long)]
        document: Option<PathBuf>,

        #[arg(long)]
        max_depth: Option<usize>,

        #[arg(long)]
        token_budget: Option<usize>,
    },

    Index {
        #[arg(short, long)]
        path: PathBuf,

        #[arg(long, default_value = "512")]
        chunk_size: usize,

        #[arg(long, default_value = "64")]
        overlap: usize,

        #[arg(long, default_value = "documents")]
        collection: String,
    },

    Query {
        #[arg(short, long)]
        query: String,

        #[arg(long, default_value = "documents")]
        collection: String,

        #[arg(long, default_value = "5")]
        top_k: usize,

        #[arg(long)]
        model: Option<String>,
    },
}

pub async fn handle_execute_command(command: ExecuteCommand) -> Result<()> {
    match command {
        ExecuteCommand::Execute {
            workflow,
            context_strategy,
            document,
            max_depth,
            token_budget,
        } => {
            let mut engine = ExecutionEngine::new_with_context(load_config()?)?;
            let workflow_def = load_workflow(workflow)?;

            // Override context strategy if specified
            let enhanced_workflow = if let Some(strategy) = context_strategy {
                workflow_def.override_strategy(strategy)
            } else {
                workflow_def
            };

            let result = engine.execute_workflow(enhanced_workflow).await?;
            println!("{}", result.output);
        },

        ExecuteCommand::Index {
            path,
            chunk_size,
            overlap,
            collection,
        } => {
            let mut ingester = DocumentIngester::new(chunk_size, overlap)?;
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                if entry.path().is_file() {
                    let count = ingester.ingest_document(&entry.path()).await?;
                    println!("✓ Indexed {}: {} chunks", entry.path().display(), count);
                }
            }
            println!("✓ Indexing complete");
        },

        ExecuteCommand::Query {
            query,
            collection,
            top_k,
            model,
        } => {
            let engine = ExecutionEngine::new_with_context(load_config()?)?;
            let result = engine.query_collection(query, collection, top_k, model).await?;
            println!("{}", result);
        },
    }

    Ok(())
}
```

## Testing

### Workflow Testing

```rust
#[tokio::test]
async fn test_context_aware_workflow() {
    // Create test workflow
    let workflow = Workflow {
        name: "Test Workflow".to_string(),
        steps: vec![
            Step {
                id: "step1".to_string(),
                step_type: StepType::LLMContextAware,
                data: json!({
                    "prompt": "Analyze: {{ context }}",
                    "context": {
                        "source": "direct",
                        "content": "Test content",
                        "strategy": "direct"
                    }
                }),
            },
        ],
    };

    // Execute
    let engine = ExecutionEngine::new_with_context(test_config()).unwrap();
    let result = engine.execute_workflow(workflow).await.unwrap();

    assert!(!result.output.is_empty());
}

#[tokio::test]
async fn test_rag_workflow() {
    // Index test document
    let mut ingester = DocumentIngester::new();
    ingester.ingest_document(Path::new("test.txt")).await.unwrap();

    // Create RAG workflow
    let workflow = create_rag_workflow();

    // Execute
    let engine = ExecutionEngine::new_with_context(test_config()).unwrap();
    let result = engine.execute_workflow(workflow).await.unwrap();

    assert!(!result.output.is_empty());
    assert!(result.sources.len() > 0);
}

#[tokio::test]
async fn test_rlm_workflow() {
    // Create RLM workflow
    let workflow = create_rlm_workflow();

    // Execute
    let engine = ExecutionEngine::new_with_context(test_config()).unwrap();
    let result = engine.execute_workflow(workflow).await.unwrap();

    assert!(!result.output.is_empty());
    assert!(result.stats.sub_llm_calls > 0);
}
```

## Migration Guide

### Step 1: Update Dependencies

```toml
# Cargo.toml
[dependencies]
whitt-execution-engine = { path = "." }

# Add context engine dependencies
tokio = "1.35"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
reqwest = "0.11"
chroma-rs = "0.6"
```

### Step 2: Update Workflows

```yaml
# Old workflow (no context awareness)
workflow:
  steps:
    - type: llm
      prompt: |
        Analyze: {{ large_document_content }}

# New workflow (context-aware)
workflow:
  steps:
    - type: llm_context_aware
      prompt: |
        Analyze: {{ context }}
      context:
        source: file
        path: ./large_document.txt
        strategy: auto
```

### Step 3: Update Configuration

```yaml
# config/sdk.yaml
sdk:
  execution_engine:
    enable_context_engine: true  # NEW
    default_strategy: auto       # NEW

  # NEW: Backend configuration
  backends:
    vector_store:
      type: chroma
      path: ./chroma_db
    rlm:
      enabled: true
      max_depth: 1
```

### Step 4: Test and Deploy

```bash
# 1. Run existing workflows (should still work)
whitt-execution-engine execute workflow:existing.yaml

# 2. Test new context-aware workflows
whitt-execution-engine execute workflow:new_context_aware.yaml

# 3. Index documents for RAG
whitt-execution-engine index --path ./documents/

# 4. Test RAG
whitt-execution-engine query --query "Test query" --collection documents
```

---

*Last updated: April 13, 2026*
