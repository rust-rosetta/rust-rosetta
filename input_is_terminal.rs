extern mod std;
use std::libc;

#[fixed_stack_segment]
fn main() {
	let istty = unsafe { libc::isatty(libc::STDIN_FILENO as i32) } != 0;
	if (istty) {
		println("stdinn is tty");
	} else {
		println("stdin is not tty");
	}
}
