use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn validate_quine() {
    let contents = {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/main.rs");
        let mut code = File::open(path).unwrap();
        let mut contents = String::new();
        code.read_to_string(&mut contents).unwrap();
        contents
    };

    let output = Command::new("cargo")
        .args(&["run", "--release"])
        .output()
        .expect("failed to execute binary");

    let output = String::from_utf8_lossy(&output.stdout);

    assert_eq!(contents, output);
}
