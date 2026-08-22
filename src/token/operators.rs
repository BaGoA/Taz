use crate::error::Error;

/// Trait defining an operator
pub trait Operator {
    /// Association between operator and its precedence
    fn precedence(&self) -> u8;

    /// Association between operator and boolean corresponding to left associativity
    fn is_left_associative(&self) -> bool;
}

/// Available binary operators used library
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
}

impl BinaryOperator {
    /// Create a BinaryOperator from a char
    /// If char given in argument does not correspond to operator,
    /// an error message is stored in string contained in Result output
    pub fn from_char(ops: char) -> Result<BinaryOperator, Error> {
        match ops {
            '+' => Ok(BinaryOperator::Plus),
            '-' => Ok(BinaryOperator::Minus),
            '*' => Ok(BinaryOperator::Multiply),
            '/' => Ok(BinaryOperator::Divide),
            '^' => Ok(BinaryOperator::Power),
            _ => Err(Error::UnknownBinaryOperatorCharacter),
        }
    }

    /// Check if a char corresponds to binary operator
    pub fn is_ops(ops: char) -> bool {
        match ops {
            '+' => true,
            '-' => true,
            '*' => true,
            '/' => true,
            '^' => true,
            _ => false,
        }
    }

    /// Apply the operation on two values given in argument.
    /// For division case, we check that right_operand is non-null.
    /// To take into account this error, the function return a Result<f64, String>
    pub fn apply(&self, left_operand: f64, right_operand: f64) -> Result<f64, Error> {
        match self {
            BinaryOperator::Plus => Ok(left_operand + right_operand),
            BinaryOperator::Minus => Ok(left_operand - right_operand),
            BinaryOperator::Multiply => Ok(left_operand * right_operand),
            BinaryOperator::Divide => {
                if right_operand != 0.0 {
                    return Ok(left_operand / right_operand);
                } else {
                    return Err(Error::DivisionByZero);
                }
            }
            BinaryOperator::Power => Ok(left_operand.powf(right_operand)),
        }
    }
}

impl Operator for BinaryOperator {
    /// Association between operator and its precedence
    fn precedence(&self) -> u8 {
        match self {
            BinaryOperator::Plus => 6,
            BinaryOperator::Minus => 6,
            BinaryOperator::Multiply => 7,
            BinaryOperator::Divide => 7,
            BinaryOperator::Power => 8,
        }
    }

    /// Association between operator and boolean corresponding to left associativity
    fn is_left_associative(&self) -> bool {
        match self {
            BinaryOperator::Plus => true,
            BinaryOperator::Minus => true,
            BinaryOperator::Multiply => true,
            BinaryOperator::Divide => true,
            BinaryOperator::Power => false,
        }
    }
}

//// Available binary operators used in application
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnaryOperator {
    Plus,
    Minus,
}

impl UnaryOperator {
    /// Create a UnaryOperator from a char
    /// If char given in argument does not correspond to operator,
    /// an error message is stored in string contained in Result output
    pub fn from_char(ops: char) -> Result<UnaryOperator, Error> {
        match ops {
            '+' => Ok(UnaryOperator::Plus),
            '-' => Ok(UnaryOperator::Minus),
            _ => Err(Error::UnknownUnaryOperatorCharacter),
        }
    }

    /// Check if a char correspond to unary operator
    pub fn is_ops(ops: char) -> bool {
        match ops {
            '+' => true,
            '-' => true,
            _ => false,
        }
    }

    /// Apply the operation on value given in argument.
    pub fn apply(&self, operand: f64) -> f64 {
        match self {
            UnaryOperator::Plus => operand,
            UnaryOperator::Minus => -operand,
        }
    }
}

impl Operator for UnaryOperator {
    /// Association between operator and its precedence
    fn precedence(&self) -> u8 {
        return 9;
    }

    /// Association between operator and boolean corresponding to left associativity
    fn is_left_associative(&self) -> bool {
        return false;
    }
}

/// Available comparison operators used library
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ComparisonOperator {
    Lower,
    LowerEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
}

impl ComparisonOperator {
    /// Create a ComparisonOperator from a string
    /// If string given in argument does not correspond to operator,
    /// an error message is stored in string contained in Result output
    pub fn from_string(ops: &str) -> Result<ComparisonOperator, Error> {
        match ops {
            "<" => Ok(ComparisonOperator::Lower),
            "<=" => Ok(ComparisonOperator::LowerEqual),
            ">" => Ok(ComparisonOperator::Greater),
            ">=" => Ok(ComparisonOperator::GreaterEqual),
            "==" => Ok(ComparisonOperator::Equal),
            "!=" => Ok(ComparisonOperator::NotEqual),
            _ => Err(Error::UnknownComparisonOperatorString),
        }
    }

