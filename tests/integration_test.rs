use reqwest::{Client, ClientBuilder};
use serde::Deserialize;
use std::sync::LazyLock;
use std::time::Duration;
use tokio::time::sleep;

static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    ClientBuilder::new()
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(120))
        .pool_idle_timeout(Duration::from_secs(90))
        .pool_max_idle_per_host(4)
        .tcp_keepalive(Duration::from_secs(30))
        .build()
        .expect("Failed to build shared HTTP client")
});

const BASE_URL: &str = "http://localhost:8080";
const TEST_MODEL: &str = "Qwen2.5-0.5B-Instruct-Q4_K_M";

async fn wait_for_server(max_wait: Duration) {
    let start = std::time::Instant::now();
    loop {
        match CLIENT.get(format!("{}/health", BASE_URL)).send().await {
            Ok(resp) if resp.status().is_success() => {
                eprintln!("[READY] server healthy after {}ms", start.elapsed().as_millis());
                return;
            }
            Ok(resp) => {
                eprintln!("[WAIT] server responded {} (not healthy)", resp.status());
            }
            Err(e) => {
                eprintln!("[WAIT] server unreachable: {}", e);
            }
        }
        if start.elapsed() > max_wait {
            panic!("Server not healthy within {:?}", max_wait);
        }
        sleep(Duration::from_secs(2)).await;
    }
}

async fn post_with_retry(body: serde_json::Value) -> reqwest::Response {
    let max_retries = 10;
    let mut attempt = 0;
    loop {
        let req = CLIENT
            .post(format!("{}/v1/chat/completions", BASE_URL))
            .json(&body)
            .build()
            .expect("build failed");

        match CLIENT.execute(req.try_clone().unwrap()).await {
            Ok(resp) if resp.status().is_success() => return resp,
            Ok(resp) => {
                attempt += 1;
                if attempt >= max_retries {
                    panic!("[FAIL] POST returned {} after {} retries", resp.status(), max_retries);
                }
                eprintln!("[RETRY {}/{}] POST returned {}", attempt, max_retries, resp.status());
                wait_for_server(Duration::from_secs(30)).await;
            }
            Err(e) => {
                attempt += 1;
                if attempt >= max_retries {
                    panic!("[FAIL] POST failed after {} retries: {}", max_retries, e);
                }
                eprintln!("[RETRY {}/{}] POST failed: {}", attempt, max_retries, e);
                wait_for_server(Duration::from_secs(30)).await;
            }
        }
    }
}

#[derive(Debug, Deserialize)]
struct HealthResponse {
    status: String,
}

#[tokio::test]
#[ignore]
async fn test_integration_all() {
    wait_for_server(Duration::from_secs(60)).await;

    // --- Sub-test 1: Health ---
    eprintln!("\n[TEST 1/4] Health check");
    let health: HealthResponse = CLIENT
        .get(format!("{}/health", BASE_URL))
        .send()
        .await
        .expect("Health request failed")
        .json()
        .await
        .expect("Failed to parse health");
    assert_eq!(health.status, "ok");
    eprintln!("[PASS 1/4] Health check");

    eprintln!("\n[SETUP] Loading model {}", TEST_MODEL);
    let _ = CLIENT
        .post(format!("{}/models/load", BASE_URL))
        .json(&serde_json::json!({"model": TEST_MODEL}))
        .send()
        .await;
    let load_start = std::time::Instant::now();
    loop {
        let models: serde_json::Value = CLIENT
            .get(format!("{}/v1/models", BASE_URL))
            .send()
            .await
            .expect("list failed")
            .json()
            .await
            .expect("parse failed");
        let loaded = models["data"].as_array()
            .and_then(|arr| arr.iter().find(|m| m["id"] == TEST_MODEL))
            .and_then(|m| m["status"]["value"].as_str());
        if loaded == Some("loaded") { break; }
        if load_start.elapsed() > Duration::from_secs(120) {
            panic!("Model did not load within 120s");
        }
        sleep(Duration::from_millis(500)).await;
    }
    eprintln!("[SETUP] model loaded in {:?}", load_start.elapsed());

    // --- Sub-test 2: Chat completion ---
    eprintln!("\n[TEST 2/4] Chat completion (non-streaming)");
    let body = serde_json::json!({
        "model": TEST_MODEL,
        "messages": [{"role": "user", "content": "Say hello"}],
        "max_tokens": 50,
        "temperature": 0.7,
        "stream": false
    });

    let resp = post_with_retry(body).await;
    let json: serde_json::Value = resp.json().await.expect("Failed to parse chat response");
    assert_eq!(json["choices"][0]["message"]["role"], "assistant");
    assert!(json["choices"][0]["message"]["content"].is_string());
    assert!(json["usage"]["total_tokens"].is_number());
    eprintln!("[PASS 2/4] Chat completion content={:?}", json["choices"][0]["message"]["content"]);

    // --- Sub-test 3: Streaming completion ---
    eprintln!("\n[TEST 3/4] Streaming completion");
    let body = serde_json::json!({
        "model": TEST_MODEL,
        "messages": [{"role": "user", "content": "Count from 1 to 5"}],
        "max_tokens": 50,
        "temperature": 0.7,
        "stream": true
    });

    let resp = post_with_retry(body).await;
    assert_eq!(resp.headers().get("content-type").unwrap(), "text/event-stream");

    let mut stream = resp.bytes_stream();
    let mut chunks_received = 0;
    let mut buffer = String::new();

    use futures::StreamExt;
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.expect("Failed to read stream chunk");
        buffer.push_str(std::str::from_utf8(&chunk).expect("Invalid UTF-8"));

        while let Some(pos) = buffer.find('\n') {
            let line = buffer[..pos].to_string();
            buffer = buffer[pos + 1..].to_string();

            if line.starts_with("data: ") {
                let data = &line[6..];
                if data == "[DONE]" {
                    break;
                }
                chunks_received += 1;
            }
        }
    }
    assert!(chunks_received > 0, "No SSE chunks received");
    eprintln!("[PASS 3/4] Streaming completion received {} chunks", chunks_received);

    // --- Sub-test 4: End-to-end with content verification ---
    eprintln!("\n[TEST 4/4] End-to-end (math question)");
    let body = serde_json::json!({
        "model": TEST_MODEL,
        "messages": [{"role": "user", "content": "What is 2 + 2? Answer with just the number."}],
        "max_tokens": 20,
        "temperature": 0.7,
        "stream": false
    });

    let resp = post_with_retry(body).await;
    let json: serde_json::Value = resp.json().await.expect("Failed to parse e2e response");
    assert!(json["choices"][0]["message"]["content"].is_string());
    assert!(json["usage"]["total_tokens"].as_u64().unwrap() > 0);

    let content = json["choices"][0]["message"]["content"].as_str().unwrap();
    assert!(content.contains('4') || content.contains("four"), "Expected '4' or 'four' in response, got: {:?}", content);
    eprintln!("[PASS 4/4] End-to-end response: {:?}", content);

    eprintln!("\n[ALL PASS] 4/4 integration tests succeeded");

    let _ = CLIENT
        .post(format!("{}/models/unload", BASE_URL))
        .json(&serde_json::json!({"model": TEST_MODEL}))
        .send()
        .await;
}
