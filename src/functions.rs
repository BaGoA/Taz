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
    #[allow(dead_code)]
    pub fn from_string(fun: &str) -> Result<Function, String> {
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
            _ => Err(String::from("Unknown function string")),
        }
    }

    /// Check if a string corresponds to function
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn apply(&self, arg: f64) -> Result<f64, String> {
        match self {
            Function::Abs => Ok(arg.abs()),
            Function::Sqrt => {
                if arg >= 0.0 {
                    return Ok(arg.sqrt());
                } else {
                    return Err(String::from("Argument of sqrt function is negative"));
                }
            }
            Function::Cbrt => Ok(arg.cbrt()),
            Function::Exp => Ok(arg.exp()),
            Function::Ln => {
                if arg > 0.0 {
                    return Ok(arg.ln());
                } else {
                    return Err(String::from("Argument of ln function is negative or null"));
                }
            }
            Function::Log10 => {
                if arg > 0.0 {
                    return Ok(arg.log10());
                } else {
                    return Err(String::from(
                        "Argument of log10 function is negative or null",
                    ));
                }
            }
            Function::Log2 => {
                if arg > 0.0 {
                    return Ok(arg.log2());
                } else {
                    return Err(String::from(
                        "Argument of log2 function is negative or null",
                    ));
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
                    return Err(String::from("Argument of tan function is not valid"));
                }
            }
            Function::Asin => {
                if -1.0 <= arg && arg <= 1.0 {
                    return Ok(arg.asin());
                } else {
                    return Err(String::from(
                        "Argument of asin function is not containing in [-1, 1]",
                    ));
                }
            }
            Function::Acos => {
                if -1.0 <= arg && arg <= 1.0 {
                    return Ok(arg.acos());
                } else {
                    return Err(String::from(
                        "Argument of acos function is not containing in [-1, 1]",
                    ));
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
        let res: Result<Function, String> = Function::from_string("abs");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Function::Abs);
    }

    #[test]
    fn test_function_from_sqrt_string() {
        let res: Result<Function, String> = Function::from_string("sqrt");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Function::Sqrt);
    }

    #[test]
    fn test_function_from_cbrt_string() {
        let res: Result<Function, String> = Function::from_string("cbrt");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Function::Cbrt);
    }

    #[test]
    fn test_function_from_exp_string() {
        let res: Result<Function, String> = Function::from_string("exp");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Function::Exp);
    }

    #[test]
    fn test_function_from_ln_string() {
        let res: Result<Function, String> = Function::from_string("ln");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Function::Ln);
    }

    #[test]
    fn test_function_from_log10_string() {
        let res: Result<Function, String> = Function::from_string("log10");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Function::Log10);
    }

    #[test]
    fn test_function_from_log2_string() {
        let res: Result<Function, String> = Function::from_string("log2");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Function::Log2);
    }

    #[test]
    fn test_function_from_sin_string() {
        let res: Result<Function, String> = Function::from_string("sin");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Function::Sin);
    }

    #[test]
    fn test_function_from_cos_string() {
        let res: Result<Function, String> = Function::from_string("cos");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Function::Cos);
    }

    #[test]
    fn test_function_from_tan_string() {
        let res: Result<Function, String> = Function::from_string("tan");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Function::Tan);
    }

    #[test]
    fn test_function_from_asin_string() {
        let res: Result<Function, String> = Function::from_string("asin");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Function::Asin);
    }

    #[test]
    fn test_function_from_acos_string() {
        let res: Result<Function, String> = Function::from_string("acos");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Function::Acos);
    }

    #[test]
    fn test_function_from_atan_string() {
        let res: Result<Function, String> = Function::from_string("atan");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Function::Atan);
    }

    #[test]
    fn test_function_from_sinh_string() {
        let res: Result<Function, String> = Function::from_string("sinh");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Function::Sinh);
    }

    #[test]
    fn test_function_from_cosh_string() {
        let res: Result<Function, String> = Function::from_string("cosh");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Function::Cosh);
    }

    #[test]
    fn test_function_from_tanh_string() {
        let res: Result<Function, String> = Function::from_string("tanh");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Function::Tanh);
    }

    #[test]
    fn test_function_from_asinh_string() {
        let res: Result<Function, String> = Function::from_string("asinh");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Function::Asinh);
    }

    #[test]
    fn test_function_from_acosh_string() {
        let res: Result<Function, String> = Function::from_string("acosh");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Function::Acosh);
    }

    #[test]
    fn test_function_from_atanh_string() {
        let res: Result<Function, String> = Function::from_string("atanh");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Function::Atanh);
    }

    #[test]
    fn test_function_from_unknown_string() {
        let res: Result<Function, String> = Function::from_string("toto");
        assert!(res.is_err());
        assert_eq!(res.err(), Some(String::from("Unknown function string")));
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

        let res: Result<f64, String> = fun.apply(-2.0);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 2.0);
    }

    #[test]
    fn test_function_apply_sqrt() {
        let fun: Function = Function::Sqrt;

        let res: Result<f64, String> = fun.apply(4.0);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 2.0);

        let res_in_err: Result<f64, String> = fun.apply(-4.0);
        assert!(res_in_err.is_err());
        assert_eq!(
            res_in_err.err(),
            Some(String::from("Argument of sqrt function is negative"))
        );
    }

    #[test]
    fn test_function_apply_cbrt() {
        let fun: Function = Function::Cbrt;

        let res: Result<f64, String> = fun.apply(-8.0);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), -2.0);
    }

    #[test]
    fn test_function_apply_exp() {
        let fun: Function = Function::Exp;

        let res: Result<f64, String> = fun.apply(0.0);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1.0);
    }

    #[test]
    fn test_function_apply_ln() {
        let fun: Function = Function::Ln;

        let res: Result<f64, String> = fun.apply(1.0);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 0.0);

        let res_in_err: Result<f64, String> = fun.apply(-4.0);
        assert!(res_in_err.is_err());
        assert_eq!(
            res_in_err.err(),
            Some(String::from("Argument of ln function is negative or null"))
        );
    }

    #[test]
    fn test_function_apply_log10() {
        let fun: Function = Function::Log10;

        let res: Result<f64, String> = fun.apply(10.0);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1.0);

        let res_in_err: Result<f64, String> = fun.apply(-4.0);
        assert!(res_in_err.is_err());
        assert_eq!(
            res_in_err.err(),
            Some(String::from(
                "Argument of log10 function is negative or null"
            ))
        );
    }

    #[test]
    fn test_function_apply_log2() {
        let fun: Function = Function::Log2;

        let res: Result<f64, String> = fun.apply(2.0);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1.0);

        let res_in_err: Result<f64, String> = fun.apply(-4.0);
        assert!(res_in_err.is_err());
        assert_eq!(
            res_in_err.err(),
            Some(String::from(
                "Argument of log2 function is negative or null"
            ))
        );
    }

    #[test]
    fn test_function_apply_sin() {
        let fun: Function = Function::Sin;

        let res: Result<f64, String> = fun.apply(0.0);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 0.0);
    }

    #[test]
    fn test_function_apply_cos() {
        let fun: Function = Function::Cos;

        let res: Result<f64, String> = fun.apply(0.0);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1.0);
    }

    #[test]
    fn test_function_apply_tan() {
        let fun: Function = Function::Tan;

        let res: Result<f64, String> = fun.apply(0.0);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 0.0);

        let res_in_err: Result<f64, String> = fun.apply(std::f64::consts::FRAC_PI_2);
        assert!(res_in_err.is_err());
        assert_eq!(
            res_in_err.err(),
            Some(String::from("Argument of tan function is not valid"))
        );
    }

    #[test]
    fn test_function_apply_asin() {
        let fun: Function = Function::Asin;

        let res: Result<f64, String> = fun.apply(0.0);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 0.0);

        let res_in_err: Result<f64, String> = fun.apply(1.8);
        assert!(res_in_err.is_err());
        assert_eq!(
            res_in_err.err(),
            Some(String::from(
                "Argument of asin function is not containing in [-1, 1]"
            ))
        );
    }

    #[test]
    fn test_function_apply_acos() {
        let fun: Function = Function::Acos;

        let res: Result<f64, String> = fun.apply(1.0);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 0.0);

        let res_in_err: Result<f64, String> = fun.apply(1.8);
        assert!(res_in_err.is_err());
        assert_eq!(
            res_in_err.err(),
            Some(String::from(
                "Argument of acos function is not containing in [-1, 1]"
            ))
        );
    }

    #[test]
    fn test_function_apply_atan() {
        let fun: Function = Function::Atan;

        let res: Result<f64, String> = fun.apply(0.0);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 0.0);
    }

    #[test]
    fn test_function_apply_sinh() {
        let fun: Function = Function::Sinh;

        let res: Result<f64, String> = fun.apply(0.0);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 0.0);
    }

    #[test]
    fn test_function_apply_cosh() {
        let fun: Function = Function::Cosh;

        let res: Result<f64, String> = fun.apply(0.0);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1.0);
    }

    #[test]
    fn test_function_apply_tanh() {
        let fun: Function = Function::Tanh;

        let res: Result<f64, String> = fun.apply(0.0);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 0.0);
    }

    #[test]
    fn test_function_apply_asinh() {
        let fun: Function = Function::Asinh;

        let res: Result<f64, String> = fun.apply(0.0);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 0.0);
    }

    #[test]
    fn test_function_apply_acosh() {
        let fun: Function = Function::Acosh;

        let res: Result<f64, String> = fun.apply(1.0);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 0.0);
    }

    #[test]
    fn test_function_apply_atanh() {
        let fun: Function = Function::Atanh;

        let res: Result<f64, String> = fun.apply(0.0);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 0.0);
    }
}
