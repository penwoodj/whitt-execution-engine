use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct HealthResponse {
    status: String,
}

#[tokio::test]
#[ignore] // Run with: cargo test --test integration_test -- --ignored
async fn test_server_health() {
    let client = Client::new();
    let url = "http://localhost:8080/health";

    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());

    let health: HealthResponse = response
        .json()
        .await
        .expect("Failed to parse response");

    assert_eq!(health.status, "ok");
}

#[tokio::test]
#[ignore]
async fn test_chat_completion() {
    let client = Client::new();
    let url = "http://localhost:8080/v1/chat/completions";

    let request = serde_json::json!({
        "model": "model",
        "messages": [
            {"role": "user", "content": "Say hello"}
        ],
        "max_tokens": 50,
        "temperature": 0.7,
        "stream": false
    });

    let response = client
        .post(url)
        .json(&request)
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());

    let json: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response");

    assert_eq!(json["choices"][0]["message"]["role"], "assistant");
    assert!(json["choices"][0]["message"]["content"].is_string());
    assert!(json["usage"]["total_tokens"].is_number());
}

#[tokio::test]
#[ignore]
async fn test_streaming_completion() {
    let client = Client::new();
    let url = "http://localhost:8080/v1/chat/completions";

    let request = serde_json::json!({
        "model": "model",
        "messages": [
            {"role": "user", "content": "Count from 1 to 5"}
        ],
        "max_tokens": 50,
        "temperature": 0.7,
        "stream": true
    });

    let response = client
        .post(url)
        .json(&request)
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "text/event-stream"
    );

    let mut stream = response.bytes_stream();
    let mut chunks_received = 0;
    let mut buffer = String::new();

    use futures::StreamExt;
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.expect("Failed to read stream chunk");
        buffer.push_str(std::str::from_utf8(&chunk).expect("Invalid UTF-8"));

        // Parse SSE events
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

    assert!(chunks_received > 0, "No chunks received");
}

#[tokio::test]
#[ignore]
async fn test_end_to_end() {
    // This test assumes server is running
    let url = "http://localhost:8080";

    // 1. Health check
    let client = Client::new();
    let health: HealthResponse = client
        .get(&format!("{}/health", url))
        .send()
        .await
        .expect("Health check failed")
        .json()
        .await
        .expect("Failed to parse health");

    assert_eq!(health.status, "ok");

    // 2. Send request
    let request = serde_json::json!({
        "model": "model",
        "messages": [
            {"role": "user", "content": "What is 2 + 2?"}
        ],
        "max_tokens": 20,
        "stream": false
    });

    let response = client
        .post(&format!("{}/v1/chat/completions", url))
        .json(&request)
        .send()
        .await
        .expect("Request failed");

    assert!(response.status().is_success());

    let json: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response");

    assert!(json["choices"][0]["message"]["content"].is_string());
    assert!(json["usage"]["total_tokens"].as_u64().unwrap() > 0);

    // 3. Verify response contains "4"
    let content = json["choices"][0]["message"]["content"].as_str().unwrap();
    assert!(content.contains('4') || content.contains("four"));
}
