# Model Router for Transpiler - Complete Analysis

## Overview

This folder contains comprehensive analysis for designing a model router system that intelligently selects and retries local LLM models for a YAML-to-Rust transpiler.

## Contents

### 1. OpenCode Brainstorming
**File**: `opencode-brainstorming/README.md`

OpenCode's original analysis focused on:
- Three alternative tech stack designs (Python/SQLite, Rust/Sled, Node.js/LevelDB)
- Complete implementations with ASCII diagrams
- Focus on minimal hardware requirements
- Vector similarity-only retrieval
- Basic log penalty scoring

**Key Position**: Practical, single-stack implementations with clear trade-offs.

### 2. ChatGPT Brainstorming
**File**: `chatgpt-brainstorming/README.md`

ChatGPT's research-backed approach emphasized:
- Three-phase progressive rollout (MVP → Learned → Adaptive)
- Hybrid retrieval (dense + sparse + keyword + filters)
- Similarity-aware Bayesian smoothing for logs
- Failure-mode memory beyond binary success tracking
- Two-layer routing architecture (row selection + model ranking)
- Research citations for all major decisions

**Key Position**: Evolutionary, research-driven, data-adaptive.

### 3. Combined Decisions
**File**: `combined-brainstorming/DECISIONS.md`

Synthesis document that:
- Compares all options considered from both approaches
- Documents final decisions with detailed rationale
- References research evidence
- Provides phased implementation plan
- Identifies non-goals and risk mitigations

**Key Position**: Unified, evidence-based design decisions.

## Quick Reference: Final Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| **Routing Granularity** | Two-layer (row + model) | Separates concerns, enables independent optimization |
| **Retrieval** | Hybrid (dense + sparse) | 15-30% improvement from research |
| **Vector DB** | sqlite-vec (MVP) → LanceDB → Qdrant | Scales with needs, minimal for start |
| **Embedding** | all-MiniLM-L6-v2 + FastEmbed | Best RAM/accuracy/speed balance |
| **Log Statistics** | Similarity-aware KNN | Heterogeneous tasks need task-specific stats |
| **Ranking** | Adaptive weights (35% → 55% logs) | Data-driven evolution |
| **Verification** | Specialization-driven multi-stage | Different tasks need different verification |
| **Retry Strategy** | Budget-based by specialization | Efficient retry allocation |
| **Failure Tracking** | Categorization → embedding (progressive) | Rich learning signal |
| **Hardware Awareness** | Catalog + dynamic penalties | Baseline with adaptation |

## Architecture Summary

```
Input YAML
    ↓
Task Analysis
    ↓
┌─────────────────────────────────────┐
│ Layer 1: Row Selection (Hybrid)  │
└─────────────────┬───────────────────┘
                  │
                  ↓
┌─────────────────────────────────────┐
│ Layer 2: Model Ranking (Adaptive) │
└─────────────────┬───────────────────┘
                  │
                  ↓
          Execution Loop (Retry/Switch)
                  │
                  ↓
          Verification (Multi-Stage)
                  │
                  ↓
          Output + Log Update
```

## Technology Stack

| Phase | Vector DB | Storage | ML | Hardware |
|-------|-----------|---------|-----|----------|
| **Phase 1: TinyRouter** | sqlite-vec | SQLite | Static weights | 380MB RAM |
| **Phase 2: LearnedRouter** | LanceDB | DuckDB | Logistic/XGBoost | 525MB RAM |
| **Phase 3: AdaptiveRouter** | Qdrant | Qdrant+SQLite | Online | 200MB+ RAM |

## Implementation Timeline

### Phase 1: TinyRouter (Weeks 1-4)
- Single-file SQLite
- Hand-tuned weights
- Hybrid retrieval
- 1000+ attempts logged
- Proves concept

### Phase 2: LearnedRouter (Weeks 5-12)
- DuckDB analytics
- LanceDB retrieval
- Learned ranker
- 5000+ attempts logged
- Real log-driven ranking

