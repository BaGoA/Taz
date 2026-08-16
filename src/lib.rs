//! Taz
//!
//! Taz is Rust library to evaluate a mathematical expression.
//!

pub mod error;

mod expression;
mod token;

/// Evaluate a mathematical expression.
///
/// If error occurs during evaluation, an error message is stored in string contained in Result output.
/// Otherwise, the Result output contains the value of evaluation stored in 64-bits float.
///
/// # Example of simple expression
/// ```
/// use taz;
///
/// let expression: String = String::from("2.0 * (4.43 - 5.99) / 3.0");
///
/// let result: Result<f64, taz::error::Error> = taz::evaluate(expression.as_str());
/// assert!(result.is_ok());
///
/// match result {
///     Ok(value) => println!("{expression} = {value}"),
///     Err(error) => println!("Error occured: {}", error.message())
/// }
/// ```
///
/// # Example of expression containing predefined constants and function
/// ```
/// use taz;
///
/// let expression: String = String::from("cos(pi / 4.0)^2 + sin(pi / 4.0)^2");
///
/// let result: Result<f64, taz::error::Error> = taz::evaluate(expression.as_str());
/// assert!(result.is_ok());
///
/// match result {
///     Ok(value) => println!("{expression} = {value}"),
///     Err(error) => println!("Error occured: {}", error.message())
/// }
///
/// ```
pub fn evaluate(raw_expression: &str) -> Result<f64, error::Error> {
    let expression = expression::Expression::new(raw_expression);
    return expression.evaluate();
}
