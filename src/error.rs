/// Enum to specify generic error for this library
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    // constants
    UnknownConstantString,

    // operators
    UnknownBinaryOperatorCharacter,
    UnknownUnaryOperatorCharacter,
    UnknownComparisonOperatorString,
    DivisionByZero,

    // functions
    UnknownFunctionString,
    ArgumentSqrtIsNegative,
    ArgumentLogIsNegativeOrNull,
    ArgumentTanIsInvalid,
    ArgumentASinIsInvalid,
    ArgumentACosIsInvalid,

    // infix
    CannotParseFloat(String),
    WordDoesNotMatchWithToken(String),
    CharacterDoesNotMatchWithToken(char),

    // posfix
    MismatchedParenthesis,

    // evaluator
    MissingLeftOperandForBinaryOperator,
    MissingRightOperandForBinaryOperator,
    MissingOperandForUnaryOperator,
    MissingArgumentForFunction,
    UnacceptableToken,
}

impl Error {
    pub fn message(self) -> String {
        match self {
            Error::UnknownConstantString => String::from("Unknown constant string"),
            Error::UnknownBinaryOperatorCharacter => {
                String::from("Unknown binary operator character")
            }
            Error::UnknownUnaryOperatorCharacter => {
                String::from("Unknown unary operator character")
            }
            Error::UnknownComparisonOperatorString => {
                String::from("Unknown comparison operator string")
            }
            Error::DivisionByZero => String::from("Division by zero"),
            Error::UnknownFunctionString => String::from("Unknown function string"),
            Error::ArgumentSqrtIsNegative => String::from("Argument of sqrt function is negative"),
            Error::ArgumentLogIsNegativeOrNull => {
                String::from("Argument of logarithm function is negative or null")
            }
            Error::ArgumentTanIsInvalid => {
                String::from("Argument of tangent function is a number equal to PI/2 + k*PI with k a relative integer")
            }
            Error::ArgumentASinIsInvalid => String::from("Argument of arc sinus is out of [-1, 1]"),
            Error::ArgumentACosIsInvalid => String::from("Argument of arc cosinus is out of [-1, 1]"),
            Error::CannotParseFloat(details) => format!("Cannot parse float: {details}"),
            Error::WordDoesNotMatchWithToken(word) => format!("The word {word} does not match with existing token"),
            Error::CharacterDoesNotMatchWithToken(c) => format!("The character {c} does not match with existing token"),
            Error::MismatchedParenthesis => String::from("Mismatched parenthesis"),
            Error::MissingLeftOperandForBinaryOperator => String::from("Missing left operand to apply binary operation"),
            Error::MissingRightOperandForBinaryOperator => String::from("Missing right operand to apply binary operation"),
            Error::MissingOperandForUnaryOperator => String::from("Missing operand to apply unary operation"),
            Error::MissingArgumentForFunction => String::from("Missing argument to apply function"),
            Error::UnacceptableToken => String::from("Token non-accepted for evaluation of postfix expression"),
        }
    }
}
