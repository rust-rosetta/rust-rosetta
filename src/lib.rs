// Dummy main library
// It also contains a test module, which checks if all source files are covered by `Cargo.toml`
#![allow(unused_features)]
#![feature(plugin)]
#![feature(io)]
#![feature(collections)]
#![plugin(regex_macros)]
#![feature(old_path)]
#![feature(path)]

extern crate regex;

#[allow(dead_code)]
#[cfg(not(test))]
fn main() { }

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use std::io::{BufReader, BufReadExt};
    use std::fs::{self, File};
    // A test to check if all source files are covered by `Cargo.toml`
    #[test]
    fn check_sources_covered() {
        let sources = get_source_files();
        let bins = get_toml_paths();
        let not_covered = get_not_covered(&sources, &bins);

        if not_covered.len() > 0 {
            println!("Error, the following source files are not covered by Cargo.toml:");

            for source in not_covered.iter() {
                println!("{}", source);
            }

            panic!("Please add the previous source files to Cargo.toml");
        }
    }

    // Returns the names of the source files in the `src` directory
    fn get_source_files() -> HashSet<String> {
        let paths = fs::read_dir("./src").unwrap();
        paths.map(|p| p.unwrap().path().file_name().unwrap().to_os_string()
                    .into_string().unwrap())
                    .filter(|s| s[..].ends_with(".rs")).collect()
    }

    // Returns the paths of the source files referenced in Cargo.toml
    fn get_toml_paths() -> HashSet<String> {
        let c_toml = File::open(&Path::new("./Cargo.toml")).unwrap();
        let reader = BufReader::new(c_toml);
        let regex = regex!("path = \"(.*)\"");
        reader.lines().filter_map(|l| {
            let l = l.unwrap();
            regex.captures(&l).map(|c| c.at(1).map(|s| Path::new(s))
                                               .unwrap()
                                               .filename_str()
                                               .unwrap()
                                               .to_string())
        }).collect()
    }

    // Returns the filenames of the source files which are not covered by Cargo.toml
    fn get_not_covered<'a>(sources: &'a HashSet<String>, paths: &'a HashSet<String>) ->
                           HashSet<&'a String> {
        sources.difference(paths).collect()
    }
}
