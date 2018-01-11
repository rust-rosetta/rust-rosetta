//! Utilities for interacting with tasks implemented in the rust-rosetta repository.

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use reqwest::Url;
use toml::Value;
use walkdir::WalkDir;

use TASK_URL_RE;
use remote;

/// A representation of a RosettaCode task.
///
/// The task may have an implementation in the GitHub repository (local), on the RosettaCode wiki
/// (remote), both, or neither.
#[derive(Debug, Clone)]
pub struct LocalTask {
    crate_name: String,
    path: PathBuf,
    source: Vec<PathBuf>,
    local_url: Result<Url, TaskParseError>,
}

impl LocalTask {
    /// The URL of the task on the RosettaCode wiki.
    ///
    /// If we are reading the task data from the wiki itself, the URL will always be available.
    ///
    /// If we are reading the task information solely from the repository, this information may be
    /// missing or malformed.
    pub fn url(&self) -> Result<Url, TaskParseError> {
        self.local_url.clone()
    }

    /// The title of the task on RosettaCode.
    ///
    /// This is the title used to identify the task on the RosettaCode wiki.
    pub fn title(&self) -> String {
        remote::decode_title(TASK_URL_RE
            .captures(self.url().unwrap().as_str())
            .and_then(|c| c.get(1))
            .map(|m| m.as_str())
            .expect(&format!(
                "Found task URL that does not match rosettacode regex: {}",
                self.url().unwrap()
            )))
    }

    /// The name of the crate that implements the task.
    pub fn crate_name(&self) -> String {
        self.crate_name.clone()
    }

    /// A list of paths pointing to the Rust source files of the task.
    pub fn source(&self) -> Vec<PathBuf> {
        self.source.clone()
    }

    /// The directory containing the task crate.
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

/// Errors that may occur while parsing a task crate.
#[derive(Debug, Clone)]
pub enum TaskParseError {
    /// The task URL was not a valid RosettaCode URL.
    InvalidURL(String),

    /// The task does not have any rosettacode metadata associated with it.
    MissingMetadata,
}

/// Given a path to the root `Cargo.toml`, returns a list of tasks implemented in the rust-rosetta
/// repository.
pub fn parse_tasks<P>(path: P) -> Vec<LocalTask>
where
    P: AsRef<Path>,
{
    let cargo_toml = parse_toml(path).unwrap();

    let members = {
        let workspace_table = &cargo_toml["workspace"];
        match workspace_table.get("members") {
            Some(&Value::Array(ref members)) => {
                members
                    .iter()
                    .map(|path| parse_task(path.as_str().unwrap()))
                    .collect()
            }
            _ => vec![],
        }
    };

    members
}

fn parse_toml<P>(path: P) -> io::Result<Value>
where
    P: AsRef<Path>,
{
    let mut toml_file = try!(File::open(path));
    let mut contents = String::new();
    try!(toml_file.read_to_string(&mut contents));
    Ok(contents.parse().unwrap())
}

fn parse_task<P>(path: P) -> LocalTask
where
    P: AsRef<Path>,
{
    let path = path.as_ref().canonicalize().unwrap();

    // Determine the path to the crate relative from the "tasks" folder. This path will serve as
    // the unique identifier of the task.
    let crate_name = path.components()
        .skip_while(|component| {
            String::from("tasks") != component.as_os_str().to_str().unwrap()
        })
        .filter_map(|component| {
            let component = component.as_os_str();

            if component.to_str().unwrap() == "tasks" {
                None
            } else {
                Some(Path::new(component))
            }
        })
        .collect::<PathBuf>()
        .to_str()
        .unwrap()
        .to_owned();

    let member_toml = parse_toml(path.join("Cargo.toml")).unwrap();

    let url = member_toml
        .get("package")
        .and_then(|p| p.get("metadata"))
        .and_then(|m| m.get("rosettacode"))
        .and_then(|m| m.get("url"))
        .ok_or(TaskParseError::MissingMetadata)
        .map(|metadata| metadata.as_str().unwrap())
        .and_then(|metadata| {
            Url::parse(metadata).or(Err(TaskParseError::InvalidURL(String::from(metadata))))
        });

    let mut sources = vec![];

    for entry in WalkDir::new(&path) {
        let entry = entry.unwrap();

        if let Some("rs") = entry.path().extension().and_then(|s| s.to_str()) {
            sources.push(entry.path().to_owned());
        }
    }

    LocalTask {
        crate_name: String::from(crate_name),
        path: path.to_owned(),
        source: sources,
        local_url: url,
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use reqwest::Url;

    #[test]
    fn parse_local_task() {
        let parsed_task = super::parse_task(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
            "../tasks/a-plus-b",
        ));

        assert_eq!(parsed_task.title(), "A+B");
        assert_eq!(
            parsed_task.url().unwrap(),
            Url::parse("http://rosettacode.org/wiki/A%2BB").unwrap()
        );
        assert_eq!(parsed_task.crate_name(), "a-plus-b");
    }

    #[test]
    fn parse_nested_local_task() {
        let parsed_task = super::parse_task(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
            "../tasks/rosetta-code/find-unimplemented-tasks",
        ));

        let name = if cfg!(windows) {
            "rosetta-code\\find-unimplemented-tasks"
        } else {
            "rosetta-code/find-unimplemented-tasks"
        };
        assert_eq!(parsed_task.crate_name(), name);
    }
}
