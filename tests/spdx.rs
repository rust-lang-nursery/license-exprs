extern crate license_exprs;

use license_exprs::validate_license_expr;

#[test]
fn valid_license_exprs() {
    assert!(validate_license_expr("MIT").is_ok());
    assert!(validate_license_expr("(MIT)").is_ok());
    assert!(validate_license_expr("MIT OR MIT").is_ok());
    assert!(validate_license_expr("MIT AND MIT").is_ok());
    assert!(validate_license_expr("MIT WITH Autoconf-exception-2.0").is_ok());
    assert!(validate_license_expr("DocumentRef-foo:LicenseRef-bar").is_ok());
    assert!(validate_license_expr("LicenseRef-foo.BAR-10").is_ok());
}

#[test]
fn invalid_license_exprs() {
    assert!(validate_license_expr("Ph'nglui mglw'nafh Cthulhu R'lyeh wgah'nagl fhtagn").is_err());
    assert!(validate_license_expr("(MIT").is_err());
    assert!(validate_license_expr("LicenseRef-").is_err());
    assert!(validate_license_expr("DocumentRef-LicenseRef-foo").is_err());
    assert!(validate_license_expr("DocumentRef-LicenseRef:-foo").is_err());
}
