# Model Router System Design 3: Node.js/LevelDB/TensorFlow.js Stack

## Executive Summary

A JavaScript-based model routing system using Node.js for the runtime, LevelDB for embedded key-value storage, and TensorFlow.js for vector encoding. Designed for ease of development, rapid prototyping, and integration with existing JavaScript ecosystems.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                         Express Server                              │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  Node.js Runtime (event loop, async/await)                  │   │
│  └──────────────────────────────────────────────────────────────┘   │
└────────────────────┬────────────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────────────┐
│                   Service Layer                                     │
│  ┌──────────────────┐  ┌──────────────────┐  ┌────────────────┐  │
│  │ TF.js Encoder    │  │ Similarity Engine│  │ Rank Engine    │  │
│  │ (Universal Sent) │  │ (Matrix math)    │  │ (Weighted)     │  │
│  └──────────────────┘  └──────────────────┘  └────────────────┘  │
└────────────────────┬────────────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────────────┐
│                   Storage Layer (level-db)                           │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  Sublevel: specializations                                   │   │
│  │  Sublevel: embeddings                                        │   │
│  │  Sublevel: execution_logs                                    │   │
│  │  Sublevel: model_stats                                       │   │
│  └──────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
```

## Technology Stack

### Core Dependencies

| Component | Package | Size | Notes |
|-----------|---------|------|-------|
| HTTP Server | express 4.18 | ~2MB | RESTful routing |
| Embedded DB | level 8.0 | ~1.5MB | Key-value store |
| Sublevels | sublevel 6.0 | ~500KB | Namespaced storage |
| TF.js | @tensorflow/tfjs-node 4.17 | ~100MB | Node.js bindings |
| Sentence Encoder | @tensorflow-models/universal-sentence-encoder | ~80MB | Pre-trained model |
| CSV | csv-parser 3.0 | ~500KB | Fast CSV parsing |
| Utils | lodash 4.17 | ~70KB | Utility functions |
| Hash | crypto (native) | - | Built-in |

### Total Memory Footprint
- **Minimal runtime**: ~50MB (without TF.js)
- **With TF.js model**: ~200MB (Universal Sentence Encoder)
- **With embeddings cached**: ~300MB (100 rows in RAM)

## Data Schema

### LevelDB Structure

```javascript
// Sublevel: specializations
// Key: spec_id (string)
// Value: JSON string of Specialization object

class Specialization {
  constructor() {
    this.row_kind = '';
    this.specialization_category = '';
    this.specialization_synonyms = '';
    this.related_synonym_tags = '';
    this.top1_model = '';
    this.top1_ctx = 0;
    this.top2_model = '';
    this.top2_ctx = 0;
    this.top3_model = '';
    this.top3_ctx = 0;
    this.wildcard_model = '';
    this.wildcard_ctx = 0;
    this.primary_strength = '';
    this.secondary_strengths = '';
    this.preferred_input_shape = '';
    this.preferred_output_shape = '';
    this.task_granularity = '';
    this.context_need = '';
    this.output_rigidity = '';
    this.verification_need = '';
    this.recommended_decoding = '';
    this.latency_bias = '';
    this.router_keywords = '';
    this.avoid_when = '';
    this.selection_notes = '';
  }
}

// Sublevel: embeddings
// Key: spec_id (string)
// Value: Float32Array (512 bytes, 128-dim * 4 bytes)

// Sublevel: execution_logs
// Key: timestamp_specId (string)
// Value: JSON string of ExecutionLog

class ExecutionLog {
  constructor() {
    this.promptHash = '';  // SHA256 hex
    this.promptPrefix = '';
    this.selectedModels = [];
    this.successfulModel = null;
    this.attempts = 0;
    this.success = false;
    this.latencyMs = 0;
    this.timestamp = 0;
    this.specializationId = 0;
  }
}

// Sublevel: model_stats
// Key: model_name
// Value: JSON string of ModelStats

class ModelStats {
  constructor() {
    this.totalAttempts = 0;
    this.successes = 0;
    this.failures = 0;
    this.avgAttemptsToSuccess = 0;
    this.penaltyScore = 0.0;
    this.lastUpdated = 0;
  }
}
```

## Core Implementation

### Vector Encoder (TF.js)

```javascript
const tf = require('@tensorflow/tfjs-node');
const use = require('@tensorflow-models/universal-sentence-encoder');

class VectorEncoder {
  constructor() {
    this.model = null;
    this.dim = 512;  // Universal Sentence Encoder output
  }

  async initialize() {
    this.model = await use.load();
    // Warm up
    await this.encodeText('warmup');
  }

