use serial_test::serial;
use whitt_execution_engine::client::http_client::LlamaHttpClient;

const BASE_URL: &str = "http://localhost:8080";
const TEST_MODEL: &str = "Qwen2.5-0.5B-Instruct-Q4_K_M";

#[tokio::test]
#[ignore]
#[serial]
async fn test_list_models() {
    eprintln!("[TEST] test_list_models starting");
    let client = LlamaHttpClient::new(BASE_URL).expect("Failed to create client");
    client.wait_for_healthy(std::time::Duration::from_secs(30)).await.expect("Server not healthy");

    let models = client.list_models().await.expect("Failed to list models");
    eprintln!("[INFO] found {} models", models.len());
    for m in &models {
        eprintln!("  {} (status={})", m.id, m.status.value);
    }
    assert!(!models.is_empty(), "Model list should not be empty");
    assert!(models.iter().any(|m| m.id == TEST_MODEL), "Test model should be in list");
    eprintln!("[PASS] test_list_models");
}

#[tokio::test]
#[ignore]
#[serial]
async fn test_load_and_unload_model() {
    eprintln!("[TEST] test_load_and_unload_model starting");
    let client = LlamaHttpClient::new(BASE_URL).expect("Failed to create client");
    client.wait_for_healthy(std::time::Duration::from_secs(30)).await.expect("Server not healthy");

    let before = std::time::Instant::now();
    client.load_model(TEST_MODEL).await.expect("Failed to load model");
    let load_time = before.elapsed();
    eprintln!("[INFO] model loaded in {:?}", load_time);

    let models = client.list_models().await.expect("Failed to list models");
    let loaded = models.iter().find(|m| m.id == TEST_MODEL).expect("Test model not found");
    assert_eq!(loaded.status.value, "loaded", "Model should be loaded");

    let chat = whitt_execution_engine::client::types::ChatCompletionRequest {
        model: TEST_MODEL.to_string(),
        messages: vec![whitt_execution_engine::client::types::ChatMessage::user("Say hi")],
        max_tokens: Some(20),
        stream: false,
        ..Default::default()
    };
    let resp = client.chat_completion(chat).await.expect("Chat completion failed");
    assert!(!resp.choices.is_empty(), "Should have response");
    eprintln!("[INFO] chat response: {:?}", resp.choices[0].message.content);

    client.unload_model(TEST_MODEL).await.expect("Failed to unload model");
    let models = client.list_models().await.expect("Failed to list after unload");
    let unloaded = models.iter().find(|m| m.id == TEST_MODEL).expect("Test model not found");
    assert_eq!(unloaded.status.value, "unloaded", "Model should be unloaded");
    eprintln!("[PASS] test_load_and_unload_model (load_time={:?})", load_time);
}

#[tokio::test]
#[ignore]
#[serial]
async fn test_load_already_loaded_model() {
    eprintln!("[TEST] test_load_already_loaded_model starting");
    let client = LlamaHttpClient::new(BASE_URL).expect("Failed to create client");
    client.wait_for_healthy(std::time::Duration::from_secs(30)).await.expect("Server not healthy");

    client.load_model(TEST_MODEL).await.expect("First load failed");
    client.load_model(TEST_MODEL).await.expect("Second load of already loaded should succeed");
    let _ = client.unload_model(TEST_MODEL).await;
    eprintln!("[PASS] test_load_already_loaded_model");
}

#[tokio::test]
#[ignore]
#[serial]
async fn test_model_hot_swap() {
    eprintln!("[TEST] test_model_hot_swap starting");
    let client = LlamaHttpClient::new(BASE_URL).expect("Failed to create client");
    client.wait_for_healthy(std::time::Duration::from_secs(30)).await.expect("Server not healthy");

    client.load_model(TEST_MODEL).await.expect("Failed to load model");

    let chat = whitt_execution_engine::client::types::ChatCompletionRequest {
        model: TEST_MODEL.to_string(),
        messages: vec![whitt_execution_engine::client::types::ChatMessage::user("Say the word hello.")],
        max_tokens: Some(50),
        stream: false,
        ..Default::default()
    };
    let resp = client.chat_completion(chat).await.expect("Chat failed");
    assert!(!resp.choices[0].message.content.is_empty(), "Response should not be empty");

    client.unload_model(TEST_MODEL).await.expect("Unload failed");
    let models = client.list_models().await.expect("List failed");
    let m = models.iter().find(|m| m.id == TEST_MODEL).expect("Model not found");
    assert_eq!(m.status.value, "unloaded");
    eprintln!("[PASS] test_model_hot_swap");
}
