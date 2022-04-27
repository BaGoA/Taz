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

/// Available constants used in application
pub const PI: f64 = std::f64::consts::PI;
pub const E: f64 = std::f64::consts::E;
pub const C: f64 = 299792458.0;

/// Check if a string correspond to constant
#[allow(dead_code)]
pub fn is_constant(constant: &str) -> bool {
    match constant {
        "pi" => true,
        "e" => true,
        "c" => true,
        _ => false,
    }
}

/// Get constant value from a string
/// If string given in argument does not correspond to constants,
/// an error message is stored in string contained in Result output
#[allow(dead_code)]
pub fn from_string(constant: &str) -> Result<f64, String> {
    match constant {
        "pi" => Ok(PI),
        "e" => Ok(E),
        "c" => Ok(C),
        _ => Err(String::from("Unknown constant string")),
    }
}

// Units tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_from_pi_string() {
        let result: Result<f64, String> = from_string("pi");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PI);
    }

    #[test]
    fn test_constant_from_e_string() {
        let result: Result<f64, String> = from_string("e");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), E);
    }

    #[test]
    fn test_constant_from_c_string() {
        let result: Result<f64, String> = from_string("c");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), C);
    }

    #[test]
    fn test_constant_from_unknown_string() {
        let result: Result<f64, String> = from_string("toto");
        assert!(result.is_err());
    }

    #[test]
    fn test_constant_is_constant() {
        assert!(is_constant("pi"));
        assert!(is_constant("e"));
        assert!(is_constant("c"));
        assert!(!is_constant("toto"));
    }
}
