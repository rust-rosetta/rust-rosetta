#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate coverage;
extern crate difference;
extern crate docopt;
extern crate rust_rosetta;
extern crate rustc_serialize;
extern crate term;

use difference::Difference;
use term::Terminal;

use std::io::prelude::*;

docopt!(Args derive Debug, "
Detect unimplemented tasks.

This script prints out the name of each task, followed by whether it is implemented online,
locally, or both.

If no tasks are specified, determines the status for all tasks.

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

fn main() {
    let args: Args = Args::docopt()
                         .decode()
                         .unwrap_or_else(|e| e.exit());

    let mut t = term::stdout().unwrap();

    let tasks = if args.arg_tasks.len() > 0 {
        coverage::fetch_tasks(&args.arg_tasks
                                   .clone()
                                   .iter()
                                   .map(|title| title.to_owned())
                                   .collect::<Vec<_>>())
    } else {
        coverage::fetch_all_tasks()
    };

    let task_filter = args.flag_filter.unwrap_or_default().to_owned();

    for task in tasks {
        match task_filter {
            TaskFilter::LocalOnly if !task.is_local_only() => continue,
            TaskFilter::RemoteOnly if !task.is_remote_only() => continue,
            TaskFilter::Unimplemented if !task.is_unimplemented() => continue,
            TaskFilter::All | _ => (),
        }

        t.attr(term::Attr::Bold).unwrap();
        writeln!(t, "{}", task.title).unwrap();
        t.reset().unwrap();

        writeln!(t,
                 "Local: {}, Remote: {}",
                 task.local_code.is_some(),
                 task.remote_code.is_some())
            .unwrap();

        if !args.flag_nodiff && task.remote_code.is_some() && task.local_code.is_some() {
            let (_dist, changeset) = difference::diff(&task.remote_code.unwrap(),
                                                      &task.local_code.unwrap(),
                                                      "\n");

            let mut t = term::stdout().unwrap();

            for i in 0..changeset.len() {
                match changeset[i] {
                    Difference::Same(ref x) => {
                        t.reset().unwrap();
                        writeln!(t, " {}", x).unwrap();
                    }
                    Difference::Add(ref x) => {
                        t.fg(term::color::GREEN).unwrap();
                        for line in x.split("\n") {
                            writeln!(t, "+{}", line).unwrap();
                        }
                    }
                    Difference::Rem(ref x) => {
                        t.fg(term::color::RED).unwrap();
                        for line in x.split("\n") {
                            writeln!(t, "-{}", line).unwrap();
                        }
                    }
                }
            }
        }
        t.reset().unwrap();
        t.flush().unwrap();
    }
}
