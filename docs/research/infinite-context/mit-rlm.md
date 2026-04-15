# MIT Recursive Language Models (RLMs)

## Executive Summary

**Paper**: "Recursive Language Models" (arXiv:2512.24601)
**Authors**: Alex Zhang, Tim Kraska, Omar Khattab (MIT CSAIL)
**Published**: December 31, 2025
**Status**: Production-ready with multiple open-source implementations

## Core Innovation

### Problem Statement
Traditional LLMs face fundamental limitations:
- Context windows are fixed (4K, 8K, 32K, 128K tokens)
- Performance degrades with long contexts (context rot)
- Memory and computation scale quadratically O(n²)
- Information gets lost in summarization or truncation

### RLM Solution
Treat **long prompts as external environments** rather than direct neural network inputs:

1. **Load context into Python REPL environment** as a variable
2. **LLM writes code** to inspect, search, chunk the context
3. **Recursive sub-calls** decompose tasks into smaller pieces
4. **Synthesize results** without ever loading full context into prompt

### Key Mechanism

```
Traditional Approach (Fails):
prompt = load_5M_token_document()  # ← Memory error
response = llm.completion(prompt)      # ← Truncates at 32K

RLM Approach (Succeeds):
env = PythonREPL()
env.load_variable("context", load_5M_token_document())  # ← External to LLM

code = llm.completion(f"""
    # Write Python code to examine context
    chunks = context.split("\\n---\\n")  # ← Model chooses decomposition
    for chunk in chunks:
        if relevant(chunk, "{query}"):
            return FINAL(chunk.analyze())
""")
result = env.execute(code)
```

## Architecture

### Two-Tier System

```
┌─────────────────────────────────────────────┐
│         Root Language Model (GPT-5)         │
│  - Capabilities: planning, decomposition  │
│  - Context: < 2K tokens                │
└──────────────┬──────────────────────────┘
               │
               │ system prompt: "Write code to..."
               ↓
┌─────────────────────────────────────────────┐
│   Recursive Language Model (GPT-5-mini)    │
│  - Capabilities: execution, reasoning      │
│  - Context: 5K-10K tokens per chunk   │
│  - Runs in Python REPL sandbox             │
└──────────────┬──────────────────────────┘
               │
               │ Can call sub-LLMs
               ↓
┌─────────────────────────────────────────────┐
│   Python REPL Environment                   │
│  - Variables: context, results, state   │
│  - Tools: peek(), chunk(), llm_query() │
│  - Memory: Unbounded (filesystem)     │
└─────────────────────────────────────────────┘
```

### REPL Tools Available to LLM

```python
class ReplEnvironment:
    def peek(self, start: int, end: int) -> str:
        """Read a slice of context variable"""
        return self.variables["context"][start:end]

    def chunk(self, delimiter: str) -> List[str]:
        """Split context into semantic sections"""
        return self.variables["context"].split(delimiter)

    def llm_query(self, query: str, context_slice: str) -> str:
        """Call sub-LLM with focused context"""
        return sub_llm.complete(
            f"Context:\\n{context_slice}\\n\\nTask: {query}"
        )

    def FINAL(self, result: str):
        """Return final answer"""
        return result
```

### Recursive Decomposition Pattern

```
1. Root LM analyzes task:
   "Analyze this 5M token document and summarize key findings"

2. Root LM writes decomposition code:
   code = """
   # Plan:
   # 1. Peek at first 100K tokens to understand structure
   # 2. Chunk document by sections (headers, paragraphs)
   # 3. For each section, extract key points via sub-LLM
   # 4. Synthesize all findings
   """

3. REPL executes code:
   - chunk1 = env.peek(0, 100000)
   - chunk2 = env.peek(100000, 200000)
   ...

4. For each chunk, spawn sub-LLM:
   sub_result1 = env.llm_query("Extract key points", chunk1)
   sub_result2 = env.llm_query("Extract key points", chunk2)
   ...

5. Root LM synthesizes:
   - "Combine these findings: " + [sub_result1, sub_result2, ...]
   - → FINAL(synthesized_summary)
```

## Performance Results

### BrowseComp-Plus Benchmark
| Model | Approach | Score |
|-------|----------|-------|
| GPT-5 baseline | Truncated at 32K | 0.04% |
| Summary Agent | Summarization pipeline | 70.47% |
| **RLM + GPT-5** | Recursive decomposition | **91.33%** |

