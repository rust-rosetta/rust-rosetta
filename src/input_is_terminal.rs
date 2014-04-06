extern crate libc;

fn main() {
    let istty = unsafe { libc::isatty(libc::STDIN_FILENO as i32) } != 0;
    if istty {
        println!("stdin is tty");
    } else {
        println!("stdin is not tty");
    }
}
