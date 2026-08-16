use taz;

// Compute the relative error between a value and a reference
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

    match taz::evaluate(expression.as_str()) {
        Ok(result) => assert!(relative_error(result, reference) < 0.01),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_evaluation_expresion_with_numbers_operators() {
    let expression: String = String::from("-43.75 + 20.97");
    let reference: f64 = -43.75 + 20.97;

    match taz::evaluate(expression.as_str()) {
        Ok(result) => {
            println!("Evaluation = {}", result);
            println!("Reference = {}", reference);
            assert!(relative_error(result, reference) < 0.01);
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn test_evaluation_expression_with_numbers_operators_parenthesis() {
    let expression: String = String::from("43.75 + (-20.97 / 2.87) * 3.14");
    let reference: f64 = 43.75 + (-20.97 / 2.87) * 3.14;

    match taz::evaluate(expression.as_str()) {
        Ok(result) => assert!(relative_error(result, reference) < 0.01),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_evaluation_expression_with_function_and_number() {
    let expression: String = String::from("sqrt(9.0)");
    let reference: f64 = 3.0;

    match taz::evaluate(expression.as_str()) {
        Ok(result) => assert!(relative_error(result, reference) < 0.01),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_evaluation_expression_with_constant_and_number() {
    let expression: String = String::from("pi / 2.0");
    let reference: f64 = std::f64::consts::PI / 2.0;

    match taz::evaluate(expression.as_str()) {
        Ok(result) => assert!(relative_error(result, reference) < 0.01),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_evaluation_expression_with_all() {
    let expression: String = String::from("sin(2.0 - pi) * cos((-pi + 2.0) / 2.0)");
    let reference: f64 =
        (2.0 - std::f64::consts::PI).sin() * ((-std::f64::consts::PI + 2.0) / 2.0).cos();

    match taz::evaluate(expression.as_str()) {
        Ok(result) => assert!(relative_error(result, reference) < 0.01),
        Err(_) => assert!(false),
    }
}
