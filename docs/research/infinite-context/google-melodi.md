# Google MELODI Architecture

## Executive Summary

**Paper**: "MELODI: Exploring Memory Compression for Long Contexts"
**Authors**: Yinpeng Chen, DeLesley Hutchins, Aren Jansen, Andrey Zhmoginov, David Racz, Jesper Andersen (Google DeepMind)
**Published**: ICLR 2025 (arXiv:2410.03156)
**Status**: Research phase, reference implementation available

## Core Innovation

### Problem Statement
- Standard Transformers require O(n²) memory for long contexts
- Even with efficient attention, KV cache grows unbounded
- MemoryLLM adds external memory to every layer (high overhead)
- Need efficient compression that reduces memory without losing performance

### MELODI Solution
**Hierarchical memory compression** across two dimensions:
1. **Short-term memory**: Recurrent compression across multiple layers
2. **Long-term memory**: Further compression in single middle layer

### Key Mechanism

```
┌────────────────────────────────────────────────────┐
│  Multi-Layer Transformer                      │
│                                              │
│  Layer 1: Input → [Short-term] → Output    │
│  Layer 2: Input → [Short-term] → Output    │
│  Layer 3: Input → [Short-term] → Output    │
│  Layer 4: Input → [Long-term] → Output ←──┐   │
│  Layer 5: Input → [Short-term] → Output    │  │  │
│  Layer 6: Input → [Short-term] → Output    │   │  │
│  ...                                         │   │  │
│                                              │   └── Hierarchical compression
└────────────────────────────────────────────────────┘
```

## Architecture

### Two-Tier Memory System

```
┌──────────────────────────────────────────────────────────┐
│ Layer 1                                              │
│                                                       │
│  Input ──→ Standard Attention ──→ Short-Term ──→ Output │
│            (512 tokens)            Memory (S tokens)      │
│                                                       │
│  Short-term: Recurrently compresses context            │
└──────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────┐
│ Layer 2                                              │
│                                                       │
│  Input ──→ Standard Attention ──→ Short-Term ──→ Output │
│            (512 tokens)            Memory (S tokens)      │
│                                                       │
└──────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────┐
│ Layer 3                                              │
│                                                       │
│  Input ──→ Standard Attention ──→ Short-Term ──→ Output │
│            (512 tokens)            Memory (S tokens)      │
│                                                       │
└──────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────┐
│ Layer 4 (Middle Layer)                               │
│                                                       │
│  Input ──→ Standard Attention ──→ Long-Term ──→ Output   │
│            (512 tokens)           Memory (L tokens)        │
│                                                       │
│  Long-term: Compresses entire history               │
└──────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────┐
│ Layer 5                                              │
│                                                       │
│  Input ──→ Standard Attention ──→ Short-Term ──→ Output │
│            (512 tokens)            Memory (S tokens)      │
│                                                       │
└──────────────────────────────────────────────────────────┘
```

### Memory Compression Functions

**Short-term Memory (per layer)**:
```python
def short_term_compression(input, prev_short_memory):
    """
    Recurrent compression across multiple layers
    """
    # Combine input with previous short-term memory
    combined = concatenate(input, prev_short_memory)

    # Compress to fixed size S
    compressed = compress(combined, target_size=S)

    return compressed
```

**Long-term Memory (middle layer only)**:
```python
def long_term_compression(all_short_memories, prev_long_memory):
    """
    Hierarchical compression across entire history
    """
    # Aggregate all short-term memories from previous layers
    aggregated = concatenate(all_short_memories)

    # Further compress with long-term history
    combined = concatenate(aggregated, prev_long_memory)

    # Compress to fixed size L
    compressed = compress(combined, target_size=L)

    return compressed
```

### Context Window Management

```python
# Process long document in sliding windows
window_size = 512  # Tokens per segment
overlap = 64  # Overlap between windows

for i in range(0, len(document), window_size - overlap):
    window = document[i:i+window_size]

    # Feed through MELODI transformer
    output = melodi_model(window)

    # Memory automatically carries information forward
    # via recurrent compression
```

## Performance Results

### Memory Footprint Reduction

| Model | Short-term (S) | Long-term (L) | Total | Reduction |
|-------|-----------------|----------------|-------|-----------|
| Memorizing Transformer | N/A | 64K | 64K | 1× (baseline) |
| **MELODI S192+L32** | 192 | 32 | **224** | **8× reduction** |
| MELODI S128+L64 | 128 | 64 | 192 | 3.3× |
| MELODI S192+L96 | 192 | 96 | 288 | 2.2× |

### Benchmark Performance

#### PG-19 Book Modeling (Long-Context Benchmark)
| Configuration | Perplexity | Baseline Perplexity |
|---------------|------------|---------------------|
| Transformer-XL | 6.92 | - |
| **MELODI S192+L32** | **5.67** | 6.92 |
| MELODI S192+L96 | 5.71 | 6.92 |

