use super::operators::BinaryOperator;
use super::token::Token;

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

/// Convert vector of token corresponding to infix representation of expression
/// to vector of token corresponding to postfix representation
#[allow(dead_code)]
pub fn infix_to_postfix(tokens: &Vec<Token>) -> Result<Vec<Token>, String> {
    // Build postfix expression from infix expression
    let mut tokens_postfix: Vec<Token> = Vec::with_capacity(tokens.len());
    let mut stack_operator: Vec<Token> = Vec::with_capacity(tokens.len());

    for token in tokens {
        match token {
            &Token::Number(_) => tokens_postfix.push(*token),
            &Token::Constant(_) => tokens_postfix.push(*token),
            &Token::BinaryOperator(ops) => {
                // Pop stack operator according to last operators precedence
                while let Some(&stack_last) = stack_operator.last() {
                    if last_operator_is_primary(stack_last, ops) {
                        tokens_postfix.push(stack_last);
                        stack_operator.pop();
                    } else {
                        break;
                    }
                }

                stack_operator.push(*token);
            }
            &Token::UnaryOperator(_) => stack_operator.push(*token),
            &Token::Function(_) => stack_operator.push(*token),
            &Token::LeftParenthesis => stack_operator.push(*token),
            &Token::RightParenthesis => {
                // Pop stack operator between left and right parenthesis
                while let Some(&stack_last) = stack_operator.last() {
                    if stack_last != Token::LeftParenthesis {
                        tokens_postfix.push(stack_last);
                        stack_operator.pop();
                    } else {
                        break;
                    }
                }

                if stack_operator.is_empty() {
                    return Err(String::from("Mismatched parenthesis"));
                }

                // Pop left parenthesis and function from stack operator
                stack_operator.pop();

                if let Some(&stack_last) = stack_operator.last() {
                    match stack_last {
                        Token::Function(_) => {
                            tokens_postfix.push(stack_last);
                            stack_operator.pop();
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    // Push rest of operator. If stack operator contains left parenthesis, then there is an error
    if !stack_operator.is_empty() {
        if stack_operator.contains(&Token::LeftParenthesis) {
            return Err(String::from("Mismatched parenthesis"));
        }

        stack_operator.reverse();
        tokens_postfix.splice(tokens_postfix.len().., stack_operator);
    }

    return Ok(tokens_postfix);
}

// Units tests
#[cfg(test)]
mod tests {
    use super::super::constants;
    use super::super::functions::Function;
    use super::super::operators::UnaryOperator;
    use super::*;

    #[test]
    fn test_infix_to_postfix_expression_with_numbers_plus_operator() {
        let tokens: Vec<Token> = vec![
            Token::Number(2.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(3.0),
        ];

        match infix_to_postfix(&tokens) {
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
        let tokens: Vec<Token> = vec![
            Token::UnaryOperator(UnaryOperator::Minus),
            Token::Number(2.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(3.0),
        ];

        match infix_to_postfix(&tokens) {
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
        let tokens: Vec<Token> = vec![
            Token::Number(8.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(2.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(9.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(3.0),
        ];

        match infix_to_postfix(&tokens) {
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
        let tokens: Vec<Token> = vec![
            Token::Number(8.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(9.0),
            Token::BinaryOperator(BinaryOperator::Multiply),
            Token::Number(2.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(3.0),
        ];

        match infix_to_postfix(&tokens) {
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
        let tokens: Vec<Token> = vec![
            Token::Number(8.0),
            Token::BinaryOperator(BinaryOperator::Divide),
            Token::Number(2.0),
            Token::BinaryOperator(BinaryOperator::Minus),
            Token::Number(9.0),
            Token::BinaryOperator(BinaryOperator::Divide),
            Token::Number(3.0),
        ];

        match infix_to_postfix(&tokens) {
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
        let tokens: Vec<Token> = vec![
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

        match infix_to_postfix(&tokens) {
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
        let tokens: Vec<Token> = vec![
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

        match infix_to_postfix(&tokens) {
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
        let tokens: Vec<Token> = vec![
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

        match infix_to_postfix(&tokens) {
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
        let tokens: Vec<Token> = vec![
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

        match infix_to_postfix(&tokens) {
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
        let tokens: Vec<Token> = vec![
            Token::Function(Function::Acos),
            Token::LeftParenthesis,
            Token::UnaryOperator(UnaryOperator::Minus),
            Token::Number(1.0),
            Token::RightParenthesis,
        ];

        match infix_to_postfix(&tokens) {
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
        let tokens: Vec<Token> = vec![
            Token::Function(Function::Cos),
            Token::LeftParenthesis,
            Token::Constant(constants::PI),
            Token::RightParenthesis,
        ];

        match infix_to_postfix(&tokens) {
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
        let tokens: Vec<Token> = vec![
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

        match infix_to_postfix(&tokens) {
            Ok(_tokens_postfix) => assert!(false),
            Err(message) => assert!(message.len() > 0),
        }
    }

    #[test]
    fn test_infix_to_postfix_expression_forgot_right_parenthesis() {
        let tokens: Vec<Token> = vec![
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

        match infix_to_postfix(&tokens) {
            Ok(_tokens_postfix) => assert!(false),
            Err(message) => assert!(message.len() > 0),
        }
    }
}
