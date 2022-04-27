/*
Taz is mathematical expression evaluation library
Copyright (C) 2022  Bastian Gonzalez Acevedo

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Lesser General Public License for more details.

You should have received a copy of the GNU Lesser General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use super::constants;
use super::functions::Function;
use super::operators::{BinaryOperator, UnaryOperator};

/// Token used in taz calculator
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
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
    #[allow(dead_code)]
    pub fn new_number(value: f64) -> Token {
        Token::Number(value)
    }

    /// Create a binary operator token from char
    /// If char given in argument does not correspond to operator,
    /// an error message is stored in string contained in Result output
    #[allow(dead_code)]
    pub fn new_binary_ops(ops: char) -> Result<Token, String> {
        match BinaryOperator::from_char(ops) {
            Ok(ops) => Ok(Token::BinaryOperator(ops)),
            Err(message) => Err(message),
        }
    }

    /// Create a unary operator token from char
    /// If char given in argument does not correspond to operator,
    /// an error message is stored in string contained in Result output
    #[allow(dead_code)]
    pub fn new_unary_ops(ops: char) -> Result<Token, String> {
        match UnaryOperator::from_char(ops) {
            Ok(ops) => Ok(Token::UnaryOperator(ops)),
            Err(message) => Err(message),
        }
    }

    /// Create a constant token from string
    /// If string given in argument does not correspond to constants,
    /// an error message is stored in string contained in Result output
    #[allow(dead_code)]
    pub fn new_constant(constant: &str) -> Result<Token, String> {
        match constants::from_string(constant) {
            Ok(value) => Ok(Token::Constant(value)),
            Err(message) => Err(message),
        }
    }

    /// Create a function token from string
    /// If string given in argument does not correspond to constants,
    /// an error message is stored in string contained in Result output
    #[allow(dead_code)]
    pub fn new_function(fun_name: &str) -> Result<Token, String> {
        match Function::from_string(fun_name) {
            Ok(fun) => Ok(Token::Function(fun)),
            Err(message) => Err(message),
        }
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
