# RAG Implementations with Ollama & llama.cpp

## Executive Summary

**Status**: Production-ready, multiple open-source implementations
**Purpose**: Extend LLM context windows using external memory via vector databases
**Performance**: Near-infinite context with local privacy
**Technologies**: Ollama, llama.cpp, ChromaDB, FAISS, Weaviate

## Overview

Retrieval-Augmented Generation (RAG) enables LLMs to access knowledge beyond their context windows:

```
┌─────────────────────────────────────────────────────────┐
│                    Knowledge Base                  │
│  (Documents, Code, Data)                      │
│                                                      │
│  1. Chunking: Split into segments           │
│  2. Embedding: Vector representation      │
│  3. Indexing: Store in vector DB         │
└──────────────────┬──────────────────────────────┘
                   │
                   ↓
┌─────────────────────────────────────────────────────────┐
│                Vector Database                    │
│  (ChromaDB, FAISS, Weaviate, Qdrant)          │
│                                                      │
│  Semantic Search: Find relevant chunks             │
└──────────────────┬──────────────────────────────┘
                   │
                   ↓
┌─────────────────────────────────────────────────────────┐
│              User Query                           │
│                                                      │
│  1. Embed query                              │
│  2. Search vector DB                           │
│  3. Retrieve top-K chunks                     │
└──────────────────┬──────────────────────────────┘
                   │
                   ↓
┌─────────────────────────────────────────────────────────┐
│            LLM Generation (Ollama/llama.cpp)     │
│                                                      │
│  Prompt = Query + Retrieved Context              │
│  Response = LLM.complete(prompt)              │
└─────────────────────────────────────────────────────────┘
```

## Architecture Components

### 1. Document Ingestion Pipeline

```python
class DocumentIngester:
    def __init__(self, chunk_size=512, overlap=64):
        self.chunk_size = chunk_size
        self.overlap = overlap
        self.embedder = OllamaEmbeddings()

    def ingest(self, documents):
        """Process documents into indexed chunks"""
        chunks = []

        for doc in documents:
            # Split into overlapping chunks
            doc_chunks = self._chunk_document(doc)

            # Generate embeddings
            embeddings = self.embedder.embed_batch(doc_chunks)

            # Create chunk metadata
            for chunk, embedding in zip(doc_chunks, embeddings):
                chunks.append({
                    "text": chunk,
                    "embedding": embedding,
                    "metadata": {
                        "source": doc.source,
                        "page": doc.page,
                        "timestamp": doc.timestamp
                    }
                })

        return chunks

    def _chunk_document(self, doc):
        """Split document into overlapping chunks"""
        chunks = []
        for i in range(0, len(doc.text), self.chunk_size - self.overlap):
            chunk = doc.text[i:i+self.chunk_size]
            chunks.append(chunk)
        return chunks
```

### 2. Embedding with Ollama

```python
import ollama
from typing import List

class OllamaEmbeddings:
    def __init__(self, model="mxbai-embed-large"):
        self.client = ollama.Client()
        self.model = model

    def embed(self, text: str) -> List[float]:
        """Generate single embedding"""
        response = self.client.embeddings(
            model=self.model,
            prompt=text
        )
        return response["embedding"]

    def embed_batch(self, texts: List[str]) -> List[List[float]]:
        """Generate embeddings for batch of texts"""
        return [self.embed(text) for text in texts]

    def embed_file(self, filepath: str) -> List[float]:
        """Generate embedding for file content"""
        with open(filepath, 'r') as f:
            content = f.read()
        return self.embed(content)
```

### 3. Vector Database Integration

#### ChromaDB (Local Disk-Based)

```python
import chromadb
from chromadb.config import Settings

class ChromaVectorStore:
    def __init__(self, path="./chroma_db"):
        self.client = chromadb.PersistentClient(
            path=path,
            settings=Settings(
                anonymized_telemetry=False,
                allow_reset=True
            )
        )
        self.collection = None

    def create_collection(self, name):
        """Create or get collection"""
        self.collection = self.client.get_or_create_collection(
            name=name,
            metadata={"hnsw:space": "cosine"}
        )

    def add(self, chunks):
        """Index chunks"""
        embeddings = [chunk["embedding"] for chunk in chunks]
        documents = [chunk["text"] for chunk in chunks]
        metadatas = [chunk["metadata"] for chunk in chunks]

        self.collection.add(
            embeddings=embeddings,
            documents=documents,
            metadatas=metadatas,
            ids=[f"chunk_{i}" for i in range(len(chunks))]
        )

    def query(self, query_embedding, top_k=5):
        """Search for relevant chunks"""
        results = self.collection.query(
            query_embeddings=[query_embedding],
            n_results=top_k
        )

        return [
            {
                "text": result["documents"][0],
                "metadata": result["metadatas"][0],
                "score": result["distances"][0]
            }
            for result in results["documents"][0]
        ]
```