  async encodeText(text) {
    if (!this.model) {
      await this.initialize();
    }

    const embeddings = await this.model.embed([text]);
    const result = await embeddings.data();
    embeddings.dispose();

    // Convert to regular array
    return Array.from(result);
  }

  async encodeSpecialization(spec) {
    const textParts = [
      spec.specializationCategory,
      spec.specializationSynonyms,
      spec.routerKeywords,
      spec.primaryStrength,
      spec.secondaryStrengths
    ].filter(Boolean);

    const combined = textParts.join(' | ');
    return this.encodeText(combined);
  }
}

module.exports = VectorEncoder;
```

### Similarity Engine

```javascript
const _ = require('lodash');

class SimilarityEngine {
  constructor() {
    this.embeddingsCache = null;
  }

  async loadEmbeddings(db) {
    const embeddingsLevel = db.sublevel('embeddings');
    const embeddings = [];

    for await (const [key, value] of embeddingsLevel.iterator()) {
      const specId = key.toString();
      const embedding = new Float32Array(value);
      embeddings.push({ specId, embedding });
    }

    this.embeddingsCache = embeddings;
  }

  findClosest(inputEmbedding, topK = 10) {
    if (!this.embeddingsCache || this.embeddingsCache.length === 0) {
      return [];
    }

    const similarities = this.embeddingsCache.map(({ specId, embedding }) => {
      const similarity = this.cosineSimilarity(inputEmbedding, embedding);
      return { specId, similarity };
    });

    // Sort by similarity descending
    similarities.sort((a, b) => b.similarity - a.similarity);

    // Return top-K
    return similarities.slice(0, topK);
  }

  cosineSimilarity(a, b) {
    let dotProduct = 0;
    let normA = 0;
    let normB = 0;

    for (let i = 0; i < a.length; i++) {
      dotProduct += a[i] * b[i];
      normA += a[i] * a[i];
      normB += b[i] * b[i];
    }

    normA = Math.sqrt(normA);
    normB = Math.sqrt(normB);

    if (normA === 0 || normB === 0) {
      return 0;
    }

    return dotProduct / (normA * normB);
  }
}

module.exports = SimilarityEngine;
```

### Ranking Engine

```javascript
class RankingEngine {
  constructor(config) {
    this.penaltyWeight = config?.penaltyWeight || 1.0;
    this.contextWeight = config?.contextWeight || 0.1;
  }

  rankModels(spec, similarity, modelStats) {
    const ranked = [];

    // Model candidates with position weights
    const candidates = [
      { model: spec.top1Model, posWeight: 1.0, ctxSize: spec.top1Ctx },
      { model: spec.top2Model, posWeight: 0.9, ctxSize: spec.top2Ctx },
      { model: spec.top3Model, posWeight: 0.8, ctxSize: spec.top3Ctx },
      { model: spec.wildcardModel, posWeight: 0.6, ctxSize: spec.wildcardCtx }
    ];

    for (const { model, posWeight, ctxSize } of candidates) {
      if (!model) continue;

      // Stage 1: Base score
      const baseScore = similarity * posWeight;

      // Stage 2: Apply log penalty
      const stats = modelStats[model] || { penaltyScore: 0 };
      const penalty = stats.penaltyScore || 0;
      const logAdjusted = baseScore * (1 - penalty);

      // Stage 3: Context bonus
      const ctxBonus = Math.min(ctxSize / 524288, 1) * this.contextWeight;

      // Stage 4: Final score
      const finalScore = logAdjusted + ctxBonus;

      ranked.push({ model, score: finalScore });
    }

    // Sort by score descending
    ranked.sort((a, b) => b.score - a.score);
    return ranked;
  }
}

module.exports = RankingEngine;
```

### Stats Updater

```javascript
const crypto = require('crypto');
const level = require('level');
const sublevel = require('sublevel');

class StatsUpdater {
  constructor(dbPath = './data/router.db') {
    this.db = level(dbPath, { valueEncoding: 'json' });
    this.statsLevel = this.db.sublevel('model_stats');
    this.logsLevel = this.db.sublevel('execution_logs');
  }

  async updateExecution(model, promptHash, attempts, success, latencyMs) {
    // Get or create stats
    let stats = await this.statsLevel.get(model).catch(() => null);

    if (!stats) {
      stats = {
        totalAttempts: 0,
        successes: 0,
        failures: 0,
        avgAttemptsToSuccess: 0,
        penaltyScore: 0,
        lastUpdated: 0
      };
    }

    // Update stats
    stats.totalAttempts++;

    if (success) {
      stats.successes++;
      // Update average attempts
      stats.avgAttemptsToSuccess =
        (stats.avgAttemptsToSuccess * (stats.successes - 1) + attempts) /
        stats.successes;
    } else {
      stats.failures++;
    }

    // Calculate penalty
    const failureRate = stats.failures / stats.totalAttempts;
    const attemptPenalty = Math.min(stats.avgAttemptsToSuccess / 10, 1);
    stats.penaltyScore = (failureRate * 0.7) + (attemptPenalty * 0.3);

    stats.lastUpdated = Date.now();

    // Save stats
    await this.statsLevel.put(model, stats);

    return stats;
  }

