//! Taz
//!
//! Taz is Rust library to evaluate a mathematical expression.
//!

pub mod error;

mod expression;
mod token;

/// Evaluate a mathematical expression.
///
/// If error occurs during evaluation, an custom error is stored in Result output. You can
/// obtain a string message by calling the function `message()`.
/// Otherwise, the Result output contains the value of evaluation stored in 64-bits float.
///
/// The evaluator handles unary operator (+, -) and binary operators (+, -, *, / and ^ for power).
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
/// It handles comparison operators (<, <=, >, >=, !=, ==).
///
/// # Example of expression using comparison operators
/// ```
/// use taz;
///
/// let expression: String = String::from("2.0 * (4.43 - 5.99) <= 3.0");
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
/// Also, it handles following functions:
/// - abs: the absolute value
/// - sqrt: the square root
/// - cbrt: the cubic root
/// - exp: the exponential
/// - ln: the natural logarithm
/// - log10: the logarithm with base 10
/// - log2: the logarithm with base 2
/// - sin: the sinus
/// - cos: the cosinus
/// - tan: the tangent
/// - asin: the arc sinus
/// - acos: the arc cosinus
/// - atan: the arc tangent
/// - sinh: the hyperbolic sinus
/// - cosh: the hyperbolic cosinus
/// - tanh: the hyperbolic tangent
/// - asinh: the hyperbolic arc sinus
/// - acosh: the hyperbolic arc cosinus
/// - atanh: the hyperbolic arc tangent
///
/// And the following predefined constants:
///- pi: The constant pi
/// - e: The exponential constant
/// - c: The speed of light constant
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
