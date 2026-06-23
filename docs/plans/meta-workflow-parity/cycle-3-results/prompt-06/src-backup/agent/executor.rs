use std::time::Duration;
use tracing::debug;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackoffStrategy {
    Exponential { multiplier: f64 },
    Linear { increment_secs: u64 },
    Fixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub backoff: BackoffStrategy,
    pub initial_delay: Duration,
    pub max_delay: Duration,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            backoff: BackoffStrategy::Exponential { multiplier: 2.0 },
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(60),
        }
    }
}

pub struct StepExecutor {
    retry_config: RetryConfig,
}

impl StepExecutor {
    pub fn new(config: RetryConfig) -> Self {
        Self { retry_config: config }
    }

    pub async fn execute_step<F, T, Fut>(&self, step_fn: F) -> Result<T, anyhow::Error>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, anyhow::Error>>,
    {
        let mut last_error = None;

        for attempt in 0..=self.retry_config.max_retries {
            debug!("Executing step, attempt: {}/{}", attempt + 1, self.retry_config.max_retries + 1);

            match step_fn().await {
                Ok(result) => {
                    if attempt > 0 {
                        debug!("Step succeeded on attempt {}", attempt + 1);
                    }
                    return Ok(result);
                }
                Err(e) => {
                    last_error = Some(e);
                    
                    // Don't sleep after the last attempt
                    if attempt < self.retry_config.max_retries {
                        let delay = self.calculate_delay(attempt)?;
                        debug!("Step failed, retrying after {:?}", delay);
                        tokio::time::sleep(delay).await;
                    } else {
                        debug!("Step failed on final attempt, no more retries");
                    }
                }
            }
        }

        // All attempts failed
        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Step execution failed")))
    }

    fn calculate_delay(&self, attempt: u32) -> Result<Duration, anyhow::Error> {
        let base_delay = self.retry_config.initial_delay;
        let max_delay = self.retry_config.max_delay;

        let delay = match &self.retry_config.backoff {
            BackoffStrategy::Exponential { multiplier } => {
                let multiplier_f64 = multiplier.max(1.0);
                let delay_ms = base_delay.as_millis() as f64 * multiplier_f64.powi(attempt as i32);
                Duration::from_millis(delay_ms as u64)
            }
            BackoffStrategy::Linear { increment_secs } => {
                let increment = Duration::from_secs(*increment_secs);
                base_delay + increment * attempt
            }
            BackoffStrategy::Fixed => base_delay,
        };

        // Apply jitter (random value between 0-25% of delay)
        let jitter_ms = fastrand::u64(0..=delay.as_millis() as u64) / 4;
        let delay_with_jitter = delay + Duration::from_millis(jitter_ms);

        // Cap at max_delay
        Ok(delay_with_jitter.min(max_delay))
    }

    pub fn parse_backoff_strategy(strategy: &str, multiplier: Option<f64>) -> BackoffStrategy {
        let strategy_lower = strategy.to_lowercase();
        match strategy_lower.as_str() {
            "exponential" => BackoffStrategy::Exponential {
                multiplier: multiplier.unwrap_or(2.0),
            },
            "linear" => BackoffStrategy::Linear {
                increment_secs: 1,
            },
            "fixed" => BackoffStrategy::Fixed,
            _ => BackoffStrategy::Exponential {
                multiplier: 2.0,
            },
        }
    }

    pub fn parse_duration(s: &str) -> Result<Duration, anyhow::Error> {
        let s = s.trim().to_lowercase();
        let (value_str, unit) = s.split_at(
            s.find(|c: char| !c.is_numeric() && c != '.')
                .unwrap_or(s.len()),
        );

        let value: f64 = value_str.parse().map_err(|_| {
            anyhow::anyhow!("Invalid duration value: {}", value_str)
        })?;

        let duration = match unit {
            "ms" => Duration::from_millis(value as u64),
            "s" | "sec" | "second" | "seconds" => Duration::from_secs_f64(value),
            "m" | "min" | "minute" | "minutes" => Duration::from_secs_f64(value * 60.0),
            "h" | "hour" | "hours" => Duration::from_secs_f64(value * 3600.0),
            _ => {
                return Err(anyhow::anyhow!("Unknown duration unit: {}", unit));
            }
        };

        Ok(duration)
    }
}

impl Default for StepExecutor {
    fn default() -> Self {
        Self::new(RetryConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration() {
        assert_eq!(
            StepExecutor::parse_duration("1s").unwrap(),
            Duration::from_secs(1)
        );
        assert_eq!(
            StepExecutor::parse_duration("500ms").unwrap(),
            Duration::from_millis(500)
        );
        assert_eq!(
            StepExecutor::parse_duration("2m").unwrap(),
            Duration::from_secs(120)
        );
        assert_eq!(
            StepExecutor::parse_duration("1.5s").unwrap(),
            Duration::from_millis(1500)
        );
    }

    #[test]
    fn test_parse_backoff_strategy() {
        match StepExecutor::parse_backoff_strategy("exponential", Some(3.0)) {
            BackoffStrategy::Exponential { multiplier } => {
                assert_eq!(multiplier, 3.0);
            }
            _ => panic!("Expected exponential strategy"),
        }

        match StepExecutor::parse_backoff_strategy("linear", None) {
            BackoffStrategy::Linear { increment_secs } => {
                assert_eq!(increment_secs, 1);
            }
            _ => panic!("Expected linear strategy"),
        }

        assert!(matches!(
            StepExecutor::parse_backoff_strategy("fixed", None),
            BackoffStrategy::Fixed
        ));
    }
}
