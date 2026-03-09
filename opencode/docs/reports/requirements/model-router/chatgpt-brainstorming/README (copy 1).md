# ChatGPT Take on Model Routing System

## Overview

This document outlines ChatGPT's perspective on building a practical local model selector with retry loop capabilities for the model routing CSV data.

## Key Differences from Original Designs

ChatGPT's approach differs from the three original designs in several important ways:

| Aspect | Original Designs | ChatGPT's Approach |
|--------|-----------------|---------------------|
| **Tech stacks** | Python, Rust, Node.js (different paradigms) | All Python, progressive sophistication |
| **Search strategy** | Vector similarity only | Hybrid (dense + sparse + keyword) |
| **Log usage** | Basic penalty scoring | Similarity-aware, Bayesian smoothing |
| **Verification** | Simple binary check | Specialization-driven chains |
| **Progression** | Three equal alternatives | Phased evolution (Tiny → Learned → Adaptive) |
| **Ranking** | Static weights | Learned ranker with decay |
| **Failure handling** | Simple retry | Failure-mode memory |

## Core Philosophy

### Two-Stage Routing

1. **Row Router**
   - Select best specialization rows from CSV
   - Uses hybrid search (dense + sparse + filters)

2. **Model Ranker**
   - Expand winning rows into candidate models
   - Rank using: row relevance, slot bonus, popularity, hardware fit, **log-derived success**

### Retry / Verify / Switch Loop

```
For each task:
  1. Rank N models
  2. Try model #1 up to k times
  3. Verify with specialization-specific chain
  4. If fail but recoverable → retry with repair prompt
  5. If fail after k attempts → switch to next model
  6. Stop on verified success or candidate exhaustion
```

### Log-Driven Evolution

- **Early phase**: Hand-tuned weights dominate
- **Later phase**: Log success score dominates (55% weight)
- **Similarity-aware**: Logs are matched to similar tasks, not global averages

## Document Structure

```
chatgpt_take/
├── README.md                    # This overview
├── common_routing_logic.md       # Foundation logic for all systems
├── report1_tinyrouter.md       # Minimal viable implementation
├── report2_learnedrouter.md    # Learned ranking with analytics
├── report3_adaptiverouter.md   # Full hybrid retrieval system
└── rollout_plan.md             # Recommended phased deployment
```

## Progression Path

### Phase A: TinyRouter
- **Goal**: Prove CSV routing, validators, and retry logic work
- **Tech**: SQLite + sqlite-vec + FTS5 + FastEmbed + Ollama
- **Why**: One-file persistence, easy debugging, fastest to working

### Phase B: LearnedRouter
- **Goal**: Real log-driven ranking with offline analysis
- **Tech**: DuckDB + LanceDB + SentenceTransformers + Ollama
- **Why**: Better analytics, learned ranker, still local

### Phase C: AdaptiveRouter
- **Goal**: Best retrieval with online adaptation
- **Tech**: Qdrant + FastEmbed + Ollama
- **Why**: Strongest search, hybrid retrieval, scalable

## Ranking Formula Evolution

### Initial Formula (Early)
```
final_score =
  0.35 * row_relevance +
  0.10 * slot_bonus +
  0.10 * popularity_prior +
  0.10 * hardware_fit +
  0.35 * log_success_score
```

### Mature Formula (Later)
```
final_score =
  0.25 * row_relevance +
  0.05 * slot_bonus +
  0.05 * popularity_prior +
  0.10 * hardware_fit +
  0.55 * log_success_score
```

**Key insight**: Logs eventually dominate (55% weight), but row relevance remains important (25%).

## Log Success Score Components

```
log_success_score =
  0.50 * verified_success_rate_on_similar_tasks +
  0.20 * first_try_success_rate +
  0.15 * low_retry_rate +
  0.10 * format_or_compile_pass_rate +
  0.05 * latency_score
```

**Key insight**: Similarity-aware matching, not global averages.

## Two-Layer Ranking Philosophy

**Layer 1: Specialization Row**
- "What kind of task is this?"
- Select relevant CSV rows

**Layer 2: Model Candidate**
- "Among models from winning rows, which is best on this machine, for this verifier, on tasks like this, with recent logs?"
- Rank and execute

**Key insight**: This separation enables actual improvement over time instead of becoming a fuzzy tag matcher.

## References

ChatGPT's approach cites:
- [SQLite FTS5 Extension](https://www.sqlite.org/fts5.html)
- [DuckDB Full-Text Search](https://duckdb.org/docs/stable/core_extensions/full_text_search)
- [LanceDB Custom Rerankers](https://docs.lancedb.com/reranking/custom-reranker)
- [Qdrant Documentation](https://qdrant.tech/documentation)
- [Qdrant Hybrid Queries](https://qdrant.tech/documentation/concepts/hybrid-queries/)
- [Qdrant Quantization](https://qdrant.tech/documentation/guides/quantization/)
