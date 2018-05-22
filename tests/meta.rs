#![feature(fs_read_write)]

#[macro_use]
extern crate lazy_static;
extern crate meta;
extern crate toml;

use std::fs;
use std::path::{Path, PathBuf};

use meta::local;
use toml::Value;

lazy_static! {
    static ref MANIFEST_PATH: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
}

#[test]
fn tasks_are_sorted() {
    let toml: Value = fs::read_to_string(&*MANIFEST_PATH)
        .unwrap()
        .parse()
        .unwrap();
    let members = toml
        .get("workspace")
        .and_then(|w| w.get("members"))
        .and_then(|m| m.as_array())
        .unwrap()
        .iter()
        .flat_map(|m| m.as_str())
        .collect::<Vec<_>>();

    let mut sorted_members = members.clone();
    sorted_members.sort_unstable();

    for (member, sorted_member) in members.iter().zip(sorted_members) {
        if *member != sorted_member {
            panic!("{} is not in the correct order in `Cargo.toml`!", member);
        }
    }
}

#[test]
fn parse_local_tasks() {
    local::parse_tasks(&*MANIFEST_PATH).unwrap();
}
