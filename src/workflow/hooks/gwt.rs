//! GWT (Given-When-Then) expression evaluator for workflow hooks
//!
//! Provides a complete expression language for evaluating conditions against
//! runtime context (serde_json::Value). Supports:
//!
//! - Comparisons: `==`, `!=`, `>=`, `<=`, `>`, `<`
//! - Logical: `&&`, `||`, `!`
//! - Arithmetic: `+`, `-`, `*`, `/`
//! - Field paths: `error.is_retryable` (dot notation)
//! - Literals: numbers, strings, booleans, null
//!
//! # Example
//!
//! ```ignore
//! use whitt_execution_engine::workflow::hooks::gwt;
//! use serde_json::json;
//!
//! let context = json!({
//!     "error": {"is_retryable": true, "count": 2},
//!     "quality_score": 0.95
//! });
//!
//! let result = gwt::evaluate("error.is_retryable == true && error.count < 3", &context)?;
//! assert!(result);
//! ```

use serde_json::Value as JsonValue;
use std::fmt;

pub use self::token::Token;
pub use self::expr::Expr;

#[derive(Debug, Clone, PartialEq)]
pub enum GwtError {
    LexerError { pos: usize, message: String },
    ParserError { pos: usize, message: String },
    EvalError { message: String },
    TypeError { expected: String, found: String },
}

impl fmt::Display for GwtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GwtError::LexerError { pos, message } => {
                write!(f, "Lexer error at position {}: {}", pos, message)
            }
            GwtError::ParserError { pos, message } => {
                write!(f, "Parser error at position {}: {}", pos, message)
            }
            GwtError::EvalError { message } => {
                write!(f, "Evaluation error: {}", message)
            }
            GwtError::TypeError { expected, found } => {
                write!(f, "Type error: expected {}, found {}", expected, found)
            }
        }
    }
}

impl std::error::Error for GwtError {}

pub fn evaluate(expr: &str, context: &JsonValue) -> Result<bool, GwtError> {
    let tokens = token::Lexer::new(expr).tokenize()?;
    let ast = expr::Parser::new(&tokens).parse()?;
    let result = expr::Evaluator::new(context).evaluate(&ast)?;
    Ok(result)
}

mod token {
    use super::GwtError;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Token {
        Number(f64),
        String(String),
        Bool(bool),
        Null,
        Ident(String),
        Dot,
        Eq,
        Neq,
        Gte,
        Lte,
        Gt,
        Lt,
        And,
        Or,
        Not,
        Plus,
        Minus,
        Star,
        Slash,
        LParen,
        RParen,
    }

