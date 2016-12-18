extern crate meta;

use std::fs;
use std::io;
use std::path::{PathBuf, Path};
use meta::local;

fn main() {
    let tasks_toml: Vec<String> = local::parse_tasks("Cargo.toml")
        .into_iter()
        .map(|x| {
            // println!("{:?}", x);
            x.crate_name()
        })
        .collect();

    // TODO Need to look into nested structures
    let tasks_dir = find_all_local(Path::new("./tasks/")).unwrap();



    // for task in &tasks_toml {
    //     println!("{:?}", task);
    // }

    for task_solution in &tasks_dir {
        if !tasks_toml.contains(task_solution) {
            println!("cargo:warning={} is not added to the root Cargo.toml",
                     task_solution);
        }
    }
}

fn find_all_local(root_dir: &Path) -> io::Result<Vec<String>> {
    let mut tasks_dir = Vec::new();
    for dir in try!(fs::read_dir(root_dir)) {
        let entry = try!(dir);
        let path = entry.path();
        // Only stop when one of the directories is 'src'
        let src_dir = PathBuf::from("src");
        if path.is_dir() && !path.ends_with(&src_dir) {
            tasks_dir.append(&mut find_all_local(&path).unwrap());
        } else if path.ends_with(&src_dir) {
            let path = path.strip_prefix("./tasks/").unwrap().parent().unwrap();
            // Remove the src dir
            // println!("Adding {:?}", path);
            let path_str = path.to_str().unwrap();
            tasks_dir.push(path_str.to_owned());
        }
    }
    Ok(tasks_dir)
}
