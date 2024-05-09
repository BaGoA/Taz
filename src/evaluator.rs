use super::token::Token;

/// Evaluate postfix expression given as vector of token
/// If error occurs during evaluation, an error message is stored
/// in string contained in Result output
pub fn postfix_evaluation(tokens: Vec<Token>) -> Result<f64, String> {
    let mut stack_operand: Vec<f64> = Vec::new();
    stack_operand.reserve(10);

    for token in tokens {
        match token {
            Token::Number(number) => stack_operand.push(number),
            Token::BinaryOperator(ops) => {
                if let Some(right) = stack_operand.pop() {
                    if let Some(left) = stack_operand.pop() {
                        stack_operand.push(ops.apply(left, right)?);
                    } else {
                        return Err(String::from(
                            "Missing left operand to apply binary operation",
                        ));
                    }
                } else {
                    return Err(String::from(
                        "Missing right operand to apply binary operation",
                    ));
                }
            }
            Token::UnaryOperator(ops) => {
                if let Some(number) = stack_operand.pop() {
                    stack_operand.push(ops.apply(number));
                } else {
                    return Err(String::from("Missing operand to apply unary operation"));
                }
            }
            Token::Function(fun) => {
                if let Some(arg) = stack_operand.pop() {
                    stack_operand.push(fun.apply(arg)?);
                } else {
                    return Err(String::from("Missing argument to apply function"));
                }
            }
            Token::Constant(constant) => stack_operand.push(constant),
            _ => {
                return Err(String::from(
                    "Token non-accepted for evaluation of postfix expression",
                ));
            }
        }
    }

    return Ok(stack_operand[0]);
}

// Units tests
#[cfg(test)]
mod tests {
    use super::super::constants::PI;
    use super::super::functions::Function;
    use super::super::operators::BinaryOperator;
    use super::super::operators::UnaryOperator;
    use super::*;

    fn relative_error(value: f64, reference: f64) -> f64 {
        if reference == 0.0 {
            return value.abs();
        } else {
            return (value - reference).abs() / reference.abs();
        }
    }

    #[test]
    fn test_postfix_evaluation_with_numbers_operator() {
        let tokens: Vec<Token> = vec![
            Token::Number(2.0),
            Token::Number(3.0),
            Token::BinaryOperator(BinaryOperator::Plus),
        ];

        match postfix_evaluation(tokens) {
            Ok(result) => {
                let result_ref: f64 = 2.0 + 3.0;
                assert!(relative_error(result, result_ref) < 0.01)
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_postfix_evaluation_with_numbers_plus_multiply_operators() {
        let tokens: Vec<Token> = vec![
            Token::Number(8.0),
            Token::Number(9.0),
            Token::Number(2.0),
            Token::BinaryOperator(BinaryOperator::Multiply),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(3.0),
            Token::BinaryOperator(BinaryOperator::Plus),
        ];

        match postfix_evaluation(tokens) {
            Ok(result) => {
                let result_ref: f64 = 8.0 + 9.0 * 2.0 + 3.0;
                assert!(relative_error(result, result_ref) < 0.01)
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_postfix_evaluation_with_numbers_minus_divide_operators() {
        let tokens: Vec<Token> = vec![
            Token::Number(8.0),
            Token::Number(2.0),
            Token::BinaryOperator(BinaryOperator::Divide),
            Token::Number(9.0),
            Token::Number(3.0),
            Token::BinaryOperator(BinaryOperator::Divide),
            Token::BinaryOperator(BinaryOperator::Minus),
        ];

        match postfix_evaluation(tokens) {
            Ok(result) => {
                let result_ref: f64 = 8.0 / 2.0 - 9.0 / 3.0;
                assert!(relative_error(result, result_ref) < 0.01)
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_postfix_evaluation_with_numbers_several_plus_operator() {
        let tokens: Vec<Token> = vec![
            Token::Number(8.0),
            Token::Number(2.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(9.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(3.0),
            Token::BinaryOperator(BinaryOperator::Plus),
        ];

        match postfix_evaluation(tokens) {
            Ok(result) => {
                let result_ref: f64 = 8.0 + 2.0 + 9.0 + 3.0;
                assert!(relative_error(result, result_ref) < 0.01)
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_postfix_evaluation_with_numbers_plus_multiply_operators_parenthesis() {
        let tokens: Vec<Token> = vec![
            Token::Number(8.0),
            Token::Number(2.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(9.0),
            Token::Number(3.0),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::BinaryOperator(BinaryOperator::Multiply),
        ];

        match postfix_evaluation(tokens) {
            Ok(result) => {
                let result_ref: f64 = (8.0 + 2.0) * (9.0 + 3.0);
                assert!(relative_error(result, result_ref) < 0.01)
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_postfix_evaluation_with_numbers_plus_minus_multiply_divide_power_operators() {
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

        match postfix_evaluation(tokens) {
            Ok(result) => {
                let result_ref: f64 = 3.0 + 4.0 * 2.0 / (16.0 as f64).powf(3.0);
                assert!(relative_error(result, result_ref) < 0.01)
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_postfix_evaluation_with_numbers_operators_functions() {
        let tokens: Vec<Token> = vec![
            Token::Number(9.0),
            Token::Function(Function::Sqrt),
            Token::Number(3.0),
            Token::BinaryOperator(BinaryOperator::Divide),
            Token::Number(3.1415),
            Token::BinaryOperator(BinaryOperator::Multiply),
            Token::Function(Function::Sin),
        ];

        match postfix_evaluation(tokens) {
            Ok(result) => {
                let result_ref: f64 = ((9.0 as f64).sqrt() / 3.0 * 3.1415).sin();
                assert!(relative_error(result, result_ref) < 0.01)
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_postfix_evaluation_with_numbers_unary_minus_binary_plus_operator() {
        let tokens: Vec<Token> = vec![
            Token::Number(8.0),
            Token::UnaryOperator(UnaryOperator::Minus),
            Token::Number(9.0),
            Token::BinaryOperator(BinaryOperator::Plus),
        ];

        match postfix_evaluation(tokens) {
            Ok(result) => {
                let result_ref: f64 = -8.0 + 9.0;
                assert!(relative_error(result, result_ref) < 0.01)
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_postfix_evaluation_with_unary_minus_binary_plus_multiply_divide_parenthesis() {
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

        match postfix_evaluation(tokens) {
            Ok(result) => {
                let result_ref: f64 = (8.0 + 2.0) * (-9.0 / 3.0);
                assert!(relative_error(result, result_ref) < 0.01)
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_postfix_evaluation_with_numbers_unary_minus_function() {
        let tokens: Vec<Token> = vec![
            Token::Number(1.0),
            Token::UnaryOperator(UnaryOperator::Minus),
            Token::Function(Function::Acos),
        ];

        match postfix_evaluation(tokens) {
            Ok(result) => {
                let result_ref: f64 = (-1.0 as f64).acos();
                assert!(relative_error(result, result_ref) < 0.01)
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_postfix_evaluation_with_function_constant() {
        let tokens: Vec<Token> = vec![Token::Constant(PI), Token::Function(Function::Cos)];

        match postfix_evaluation(tokens) {
            Ok(result) => {
                let result_ref: f64 = -1.0;
                assert!(relative_error(result, result_ref) < 0.01)
            }
            Err(_) => assert!(false),
        }
    }
}
