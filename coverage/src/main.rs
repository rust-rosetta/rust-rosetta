#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate coverage;
extern crate difference;
extern crate docopt;
extern crate rust_rosetta;
extern crate rustc_serialize;
extern crate term;

use std::io;

use difference::Difference;
use term::Terminal;

use coverage::Task;

docopt!(Args derive Debug, "
Detect unimplemented tasks.

This script prints out the name of each task, followed by whether it is implemented online,
locally, or both.

Tasks must be specified using normalized names, e.g. \"K-d tree\". If no tasks are specified,
determines the status for all tasks.

Optionally prints out a diff as well.

Usage:
    coverage [options] [<tasks>...]

Options:
    -h --help           Show this screen.

    --nodiff            Don't print diffs.

    --filter=<type>     Filter tasks printed by the program. Accepted values:

                            all                 Print all tasks (default).

                            localonly           Only print tasks that are implemented locally, but
                                                not on the wiki.

                            remoteonly          Only print tasks that are implemented on the wiki,
                                                but not locally.

                            unimplemented       Only print tasks that neither implemented locally
                                                nor remotely.
", flag_filter: Option<TaskFilter>);

#[derive(Debug, Clone, RustcDecodable)]
enum TaskFilter {
    All,
    LocalOnly,
    RemoteOnly,
    Unimplemented,
}

impl Default for TaskFilter {
    fn default() -> Self {
        TaskFilter::All
    }
}

/// Prints a colored diff of two strings to the terminal.
fn print_diff<T: ?Sized>(t: &mut T, s1: &str, s2: &str) -> io::Result<()>
    where T: Terminal
{
    let (_dist, changeset) = difference::diff(&s1, &s2, "\n");

    for i in 0..changeset.len() {
        match changeset[i] {
            Difference::Same(ref x) => {
                try!(t.reset());
                try!(writeln!(t, " {}", x));
            }
            Difference::Add(ref x) => {
                try!(t.fg(term::color::GREEN));
                for line in x.split("\n") {
                    try!(writeln!(t, "+{}", line));
                }
            }
            Difference::Rem(ref x) => {
                try!(t.fg(term::color::RED));
                for line in x.split("\n") {
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
    where T: Terminal
{
    try!(t.attr(term::Attr::Bold));
    try!(writeln!(t, "{}", task.title));
    try!(t.reset());

    try!(write!(t, "Local:"));
    try!(write_status(t, task.local_code.is_some()));

    try!(write!(t, "Remote:"));
    try!(write_status(t, task.remote_code.is_some()));
    try!(writeln!(t, ""));

    if let (Some(ref local_code), Some(ref remote_code)) = (task.local_code.clone(),
                                                            task.remote_code.clone()) {
        if diff {
            try!(print_diff(t, remote_code, local_code));
        }
    }

    Ok(())
}

/// Writes a boolean as a pretty, human-readable string.
fn write_status<T: ?Sized>(t: &mut T, boolean: bool) -> io::Result<()>
    where T: Terminal
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
    let args: Args = Args::docopt()
        .decode()
        .unwrap_or_else(|e| e.exit());

    let mut t = term::stdout().unwrap();

    let tasks = if args.arg_tasks.len() > 0 {
        coverage::fetch_tasks(&args.arg_tasks.as_slice())
    } else {
        coverage::fetch_all_tasks()
    };

    let task_filter = args.flag_filter.unwrap_or_default().to_owned();

    for task in tasks {
        match task_filter {
            TaskFilter::LocalOnly if !task.is_local_only() => continue,
            TaskFilter::RemoteOnly if !task.is_remote_only() => continue,
            TaskFilter::Unimplemented if !task.is_unimplemented() => continue,
            TaskFilter::All | _ => print_task(&mut *t, &task, !args.flag_nodiff).unwrap(),
        }
    }
}
