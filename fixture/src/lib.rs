pub fn assert_unset_ran() {
    assert_eq!(
        Err(std::env::VarError::NotPresent),
        std::env::var("CARGO_TERM_COLOR")
    );
    assert_eq!(
        Ok("unchanged"),
        std::env::var("VARIABLE_TO_IGNORE").as_deref()
    );
}

#[test]
fn unit_test_sees_unset_environment() {
    assert_unset_ran();
}
