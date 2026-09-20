use crate::error::Error;

/// Available functions used in library
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Function {
    Abs,
    Sqrt,
    Cbrt,
    Exp,
    Ln,
    Log10,
    Log2,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Sinh,
    Cosh,
    Tanh,
    Asinh,
    Acosh,
    Atanh,
}

impl Function {
    /// Create a Function from a string
    /// If string given in argument does not correspond to function,
    /// an error message is stored in string contained in Result output
    pub fn from_string(fun: &str) -> Result<Function, Error> {
        match fun {
            "abs" => Ok(Function::Abs),
            "sqrt" => Ok(Function::Sqrt),
            "cbrt" => Ok(Function::Cbrt),
            "exp" => Ok(Function::Exp),
            "ln" => Ok(Function::Ln),
            "log10" => Ok(Function::Log10),
            "log2" => Ok(Function::Log2),
            "sin" => Ok(Function::Sin),
            "cos" => Ok(Function::Cos),
            "tan" => Ok(Function::Tan),
            "asin" => Ok(Function::Asin),
            "acos" => Ok(Function::Acos),
            "atan" => Ok(Function::Atan),
            "sinh" => Ok(Function::Sinh),
            "cosh" => Ok(Function::Cosh),
            "tanh" => Ok(Function::Tanh),
            "asinh" => Ok(Function::Asinh),
            "acosh" => Ok(Function::Acosh),
            "atanh" => Ok(Function::Atanh),
            _ => Err(Error::UnknownFunctionString),
        }
    }

    /// Check if a string corresponds to function
    pub fn is_fun(fun: &str) -> bool {
        match fun {
            "abs" => true,
            "sqrt" => true,
            "cbrt" => true,
            "exp" => true,
            "ln" => true,
            "log10" => true,
            "log2" => true,
            "sin" => true,
            "cos" => true,
            "tan" => true,
            "asin" => true,
            "acos" => true,
            "atan" => true,
            "sinh" => true,
            "cosh" => true,
            "tanh" => true,
            "asinh" => true,
            "acosh" => true,
            "atanh" => true,
            _ => false,
        }
    }

    /// Apply the function on value given in argument.
    /// For limits cases, we check that value is valid.
    /// To take into account this error, the function return a Result<f64, String>
    pub fn apply(&self, arg: f64) -> Result<f64, Error> {
        match self {
            Function::Abs => Ok(arg.abs()),
            Function::Sqrt => {
                if arg >= 0.0 {
                    return Ok(arg.sqrt());
                } else {
                    return Err(Error::ArgumentSqrtIsNegative);
                }
            }
            Function::Cbrt => Ok(arg.cbrt()),
            Function::Exp => Ok(arg.exp()),
            Function::Ln => {
                if arg > 0.0 {
                    return Ok(arg.ln());
                } else {
                    return Err(Error::ArgumentLogIsNegativeOrNull);
                }
            }
            Function::Log10 => {
                if arg > 0.0 {
                    return Ok(arg.log10());
                } else {
                    return Err(Error::ArgumentLogIsNegativeOrNull);
                }
            }
            Function::Log2 => {
                if arg > 0.0 {
                    return Ok(arg.log2());
                } else {
                    return Err(Error::ArgumentLogIsNegativeOrNull);
                }
            }
            Function::Sin => Ok(arg.sin()),
            Function::Cos => Ok(arg.cos()),
            Function::Tan => {
                // Check if arg is different that PI/2 + k*PI with k a relative integer
                let remainder: f64 = (arg - std::f64::consts::FRAC_PI_2) % std::f64::consts::PI;

                if remainder != 0.0 {
                    return Ok(arg.tan());
                } else {
                    return Err(Error::ArgumentTanIsInvalid);
                }
            }
            Function::Asin => {
                if -1.0 <= arg && arg <= 1.0 {
                    return Ok(arg.asin());
                } else {
                    return Err(Error::ArgumentASinIsInvalid);
                }
            }
            Function::Acos => {
                if -1.0 <= arg && arg <= 1.0 {
                    return Ok(arg.acos());
                } else {
                    return Err(Error::ArgumentACosIsInvalid);
                }
            }
            Function::Atan => Ok(arg.atan()),
            Function::Sinh => Ok(arg.sinh()),
            Function::Cosh => Ok(arg.cosh()),
            Function::Tanh => Ok(arg.tanh()),
            Function::Asinh => Ok(arg.asinh()),
            Function::Acosh => Ok(arg.acosh()),
            Function::Atanh => Ok(arg.atanh()),
        }
    }
}

