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

/// Postfix is an iterator over tokens of an postfix expression
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
    pub fn new(infix_iterator: T) -> Self {
        return Self {
            infix_iterator,
            stack_operator: Vec::with_capacity(25),
            primary_operator: Vec::with_capacity(25),
        };
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

    fn infix_to_postfix(tokens: &mut Vec<Token>) -> Result<Vec<Token>, String> {
        let postfix_iterator = Postfix::new(MockInfix::new(tokens));
        return postfix_iterator.collect_all_tokens();
    }

    #[test]
    fn test_infix_to_postfix_expression_with_numbers_plus_operator() {
        let mut tokens: Vec<Token> = vec![
            Token::Number(2.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(3.0),
        ];

        match infix_to_postfix(&mut tokens) {
            Ok(tokens_postfix) => {
                assert_eq!(tokens_postfix.len(), 3);

                match tokens_postfix[0] {
                    Token::Number(number) => assert_eq!(number, 2.0),
                    _ => assert!(false),
                }

                match tokens_postfix[1] {
                    Token::Number(number) => assert_eq!(number, 3.0),
                    _ => assert!(false),
                }

                match tokens_postfix[2] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Plus),
                    _ => assert!(false),
                }
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_infix_to_postfix_expression_with_numbers_plus_operator_minus_unary_operator() {
        let mut tokens: Vec<Token> = vec![
            Token::UnaryOperator(UnaryOperator::Minus),
            Token::Number(2.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(3.0),
        ];

        match infix_to_postfix(&mut tokens) {
            Ok(tokens_postfix) => {
                assert_eq!(tokens_postfix.len(), 4);

                match tokens_postfix[0] {
                    Token::Number(number) => assert_eq!(number, 2.0),
                    _ => assert!(false),
                }

                match tokens_postfix[1] {
                    Token::UnaryOperator(ops) => assert_eq!(ops, UnaryOperator::Minus),
                    _ => assert!(false),
                }

                match tokens_postfix[2] {
                    Token::Number(number) => assert_eq!(number, 3.0),
                    _ => assert!(false),
                }

                match tokens_postfix[3] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Plus),
                    _ => assert!(false),
                }
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_infix_to_postfix_expression_with_numbers_plus_operators() {
        let mut tokens: Vec<Token> = vec![
            Token::Number(8.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(2.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(9.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(3.0),
        ];

        match infix_to_postfix(&mut tokens) {
            Ok(tokens_postfix) => {
                assert_eq!(tokens_postfix.len(), 7);

                match tokens_postfix[0] {
                    Token::Number(number) => assert_eq!(number, 8.0),
                    _ => assert!(false),
                }

                match tokens_postfix[1] {
                    Token::Number(number) => assert_eq!(number, 2.0),
                    _ => assert!(false),
                }

                match tokens_postfix[2] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Plus),
                    _ => assert!(false),
                }

                match tokens_postfix[3] {
                    Token::Number(number) => assert_eq!(number, 9.0),
                    _ => assert!(false),
                }

                match tokens_postfix[4] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Plus),
                    _ => assert!(false),
                }

                match tokens_postfix[5] {
                    Token::Number(number) => assert_eq!(number, 3.0),
                    _ => assert!(false),
                }

                match tokens_postfix[6] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Plus),
                    _ => assert!(false),
                }
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_infix_to_postfix_expression_with_numbers_plus_multiply_operators() {
        let mut tokens: Vec<Token> = vec![
            Token::Number(8.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(9.0),
            Token::BinaryOperator(BinaryOperator::Multiply),
            Token::Number(2.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(3.0),
        ];

        match infix_to_postfix(&mut tokens) {
            Ok(tokens_postfix) => {
                assert_eq!(tokens_postfix.len(), 7);

                match tokens_postfix[0] {
                    Token::Number(number) => assert_eq!(number, 8.0),
                    _ => assert!(false),
                }

                match tokens_postfix[1] {
                    Token::Number(number) => assert_eq!(number, 9.0),
                    _ => assert!(false),
                }

                match tokens_postfix[2] {
                    Token::Number(number) => assert_eq!(number, 2.0),
                    _ => assert!(false),
                }

                match tokens_postfix[3] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Multiply),
                    _ => assert!(false),
                }

                match tokens_postfix[4] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Plus),
                    _ => assert!(false),
                }

                match tokens_postfix[5] {
                    Token::Number(number) => assert_eq!(number, 3.0),
                    _ => assert!(false),
                }

                match tokens_postfix[6] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Plus),
                    _ => assert!(false),
                }
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_infix_to_postfix_expression_with_numbers_minus_divide_operators() {
        let mut tokens: Vec<Token> = vec![
            Token::Number(8.0),
            Token::BinaryOperator(BinaryOperator::Divide),
            Token::Number(2.0),
            Token::BinaryOperator(BinaryOperator::Minus),
            Token::Number(9.0),
            Token::BinaryOperator(BinaryOperator::Divide),
            Token::Number(3.0),
        ];

        match infix_to_postfix(&mut tokens) {
            Ok(tokens_postfix) => {
                assert_eq!(tokens_postfix.len(), 7);

                match tokens_postfix[0] {
                    Token::Number(number) => assert_eq!(number, 8.0),
                    _ => assert!(false),
                }

                match tokens_postfix[1] {
                    Token::Number(number) => assert_eq!(number, 2.0),
                    _ => assert!(false),
                }

                match tokens_postfix[2] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Divide),
                    _ => assert!(false),
                }

                match tokens_postfix[3] {
                    Token::Number(number) => assert_eq!(number, 9.0),
                    _ => assert!(false),
                }

                match tokens_postfix[4] {
                    Token::Number(number) => assert_eq!(number, 3.0),
                    _ => assert!(false),
                }

                match tokens_postfix[5] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Divide),
                    _ => assert!(false),
                }

                match tokens_postfix[6] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Minus),
                    _ => assert!(false),
                }
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_infix_to_postfix_expression_with_numbers_plus_multiply_operators_parenthesis() {
        let mut tokens: Vec<Token> = vec![
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

        match infix_to_postfix(&mut tokens) {
            Ok(tokens_postfix) => {
                assert_eq!(tokens_postfix.len(), 7);

                match tokens_postfix[0] {
                    Token::Number(number) => assert_eq!(number, 8.0),
                    _ => assert!(false),
                }

                match tokens_postfix[1] {
                    Token::Number(number) => assert_eq!(number, 2.0),
                    _ => assert!(false),
                }

                match tokens_postfix[2] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Plus),
                    _ => assert!(false),
                }

                match tokens_postfix[3] {
                    Token::Number(number) => assert_eq!(number, 9.0),
                    _ => assert!(false),
                }

                match tokens_postfix[4] {
                    Token::Number(number) => assert_eq!(number, 3.0),
                    _ => assert!(false),
                }

                match tokens_postfix[5] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Plus),
                    _ => assert!(false),
                }

                match tokens_postfix[6] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Multiply),
                    _ => assert!(false),
                }
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_infix_to_postfix_expression_with_numbers_plus_multiply_divide_operators_minus_unary_operator_parenthesis(
    ) {
        let mut tokens: Vec<Token> = vec![
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

        match infix_to_postfix(&mut tokens) {
            Ok(tokens_postfix) => {
                assert_eq!(tokens_postfix.len(), 8);

                match tokens_postfix[0] {
                    Token::Number(number) => assert_eq!(number, 8.0),
                    _ => assert!(false),
                }

                match tokens_postfix[1] {
                    Token::Number(number) => assert_eq!(number, 2.0),
                    _ => assert!(false),
                }

                match tokens_postfix[2] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Plus),
                    _ => assert!(false),
                }

                match tokens_postfix[3] {
                    Token::Number(number) => assert_eq!(number, 9.0),
                    _ => assert!(false),
                }

                match tokens_postfix[4] {
                    Token::UnaryOperator(ops) => assert_eq!(ops, UnaryOperator::Minus),
                    _ => assert!(false),
                }

                match tokens_postfix[5] {
                    Token::Number(number) => assert_eq!(number, 3.0),
                    _ => assert!(false),
                }

                match tokens_postfix[6] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Divide),
                    _ => assert!(false),
                }

                match tokens_postfix[7] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Multiply),
                    _ => assert!(false),
                }
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_infix_to_postfix_expression_with_numbers_plus_multiply_divide_minus_power_operators_parenthesis(
    ) {
        let mut tokens: Vec<Token> = vec![
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

        match infix_to_postfix(&mut tokens) {
            Ok(tokens_postfix) => {
                assert_eq!(tokens_postfix.len(), 13);

                match tokens_postfix[0] {
                    Token::Number(number) => assert_eq!(number, 3.0),
                    _ => assert!(false),
                }

                match tokens_postfix[1] {
                    Token::Number(number) => assert_eq!(number, 4.0),
                    _ => assert!(false),
                }

                match tokens_postfix[2] {
                    Token::Number(number) => assert_eq!(number, 2.0),
                    _ => assert!(false),
                }

                match tokens_postfix[3] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Multiply),
                    _ => assert!(false),
                }

                match tokens_postfix[4] {
                    Token::Number(number) => assert_eq!(number, 1.0),
                    _ => assert!(false),
                }

                match tokens_postfix[5] {
                    Token::Number(number) => assert_eq!(number, 5.0),
                    _ => assert!(false),
                }

                match tokens_postfix[6] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Minus),
                    _ => assert!(false),
                }

                match tokens_postfix[7] {
                    Token::Number(number) => assert_eq!(number, 2.0),
                    _ => assert!(false),
                }

                match tokens_postfix[8] {
                    Token::Number(number) => assert_eq!(number, 3.0),
                    _ => assert!(false),
                }

                match tokens_postfix[9] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Power),
                    _ => assert!(false),
                }

                match tokens_postfix[10] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Power),
                    _ => assert!(false),
                }

                match tokens_postfix[11] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Divide),
                    _ => assert!(false),
                }

                match tokens_postfix[12] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Plus),
                    _ => assert!(false),
                }
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_infix_to_postfix_expression_with_numbers_divide_multiply_operators_functions() {
        let mut tokens: Vec<Token> = vec![
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

        match infix_to_postfix(&mut tokens) {
            Ok(tokens_postfix) => {
                assert_eq!(tokens_postfix.len(), 7);

                match tokens_postfix[0] {
                    Token::Number(number) => assert_eq!(number, 9.0),
                    _ => assert!(false),
                }

                match tokens_postfix[1] {
                    Token::Function(fun) => assert_eq!(fun, Function::Sqrt),
                    _ => assert!(false),
                }

                match tokens_postfix[2] {
                    Token::Number(number) => assert_eq!(number, 3.0),
                    _ => assert!(false),
                }

                match tokens_postfix[3] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Divide),
                    _ => assert!(false),
                }

                match tokens_postfix[4] {
                    Token::Number(number) => assert_eq!(number, 3.0),
                    _ => assert!(false),
                }

                match tokens_postfix[5] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Multiply),
                    _ => assert!(false),
                }

                match tokens_postfix[6] {
                    Token::Function(fun) => assert_eq!(fun, Function::Sin),
                    _ => assert!(false),
                }
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_infix_to_postfix_expression_with_numbers_minus_unary_operator_function() {
        let mut tokens: Vec<Token> = vec![
            Token::Function(Function::Acos),
            Token::LeftParenthesis,
            Token::UnaryOperator(UnaryOperator::Minus),
            Token::Number(1.0),
            Token::RightParenthesis,
        ];

        match infix_to_postfix(&mut tokens) {
            Ok(tokens_postfix) => {
                assert_eq!(tokens_postfix.len(), 3);

                match tokens_postfix[0] {
                    Token::Number(number) => assert_eq!(number, 1.0),
                    _ => assert!(false),
                }

                match tokens_postfix[1] {
                    Token::UnaryOperator(ops) => assert_eq!(ops, UnaryOperator::Minus),
                    _ => assert!(false),
                }

                match tokens_postfix[2] {
                    Token::Function(fun) => assert_eq!(fun, Function::Acos),
                    _ => assert!(false),
                }
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_infix_to_postfix_expression_with_constant_function() {
        let mut tokens: Vec<Token> = vec![
            Token::Function(Function::Cos),
            Token::LeftParenthesis,
            Token::Constant(constants::PI),
            Token::RightParenthesis,
        ];

        match infix_to_postfix(&mut tokens) {
            Ok(tokens_postfix) => {
                assert_eq!(tokens_postfix.len(), 2);

                match tokens_postfix[0] {
                    Token::Constant(number) => assert_eq!(number, constants::PI),
                    _ => assert!(false),
                }

                match tokens_postfix[1] {
                    Token::Function(fun) => assert_eq!(fun, Function::Cos),
                    _ => assert!(false),
                }
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_infix_to_postfix_expression_forgot_left_parenthesis() {
        let mut tokens: Vec<Token> = vec![
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

        match infix_to_postfix(&mut tokens) {
            Ok(_tokens_postfix) => assert!(false),
            Err(message) => assert!(message.len() > 0),
        }
    }

    #[test]
    fn test_infix_to_postfix_expression_forgot_right_parenthesis() {
        let mut tokens: Vec<Token> = vec![
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

        match infix_to_postfix(&mut tokens) {
            Ok(_tokens_postfix) => assert!(false),
            Err(message) => assert!(message.len() > 0),
        }
    }
}
