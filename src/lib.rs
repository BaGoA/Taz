mod constants;
mod functions;
mod operators;
mod token;

mod converter;
mod evaluator;
mod tokenizer;

use std::collections::HashMap;

/// Evaluate an expression that can contain customs variables given in argument.
/// These custom variables is represented with hash map which associate name of variable and its value.
///
/// If error occurs during evaluation, an error message is stored in string contained in Result output.
/// Otherwise, the Result output contains the value of evaluation stored in 64-bits float.
///
/// # Example of simple expression
/// ```
/// use taz;
/// use std::collections::HashMap;
///
/// let expression: String = String::from("2.0 * (4.43 - 5.99) / 3.0");
///
/// let result: Result<f64, String> = taz::evaluate(&expression, &HashMap::new());
/// assert!(result.is_ok());
///
/// match result {
///     Ok(value) => println!("{expression} = {value}"),
///     Err(message) => println!("Error occured: {message}")
/// }
/// ```
///
/// # Example of expression containing predefined constants and function
/// ```
/// use taz;
/// use std::collections::HashMap;
///
/// let expression: String = String::from("cos(pi / 4.0)^2 + sin(pi / 4.0)^2");
///
/// let result: Result<f64, String> = taz::evaluate(&expression, &HashMap::new());
/// assert!(result.is_ok());
///
/// match result {
///     Ok(value) => println!("{expression} = {value}"),
///     Err(message) => println!("Error occured: {message}")
/// }
///
/// ```
///
/// # Example of expression containing customs variables
/// ```
/// use taz;
/// use std::collections::HashMap;
///
/// let expression: String = String::from("cos(theta) * sin(phi)");
///
/// let variables: HashMap<String, f64> = HashMap::from([
///    (String::from("theta"), 0.25),
///    (String::from("phi"), 1.54)
/// ]);
///
/// let result: Result<f64, String> = taz::evaluate(&expression, &variables);
/// assert!(result.is_ok());
///
/// match result {
///     Ok(value) => println!("{expression} = {value}"),
///     Err(message) => println!("Error occured: {message}")
/// }
/// ```
pub fn evaluate(expression: &String, variables: &HashMap<String, f64>) -> Result<f64, String> {
    let tokens: Vec<token::Token> = tokenizer::tokenize(expression.as_str(), variables)?;
    let posfix_tokens: Vec<token::Token> = converter::infix_to_postfix(tokens)?;

    return evaluator::postfix_evaluation(posfix_tokens);
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

        match evaluate(&expression, &HashMap::new()) {
            Ok(result) => assert!(relative_error(result, reference) < 0.01),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_evaluation_expresion_with_numbers_operators() {
        let expression: String = String::from("-43.75 + 20.97");
        let reference: f64 = -43.75 + 20.97;

        match evaluate(&expression, &HashMap::new()) {
            Ok(result) => assert!(relative_error(result, reference) < 0.01),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_evaluation_expression_with_numbers_operators_parenthesis() {
        let expression: String = String::from("43.75 + (-20.97 / 2.87) * 3.14");
        let reference: f64 = 43.75 + (-20.97 / 2.87) * 3.14;

        match evaluate(&expression, &HashMap::new()) {
            Ok(result) => assert!(relative_error(result, reference) < 0.01),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_evaluation_expression_with_function_and_number() {
        let expression: String = String::from("sqrt(9.0)");
        let reference: f64 = 3.0;

        match evaluate(&expression, &HashMap::new()) {
            Ok(result) => assert!(relative_error(result, reference) < 0.01),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_evaluation_expression_with_constant_and_number() {
        let expression: String = String::from("pi / 2.0");
        let reference: f64 = std::f64::consts::PI / 2.0;

        match evaluate(&expression, &HashMap::new()) {
            Ok(result) => assert!(relative_error(result, reference) < 0.01),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_evaluation_expression_with_all() {
        let expression: String = String::from("sin(2.0 - pi) * cos((-pi + 2.0) / 2.0)");
        let reference: f64 =
            (2.0 - std::f64::consts::PI).sin() * ((-std::f64::consts::PI + 2.0) / 2.0).cos();

        match evaluate(&expression, &HashMap::new()) {
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

        match evaluate(&expression, &variables) {
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

        match evaluate(&expression, &variables) {
            Ok(result) => assert!(relative_error(result, reference) < 0.01),
            Err(_) => assert!(false),
        }
    }
}
