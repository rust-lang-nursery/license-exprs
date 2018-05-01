#[macro_use]
extern crate failure;

use std::fmt;

mod spdx;

use self::LicenseExpr::*;

#[derive(Clone, Debug)]
pub enum LicenseExpr {
    License(String),
    Exception(String),
    And,
    Or,
    With,
}

impl fmt::Display for LicenseExpr {
    fn fmt(&self, format: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            With => format.write_str("WITH"),
            And => format.write_str("AND"),
            Or => format.write_str("OR"),
            License(ref info) | Exception(ref info) => format.write_str(info),
        }
    }
}

#[derive(Clone, Debug, Fail)]
pub enum ParseError {
    #[fail(display = "unknown license or other term: {}", _0)]
    UnknownLicenseId(String),
    #[fail(display = "invalid license expression: {}", _0)]
    InvalidStructure(LicenseExpr),
}

pub fn validate_license_expr(license_expr: &str) -> Result<(), ParseError> {
    license_expr
        .split_whitespace()
        .map(|word| match word {
            "AND" => Ok(And),
            "OR" => Ok(Or),
            "WITH" => Ok(With),
            _ if spdx::LICENSES
                .binary_search(&word.trim_right_matches('+'))
                .is_ok() =>
            {
                Ok(License(word.to_string()))
            }
            _ if spdx::EXCEPTIONS.binary_search(&word).is_ok() => Ok(Exception(word.to_string())),
            _ => Err(ParseError::UnknownLicenseId(word.to_string())),
        })
        .fold(Ok(Or), |prev, word| match (prev, word.clone()) {
            (err @ Err(_), _) | (_, err @ Err(_)) => err,
            (Ok(License(_)), Ok(With))
            | (Ok(License(_)), Ok(And))
            | (Ok(License(_)), Ok(Or))
            | (Ok(Exception(_)), Ok(And))
            | (Ok(Exception(_)), Ok(Or))
            | (Ok(And), Ok(License(_)))
            | (Ok(Or), Ok(License(_)))
            | (Ok(With), Ok(Exception(_))) => word,
            _ => Err(ParseError::InvalidStructure(word.unwrap())),
        })
        .and(Ok(()))
}

pub fn license_version() -> &'static str {
    spdx::VERSION
}

#[cfg(test)]
mod tests {
    use super::validate_license_expr;

    #[test]
    fn single_license() {
        assert!(validate_license_expr("MIT").is_ok());
    }

    #[test]
    fn compound_license() {
        assert!(
            validate_license_expr("GPL-3.0+ WITH Classpath-exception-2.0 OR MIT AND AAL").is_ok()
        );
    }

    #[test]
    fn fails_invalid_license() {
        assert!(validate_license_expr("asdfghjkl").is_err());
        assert!(validate_license_expr("MIT AND qwerty").is_err())
    }

    #[test]
    fn fails_incorrect_structure() {
        assert!(validate_license_expr("WITH").is_err());
        assert!(validate_license_expr("MIT OR WITH").is_err());
        assert!(validate_license_expr("MIT AND Classpath-exception-2.0").is_err());
        assert!(validate_license_expr("Classpath-exception-2.0").is_err());
    }
}
