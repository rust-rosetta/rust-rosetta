// Implements http://rosettacode.org/wiki/Check_output_device_is_a_terminal
#![feature(libc)]

extern crate libc;

fn main() {
    let istty = unsafe { libc::isatty(libc::STDOUT_FILENO as i32) } != 0;
    if istty {
        println!("stdout is tty");
    } else {
        println!("stdout is not tty");
    }
}
