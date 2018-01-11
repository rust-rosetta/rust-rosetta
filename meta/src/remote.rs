//! Utilities for interacting with tasks implemented on the RosettaCode wiki.

use std::collections::HashMap;

use percent_encoding::{self, QUERY_ENCODE_SET};
use regex::Regex;
use reqwest::Url;

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

#[derive(Debug, Deserialize)]
pub struct Response {
    pub query: Pages,
}

/// Intermediate container in the query API response.
#[derive(Debug, Deserialize)]
pub struct Pages {
    pub pages: HashMap<i32, Page>,
}

/// Individual page in the query API response.
#[derive(Debug, Deserialize)]
pub struct Page {
    #[serde(rename = "pageid")]
    pub id: i32,

    pub title: String,

    #[serde(default)]
    pub revisions: Vec<Revision>,
}

#[derive(Debug, Deserialize)]
pub struct Revision {
    #[serde(rename = "*")]
    pub content: String,
}

/// Transforms a URL-encoded task title from the wiki to a human-readable task title.
pub fn decode_title(title: &str) -> String {
    let title = title.replace("_", " ").into_bytes();
    let decoded = percent_encoding::percent_decode(&title);
    String::from_utf8(decoded.collect()).unwrap()
}

/// Transforms a human-readable task title to the form used in a wiki URL.
pub fn encode_title(title: &str) -> String {
    let snake_case_title = title.replace(" ", "_").into_bytes();
    let encoded = percent_encoding::percent_encode(&snake_case_title, ROSETTA_ENCODE_SET);
    encoded.collect()
}

pub fn parse_tasks(response: &Response) -> Vec<RemoteTask> {
    response
        .query
        .pages
        .values()
        .map(|page| {
            let source = RUST_WIKI_SECTION_RE
                .captures(&page.revisions[0].content)
                .map(|captures| captures.get(1).unwrap())
                .map(|m| m.as_str().to_owned());

            let title = page.title.clone();
            let url = Url::parse(&format!(
                "http://rosettacode.org/wiki/{}",
                encode_title(&title)
            )).unwrap();

            RemoteTask { title, url, source }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn encode() {
        use super::encode_title;
        assert_eq!(encode_title("K-means++ clustering"), "K-means%2B%2B_clustering");
    }

    #[test]
    fn decode() {
        use super::decode_title;
        assert_eq!(decode_title("K-means%2B%2B_clustering"), "K-means++ clustering");
    }

    #[test]
    fn encode_decode() {
        const TITLE: &'static str = "Penney's game";

        assert_eq!(super::decode_title(&super::encode_title(TITLE)), TITLE);
    }
}