    pub struct Lexer<'a> {
        input: &'a str,
        pos: usize,
        chars: Vec<char>,
    }

    impl<'a> Lexer<'a> {
        pub fn new(input: &'a str) -> Self {
            let chars: Vec<char> = input.chars().collect();
            Lexer {
                input,
                pos: 0,
                chars,
            }
        }

        pub fn tokenize(&mut self) -> Result<Vec<Token>, GwtError> {
            let mut tokens = Vec::new();

            while self.pos < self.chars.len() {
                self.skip_whitespace();

                if self.pos >= self.chars.len() {
                    break;
                }

                let token = self.read_token()?;
                tokens.push(token);
            }

            Ok(tokens)
        }

        fn skip_whitespace(&mut self) {
            while self.pos < self.chars.len() && self.chars[self.pos].is_whitespace() {
                self.pos += 1;
            }
        }

        fn read_token(&mut self) -> Result<Token, GwtError> {
            let c = self.chars[self.pos];
            let start = self.pos;

            match c {
                '"' => self.read_string(),
                '0'..='9' => self.read_number(),
                'a'..='z' | 'A'..='Z' | '_' => self.read_ident(),
                '(' => {
                    self.pos += 1;
                    Ok(Token::LParen)
                }
                ')' => {
                    self.pos += 1;
                    Ok(Token::RParen)
                }
                '&' => {
                    if self.peek(1) == Some('&') {
                        self.pos += 2;
                        Ok(Token::And)
                    } else {
                        Err(GwtError::LexerError {
                            pos: start,
                            message: "Unexpected '&' without following '&'".to_string(),
                        })
                    }
                }
                '|' => {
                    if self.peek(1) == Some('|') {
                        self.pos += 2;
                        Ok(Token::Or)
                    } else {
                        Err(GwtError::LexerError {
                            pos: start,
                            message: "Unexpected '|' without following '|'".to_string(),
                        })
                    }
                }
                '!' => {
                    if self.peek(1) == Some('=') {
                        self.pos += 2;
                        Ok(Token::Neq)
                    } else {
                        self.pos += 1;
                        Ok(Token::Not)
                    }
                }
                '=' => {
                    if self.peek(1) == Some('=') {
                        self.pos += 2;
                        Ok(Token::Eq)
                    } else {
                        Err(GwtError::LexerError {
                            pos: start,
                            message: "Unexpected '=' without following '='".to_string(),
                        })
                    }
                }
                '>' => {
                    if self.peek(1) == Some('=') {
                        self.pos += 2;
                        Ok(Token::Gte)
                    } else {
                        self.pos += 1;
                        Ok(Token::Gt)
                    }
                }
                '<' => {
                    if self.peek(1) == Some('=') {
                        self.pos += 2;
                        Ok(Token::Lte)
                    } else {
                        self.pos += 1;
                        Ok(Token::Lt)
                    }
                }
                '+' => {
                    self.pos += 1;
                    Ok(Token::Plus)
                }
                '-' => {
                    self.pos += 1;
                    Ok(Token::Minus)
                }
                '*' => {
                    self.pos += 1;
                    Ok(Token::Star)
                }
                '/' => {
                    self.pos += 1;
                    Ok(Token::Slash)
                }
                '.' => {
                    self.pos += 1;
                    Ok(Token::Dot)
                }
                _ => Err(GwtError::LexerError {
                    pos: start,
                    message: format!("Unexpected character: '{}'", c),
                }),
            }
        }

        fn peek(&self, offset: usize) -> Option<char> {
            let idx = self.pos + offset;
            if idx < self.chars.len() {
                Some(self.chars[idx])
            } else {
                None
            }
        }

        fn read_string(&mut self) -> Result<Token, GwtError> {
            let start = self.pos;
            self.pos += 1;

            let mut result = String::new();
            while self.pos < self.chars.len() {
                let c = self.chars[self.pos];
                if c == '"' {
                    self.pos += 1;
                    return Ok(Token::String(result));
                } else if c == '\\' {
                    self.pos += 1;
                    if self.pos < self.chars.len() {
                        let escaped = self.chars[self.pos];
                        let decoded = match escaped {
                            '"' => '"',
                            '\\' => '\\',
                            'n' => '\n',
                            'r' => '\r',
                            't' => '\t',
                            _ => {
                                return Err(GwtError::LexerError {
                                    pos: self.pos,
                                    message: format!("Invalid escape sequence: \\{}", escaped),
                                })
                            }
                        };
                        result.push(decoded);
                        self.pos += 1;
                    } else {
                        return Err(GwtError::LexerError {
                            pos: self.pos - 1,
                            message: "Unterminated escape sequence".to_string(),
                        });
                    }
                } else {
                    result.push(c);
                    self.pos += 1;
                }
            }

            Err(GwtError::LexerError {
                pos: start,
                message: "Unterminated string literal".to_string(),
            })
        }

        fn read_number(&mut self) -> Result<Token, GwtError> {
            let start = self.pos;
            let mut has_dot = false;

            while self.pos < self.chars.len() {
                let c = self.chars[self.pos];
                if c.is_ascii_digit() {
                    self.pos += 1;
                } else if c == '.' && !has_dot {
                    has_dot = true;
                    self.pos += 1;
                } else {
                    break;
                }
            }

            let num_str = &self.input[start..self.pos];
            let value: f64 = num_str
                .parse()
                .map_err(|_| GwtError::LexerError {
                    pos: start,
                    message: format!("Invalid number literal: {}", num_str),
                })?;

            Ok(Token::Number(value))
        }

        fn read_ident(&mut self) -> Result<Token, GwtError> {
            let start = self.pos;

            while self.pos < self.chars.len() {
                let c = self.chars[self.pos];
                if c.is_ascii_alphanumeric() || c == '_' {
                    self.pos += 1;
                } else {
                    break;
                }
            }

            let ident = &self.input[start..self.pos];

            match ident {
                "true" => Ok(Token::Bool(true)),
                "false" => Ok(Token::Bool(false)),
                "null" => Ok(Token::Null),
                _ => Ok(Token::Ident(ident.to_string())),
            }
        }
    }
}

