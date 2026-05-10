pub fn invariants() {
    assert_eq!(
        Err(std::env::VarError::NotPresent),
        std::env::var("CARGO_TERM_COLOR")
    );
    assert_eq!(
        Ok("unchanged"),
        std::env::var("VARIABLE_TO_LEAVE_UNCHANGED").as_deref()
    );
}

#[test]
fn unit() {
    invariants();
}
