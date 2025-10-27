use crate::expression::evaluator::Evaluator;
use crate::expression::token_iterator::TokenIterator;
use crate::token::operators::BinaryOperator;
use crate::token::Token;

/// Check if last token, which can represent an operator or left parenthesis, is primary
/// with binary operator given in argument
fn last_operator_is_primary(token_ops: Token, current_ops: BinaryOperator) -> bool {
    match token_ops {
        Token::UnaryOperator(_) => true,
        Token::BinaryOperator(last_ops) => {
            let last_precedence: u8 = last_ops.precedence();
            let current_precedence: u8 = current_ops.precedence();

            let is_primary: bool = last_precedence > current_precedence;
            let is_left_associativity: bool =
                (last_precedence == current_precedence) && current_ops.is_left_associative();

            return is_primary || is_left_associativity;
        }
        _ => false,
    }
}

/// Postfix is an iterator over tokens from an postfix expression
pub struct Postfix<T>
where
    T: TokenIterator,
{
    infix_iterator: T,
    stack_operator: Vec<Token>,
    primary_operator: Vec<Token>,
}

impl<T> Postfix<T>
where
    T: TokenIterator,
{
    /// Create Postfix iterator from token iterator
    pub fn new(infix_iterator: T) -> Self {
        return Self {
            infix_iterator,
            stack_operator: Vec::with_capacity(25),
            primary_operator: Vec::with_capacity(25),
        };
    }

    /// Evaluate the postfix expression
    /// If an error occurs during the evaluation, we return an error message in Err of the result.
    pub fn evaluate(self) -> Result<f64, String> {
        return Evaluator::new(self).evaluate();
    }
}

impl<T> TokenIterator for Postfix<T>
where
    T: TokenIterator,
{
    fn next_token(&mut self) -> Result<Token, String> {
        if !self.primary_operator.is_empty() {
            let token: Token = self.primary_operator[0];
            self.primary_operator.remove(0);

            return Ok(token);
        }

        let infix_token: Token = self.infix_iterator.next_token()?;

        match infix_token {
            Token::Number(_) => return Ok(infix_token),
            Token::Constant(_) => return Ok(infix_token),
            Token::BinaryOperator(ops) => {
                // Pop stack operator according to last operators precedence
                while let Some(&stack_last) = self.stack_operator.last() {
                    if last_operator_is_primary(stack_last, ops) {
                        self.primary_operator.push(stack_last);
                        self.stack_operator.pop();
                    } else {
                        break;
                    }
                }

                self.stack_operator.push(infix_token);
                return Ok(Token::Empty);
            }
            Token::UnaryOperator(_) => {
                self.stack_operator.push(infix_token);
                return Ok(Token::Empty);
            }
            Token::Function(_) => {
                self.stack_operator.push(infix_token);
                return Ok(Token::Empty);
            }
            Token::LeftParenthesis => {
                self.stack_operator.push(infix_token);
                return Ok(Token::Empty);
            }
            Token::RightParenthesis => {
                // Pop stack operator between left and right parenthesis
                while let Some(&stack_last) = self.stack_operator.last() {
                    if stack_last != Token::LeftParenthesis {
                        self.primary_operator.push(stack_last);
                        self.stack_operator.pop();
                    } else {
                        break;
                    }
                }

                if self.stack_operator.is_empty() {
                    return Err(String::from("Mismatched parenthesis"));
                }

                // Pop left parenthesis and function from stack operator
                self.stack_operator.pop();

                if let Some(&stack_last) = self.stack_operator.last() {
                    match stack_last {
                        Token::Function(_) => {
                            self.primary_operator.push(stack_last);
                            self.stack_operator.pop();
                        }
                        _ => (),
                    }
                }

                return Ok(Token::Empty);
            }
            _ => {
                // Push rest of operator. If stack operator contains left parenthesis, then there is an error
                if self.stack_operator.is_empty() {
                    return Ok(Token::Stop);
                } else {
                    if self.stack_operator.contains(&Token::LeftParenthesis) {
                        return Err(String::from("Mismatched parenthesis"));
                    }

                    return Ok(self.stack_operator.pop().unwrap());
                }
            }
        }
    }
}

