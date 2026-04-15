# Infinite Context Documentation Index

This directory contains comprehensive documentation on extending Large Language Model (LLM) context windows to effectively infinite limits.

## Quick Start

### What Problem Are We Solving?

Traditional LLMs face fundamental limitations:
- **Fixed context windows**: 4K, 8K, 32K, 128K tokens (not enough for large documents)
- **Context rot**: Performance degrades as information ages
- **Quadratic memory**: O(n²) memory usage where n = token count
- **Information loss**: Critical details lost in summarization or truncation

### Four Major Approaches

| Approach | Description | Best For | Status |
|-----------|-------------|----------|--------|
| [RLM](./mit-rlm.md) | MIT's inference-time scaling via REPL | Complex analysis | Production |
| [Infini-Attention](./google-infiniattention.md) | Google's compressive memory | Book-length processing | Research |
| [MELODI](./google-melodi.md) | Google's hierarchical compression | Memory-constrained | Research |
| [RAG](./rag-implementations.md) | External memory via vector DB | Knowledge bases | Production |
| [Integration Schema](./integration-schema.md) | Proposed Rust implementation | Engine development | Design |
| [Tradeoffs](./tradeoffs.md) | Detailed comparison and selection guide | Decision making | Complete |

## Documentation Structure

### Research Papers

1. **[MIT RLM](./mit-rlm.md)**
   - Recursive Language Models paper (arXiv:2512.24601)
   - 100x context extension (50K → 5M tokens)
   - Open source implementations (3,296★, 508★, 122★)
   - Best practices and integration guides

2. **[Google Infini-Attention](./google-infiniattention.md)**
   - "Leave No Context Behind" paper (arXiv:2404.07143)
   - 114× memory reduction
   - 1M token sequence handling
   - Meliad library (260★)

3. **[Google MELODI](./google-melodi.md)**
   - "MELODI" paper (ICLR 2025)
   - 8× memory footprint reduction
   - Hierarchical compression architecture
   - Implementation details and tuning guide

### Implementation Guides

4. **[Ollama & llama.cpp with Vulkan](./ollama-vulkan.md)**
   - Vulkan backend status and performance
   - Building and configuration
   - Performance benchmarks (25 tok/s with 4B models)
   - Known issues and troubleshooting

5. **[RAG Implementations](./rag-implementations.md)**
   - 6+ open-source projects (120★, 20★, etc.)
   - Vector databases (ChromaDB, FAISS, Weaviate)
   - Document ingestion pipelines
   - Query optimization strategies

### Architecture & Design

6. **[Integration Schema](./integration-schema.md)**
   - Rust-based execution engine design
   - Component definitions (Orchestrator, RLM, RAG, LLM backends)
   - Configuration schemas
   - Error handling and validation
   - Testing framework

7. **[Tradeoffs Analysis](./tradeoffs.md)**
   - Detailed comparison matrix
   - Cost analysis (tokens, memory, latency)
   - Selection guide by use case
   - Risk assessment and mitigation

## Key Findings Summary

### Performance Comparison

| Approach | Context Limit | Memory (1M tokens) | Compute (1M tokens) | Latency | Accuracy |
|-----------|---------------|---------------------|------------------|---------|----------|
| Standard LLM | 32K | 3.2TB | 100× baseline | N/A | Baseline |
| **RLM** | Unbounded | 10GB | 2-3× baseline | 3-300s | +28% |
| **Infini-Attention** | Unbounded | 2GB | 1.5× baseline | 2-3s | Comparable |
| **MELODI** | 512K windows | 8GB | 1.1× baseline | 200-400ms | Slightly better |
| **RAG** | Unbounded | 2GB + 10GB RAM | 1.2× baseline | 100-200ms | 85-90% |

### Implementation Complexity

| Approach | Code Complexity | ML Expertise | Time to Implement | Production Risk |
|-----------|----------------|---------------|-------------------|---------------|
| RLM | Medium | Low | 2-4 weeks | Medium |
| Infini-Attention | High | High | 2-3 months | High |
| MELODI | Very High | Very High | 3-4 months | Very High |
| RAG | Low | None | 1-2 weeks | Low |

### Quick Decision Guide

```
┌─────────────────────────────────────────────────────┐
│ What's your use case?                  │
└────────────────┬──────────────────────────────────┘
             │
             ↓
    ┌────────────────┬────────────────┐    ┌────────────────┬────────────────┐    ┌────────────────┬────────────────┐    ┌────────────────┬────────────────┐
    │ Real-time chat │    │ Interactive coding │    │ Document QA    │    │ Batch analysis │
    └──────┬─────────┘    └──────┬───────────┘    └──────┬───────────┘    └──────┬───────────┘
           │                   │                   │                   │                   │
           ↓                   ↓                   ↓                   ↓
    Use RAG (top-5)   Use RAG + code search   Use RAG           Use RLM (depth=1)
    Cache frequently    Use RAG (top-10)      Consider RAG       Consider RLM+RAG
```

## Getting Started Examples

### Example 1: Simple RAG with Ollama

