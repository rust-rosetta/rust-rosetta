extern crate libc;

#[cfg(unix)]
fn main() {
    let istty = unsafe { libc::isatty(libc::STDOUT_FILENO as i32) } != 0;
    if istty {
        println!("stdout is tty");
    } else {
        println!("stdout is not tty");
    }
}

#[cfg(not(unix))]
fn main() {
    unimplemented!();
}
