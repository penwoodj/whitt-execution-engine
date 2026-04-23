use whitt_execution_engine::client::http_client::LlamaHttpClient;
use whitt_execution_engine::client::prompt_chain::PromptChain;

const BASE_URL: &str = "http://localhost:8080";
const TEST_MODEL: &str = "Qwen2.5-0.5B-Instruct-Q4_K_M";

async fn ensure_model_loaded(client: &LlamaHttpClient) {
    let _ = client.load_model(TEST_MODEL).await;
}

#[tokio::test]
#[ignore]
async fn test_prompt_chain_all() {
    let client = LlamaHttpClient::new(BASE_URL)
        .expect("Failed to create client");

    client.wait_for_healthy(std::time::Duration::from_secs(60)).await
        .expect("Server not healthy");
    ensure_model_loaded(&client).await;

    // Multi-step chain
    eprintln!("[TEST 1/2] Multi-step chain");
    let mut chain = PromptChain::new(client, TEST_MODEL);

    let result1 = chain
        .step("What is 2+2? Answer with just the number.")
        .await
        .expect("Step 1 failed");
    assert!(!result1.content.is_empty(), "Step 1 content is empty");
    assert_eq!(result1.step_number, 1);
    assert_eq!(chain.step_count(), 1);
    assert_eq!(chain.history().len(), 2);
    assert!(chain.last_response().is_some());
    eprintln!("[STEP 1] content={:?}", result1.content);

    let result2 = chain
        .step("Take that number and multiply it by 3. What do you get?")
        .await
        .expect("Step 2 failed");
    assert!(!result2.content.is_empty(), "Step 2 content is empty");
    assert_eq!(result2.step_number, 2);
    assert_eq!(chain.step_count(), 2);
    assert_eq!(chain.history().len(), 4);
    eprintln!("[STEP 2] content={:?}", result2.content);

    let result3 = chain
        .step("What is the square root of your previous answer?")
        .await
        .expect("Step 3 failed");
    assert!(!result3.content.is_empty(), "Step 3 content is empty");
    assert_eq!(result3.step_number, 3);
    assert_eq!(chain.step_count(), 3);
    assert_eq!(chain.history().len(), 6);
    eprintln!("[PASS 1/2] Multi-step chain 3/3 steps succeeded");

    // Chain with system prompt
    eprintln!("[TEST 2/2] Chain with system prompt");
    let client2 = LlamaHttpClient::new(BASE_URL)
        .expect("Failed to create client 2");
    client2.wait_for_healthy(std::time::Duration::from_secs(30)).await
        .expect("Server not healthy for test 2");
    ensure_model_loaded(&client2).await;

    let mut chain2 = PromptChain::new(client2, TEST_MODEL)
        .with_system_prompt("You are a helpful assistant. Always be concise.");

    assert_eq!(chain2.history().len(), 1);
    assert_eq!(chain2.history()[0].role, "system");

    let result = chain2.step("What is 5+5?").await.expect("Step failed");
    assert!(!result.content.is_empty());
    assert_eq!(chain2.history().len(), 3);
    eprintln!("[PASS 2/2] System prompt chain content={:?}", result.content);

    eprintln!("[ALL PASS] 2/2 prompt chain tests succeeded");
}