#### Memory Efficiency
| Memory Type | Size | Access Pattern |
|-------------|------|---------------|
| Short-term (S tokens) | 192 | Layer-local, recurrent |
| Long-term (L tokens) | 32 | Middle layer, global history |
| Total effective | ~224 tokens | With 512-token windows |

**Key finding**: MELODI processes 512-token windows but maintains 224-token effective memory.

### Scaling Characteristics

| Window Size | Layers | Short-term per Layer | Long-term (middle) | Total Memory |
|-------------|--------|--------------------|--------------------|--------------|
| 512 | 13 | 192 | 32 | 2,528 tokens |
| 512 | 12 | 128 | 64 | 1,600 tokens |

## Implementation Details

### Network Architecture

```python
class MELODITransformer(nn.Module):
    def __init__(self, config):
        super().__init__()

        # Standard transformer layers
        self.layers = nn.ModuleList([
            MELODILayer(
                d_model=config.d_model,
                n_heads=config.n_heads,
                is_middle=(i == config.middle_layer)
            )
            for i in range(config.num_layers)
        ])

    def forward(self, x):
        # Process through layers
        for layer in self.layers:
            x = layer(x)

        return x
```

### MELODI Layer

```python
class MELODILayer(nn.Module):
    def __init__(self, d_model, n_heads, is_middle=False):
        super().__init__()

        # Standard multi-head attention
        self.attention = MultiHeadAttention(d_model, n_heads)

        # Memory compression
        if not is_middle:
            # Short-term memory: recurrent compression
            self.short_term_memory = CompressionModule(
                input_size=d_model,
                compressed_size=config.S  # e.g., 192
            )
        else:
            # Middle layer: long-term memory
            self.long_term_memory = CompressionModule(
                input_size=d_model * config.num_layers,  # All short-terms
                compressed_size=config.L  # e.g., 32
            )

    def forward(self, x):
        # Standard attention
        attn_output = self.attention(x)

        if not self.is_middle:
            # Short-term: recurrent with layer's previous state
            compressed = self.short_term_memory(
                concatenate(attn_output, self.prev_short_term)
            )
            self.prev_short_term = compressed
            return compressed
        else:
            # Middle layer: aggregate all short-terms and compress
            all_short_terms = [layer.prev_short_term for layer in prev_layers]
            compressed = self.long_term_memory(
                concatenate(all_short_terms, self.prev_long_term)
            )
            self.prev_long_term = compressed
            return compressed
```

### Compression Module

```python
class CompressionModule(nn.Module):
    def __init__(self, input_size, compressed_size):
        super().__init__()

        # Learned compression (trainable)
        self.compressor = nn.Sequential(
            nn.Linear(input_size, 4 * compressed_size),
            nn.GELU(),
            nn.Linear(4 * compressed_size, compressed_size),
        )

        # Optional: add reconstruction for training loss
        self.reconstructor = nn.Sequential(
            nn.Linear(compressed_size, 4 * compressed_size),
            nn.GELU(),
            nn.Linear(4 * compressed_size, input_size),
        )

    def forward(self, x):
        compressed = self.compressor(x)
        return compressed

    def reconstruct(self, compressed):
        return self.reconstructor(compressed)
```

### Training Procedure

```python
# 1. Load base transformer
base_model = load_pretrained("transformer-base")

# 2. Initialize MELODI with memory
melodi = MELODITransformer(config={
    "d_model": 1024,
    "num_layers": 13,
    "middle_layer": 6,  # 7th layer (0-indexed)
    "S": 192,  # Short-term tokens per layer
    "L": 32,   # Long-term tokens (middle layer only)
})

# 3. Fine-tune with long-context data
train_config = {
    "window_size": 512,
    "batch_size": 16,
    "learning_rate": 1e-4,
    "epochs": 50000,
}

train(melodi, dataset=PG19(), config=train_config)

# 4. Result: Processes 512-token windows with 8× less memory
```

## Open Source Availability

### Official Google Implementation