// Units tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{constants, functions::Function, operators::UnaryOperator};

    // Mock infix iterator from vector of token
    struct MockInfix<'a> {
        tokens: core::slice::IterMut<'a, Token>,
    }

    impl<'a> MockInfix<'a> {
        fn new(tokens: &'a mut Vec<Token>) -> Self {
            return Self {
                tokens: (*tokens).iter_mut(),
            };
        }
    }

    impl<'a> TokenIterator for MockInfix<'a> {
        fn next_token(&mut self) -> Result<Token, String> {
            return match self.tokens.next() {
                Some(&mut token) => Ok(token),
                None => Ok(Token::Stop),
            };
        }
    }

    #[test]
    fn test_postfix_expression_with_numbers_plus_operator() {
        let mut infix_tokens: Vec<Token> = vec![
            Token::Number(2.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(3.0),
        ];

        let postfix = Postfix::new(MockInfix::new(&mut infix_tokens)).filter(|token: Token| {
            return token != Token::Empty;
        });

        let tokens: Vec<Token> = vec![
            Token::Number(2.0),
            Token::Number(3.0),
            Token::BinaryOperator(BinaryOperator::Plus),
        ];

        match postfix.equal(tokens.as_slice()) {
            Ok(are_equal) => assert!(are_equal),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_postfix_expression_with_numbers_plus_operator_minus_unary_operator() {
        let mut infix_tokens: Vec<Token> = vec![
            Token::UnaryOperator(UnaryOperator::Minus),
            Token::Number(2.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(3.0),
        ];

        let postfix = Postfix::new(MockInfix::new(&mut infix_tokens)).filter(|token: Token| {
            return token != Token::Empty;
        });

        let tokens: Vec<Token> = vec![
            Token::Number(2.0),
            Token::UnaryOperator(UnaryOperator::Minus),
            Token::Number(3.0),
            Token::BinaryOperator(BinaryOperator::Plus),
        ];

        match postfix.equal(tokens.as_slice()) {
            Ok(are_equal) => assert!(are_equal),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_postfix_expression_with_numbers_plus_operators() {
        let mut infix_tokens: Vec<Token> = vec![
            Token::Number(8.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(2.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(9.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(3.0),
        ];

        let postfix = Postfix::new(MockInfix::new(&mut infix_tokens)).filter(|token: Token| {
            return token != Token::Empty;
        });

        let tokens: Vec<Token> = vec![
            Token::Number(8.0),
            Token::Number(2.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(9.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(3.0),
            Token::BinaryOperator(BinaryOperator::Plus),
        ];

        match postfix.equal(tokens.as_slice()) {
            Ok(are_equal) => assert!(are_equal),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_postfix_expression_with_numbers_plus_multiply_operators() {
        let mut infix_tokens: Vec<Token> = vec![
            Token::Number(8.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(9.0),
            Token::BinaryOperator(BinaryOperator::Multiply),
            Token::Number(2.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(3.0),
        ];

        let postfix = Postfix::new(MockInfix::new(&mut infix_tokens)).filter(|token: Token| {
            return token != Token::Empty;
        });

        let tokens: Vec<Token> = vec![
            Token::Number(8.0),
            Token::Number(9.0),
            Token::Number(2.0),
            Token::BinaryOperator(BinaryOperator::Multiply),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(3.0),
            Token::BinaryOperator(BinaryOperator::Plus),
        ];

        match postfix.equal(tokens.as_slice()) {
            Ok(are_equal) => assert!(are_equal),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_postfix_expression_with_numbers_minus_divide_operators() {
        let mut infix_tokens: Vec<Token> = vec![
            Token::Number(8.0),
            Token::BinaryOperator(BinaryOperator::Divide),
            Token::Number(2.0),
            Token::BinaryOperator(BinaryOperator::Minus),
            Token::Number(9.0),
            Token::BinaryOperator(BinaryOperator::Divide),
            Token::Number(3.0),
        ];

        let postfix = Postfix::new(MockInfix::new(&mut infix_tokens)).filter(|token: Token| {
            return token != Token::Empty;
        });

        let tokens: Vec<Token> = vec![
            Token::Number(8.0),
            Token::Number(2.0),
            Token::BinaryOperator(BinaryOperator::Divide),
            Token::Number(9.0),
            Token::Number(3.0),
            Token::BinaryOperator(BinaryOperator::Divide),
            Token::BinaryOperator(BinaryOperator::Minus),
        ];

        match postfix.equal(tokens.as_slice()) {
            Ok(are_equal) => assert!(are_equal),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_postfix_expression_with_numbers_plus_multiply_operators_parenthesis() {
        let mut infix_tokens: Vec<Token> = vec![
            Token::LeftParenthesis,
            Token::Number(8.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(2.0),
            Token::RightParenthesis,
            Token::BinaryOperator(BinaryOperator::Multiply),
            Token::LeftParenthesis,
            Token::Number(9.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(3.0),
            Token::RightParenthesis,
        ];

        let postfix = Postfix::new(MockInfix::new(&mut infix_tokens)).filter(|token: Token| {
            return token != Token::Empty;
        });

        let tokens: Vec<Token> = vec![
            Token::Number(8.0),
            Token::Number(2.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(9.0),
            Token::Number(3.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::BinaryOperator(BinaryOperator::Multiply),
        ];

        match postfix.equal(tokens.as_slice()) {
            Ok(are_equal) => assert!(are_equal),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_postfix_expression_with_numbers_plus_multiply_divide_operators_minus_unary_operator_parenthesis(
    ) {
        let mut infix_tokens: Vec<Token> = vec![
            Token::LeftParenthesis,
            Token::Number(8.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(2.0),
            Token::RightParenthesis,
            Token::BinaryOperator(BinaryOperator::Multiply),
            Token::LeftParenthesis,
            Token::UnaryOperator(UnaryOperator::Minus),
            Token::Number(9.0),
            Token::BinaryOperator(BinaryOperator::Divide),
            Token::Number(3.0),
            Token::RightParenthesis,
        ];

        let postfix = Postfix::new(MockInfix::new(&mut infix_tokens)).filter(|token: Token| {
            return token != Token::Empty;
        });

        let tokens: Vec<Token> = vec![
            Token::Number(8.0),
            Token::Number(2.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(9.0),
            Token::UnaryOperator(UnaryOperator::Minus),
            Token::Number(3.0),
            Token::BinaryOperator(BinaryOperator::Divide),
            Token::BinaryOperator(BinaryOperator::Multiply),
        ];

        match postfix.equal(tokens.as_slice()) {
            Ok(are_equal) => assert!(are_equal),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_postfix_expression_with_numbers_plus_multiply_divide_minus_power_operators_parenthesis()
    {
        let mut infix_tokens: Vec<Token> = vec![
            Token::Number(3.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(4.0),
            Token::BinaryOperator(BinaryOperator::Multiply),
            Token::Number(2.0),
            Token::BinaryOperator(BinaryOperator::Divide),
            Token::LeftParenthesis,
            Token::Number(1.0),
            Token::BinaryOperator(BinaryOperator::Minus),
            Token::Number(5.0),
            Token::RightParenthesis,
            Token::BinaryOperator(BinaryOperator::Power),
            Token::Number(2.0),
            Token::BinaryOperator(BinaryOperator::Power),
            Token::Number(3.0),
        ];

        let postfix = Postfix::new(MockInfix::new(&mut infix_tokens)).filter(|token: Token| {
            return token != Token::Empty;
        });

        let tokens: Vec<Token> = vec![
            Token::Number(3.0),
            Token::Number(4.0),
            Token::Number(2.0),
            Token::BinaryOperator(BinaryOperator::Multiply),
            Token::Number(1.0),
            Token::Number(5.0),
            Token::BinaryOperator(BinaryOperator::Minus),
            Token::Number(2.0),
            Token::Number(3.0),
            Token::BinaryOperator(BinaryOperator::Power),
            Token::BinaryOperator(BinaryOperator::Power),
            Token::BinaryOperator(BinaryOperator::Divide),
            Token::BinaryOperator(BinaryOperator::Plus),
        ];

        match postfix.equal(tokens.as_slice()) {
            Ok(are_equal) => assert!(are_equal),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_postfix_expression_with_numbers_divide_multiply_operators_functions() {
        let mut infix_tokens: Vec<Token> = vec![
            Token::Function(Function::Sin),
            Token::LeftParenthesis,
            Token::Function(Function::Sqrt),
            Token::LeftParenthesis,
            Token::Number(9.0),
            Token::RightParenthesis,
            Token::BinaryOperator(BinaryOperator::Divide),
            Token::Number(3.0),
            Token::BinaryOperator(BinaryOperator::Multiply),
            Token::Number(3.0),
            Token::RightParenthesis,
        ];

        let postfix = Postfix::new(MockInfix::new(&mut infix_tokens)).filter(|token: Token| {
            return token != Token::Empty;
        });

        let tokens: Vec<Token> = vec![
            Token::Number(9.0),
            Token::Function(Function::Sqrt),
            Token::Number(3.0),
            Token::BinaryOperator(BinaryOperator::Divide),
            Token::Number(3.0),
            Token::BinaryOperator(BinaryOperator::Multiply),
            Token::Function(Function::Sin),
        ];

        match postfix.equal(tokens.as_slice()) {
            Ok(are_equal) => assert!(are_equal),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_postfix_expression_with_numbers_minus_unary_operator_function() {
        let mut infix_tokens: Vec<Token> = vec![
            Token::Function(Function::Acos),
            Token::LeftParenthesis,
            Token::UnaryOperator(UnaryOperator::Minus),
            Token::Number(1.0),
            Token::RightParenthesis,
        ];

        let postfix = Postfix::new(MockInfix::new(&mut infix_tokens)).filter(|token: Token| {
            return token != Token::Empty;
        });

        let tokens: Vec<Token> = vec![
            Token::Number(1.0),
            Token::UnaryOperator(UnaryOperator::Minus),
            Token::Function(Function::Acos),
        ];

        match postfix.equal(tokens.as_slice()) {
            Ok(are_equal) => assert!(are_equal),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_postfix_expression_with_constant_function() {
        let mut infix_tokens: Vec<Token> = vec![
            Token::Function(Function::Cos),
            Token::LeftParenthesis,
            Token::Constant(constants::PI),
            Token::RightParenthesis,
        ];

        let postfix = Postfix::new(MockInfix::new(&mut infix_tokens)).filter(|token: Token| {
            return token != Token::Empty;
        });

        let tokens: Vec<Token> = vec![
            Token::Constant(constants::PI),
            Token::Function(Function::Cos),
        ];

        match postfix.equal(tokens.as_slice()) {
            Ok(are_equal) => assert!(are_equal),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_postfix_expression_forgot_left_parenthesis() {
        let mut infix_tokens: Vec<Token> = vec![
            Token::LeftParenthesis,
            Token::Number(8.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(2.0),
            Token::RightParenthesis,
            Token::BinaryOperator(BinaryOperator::Multiply),
            Token::Number(9.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(3.0),
            Token::RightParenthesis,
        ];

        let mut postfix = Postfix::new(MockInfix::new(&mut infix_tokens)).filter(|token: Token| {
            return token != Token::Empty;
        });

        let mut result_token: Result<Token, String> = postfix.next_token();

        // You must have an error at the end
        loop {
            match result_token {
                Ok(token) => {
                    if token != Token::Stop {
                        result_token = postfix.next_token();
                    } else {
                        break;
                    }
                }
                Err(_) => break,
            }
        }

        assert!(result_token.is_err());
    }

    #[test]
    fn test_postfix_expression_forgot_right_parenthesis() {
        let mut infix_tokens: Vec<Token> = vec![
            Token::LeftParenthesis,
            Token::Number(8.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(2.0),
            Token::BinaryOperator(BinaryOperator::Multiply),
            Token::LeftParenthesis,
            Token::Number(9.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(3.0),
            Token::RightParenthesis,
        ];

        let mut postfix = Postfix::new(MockInfix::new(&mut infix_tokens)).filter(|token: Token| {
            return token != Token::Empty;
        });

        let mut result_token: Result<Token, String> = postfix.next_token();

        // You must have an error at the end
        loop {
            match result_token {
                Ok(token) => {
                    if token != Token::Stop {
                        result_token = postfix.next_token();
                    } else {
                        break;
                    }
                }
                Err(_) => break,
            }
        }

        assert!(result_token.is_err());
    }
}
