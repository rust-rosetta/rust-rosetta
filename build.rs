//! This build script runs automated checks across the repository and emits warnings if they are
//! violated.
//!
//! - All files have a maximum line length of 100.
//! - No trailing whitespace.
//! - All workspace members must be listed in `Cargo.toml` alphabetically.
//! - We are able to extract the needed metadata for task coverage.

#![feature(fs_read_write)]

extern crate failure;
extern crate meta;
extern crate toml;
extern crate unicode_segmentation;

use std::fs;
use std::path::Path;

use failure::Error;
use toml::Value;
use unicode_segmentation::UnicodeSegmentation;

use meta::local;

/// Verify that all workspace members are declared in `Cargo.toml` in lexicographic order.
fn verify_sort<P>(manifest_path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let toml: Value = fs::read_string(manifest_path)?.parse()?;
    let members = toml.get("workspace")
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
            println!(
                "cargo:warning={}",
                format!("{} is not in the correct order in `Cargo.toml`!", member)
            );
        }
    }

    Ok(())
}

/// Performs all checks on a particular file.
fn check_file<P>(path: &P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let content = fs::read_string(path)?;

    for (i, mut line) in content.lines().enumerate() {
        // Ignore '\r'
        if let Some('\r') = line.chars().rev().next() {
            line = &line[..line.len() - 1];
        }

        // Check length.
        if UnicodeSegmentation::graphemes(line, true).count() > 100 {
            line_error(i + 1, path, "line is longer than 100 characters");
        }

        // Check trailing whitespace.
        if let Some(last_char) = line.chars().rev().next() {
            if last_char.is_whitespace() {
                line_error(i + 1, path, "line has trailing whitespace");
            }
        }
    }

    Ok(())
}

fn line_error<P>(line: usize, path: &P, msg: &str)
where
    P: AsRef<Path>,
{
    println!(
        "cargo:warning={}",
        format!(
            "Formatting error: {} (line {} of file \"{}\")",
            msg,
            line,
            path.as_ref().to_str().unwrap()
        )
    );
}

fn run() -> Result<(), Error> {
    let manifest_path = "Cargo.toml";

    let tasks = local::parse_tasks(manifest_path)?;

    verify_sort(manifest_path)?;

    // Verify that all workspace members have the required metadata.
    for task in &tasks {
        for source in &task.source {
            check_file(source)?;
        }
    }

    Ok(())
}

fn main() {
    run().unwrap();
}
