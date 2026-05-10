use assert_cmd::assert::OutputAssertExt;
use std::process::Command;

#[test]
fn cargo_runner_unsets_variables_from_unset_txt() {
    Command::new("cargo")
        .args([
            "test",
            "--config",
            &format!(
                "target.'cfg(all())'.runner = '{}'",
                env!("CARGO_BIN_EXE_unset"),
            ),
            "--",
            "--test-threads=1",
        ])
        .env("CARGO_TERM_COLOR", "always")
        .env("VARIABLE_TO_IGNORE", "unchanged")
        .current_dir("fixture")
        .assert()
        .success();
}
