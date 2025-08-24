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

    /// Create infix expression by returning Infix iterator
    pub fn infix(self) -> infix::Infix<'a> {
        return infix::Infix::new(self.raw_expression);
    }
}
