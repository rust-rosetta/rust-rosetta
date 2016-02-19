//! This build script checks that all files in the `src` directory have lines of at most 100
//! characters and don't contain trailing whitespace.
//!
//! It also ensures that comment indicating which task a file solves is at the top.
//!
//! In case we find a line that doesn't comply with this rules, the build will fail and indicate
//! the cause of the problem.
extern crate regex;
extern crate toml;
extern crate unicode_segmentation;
extern crate walkdir;

use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;
use walkdir::WalkDir;

fn check_toml() {
    let mut cargo_toml_file = File::open("Cargo.toml").unwrap();
    let mut cargo_toml_string = String::new();
    cargo_toml_file.read_to_string(&mut cargo_toml_string).unwrap();

    let cargo_toml = toml::Parser::new(&cargo_toml_string).parse().unwrap();
    let binaries = cargo_toml.get("bin").unwrap().as_slice().unwrap().to_owned();
    let mut sorted_binaries = binaries.clone();
    sorted_binaries.sort_by(|a, b| {
        let a_name = a.as_table().unwrap().get("name").unwrap().as_str().unwrap();
        let b_name = b.as_table().unwrap().get("name").unwrap().as_str().unwrap();
        a_name.cmp(b_name)
    });

    for (bin, correct_bin) in binaries.iter().zip(sorted_binaries) {
        let bin_name = bin.as_table().unwrap().get("name").unwrap().as_str().unwrap();
        let correct_bin_name = correct_bin.as_table()
                                          .unwrap()
                                          .get("name")
                                          .unwrap()
                                          .as_str()
                                          .unwrap();
        if bin_name != correct_bin_name {
            panic!("{} is not in the correct order in Cargo.toml!", bin_name);
        }
    }
}

fn main() {
    check_toml();

    for entry in WalkDir::new("src") {
        let entry = entry.unwrap();
        if fs::metadata(&entry.path()).unwrap().is_file() {
            check(&entry.path());
        }
    }
}

fn check_task_name(path: &Path, line: &str) {
    // Ensure the first line has a URL of the proper form
    let task_comment = Regex::new(r"^// http://rosettacode\.org/wiki/[^#]+$").unwrap();
    if !task_comment.is_match(line) {
        line_error(1,
                   path,
                   "header is missing or malformed. The line should exactly match \
                   '// http://rosettacode.org/wiki/<TASK NAME>'");
    }
}

fn check(path: &Path) {
    let mut content = String::new();
    File::open(&path).unwrap().read_to_string(&mut content).unwrap();

    let lib_or_mod = Regex::new("^lib|mod$").unwrap();
    for (i, mut line) in content.lines().enumerate() {
        if i == 0 {
            match path.extension().and_then(|s| s.to_str()) {
                Some("rs") => {
                    // Ignore lib and mod files.
                    if !lib_or_mod.is_match(path.file_stem().unwrap().to_str().unwrap()) {
                        check_task_name(&path, &line);
                    }
                }
                _ => (),
            }
        }

        // Ignore '\r'
        if let Some('\r') = line.chars().rev().next() {
            line = &line[..line.len() - 1];
        }

        // Check length
        if UnicodeSegmentation::graphemes(line, true).count() > 100 {
            line_error(i + 1, path, "line is longer than 100 characters");
        }

        // Check trailing whitespace
        if let Some(last_char) = line.chars().rev().next() {
            if last_char.is_whitespace() {
                line_error(i + 1, path, "line has trailing whitespace");
            }
        }
    }
}

fn line_error(line: usize, path: &Path, msg: &str) {
    panic!("Formatting error, {} (line {} of file \"{}\")",
           msg,
           line,
           path.to_str().unwrap())
}