### LongBench Experiments
| Task | Context Size | Baseline | RLM | Improvement |
|------|--------------|----------|-----|------------|
| OOLONG | 131K tokens | 44.0% | 56.5% | +28.4% |
| OOLONG-Pairs | 32K tokens | 0.04% | 58.0% | +58.0% |

### Context Extension Capability
- **100x extension**: 50K → 5M tokens with same model
- **Small model boost**: Qwen3-8B (32K context) → 100M tokens processed
- **28.3% improvement**: RLM fine-tuned Qwen3-8B over base model
- **Approaches GPT-5**: RLM + Qwen3-8B = GPT-5 baseline performance

## Implementation Details

### System Prompt Design

The system prompt is CRITICAL for RLM performance:

```python
RLM_SYSTEM_PROMPT = """
You are operating in a Python REPL environment with access to these tools:

- peek(range_start, range_end): Read a slice of the 'context' variable
- chunk(delimiter): Split the 'context' variable into sections
- llm_query(query, context_slice): Call a sub-language model with focused context
- FINAL(result): Return your final answer and stop recursion

The full context is stored in a variable named 'context'. DO NOT include the entire
context in your prompt. Your job is to decompose the task, retrieve relevant
context using the tools above, and then call sub-LLMs for focused reasoning.

When you have gathered all necessary information, return FINAL(your_answer).

Guidelines:
- Start with planning: What chunks do I need to examine?
- Use peek() to examine small relevant sections (1K-10K tokens)
- Use chunk() to identify document structure
- Call llm_query() for each chunk that needs analysis
- Be selective: Don't analyze chunks that aren't relevant to the query
- Synthesize: Combine sub-results into a coherent final answer
"""
```

### Depth Control

**Critical finding from paper**: Depth > 1 causes latency explosions

```
Current best practice:
config = {
    "max_depth": 1,  # Root spawns depth=1 sub-LLMs only
}

Why:
- Depth=1: Stable, predictable (3-10s for 1M tokens)
- Depth=2+: Unpredictable, can take 300s+ (exponential growth)
- Paper shows minimal benefit from deeper recursion
```

### Token Budget Management

Track total tokens across all sub-calls:

```python
class TokenBudget:
    def __init__(self, budget: int):
        self.total = budget
        self.used = 0

    def check(self, estimated: int) -> bool:
        return self.used + estimated <= self.total

    def reserve(self, amount: int):
        self.used += amount

# Usage:
budget = TokenBudget(1_000_000)  # 1M total tokens allowed
for sub_call in sub_calls:
    if budget.check(sub_call.estimate):
        budget.reserve(sub_call.estimate)
        sub_call.execute()
```

## Open Source Implementations

### Official MIT Implementations

