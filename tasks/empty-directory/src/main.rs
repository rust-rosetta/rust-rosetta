use std::fs;
use std::env::args;

fn main() {
    // Open path
    let dir = if let Some(arg) = args().nth(1) {
        arg
    } else {
        println!("You must give a path as argument");
        return;
    };

    // map function for type Result executes a function (closure)
    // on Ok(_) types but leaves Err(_) untouched.
    //
    // The directory is empty if the result of paths.count() == Ok(0)
    // In any other case, the directory is not empty
    match fs::read_dir(dir).map(|paths| paths.count()) {
        Err(e) => println!("Error: {}", e),
        Ok(0) => println!("Directory is empty"),
        Ok(_) => println!("Directory is not empty"),
    }
}