#### FAISS (In-Memory)

```python
import faiss
import numpy as np

class FAISSVectorStore:
    def __init__(self, dimension=768):
        self.dimension = dimension
        self.index = faiss.IndexFlatL2(dimension)
        self.embeddings = []
        self.metadata = []

    def add(self, chunks):
        """Add embeddings to index"""
        embeddings = np.array([
            chunk["embedding"] for chunk in chunks
        ]).astype('float32')

        self.index.add(embeddings)
        self.embeddings.extend(embeddings)
        self.metadata.extend([chunk["metadata"] for chunk in chunks])

    def query(self, query_embedding, top_k=5):
        """Search index"""
        query_vector = np.array([query_embedding]).astype('float32')

        distances, indices = self.index.search(query_vector, top_k)

        results = []
        for distance, idx in zip(distances[0], indices[0]):
            results.append({
                "text": self.embeddings[idx],
                "metadata": self.metadata[idx],
                "score": float(distance)
            })

        return results
```

#### Weaviate (Cloud/Local)

```python
import weaviate

class WeaviateVectorStore:
    def __init__(self, url="http://localhost:8080"):
        self.client = weaviate.Client(url)
        self.collection = None

    def create_collection(self, name):
        """Create Weaviate collection"""
        self.collection = self.client.collections.create(
            name=name,
            properties=[
                weaviate.Property(
                    name="text",
                    data_type=weaviate.DataType.TEXT
                ),
                weaviate.Property(
                    name="metadata",
                    data_type=weaviate.DataType.OBJECT
                )
            ],
            vectorizer_config=weaviate.Configure.Vectorizer.text2Vec(
                skip=False,
                vectorize_collection_name=name
            )
        )

    def add(self, chunks):
        """Add objects to collection"""
        with self.collection.batch.dynamic() as batch:
            for chunk in chunks:
                batch.add_object(
                    properties={
                        "text": chunk["text"],
                        "metadata": chunk["metadata"]
                    },
                    vector=chunk["embedding"]
                )

    def query(self, query_embedding, limit=5):
        """Search Weaviate"""
        results = self.collection.query.hybrid(
            alpha=0.7,  # Weight fusion
            query=chunk["text"],
            vector=query_embedding,
            limit=limit
        )

        return results
```

### 4. RAG Query Pipeline

```python
class RAGPipeline:
    def __init__(self, vector_store, llm_model="llama3.2"):
        self.vector_store = vector_store
        self.embedder = OllamaEmbeddings()
        self.llm = ollama.Client()

    def query(self, user_question: str, top_k=5):
        """Execute RAG query"""

        # 1. Embed user question
        query_embedding = self.embedder.embed(user_question)

        # 2. Retrieve relevant chunks
        retrieved_chunks = self.vector_store.query(
            query_embedding=query_embedding,
            top_k=top_k
        )

        # 3. Construct prompt with context
        context = self._build_context(retrieved_chunks)
        prompt = f"""
        Answer the question based on the provided context.

        Context:
        {context}

        Question: {user_question}

        Answer:
        """

        # 4. Generate response with local LLM
        response = self.llm.generate(
            model=self.llm_model,
            prompt=prompt
        )

        return {
            "answer": response["response"],
            "sources": [chunk["metadata"] for chunk in retrieved_chunks],
            "confidence": self._calculate_confidence(retrieved_chunks)
        }

    def _build_context(self, chunks):
        """Format retrieved chunks into context"""
        context_parts = []
        for i, chunk in enumerate(chunks):
            context_parts.append(f"""
            Source {i+1}: {chunk['metadata']['source']}
            ---
            {chunk['text']}
            """)
        return "\n\n".join(context_parts)

    def _calculate_confidence(self, chunks):
        """Calculate confidence based on retrieval scores"""
        if not chunks:
            return 0.0

        scores = [chunk["score"] for chunk in chunks]
        avg_score = sum(scores) / len(scores)

        # Normalize to 0-1 (lower is better)
        return max(0.0, 1.0 - (avg_score / 2.0))
```

## Open Source Implementations

### Python-Based RAG Systems

#### 1. [digithree/ollama-rag](https://github.com/digithree/ollama-rag) (120★)

**Features**:
- Web interface for easy interaction
- ChromaDB integration
- Multiple document formats (PDF, Markdown, text)
- Automatic chunking and indexing
- Configurable retrieval parameters

**Installation**:
```bash
git clone https://github.com/digithree/ollama-rag.git
cd ollama-rag
pip install -r requirements.txt
python app.py
```

