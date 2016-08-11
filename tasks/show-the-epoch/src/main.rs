extern crate time;

use time::{at_utc, Timespec};

fn main() {
    let epoch = at_utc(Timespec::new(0, 0));
    println!("{}", epoch.asctime());
}
