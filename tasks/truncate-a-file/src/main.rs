use std::fs::OpenOptions;
use std::env;
use std::str::FromStr;

fn main() {
    if env::args().count() != 3 {
        println!("Usage: {} <filename> <size>", env::args().nth(0).unwrap());
    } else {
        let filename = env::args().nth(1).unwrap();
        let size = env::args().nth(2).unwrap();
        match u64::from_str(&size) {
            Ok(integer_size) => {
                let file = OpenOptions::new().write(true).open(filename).unwrap();
                let _ = file.set_len(integer_size);
            }
            Err(_) => println!("Invalid size"),
        }
    }
}
