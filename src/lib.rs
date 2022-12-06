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

mod constants;
mod functions;
mod operators;
mod token;

mod converter;
mod evaluator;
mod tokenizer;

use std::collections::HashMap;

/// Evaluation of expression without variables
pub fn evaluate(expression: &String) -> Result<f64, String> {
    let variables: HashMap<String, f64> = HashMap::new();
    let tokens: Vec<token::Token> = tokenizer::tokenize(expression.as_str(), &variables)?;

    let posfix_tokens: Vec<token::Token> = converter::infix_to_postfix(&tokens)?;
    return evaluator::postfix_evaluation(&posfix_tokens);
}

pub fn evaluate_with_variables(
    expression: &String,
    variables: &HashMap<String, f64>,
) -> Result<f64, String> {
    let tokens: Vec<token::Token> = tokenizer::tokenize(expression.as_str(), variables)?;
    let posfix_tokens: Vec<token::Token> = converter::infix_to_postfix(&tokens)?;
    return evaluator::postfix_evaluation(&posfix_tokens);
}

/// Units tests
#[cfg(test)]
mod tests {
    use super::*;

    fn relative_error(value: f64, reference: f64) -> f64 {
        if reference == 0.0 {
            return value.abs();
        } else {
            return (value - reference).abs() / reference.abs();
        }
    }

    #[test]
    fn test_evaluation_expression_with_numbers_binary_operator() {
        let expression: String = String::from("43.75 - 20.97");
        let reference: f64 = 43.75 - 20.97;

        match evaluate(&expression) {
            Ok(result) => assert!(relative_error(result, reference) < 0.01),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_evaluation_expresion_with_numbers_operators() {
        let expression: String = String::from("-43.75 + 20.97");
        let reference: f64 = -43.75 + 20.97;

        match evaluate(&expression) {
            Ok(result) => assert!(relative_error(result, reference) < 0.01),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_evaluation_expression_with_numbers_operators_parenthesis() {
        let expression: String = String::from("43.75 + (-20.97 / 2.87) * 3.14");
        let reference: f64 = 43.75 + (-20.97 / 2.87) * 3.14;

        match evaluate(&expression) {
            Ok(result) => assert!(relative_error(result, reference) < 0.01),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_evaluation_expression_with_function_and_number() {
        let expression: String = String::from("sqrt(9.0)");
        let reference: f64 = 3.0;

        match evaluate(&expression) {
            Ok(result) => assert!(relative_error(result, reference) < 0.01),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_evaluation_expression_with_constant_and_number() {
        let expression: String = String::from("pi / 2.0");
        let reference: f64 = std::f64::consts::PI / 2.0;

        match evaluate(&expression) {
            Ok(result) => assert!(relative_error(result, reference) < 0.01),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_evaluation_expression_with_all() {
        let expression: String = String::from("sin(2.0 - pi) * cos((-pi + 2.0) / 2.0)");
        let reference: f64 =
            (2.0 - std::f64::consts::PI).sin() * ((-std::f64::consts::PI + 2.0) / 2.0).cos();

        match evaluate(&expression) {
            Ok(result) => assert!(relative_error(result, reference) < 0.01),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_evaluation_expression_with_variables() {
        let expression: String = String::from("left - right");

        let left: f64 = 43.75;
        let right: f64 = 20.97;
        let reference: f64 = left - right;

        let variables: HashMap<String, f64> =
            HashMap::from([(String::from("left"), left), (String::from("right"), right)]);

        match evaluate_with_variables(&expression, &variables) {
            Ok(result) => assert!(relative_error(result, reference) < 0.01),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_evaluation_expression_with_variables_and_function() {
        let expression: String = String::from("left - right + sqrt(arg)");

        let left: f64 = 43.75;
        let right: f64 = 20.97;
        let arg: f64 = 9.0;
        let reference: f64 = left - right + arg.sqrt();

        let variables: HashMap<String, f64> = HashMap::from([
            (String::from("left"), left),
            (String::from("right"), right),
            (String::from("arg"), arg),
        ]);

        match evaluate_with_variables(&expression, &variables) {
            Ok(result) => assert!(relative_error(result, reference) < 0.01),
            Err(_) => assert!(false),
        }
    }
}
