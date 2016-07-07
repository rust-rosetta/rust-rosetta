//! A crate for analyzing the contents of the rust-rosetta repository.

#![warn(missing_docs)]

#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate toml;
extern crate walkdir;
extern crate url;

use regex::Regex;

mod local;

pub use local::*;

lazy_static! {
    /// A Regex that matches valid RosettaCode URLs.
    pub static ref TASK_URL_RE: Regex =
        Regex::new(r"^http://rosettacode\.org/wiki/[^#]+$").unwrap();
}
