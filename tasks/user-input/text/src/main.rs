//! This program demonstrates proper error handling.

use std::io::{self, Write};
use std::fmt::Display;
use std::process;

fn grab_input(msg: &str) -> io::Result<String> {
    let mut buf = String::new();
    print!("{}: ", msg);
    try!(io::stdout().flush());

    try!(io::stdin().read_line(&mut buf));
    Ok(buf)
}

fn exit_err<T: Display>(msg: T, code: i32) -> ! {
    let _ = writeln!(&mut io::stderr(), "Error: {}", msg);
    process::exit(code)
}

fn main() {
    let s = grab_input("Give me a string")
        .unwrap_or_else(|e| exit_err(&e, e.raw_os_error().unwrap_or(-1)));

    println!("You entered: {}", s.trim());

    let n: i32 = grab_input("Give me an integer")
        .unwrap_or_else(|e| exit_err(&e, e.raw_os_error().unwrap_or(-1)))
        .trim()
        .parse()
        .unwrap_or_else(|e| exit_err(&e, 2));

    println!("You entered: {}", n);
}