```python
from rag_pipeline import RAGPipeline
from vector_store import ChromaVectorStore
from llm_backend import OllamaBackend

# Initialize
vector_store = ChromaVectorStore(path="./chroma_db")
llm = OllamaBackend(endpoint="http://localhost:11434")
rag = RAGPipeline(vector_store=vector_store, llm_model="llama3.2")

# Query
result = rag.query("What is the API endpoint format?")
print(result.answer)
print(result.sources)
```

### Example 2: RLM with Local Model

```python
from rlm_backend import RLMBackend
from repl_environment import REPLEnvironment

# Initialize RLM
env = REPLEnvironment(repl_type=REPLType::Local)
rlm = RLMBackend(env=env, llm=llm)

# Load large context
with open("massive_codebase.txt") as f:
    context = f.read()  # 5M+ tokens

# Process with RLM
result = rlm.complete(
    "Find security vulnerabilities in this codebase",
    context_source=context
)

print(result.answer)
```

### Example 3: Hybrid RAG + RLM

```python
from hybrid_orchestrator import HybridOrchestrator

# Initialize hybrid system
orchestrator = HybridOrchestrator(
    rag_backend=rag_backend,
    rlm_backend=rlm_backend,
    config={
        "rag_threshold": 50000,  # Use RAG for < 50K tokens
        "rlm_threshold": 100000,  # Use RLM for 50K-100K tokens
    }
)

# Query
result = orchestrator.query("Analyze this complex document")
print(result.strategy)  # Shows which approach was used
print(result.answer)
```

## Performance Tuning

### Vulkan Backend Optimization

```bash
# Optimal settings for AMD RX 580 8GB
llama-cli \
    --model qwen3.5-4b-instruct-q4_k_m.gguf \
    --device Vulkan0 \
    --n-gpu-layers 33 \
    --ctx-size 16384 \
    --batch-size 8192 \
    --ubatch-size 256 \
    --ctk q8_0 \
    --ctv q8_0
```

### RLM Best Practices

```python
# Configuration from MIT paper
rlm_config = {
    "max_depth": 1,  # CRITICAL: Never use depth > 1
    "token_budget": 1_000_000,  # Prevent runaway costs
    "system_prompt": RLM_SYSTEM_PROMPT,  # From paper
    "parallel_subcalls": True,  # For independent chunks
}

rlm = RLMBackend(config=rlm_config)
```

### RAG Optimization

```python
# Hybrid search configuration
rag_config = {
    "chunk_size": 512,
    "overlap": 64,
    "top_k": 5,
    "score_threshold": 0.3,
    "hybrid_search": True,  # Semantic + keyword
    "max_context_tokens": 2000,
}

rag = RAGPipeline(config=rag_config)
```

## Troubleshooting

### Common Issues

#### RLM Problems
- **Latency too high**: Reduce max_depth to 1
- **Sub-LLM failures**: Add retry logic with fallback
- **Memory overflow**: Add strict token budgets
- **Format collapse**: Add parsing layer with schemas

#### Vulkan Problems
- **Out of memory**: Reduce batch size or context
- **Slow performance**: Try ROCm backend for AMD (if available)
- **Device not found**: Install Vulkan SDK
- **Context ignored**: Check known issues in docs

#### RAG Problems
- **Poor retrieval**: Improve chunking, try different embeddings
- **High latency**: Use FAISS in-memory instead of disk-based
- **Memory usage**: Switch to disk-based vector store
- **Wrong answers**: Improve prompt engineering

## Resources

### Papers
- [Recursive Language Models](https://arxiv.org/abs/2512.24601) - MIT CSAIL
- [Leave No Context Behind](https://arxiv.org/abs/2404.07143) - Google DeepMind
- [MELODI](https://arxiv.org/abs/2410.03156) - Google DeepMind

### GitHub Projects
- [RLM Library](https://github.com/alexzhang13/rlm) - MIT implementation (3,296★)
- [Meliad](https://github.com/google-research/meliad) - Google implementations (260★)
- [llama.cpp](https://github.com/ggml-org/llama.cpp) - Vulkan backend (101K★)
- [Ollama](https://github.com/ollama/ollama) - Official client (167K★)
- [RAG Examples](https://github.com/digithree/ollama-rag) - 120★

### Documentation
- [Meliad Documentation](https://github.com/google-research/meliad/blob/main/README.md)
- [Ollama Documentation](https://ollama.ai/docs)
- [llama.cpp Documentation](https://github.com/ggml-org/llama.cpp/tree/master/examples)

## Next Steps

### For Integration
1. Review [Integration Schema](./integration-schema.md)
2. Decide on approach based on use case (see [Tradeoffs](./tradeoffs.md))
3. Implement core infrastructure (REPL, backends)
4. Add chosen technique (RLM, RAG, etc.)
5. Test with local workloads
6. Profile and optimize

### For Evaluation
1. Set up test environment with Ollama + Vulkan
2. Implement benchmark suite
3. Test each approach with representative workloads
4. Measure latency, memory, accuracy
5. Document results
6. Make final recommendation

## Contributing

To add to this documentation:
1. Create new .md file in this directory
2. Update this README with link
3. Follow existing documentation style
4. Ensure examples are tested
5. Add references to relevant papers/projects

---

*Last updated: April 13, 2026*
