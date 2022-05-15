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

mod evaluator;
mod tokenizer;

/// Evaluation of mathematical expression
pub fn evaluate(expression: &String) -> Result<f64, String> {
    return evaluator::postfix_evaluation(&evaluator::infix_to_postfix(&tokenizer::tokenize(
        expression.as_str(),
    )?)?);
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
}
