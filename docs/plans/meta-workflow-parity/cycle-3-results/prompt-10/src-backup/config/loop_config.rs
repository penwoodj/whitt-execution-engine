use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComparisonOperator {
    #[serde(rename = ">=")]
    Gte,
    #[serde(rename = "<=")]
    Lte,
    #[serde(rename = "==")]
    Eq,
    #[serde(rename = "!=")]
    Neq,
    #[serde(rename = ">")]
    Gt,
    #[serde(rename = "<")]
    Lt,
}

impl ComparisonOperator {
    pub fn evaluate(&self, actual: f64, target: f64) -> bool {
        match self {
            Self::Gte => actual >= target,
            Self::Lte => actual <= target,
            Self::Eq => (actual - target).abs() < 1e-6,
            Self::Neq => (actual - target).abs() >= 1e-6,
            Self::Gt => actual > target,
            Self::Lt => actual < target,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ValidationCriterion {
    pub metric: String,
    pub operator: ComparisonOperator,
    pub target: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CountLoop {
    pub max_iterations: usize,
    #[serde(default)]
    pub iteration_variable: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ValidationLoop {
    #[serde(default)]
    pub tolerance: f64,
    pub max_iterations: usize,
    #[serde(default)]
    pub exact_criteria: Vec<ValidationCriterion>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ForEachLoop {
    pub items: Vec<serde_json::Value>,
    pub iteration_variable: String,
}

// The schema uses nested structure: loop: { count: { ... }, validation: { ... } }
// Use untagged enum to match by field presence
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum LoopType {
    Count { count: CountLoop },
    Validation { validation: ValidationLoop },
    ForEach { foreach: ForEachLoop },
}

// ===========================================================================
// Unit tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comparison_operator_evaluate_gte() {
        assert!(ComparisonOperator::Gte.evaluate(5.0, 3.0));
        assert!(ComparisonOperator::Gte.evaluate(3.0, 3.0));
        assert!(!ComparisonOperator::Gte.evaluate(2.0, 3.0));
    }

    #[test]
    fn comparison_operator_evaluate_lte() {
        assert!(ComparisonOperator::Lte.evaluate(2.0, 3.0));
        assert!(ComparisonOperator::Lte.evaluate(3.0, 3.0));
        assert!(!ComparisonOperator::Lte.evaluate(5.0, 3.0));
    }

    #[test]
    fn comparison_operator_evaluate_eq() {
        assert!(ComparisonOperator::Eq.evaluate(3.0, 3.0));
        assert!(ComparisonOperator::Eq.evaluate(3.0000001, 3.0));
        assert!(!ComparisonOperator::Eq.evaluate(3.1, 3.0));
    }

    #[test]
    fn comparison_operator_evaluate_neq() {
        assert!(!ComparisonOperator::Neq.evaluate(3.0, 3.0));
        assert!(ComparisonOperator::Neq.evaluate(3.1, 3.0));
        assert!(ComparisonOperator::Neq.evaluate(2.0, 3.0));
    }

    #[test]
    fn comparison_operator_evaluate_gt() {
        assert!(ComparisonOperator::Gt.evaluate(5.0, 3.0));
        assert!(!ComparisonOperator::Gt.evaluate(3.0, 3.0));
        assert!(!ComparisonOperator::Gt.evaluate(2.0, 3.0));
    }

    #[test]
    fn comparison_operator_evaluate_lt() {
        assert!(ComparisonOperator::Lt.evaluate(2.0, 3.0));
        assert!(!ComparisonOperator::Lt.evaluate(3.0, 3.0));
        assert!(!ComparisonOperator::Lt.evaluate(5.0, 3.0));
    }

    #[test]
    fn loop_type_deserialize_count_from_yaml() {
        let yaml = r#"
count:
  max_iterations: 10
  iteration_variable: current_file
"#;
        let result: LoopType = serde_saphyr::from_str(yaml).expect("parse");
        match result {
            LoopType::Count { count } => {
                assert_eq!(count.max_iterations, 10);
                assert_eq!(count.iteration_variable, Some("current_file".to_string()));
            }
            _ => panic!("Expected Count variant"),
        }
    }

    #[test]
    fn loop_type_deserialize_count_from_json() {
        let json = r#"{
  "count": {
    "max_iterations": 10,
    "iteration_variable": "current_file"
  }
}"#;
        let result: LoopType = serde_json::from_str(json).expect("parse");
        match result {
            LoopType::Count { count } => {
                assert_eq!(count.max_iterations, 10);
                assert_eq!(count.iteration_variable, Some("current_file".to_string()));
            }
            _ => panic!("Expected Count variant"),
        }
    }

    #[test]
    fn loop_type_deserialize_validation_from_yaml() {
        let yaml = r#"
validation:
  tolerance: 0.05
  max_iterations: 5
  exact_criteria:
    - metric: quality_score
      operator: ">="
      target: 0.90
"#;
        let result: LoopType = serde_saphyr::from_str(yaml).expect("parse");
        match result {
            LoopType::Validation { validation } => {
                assert_eq!(validation.tolerance, 0.05);
                assert_eq!(validation.max_iterations, 5);
                assert_eq!(validation.exact_criteria.len(), 1);
                assert_eq!(validation.exact_criteria[0].metric, "quality_score");
                assert_eq!(validation.exact_criteria[0].operator, ComparisonOperator::Gte);
                assert_eq!(validation.exact_criteria[0].target, 0.90);
            }
            _ => panic!("Expected Validation variant"),
        }
    }

    #[test]
    fn loop_type_deserialize_validation_from_json() {
        let json = r#"{
  "validation": {
    "tolerance": 0.05,
    "max_iterations": 5,
    "exact_criteria": [
      {
        "metric": "quality_score",
        "operator": ">=",
        "target": 0.90
      }
    ]
  }
}"#;
        let result: LoopType = serde_json::from_str(json).expect("parse");
        match result {
            LoopType::Validation { validation } => {
                assert_eq!(validation.tolerance, 0.05);
                assert_eq!(validation.max_iterations, 5);
                assert_eq!(validation.exact_criteria.len(), 1);
                assert_eq!(validation.exact_criteria[0].metric, "quality_score");
                assert_eq!(validation.exact_criteria[0].operator, ComparisonOperator::Gte);
                assert_eq!(validation.exact_criteria[0].target, 0.90);
            }
            _ => panic!("Expected Validation variant"),
        }
    }

    #[test]
    fn loop_type_deserialize_foreach_from_yaml() {
        let yaml = r#"
foreach:
  items:
    - item1
    - item2
    - item3
  iteration_variable: current_item
"#;
        let result: LoopType = serde_saphyr::from_str(yaml).expect("parse");
        match result {
            LoopType::ForEach { foreach } => {
                assert_eq!(foreach.items.len(), 3);
                assert_eq!(foreach.iteration_variable, "current_item");
            }
            _ => panic!("Expected ForEach variant"),
        }
    }

    #[test]
    fn loop_type_deserialize_foreach_from_json() {
        let json = r#"{
  "foreach": {
    "items": ["item1", "item2", "item3"],
    "iteration_variable": "current_item"
  }
}"#;
        let result: LoopType = serde_json::from_str(json).expect("parse");
        match result {
            LoopType::ForEach { foreach } => {
                assert_eq!(foreach.items.len(), 3);
                assert_eq!(foreach.iteration_variable, "current_item");
            }
            _ => panic!("Expected ForEach variant"),
        }
    }
}