**Repository**: [google-research/meliad](https://github.com/google-research/meliad)
- Stars: 260
- Language: Python
- License: Apache 2.0
- Models available:
  - `melodi_s192_l32`: MELODI with 192 short-term, 32 long-term
  - `melodi_s128_l64`: MELODI with 128 short-term, 64 long-term
  - `melodi_s192_l96`: MELODI with 192 short-term, 96 long-term

### Installation

```bash
# Clone repository
git clone https://github.com/google-research/meliad.git
cd meliad

# Install dependencies
pip install -r requirements.txt

# Or install from PyPI (if available)
pip install meliad
```

### Usage Example

```python
from meliad.models import MelodiModel

# Load pre-trained MELODI model
model = MelodiModel.load("melodi_s192_l32")

# Generate with long context (processed in windows)
input_text = load_text("long_book.txt")
output = model.generate(
    input_text,
    max_new_tokens=2048,
    temperature=0.8,
)

# Model internally manages compression across layers
```

## Tradeoffs

### Advantages
✅ **8× memory reduction**: From 64K to ~224 tokens effective memory
✅ **Maintains performance**: Better perplexity than Transformer-XL
✅ **Hierarchical design**: Short-term for recency, long-term for history
✅ **Works with short windows**: 512-token windows handle long documents
✅ **Incremental compression**: No need to store full history
✅ **Trainable compression**: Learns optimal compression strategy

### Limitations
❌ **Information loss**: Compression discards some details
❌ **Architecture change**: Requires modified transformer layers
❌ **Training complexity**: Need to learn compression
❌ **Middle layer bottleneck**: All compression flows through single layer
❌ **Research phase**: Not in production models yet
❌ **Fixed compression ratios**: S and L must be predetermined

### Comparison: MELODI vs Alternatives

| Aspect | MELODI | Infini-Attention | RLM | Long Context |
|--------|---------|------------------|-----|-------------|
| Memory | O(n) 8× reduction | O(n) 114× | O(n) linear | O(n²) |
| Compute | O(n) slight overhead | O(n) | 2-3× baseline | 1× |
| Training | Required | Required | Not required | Not required |
| Architecture | Modified layers | Modified attention | Same model | Modified attention |
| Production | Research | Research | Production | Production |

## Integration Strategies

### 1. Adding MELODI to Existing Models

```python
import torch
import torch.nn as nn

def convert_to_melodi(base_model, config):
    """Convert standard transformer to MELODI"""

    # Extract layers
    layers = list(base_model.layers)

    # Replace with MELODI layers
    melodi_layers = []
    for i, layer in enumerate(layers):
        melodi_layer = MELODILayer(
            d_model=layer.d_model,
            n_heads=layer.n_heads,
            is_middle=(i == config.middle_layer)
        )

        # Copy weights where possible
        melodi_layer.attention.copy_weights(layer.attention)

        melodi_layers.append(melodi_layer)

    # Create new model
    melodi_model = nn.Sequential(*melodi_layers)

    return melodi_model
```

### 2. Fine-Tuning with Long Context

```python
# Fine-tune on long-context dataset
train_config = {
    "window_size": 512,  # Process in windows
    "overlap": 64,     # Maintain continuity
    "batch_size": 16,
    "gradient_accumulation": 4,  # Effective batch = 64
}

fine_tune(
    model=melodi_model,
    dataset=LongContextDataset(),
    config=train_config
)
```

### 3. Inference with Streaming

```python
def streaming_generate(model, long_document, query):
    """Process long document in windows"""

    results = []
    for window in sliding_windows(long_document, size=512, overlap=64):
        # Process window
        output = model(window)

        # Memory automatically carries forward
        results.append(output)

    # Aggregate results
    final_answer = aggregate(results, query)
    return final_answer
```

## Best Practices

### 1. Choose S and L Ratios

```python
# High recall: More short-term, less long-term
config = {"S": 192, "L": 32}  # Recommended by paper

# Balanced
config = {"S": 128, "L": 64}

# High precision
config = {"S": 192, "L": 96}
```

### 2. Set Middle Layer Position

```python
# Place middle layer at 40-60% of depth
middle_layer = int(num_layers * 0.5)  # Center

# Paper recommends: middle of network
# Example: 13 layers → layer 6 (0-indexed)
```

### 3. Use Reconstruction Loss

```python
# Add reconstruction loss during training
def training_loss(logits, targets, compressed, reconstructed):
    # Standard language modeling loss
    lm_loss = cross_entropy(logits, targets)

    # Reconstruction loss (preserve information)
    recon_loss = mse_loss(compressed, reconstructed)

    # Weighted sum
    total_loss = lm_loss + 0.1 * recon_loss

    return total_loss
```

### 4. Profile Memory Usage

```python
import torch

# Track memory during forward pass
with torch.cuda.profile_memory():
    output = model(input_tokens)

print(torch.cuda.memory_summary())
```

## Applications

### Ideal Use Cases
- **Document analysis**: Long reports, books, legal documents
- **Code understanding**: Large codebases with long contexts
- **Time-series processing**: Sequential data with long history
- **Memory-constrained deployment**: Edge devices with limited RAM

### Less Ideal Use Cases
- **Short conversations**: Benefits don't outweigh complexity
- **Low-latency requirements**: Compression adds overhead
- **Real-time inference**: Processing window latency

## Research Timeline

- **Oct 2024**: Original paper published on arXiv
- **Dec 2024**: ICLR 2025 acceptance
- **Apr 2025**: Meliad library released
- **June 2025**: Open-source implementation
- **Ongoing**: Integration into production models

## References

- [Paper (ICLR 2025)](https://openreview.net/pdf?id=TvGPP8i18S)
- [arXiv version](https://arxiv.org/abs/2410.03156)
- [Meliad library](https://github.com/google-research/meliad)
- [Google DeepMind blog](https://deepmind.google/research/publications/121073/)

---

*Last updated: April 13, 2026*
