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

/// Available binary operators used in application
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
    #[allow(dead_code)]
    pub fn from_char(ops: char) -> Result<BinaryOperator, String> {
        match ops {
            '+' => Ok(BinaryOperator::Plus),
            '-' => Ok(BinaryOperator::Minus),
            '*' => Ok(BinaryOperator::Multiply),
            '/' => Ok(BinaryOperator::Divide),
            '^' => Ok(BinaryOperator::Power),
            _ => Err(String::from("Unknown operator characters")),
        }
    }

    /// Check if a char corresponds to binary operator
    #[allow(dead_code)]
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

    /// Association between operator and its precedence
    #[allow(dead_code)]
    pub fn precedence(&self) -> u8 {
        match self {
            BinaryOperator::Plus => 2,
            BinaryOperator::Minus => 2,
            BinaryOperator::Multiply => 3,
            BinaryOperator::Divide => 3,
            BinaryOperator::Power => 4,
        }
    }

    /// Association between operator and boolean corresponding to left associativity
    #[allow(dead_code)]
    pub fn is_left_associative(&self) -> bool {
        match self {
            BinaryOperator::Plus => true,
            BinaryOperator::Minus => true,
            BinaryOperator::Multiply => true,
            BinaryOperator::Divide => true,
            BinaryOperator::Power => false,
        }
    }

    /// Apply the operation on two values given in argument.
    /// For division case, we check that right_operand is non-null.
    /// To take into account this error, the function return a Result<f64, String>
    #[allow(dead_code)]
    pub fn apply(&self, left_operand: f64, right_operand: f64) -> Result<f64, String> {
        match self {
            BinaryOperator::Plus => Ok(left_operand + right_operand),
            BinaryOperator::Minus => Ok(left_operand - right_operand),
            BinaryOperator::Multiply => Ok(left_operand * right_operand),
            BinaryOperator::Divide => {
                if right_operand != 0.0 {
                    return Ok(left_operand / right_operand);
                } else {
                    return Err(String::from("Division by zero"));
                }
            }
            BinaryOperator::Power => Ok(left_operand.powf(right_operand)),
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
    #[allow(dead_code)]
    pub fn from_char(ops: char) -> Result<UnaryOperator, String> {
        match ops {
            '+' => Ok(UnaryOperator::Plus),
            '-' => Ok(UnaryOperator::Minus),
            _ => Err(String::from("Unknown operator characters")),
        }
    }

    /// Check if a char correspond to unary operator
    #[allow(dead_code)]
    pub fn is_ops(ops: char) -> bool {
        match ops {
            '+' => true,
            '-' => true,
            _ => false,
        }
    }

    /// Apply the operation on value given in argument.
    #[allow(dead_code)]
    pub fn apply(&self, operand: f64) -> f64 {
        match self {
            UnaryOperator::Plus => operand,
            UnaryOperator::Minus => -operand,
        }
    }
}

// Units tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_operator_from_plus_char() {
        let res_plus: Result<BinaryOperator, String> = BinaryOperator::from_char('+');
        assert!(res_plus.is_ok());
        assert_eq!(res_plus.unwrap(), BinaryOperator::Plus);
    }

    #[test]
    fn test_binary_operator_from_minus_char() {
        let res_minus: Result<BinaryOperator, String> = BinaryOperator::from_char('-');
        assert!(res_minus.is_ok());
        assert_eq!(res_minus.unwrap(), BinaryOperator::Minus);
    }

    #[test]
    fn test_binary_operator_from_multiply_char() {
        let res_multiply: Result<BinaryOperator, String> = BinaryOperator::from_char('*');
        assert!(res_multiply.is_ok());
        assert_eq!(res_multiply.unwrap(), BinaryOperator::Multiply);
    }

    #[test]
    fn test_binary_operator_from_divide_char() {
        let res_divide: Result<BinaryOperator, String> = BinaryOperator::from_char('/');
        assert!(res_divide.is_ok());
        assert_eq!(res_divide.unwrap(), BinaryOperator::Divide);
    }

    #[test]
    fn test_binary_operator_from_power_char() {
        let res_power: Result<BinaryOperator, String> = BinaryOperator::from_char('^');
        assert!(res_power.is_ok());
        assert_eq!(res_power.unwrap(), BinaryOperator::Power);
    }

    #[test]
    fn test_binary_operator_from_unknown_char() {
        let res_unknown: Result<BinaryOperator, String> = BinaryOperator::from_char('!');
        assert!(res_unknown.is_err());
        assert_eq!(
            res_unknown.err(),
            Some(String::from("Unknown operator characters"))
        );
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
        assert_eq!(plus_ops.precedence(), 2);

        let minus_ops: BinaryOperator = BinaryOperator::Minus;
        assert_eq!(minus_ops.precedence(), 2);

        let multiply_ops: BinaryOperator = BinaryOperator::Multiply;
        assert_eq!(multiply_ops.precedence(), 3);

        let divide_ops: BinaryOperator = BinaryOperator::Divide;
        assert_eq!(divide_ops.precedence(), 3);

        let power_ops: BinaryOperator = BinaryOperator::Power;
        assert_eq!(power_ops.precedence(), 4);
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
        assert_eq!(
            ops_plus.apply(left_operand, right_operand).unwrap(),
            ref_plus
        );
    }

    #[test]
    fn test_binary_operator_apply_minus() {
        let left_operand: f64 = 5.0;
        let right_operand: f64 = 2.0;

        let ops_minus: BinaryOperator = BinaryOperator::Minus;
        let ref_minus: f64 = 3.0;
        assert_eq!(
            ops_minus.apply(left_operand, right_operand).unwrap(),
            ref_minus
        );
    }

    #[test]
    fn test_binary_operator_apply_multiply() {
        let left_operand: f64 = 5.0;
        let right_operand: f64 = 2.0;

        let ops_multiply: BinaryOperator = BinaryOperator::Multiply;
        let ref_multiply: f64 = 10.0;
        assert_eq!(
            ops_multiply.apply(left_operand, right_operand).unwrap(),
            ref_multiply
        );
    }

    #[test]
    fn test_binary_operator_apply_divide() {
        let left_operand: f64 = 6.0;
        let right_operand: f64 = 2.0;

        let ops_divide: BinaryOperator = BinaryOperator::Divide;
        let ref_divide: f64 = 3.0;
        assert_eq!(
            ops_divide.apply(left_operand, right_operand).unwrap(),
            ref_divide
        );
    }

    #[test]
    fn test_binary_operator_apply_divide_by_zero() {
        let left_operand: f64 = 5.0;
        let right_operand: f64 = 0.0;

        let ops_divide: BinaryOperator = BinaryOperator::Divide;
        let res_divide: Result<f64, String> = ops_divide.apply(left_operand, right_operand);

        assert!(res_divide.is_err());
        assert_eq!(res_divide.err(), Some(String::from("Division by zero")));
    }

    #[test]
    fn test_binary_operator_apply_power() {
        let left_operand: f64 = 5.0;
        let right_operand: f64 = 2.0;

        let ops_power: BinaryOperator = BinaryOperator::Power;
        let ref_power: f64 = 25.0;
        assert_eq!(
            ops_power.apply(left_operand, right_operand).unwrap(),
            ref_power
        );
    }

    #[test]
    fn test_unary_operator_from_plus_char() {
        let res_plus: Result<UnaryOperator, String> = UnaryOperator::from_char('+');
        assert!(res_plus.is_ok());
        assert_eq!(res_plus.unwrap(), UnaryOperator::Plus);
    }

    #[test]
    fn test_unary_operator_from_minus_char() {
        let res_minus: Result<UnaryOperator, String> = UnaryOperator::from_char('-');
        assert!(res_minus.is_ok());
        assert_eq!(res_minus.unwrap(), UnaryOperator::Minus);
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
}
