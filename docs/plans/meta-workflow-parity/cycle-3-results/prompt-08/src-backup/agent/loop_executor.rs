use crate::config::loop_config::{LoopType, ValidationCriterion};

pub struct LoopResult {
    pub iterations_completed: usize,
    pub final_output: String,
    pub outputs: Vec<String>,
    pub errors: Vec<String>,
}

pub struct LoopContext {
    pub iteration: usize,
    pub iteration_variable: Option<String>,
    pub iteration_value: Option<String>,
    pub previous_output: Option<String>,
}

pub struct LoopExecutor;

impl LoopExecutor {
    pub async fn execute_count<F, Fut>(
        max_iterations: usize,
        iteration_variable: Option<String>,
        mut step_fn: F,
    ) -> Result<LoopResult, anyhow::Error>
    where
        F: FnMut(LoopContext) -> Fut,
        Fut: std::future::Future<Output = Result<String, anyhow::Error>>,
    {
        let mut outputs = Vec::new();
        let mut errors = Vec::new();
        let mut previous_output = None;

        for i in 0..max_iterations {
            let ctx = LoopContext {
                iteration: i,
                iteration_variable: iteration_variable.clone(),
                iteration_value: Some(i.to_string()),
                previous_output: previous_output.clone(),
            };

            match step_fn(ctx).await {
                Ok(output) => {
                    previous_output = Some(output.clone());
                    outputs.push(output);
                }
                Err(e) => {
                    errors.push(e.to_string());
                }
            }
        }

        let final_output = outputs.last().cloned().unwrap_or_default();
        Ok(LoopResult {
            iterations_completed: outputs.len(),
            final_output,
            outputs,
            errors,
        })
    }

    pub async fn execute_validation<F, Fut>(
        max_iterations: usize,
        criteria: Vec<ValidationCriterion>,
        tolerance: f64,
        mut step_fn: F,
        evaluate_fn: impl Fn(&str, &[ValidationCriterion], f64) -> bool,
    ) -> Result<LoopResult, anyhow::Error>
    where
        F: FnMut(LoopContext) -> Fut,
        Fut: std::future::Future<Output = Result<String, anyhow::Error>>,
    {
        let mut outputs = Vec::new();
        let mut errors = Vec::new();
        let mut previous_output = None;

        for i in 0..max_iterations {
            let ctx = LoopContext {
                iteration: i,
                iteration_variable: None,
                iteration_value: None,
                previous_output: previous_output.clone(),
            };

            match step_fn(ctx).await {
                Ok(output) => {
                    if evaluate_fn(&output, &criteria, tolerance) {
                        outputs.push(output.clone());
                        return Ok(LoopResult {
                            iterations_completed: i + 1,
                            final_output: output.clone(),
                            outputs,
                            errors,
                        });
                    }
                    previous_output = Some(output.clone());
                    outputs.push(output);
                }
                Err(e) => {
                    errors.push(e.to_string());
                }
            }
        }

        let final_output = outputs.last().cloned().unwrap_or_default();
        Ok(LoopResult {
            iterations_completed: outputs.len(),
            final_output,
            outputs,
            errors,
        })
    }

    pub async fn execute_foreach<F, Fut>(
        items: Vec<serde_json::Value>,
        iteration_variable: String,
        mut step_fn: F,
    ) -> Result<LoopResult, anyhow::Error>
    where
        F: FnMut(LoopContext) -> Fut,
        Fut: std::future::Future<Output = Result<String, anyhow::Error>>,
    {
        let mut outputs = Vec::new();
        let mut errors = Vec::new();
        let mut previous_output = None;

        for (i, item) in items.iter().enumerate() {
            let value_str = match item {
                serde_json::Value::String(s) => s.clone(),
                other => other.to_string(),
            };

            let ctx = LoopContext {
                iteration: i,
                iteration_variable: Some(iteration_variable.clone()),
                iteration_value: Some(value_str.clone()),
                previous_output: previous_output.clone(),
            };

            match step_fn(ctx).await {
                Ok(output) => {
                    previous_output = Some(output.clone());
                    outputs.push(output);
                }
                Err(e) => {
                    errors.push(e.to_string());
                }
            }
        }

        let final_output = outputs.last().cloned().unwrap_or_default();
        Ok(LoopResult {
            iterations_completed: outputs.len(),
            final_output,
            outputs,
            errors,
        })
    }

