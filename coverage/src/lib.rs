#![feature(plugin, custom_derive)]
#![plugin(serde_macros)]

#[macro_use]
extern crate lazy_static;

extern crate hyper;
extern crate regex;
extern crate rust_rosetta;
extern crate serde;
extern crate url;
extern crate walkdir;

use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::path::PathBuf;

use hyper::Client;
use hyper::header::Connection;
use regex::Regex;
use url::{percent_encoding, Url};
use walkdir::WalkDir;

use rust_rosetta::rosetta_code::find_unimplemented_tasks;

lazy_static!{
    static ref TASK_COMMENT_RE: Regex = Regex::new("// http://rosettacode.org/wiki/(.+)").unwrap();
    static ref LIB_OR_MOD: Regex = Regex::new("^(lib|mod)$").unwrap();

    /// Extracts code from the first Rust section from Rosetta Code wiki markup.
    static ref RUST_WIKI_SECTION_RE: Regex =
        Regex::new(r"==\{\{header\|Rust\}\}==(?s:.*?)<lang rust>((?s:.*?))</lang>").unwrap();

    /// The location of implemented tasks.
    ///
    /// Uses the `RUST_ROSETTA_SRC` environment variable if specified, otherwise uses `src` in the
    /// parent directory.
    static ref RUST_ROSETTA_SRC: PathBuf =
        PathBuf::from(option_env!("RUST_ROSETTA_SRC").unwrap_or("../src"));
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    /// The title of the task.
    pub title: String,

    /// The code of the task implemented locally. If the tasks is not implemented in the
    /// repository, then it will be `None`.
    pub local_code: Option<String>,

    /// The code of the task implemented on the Rosetta Code wiki. If the task is not yet
    /// implemented online, it will be `None`.
    pub remote_code: Option<String>,

    /// The URL of the task on the RosettaCode wiki.
    pub url: String, // FIXME: Should be a url::Url

    /// The relative path to where the task is in the repository, if implemented.
    pub path: Option<PathBuf>,
}

impl Task {
    /// True if and only if the task is only implemented on the Rosetta Code wiki.
    pub fn is_remote_only(&self) -> bool {
        self.local_code.is_none() && self.remote_code.is_some()
    }

    /// True if and only if the task is is only implemented in the repository.
    pub fn is_local_only(&self) -> bool {
        self.local_code.is_some() && self.remote_code.is_none()
    }

    /// True if and only if the task is neither implemented on the Rosetta code wiki or on the
    /// repository.
    pub fn is_unimplemented(&self) -> bool {
        self.local_code.is_none() && self.local_code.is_none()
    }
}

/// Transforms a task title to a URL task title.
fn normalize(title: &str) -> String {
    String::from_utf8(percent_encoding::percent_decode(
        &title.replace(" ", "_").into_bytes()).collect())
        .unwrap()
}

/// Returns the titles of every task on Rosetta Code.
fn all_task_titles() -> Vec<String> {
    find_unimplemented_tasks::all_tasks().iter().map(|task| task.title.to_owned()).collect()
}

/// Iterator over task information. One `Task` will be returned each iteration.
pub struct TaskIterator {
    task_titles: VecDeque<String>,
    local_tasks: HashMap<String, PathBuf>,
    client: Client,
}

impl TaskIterator {
    pub fn new(titles: &[String]) -> Self {
        // Determine which tasks are implemented locally by walking the src folder and reading the
        // comment at the top of the file.
        let mut local_tasks = HashMap::new();
        for entry in WalkDir::new(RUST_ROSETTA_SRC.as_path()) {
            let entry = entry.unwrap();
            let path = entry.path();
            let file_stem = path.file_stem().unwrap().to_str().unwrap();

            // If we find a non-Rust file (or a lib or mod) skip it.
            match path.extension().and_then(|s| s.to_str()) {
                Some("rs") if !LIB_OR_MOD.is_match(file_stem) => (),
                _ => continue,
            }

            let file = File::open(path).unwrap();
            let first_line = BufReader::new(file).lines().next().unwrap().unwrap();
            let task_name = TASK_COMMENT_RE.captures(&first_line)
                .and_then(|c| c.at(1))
                .expect(&format!("could not parse task name for {:?}", path));

            local_tasks.insert(normalize(&task_name.to_owned()), path.to_owned());
        }

        TaskIterator {
            client: Client::new(),
            task_titles: titles.iter().cloned().collect(),
            local_tasks: local_tasks,
        }
    }
}

impl Iterator for TaskIterator {
    type Item = Task;

    fn next(&mut self) -> Option<Self::Item> {
        self.task_titles.pop_front().map(|title| {
            let normalized_title = normalize(&title);

            let mut task_url = Url::parse(&format!("http://rosettacode.org/wiki/{}",
                                                   normalized_title))
                .unwrap();
            task_url.query_pairs_mut().append_pair("action", "raw");

            let path = self.local_tasks.remove(&normalized_title);
            let local_code = path.clone().and_then(|path| {
                File::open(path).ok().map(|mut file| {
                    let mut local_code = String::new();
                    file.read_to_string(&mut local_code).unwrap();
                    local_code
                })
            });

            let mut res = self.client
                .get(task_url.as_str())
                .header(Connection::close())
                .send()
                .unwrap();

            let mut body = String::new();
            res.read_to_string(&mut body).unwrap();
            let remote_code = RUST_WIKI_SECTION_RE.captures(&body)
                .map(|captures| captures.at(1).unwrap())
                .map(|code| code.to_owned());

            let mut wiki_url = task_url.clone();
            wiki_url.set_query(None);

            let relative_path = path.map(|path| {
                path.strip_prefix(RUST_ROSETTA_SRC.parent().unwrap()).unwrap().to_owned()
            });

            Task {
                title: title.to_owned(),
                local_code: local_code,
                remote_code: remote_code,
                url: wiki_url.into_string(),
                path: relative_path,
            }
        })
    }
}

/// Retrieves data for every task on Rosetta Code.
pub fn fetch_all_tasks() -> TaskIterator {
    fetch_tasks(&all_task_titles())
}

/// Parses both local (implemented in this repository) and remote (implemented on the wiki) tasks,
/// and returns the code of each.
pub fn fetch_tasks(tasks: &[String]) -> TaskIterator {
    let all_task_titles: HashSet<String> = HashSet::from_iter(all_task_titles());
    let requested_task_titles = HashSet::from_iter(tasks.iter().cloned());
    let mut task_titles: Vec<String> = all_task_titles.intersection(&requested_task_titles)
        .cloned()
        .collect();
    task_titles.sort();
    TaskIterator::new(&task_titles)
}