#### 2. [NidarshN/local_rag_ollama](https://github.com/NidarshN/local_rag_ollama) (0★)

**Features**:
- CLI-based PDF RAG
- ChromaDB vector store
- Ollama for embeddings and generation
- `uv` for dependency management

**Usage**:
```bash
# Populate database
uv run main.py --populate --data-path ./docs

# Query
uv run main.py --query-text "What is the API key format?"
```

#### 3. [cpepper96/ollama-local-rag](https://github.com/cpepper96/ollama-local-rag) (20★)

**Features**:
- LangChain integration
- FAISS vector store
- Markdown document support
- Simple question answering

**Installation**:
```bash
pip install -r requirements.txt
python create_database.py  # Index documents
python query_data.py "Your question here"
```

#### 4. [Vinci141/ollama-local-rag-setup](https://github.com/Vinci141/ollama-local-rag-setup) (3★)

**Features**:
- FAISS-based vector search
- `mxbai-embed-large` embeddings via Ollama
- Automatic document indexing
- CLI interface

**Setup**:
```bash
ollama pull mxbai-embed-large
ollama pull llama3

python Ollama_setup.py --index --folder "C:\path\to\docs"
python Ollama_setup.py --query "Summarize project goals"
```

#### 5. [ryanm101/LocalLLMRAG](https://github.com/ryanm101/LocalLLMRAG) (10★)

**Features**:
- RAG for code with auto-updates
- Monitors file system for changes
- ChromaDB persistence
- Configurable via YAML
- Supports multiple file types (.py, .java, .js, etc.)

**Configuration**:
```yaml
global:
  include_file_types:
    - .py
    - .java
    - .js
    - .cpp
    - .ts
  exclude_dirs:
    - venv
    - .venv
    - node_modules
    - __pycache__

vector_db_dir: "./chroma_db_code"
llm_model: "llama3.1"
```

#### 6. [tyrell/llm-ollama-llamaindex-bootstrap](https://github.com/tyrell/llm-ollama-llamaindex-bootstrap) (47★)

**Features**:
- Offline RAG application template
- Weaviate vector store
- LlamaIndex indexing
- Docker-based deployment
- Supports PDF ingestion

**Stack**:
- Ollama: Local LLM
- Weaviate: Vector store (Docker)
- LlamaIndex: Index orchestration
- LangChain: Framework integration

### C++ Implementation

#### [JG-Adams/Ollama-RAG-hpp](https://github.com/JG-Adams/Ollama-RAG-hpp) (6★)

**Features**:
- Header-only C++ library
- Ollama-hpp integration
- Cosine similarity retrieval
- No external dependencies

**Installation**:
```bash
# Clone
git clone https://github.com/jmont-dev/ollama-hpp.git
cd ollama-hpp

# Clone RAG library
git clone https://github.com/JG-Adams/Ollama-RAG-hpp.git

# Build (requires C++17)
mkdir build && cd build
cmake ..
make
```

## Performance Optimization

### 1. Chunking Strategy

```python
class SemanticChunker:
    def __init__(self, chunk_size=512):
        self.chunk_size = chunk_size

    def chunk(self, text):
        """Semantic chunking based on paragraphs"""
        chunks = []
        paragraphs = text.split('\n\n')

        current_chunk = ""
        for paragraph in paragraphs:
            if len(current_chunk) + len(paragraph) < self.chunk_size:
                current_chunk += paragraph + "\n\n"
            else:
                chunks.append(current_chunk)
                current_chunk = paragraph + "\n\n"

        if current_chunk:
            chunks.append(current_chunk)

        return chunks

    def chunk_with_overlap(self, text, overlap=64):
        """Fixed-size chunks with overlap"""
        chunks = []
        for i in range(0, len(text), self.chunk_size - overlap):
            chunks.append(text[i:i+self.chunk_size])
        return chunks
```

### 2. Batch Embedding

```python
def batch_embed(texts, batch_size=32, embedder):
    """Process embeddings in batches"""
    all_embeddings = []

    for i in range(0, len(texts), batch_size):
        batch = texts[i:i+batch_size]
        embeddings = embedder.embed_batch(batch)
        all_embeddings.extend(embeddings)

    return all_embeddings
```

### 3. Hybrid Search

```python
def hybrid_search(vector_db, query_embedding, query_text, alpha=0.7):
    """Combine semantic and keyword search"""

    # Semantic search (vector DB)
    semantic_results = vector_db.query(query_embedding, top_k=5)

    # Keyword search (if supported)
    keyword_results = vector_db.keyword_search(query_text, top_k=5)

    # Combine with weighted fusion
    combined = []
    for semantic, keyword in zip(semantic_results, keyword_results):
        combined.append({
            "score": alpha * semantic["score"] +
                     (1 - alpha) * keyword["score"],
            "text": semantic["text"]
        })

    # Sort by combined score
    combined.sort(key=lambda x: x["score"], reverse=True)

    return combined[:5]
```