  async loadAllStats() {
    const stats = {};

    for await (const [model, data] of this.statsLevel.iterator()) {
      stats[model] = data;
    }

    return stats;
  }

  close() {
    this.db.close();
  }
}

module.exports = StatsUpdater;
```

### Agentic Executor

```javascript
class AgenticExecutor {
  constructor(config, statsUpdater) {
    this.maxRetriesPerModel = config?.maxRetriesPerModel || 2;
    this.maxModelsToTry = config?.maxModelsToTry || 3;
    this.timeoutMs = config?.timeoutMs || 30000;
    this.statsUpdater = statsUpdater;
  }

  async executeWithRetry(prompt, rankedModels, verifier) {
    const promptHash = crypto.createHash('sha256').update(prompt).digest('hex');

    for (let i = 0; i < Math.min(rankedModels.length, this.maxModelsToTry); i++) {
      const { model } = rankedModels[i];
      let modelSuccess = false;
      let modelAttempts = 0;
      const startTime = Date.now();

      for (let attempt = 0; attempt < this.maxRetriesPerModel; attempt++) {
        modelAttempts++;

        try {
          const result = await this.callModelWithTimeout(model, prompt);

          if (verifier(result)) {
            modelSuccess = true;
            const latencyMs = Date.now() - startTime;

            await this.statsUpdater.updateExecution(
              model, promptHash, modelAttempts, true, latencyMs
            );

            return { result, model };
          }
        } catch (error) {
          console.error(`Model ${model} attempt ${attempt + 1} failed:`, error.message);
        }
      }

      // Model failed all attempts
      const latencyMs = Date.now() - startTime;
      await this.statsUpdater.updateExecution(
        model, promptHash, modelAttempts, false, latencyMs
      );
    }

    throw new Error(`All ${this.maxModelsToTry} models failed`);
  }

  async callModelWithTimeout(model, prompt) {
    return new Promise((resolve, reject) => {
      const timeout = setTimeout(() => {
        reject(new Error('Timeout'));
      }, this.timeoutMs);

      this.callModel(model, prompt)
        .then(resolve)
        .catch(reject)
        .finally(() => clearTimeout(timeout));
    });
  }

  async callModel(model, prompt) {
    // Placeholder for actual LLM call
    // Could use fetch() to call local API
    return `Output from ${model} for: ${prompt}`;
  }
}

module.exports = AgenticExecutor;
```

### Express API

```javascript
const express = require('express');
const VectorEncoder = require('./services/VectorEncoder');
const SimilarityEngine = require('./services/SimilarityityEngine');
const RankingEngine = require('./services/RankingEngine');
const StatsUpdater = require('./services/StatsUpdater');
const AgenticExecutor = require('./services/AgenticExecutor');
const level = require('level');

class RouterServer {
  constructor(config) {
    this.app = express();
    this.config = config;

    this.initializeMiddleware();
    this.initializeRoutes();
  }

  initializeMiddleware() {
    this.app.use(express.json());

    // CORS
    this.app.use((req, res, next) => {
      res.header('Access-Control-Allow-Origin', '*');
      res.header('Access-Control-Allow-Methods', 'GET,POST,OPTIONS');
      res.header('Access-Control-Allow-Headers', 'Content-Type');
      next();
    });
  }

  async initialize() {
    // Initialize services
    this.db = level(this.config.dbPath, { valueEncoding: 'json' });
    this.encoder = new VectorEncoder();
    await this.encoder.initialize();

    this.similarity = new SimilarityEngine();
    await this.similarity.loadEmbeddings(this.db);

    this.ranking = new RankingEngine(this.config.ranking);
    this.statsUpdater = new StatsUpdater(this.config.dbPath);

    this.executor = new AgenticExecutor(
      this.config.retry,
      this.statsUpdater
    );

    console.log('Router initialized');
  }