### Phase 3: AdaptiveRouter (Weeks 13-24)
- Qdrant hybrid queries
- Failure-mode embeddings
- Online adaptation
- 10000+ attempts logged
- Production-grade system

## Key Innovations

1. **Hybrid Retrieval** - Combines semantic and lexical search
2. **Similarity-Aware Stats** - Matches logs to similar tasks, not globally
3. **Bayesian Smoothing** - Prevents overfitting to small samples
4. **Failure-Mode Memory** - Tracks how models fail, enables proactive avoidance
5. **Two-Layer Routing** - Separates task classification from model selection
6. **Adaptive Weights** - Shifts from heuristics to evidence over time
7. **Specialization-Driven Verification** - Different chains for different task types
8. **Cluster-Based Similarity** - O(1) lookup for efficient log matching
9. **Exponential Decay** - Temporal weighting favoring recent logs
10. **Progressive Complexity** - Starts simple, grows based on needs

## Research References

Key research backing the design:

1. **BEIR Benchmark** - Hybrid outperforms dense by 8.5% and sparse by 6.9%
2. **Production RAG** - 15-30% improvement with hybrid search
3. **Bayesian Smoothing** - Alpha=10 recommended for small samples
4. **FastEmbed + ONNX** - 4x faster inference with quantization
5. **sqlite-vec** - Efficient vector search for embedded use

## Success Metrics

Track throughout implementation:

| Metric | Phase 1 Target | Phase 2 Target | Phase 3 Target |
|--------|---------------|---------------|---------------|
| Routing accuracy (top 3) | 80% | 85% | 90% |
| Tasks succeeding in 2 models | 70% | 75% | 80% |
| Tasks succeeding in 3 models | 90% | 92% | 95% |
| Average latency per task | < 10s | < 8s | < 5s |
| System uptime | 95% | 98% | 99% |

## Non-Goals

What we're NOT building:

1. ❌ Distributed multi-tenant service
2. ❌ Real-time online ML training
3. ❌ Cloud fallback mechanisms
4. ❌ Production HA/load balancing
5. ❌ GPU-accelerated (CPU-optimized)
6. ❌ Complex MLOps pipeline

## How to Use This Analysis

1. **Start Here**: Read `combined-brainstorming/DECISIONS.md` for complete design
2. **See Both Perspectives**: Review `opencode-brainstorming/` and `chatgpt-brainstorming/` for alternative approaches
3. **Understand Decisions**: Review decision matrix for why each choice was made
4. **Plan Implementation**: Follow phased timeline in `DECISIONS.md`
5. **Reference Architecture**: Use ASCII diagrams for implementation guidance

## Next Steps

1. ✅ **Phase 1 Setup** - Install sqlite-vec, FastEmbed, import CSV
2. ✅ **Implement Layer 1** - Hybrid retrieval with fusion
3. ✅ **Implement Layer 2** - Model expansion and adaptive ranking
4. ✅ **Implement Verifiers** - Specialization-driven chains
5. ✅ **Implement Execution Loop** - Retry/switch logic
6. ✅ **Collect Logs** - Start data gathering immediately
7. ✅ **Evaluate Phase 1** - Measure against success criteria
8. ⏳ **Phase 2 Planning** - Prepare for DuckDB + LanceDB migration
9. ⏳ **Phase 3 Planning** - Prepare for Qdrant + online adaptation

## Conclusion

This combined analysis provides a complete, research-backed design for a model router that:

- Starts simple and practical (Phase 1)
- Evolves based on actual usage (Phases 2-3)
- Works on minimal hardware (4-8GB RAM)
- Is fully local and self-contained
- Learns continuously from logs
- Handles diverse YAML-to-code tasks intelligently

**Primary Insight**: The best model router isn't one static design but a system that starts working and gets better over time based on data, with clear upgrade paths when sophistication is needed.
