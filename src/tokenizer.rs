use super::constants::*;
use super::functions::Function;
use super::operators::{BinaryOperator, UnaryOperator};
use super::token::Token;

use std::iter::Peekable;
use std::ops::Fn;
use std::str::Chars;

/// Extract a substring from string given by user
/// where each characters check a predicat
fn extract_if<P>(char_it: &mut Peekable<Chars<'_>>, predicate: P) -> String
where
    P: Fn(char) -> bool,
{
    let mut substr: String = String::new();

    // Search maximal size that can reach the substr to reserve the memory space
    if let (_lb_size, Some(ub_size)) = char_it.size_hint() {
        substr.reserve(ub_size);
    }

    while let Some(&c) = char_it.peek() {
        if predicate(c) {
            substr.push(c);
            char_it.next();
        } else {
            break;
        }
    }

    substr.shrink_to_fit();
    return substr;
}

/// Extract a number from string given by user via its char iterator
/// We return an Option<f64>, if we don't find a number the option is none.
fn extract_number(char_it: &mut Peekable<Chars<'_>>) -> Option<f64> {
    let str_number: String = extract_if(char_it, |c: char| c.is_digit(10) || c == '.');
    return str_number.parse().ok();
}

/// Extract a word from string given by user via its char iterator
fn extract_word(char_it: &mut Peekable<Chars<'_>>) -> String {
    return extract_if(char_it, |c: char| c.is_alphanumeric() || c == '_');
}

/// Tokenization of expression given in argument as string.
/// If error occurs during evaluation, an error message is stored
/// in string contained in Result output
pub fn tokenize(expression: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::with_capacity(expression.len());
    let mut char_it = expression.chars().peekable();

    while let Some(&c) = char_it.peek() {
        if c.is_whitespace() {
            char_it.next();
        } else if c.is_digit(10) {
            match extract_number(char_it.by_ref()) {
                Some(number) => tokens.push(Token::new_number(number)),
                None => return Err(String::from("Cannot parse this expression")),
            }
        } else if BinaryOperator::is_ops(c) || UnaryOperator::is_ops(c) {
            if tokens.is_empty() {
                tokens.push(Token::new_unary_ops(c)?);
            } else {
                match tokens.last().unwrap() {
                    &Token::LeftParenthesis => tokens.push(Token::new_unary_ops(c)?),
                    _ => tokens.push(Token::new_binary_ops(c)?),
                }
            }

            char_it.next();
        } else if c == '(' {
            tokens.push(Token::LeftParenthesis);
            char_it.next();
        } else if c == ')' {
            tokens.push(Token::RightParenthesis);
            char_it.next();
        } else if c.is_alphanumeric() {
            let name: String = extract_word(char_it.by_ref());

            if is_constant(name.as_str()) {
                tokens.push(Token::new_constant(name.as_str())?);
            } else if Function::is_fun(name.as_str()) {
                tokens.push(Token::new_function(name.as_str())?);
            } else {
                return Err(String::from("Cannot parse this expression"));
            }
        } else {
            return Err(String::from("Cannot parse this expression"));
        }
    }

    return Ok(tokens);
}

// Units tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_number_integer_solo() {
        let number: i64 = 4354;
        let str_number: String = number.to_string();

        let value: Option<f64> = extract_number(str_number.chars().peekable().by_ref());
        assert!(value.is_some());
        assert_eq!(value.unwrap(), number as f64);
    }

    #[test]
    fn test_extract_number_float_solo() {
        let number: f64 = 4354.75;
        let str_number: String = number.to_string();

        let value: Option<f64> = extract_number(str_number.chars().peekable().by_ref());
        assert!(value.is_some());
        assert_eq!(value.unwrap(), number);
    }

    #[test]
    fn test_extract_number_integer_with_string() {
        let number: i64 = 4354;
        let mut str_number: String = number.to_string();

        str_number.push_str("Hello World");

        let value: Option<f64> = extract_number(str_number.chars().peekable().by_ref());
        assert!(value.is_some());
        assert_eq!(value.unwrap(), number as f64);
    }

    #[test]
    fn test_extract_number_float_with_string() {
        let number: f64 = 4354.75;
        let mut str_number: String = number.to_string();

        str_number.push_str("Hello World");

        let value: Option<f64> = extract_number(str_number.chars().peekable().by_ref());
        assert!(value.is_some());
        assert_eq!(value.unwrap(), number);
    }

    #[test]
    fn test_extract_number_integer_arounded_by_string() {
        let number: i64 = 4354;

        let mut expression: String = String::from("sqrt(");
        expression.push_str(number.to_string().as_str());
        expression.push(')');

        let mut char_it = expression.chars();
        assert_eq!(char_it.next(), Some('s'));
        assert_eq!(char_it.next(), Some('q'));
        assert_eq!(char_it.next(), Some('r'));
        assert_eq!(char_it.next(), Some('t'));
        assert_eq!(char_it.next(), Some('('));

        let value: Option<f64> = extract_number(char_it.peekable().by_ref());
        assert!(value.is_some());
        assert_eq!(value.unwrap(), number as f64);
    }

    #[test]
    fn test_extract_number_float_arounded_by_string() {
        let number: f64 = 4354.75;

        let mut expression: String = String::from("sqrt(");
        expression.push_str(number.to_string().as_str());
        expression.push(')');

        let mut char_it = expression.chars();
        assert_eq!(char_it.next(), Some('s'));
        assert_eq!(char_it.next(), Some('q'));
        assert_eq!(char_it.next(), Some('r'));
        assert_eq!(char_it.next(), Some('t'));
        assert_eq!(char_it.next(), Some('('));

        let value: Option<f64> = extract_number(char_it.peekable().by_ref());
        assert!(value.is_some());
        assert_eq!(value.unwrap(), number);
    }

    #[test]
    fn test_extract_word_solo() {
        let expression: String = String::from("abs");
        let word: String = extract_word(expression.chars().peekable().by_ref());

        assert_eq!(expression, word);
    }

    #[test]
    fn test_extract_word_with_seperator_solo() {
        let expression: String = String::from("abs_f");
        let word: String = extract_word(expression.chars().peekable().by_ref());

        assert_eq!(expression, word);
    }

    #[test]
    fn test_extract_word_with_number_solo() {
        let expression: String = String::from("log10");
        let word: String = extract_word(expression.chars().peekable().by_ref());

        assert_eq!(expression, word);
    }

    #[test]
    fn test_extract_word_with_parenthesis() {
        let expression: String = String::from("abs(");
        let word: String = extract_word(expression.chars().peekable().by_ref());

        let word_ref: String = String::from("abs");
        assert_eq!(word_ref, word);
    }

    #[test]
    fn test_extract_word_in_expression() {
        let expression: String = String::from("Hello Ariane 5");
        let mut char_it = expression.chars();

        assert_eq!(char_it.next(), Some('H'));
        assert_eq!(char_it.next(), Some('e'));
        assert_eq!(char_it.next(), Some('l'));
        assert_eq!(char_it.next(), Some('l'));
        assert_eq!(char_it.next(), Some('o'));
        assert_eq!(char_it.next(), Some(' '));

        let word: String = extract_word(char_it.peekable().by_ref());
        let word_ref: String = String::from("Ariane");
        assert_eq!(word_ref, word);
    }

    #[test]
    fn test_tokenization_expression_with_number() {
        let expression: &str = "4354.75";
        let number_ref: f64 = 4354.75;

        match tokenize(expression) {
            Ok(tokens) => {
                assert_eq!(tokens.len(), 1);

                match tokens[0] {
                    Token::Number(number) => assert_eq!(number, number_ref),
                    _ => assert!(false),
                }
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_tokenization_expression_with_numbers_binary_operator() {
        let expression: &str = "43.75 - 20.97";
        let left_number_ref: f64 = 43.75;
        let right_number_ref: f64 = 20.97;

        match tokenize(expression) {
            Ok(tokens) => {
                assert_eq!(tokens.len(), 3);

                match tokens[0] {
                    Token::Number(number) => assert_eq!(number, left_number_ref),
                    _ => assert!(false),
                }

                match tokens[1] {
                    Token::BinaryOperator(operator) => assert_eq!(operator, BinaryOperator::Minus),
                    _ => assert!(false),
                }

                match tokens[2] {
                    Token::Number(number) => assert_eq!(number, right_number_ref),
                    _ => assert!(false),
                }
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_tokenization_expresion_with_numbers_operators() {
        let expression: &str = "-43.75 + 20.97";
        let left_number_ref: f64 = 43.75;
        let right_number_ref: f64 = 20.97;

        match tokenize(expression) {
            Ok(tokens) => {
                assert_eq!(tokens.len(), 4);

                match tokens[0] {
                    Token::UnaryOperator(operator) => assert_eq!(operator, UnaryOperator::Minus),
                    _ => assert!(false),
                }

                match tokens[1] {
                    Token::Number(number) => assert_eq!(number, left_number_ref),
                    _ => assert!(false),
                }

                match tokens[2] {
                    Token::BinaryOperator(operator) => assert_eq!(operator, BinaryOperator::Plus),
                    _ => assert!(false),
                }

                match tokens[3] {
                    Token::Number(number) => assert_eq!(number, right_number_ref),
                    _ => assert!(false),
                }
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_tokenization_expression_with_numbers_operators_parenthesis() {
        let expression: &str = "43.75 + (-20.97 / 2.87) * 3.14";
        let numbers: Vec<f64> = vec![43.75, 20.97, 2.87, 3.14];

        match tokenize(expression) {
            Ok(tokens) => {
                assert_eq!(tokens.len(), 10);

                match tokens[0] {
                    Token::Number(number) => assert_eq!(number, numbers[0]),
                    _ => assert!(false),
                }

                match tokens[1] {
                    Token::BinaryOperator(operator) => assert_eq!(operator, BinaryOperator::Plus),
                    _ => assert!(false),
                }

                match tokens[2] {
                    Token::LeftParenthesis => assert!(true),
                    _ => assert!(false),
                }

                match tokens[3] {
                    Token::UnaryOperator(operator) => assert_eq!(operator, UnaryOperator::Minus),
                    _ => assert!(false),
                }

                match tokens[4] {
                    Token::Number(number) => assert_eq!(number, numbers[1]),
                    _ => assert!(false),
                }

                match tokens[5] {
                    Token::BinaryOperator(operator) => assert_eq!(operator, BinaryOperator::Divide),
                    _ => assert!(false),
                }

                match tokens[6] {
                    Token::Number(number) => assert_eq!(number, numbers[2]),
                    _ => assert!(false),
                }

                match tokens[7] {
                    Token::RightParenthesis => assert!(true),
                    _ => assert!(false),
                }

                match tokens[8] {
                    Token::BinaryOperator(operator) => {
                        assert_eq!(operator, BinaryOperator::Multiply)
                    }
                    _ => assert!(false),
                }

                match tokens[9] {
                    Token::Number(number) => assert_eq!(number, numbers[3]),
                    _ => assert!(false),
                }
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_tokenization_expression_with_function_and_number() {
        let expression: &str = "sqrt(9.0)";
        let number_ref: f64 = 9.0;

        match tokenize(expression) {
            Ok(tokens) => {
                assert_eq!(tokens.len(), 4);

                match tokens[0] {
                    Token::Function(fun) => assert_eq!(fun, Function::Sqrt),
                    _ => assert!(false),
                }

                match tokens[1] {
                    Token::LeftParenthesis => assert!(true),
                    _ => assert!(false),
                }

                match tokens[2] {
                    Token::Number(number) => assert_eq!(number, number_ref),
                    _ => assert!(false),
                }

                match tokens[3] {
                    Token::RightParenthesis => assert!(true),
                    _ => assert!(false),
                }
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_tokenization_expression_with_constant_and_number() {
        let expression: &str = "pi / 2.0";
        let number_ref: f64 = 2.0;

        match tokenize(expression) {
            Ok(tokens) => {
                assert_eq!(tokens.len(), 3);

                match tokens[0] {
                    Token::Constant(constant) => assert_eq!(constant, PI),
                    _ => assert!(false),
                }

                match tokens[1] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Divide),
                    _ => assert!(false),
                }

                match tokens[2] {
                    Token::Number(number) => assert_eq!(number, number_ref),
                    _ => assert!(false),
                }
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_tokenization_expression_with_all() {
        let expression: &str = "sin(2.0 - pi) * cos((-pi + 2.0) / 2.0)";
        let number_ref: f64 = 2.0;

        match tokenize(expression) {
            Ok(tokens) => {
                assert_eq!(tokens.len(), 18);

                match tokens[0] {
                    Token::Function(fun) => assert_eq!(fun, Function::Sin),
                    _ => assert!(false),
                }

                match tokens[1] {
                    Token::LeftParenthesis => assert!(true),
                    _ => assert!(false),
                }

                match tokens[2] {
                    Token::Number(number) => assert_eq!(number, number_ref),
                    _ => assert!(false),
                }

                match tokens[3] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Minus),
                    _ => assert!(false),
                }

                match tokens[4] {
                    Token::Constant(constant) => assert_eq!(constant, PI),
                    _ => assert!(false),
                }

                match tokens[5] {
                    Token::RightParenthesis => assert!(true),
                    _ => assert!(false),
                }

                match tokens[6] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Multiply),
                    _ => assert!(false),
                }

                match tokens[7] {
                    Token::Function(fun) => assert_eq!(fun, Function::Cos),
                    _ => assert!(false),
                }

                match tokens[8] {
                    Token::LeftParenthesis => assert!(true),
                    _ => assert!(false),
                }

                match tokens[9] {
                    Token::LeftParenthesis => assert!(true),
                    _ => assert!(false),
                }

                match tokens[10] {
                    Token::UnaryOperator(ops) => assert_eq!(ops, UnaryOperator::Minus),
                    _ => assert!(false),
                }

                match tokens[11] {
                    Token::Constant(constant) => assert_eq!(constant, PI),
                    _ => assert!(false),
                }

                match tokens[12] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Plus),
                    _ => assert!(false),
                }

                match tokens[13] {
                    Token::Number(number) => assert_eq!(number, number_ref),
                    _ => assert!(false),
                }

                match tokens[14] {
                    Token::RightParenthesis => assert!(true),
                    _ => assert!(false),
                }

                match tokens[15] {
                    Token::BinaryOperator(ops) => assert_eq!(ops, BinaryOperator::Divide),
                    _ => assert!(false),
                }

                match tokens[16] {
                    Token::Number(number) => assert_eq!(number, number_ref),
                    _ => assert!(false),
                }

                match tokens[17] {
                    Token::RightParenthesis => assert!(true),
                    _ => assert!(false),
                }
            }
            Err(_) => assert!(false),
        }
    }
}
