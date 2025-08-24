use crate::expression::token_iterator::TokenIterator;
use crate::token::constants::*;
use crate::token::functions::Function;
use crate::token::operators::{BinaryOperator, UnaryOperator};
use crate::token::Token;

use std::iter::Peekable;
use std::num::ParseFloatError;
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
/// If we don't find a number, we return an error message in Err of the result.
fn extract_number(char_it: &mut Peekable<Chars<'_>>) -> Result<f64, String> {
    let str_number: String = extract_if(char_it, |c: char| c.is_digit(10) || c == '.');

    return str_number
        .parse()
        .map_err(|err: ParseFloatError| err.to_string());
}

/// Extract a word from string given by user via its char iterator
fn extract_word(char_it: &mut Peekable<Chars<'_>>) -> String {
    return extract_if(char_it, |c: char| c.is_alphanumeric() || c == '_');
}

/// Infix is an iterator over tokens of an infix expression
pub struct Infix<'a> {
    chars_iterator: Peekable<Chars<'a>>,
    last_extracted_token: Token,
    is_first_token: bool,
}

impl<'a> Infix<'a> {
    /// Create Infix iterator from raw expression
    pub fn new(expression: &'a str) -> Self {
        return Self {
            chars_iterator: expression.chars().peekable(),
            last_extracted_token: Token::Empty,
            is_first_token: true,
        };
    }
}

impl TokenIterator for Infix<'_> {
    fn next_token(&mut self) -> Result<Token, String> {
        let mut next_token: Result<Token, String> = Ok(Token::Stop);

        match self.chars_iterator.peek() {
            Some(mut c) => {
                // Skip whitespace
                while c.is_whitespace() {
                    self.chars_iterator.next();

                    match self.chars_iterator.peek() {
                        Some(next_char) => c = next_char,
                        None => return Ok(Token::Stop),
                    }
                }

                // Extract token
                if c.is_digit(10) {
                    next_token = extract_number(self.chars_iterator.by_ref())
                        .map(|number: f64| Token::new_number(number));
                } else if BinaryOperator::is_ops(*c) || UnaryOperator::is_ops(*c) {
                    next_token = if self.is_first_token
                        || self.last_extracted_token == Token::LeftParenthesis
                    {
                        Token::new_unary_ops(*c)
                    } else {
                        Token::new_binary_ops(*c)
                    };

                    self.chars_iterator.next();
                } else if *c == '(' {
                    next_token = Ok(Token::LeftParenthesis);
                    self.chars_iterator.next();
                } else if *c == ')' {
                    next_token = Ok(Token::RightParenthesis);
                    self.chars_iterator.next();
                } else if c.is_alphanumeric() {
                    let name: String = extract_word(self.chars_iterator.by_ref());

                    next_token = if is_constant(name.as_str()) {
                        Token::new_constant(name.as_str())
                    } else if Function::is_fun(name.as_str()) {
                        Token::new_function(name.as_str())
                    } else {
                        Err(format!(
                            "The string {} does not correspond to existing tokens",
                            name
                        ))
                    }
                } else {
                    next_token = Err(format!(
                        "The character {} does not correspond to existing tokens",
                        c
                    ))
                }
            }
            None => (),
        }

        self.is_first_token = false;
        self.last_extracted_token = match next_token {
            Ok(token) => token,
            Err(_) => Token::Stop,
        };

        return next_token;
    }
}

// Units tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_number_integer_solo() {
        let number: i64 = 4354;
        let str_number: String = number.to_string();

        match extract_number(str_number.chars().peekable().by_ref()) {
            Ok(extracted_number) => assert_eq!(extracted_number, number as f64),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_extract_number_float_solo() {
        let number: f64 = 4354.75;
        let str_number: String = number.to_string();

        match extract_number(str_number.chars().peekable().by_ref()) {
            Ok(extracted_number) => assert_eq!(extracted_number, number as f64),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_extract_number_integer_with_string() {
        let number: i64 = 4354;
        let mut str_number: String = number.to_string();

        str_number.push_str("Hello World");

        match extract_number(str_number.chars().peekable().by_ref()) {
            Ok(extracted_number) => assert_eq!(extracted_number, number as f64),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_extract_number_float_with_string() {
        let number: f64 = 4354.75;
        let mut str_number: String = number.to_string();

        str_number.push_str("Hello World");

        match extract_number(str_number.chars().peekable().by_ref()) {
            Ok(extracted_number) => assert_eq!(extracted_number, number as f64),
            Err(_) => assert!(false),
        }
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

        match extract_number(char_it.peekable().by_ref()) {
            Ok(extracted_number) => assert_eq!(extracted_number, number as f64),
            Err(_) => assert!(false),
        }
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

        match extract_number(char_it.peekable().by_ref()) {
            Ok(extracted_number) => assert_eq!(extracted_number, number as f64),
            Err(_) => assert!(false),
        }
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

    fn tokenize(expression: &str) -> Result<Vec<Token>, String> {
        let infix = Infix::new(expression);
        return infix.collect_all_tokens();
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
