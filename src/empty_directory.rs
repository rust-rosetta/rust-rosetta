// http://rosettacode.org/wiki/Empty_directory
use std::fs;
use std::path::Path;
use std::env::args;

fn main() {

    // Open path
    let path = if let Some(arg) = args().nth(1) {
        arg
    } else {
        println!("You must give a path as argument");
        return;
    };

    let dir = Path::new(&path);

    let paths = match fs::read_dir(dir) {
        Err(e) => {
            println!("Error: {}", e);
            return;
        },
        Ok(f) => f,
    };

    if paths.count() > 0 {
        println!("Directory is not empty");
    } else {
        println!("Directory is empty");
    }
}
