// Implements http://rosettacode.org/wiki/Create_a_file

extern crate libc;

use std::io::{File, fs};
use libc::S_IRWXU;

#[cfg(not(test))]
fn main () {
    // Create a new file.
    // We get a Result object from File::create. We could check if there is an error
    // by using .is_error() or by using pattern matching. We choose here to ignore
    // the possibility of an error and just unwrap the value contained in the Result object.
    // This means that an error will cause the program to fail at runtime.
    let mut new_file = File::create(&Path::new("output.txt")).unwrap();

    // Write something trivial to the file.
    // Now we are handling a possible error by using pattern matching
    match writeln!(&mut new_file as &mut Writer, "Nothing here...") {
        Ok(()) => (),
        Err(e) => println!("Failed to write to file: {}", e),
    }

    // Create a directory
    // Here we handle a possible error by using the functions provided by result
    // The second argument is an unsigned int that sets the file permissions
    let result = fs::mkdir(&Path::new("docs"), S_IRWXU as u32);
    if result.is_err() {
        println!("Failed to create a directory: {}", result.err().unwrap());
    }
}
