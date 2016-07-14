extern crate meta;

use std::path::PathBuf;
use std::process::Command;

/// Calls `cargo test` in every workspace member.
///
/// FIXME: This test should be removed once `cargo` has a way to run tests for all crates in a
/// workspace.
#[test]
fn test_all_workspace_members() {
    for task in meta::local::parse_tasks("Cargo.toml") {
        let status = Command::new("cargo")
            .current_dir(PathBuf::from("tasks").join(&task.crate_name()))
            .arg("test")
            .status()
            .unwrap();

        assert!(status.success());
    }
}