    /// Check if a string corresponds to comparison operator
    pub fn is_ops(ops: &str) -> bool {
        match ops {
            "<" => true,
            "<=" => true,
            ">" => true,
            ">=" => true,
            "==" => true,
            "!=" => true,
            _ => false,
        }
    }

    /// Apply the operation on two values given in argument.
    /// For division case, we check that right_operand is non-null.
    /// To take into account this error, the function return a Result<f64, String>
    pub fn apply(&self, left_operand: f64, right_operand: f64) -> Result<f64, Error> {
        let convert_bool_into_f64 = |boolean: bool| -> f64 {
            if boolean {
                return 1.0;
            } else {
                return 0.0;
            }
        };

        match self {
            ComparisonOperator::Lower => Ok(convert_bool_into_f64(left_operand < right_operand)),
            ComparisonOperator::LowerEqual => {
                Ok(convert_bool_into_f64(left_operand <= right_operand))
            }
            ComparisonOperator::Greater => Ok(convert_bool_into_f64(left_operand > right_operand)),
            ComparisonOperator::GreaterEqual => {
                Ok(convert_bool_into_f64(left_operand >= right_operand))
            }
            ComparisonOperator::Equal => Ok(convert_bool_into_f64(left_operand == right_operand)),
            ComparisonOperator::NotEqual => {
                Ok(convert_bool_into_f64(left_operand != right_operand))
            }
        }
    }
}

impl Operator for ComparisonOperator {
    /// Association between operator and its precedence
    fn precedence(&self) -> u8 {
        match self {
            ComparisonOperator::Lower => 5,
            ComparisonOperator::LowerEqual => 5,
            ComparisonOperator::Greater => 5,
            ComparisonOperator::GreaterEqual => 5,
            ComparisonOperator::Equal => 4,
            ComparisonOperator::NotEqual => 4,
        }
    }

    /// Association between operator and boolean corresponding to left associativity
    fn is_left_associative(&self) -> bool {
        return true;
    }
}

