extern crate lalrpop_util;
#[macro_use]
extern crate lazy_static;
extern crate regex;

pub mod constants;
pub mod errors;

mod spdx;

use errors::SpdxParseError;
use lalrpop_util::ParseError;

pub fn validate_license_expr(license_expr: &str) -> Result<(), SpdxParseError> {
  match spdx::parse_Expr(license_expr) {
    Ok(_) => Ok(()),
    // TODO: distinguish between UnknownLicenseId and UnknownExceptionId
    // after https://github.com/nikomatsakis/lalrpop/issues/113 is resolved
    Err(ParseError::User {error: id}) =>
      Err(SpdxParseError::UnknownLicenseId(id)),
    Err(_) =>
      Err(SpdxParseError::InvalidStructure(license_expr))
  }
}
