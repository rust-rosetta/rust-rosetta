//! Utilities for interacting with tasks implemented on the RosettaCode wiki.

use std::io::prelude::*;

use regex::Regex;
use reqwest::{self, StatusCode};
use url::Url;
use url::percent_encoding::{self, QUERY_ENCODE_SET};

use find_unimplemented_tasks;

lazy_static!{
    /// Extracts code from the first Rust section from Rosetta Code wiki markup.
    static ref RUST_WIKI_SECTION_RE: Regex =
        Regex::new(r"==\{\{header\|Rust\}\}==(?s:.*?)<lang rust>((?s:.*?))</lang>").unwrap();
}

define_encode_set! {
    /// Encoding set used for Rosetta Code URLs.
    ///
    /// The wiki generally encodes characters found in the query encode set, as well as some
    /// additional characters.
    pub ROSETTA_ENCODE_SET = [QUERY_ENCODE_SET] | { '+' }
}

/// Represents a task implemented on the RosettaCode wiki.
#[derive(Debug, Clone)]
pub struct RemoteTask {
    title: String,
    url: Url,
    source: Option<String>,
}

impl RemoteTask {
    /// Returns the title of the task.
    pub fn title(&self) -> String {
        self.title.clone()
    }

    /// Returns the URL to the task on the wiki.
    pub fn url(&self) -> Url {
        self.url.clone()
    }

    /// If the task contains a Rust implementation on the wiki, this field will contain the first
    /// Rust code block of that section. Otherwise, returns `None`.
    pub fn source(&self) -> Option<String> {
        self.source.clone()
    }
}

/// Transforms a URL-encoded task title from the wiki to a human-readable task title.
///
/// # Examples
///
/// ```
/// use meta::remote::decode_title;
///
/// assert_eq!(decode_title("K-means%2B%2B_clustering"), "K-means++ clustering");
/// ```
pub fn decode_title(title: &str) -> String {
    let title = title.replace("_", " ").into_bytes();
    let decoded = percent_encoding::percent_decode(&title);
    String::from_utf8(decoded.collect()).unwrap()
}

/// Transforms a human-readable task title to the form used in a wiki URL.
///
/// # Examples
///
/// ```
/// use meta::remote::encode_title;
///
/// assert_eq!(encode_title("K-means++ clustering"), "K-means%2B%2B_clustering");
/// ```
pub fn encode_title(title: &str) -> String {
    let snake_case_title = title.replace(" ", "_").into_bytes();
    let encoded = percent_encoding::percent_encode(&snake_case_title, ROSETTA_ENCODE_SET);
    encoded.collect()
}

/// Returns the titles of every task on Rosetta Code.
pub fn all_task_titles() -> Vec<String> {
    find_unimplemented_tasks::all_tasks().iter().map(|task| task.title.to_owned()).collect()
}

/// Given a task title, pulls the task page from the RosettaCode wiki and parses its information.
pub fn request_task(title: &str) -> Result<RemoteTask, StatusCode> {
    let encoded_title = encode_title(title);

    let url = Url::parse(&format!("http://rosettacode.org/wiki/{}", encoded_title)).unwrap();

    let response = {
        let mut raw_url = url.clone();
        raw_url.query_pairs_mut().append_pair("action", "raw");

        let mut res = reqwest::get(raw_url.as_str()).unwrap();

        if !res.status().is_success() {
            return Err(res.status().to_owned());
        }

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        body
    };

    let rust_wiki_section = RUST_WIKI_SECTION_RE.captures(&response)
        .map(|captures| captures.at(1).unwrap())
        .map(String::from);

    let task = RemoteTask {
        title: title.to_owned(),
        url: url,
        source: rust_wiki_section,
    };

    Ok(task)
}

#[cfg(test)]
mod tests {
    use url::Url;

    #[ignore]
    #[test]
    fn parse_online() {
        let remote_task = super::request_task("Quine").unwrap();

        assert_eq!(remote_task.title, "Quine");
        assert_eq!(remote_task.url,
                   Url::parse("http://rosettacode.org/wiki/Quine").unwrap());
        assert!(remote_task.source.is_some());
    }

    #[test]
    fn encode_decode() {
        const TITLE: &'static str = "Penney's game";

        assert_eq!(super::decode_title(&super::encode_title(TITLE)), TITLE);
    }
}
