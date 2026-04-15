# Infinite Context Research

This directory contains comprehensive research on extending Large Language Model (LLM) context windows to effectively infinite limits through various architectural approaches.

## Overview

LLMs have rapidly evolved from 4K to 1M+ token context windows, yet fundamental challenges remain:

- **Memory Quadratic Complexity**: Standard attention requires O(n²) memory where n = token count
- **Context Rot**: Performance degrades as information ages in long contexts
- **Computation Cost**: Processing millions of tokens becomes prohibitively expensive
- **Information Loss**: Critical details get lost in summarization or truncation

This research explores three major approaches to solve these problems:

1. **Recursive Language Models (RLMs)** - MIT's inference-time scaling paradigm
2. **Architectural Innovations** - Google's compressive memory and attention mechanisms
3. **Retrieval-Augmented Generation (RAG)** - External memory with local LLMs

## Research Categories

| Category | Description | Status |
|-----------|-------------|--------|
| [MIT RLM](./mit-rlm.md) | Recursive decomposition via REPL | Production-ready |
| [Google Infini-Attention](./google-infiniattention.md) | Compressive memory with bounded compute | Research phase |
| [MELODI](./google-melodi.md) | Hierarchical compression layers | Research phase |
| [Ollama/llama.cpp + Vulkan](./ollama-vulkan.md) | GPU acceleration for local models | Production-ready |
| [RAG Implementations](./rag-implementations.md) | Vector database integration | Production-ready |
| [Integration Schema](./integration-schema.md) | Proposed architecture | Design phase |

## Key Findings

### MIT Recursive Language Models
- **Breakthrough**: 100x context extension (50K → 5M tokens) without retraining
- **Mechanism**: Treat prompts as external environment in Python REPL
- **Performance**: 91.33% vs 70.47% for baseline on BrowseComp-Plus
- **Cost**: 2-3x overhead over baseline queries
- **Limitation**: Depth > 1 causes latency explosions (3s → 300s+)

### Google Infini-Attention
- **Breakthrough**: 114x memory reduction vs Memorizing Transformers
- **Mechanism**: Local masked attention + global linear compressive memory
- **Performance**: 1M token sequences on 5K fine-tuned models
- **Memory**: O(n) instead of O(n²) complexity
- **Status**: Research proven, not yet in production models

### MELODI Architecture
- **Breakthrough**: 8x memory footprint reduction
- **Mechanism**: Hierarchical compression across network layers and context windows
- **Performance**: Superior to Memorizing Transformer with 512-token windows
- **Innovation**: Long-term memory in single middle layer, short-term in multiple layers
- **Status**: ICLR 2025 published, Meliad library available

## Implementation Roadmap

For integration into execution engines, see:
- [Integration Schema](./integration-schema.md) - Proposed Rust architecture and data structures
- [Agent-Queue Integration](./agent-queue-integration.md) - Multi-agent workflow orchestration
- [SDK Integration](./sdk-integration.md) - YAML-to-Rust-Agent SDK execution engine
- [Implementation Plan](./implementation-plan.md) - 8-week rollout plan (phase-by-phase)
- [Tradeoff Analysis](./tradeoffs.md) - Detailed comparison matrix
- [Complete Guide](./COMPLETE.md) - Full documentation index with quick start

## References

### Papers
- [Recursive Language Models](https://arxiv.org/abs/2512.24601) - MIT CSAIL (Dec 2025)
- [Leave No Context Behind](https://arxiv.org/abs/2404.07143) - Google (Apr 2024)
- [MELODI](https://arxiv.org/abs/2410.03156) - Google DeepMind (Oct 2024)
- [Improving LLMs by Retrieving](https://arxiv.org/abs/2112.01274) - Google DeepMind (Dec 2021)

### GitHub Projects
- [RLM Library](https://github.com/alexzhang13/rlm) - MIT implementation (3,296★)
- [pyrlm-runtime](https://github.com/apenab/pyrlm-runtime) - Minimal RLM runtime (14★)
- [MCP-RLM](https://github.com/MuhammadIndar/MCP-RLM) - MCP server (11★)
- [llama.cpp](https://github.com/ggml-org/llama.cpp) - Vulkan backend (101K★)
- [Ollama](https://github.com/ollama/ollama) - Vulkan support (167K★)

---

*Last updated: April 13, 2026*