// Units tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_from_abs_string() {
        match Function::from_string("abs") {
            Ok(fun) => assert_eq!(fun, Function::Abs),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_from_sqrt_string() {
        match Function::from_string("sqrt") {
            Ok(fun) => assert_eq!(fun, Function::Sqrt),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_from_cbrt_string() {
        match Function::from_string("cbrt") {
            Ok(fun) => assert_eq!(fun, Function::Cbrt),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_from_exp_string() {
        match Function::from_string("exp") {
            Ok(fun) => assert_eq!(fun, Function::Exp),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_from_ln_string() {
        match Function::from_string("ln") {
            Ok(fun) => assert_eq!(fun, Function::Ln),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_from_log10_string() {
        match Function::from_string("log10") {
            Ok(fun) => assert_eq!(fun, Function::Log10),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_from_log2_string() {
        match Function::from_string("log2") {
            Ok(fun) => assert_eq!(fun, Function::Log2),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_from_sin_string() {
        match Function::from_string("sin") {
            Ok(fun) => assert_eq!(fun, Function::Sin),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_from_cos_string() {
        match Function::from_string("cos") {
            Ok(fun) => assert_eq!(fun, Function::Cos),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_from_tan_string() {
        match Function::from_string("tan") {
            Ok(fun) => assert_eq!(fun, Function::Tan),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_from_asin_string() {
        match Function::from_string("asin") {
            Ok(fun) => assert_eq!(fun, Function::Asin),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_from_acos_string() {
        match Function::from_string("acos") {
            Ok(fun) => assert_eq!(fun, Function::Acos),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_from_atan_string() {
        match Function::from_string("atan") {
            Ok(fun) => assert_eq!(fun, Function::Atan),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_from_sinh_string() {
        match Function::from_string("sinh") {
            Ok(fun) => assert_eq!(fun, Function::Sinh),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_from_cosh_string() {
        match Function::from_string("cosh") {
            Ok(fun) => assert_eq!(fun, Function::Cosh),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_from_tanh_string() {
        match Function::from_string("tanh") {
            Ok(fun) => assert_eq!(fun, Function::Tanh),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_from_asinh_string() {
        match Function::from_string("asinh") {
            Ok(fun) => assert_eq!(fun, Function::Asinh),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_from_acosh_string() {
        match Function::from_string("acosh") {
            Ok(fun) => assert_eq!(fun, Function::Acosh),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_from_atanh_string() {
        match Function::from_string("atanh") {
            Ok(fun) => assert_eq!(fun, Function::Atanh),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_from_unknown_string() {
        match Function::from_string("toto") {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err, Error::UnknownFunctionString),
        }
    }

    #[test]
    fn test_function_is_fun() {
        assert!(Function::is_fun("abs"));
        assert!(Function::is_fun("sqrt"));
        assert!(Function::is_fun("cbrt"));
        assert!(Function::is_fun("exp"));
        assert!(Function::is_fun("ln"));
        assert!(Function::is_fun("log10"));
        assert!(Function::is_fun("log2"));
        assert!(Function::is_fun("sin"));
        assert!(Function::is_fun("cos"));
        assert!(Function::is_fun("tan"));
        assert!(Function::is_fun("asin"));
        assert!(Function::is_fun("acos"));
        assert!(Function::is_fun("atan"));
        assert!(Function::is_fun("sinh"));
        assert!(Function::is_fun("cosh"));
        assert!(Function::is_fun("tanh"));
        assert!(Function::is_fun("asinh"));
        assert!(Function::is_fun("acosh"));
        assert!(Function::is_fun("atanh"));
        assert!(!Function::is_fun("bunny"));
    }

    #[test]
    fn test_function_apply_abs() {
        let fun: Function = Function::Abs;

        match fun.apply(-2.0) {
            Ok(value) => assert_eq!(value, 2.0),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_apply_sqrt() {
        let fun: Function = Function::Sqrt;

        match fun.apply(4.0) {
            Ok(value) => assert_eq!(value, 2.0),
            Err(_) => assert!(false),
        }

        match fun.apply(-4.0) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err, Error::ArgumentSqrtIsNegative),
        }
    }

    #[test]
    fn test_function_apply_cbrt() {
        let fun: Function = Function::Cbrt;

        match fun.apply(-8.0) {
            Ok(value) => assert_eq!(value, -2.0),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_apply_exp() {
        let fun: Function = Function::Exp;

        match fun.apply(0.0) {
            Ok(value) => assert_eq!(value, 1.0),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_apply_ln() {
        let fun: Function = Function::Ln;

        match fun.apply(1.0) {
            Ok(value) => assert_eq!(value, 0.0),
            Err(_) => assert!(false),
        }

        match fun.apply(-4.0) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err, Error::ArgumentLogIsNegativeOrNull),
        }
    }

    #[test]
    fn test_function_apply_log10() {
        let fun: Function = Function::Log10;

        match fun.apply(10.0) {
            Ok(value) => assert_eq!(value, 1.0),
            Err(_) => assert!(false),
        }

        match fun.apply(-4.0) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err, Error::ArgumentLogIsNegativeOrNull),
        }
    }

    #[test]
    fn test_function_apply_log2() {
        let fun: Function = Function::Log2;

        match fun.apply(2.0) {
            Ok(value) => assert_eq!(value, 1.0),
            Err(_) => assert!(false),
        }

        match fun.apply(-4.0) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err, Error::ArgumentLogIsNegativeOrNull),
        }
    }

    #[test]
    fn test_function_apply_sin() {
        let fun: Function = Function::Sin;

        match fun.apply(0.0) {
            Ok(value) => assert_eq!(value, 0.0),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_apply_cos() {
        let fun: Function = Function::Cos;

        match fun.apply(0.0) {
            Ok(value) => assert_eq!(value, 1.0),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_apply_tan() {
        let fun: Function = Function::Tan;

        match fun.apply(0.0) {
            Ok(value) => assert_eq!(value, 0.0),
            Err(_) => assert!(false),
        }

        match fun.apply(std::f64::consts::FRAC_PI_2) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err, Error::ArgumentTanIsInvalid),
        }
    }

    #[test]
    fn test_function_apply_asin() {
        let fun: Function = Function::Asin;

        match fun.apply(0.0) {
            Ok(value) => assert_eq!(value, 0.0),
            Err(_) => assert!(false),
        }

        match fun.apply(1.8) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err, Error::ArgumentASinIsInvalid),
        }
    }

    #[test]
    fn test_function_apply_acos() {
        let fun: Function = Function::Acos;

        match fun.apply(1.0) {
            Ok(value) => assert_eq!(value, 0.0),
            Err(_) => assert!(false),
        }

        match fun.apply(1.8) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err, Error::ArgumentACosIsInvalid),
        }
    }

    #[test]
    fn test_function_apply_atan() {
        let fun: Function = Function::Atan;

        match fun.apply(0.0) {
            Ok(value) => assert_eq!(value, 0.0),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_apply_sinh() {
        let fun: Function = Function::Sinh;

        match fun.apply(0.0) {
            Ok(value) => assert_eq!(value, 0.0),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_apply_cosh() {
        let fun: Function = Function::Cosh;

        match fun.apply(0.0) {
            Ok(value) => assert_eq!(value, 1.0),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_apply_tanh() {
        let fun: Function = Function::Tanh;

        match fun.apply(0.0) {
            Ok(value) => assert_eq!(value, 0.0),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_apply_asinh() {
        let fun: Function = Function::Asinh;

        match fun.apply(0.0) {
            Ok(value) => assert_eq!(value, 0.0),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_apply_acosh() {
        let fun: Function = Function::Acosh;

        match fun.apply(1.0) {
            Ok(value) => assert_eq!(value, 0.0),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_function_apply_atanh() {
        let fun: Function = Function::Atanh;

        match fun.apply(0.0) {
            Ok(value) => assert_eq!(value, 0.0),
            Err(_) => assert!(false),
        }
    }
}
