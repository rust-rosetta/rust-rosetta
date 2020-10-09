use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();

    let filename = args
        .nth(1)
        .ok_or("You must enter a filename to read line by line")?;

    let line_number = args
        .next()
        .ok_or("You must enter a line number")?
        .parse::<usize>()?;

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    match reader.lines().nth(line_number - 1) {
        None => panic!("No such line (file is too short)"),
        Some(result) => {
            match result {
                // Handle any errors that may arise
                Ok(ln) => print!("{}", ln),
                Err(error) => print!("{}", error),
            }
        }
    }

    Ok(())
}
