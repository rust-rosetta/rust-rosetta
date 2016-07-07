use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use toml::{self, Table, Value};
use url::Url;
use walkdir::WalkDir;

/// A representation of a RosettaCode task.
///
/// The task may have an implementation in the GitHub repository (local), on the RosettaCode wiki
/// (remote), both, or neither.
#[derive(Clone, Debug)]
pub struct LocalTask {
    name: String,
    path: PathBuf,
    source: Vec<PathBuf>,
    url: Result<Url, TaskParseError>,
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

    /// The name of the task on RosettaCode.
    pub fn name(&self) -> String {
        self.name.clone()
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
#[derive(Clone, Debug)]
pub enum TaskParseError {
    /// The task URL was not a valid RosettaCode URL.
    InvalidURL(String),

    /// The task does not have any rosettacode metadata associated with it.
    MissingMetadata,
}

/// Given a path to the root `Cargo.toml`, returns a list of tasks implemented in the rust-rosetta
/// repository.
pub fn parse_tasks<P>(path: P) -> Vec<LocalTask>
    where P: AsRef<Path>
{
    let cargo_toml = parse_toml(path).unwrap();

    let members = {
        let workspace_table = cargo_toml.get("workspace").unwrap();
        match workspace_table.lookup("members") {
            Some(&Value::Array(ref members)) => {
                members.iter()
                    .flat_map(|path| {
                        let path = path.as_str().unwrap();

                        if !path.starts_with("tasks") {
                            None
                        } else {
                            Some(parse_task(path))
                        }
                    })
                    .collect()
            }
            _ => vec![],
        }
    };

    members
}

fn parse_toml<P>(path: P) -> io::Result<Table>
    where P: AsRef<Path>
{
    let mut toml_file = try!(File::open(path));
    let mut contents = String::new();
    try!(toml_file.read_to_string(&mut contents));
    Ok(toml::Parser::new(&contents).parse().unwrap())
}

fn parse_task<P>(path: P) -> LocalTask
    where P: AsRef<Path>
{
    let path = path.as_ref();

    let name = path.to_str().unwrap().trim_left_matches("tasks/");
    let member_toml = Value::Table(parse_toml(path.join("Cargo.toml")).unwrap());

    let url = member_toml.lookup("package.metadata.rosettacode.url")
        .ok_or(TaskParseError::MissingMetadata)
        .map(|metadata| metadata.as_str().unwrap())
        .and_then(|metadata| {
            Url::parse(metadata).or(Err(TaskParseError::InvalidURL(String::from(metadata))))
        });

    let mut sources = vec![];

    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();

        if let Some("rs") = entry.path().extension().and_then(|s| s.to_str()) {
            sources.push(entry.path().to_owned());
        }
    }

    LocalTask {
        name: String::from(name),
        path: path.to_owned(),
        source: sources,
        local_url: url,
    }
}
