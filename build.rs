//! This build script checks that all files in the `src` directory have lines of at most 100
//! characters and don't contain trailing whitespace.
//!
//! It also ensures that comment indicating which task a file solves is at the top.
//!
//! In case we find a line that doesn't comply with this rules, the build will fail and indicate
//! the cause of the problem.

extern crate meta;
extern crate unicode_segmentation;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use unicode_segmentation::UnicodeSegmentation;

use meta::local::{self, TaskParseError};

fn main() {
    let tasks = local::parse_tasks("Cargo.toml");

    // Verify that all workspace members are in lexicographic order.
    let sorted_tasks = {
        let mut sorted_tasks = tasks.clone();
        sorted_tasks.sort_by_key(|task| task.path());
        sorted_tasks
    };

    for (task, sorted_task) in tasks.iter().zip(sorted_tasks) {
        if task.path() != sorted_task.path() {
            println!("cargo:warning={}",
                     format!("{} is not in the correct order in `Cargo.toml`!",
                             task.path().to_str().unwrap()));
        }
    }

    // Verify that all workspace members have the required metadata.
    for task in &tasks {
        if let Err(ref err) = task.url() {
            match *err {
                TaskParseError::InvalidURL(ref url) => {
                    println!("cargo:warning={}",
                             format!("invalid URL '{}' found for task {}", url, task.crate_name()));
                }
                TaskParseError::MissingMetadata => {
                    println!("cargo:warning={}",
                             format!("missing URL metadata for {}. See CONTRIBUTING.md for \
                                      details.",
                                     task.crate_name()));
                }

            }
        }

        for source in &task.source() {
            check_file(source);
        }
    }
}

/// Performs all checks on a particular file.
fn check_file<P>(path: &P)
    where P: AsRef<Path>
{
    let mut content = String::new();
    File::open(&path).unwrap().read_to_string(&mut content).unwrap();

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
}

fn line_error<P>(line: usize, path: &P, msg: &str)
    where P: AsRef<Path>
{
    println!("cargo:warning={}",
             format!("Formatting error: {} (line {} of file \"{}\")",
                     msg,
                     line,
                     path.as_ref().to_str().unwrap()));
}
