mod token_iterator;

mod evaluator;
mod infix;
mod postfix;

/// Definition of expression
pub struct Expression<'a> {
    raw_expression: &'a str,
}

impl<'a> Expression<'a> {
    /// Create Expression from string
    pub fn new(raw_expression: &'a str) -> Self {
        return Self { raw_expression };
    }

    /// Evaluate the expression
    /// If error occurs during evaluation, an error message is stored
    /// in string contained in Result output
    pub fn evaluate(self) -> Result<f64, String> {
        return evaluator::evaluate(postfix::Postfix::new(infix::Infix::new(
            self.raw_expression,
        )));
    }
}
