//! Utilities for interacting with tasks implemented in the rust-rosetta repository.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Error};
use reqwest::Url;
use toml::Value;
use walkdir::WalkDir;

use crate::remote;
use crate::TASK_URL_RE;

/// A local (in repository) implementation of a Rosetta Code task.
#[derive(Debug, Clone)]
pub struct LocalTask {
    /// The name of the package implementing this task.
    pub package_name: String,

    /// The path to the crate manifest for the task.
    pub manifest_path: PathBuf,

    /// A set of filenames containing the Rust source code of the tasks.
    pub source: HashSet<PathBuf>,

    /// The URL of the task on the Rosetta Code wiki.
    ///
    /// This is parsed from the crate metadata, so it may not point to an actual Rosetta Code URL.
    pub url: Url,

    /// The title of the task on Rosetta Code.
    ///
    /// The title is normalized, so it can be used to uniquely identify the task on the wiki.
    pub title: String,
}

/// Check if the target of a package is of kind dylib or proc-macro.
fn is_dylib_or_proc_macro(target: &cargo_metadata::Target) -> bool {
    target.kind.contains(&String::from("dylib"))
        || target.kind.contains(&String::from("proc-macro"))
}

/// Given a path to the root `Cargo.toml`, returns a list of tasks implemented in the rust-rosetta
/// repository.
pub fn parse_tasks<P>(manifest_path: P) -> Result<Vec<LocalTask>, Error>
where
    P: AsRef<Path>,
{
    let metadata = cargo_metadata::metadata(Some(manifest_path.as_ref())).unwrap();
    let packages = &metadata.packages;

    let mut tasks = vec![];

    for member in &metadata.workspace_members {
        // Skip if we encounter known non-task crates.
        if member.name() == "rust-rosetta" || member.name() == "meta" {
            continue;
        }

        let package = packages.iter().find(|p| p.name == member.name()).unwrap();

        // If the package has a proc-macro or dylib target, it's probably just a dependency of
        // another task. Skip it.
        if package.targets.iter().any(is_dylib_or_proc_macro) {
            continue;
        }

        let manifest_path = Path::new(&package.manifest_path);

        let rosetta_url = parse_rosetta_url(manifest_path).context(format!(
            "could not parse rosetta code URL from {}",
            manifest_path.display()
        ))?;

        let title = {
            let caps = TASK_URL_RE.captures(rosetta_url.as_str()).ok_or_else(|| {
                anyhow!(
                    "task URL does not match rosetta code regex: {}",
                    rosetta_url
                )
            })?;
            remote::decode_title(&caps[1])
        };

        tasks.push(LocalTask {
            package_name: member.name().to_owned(),
            manifest_path: manifest_path.to_owned(),
            source: find_sources(manifest_path.parent().unwrap())?,
            url: rosetta_url,
            title,
        });
    }

    Ok(tasks)
}

/// Parses the Rosetta Code URL from the package metadata in a task's crate manifest.
fn parse_rosetta_url<P>(manifest_path: P) -> Result<Url, Error>
where
    P: AsRef<Path>,
{
    let manifest: Value = fs::read_to_string(manifest_path)?.parse()?;

    let url = manifest
        .get("package")
        .and_then(|p| p.get("metadata"))
        .and_then(|m| m.get("rosettacode"))
        .and_then(|m| m.get("url"))
        .and_then(|u| u.as_str())
        .ok_or_else(|| anyhow!("unexpected metadata format"))?;

    Ok(Url::parse(url)?)
}

/// Collect the paths to the source files of a given package.
fn find_sources<P>(directory: P) -> Result<HashSet<PathBuf>, Error>
where
    P: AsRef<Path>,
{
    let mut sources = HashSet::new();

    for entry in WalkDir::new(directory) {
        let entry = entry?;

        if let Some("rs") = entry.path().extension().and_then(|s| s.to_str()) {
            sources.insert(entry.path().to_owned());
        }
    }

    Ok(sources)
}
