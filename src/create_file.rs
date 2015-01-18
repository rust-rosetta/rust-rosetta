// Implements http://rosettacode.org/wiki/Create_a_file
#![allow(unstable)]
use std::io::{self, File, fs};

#[cfg(not(test))]
fn main () {
    // Create a new file.  We get a Result object from
    // File::create. We could check if there is an error by using
    // .is_error() or by using pattern matching. We choose here to
    // ignore the possibility of an error and just unwrap the value
    // contained in the Result object.  This means that an error will
    // cause the program to fail at runtime.
    let mut new_file = File::create(&Path::new("build/output.txt")).unwrap();

    // Write something trivial to the file.
    // Now we are handling a possible error by using pattern matching
    match writeln!(&mut new_file as &mut Writer, "Nothing here...") {
        Ok(()) => (),
        Err(e) => println!("Failed to write to file: {}", e),
    }

    // Create a directory. Here we handle a possible error by using
    // the functions provided by result.  The second argument sets the
    // file permissions
    let result = fs::mkdir(&Path::new("build/docs"), io::USER_RWX);
    if result.is_err() {
        println!("Failed to create a directory: {}", result.err().unwrap());
    }
}

#[test]
fn test_create_file() {
    use std::io::fs::PathExtensions;

    let build_dir = Path::new("build-tests");
    if !(build_dir.exists() && build_dir.is_dir()) {
        let r = fs::mkdir(&build_dir, io::USER_RWX);
        assert!(r.is_ok());
    }

    let file_path = Path::new("build-tests/create_file_test.txt");
    if file_path.exists() && file_path.is_file() {
        let r = fs::unlink(&file_path);
        assert!(r.is_ok());
    }
    match File::create(&file_path) {
        Ok(_) => assert!(true),
        Err(e) => panic!("failed to create_file at {}, error: {}",
                        file_path.display(),
                        e.desc)
    }
}
