use std::fs::{self, File};
use std::io::Write;

fn main() {
    // Create a new file.  We get a `Result` object from `File::create`. We could check if there is
    // an error by using `.is_err()` or by using pattern matching. We choose here to ignore the
    // possibility of an error and just unwrap the value contained in the `Result` object.  This
    // means that an error will cause the program to fail at runtime.
    let mut new_file = File::create("build/output.txt").unwrap();

    // Write something trivial to the file. Now we are handling a possible error by using pattern
    // matching.
    match new_file.write_all(b"Nothing here...") {
        Ok(()) => (),
        Err(e) => println!("Failed to write to file: {}", e),
    }

    // Create a directory. Here we handle a possible error by using the functions provided by
    // result.  The second argument sets the file permissions
    let result = fs::create_dir("build/docs");
    if result.is_err() {
        println!("Failed to create a directory: {}", result.err().unwrap());
    }
}

#[test]
fn test_create_file() {
    use std::fs;
    use std::path::Path;

    let build_dir = Path::new("build-tests");
    if !(build_dir.exists() && build_dir.is_dir()) {
        let r = fs::create_dir(&build_dir);
        assert!(r.is_ok());
    }

    let file_path = Path::new("build-tests/create_file_test.txt");
    if file_path.exists() && file_path.is_file() {
        let r = fs::remove_file(&file_path);
        assert!(r.is_ok());
    }
    match File::create(&file_path) {
        Ok(_) => assert!(true),
        Err(e) => {
            panic!("failed to create_file at {}, error: {}",
                   file_path.display(),
                   e)
        }
    }

    // Remove the build dir, but only after making sure there's only one file in it.
    let contents = build_dir.read_dir().unwrap().collect::<Result<Vec<_>, _>>().unwrap();

    assert!(contents.len() == 1);
    assert!(contents[0].path() == file_path);

    fs::remove_dir_all(build_dir).unwrap();
}
