//! Library that contains utility functions for tests.
//!
//! It also contains a test module, which checks if all source files are covered by `Cargo.toml`

extern crate hyper;
extern crate regex;
extern crate rustc_serialize;

pub mod rosetta_code;

use std::fmt::Debug;

#[allow(dead_code)]
fn main() {}

/// Check if a slice is sorted properly.
pub fn check_sorted<E>(candidate: &[E])
    where E: Ord + Clone + Debug
{
    let sorted = {
        let mut copy = candidate.iter().cloned().collect::<Vec<_>>();
        copy.sort();
        copy
    };

    assert_eq!(sorted.as_slice(), candidate);
}

#[test]
fn test_check_sorted() {
    let sorted = vec![1, 2, 3, 4, 5];

    check_sorted(&sorted);
}

#[test]
#[should_panic]
fn test_check_unsorted() {
    let unsorted = vec![1, 3, 2];

    check_sorted(&unsorted);
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use std::collections::HashSet;
    use std::io::{BufReader, BufRead};
    use std::fs::{self, File};
    use std::path::Path;

    /// A test to check if all source files are covered by `Cargo.toml`
    #[test]
    fn check_sources_covered() {
        let sources = get_source_files();
        let bins = get_toml_paths();
        let not_covered = get_not_covered(&sources, &bins);

        if !not_covered.is_empty() {
            println!("Error, the following source files are not covered by Cargo.toml:");

            for source in &not_covered {
                println!("{}", source);
            }

            panic!("Please add the previous source files to Cargo.toml");
        }
    }

    /// Returns the names of the source files in the `src` directory
    fn get_source_files() -> HashSet<String> {
        let paths = fs::read_dir("./src").unwrap();
        paths.map(|p| {
                p.unwrap()
                    .path()
                    .file_name()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap()
            })
            .filter(|s| s[..].ends_with(".rs"))
            .collect()
    }

    /// Returns the paths of the source files referenced in Cargo.toml
    fn get_toml_paths() -> HashSet<String> {
        let c_toml = File::open("./Cargo.toml").unwrap();
        let reader = BufReader::new(c_toml);
        let regex = Regex::new("path = \"(.*)\"").unwrap();
        reader.lines()
            .filter_map(|l| {
                let l = l.unwrap();
                regex.captures(&l).map(|c| {
                    c.at(1)
                        .map(|s| Path::new(s))
                        .unwrap()
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .into_owned()
                })
            })
            .collect()
    }

    /// Returns the filenames of the source files which are not covered by Cargo.toml
    fn get_not_covered<'a>(sources: &'a HashSet<String>,
                           paths: &'a HashSet<String>)
                           -> HashSet<&'a String> {
        sources.difference(paths).collect()
    }
}
