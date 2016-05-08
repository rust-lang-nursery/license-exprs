
use std::error::Error;
use std::fmt;

// TODO: distinguish between UnknownLicenseId and UnknownExceptionId
// after https://github.com/nikomatsakis/lalrpop/issues/113 is resolved

#[derive(Debug, Clone, Copy)]
pub enum SpdxParseError<'a> {
    UnknownLicenseId(&'a str),
    UnknownExceptionId(&'a str),
    InvalidStructure(&'a str),
}

impl<'a> fmt::Display for SpdxParseError<'a> {
    fn fmt(&self, format: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
          SpdxParseError::UnknownLicenseId(info)
              => format.write_fmt(format_args!("{}: {}", self.description(), info)),
          SpdxParseError::UnknownExceptionId(info)
              => format.write_fmt(format_args!("{}: {}", self.description(), info)),
          SpdxParseError::InvalidStructure(info)
              => format.write_fmt(format_args!("{}: {}", self.description(), info)),
        }
    }
}

impl<'a> Error for SpdxParseError<'a> {
    fn description(&self) -> &str {
        match *self {
            SpdxParseError::UnknownLicenseId(_) => "unknown license id",
            SpdxParseError::UnknownExceptionId(_) => "unknown exception id",
            SpdxParseError::InvalidStructure(_) => "invalid license expression",
        }
    }
}
