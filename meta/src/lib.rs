//! A crate for analyzing the contents of the [rust-rosetta] repository.
//!
//! [rust-rosetta]: https://github.com/Hoverbear/rust-rosetta

#![warn(missing_docs)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate percent_encoding;
#[macro_use]
extern crate serde_derive;

extern crate regex;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate toml;
extern crate walkdir;

extern crate find_unimplemented_tasks;

// Used by the test_sort macro.
#[doc(hidden)]
pub extern crate rand;

mod remote;

use std::cmp;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::ops::Sub;
use std::path::Path;

use regex::Regex;
use reqwest::{Client, Url};

pub mod errors;
pub mod local;

#[macro_use]
pub mod test_utils;

use local::LocalTask;
use remote::{RemoteTask, Response};

pub use errors::{Error, Result};

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

/// The index of all tasks implemented locally and remotely.
pub struct TaskIndex {
    all_task_titles: HashSet<String>,
    local_tasks: Vec<LocalTask>,
    client: Client,
}

impl TaskIndex {
    /// Generate the task index by parsing tasks in the repository and requesting task titles from
    /// the wiki.
    pub fn create<P: AsRef<Path>>(workspace_root: P) -> Result<TaskIndex> {
        let local_tasks = local::parse_tasks(workspace_root.as_ref().join("Cargo.toml"));
        let local_task_titles = local_tasks.iter().map(LocalTask::title).collect();

        let client = Client::new();
        let all_task_titles = TaskIndex::all_task_titles(&client, local_task_titles)?;

        Ok(TaskIndex {
            all_task_titles,
            local_tasks,
            client,
        })
    }

    /// Retrieves data for every task on Rosetta Code.
    pub fn fetch_all_tasks(&self) -> TaskIterator {
        Self::fetch_tasks(&self, &[])
    }

    /// Parses both local (implemented in this repository) and remote (implemented on the wiki) tasks,
    /// and returns the code of each.
    pub fn fetch_tasks(&self, tasks: &[String]) -> TaskIterator {
        TaskIterator::new(&self, tasks)
    }

    fn all_task_titles(
        client: &Client,
        local_task_titles: HashSet<String>,
    ) -> Result<HashSet<String>> {
        let mut all_task_titles = find_unimplemented_tasks::all_tasks()
            .into_iter()
            .map(|task| task.title)
            .collect::<HashSet<_>>();

        let all_decoded_task_titles = all_task_titles
            .iter()
            .map(|title| remote::decode_title(title))
            .collect::<HashSet<_>>();

        if !local_task_titles.is_subset(&all_decoded_task_titles) {
            // If there are tasks that can't be matched on the wiki, it's possible that they are
            // just draft tasks at the moment. Perform a query to see if we get any missing pages.
            let mut possible_draft_titles = (local_task_titles.sub(&all_decoded_task_titles))
                .into_iter()
                .collect::<Vec<_>>();
            possible_draft_titles.sort();

            // FIXME: We might have to loop here if we get too many draft tasks.
            let mut request = Url::parse("http://rosettacode.org/mw/api.php").unwrap();
            request
                .query_pairs_mut()
                .append_pair("action", "query")
                .append_pair("format", "json")
                .append_pair("prop", "info")
                .append_pair("titles", &possible_draft_titles.join("|"));

            let response: Response = client.get(request.as_str()).send()?.json()?;

            // In the response, any missing pages will have a negative ID.
            let bad_titles = response
                .query
                .pages
                .into_iter()
                .flat_map(|(id, page)| if id < 0 { Some(page.title) } else { None })
                .collect::<Vec<_>>();

            if !bad_titles.is_empty() {
                panic!(
                    "Could not match some local tasks to tasks on the wiki: {:?}",
                    bad_titles
                );
            }

            all_task_titles.extend(possible_draft_titles);
        }

        Ok(all_task_titles)
    }
}

/// An iterator over tasks. Parses information from both the rust-rosetta repository and the
/// RosettaCode wiki.
pub struct TaskIterator<'a> {
    client: &'a Client,
    fetched_remote_tasks: VecDeque<RemoteTask>,
    requested_task_titles: VecDeque<String>,
    local_tasks: Vec<LocalTask>,
}

impl<'a> TaskIterator<'a> {
    /// Creates a new iterator over the specified tasks. If no tasks are supplied, iterates over
    /// all tasks.
    fn new(task_index: &'a TaskIndex, titles: &[String]) -> TaskIterator<'a> {
        let mut requested_task_titles: Vec<_> = if titles.is_empty() {
            task_index.all_task_titles.iter().cloned().collect()
        } else {
            task_index
                .all_task_titles
                .intersection(&HashSet::from_iter(titles.iter().cloned()))
                .cloned()
                .collect()
        };

        requested_task_titles.sort();

        TaskIterator {
            client: &task_index.client,
            fetched_remote_tasks: Default::default(),
            requested_task_titles: requested_task_titles.into_iter().collect(),
            local_tasks: task_index.local_tasks.clone(),
        }
    }

    /// Fetch the next set of remote tasks.
    fn fetch_task_batch(&mut self) -> Result<()> {
        let end = cmp::min(self.requested_task_titles.len(), 50);
        let next_batch_titles = self.requested_task_titles.drain(..end).collect::<Vec<_>>();
        let mut request = Url::parse("http://rosettacode.org/mw/api.php").unwrap();
        request
            .query_pairs_mut()
            .append_pair("action", "query")
            .append_pair("format", "json")
            .append_pair("prop", "revisions")
            .append_pair("rvprop", "content")
            .append_pair("titles", &next_batch_titles.join("|"));

        let response: Response = self.client.get(request.as_str()).send()?.json()?;

        let mut remote_tasks = remote::parse_tasks(&response);
        remote_tasks.sort_by_key(|task| task.title());
        self.fetched_remote_tasks = remote_tasks.into_iter().collect();

        Ok(())
    }
}

impl<'a> Iterator for TaskIterator<'a> {
    type Item = Result<Task>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.requested_task_titles.is_empty() && self.fetched_remote_tasks.is_empty() {
            return None;
        }

        if self.fetched_remote_tasks.is_empty() {
            if let Err(err) = self.fetch_task_batch() {
                return Some(Err(err));
            }
        }

        let remote_task = self.fetched_remote_tasks.pop_front().unwrap();
        let local_task = self.local_tasks.iter().cloned().find(|task| {
            task.title() == remote_task.title()
        });

        let task = Task {
            local: local_task,
            remote: remote_task,
        };

        Some(Ok(task))
    }
}
