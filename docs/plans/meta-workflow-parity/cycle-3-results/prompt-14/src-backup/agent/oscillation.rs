use anyhow::Result;
use crate::agent::chunker::Chunk;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OscillationPhase {
    Summarize,
    Expand,
}

impl fmt::Display for OscillationPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Summarize => write!(f, "summarize"),
            Self::Expand => write!(f, "expand"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OscillationConfig {
    pub oscillations: usize,
    pub summarize_prompt_template: String,
    pub expand_prompt_template: String,
}

impl Default for OscillationConfig {
    fn default() -> Self {
        Self {
            oscillations: 3,
            summarize_prompt_template: "Summarize the following text, extracting key technical points. Remove filler while preserving all specific numbers, file paths, function names, and constraints.\n\nText:\n{input}".to_string(),
            expand_prompt_template: "Expand on the following summary by adding concrete examples, missing context, edge cases, and error conditions. Maintain all existing technical substance.\n\nSummary:\n{input}".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OscillationResult {
    pub chunk_index: usize,
    pub final_output: String,
    pub iterations: usize,
    pub total_tokens_used: usize,
    pub phase_history: Vec<OscillationPhase>,
}

pub struct OscillationExecutor {
    pub config: OscillationConfig,
}

impl OscillationExecutor {
    pub fn new(config: OscillationConfig) -> Self {
        Self { config }
    }

    pub fn phase_for_iteration(&self, iteration: usize) -> OscillationPhase {
        if iteration.is_multiple_of(2) {
            OscillationPhase::Summarize
        } else {
            OscillationPhase::Expand
        }
    }

    pub fn build_prompt(&self, iteration: usize, input: &str) -> String {
        let phase = self.phase_for_iteration(iteration);
        let template = match phase {
            OscillationPhase::Summarize => &self.config.summarize_prompt_template,
            OscillationPhase::Expand => &self.config.expand_prompt_template,
        };
        template.replace("{input}", input)
    }

    pub fn total_iterations(&self) -> usize {
        self.config.oscillations * 2
    }

    pub async fn execute_chunk<F, Fut>(
        &self,
        chunk: &Chunk,
        mut llm_call: F,
    ) -> Result<OscillationResult>
    where
        F: FnMut(String) -> Fut,
        Fut: std::future::Future<Output = Result<String>>,
    {
        let total = self.total_iterations();
        let mut current_text = chunk.content.clone();
        let mut phase_history = Vec::new();

        for iteration in 0..total {
            let prompt = self.build_prompt(iteration, &current_text);
            let phase = self.phase_for_iteration(iteration);
            phase_history.push(phase);

            match llm_call(prompt).await {
                Ok(response) => {
                    current_text = response;
                }
                Err(e) => {
                    tracing::warn!(
                        "Oscillation iteration {} failed for chunk {}: {}",
                        iteration,
                        chunk.index,
                        e
                    );
                }
            }
        }

        Ok(OscillationResult {
            chunk_index: chunk.index,
            final_output: current_text,
            iterations: total,
            total_tokens_used: 0,
            phase_history,
        })
    }

    pub async fn execute_chunks<F, Fut>(
        &self,
        chunks: &[Chunk],
        llm_call: F,
    ) -> Result<Vec<OscillationResult>>
    where
        F: Fn(String) -> Fut,
        Fut: std::future::Future<Output = Result<String>>,
    {
        let mut results = Vec::new();
        for chunk in chunks {
            let result = self.execute_chunk(chunk, |prompt| {
                llm_call(prompt)
            }).await?;
            results.push(result);
        }
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_for_iteration_even_is_summarize() {
        let config = OscillationConfig::default();
        let executor = OscillationExecutor::new(config);

        assert_eq!(executor.phase_for_iteration(0), OscillationPhase::Summarize);
        assert_eq!(executor.phase_for_iteration(2), OscillationPhase::Summarize);
        assert_eq!(executor.phase_for_iteration(4), OscillationPhase::Summarize);
    }

    #[test]
    fn test_phase_for_iteration_odd_is_expand() {
        let config = OscillationConfig::default();
        let executor = OscillationExecutor::new(config);

        assert_eq!(executor.phase_for_iteration(1), OscillationPhase::Expand);
        assert_eq!(executor.phase_for_iteration(3), OscillationPhase::Expand);
        assert_eq!(executor.phase_for_iteration(5), OscillationPhase::Expand);
    }

    #[test]
    fn test_total_iterations_is_2x_oscillations() {
        let config = OscillationConfig { oscillations: 3, ..Default::default() };
        let executor = OscillationExecutor::new(config);

        assert_eq!(executor.total_iterations(), 6);
    }

    #[test]
    fn test_build_prompt_uses_correct_template() {
        let config = OscillationConfig {
            oscillations: 3,
            summarize_prompt_template: "Sum: {input}".to_string(),
            expand_prompt_template: "Exp: {input}".to_string(),
        };
        let executor = OscillationExecutor::new(config);

        let summarize_prompt = executor.build_prompt(0, "test");
        assert!(summarize_prompt.contains("Sum:"));
        assert!(summarize_prompt.contains("test"));

        let expand_prompt = executor.build_prompt(1, "test");
        assert!(expand_prompt.contains("Exp:"));
        assert!(expand_prompt.contains("test"));
    }

    #[test]
    fn test_build_prompt_replaces_input_placeholder() {
        let config = OscillationConfig {
            oscillations: 3,
            summarize_prompt_template: "Process: {input}".to_string(),
            expand_prompt_template: "Expand: {input}".to_string(),
        };
        let executor = OscillationExecutor::new(config);

        let prompt = executor.build_prompt(0, "MY_INPUT");
        assert!(prompt.contains("MY_INPUT"));
        assert!(!prompt.contains("{input}"));
    }

    #[tokio::test]
    async fn test_execute_chunk_calls_llm_correct_number_of_times() {
        let config = OscillationConfig {
            oscillations: 2,
            ..Default::default()
        };
        let executor = OscillationExecutor::new(config);

        let chunk = Chunk {
            index: 0,
            content: "Test content".to_string(),
            start_offset: 0,
            end_offset: 12,
            target_length: 100,
        };

        let mock_llm = |_: String| async {
            Ok("Response".to_string())
        };

        let result = executor.execute_chunk(&chunk, mock_llm).await.unwrap();

        assert_eq!(result.iterations, 4);
        assert_eq!(result.phase_history.len(), 4);
    }

    #[tokio::test]
    async fn test_execute_chunk_phases_alternate_correctly() {
        let config = OscillationConfig::default();
        let executor = OscillationExecutor::new(config);

        let chunk = Chunk {
            index: 0,
            content: "Test".to_string(),
            start_offset: 0,
            end_offset: 4,
            target_length: 100,
        };

        let mock_llm = |_: String| async {
            Ok("Response".to_string())
        };

        let result = executor.execute_chunk(&chunk, mock_llm).await.unwrap();

        assert_eq!(result.phase_history[0], OscillationPhase::Summarize);
        assert_eq!(result.phase_history[1], OscillationPhase::Expand);
        assert_eq!(result.phase_history[2], OscillationPhase::Summarize);
        assert_eq!(result.phase_history[3], OscillationPhase::Expand);
        assert_eq!(result.phase_history[4], OscillationPhase::Summarize);
        assert_eq!(result.phase_history[5], OscillationPhase::Expand);
    }

    #[test]
    fn test_default_config_has_3_oscillations() {
        let config = OscillationConfig::default();
        assert_eq!(config.oscillations, 3);
    }
}