// Units tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_operator_from_plus_char() {
        match BinaryOperator::from_char('+') {
            Ok(ops) => assert_eq!(ops, BinaryOperator::Plus),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_binary_operator_from_minus_char() {
        match BinaryOperator::from_char('-') {
            Ok(ops) => assert_eq!(ops, BinaryOperator::Minus),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_binary_operator_from_multiply_char() {
        match BinaryOperator::from_char('*') {
            Ok(ops) => assert_eq!(ops, BinaryOperator::Multiply),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_binary_operator_from_divide_char() {
        match BinaryOperator::from_char('/') {
            Ok(ops) => assert_eq!(ops, BinaryOperator::Divide),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_binary_operator_from_power_char() {
        match BinaryOperator::from_char('^') {
            Ok(ops) => assert_eq!(ops, BinaryOperator::Power),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_binary_operator_from_unknown_char() {
        match BinaryOperator::from_char('!') {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err, Error::UnknownBinaryOperatorCharacter),
        }
    }

    #[test]
    fn test_binary_operator_is_ops() {
        assert!(BinaryOperator::is_ops('+'));
        assert!(BinaryOperator::is_ops('-'));
        assert!(BinaryOperator::is_ops('*'));
        assert!(BinaryOperator::is_ops('/'));
        assert!(BinaryOperator::is_ops('^'));
        assert!(!BinaryOperator::is_ops('!'));
    }

    #[test]
    fn test_binary_operator_precedence() {
        let plus_ops: BinaryOperator = BinaryOperator::Plus;
        assert_eq!(plus_ops.precedence(), 6);

        let minus_ops: BinaryOperator = BinaryOperator::Minus;
        assert_eq!(minus_ops.precedence(), 6);

        let multiply_ops: BinaryOperator = BinaryOperator::Multiply;
        assert_eq!(multiply_ops.precedence(), 7);

        let divide_ops: BinaryOperator = BinaryOperator::Divide;
        assert_eq!(divide_ops.precedence(), 7);

        let power_ops: BinaryOperator = BinaryOperator::Power;
        assert_eq!(power_ops.precedence(), 8);
    }

    #[test]
    fn test_binary_operator_is_left_associative() {
        let plus_ops: BinaryOperator = BinaryOperator::Plus;
        assert!(plus_ops.is_left_associative());

        let minus_ops: BinaryOperator = BinaryOperator::Minus;
        assert!(minus_ops.is_left_associative());

        let multiply_ops: BinaryOperator = BinaryOperator::Multiply;
        assert!(multiply_ops.is_left_associative());

        let divide_ops: BinaryOperator = BinaryOperator::Divide;
        assert!(divide_ops.is_left_associative());

        let power_ops: BinaryOperator = BinaryOperator::Power;
        assert!(!power_ops.is_left_associative());
    }

    #[test]
    fn test_binary_operator_apply_plus() {
        let left_operand: f64 = 5.0;
        let right_operand: f64 = 2.0;

        let ops_plus: BinaryOperator = BinaryOperator::Plus;
        let ref_plus: f64 = 7.0;

        match ops_plus.apply(left_operand, right_operand) {
            Ok(value) => assert_eq!(value, ref_plus),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_binary_operator_apply_minus() {
        let left_operand: f64 = 5.0;
        let right_operand: f64 = 2.0;

        let ops_minus: BinaryOperator = BinaryOperator::Minus;
        let ref_minus: f64 = 3.0;

        match ops_minus.apply(left_operand, right_operand) {
            Ok(value) => assert_eq!(value, ref_minus),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_binary_operator_apply_multiply() {
        let left_operand: f64 = 5.0;
        let right_operand: f64 = 2.0;

        let ops_multiply: BinaryOperator = BinaryOperator::Multiply;
        let ref_multiply: f64 = 10.0;

        match ops_multiply.apply(left_operand, right_operand) {
            Ok(value) => assert_eq!(value, ref_multiply),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_binary_operator_apply_divide() {
        let left_operand: f64 = 6.0;
        let right_operand: f64 = 2.0;

        let ops_divide: BinaryOperator = BinaryOperator::Divide;
        let ref_divide: f64 = 3.0;

        match ops_divide.apply(left_operand, right_operand) {
            Ok(value) => assert_eq!(value, ref_divide),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_binary_operator_apply_divide_by_zero() {
        let left_operand: f64 = 5.0;
        let right_operand: f64 = 0.0;

        let ops_divide: BinaryOperator = BinaryOperator::Divide;

        match ops_divide.apply(left_operand, right_operand) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err, Error::DivisionByZero),
        }
    }

    #[test]
    fn test_binary_operator_apply_power() {
        let left_operand: f64 = 5.0;
        let right_operand: f64 = 2.0;

        let ops_power: BinaryOperator = BinaryOperator::Power;
        let ref_power: f64 = 25.0;

        match ops_power.apply(left_operand, right_operand) {
            Ok(value) => assert_eq!(value, ref_power),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_unary_operator_from_plus_char() {
        match UnaryOperator::from_char('+') {
            Ok(ops) => assert_eq!(ops, UnaryOperator::Plus),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_unary_operator_from_minus_char() {
        match UnaryOperator::from_char('-') {
            Ok(ops) => assert_eq!(ops, UnaryOperator::Minus),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_unary_operator_is_ops() {
        assert!(UnaryOperator::is_ops('+'));
        assert!(UnaryOperator::is_ops('-'));
        assert!(!UnaryOperator::is_ops('!'));
    }

    #[test]
    fn test_unary_operator_apply_plus() {
        let operand: f64 = 5.0;
        let ops_plus: UnaryOperator = UnaryOperator::Plus;

        assert_eq!(ops_plus.apply(operand), operand);
    }

    #[test]
    fn test_unary_operator_apply_minus() {
        let operand: f64 = 5.0;
        let ops_minus: UnaryOperator = UnaryOperator::Minus;

        assert_eq!(ops_minus.apply(operand), -operand);
    }

    #[test]
    fn test_unary_operator_precedence() {
        let plus_ops: UnaryOperator = UnaryOperator::Plus;
        assert_eq!(plus_ops.precedence(), 9);

        let minus_ops: UnaryOperator = UnaryOperator::Minus;
        assert_eq!(minus_ops.precedence(), 9);
    }

    #[test]
    fn test_unary_operator_is_left_associative() {
        let plus_ops: UnaryOperator = UnaryOperator::Plus;
        assert!(!plus_ops.is_left_associative());

        let minus_ops: UnaryOperator = UnaryOperator::Minus;
        assert!(!minus_ops.is_left_associative());
    }

    #[test]
    fn test_comparison_operator_from_lower_str() {
        match ComparisonOperator::from_string("<") {
            Ok(ops) => assert_eq!(ops, ComparisonOperator::Lower),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_comparison_operator_from_lower_equal_str() {
        match ComparisonOperator::from_string("<=") {
            Ok(ops) => assert_eq!(ops, ComparisonOperator::LowerEqual),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_comparison_operator_from_greater_str() {
        match ComparisonOperator::from_string(">") {
            Ok(ops) => assert_eq!(ops, ComparisonOperator::Greater),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_comparison_operator_from_greater_equal_str() {
        match ComparisonOperator::from_string(">=") {
            Ok(ops) => assert_eq!(ops, ComparisonOperator::GreaterEqual),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_comparison_operator_from_equal_str() {
        match ComparisonOperator::from_string("==") {
            Ok(ops) => assert_eq!(ops, ComparisonOperator::Equal),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_comparison_operator_from_not_equal_str() {
        match ComparisonOperator::from_string("!=") {
            Ok(ops) => assert_eq!(ops, ComparisonOperator::NotEqual),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_comparison_operator_is_ops() {
        assert!(ComparisonOperator::is_ops("<"));
        assert!(ComparisonOperator::is_ops("<="));
        assert!(ComparisonOperator::is_ops(">"));
        assert!(ComparisonOperator::is_ops(">="));
        assert!(ComparisonOperator::is_ops("=="));
        assert!(ComparisonOperator::is_ops("!="));
        assert!(!ComparisonOperator::is_ops("-"));
    }

    #[test]
    fn test_comparison_operator_precedence() {
        let lower_ops: ComparisonOperator = ComparisonOperator::Lower;
        assert_eq!(lower_ops.precedence(), 5);

        let lower_equal_ops: ComparisonOperator = ComparisonOperator::LowerEqual;
        assert_eq!(lower_equal_ops.precedence(), 5);

        let greater_ops: ComparisonOperator = ComparisonOperator::Greater;
        assert_eq!(greater_ops.precedence(), 5);

        let greater_equal_ops: ComparisonOperator = ComparisonOperator::GreaterEqual;
        assert_eq!(greater_equal_ops.precedence(), 5);

        let equal_ops: ComparisonOperator = ComparisonOperator::Equal;
        assert_eq!(equal_ops.precedence(), 4);

        let not_equal_ops: ComparisonOperator = ComparisonOperator::NotEqual;
        assert_eq!(not_equal_ops.precedence(), 4);
    }

    #[test]
    fn test_comparison_operator_apply_lower() {
        let left_operand: f64 = 1.0;
        let right_operand: f64 = 2.0;

        let lower_ops: ComparisonOperator = ComparisonOperator::Lower;

        match lower_ops.apply(left_operand, right_operand) {
            Ok(value) => assert_eq!(value, 1.0),
            Err(_) => assert!(false),
        }

        match lower_ops.apply(right_operand, left_operand) {
            Ok(value) => assert_eq!(value, 0.0),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_comparison_operator_apply_lower_equal() {
        let left_operand: f64 = 1.0;
        let right_operand: f64 = 2.0;

        let lower_equal_ops: ComparisonOperator = ComparisonOperator::LowerEqual;

        match lower_equal_ops.apply(left_operand, right_operand) {
            Ok(value) => assert_eq!(value, 1.0),
            Err(_) => assert!(false),
        }

        match lower_equal_ops.apply(right_operand, left_operand) {
            Ok(value) => assert_eq!(value, 0.0),
            Err(_) => assert!(false),
        }

        match lower_equal_ops.apply(left_operand, left_operand) {
            Ok(value) => assert_eq!(value, 1.0),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_comparison_operator_apply_greater() {
        let left_operand: f64 = 3.0;
        let right_operand: f64 = 2.0;

        let greater_ops: ComparisonOperator = ComparisonOperator::Greater;

        match greater_ops.apply(left_operand, right_operand) {
            Ok(value) => assert_eq!(value, 1.0),
            Err(_) => assert!(false),
        }

        match greater_ops.apply(right_operand, left_operand) {
            Ok(value) => assert_eq!(value, 0.0),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_comparison_operator_apply_greater_equal() {
        let left_operand: f64 = 3.0;
        let right_operand: f64 = 2.0;

        let greater_equal_ops: ComparisonOperator = ComparisonOperator::GreaterEqual;

        match greater_equal_ops.apply(left_operand, right_operand) {
            Ok(value) => assert_eq!(value, 1.0),
            Err(_) => assert!(false),
        }

        match greater_equal_ops.apply(right_operand, left_operand) {
            Ok(value) => assert_eq!(value, 0.0),
            Err(_) => assert!(false),
        }

        match greater_equal_ops.apply(left_operand, left_operand) {
            Ok(value) => assert_eq!(value, 1.0),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_comparison_operator_apply_equal() {
        let left_operand: f64 = 3.0;
        let right_operand: f64 = 2.0;

        let equal_ops: ComparisonOperator = ComparisonOperator::Equal;

        match equal_ops.apply(left_operand, right_operand) {
            Ok(value) => assert_eq!(value, 0.0),
            Err(_) => assert!(false),
        }

        match equal_ops.apply(left_operand, left_operand) {
            Ok(value) => assert_eq!(value, 1.0),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_comparison_operator_apply_not_equal() {
        let left_operand: f64 = 3.0;
        let right_operand: f64 = 2.0;

        let not_equal_ops: ComparisonOperator = ComparisonOperator::NotEqual;

        match not_equal_ops.apply(left_operand, right_operand) {
            Ok(value) => assert_eq!(value, 1.0),
            Err(_) => assert!(false),
        }

        match not_equal_ops.apply(left_operand, left_operand) {
            Ok(value) => assert_eq!(value, 0.0),
            Err(_) => assert!(false),
        }
    }
}
