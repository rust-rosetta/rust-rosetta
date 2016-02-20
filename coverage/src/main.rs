extern crate coverage;
extern crate difference;
extern crate docopt;
extern crate rustc_serialize;
extern crate rust_rosetta;
extern crate term;

use docopt::Docopt;
use difference::Difference;
use term::Terminal;

use std::io::prelude::*;

const USAGE: &'static str = r"
Detect unimplemented tasks.

This script prints out the name of each task, followed by whether it is implemented online,
locally, or both.

If no tasks are specified, determines the status for all tasks.

Optionally prints out a diff as well.

Usage:
    coverage [options] [--localonly | --remoteonly | --unimplemented] [<tasks>...]

Options:
    -h --help           Show this screen.
    --nodiff            Don't print diffs.
    --localonly         Only print tasks that are implemented locally, but not on the wiki.
    --remoteonly        Only print tasks that are implemented on the wiki, but not locally.
    --unimplemented     Only print tasks that neither implemented locally nor remotely.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_tasks: Vec<String>,
    flag_nodiff: bool,
    flag_localonly: bool,
    flag_remoteonly: bool,
    flag_unimplemented: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                         .and_then(|d| d.decode())
                         .unwrap_or_else(|e| e.exit());

    let mut t = term::stdout().unwrap();

    let tasks = if args.arg_tasks.len() > 0 {
        coverage::fetch_tasks(&args.arg_tasks
                                   .iter()
                                   .map(|title| title.to_owned())
                                   .collect::<Vec<_>>())
    } else {
        coverage::fetch_all_tasks()
    };

    for task in tasks {
        if (args.flag_localonly && !task.is_local_only()) ||
           (args.flag_remoteonly && !task.is_remote_only()) ||
           (args.flag_unimplemented && !task.is_unimplemented()) {
            continue;
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
