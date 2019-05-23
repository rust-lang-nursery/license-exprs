mod lexer;
mod parser_types;
mod spdx;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate lalrpop_util;
extern crate failure;
extern crate regex;

lalrpop_mod!(pub parser);

type Result<T, E = failure::Error> = std::result::Result<T, E>;

pub fn parse_license_expr(license_expr: &str) -> Result<parser_types::Disjunction> {
    let lexer = lexer::Lexer::new(license_expr);
    parser::DisjunctionParser::new()
        .parse(lexer)
        .map_err(|_| failure::err_msg("foo"))
}

pub fn validate_license_expr(license_expr: &str) -> Result<()> {
    parse_license_expr(license_expr).map(|_| ())
}

pub fn license_version() -> &'static str {
    spdx::VERSION
}
