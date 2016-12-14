//! A crate for analyzing the contents of the [rust-rosetta] repository.
//!
//! [rust-rosetta]: https://github.com/Hoverbear/rust-rosetta

#![warn(missing_docs)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate url;

extern crate regex;
extern crate reqwest;
extern crate toml;
extern crate walkdir;

extern crate find_unimplemented_tasks;

// Used by the test_sort macro.
#[doc(hidden)]
pub extern crate rand;

use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::iter::FromIterator;
use std::ops::Sub;
use std::path::Path;
use std::process;

use regex::Regex;
use url::Url;

pub mod local;
pub mod remote;

#[macro_use]
pub mod test_utils;

use local::LocalTask;
use remote::RemoteTask;

lazy_static! {
    /// A Regex that matches valid RosettaCode URLs.
    static ref TASK_URL_RE: Regex =
        Regex::new(r"^http://rosettacode\.org/wiki/([^#]+)$").unwrap();
}

/// A representation of a RosettaCode task. Contains information about the implementation on both
/// the local repository and the wiki.
#[derive(Debug, Clone)]
pub struct Task {
    local: Option<LocalTask>,
    remote: RemoteTask,
}

impl Task {
    /// Returns the title of the task.
    pub fn title(&self) -> String {
        self.remote.title()
    }

    /// Returns the implementation of the task in the local repository, if it exists.
    pub fn local_code(&self) -> Option<String> {
        self.local.clone().map(|task| {
            // FIXME: Too simple for the multiple source file case.
            let mut code = String::new();

            for source in &task.source() {
                let mut file = File::open(source).unwrap();
                file.read_to_string(&mut code).unwrap();
            }

            code
        })
    }

    /// Returns the implementation of the task on the wiki, if it exists.
    pub fn remote_code(&self) -> Option<String> {
        self.remote.source()
    }

    /// True if and only if the task is only implemented on the Rosetta Code wiki.
    pub fn is_remote_only(&self) -> bool {
        self.local.is_none() && self.remote.source().is_some()
    }

    /// True if and only if the task is is only implemented in the repository.
    pub fn is_local_only(&self) -> bool {
        self.local.is_some() && self.remote.source().is_none()
    }

    /// True if and only if the task is neither implemented on the Rosetta Code wiki or on the
    /// repository.
    pub fn is_unimplemented(&self) -> bool {
        self.local.is_none() && self.remote.source().is_none()
    }

    /// Returns the URL of the task on the Rosetta Code wiki.
    pub fn url(&self) -> Url {
        self.remote.url()
    }
}

/// An iterator over tasks. Parses information from both the rust-rosetta repository and the
/// RosettaCode wiki.
pub struct TaskIterator {
    requested_task_titles: VecDeque<String>,
    local_tasks: Vec<LocalTask>,
}

impl TaskIterator {
    /// Creates a new iterator over the specified tasks. If no tasks are supplied, iterates over
    /// all tasks.
    pub fn new<P>(workspace_root: P, titles: &[String]) -> TaskIterator
        where P: AsRef<Path>
    {
        let local_tasks = local::parse_tasks(workspace_root.as_ref().join("Cargo.toml"));

        let all_task_titles = remote::all_task_titles().into_iter().collect::<HashSet<_>>();

        let all_decoded_task_titles = all_task_titles.iter()
            .map(|title| remote::decode_title(title))
            .collect::<HashSet<_>>();

        let all_local_task_titles =
            local_tasks.iter().map(|task| task.title()).collect::<HashSet<_>>();

        if !all_local_task_titles.is_subset(&all_decoded_task_titles) {
            // If there are tasks that can't be matched on the wiki, it's possible that they are
            // just draft tasks at the moment. Check them by seeing if the server response with
            // 404.
            let possible_bad_local_titles = all_local_task_titles.sub(&all_decoded_task_titles);

            let mut bad_titles = vec![];
            for title in &possible_bad_local_titles {
                let task_result = remote::request_task(title);

                if task_result.is_err() {
                    bad_titles.push(title);
                }
            }

            if !bad_titles.is_empty() {
                panic!("Could not match some local tasks to tasks on the wiki: {:?}",
                       bad_titles);
            }
        }

        let mut requested_task_titles: Vec<_> = if titles.is_empty() {
            all_task_titles.into_iter().collect()
        } else {
            all_task_titles.intersection(&HashSet::from_iter(titles.iter().cloned()))
                .cloned()
                .collect()
        };

        requested_task_titles.sort();

        TaskIterator {
            requested_task_titles: requested_task_titles.into_iter().collect(),
            local_tasks: local_tasks,
        }
    }
}

impl Iterator for TaskIterator {
    type Item = Task;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref title) = self.requested_task_titles.pop_front() {
            let local_task = self.local_tasks
                .iter()
                .cloned()
                .find(|task| task.title() == title.as_str());
            let remote_task = match remote::request_task(&title) {
                Ok(remote_task) => remote_task,
                Err(e) => {
                    writeln!(io::stderr(), "Error while requesting task: {}", e).unwrap();
                    process::exit(1);
                }
            };

            let task = Task {
                local: local_task.clone(),
                remote: remote_task.clone(),
            };

            Some(task)
        } else {
            None
        }
    }
}

/// Retrieves data for every task on Rosetta Code.
pub fn fetch_all_tasks<P>(workspace_root: P) -> TaskIterator
    where P: AsRef<Path>
{
    TaskIterator::new(workspace_root, &vec![])
}

/// Parses both local (implemented in this repository) and remote (implemented on the wiki) tasks,
/// and returns the code of each.
pub fn fetch_tasks<P>(workspace_root: P, tasks: &[String]) -> TaskIterator
    where P: AsRef<Path>
{
    TaskIterator::new(workspace_root, &tasks)
}
