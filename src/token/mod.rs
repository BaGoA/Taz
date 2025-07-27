pub mod constants;
pub mod functions;
pub mod operators;

use functions::Function;
use operators::{BinaryOperator, UnaryOperator};

/// Token used in library
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Empty,
    Number(f64),
    BinaryOperator(BinaryOperator),
    UnaryOperator(UnaryOperator),
    LeftParenthesis,
    RightParenthesis,
    Constant(f64),
    Function(Function),
}

impl Token {
    /// Create a number token
    pub fn new_number(value: f64) -> Token {
        Token::Number(value)
    }

    /// Create a binary operator token from char
    /// If char given in argument does not correspond to operator,
    /// an error message is stored in string contained in Result output
    pub fn new_binary_ops(ops: char) -> Result<Token, String> {
        BinaryOperator::from_char(ops).map(|binary_ops| Token::BinaryOperator(binary_ops))
    }

    /// Create a unary operator token from char
    /// If char given in argument does not correspond to operator,
    /// an error message is stored in string contained in Result output
    pub fn new_unary_ops(ops: char) -> Result<Token, String> {
        UnaryOperator::from_char(ops).map(|unary_ops| Token::UnaryOperator(unary_ops))
    }

    /// Create a constant token from string
    /// If string given in argument does not correspond to constants,
    /// an error message is stored in string contained in Result output
    pub fn new_constant(constant: &str) -> Result<Token, String> {
        constants::from_string(constant).map(|value| Token::Constant(value))
    }

    /// Create a function token from string
    /// If string given in argument does not correspond to constants,
    /// an error message is stored in string contained in Result output
    pub fn new_function(fun_name: &str) -> Result<Token, String> {
        Function::from_string(fun_name).map(|fun| Token::Function(fun))
    }
}

// Units tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_new_number() {
        let value_ref: f64 = 5.0;
        let token: Token = Token::new_number(value_ref);

        match token {
            Token::Number(value) => assert_eq!(value, value_ref),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_token_new_binary_ops() {
        let ops_ref: BinaryOperator = BinaryOperator::Minus;

        match Token::new_binary_ops('-') {
            Ok(token) => match token {
                Token::BinaryOperator(ops) => assert_eq!(ops, ops_ref),
                _ => assert!(false),
            },
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_token_new_unary_ops() {
        let ops_ref: UnaryOperator = UnaryOperator::Minus;

        match Token::new_unary_ops('-') {
            Ok(token) => match token {
                Token::UnaryOperator(ops) => assert_eq!(ops, ops_ref),
                _ => assert!(false),
            },
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_token_new_constant() {
        let constant_ref: f64 = constants::C;

        match Token::new_constant("c") {
            Ok(token) => match token {
                Token::Constant(constant) => assert_eq!(constant, constant_ref),
                _ => assert!(false),
            },
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_token_new_function() {
        let function_ref: Function = Function::Sin;

        match Token::new_function("sin") {
            Ok(token) => match token {
                Token::Function(function) => assert_eq!(function, function_ref),
                _ => assert!(false),
            },
            Err(_) => assert!(false),
        }
    }
}