mod expr {
    use super::GwtError;
    use super::token::Token;
    use serde_json::Value as JsonValue;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Expr {
        Literal(JsonValue),
        IdentPath(Vec<String>),
        UnaryOp {
            op: UnaryOp,
            expr: Box<Expr>,
        },
        BinaryOp {
            op: BinaryOp,
            left: Box<Expr>,
            right: Box<Expr>,
        },
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum UnaryOp {
        Not,
        Neg,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum BinaryOp {
        Or,
        And,
        Eq,
        Neq,
        Gte,
        Lte,
        Gt,
        Lt,
        Add,
        Sub,
        Mul,
        Div,
    }

    pub struct Parser<'a> {
        tokens: &'a [Token],
        pos: usize,
    }

    impl<'a> Parser<'a> {
        pub fn new(tokens: &'a [Token]) -> Self {
            Parser { tokens, pos: 0 }
        }

        pub fn parse(&mut self) -> Result<Expr, GwtError> {
            let expr = self.parse_or_expr()?;
            if self.pos < self.tokens.len() {
                // Trailing unconsumed tokens = malformed expression (e.g.
                // "invalid condition syntax" parsed only the first word).
                // Historically ignored, which quietly evaluated as false.
                return Err(GwtError::ParserError {
                    pos: self.pos,
                    message: format!(
                        "Unexpected trailing tokens: {:?}",
                        &self.tokens[self.pos..]
                    ),
                });
            }
            Ok(expr)
        }

        fn parse_or_expr(&mut self) -> Result<Expr, GwtError> {
            let mut left = self.parse_and_expr()?;

            while self.match_token(Token::Or) {
                let right = self.parse_and_expr()?;
                left = Expr::BinaryOp {
                    op: BinaryOp::Or,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            }

            Ok(left)
        }

        fn parse_and_expr(&mut self) -> Result<Expr, GwtError> {
            let mut left = self.parse_not_expr()?;

            while self.match_token(Token::And) {
                let right = self.parse_not_expr()?;
                left = Expr::BinaryOp {
                    op: BinaryOp::And,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            }

            Ok(left)
        }

        fn parse_not_expr(&mut self) -> Result<Expr, GwtError> {
            if self.match_token(Token::Not) {
                let expr = self.parse_not_expr()?;
                return Ok(Expr::UnaryOp {
                    op: UnaryOp::Not,
                    expr: Box::new(expr),
                });
            }
            self.parse_comp_expr()
        }

        fn parse_comp_expr(&mut self) -> Result<Expr, GwtError> {
            let left = self.parse_add_expr()?;

            let op = if self.match_token(Token::Eq) {
                Some(BinaryOp::Eq)
            } else if self.match_token(Token::Neq) {
                Some(BinaryOp::Neq)
            } else if self.match_token(Token::Gte) {
                Some(BinaryOp::Gte)
            } else if self.match_token(Token::Lte) {
                Some(BinaryOp::Lte)
            } else if self.match_token(Token::Gt) {
                Some(BinaryOp::Gt)
            } else if self.match_token(Token::Lt) {
                Some(BinaryOp::Lt)
            } else {
                None
            };

            if let Some(op) = op {
                let right = self.parse_add_expr()?;
                return Ok(Expr::BinaryOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                });
            }

            Ok(left)
        }

        fn parse_add_expr(&mut self) -> Result<Expr, GwtError> {
            let mut left = self.parse_mul_expr()?;

            while let Some(token) = self.peek_token() {
                match token {
                    Token::Plus => {
                        self.consume();
                        let right = self.parse_mul_expr()?;
                        left = Expr::BinaryOp {
                            op: BinaryOp::Add,
                            left: Box::new(left),
                            right: Box::new(right),
                        };
                    }
                    Token::Minus => {
                        self.consume();
                        let right = self.parse_mul_expr()?;
                        left = Expr::BinaryOp {
                            op: BinaryOp::Sub,
                            left: Box::new(left),
                            right: Box::new(right),
                        };
                    }
                    _ => break,
                }
            }

            Ok(left)
        }

        fn parse_mul_expr(&mut self) -> Result<Expr, GwtError> {
            let mut left = self.parse_unary_expr()?;

            while let Some(token) = self.peek_token() {
                match token {
                    Token::Star => {
                        self.consume();
                        let right = self.parse_unary_expr()?;
                        left = Expr::BinaryOp {
                            op: BinaryOp::Mul,
                            left: Box::new(left),
                            right: Box::new(right),
                        };
                    }
                    Token::Slash => {
                        self.consume();
                        let right = self.parse_unary_expr()?;
                        left = Expr::BinaryOp {
                            op: BinaryOp::Div,
                            left: Box::new(left),
                            right: Box::new(right),
                        };
                    }
                    _ => break,
                }
            }

            Ok(left)
        }

        fn parse_unary_expr(&mut self) -> Result<Expr, GwtError> {
            if self.match_token(Token::Minus) {
                let expr = self.parse_unary_expr()?;
                return Ok(Expr::UnaryOp {
                    op: UnaryOp::Neg,
                    expr: Box::new(expr),
                });
            }
            self.parse_primary_expr()
        }

        fn parse_primary_expr(&mut self) -> Result<Expr, GwtError> {
            let token = self.consume_required("Expected expression")?;

            match token {
                Token::Number(n) => Ok(Expr::Literal(JsonValue::Number(serde_json::Number::from_f64(n).ok_or_else(|| GwtError::EvalError {
                    message: "Invalid floating point number".to_string(),
                })?))),
                Token::String(s) => Ok(Expr::Literal(JsonValue::String(s))),
                Token::Bool(b) => Ok(Expr::Literal(JsonValue::Bool(b))),
                Token::Null => Ok(Expr::Literal(JsonValue::Null)),
                Token::Ident(ident) => self.parse_ident_path(vec![ident.clone()]),
                Token::LParen => {
                    let expr = self.parse_or_expr()?;
                    self.expect_token(Token::RParen, "Expected closing parenthesis")?;
                    Ok(expr)
                }
                _ => Err(GwtError::ParserError {
                    pos: self.pos,
                    message: format!("Unexpected token in expression: {:?}", token),
                }),
            }
        }

        fn parse_ident_path(&mut self, mut path: Vec<String>) -> Result<Expr, GwtError> {
            while self.match_token(Token::Dot) {
                if let Some(segment) = self.peek_token().and_then(|t| {
                    if let Token::Ident(ref ident) = t {
                        Some(ident.clone())
                    } else {
                        None
                    }
                }) {
                    self.consume();
                    path.push(segment);
                } else {
                    return Err(GwtError::ParserError {
                        pos: self.pos,
                        message: "Expected identifier after dot".to_string(),
                    });
                }
            }
            Ok(Expr::IdentPath(path))
        }

        fn match_token(&mut self, token: Token) -> bool {
            if let Some(t) = self.peek_token() {
                if *t == token {
                    self.consume();
                    return true;
                }
            }
            false
        }

        fn expect_token(&mut self, token: Token, message: &str) -> Result<(), GwtError> {
            if let Some(t) = self.peek_token() {
                if *t == token {
                    self.consume();
                    return Ok(());
                }
            }
            Err(GwtError::ParserError {
                pos: self.pos,
                message: message.to_string(),
            })
        }

        fn peek_token(&self) -> Option<&Token> {
            if self.pos < self.tokens.len() {
                Some(&self.tokens[self.pos])
            } else {
                None
            }
        }

        fn consume(&mut self) -> Option<Token> {
            if self.pos < self.tokens.len() {
                let token = self.tokens[self.pos].clone();
                self.pos += 1;
                Some(token)
            } else {
                None
            }
        }

        fn consume_required(&mut self, message: &str) -> Result<Token, GwtError> {
            self.consume().ok_or_else(|| GwtError::ParserError {
                pos: self.pos,
                message: message.to_string(),
            })
        }
    }

    pub struct Evaluator<'a> {
        context: &'a JsonValue,
    }

    impl<'a> Evaluator<'a> {
        pub fn new(context: &'a JsonValue) -> Self {
            Evaluator { context }
        }

        pub fn evaluate(&self, expr: &Expr) -> Result<bool, GwtError> {
            let value = self.eval_expr(expr)?;
            self.as_bool(&value)
        }

        fn eval_expr(&self, expr: &Expr) -> Result<JsonValue, GwtError> {
            match expr {
                Expr::Literal(value) => Ok(value.clone()),
                Expr::IdentPath(path) => self.lookup_path(path),
                Expr::UnaryOp { op, expr } => self.eval_unary(op, expr),
                Expr::BinaryOp { op, left, right } => self.eval_binary(op, left, right),
            }
        }

        fn lookup_path(&self, path: &[String]) -> Result<JsonValue, GwtError> {
            let mut current = self.context;

            for segment in path {
                current = match current {
                    JsonValue::Object(map) => map.get(segment).unwrap_or(&JsonValue::Null),
                    JsonValue::Array(arr) => {
                        if let Ok(index) = segment.parse::<usize>() {
                            arr.get(index).unwrap_or(&JsonValue::Null)
                        } else {
                            &JsonValue::Null
                        }
                    }
                    _ => &JsonValue::Null,
                };
            }

            Ok(current.clone())
        }

        fn eval_unary(&self, op: &UnaryOp, expr: &Expr) -> Result<JsonValue, GwtError> {
            let value = self.eval_expr(expr)?;

            match op {
                UnaryOp::Not => {
                    let bool_val = self.as_bool(&value)?;
                    Ok(JsonValue::Bool(!bool_val))
                }
                UnaryOp::Neg => {
                    let num_val = self.as_number(&value)?;
                    let neg_val = -num_val;
                    Ok(JsonValue::Number(serde_json::Number::from_f64(neg_val).ok_or_else(|| GwtError::EvalError {
                        message: "Invalid floating point number after negation".to_string(),
                    })?))
                }
            }
        }

        fn eval_binary(&self, op: &BinaryOp, left: &Expr, right: &Expr) -> Result<JsonValue, GwtError> {
            let left_val = self.eval_expr(left)?;
            let right_val = self.eval_expr(right)?;

            match op {
                BinaryOp::Or | BinaryOp::And => {
                    let left_bool = self.as_bool(&left_val)?;
                    let right_bool = self.as_bool(&right_val)?;

                    match op {
                        BinaryOp::Or => Ok(JsonValue::Bool(left_bool || right_bool)),
                        BinaryOp::And => Ok(JsonValue::Bool(left_bool && right_bool)),
                        _ => unreachable!(),
                    }
                }
                BinaryOp::Eq | BinaryOp::Neq => {
                    let result = match (&left_val, &right_val) {
                        (JsonValue::Number(l), JsonValue::Number(r)) => {
                            l.as_f64().unwrap_or(0.0) == r.as_f64().unwrap_or(0.0)
                        }
                        _ => left_val == right_val,
                    };
                    let final_result = match op {
                        BinaryOp::Eq => result,
                        BinaryOp::Neq => !result,
                        _ => unreachable!(),
                    };
                    Ok(JsonValue::Bool(final_result))
                }
                BinaryOp::Gte | BinaryOp::Lte | BinaryOp::Gt | BinaryOp::Lt => {
                    self.eval_comparison(op, &left_val, &right_val)
                }
                BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div => {
                    self.eval_arithmetic(op, &left_val, &right_val)
                }
            }
        }

        fn eval_comparison(
            &self,
            op: &BinaryOp,
            left: &JsonValue,
            right: &JsonValue,
        ) -> Result<JsonValue, GwtError> {
            match (left, right) {
                (JsonValue::Number(l), JsonValue::Number(r)) => {
                    let left_num = l.as_f64().unwrap();
                    let right_num = r.as_f64().unwrap();

                    let result = match op {
                        BinaryOp::Gte => left_num >= right_num,
                        BinaryOp::Lte => left_num <= right_num,
                        BinaryOp::Gt => left_num > right_num,
                        BinaryOp::Lt => left_num < right_num,
                        _ => unreachable!(),
                    };
                    Ok(JsonValue::Bool(result))
                }
                (JsonValue::String(l), JsonValue::String(r)) => {
                    let result = match op {
                        BinaryOp::Gte => l >= r,
                        BinaryOp::Lte => l <= r,
                        BinaryOp::Gt => l > r,
                        BinaryOp::Lt => l < r,
                        _ => unreachable!(),
                    };
                    Ok(JsonValue::Bool(result))
                }
                (JsonValue::Bool(l), JsonValue::Bool(r)) => {
                    let result = match op {
                        BinaryOp::Gte => *l >= *r,
                        BinaryOp::Lte => *l <= *r,
                        BinaryOp::Gt => *l && !*r,
                        BinaryOp::Lt => !*l && *r,
                        _ => unreachable!(),
                    };
                    Ok(JsonValue::Bool(result))
                }
                (JsonValue::Null, JsonValue::Null) => Ok(JsonValue::Bool(match op {
                    BinaryOp::Gte | BinaryOp::Lte | BinaryOp::Eq => true,
                    BinaryOp::Gt | BinaryOp::Lt => false,
                    _ => unreachable!(),
                })),
                _ => {
                    let left_type = type_name(left);
                    let right_type = type_name(right);
                    Err(GwtError::TypeError {
                        expected: right_type,
                        found: left_type,
                    })
                }
            }
        }

        fn eval_arithmetic(
            &self,
            op: &BinaryOp,
            left: &JsonValue,
            right: &JsonValue,
        ) -> Result<JsonValue, GwtError> {
            let left_num = self.as_number(left)?;
            let right_num = self.as_number(right)?;

            let result = match op {
                BinaryOp::Add => left_num + right_num,
                BinaryOp::Sub => left_num - right_num,
                BinaryOp::Mul => left_num * right_num,
                BinaryOp::Div => {
                    if right_num == 0.0 {
                        return Err(GwtError::EvalError {
                            message: "Division by zero".to_string(),
                        });
                    }
                    left_num / right_num
                }
                _ => unreachable!(),
            };

            Ok(JsonValue::Number(serde_json::Number::from_f64(result).ok_or_else(|| GwtError::EvalError {
                message: "Invalid floating point number result".to_string(),
            })?))
        }

        fn as_bool(&self, value: &JsonValue) -> Result<bool, GwtError> {
            match value {
                JsonValue::Bool(b) => Ok(*b),
                JsonValue::Null => Ok(false),
                JsonValue::Number(n) => {
                    let num = n.as_f64().unwrap();
                    Ok(num != 0.0)
                }
                JsonValue::String(s) => Ok(!s.is_empty()),
                JsonValue::Array(arr) => Ok(!arr.is_empty()),
                JsonValue::Object(obj) => Ok(!obj.is_empty()),
            }
        }

        fn as_number(&self, value: &JsonValue) -> Result<f64, GwtError> {
            match value {
                JsonValue::Number(n) => Ok(n.as_f64().unwrap()),
                JsonValue::Bool(b) => Ok(if *b { 1.0 } else { 0.0 }),
                JsonValue::Null => Ok(0.0),
                _ => {
                    let value_type = type_name(value);
                    Err(GwtError::TypeError {
                        expected: "number".to_string(),
                        found: value_type,
                    })
                }
            }
        }
    }

    fn type_name(value: &JsonValue) -> String {
        match value {
            JsonValue::Null => "null".to_string(),
            JsonValue::Bool(_) => "boolean".to_string(),
            JsonValue::Number(_) => "number".to_string(),
            JsonValue::String(_) => "string".to_string(),
            JsonValue::Array(_) => "array".to_string(),
            JsonValue::Object(_) => "object".to_string(),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use serde_json::json;

        fn gwt_given_expression_when_evaluate_then_result(expr: &str, context: &JsonValue, expected: bool) {
            let result = super::super::evaluate(expr, context).expect("evaluation should succeed");
            assert_eq!(result, expected);
        }

        fn gwt_given_expression_when_evaluate_then_error(expr: &str, context: &JsonValue) {
            let result = super::super::evaluate(expr, context);
            assert!(result.is_err(), "Expected error but got success");
        }

        #[test]
        fn gwt_given_true_literal_when_evaluate_then_true() {
            let context = json!({});
            gwt_given_expression_when_evaluate_then_result("true", &context, true);
        }

        #[test]
        fn gwt_given_false_literal_when_evaluate_then_false() {
            let context = json!({});
            gwt_given_expression_when_evaluate_then_result("false", &context, false);
        }

        #[test]
        fn gwt_given_null_literal_when_evaluate_then_false() {
            let context = json!({});
            gwt_given_expression_when_evaluate_then_result("null", &context, false);
        }

        #[test]
        fn gwt_given_number_literal_when_evaluate_then_true() {
            let context = json!({});
            gwt_given_expression_when_evaluate_then_result("42", &context, true);
        }

        #[test]
        fn gwt_given_zero_literal_when_evaluate_then_false() {
            let context = json!({});
            gwt_given_expression_when_evaluate_then_result("0", &context, false);
        }

        #[test]
        fn gwt_given_string_literal_when_evaluate_then_true() {
            let context = json!({});
            gwt_given_expression_when_evaluate_then_result("\"test\"", &context, true);
        }

        #[test]
        fn gwt_given_empty_string_literal_when_evaluate_then_false() {
            let context = json!({});
            gwt_given_expression_when_evaluate_then_result("\"\"", &context, false);
        }

        #[test]
        fn gwt_given_simple_field_path_when_evaluate_then_value() {
            let context = json!({"name": "test"});
            gwt_given_expression_when_evaluate_then_result("name", &context, true);
        }

        #[test]
        fn gwt_given_nested_field_path_when_evaluate_then_value() {
            let context = json!({"error": {"is_retryable": true}});
            gwt_given_expression_when_evaluate_then_result("error.is_retryable", &context, true);
        }

        #[test]
        fn gwt_given_missing_field_when_evaluate_then_null() {
            let context = json!({});
            gwt_given_expression_when_evaluate_then_result("missing_field", &context, false);
        }

        #[test]
        fn gwt_given_deeply_nested_field_when_evaluate_then_value() {
            let context = json!({"a": {"b": {"c": {"d": true}}}});
            gwt_given_expression_when_evaluate_then_result("a.b.c.d", &context, true);
        }

        #[test]
        fn gwt_given_partial_path_exists_when_evaluate_then_null() {
            let context = json!({"a": {"b": 1}});
            gwt_given_expression_when_evaluate_then_result("a.b.c", &context, false);
        }

        #[test]
        fn gwt_given_equality_with_booleans_when_evaluate_then_result() {
            let context = json!({"flag": true});
            gwt_given_expression_when_evaluate_then_result("flag == true", &context, true);
        }

        #[test]
        fn gwt_given_inequality_with_booleans_when_evaluate_then_result() {
            let context = json!({"flag": false});
            gwt_given_expression_when_evaluate_then_result("flag != true", &context, true);
        }

        #[test]
        fn gwt_given_equality_with_numbers_when_evaluate_then_result() {
            let context = json!({"count": 42});
            gwt_given_expression_when_evaluate_then_result("count == 42", &context, true);
        }

        #[test]
        fn gwt_given_inequality_with_numbers_when_evaluate_then_result() {
            let context = json!({"count": 42});
            gwt_given_expression_when_evaluate_then_result("count != 0", &context, true);
        }

        #[test]
        fn gwt_given_greater_than_when_evaluate_then_result() {
            let context = json!({"score": 0.95});
            gwt_given_expression_when_evaluate_then_result("score > 0.9", &context, true);
        }

        #[test]
        fn gwt_given_less_than_when_evaluate_then_result() {
            let context = json!({"score": 0.95});
            gwt_given_expression_when_evaluate_then_result("score < 1.0", &context, true);
        }

        #[test]
        fn gwt_given_greater_than_or_equal_when_evaluate_then_result() {
            let context = json!({"score": 0.9});
            gwt_given_expression_when_evaluate_then_result("score >= 0.9", &context, true);
        }

        #[test]
        fn gwt_given_less_than_or_equal_when_evaluate_then_result() {
            let context = json!({"score": 0.9});
            gwt_given_expression_when_evaluate_then_result("score <= 0.9", &context, true);
        }

        #[test]
        fn gwt_given_equality_with_strings_when_evaluate_then_result() {
            let context = json!({"status": "success"});
            gwt_given_expression_when_evaluate_then_result("status == \"success\"", &context, true);
        }

        #[test]
        fn gwt_given_inequality_with_strings_when_evaluate_then_result() {
            let context = json!({"status": "success"});
            gwt_given_expression_when_evaluate_then_result("status != \"failed\"", &context, true);
        }

        #[test]
        fn gwt_given_string_comparison_when_evaluate_then_result() {
            let context = json!({"name": "beta"});
            gwt_given_expression_when_evaluate_then_result("name > \"alpha\"", &context, true);
        }

        #[test]
        fn gwt_given_and_expression_when_evaluate_then_true() {
            let context = json!({"a": true, "b": true});
            gwt_given_expression_when_evaluate_then_result("a && b", &context, true);
        }

        #[test]
        fn gwt_given_and_expression_when_evaluate_then_false() {
            let context = json!({"a": true, "b": false});
            gwt_given_expression_when_evaluate_then_result("a && b", &context, false);
        }

        #[test]
        fn gwt_given_or_expression_when_evaluate_then_true() {
            let context = json!({"a": false, "b": true});
            gwt_given_expression_when_evaluate_then_result("a || b", &context, true);
        }

        #[test]
        fn gwt_given_or_expression_when_evaluate_then_false() {
            let context = json!({"a": false, "b": false});
            gwt_given_expression_when_evaluate_then_result("a || b", &context, false);
        }

        #[test]
        fn gwt_given_not_expression_when_evaluate_then_true() {
            let context = json!({"flag": false});
            gwt_given_expression_when_evaluate_then_result("!flag", &context, true);
        }

        #[test]
        fn gwt_given_not_expression_when_evaluate_then_false() {
            let context = json!({"flag": true});
            gwt_given_expression_when_evaluate_then_result("!flag", &context, false);
        }

        #[test]
        fn gwt_given_complex_boolean_expression_when_evaluate_then_result() {
            let context = json!({
                "error": {"is_retryable": true, "count": 2},
                "quality_score": 0.95
            });
            gwt_given_expression_when_evaluate_then_result(
                "error.is_retryable == true && error.count < 3 && quality_score >= 0.9",
                &context,
                true,
            );
        }

        #[test]
        fn gwt_given_parenthesized_expression_when_evaluate_then_result() {
            let context = json!({"a": true, "b": false, "c": true});
            gwt_given_expression_when_evaluate_then_result("(a || b) && c", &context, true);
        }

        #[test]
        fn gwt_given_addition_when_evaluate_then_result() {
            let context = json!({});
            gwt_given_expression_when_evaluate_then_result("2 + 3", &context, true);
        }

        #[test]
        fn gwt_given_subtraction_when_evaluate_then_result() {
            let context = json!({});
            gwt_given_expression_when_evaluate_then_result("5 - 3", &context, true);
        }

        #[test]
        fn gwt_given_multiplication_when_evaluate_then_result() {
            let context = json!({});
            gwt_given_expression_when_evaluate_then_result("4 * 3", &context, true);
        }

        #[test]
        fn gwt_given_division_when_evaluate_then_result() {
            let context = json!({});
            gwt_given_expression_when_evaluate_then_result("6 / 3", &context, true);
        }

        #[test]
        fn gwt_given_division_by_zero_when_evaluate_then_error() {
            let context = json!({});
            gwt_given_expression_when_evaluate_then_error("1 / 0", &context);
        }

        #[test]
        fn gwt_given_negation_when_evaluate_then_result() {
            let context = json!({});
            gwt_given_expression_when_evaluate_then_result("-5", &context, true);
        }

        #[test]
        fn gwt_given_arithmetic_with_variables_when_evaluate_then_result() {
            let context = json!({"a": 2, "b": 3});
            gwt_given_expression_when_evaluate_then_result("a + b", &context, true);
        }

        #[test]
        fn gwt_given_complex_arithmetic_when_evaluate_then_result() {
            let context = json!({});
            gwt_given_expression_when_evaluate_then_result("(2 + 3) * 4", &context, true);
        }

        #[test]
        fn gwt_given_comparison_with_arithmetic_when_evaluate_then_result() {
            let context = json!({"base": 5});
            gwt_given_expression_when_evaluate_then_result("base + 3 > 7", &context, true);
        }

        #[test]
        fn gwt_given_type_mismatch_comparison_when_evaluate_then_error() {
            let context = json!({"a": 42, "b": "string"});
            gwt_given_expression_when_evaluate_then_error("a > b", &context);
        }

        #[test]
        fn gwt_given_invalid_syntax_unclosed_paren_when_evaluate_then_error() {
            let context = json!({});
            gwt_given_expression_when_evaluate_then_error("(true", &context);
        }

        #[test]
        fn gwt_given_invalid_syntax_unexpected_token_when_evaluate_then_error() {
            let context = json!({});
            gwt_given_expression_when_evaluate_then_error("&& true", &context);
        }

        #[test]
        fn gwt_given_invalid_escape_sequence_when_evaluate_then_error() {
            let context = json!({});
            gwt_given_expression_when_evaluate_then_error("\"test\\x\"", &context);
        }

        #[test]
        fn gwt_given_unterminated_string_when_evaluate_then_error() {
            let context = json!({});
            gwt_given_expression_when_evaluate_then_error("\"unterminated", &context);
        }

        #[test]
        fn gwt_given_null_comparison_when_evaluate_then_result() {
            let context = json!({"value": null});
            gwt_given_expression_when_evaluate_then_result("value == null", &context, true);
        }

        #[test]
        fn gwt_given_null_gte_comparison_when_evaluate_then_false() {
            let context = json!({"value": null});
            gwt_given_expression_when_evaluate_then_result("value >= null", &context, true);
        }

        #[test]
        fn gwt_given_boolean_comparison_when_evaluate_then_result() {
            let context = json!({"a": false, "b": true});
            gwt_given_expression_when_evaluate_then_result("a < b", &context, true);
        }

        #[test]
        fn gwt_given_escaped_string_when_evaluate_then_value() {
            let context = json!({});
            gwt_given_expression_when_evaluate_then_result("\"test\\nstring\"", &context, true);
        }

        #[test]
        fn gwt_given_complex_nested_expression_when_evaluate_then_result() {
            let context = json!({
                "result": {"config_loaded": true, "status": "success"},
                "quality_score": 0.95
            });
            gwt_given_expression_when_evaluate_then_result(
                "result.config_loaded == true && result.status == \"success\" && quality_score >= 0.9",
                &context,
                true,
            );
        }

        #[test]
        fn gwt_given_precedence_and_vs_or_when_evaluate_then_correct() {
            let context = json!({"a": true, "b": false, "c": true});
            gwt_given_expression_when_evaluate_then_result("a && b || c", &context, true);
        }

        #[test]
        fn gwt_given_precedence_comparison_vs_logical_when_evaluate_then_correct() {
            let context = json!({"a": 5, "b": 3, "c": true});
            gwt_given_expression_when_evaluate_then_result("a > b && c", &context, true);
        }

        #[test]
        fn gwt_given_precedence_arithmetic_vs_comparison_when_evaluate_then_correct() {
            let context = json!({});
            gwt_given_expression_when_evaluate_then_result("2 + 3 > 4", &context, true);
        }
    }
}