1. **[alexzhang13/rlm](https://github.com/alexzhang13/rlm)** - Main implementation
   - Stars: 3,296
   - Language: Python
   - License: MIT
   - Features:
     - Universal LLM backend support (OpenAI, Anthropic, vLLM)
     - Docker sandbox environment
     - Visualizer for debugging trajectories
     - Multi-provider support (OpenRouter, Portkey)
   - Installation: `pip install rlms`

2. **[pyrlm-runtime](https://github.com/apenab/pyrlm-runtime)** - Minimal runtime
   - Stars: 14
   - Language: Python
   - License: MIT
   - Features:
     - Focused on core RLM loop
     - Multiple backends (OpenRouter, OpenAI, Anthropic)
     - Policy-based resource limits
     - Rich REPL examples
   - Installation: `pip install pyrlm-runtime`

3. **[MCP-RLM](https://github.com/MuhammadIndar/MCP-RLM)** - MCP server
   - Stars: 11
   - Language: Python
   - License: MIT
   - Features:
     - Two-tier agent system (planning + execution)
     - Model Context Protocol integration
     - Support for Ollama backend
     - Configurable via YAML
   - Installation: Clone and run server

### Community Implementations

1. **[ysz/recursive-llm](https://github.com/ysz/recursive-llm)** - Python implementation
   - Stars: 508
   - Language: Python
   - License: MIT
   - Features:
     - Built on LiteLLM for universal support
     - RestrictedPython for safe execution
     - Rich examples directory

2. **[mitkox/rlmgw](https://github.com/mitkox/rlmgw)** - Gateway implementation
   - Stars: 122
   - Language: Python
   - License: MIT
   - Features:
     - Extensible inference engine
     - Support for various sandbox environments

## Tradeoffs

### Advantages
✅ **True unbounded context**: Limited only by available storage, not model architecture
✅ **No retraining needed**: Works with existing models
✅ **Cost-efficient**: 2-3x overhead vs baseline, not 100x
✅ **Flexible decomposition**: Model learns optimal chunking strategy
✅ **Better than summarization**: 27-38% accuracy improvement

### Limitations
❌ **Latency variance**: 3s → 300s+ depending on task complexity
❌ **Depth explosion**: Depth > 1 causes exponential latency growth
❌ **Format collapse**: Sub-LLMs may return data in different formats
❌ **Code execution overhead**: Requires sandbox and execution environment
❌ **Not yet production-ready**: Latency too high for real-time applications

### When to Use RLM

**Good fit:**
- Long document analysis (100K+ tokens)
- Codebase comprehension over entire repo
- Legal/medical document review
- Research paper summarization
- Batch processing tasks (non-interactive)

**Poor fit:**
- Real-time chat applications
- Low-latency requirements (< 1s)
- Interactive coding assistants
- Simple queries that fit in context window

## Integration with Local LLMs

### vLLM Integration
```python
from rlm import RLM

# Use vLLM's OpenAI-compatible API
rlm = RLM(
    backend="openai",
    backend_kwargs={
        "base_url": "http://localhost:8000/v1",  # vLLM endpoint
        "api_key": "not-needed"
    }
)

# Process massive document
with open("massive_codebase.txt") as f:
    result = rlm.completion(
        "Analyze this codebase and find security vulnerabilities",
        context_source=f.read()  # 5M+ tokens loaded into REPL
    )
```

### Ollama Integration
```python
from rlm import RLM

# Use Ollama's OpenAI-compatible HTTP API
rlm = RLM(
    backend="openai",
    backend_kwargs={
        "base_url": "http://localhost:11434/v1",
        "api_key": "ollama"
    }
)

result = rlm.completion(
    "Summarize this research paper",
    context_source=load_from_file("paper.pdf")
)
```

## Best Practices

### 1. Use Depth=1 Only
```python
rlm = RLM(
    model="gpt-5-mini",
    max_depth=1  # ← Critical: Never use depth > 1 in production
)
```

### 2. Implement Progress Streaming
```python
for chunk in stream_rlm_result(rlm.completion(huge_context)):
    print(chunk, end="", flush=True)  # Show user it's working
```

### 3. Add Token Budgets
```python
rlm = RLM(
    model="qwen3-8b",
    token_budget=1_000_000  # Prevent runaway costs
)
```

### 4. Handle Sub-LLM Failures
```python
try:
    sub_result = env.llm_query(...)
except SubLLMError as e:
    # Fallback to simple approach
    return FINAL(fallback_result)
```

### 5. Cache Intermediate Results
```python
# Store sub-results in REPL to avoid recomputation
env.variables["chunk_1_summary"] = analyze_chunk1(...)
env.variables["chunk_2_summary"] = analyze_chunk2(...)
```

## Comparison with Alternatives

| Approach | Context Limit | Memory | Compute | Production Ready |
|-----------|---------------|--------|---------|------------------|
| RLM | Unbounded | O(n) linear | 2-3x baseline | ✅ Yes |
| Infini-Attention | Unbounded | O(n) linear | 1.5x baseline | ❌ Research |
| RAG | Unbounded | O(n) linear | 1.2x baseline | ✅ Yes |
| Long Context Models | 1M tokens | O(n²) quadratic | 1x baseline | ✅ Yes |

## References

- [Paper](https://arxiv.org/abs/2512.24601)
- [Blog Post](https://alexzhang13.github.io/blog/2025/rlm/)
- [GitHub](https://github.com/alexzhang13/rlm)
- [Prime Intellect RLMEnv](https://www.primeintellect.ai/blog/rlm/)
- [Reproduction Study](https://arxiv.org/abs/2603.02615) - "Think, But Don't Overthink"

---

*Last updated: April 13, 2026*
