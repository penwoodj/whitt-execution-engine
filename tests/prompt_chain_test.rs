use whitt_execution_engine::client::http_client::LlamaHttpClient;
use whitt_execution_engine::client::prompt_chain::PromptChain;

#[tokio::test]
#[ignore]
async fn test_multi_step_chain() {
    let client = LlamaHttpClient::new("http://localhost:8080")
        .expect("Failed to create client");

    let mut chain = PromptChain::new(client, "Qwen2.5-0.5B-Instruct-Q4_K_M.gguf");

    let result1 = chain
        .step("What is 2+2? Answer with just the number.")
        .await
        .expect("Step 1 failed");

    assert!(!result1.content.is_empty(), "Step 1 content is empty");
    assert_eq!(result1.step_number, 1, "Step number should be 1");
    assert_eq!(chain.step_count(), 1, "Chain step count should be 1");
    assert_eq!(chain.history().len(), 2, "History should have 2 messages");
    assert!(chain.last_response().is_some(), "Last response should exist");

    let result2 = chain
        .step("Take that number and multiply it by 3. What do you get?")
        .await
        .expect("Step 2 failed");

    assert!(!result2.content.is_empty(), "Step 2 content is empty");
    assert_eq!(result2.step_number, 2, "Step number should be 2");
    assert_eq!(chain.step_count(), 2, "Chain step count should be 2");
    assert_eq!(chain.history().len(), 4, "History should have 4 messages");
    assert!(chain.last_response().is_some(), "Last response should exist");

    let result3 = chain
        .step("What is the square root of your previous answer?")
        .await
        .expect("Step 3 failed");

    assert!(!result3.content.is_empty(), "Step 3 content is empty");
    assert_eq!(result3.step_number, 3, "Step number should be 3");
    assert_eq!(chain.step_count(), 3, "Chain step count should be 3");
    assert_eq!(chain.history().len(), 6, "History should have 6 messages");
    assert!(chain.last_response().is_some(), "Last response should exist");
}

#[tokio::test]
#[ignore]
async fn test_chain_with_system_prompt() {
    let client = LlamaHttpClient::new("http://localhost:8080")
        .expect("Failed to create client");

    let mut chain = PromptChain::new(client, "Qwen2.5-0.5B-Instruct-Q4_K_M.gguf")
        .with_system_prompt("You are a helpful assistant. Always be concise.");

    assert_eq!(chain.history().len(), 1, "History should have 1 system message");

    let result1 = chain.step("What is 5+5?").await.expect("Step 1 failed");

    assert!(!result1.content.is_empty(), "Step 1 content is empty");
    assert_eq!(chain.history().len(), 3, "History should have 3 messages");
    assert_eq!(chain.history()[0].role, "system", "First message should be system");
}