  initializeRoutes() {
    // Health check
    this.app.get('/health', (req, res) => {
      res.json({ status: 'ok' });
    });

    // Route endpoint
    this.app.post('/route', async (req, res) => {
      try {
        const { prompt, maxModels = 3 } = req.body;

        // Encode input
        const inputEmbedding = await this.encoder.encodeText(prompt);

        // Find similar specializations
        const matches = this.similarity.findClosest(inputEmbedding, 10);

        if (matches.length === 0) {
          return res.status(404).json({ error: 'No matches found' });
        }

        // Get top specialization
        const topSpecId = matches[0].specId;
        const topScore = matches[0].similarity;

        const specsLevel = this.db.sublevel('specializations');
        const specData = await specsLevel.get(topSpecId);

        if (!specData) {
          return res.status(404).json({ error: 'Specialization not found' });
        }

        // Load all model stats
        const modelStats = await this.statsUpdater.loadAllStats();

        // Rank models
        const ranked = this.ranking.rankModels(specData, topScore, modelStats);

        // Response
        res.json({
          rankedModels: ranked.slice(0, maxModels),
          topSpecialization: specData,
          similarity: topScore,
          allMatches: matches.slice(0, 5)
        });

      } catch (error) {
        console.error('Route error:', error);
        res.status(500).json({ error: error.message });
      }
    });

    // Execute endpoint (agentic)
    this.app.post('/execute', async (req, res) => {
      try {
        const { prompt, maxModels = 3 } = req.body;

        // First, route to get ranked models
        const inputEmbedding = await this.encoder.encodeText(prompt);
        const matches = this.similarity.findClosest(inputEmbedding, 1);
        const topSpecId = matches[0].specId;
        const topScore = matches[0].similarity;

        const specsLevel = this.db.sublevel('specializations');
        const specData = await specsLevel.get(topSpecId);

        const modelStats = await this.statsUpdater.loadAllStats();
        const ranked = this.ranking.rankModels(specData, topScore, modelStats);

        // Execute with retry
        const verifier = (result) => result && result.length > 0;
        const { result, model } = await this.executor.executeWithRetry(
          prompt, ranked.slice(0, maxModels), verifier
        );

        res.json({
          result,
          model,
          rankedModels: ranked.slice(0, maxModels)
        });

      } catch (error) {
        console.error('Execute error:', error);
        res.status(500).json({ error: error.message });
      }
    });
  }

  start(port = 3000) {
    this.app.listen(port, () => {
      console.log(`Router server listening on port ${port}`);
    });
  }
}

// Main entry point
async function main() {
  const config = {
    dbPath: './data/router.db',
    retry: {
      maxRetriesPerModel: 2,
      maxModelsToTry: 3,
      timeoutMs: 30000
    },
    ranking: {
      penaltyWeight: 1.0,
      contextWeight: 0.1
    }
  };

  const server = new RouterServer(config);
  await server.initialize();
  server.start(3000);
}

if (require.main === module) {
  main().catch(console.error);
}

module.exports = RouterServer;
```

## Configuration

```json
{
  "server": {
    "port": 3000,
    "host": "0.0.0.0"
  },
  "db": {
    "path": "./data/router.db"
  },
  "embedding": {
    "model": "universal-sentence-encoder",
    "cacheEmbeddings": true
  },
  "routing": {
    "topK": 10,
    "similarityThreshold": 0.3,
    "contextWeight": 0.1
  },
  "retry": {
    "maxRetriesPerModel": 2,
    "maxModelsToTry": 3,
    "timeoutMs": 30000
  },
  "penalty": {
    "failureRateWeight": 0.7,
    "attemptPenaltyWeight": 0.3,
    "maxPenalty": 1.0
  }
}
```

## Performance Characteristics

| Metric | Value | Notes |
|--------|-------|-------|
| Cold start time | ~3s | Load TF.js model |
| Embedding time | ~80ms | Universal Sentence Encoder |
| Similarity search | ~10ms | JS loops, 100 rows |
| Full route | ~200ms | End-to-end |
| Memory (idle) | ~50MB | Node.js baseline |
| Memory (active) | ~300MB | With TF.js model |
| Max QPS | ~100 | Single thread |
| Concurrent requests | 1K+ | Event loop |

## Advantages

1. **Rapid development**: Familiar JS/TS ecosystem
2. **Great tooling**: npm, debugger, hot reload
3. **Easy integration**: Works with existing JS apps
4. **TypeScript support**: Strong typing available
5. **Large community**: Many packages available

## Limitations

1. **Single-threaded**: Event loop can bottleneck
2. **Memory overhead**: V8 GC pauses
3. **TF.js limitations**: Slower than native TF
4. **No true concurrency**: Parallelism via workers only

## Extensions

1. **Worker threads**: Offload embedding to workers
2. **TypeScript migration**: Add type safety
3. **Redis backend**: Replace LevelDB for distributed
4. **WebSocket API**: Real-time routing updates
5. **Docker container**: Easy deployment
