use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("resources/input.txt").unwrap();

    let mut contents: Vec<u8> = Vec::new();
    // Returns amount of bytes read and append the result to the buffer
    let result = file.read_to_end(&mut contents).unwrap();
    println!("Read {} bytes", result);

    // To print the contents of the file
    let filestr = String::from_utf8(contents).unwrap();
    println!("{}", filestr);
}