### 4. Context Window Optimization

```python
def optimize_context_length(retrieved_chunks, max_tokens=2000):
    """Select chunks to fit in context window"""

    selected = []
    total_tokens = 0

    for chunk in retrieved_chunks:
        chunk_tokens = len(chunk["text"].split())
        if total_tokens + chunk_tokens <= max_tokens:
            selected.append(chunk)
            total_tokens += chunk_tokens
        else:
            break

    return selected
```

## Best Practices

### 1. Document Preprocessing

```python
def preprocess_document(text):
    """Clean and normalize text"""
    # Remove extra whitespace
    text = " ".join(text.split())

    # Fix unicode issues
    text = text.encode('utf-8', errors='ignore').decode('utf-8')

    # Normalize line endings
    text = text.replace('\r\n', '\n')

    return text
```

### 2. Embedding Model Selection

```python
# Fast, lower quality
embedder = OllamaEmbeddings(model="nomic-embed-text")

# Balanced (recommended)
embedder = OllamaEmbeddings(model="mxbai-embed-large")

# Slow, higher quality
embedder = OllamaEmbeddings(model="bge-large-en-v1.5")
```

### 3. Retrieval Parameters

```python
# Top-K retrieval
results = vector_store.query(query_embedding, top_k=5)

# Score threshold
filtered = [r for r in results if r["score"] < 0.3]

# Minimum score threshold
filtered = [r for r in results if r["score"] > 0.7]
```

### 4. Prompt Engineering

```python
def build_rag_prompt(query, context_chunks):
    """Construct effective RAG prompt"""
    context = "\n\n---\n\n".join([
        f"Document {i+1}:\n{chunk['text']}"
        for i, chunk in enumerate(context_chunks)
    ])

    prompt = f"""
    You are a helpful assistant. Use the following context to answer the question.

    Context:
    {context}

    Question: {query}

    Instructions:
    - Use only the provided context to answer
    - If the answer is not in the context, say "I don't know"
    - Be concise and direct
    - Cite the document number when relevant

    Answer:
    """

    return prompt
```

## Troubleshooting

### Poor Retrieval Quality

**Problem**: Retrieved chunks not relevant

**Solutions**:
1. Improve chunking strategy (semantic vs fixed-size)
2. Try different embedding model
3. Increase top-K retrieval
4. Add hybrid search (semantic + keyword)
5. Re-index with better preprocessing

### Slow Query Performance

**Problem**: Queries take too long

**Solutions**:
1. Use FAISS instead of ChromaDB for in-memory search
2. Index embeddings offline
3. Use GPU-accelerated embeddings (Ollama with Vulkan)
4. Cache frequent queries
5. Limit retrieval to top-K chunks

### High Memory Usage

**Problem**: Vector database consumes too much RAM

**Solutions**:
1. Use disk-based ChromaDB instead of in-memory
2. Quantize embeddings (float16 instead of float32)
3. Use approximate search (HNSW in FAISS)
4. Limit indexed documents
5. Regular cleanup of old entries

## Applications

### Ideal Use Cases
- **Document QA**: Search technical documentation
- **Code understanding**: Query large codebases
- **Knowledge bases**: Internal company wikis
- **Research**: Literature review and paper analysis
- **Compliance**: Legal/medical document review

### Hardware Requirements

| Component | Minimum | Recommended |
|-----------|-----------|-------------|
| CPU | 4 cores | 8+ cores |
| RAM | 8GB | 16GB+ |
| GPU | None | 8GB+ VRAM |
| Storage | 10GB SSD | 100GB+ SSD |

## Comparison: Vector Databases

| Feature | ChromaDB | FAISS | Weaviate |
|---------|----------|-------|----------|
| Type | Disk-based | In-memory | Hybrid |
| Persistence | Yes | No | Yes |
| Setup | Easiest | Hardest | Medium |
| Performance | Medium | Fastest | Fast |
| Scaling | Good | Excellent | Excellent |
| Cost | Free | Free | Free/Paid |

## References

- [Ollama Documentation](https://ollama.ai/)
- [llama.cpp GitHub](https://github.com/ggml-org/llama.cpp)
- [ChromaDB](https://www.trychroma.com/)
- [FAISS](https://github.com/facebookresearch/faiss)
- [Weaviate](https://weaviate.io/)

---

*Last updated: April 13, 2026*
