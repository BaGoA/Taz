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
fn extract_if(
    chars_iterator: &mut Peekable<Chars<'_>>,
    predicate: impl Fn(char) -> bool,
) -> String {
    let mut substr: String = String::new();

    // Search maximal size that can reach the substr to reserve the memory space
    if let (_lb_size, Some(ub_size)) = chars_iterator.size_hint() {
        substr.reserve(ub_size);
    }

    while let Some(&c) = chars_iterator.peek() {
        if predicate(c) {
            substr.push(c);
            chars_iterator.next();
        } else {
            break;
        }
    }

    substr.shrink_to_fit();
    return substr;
}

/// Extract a number from string given by user via its char iterator
/// If we don't find a number, we return an error message in Err of the result.
fn extract_number(chars_iterator: &mut Peekable<Chars<'_>>) -> Result<f64, String> {
    let str_number: String = extract_if(chars_iterator, |c: char| c.is_digit(10) || c == '.');

    return str_number
        .parse()
        .map_err(|err: ParseFloatError| err.to_string());
}

/// Extract a word from string given by user via its char iterator
fn extract_word(chars_iterator: &mut Peekable<Chars<'_>>) -> String {
    return extract_if(chars_iterator, |c: char| c.is_alphanumeric() || c == '_');
}

/// Skip the whitespace in chars iterator
/// It returns true if we reach the end of iterator
fn skip_whitespace(chars_iterator: &mut Peekable<Chars<'_>>) -> bool {
    let mut reach_the_end: bool = false;

    while let Some(c) = chars_iterator.peek() {
        if c.is_whitespace() {
            reach_the_end = chars_iterator.next().is_none();
        } else {
            break;
        }
    }

    return reach_the_end;
}

/// Infix is an iterator over tokens from an infix expression
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

        let reach_the_end: bool = skip_whitespace(&mut self.chars_iterator);

        if reach_the_end {
            return Ok(Token::Stop);
        }

        match self.chars_iterator.peek() {
            Some(c) => {
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

    #[test]
    fn test_infix_expression_with_number() {
        let expression: &str = "4354.75";
        let number_ref: f64 = 4354.75;
        let tokens: Vec<Token> = vec![Token::Number(number_ref)];

        let infix = Infix::new(expression);

        match infix.equal(tokens.as_slice()) {
            Ok(are_equal) => assert!(are_equal),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_infix_expression_with_number_and_whitespace_at_the_begin_and_at_the_end() {
        let expression: &str = "   4354.75  ";
        let number_ref: f64 = 4354.75;
        let tokens: Vec<Token> = vec![Token::Number(number_ref)];

        let infix = Infix::new(expression);

        match infix.equal(tokens.as_slice()) {
            Ok(are_equal) => assert!(are_equal),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_infix_expression_with_numbers_binary_operator() {
        let expression: &str = "43.75 - 20.97";
        let left_number_ref: f64 = 43.75;
        let right_number_ref: f64 = 20.97;

        let tokens: Vec<Token> = vec![
            Token::Number(left_number_ref),
            Token::BinaryOperator(BinaryOperator::Minus),
            Token::Number(right_number_ref),
        ];

        let infix = Infix::new(expression);

        match infix.equal(tokens.as_slice()) {
            Ok(are_equal) => assert!(are_equal),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_infix_expresion_with_numbers_operators() {
        let expression: &str = "-43.75 + 20.97";
        let left_number_ref: f64 = 43.75;
        let right_number_ref: f64 = 20.97;

        let tokens: Vec<Token> = vec![
            Token::UnaryOperator(UnaryOperator::Minus),
            Token::Number(left_number_ref),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(right_number_ref),
        ];

        let infix = Infix::new(expression);

        match infix.equal(tokens.as_slice()) {
            Ok(are_equal) => assert!(are_equal),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_infix_expression_with_numbers_operators_parenthesis() {
        let expression: &str = "43.75 + (-20.97 / 2.87) * 3.14";
        let numbers: Vec<f64> = vec![43.75, 20.97, 2.87, 3.14];

        let tokens: Vec<Token> = vec![
            Token::Number(numbers[0]),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::LeftParenthesis,
            Token::UnaryOperator(UnaryOperator::Minus),
            Token::Number(numbers[1]),
            Token::BinaryOperator(BinaryOperator::Divide),
            Token::Number(numbers[2]),
            Token::RightParenthesis,
            Token::BinaryOperator(BinaryOperator::Multiply),
            Token::Number(numbers[3]),
        ];

        let infix = Infix::new(expression);

        match infix.equal(tokens.as_slice()) {
            Ok(are_equal) => assert!(are_equal),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_infix_expression_with_function_and_number() {
        let expression: &str = "sqrt(9.0)";
        let number_ref: f64 = 9.0;

        let tokens: Vec<Token> = vec![
            Token::Function(Function::Sqrt),
            Token::LeftParenthesis,
            Token::Number(number_ref),
            Token::RightParenthesis,
        ];

        let infix = Infix::new(expression);

        match infix.equal(tokens.as_slice()) {
            Ok(are_equal) => assert!(are_equal),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_infix_expression_with_constant_and_number() {
        let expression: &str = "pi / 2.0";
        let number_ref: f64 = 2.0;

        let tokens: Vec<Token> = vec![
            Token::Constant(PI),
            Token::BinaryOperator(BinaryOperator::Divide),
            Token::Number(number_ref),
        ];

        let infix = Infix::new(expression);

        match infix.equal(tokens.as_slice()) {
            Ok(are_equal) => assert!(are_equal),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_infix_expression_with_all() {
        let expression: &str = "sin(2.0 - pi) * cos((-pi + 2.0) / 2.0)";
        let number_ref: f64 = 2.0;

        let tokens: Vec<Token> = vec![
            Token::Function(Function::Sin),
            Token::LeftParenthesis,
            Token::Number(number_ref),
            Token::BinaryOperator(BinaryOperator::Minus),
            Token::Constant(PI),
            Token::RightParenthesis,
            Token::BinaryOperator(BinaryOperator::Multiply),
            Token::Function(Function::Cos),
            Token::LeftParenthesis,
            Token::LeftParenthesis,
            Token::UnaryOperator(UnaryOperator::Minus),
            Token::Constant(PI),
            Token::BinaryOperator(BinaryOperator::Plus),
            Token::Number(number_ref),
            Token::RightParenthesis,
            Token::BinaryOperator(BinaryOperator::Divide),
            Token::Number(number_ref),
            Token::RightParenthesis,
        ];

        let infix = Infix::new(expression);

        match infix.equal(tokens.as_slice()) {
            Ok(are_equal) => assert!(are_equal),
            Err(_) => assert!(false),
        }
    }
}
