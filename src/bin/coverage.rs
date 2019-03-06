#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate clap;
extern crate difference;
extern crate meta;
extern crate serde;
extern crate term;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

use difference::{Changeset, Difference};
use structopt::StructOpt;
use term::Terminal;

use meta::{Task, TaskIndex};

/// Prints a colored diff of two strings to the terminal.
fn print_diff<T: ?Sized>(t: &mut T, s1: &str, s2: &str) -> io::Result<()>
where
    T: Terminal,
{
    let changeset = Changeset::new(s1, s2, "\n");

    for change in changeset.diffs {
        match change {
            Difference::Same(ref x) => {
                t.reset()?;
                writeln!(t, " {}", x)?;
            }
            Difference::Add(ref x) => {
                t.fg(term::color::GREEN)?;
                for line in x.split('\n') {
                    writeln!(t, "+{}", line)?;
                }
            }
            Difference::Rem(ref x) => {
                t.fg(term::color::RED)?;
                for line in x.split('\n') {
                    writeln!(t, "-{}", line)?;
                }
            }
        }
    }
    t.reset()?;
    t.flush()?;
    Ok(())
}

/// Prints a task in a human-readable format.
fn print_task<T: ?Sized>(t: &mut T, task: &Task, diff: bool) -> io::Result<()>
where
    T: Terminal,
{
    t.attr(term::Attr::Bold)?;
    writeln!(t, "{}", task.title())?;
    t.reset()?;

    write!(t, "Local:")?;
    write_status(t, task.local_code().is_some())?;

    write!(t, "Remote:")?;
    write_status(t, task.remote_code().is_some())?;
    writeln!(t)?;

    if let (Some(ref local_code), Some(ref remote_code)) = (task.local_code(), task.remote_code()) {
        if diff {
            print_diff(t, remote_code, local_code)?;
        }
    }

    Ok(())
}

/// Writes a boolean as a pretty, human-readable string.
fn write_status<T: ?Sized>(t: &mut T, boolean: bool) -> io::Result<()>
where
    T: Terminal,
{
    t.attr(term::Attr::Bold)?;

    if boolean {
        t.fg(term::color::GREEN)?;
        write!(t, " ✔ ")?
    } else {
        t.fg(term::color::RED)?;
        write!(t, " ✘ ")?;
    }

    t.reset()?;
    Ok(())
}

arg_enum! {
    #[derive(Debug)]
    enum Filter {
        All,
        LocalOnly,
        RemoteOnly,
        Unimplemented
    }
}

impl Default for Filter {
    fn default() -> Self {
        Filter::All
    }
}

/// Query differences between the rust-rosetta repository and the Rosetta Code wiki.
///
/// This script prints out the name of each task, followed by whether it is implemented online,
/// locally, or both.
///
/// If no tasks are specified, determines the status for all tasks.
#[derive(Debug, StructOpt)]
struct Opt {
    /// The name of a task on the wiki, such as "K-d tree"
    #[structopt(name = "task")]
    tasks: Vec<String>,

    /// Print diffs of tasks between the local and remote version
    #[structopt(long = "diff")]
    diff: bool,

    /// Filter tasks printed by the program
    #[structopt(
        long = "filter",
        raw(possible_values = "&Filter::variants()", case_insensitive = "true"),
        default_value = "all"
    )]
    filter: Filter,

    /// Dump JSON to the provided filename
    #[structopt(long = "json", parse(from_os_str))]
    json_file: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();

    let mut t = term::stdout().unwrap();

    let task_index = TaskIndex::create(env!("CARGO_MANIFEST_DIR")).unwrap();

    let tasks = if !opt.tasks.is_empty() {
        task_index.fetch_tasks(&opt.tasks)
    } else {
        task_index.fetch_all_tasks()
    };

    let tasks = tasks
        .flat_map(|task| {
            let task = task.unwrap();

            match opt.filter {
                Filter::LocalOnly if !task.is_local_only() => return None,
                Filter::RemoteOnly if !task.is_remote_only() => return None,
                Filter::Unimplemented if !task.is_unimplemented() => return None,
                Filter::All | _ => {}
            }

            print_task(&mut *t, &task, opt.diff).unwrap();

            if opt.json_file.is_some() {
                let json = json!({
                    "title": task.title(),
                    "url": task.url().to_string(),
                    "local_code": task.local_code(),
                    "remote_code": task.remote_code(),
                    "path": task.local_path(),
                });

                Some(json)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if let Some(filename) = opt.json_file {
        let mut file = File::create(filename).unwrap();
        file.write_all(serde_json::to_string_pretty(&tasks).unwrap().as_bytes())
            .unwrap();
    }
}