    pub async fn execute<F, Fut>(
        loop_config: LoopType,
        step_fn: F,
        evaluate_fn: impl Fn(&str, &[ValidationCriterion], f64) -> bool,
    ) -> Result<LoopResult, anyhow::Error>
    where
        F: FnMut(LoopContext) -> Fut,
        Fut: std::future::Future<Output = Result<String, anyhow::Error>>,
    {
        match loop_config {
            LoopType::Count { count } => {
                Self::execute_count(count.max_iterations, count.iteration_variable, step_fn).await
            }
            LoopType::Validation { validation } => {
                Self::execute_validation(
                    validation.max_iterations,
                    validation.exact_criteria,
                    validation.tolerance,
                    step_fn,
                    evaluate_fn,
                )
                .await
            }
            LoopType::ForEach { foreach } => {
                Self::execute_foreach(foreach.items, foreach.iteration_variable, step_fn).await
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::loop_config::{CountLoop, ForEachLoop, ValidationLoop, ComparisonOperator, ValidationCriterion};

    #[tokio::test]
    async fn count_loop_verify_exactly_n_iterations() {
        let max_iterations = 5;
        let result = LoopExecutor::execute_count(
            max_iterations,
            Some("i".to_string()),
            |ctx| async move {
                Ok(format!("iteration {}", ctx.iteration))
            },
        )
        .await
        .expect("execute");

        assert_eq!(result.iterations_completed, 5);
        assert_eq!(result.outputs.len(), 5);
    }

    #[tokio::test]
    async fn count_loop_verify_iteration_numbers() {
        let max_iterations = 3;
        let result = LoopExecutor::execute_count(
            max_iterations,
            Some("i".to_string()),
            |ctx| async move {
                Ok(format!("iter {}", ctx.iteration))
            },
        )
        .await
        .expect("execute");

        assert_eq!(result.outputs[0], "iter 0");
        assert_eq!(result.outputs[1], "iter 1");
        assert_eq!(result.outputs[2], "iter 2");
    }

    #[tokio::test]
    async fn count_loop_verify_previous_output_none_first_some_subsequent() {
        let _result = LoopExecutor::execute_count(
            3,
            Some("i".to_string()),
            |ctx| async move {
                if ctx.iteration == 0 {
                    assert!(ctx.previous_output.is_none(), "first iteration should have None previous_output");
                } else {
                    assert!(ctx.previous_output.is_some(), "subsequent iterations should have Some previous_output");
                }
                Ok(format!("out {}", ctx.iteration))
            },
        )
        .await
        .expect("execute");
    }

    #[tokio::test]
    async fn validation_loop_verify_early_exit_when_criteria_met() {
        let criteria = vec![ValidationCriterion {
            metric: "score".to_string(),
            operator: ComparisonOperator::Gte,
            target: 0.9,
        }];

        let result = LoopExecutor::execute_validation(
            10,
            criteria.clone(),
            0.0,
            |ctx| async move {
                if ctx.iteration >= 2 {
                    Ok("score: 0.95".to_string())
                } else {
                    Ok("score: 0.5".to_string())
                }
            },
            |output, criteria, _| {
                let score: f64 = output
                    .split(": ")
                    .nth(1)
                    .unwrap()
                    .parse()
                    .unwrap();
                criteria[0].operator.evaluate(score, criteria[0].target)
            },
        )
        .await
        .expect("execute");

        assert_eq!(result.iterations_completed, 3);
        assert_eq!(result.outputs.len(), 3);
        assert_eq!(result.final_output, "score: 0.95");
    }

    #[tokio::test]
    async fn validation_loop_verify_runs_all_iterations_when_criteria_never_met() {
        let criteria = vec![ValidationCriterion {
            metric: "score".to_string(),
            operator: ComparisonOperator::Gte,
            target: 1.0,
        }];

        let result = LoopExecutor::execute_validation(
            5,
            criteria.clone(),
            0.0,
            |_ctx| async move {
                Ok("score: 0.5".to_string())
            },
            |output, criteria, _| {
                let score: f64 = output
                    .split(": ")
                    .nth(1)
                    .unwrap()
                    .parse()
                    .unwrap();
                criteria[0].operator.evaluate(score, criteria[0].target)
            },
        )
        .await
        .expect("execute");

        assert_eq!(result.iterations_completed, 5);
        assert_eq!(result.outputs.len(), 5);
    }

    #[tokio::test]
    async fn foreach_loop_verify_correct_items_passed() {
        let items = vec![
            serde_json::Value::String("item1".to_string()),
            serde_json::Value::String("item2".to_string()),
            serde_json::Value::String("item3".to_string()),
        ];

        let result = LoopExecutor::execute_foreach(
            items,
            "item".to_string(),
            |ctx| async move {
                Ok(format!("processing {}", ctx.iteration_value.unwrap()))
            },
        )
        .await
        .expect("execute");

        assert_eq!(result.iterations_completed, 3);
        assert_eq!(result.outputs[0], "processing item1");
        assert_eq!(result.outputs[1], "processing item2");
        assert_eq!(result.outputs[2], "processing item3");
    }

    #[tokio::test]
    async fn foreach_loop_verify_iteration_values_match_items() {
        let items = vec![
            serde_json::Value::String("alpha".to_string()),
            serde_json::Value::String("beta".to_string()),
            serde_json::Value::Number(42.into()),
        ];

        let result = LoopExecutor::execute_foreach(
            items,
            "value".to_string(),
            |ctx| async move {
                Ok(ctx.iteration_value.unwrap())
            },
        )
        .await
        .expect("execute");

        assert_eq!(result.outputs[0], "alpha");
        assert_eq!(result.outputs[1], "beta");
        assert_eq!(result.outputs[2], "42");
    }

    #[tokio::test]
    async fn execute_dispatches_to_count() {
        let loop_config = LoopType::Count {
            count: CountLoop {
                max_iterations: 3,
                iteration_variable: Some("i".to_string()),
            },
        };

        let result = LoopExecutor::execute(
            loop_config,
            |ctx| async move {
                Ok(format!("count {}", ctx.iteration))
            },
            |_output, _criteria, _tolerance| false,
        )
        .await
        .expect("execute");

        assert_eq!(result.iterations_completed, 3);
    }

    #[tokio::test]
    async fn execute_dispatches_to_validation() {
        let loop_config = LoopType::Validation {
            validation: ValidationLoop {
                tolerance: 0.0,
                max_iterations: 5,
                exact_criteria: vec![ValidationCriterion {
                    metric: "score".to_string(),
                    operator: ComparisonOperator::Gte,
                    target: 0.9,
                }],
            },
        };

        let result = LoopExecutor::execute(
            loop_config,
            |ctx| async move {
                if ctx.iteration == 1 {
                    Ok("score: 0.95".to_string())
                } else {
                    Ok("score: 0.5".to_string())
                }
            },
            |output, criteria, _| {
                let score: f64 = output
                    .split(": ")
                    .nth(1)
                    .unwrap()
                    .parse()
                    .unwrap();
                criteria[0].operator.evaluate(score, criteria[0].target)
            },
        )
        .await
        .expect("execute");

        assert_eq!(result.iterations_completed, 2);
        assert_eq!(result.final_output, "score: 0.95");
    }

    #[tokio::test]
    async fn execute_dispatches_to_foreach() {
        let loop_config = LoopType::ForEach {
            foreach: ForEachLoop {
                items: vec![
                    serde_json::Value::String("x".to_string()),
                    serde_json::Value::String("y".to_string()),
                ],
                iteration_variable: "v".to_string(),
            },
        };

        let result = LoopExecutor::execute(
            loop_config,
            |ctx| async move {
                Ok(ctx.iteration_value.unwrap())
            },
            |_output, _criteria, _tolerance| false,
        )
        .await
        .expect("execute");

        assert_eq!(result.iterations_completed, 2);
        assert_eq!(result.outputs[0], "x");
        assert_eq!(result.outputs[1], "y");
    }
}
