#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_json;

extern crate difference;
extern crate meta;
extern crate serde;
extern crate term;

use std::fs::File;
use std::io::prelude::*;
use std::io;

use clap::{App, Arg};
use difference::{Changeset, Difference};
use term::Terminal;

use meta::{Task, TaskIndex};

const ABOUT: &str = r#"
Query differences between the rust-rosetta repository and the Rosetta Code wiki.

This script prints out the name of each task, followed by whether it is implemented online,
locally, or both.

If no tasks are specified, determines the status for all tasks."#;

arg_enum!{
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

/// Prints a colored diff of two strings to the terminal.
fn print_diff<T: ?Sized>(t: &mut T, s1: &str, s2: &str) -> io::Result<()>
where
    T: Terminal,
{
    let changeset = Changeset::new(s1, s2, "\n");

    for change in changeset.diffs {
        match change {
            Difference::Same(ref x) => {
                try!(t.reset());
                try!(writeln!(t, " {}", x));
            }
            Difference::Add(ref x) => {
                try!(t.fg(term::color::GREEN));
                for line in x.split('\n') {
                    try!(writeln!(t, "+{}", line));
                }
            }
            Difference::Rem(ref x) => {
                try!(t.fg(term::color::RED));
                for line in x.split('\n') {
                    try!(writeln!(t, "-{}", line));
                }
            }
        }
    }
    try!(t.reset());
    try!(t.flush());
    Ok(())
}

/// Prints a task in a human-readable format.
fn print_task<T: ?Sized>(t: &mut T, task: &Task, diff: bool) -> io::Result<()>
where
    T: Terminal,
{
    try!(t.attr(term::Attr::Bold));
    try!(writeln!(t, "{}", task.title()));
    try!(t.reset());

    try!(write!(t, "Local:"));
    try!(write_status(t, task.local_code().is_some()));

    try!(write!(t, "Remote:"));
    try!(write_status(t, task.remote_code().is_some()));
    try!(writeln!(t, ""));

    if let (Some(ref local_code), Some(ref remote_code)) = (task.local_code(), task.remote_code()) {
        if diff {
            try!(print_diff(t, remote_code, local_code));
        }
    }

    Ok(())
}

/// Writes a boolean as a pretty, human-readable string.
fn write_status<T: ?Sized>(t: &mut T, boolean: bool) -> io::Result<()>
where
    T: Terminal,
{
    try!(t.attr(term::Attr::Bold));

    if boolean {
        try!(t.fg(term::color::GREEN));
        try!(write!(t, " ✔ "))
    } else {
        try!(t.fg(term::color::RED));
        try!(write!(t, " ✘ "));
    }

    try!(t.reset());
    Ok(())
}

fn main() {
    let matches = App::new("coverage")
        .about(ABOUT)
        .version(crate_version!())
        .max_term_width(100)
        .arg(
            Arg::with_name("task")
                .help("The name of a task on the wiki, such as 'K-d tree'")
                .multiple(true),
        )
        .arg(
            Arg::with_name("diff")
                .help("Print diffs of tasks between the local and remote version")
                .long("diff"),
        )
        .arg(
            Arg::with_name("filter")
                .help("Filter tasks printed by the program.")
                .possible_values(&["all", "local", "remote", "unimplemented"])
                .long("filter")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("json-file")
                .help("Dump json to the provided filename")
                .long("json")
                .takes_value(true),
        )
        .get_matches();

    let mut t = term::stdout().unwrap();

    let filter = value_t!(matches.value_of("filter"), Filter)
        .ok()
        .unwrap_or_default();

    let task_index = TaskIndex::create(env!("CARGO_MANIFEST_DIR")).unwrap();

    let tasks = if let Some(tasks) = matches.values_of("task") {
        let task_names = tasks.map(String::from).collect::<Vec<_>>();
        task_index.fetch_tasks(&task_names)
    } else {
        task_index.fetch_all_tasks()
    };

    let tasks = tasks
        .flat_map(|task| {
            let task = task.unwrap();

            match filter {
                Filter::LocalOnly if !task.is_local_only() => return None,
                Filter::RemoteOnly if !task.is_remote_only() => return None,
                Filter::Unimplemented if !task.is_unimplemented() => return None,
                Filter::All | _ => {}
            }

            print_task(&mut *t, &task, matches.is_present("diff")).unwrap();

            if matches.is_present("json-file") {
                let json = json!({
                    "title": task.title(),
                    "url": task.url().to_string(),
                    "local_code": task.local_code(),
                    "remote_code": task.remote_code(),
                });

                Some(json)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if let Some(filename) = matches.value_of("json-file") {
        let mut file = File::create(filename).unwrap();
        file.write_all(serde_json::to_string_pretty(&tasks).unwrap().as_bytes())
            .unwrap();
    }
}
