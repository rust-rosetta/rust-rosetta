use std::process::Command;
// files in tests/ directory are integration tests
#[test]
// add test to verify we cannot generate a password < 30 characters
fn validate_password_length() {
    let output = Command::new("cargo")
        .args(&["run", "--release", "--quiet", "0"])
        .output()
        .expect("could not run cargo");
    let output = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        "Please provide a password length greater than or equal to 30",
        output.trim()
    );
}
