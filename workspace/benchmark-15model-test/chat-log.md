# Benchmark Chat Log

Run: unix_epoch_1778897294s  
Server: http://localhost:8080  
Models: Qwen2.5-0.5B-Instruct-Q4_K_M.gguf  

---

## Qwen2.5-0.5B-Instruct-Q4_K_M.gguf (GPU)

| Metric | Value |
|---|---|
| Load | 3024ms |
| Unload | 508ms |
| Total | 4761ms |
| Tokens/s | 23.77 |
| Avg Latency | 1220.21ms |

### Prompt 1 (unix_epoch_1778897299s)

**User**: Write a short JSON object with 3 keys: greeting, language, framework. Use string values.

**Model**: (49 prompt tokens, 29 completion tokens, 1220ms, 23.77 tok/s)

**Response**:

{
  "greeting": "Hello",
  "language": "en",
  "framework": "JavaScript"
}

---

## Qwen3-0.6B-Q8_0.gguf (GPU)

| Metric | Value |
|---|---|
| Load | 6557ms |
| Unload | 508ms |
| Total | 18524ms |
| Tokens/s | 17.98 |
| Avg Latency | 11457.80ms |

### Prompt 1 (unix_epoch_1778897320s)

**User**: Write a short JSON object with 3 keys: greeting, language, framework. Use string values.

**Model**: (28 prompt tokens, 206 completion tokens, 11457ms, 17.98 tok/s)

**Response**:

{
  "greeting": "Hello!",
  "language": "English",
  "framework": "React"
}

---

## Qwen3-4B-Instruct-2507-Q4_K_M.gguf (GPU)

| Metric | Value |
|---|---|
| Load | 20720ms |
| Unload | 509ms |
| Total | 26609ms |
| Tokens/s | 4.65 |
| Avg Latency | 5377.32ms |

### Prompt 1 (unix_epoch_1778897348s)

**User**: Write a short JSON object with 3 keys: greeting, language, framework. Use string values.

**Model**: (28 prompt tokens, 25 completion tokens, 5377ms, 4.65 tok/s)

**Response**:

{
  "greeting": "Hello",
  "language": "English",
  "framework": "React"
}

---

## llama-3.2-1b-instruct-q8_0.gguf (GPU)

| Metric | Value |
|---|---|
| Load | 9584ms |
| Unload | 510ms |
| Total | 14655ms |
| Tokens/s | 10.97 |
| Avg Latency | 4556.48ms |

### Prompt 1 (unix_epoch_1778897365s)

**User**: Write a short JSON object with 3 keys: greeting, language, framework. Use string values.

**Model**: (30 prompt tokens, 50 completion tokens, 4556ms, 10.97 tok/s)

**Response**:

{
    "greeting": "Hello",
    "language": "English",
    "framework": "Django"
}

---

## gemma-3-1B-it-QAT-Q4_0.gguf (GPU)

| Metric | Value |
|---|---|
| Load | 6051ms |
| Unload | 507ms |
| Total | 8152ms |
| Tokens/s | 21.38 |
| Avg Latency | 1590.33ms |

### Prompt 1 (unix_epoch_1778897375s)

**User**: Write a short JSON object with 3 keys: greeting, language, framework. Use string values.

**Model**: (29 prompt tokens, 34 completion tokens, 1590ms, 21.38 tok/s)

**Response**:

{
  "greeting": "Hello!",
  "language": "English",
  "framework": "Python"
}

---

