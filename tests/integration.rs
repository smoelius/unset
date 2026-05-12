use assert_cmd::assert::OutputAssertExt;
use std::process::Command;

#[test]
fn integration() {
    Command::new("cargo")
        .args([
            "test",
            "--config",
            &format!(
                "target.'cfg(all())'.runner = '{}'",
                env!("CARGO_BIN_EXE_unset"),
            ),
        ])
        .env("CARGO_TERM_COLOR", "always")
        .env("VARIABLE_TO_LEAVE_UNCHANGED", "unchanged")
        .current_dir("fixture")
        .assert()
        .success();
}

#[test]
fn integration_from_child_directory() {
    Command::new("cargo")
        .args([
            "test",
            "--config",
            &format!(
                "target.'cfg(all())'.runner = '{}'",
                env!("CARGO_BIN_EXE_unset"),
            ),
        ])
        .env("CARGO_TERM_COLOR", "always")
        .env("VARIABLE_TO_LEAVE_UNCHANGED", "unchanged")
        .current_dir("fixture/src")
        .assert()
        .success();
}
