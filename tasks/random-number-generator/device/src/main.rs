extern crate rand;

use rand::{Rng, OsRng};

fn main() {
    // becuase `OsRng` opens files, it may fail
    let mut rng = OsRng::new().unwrap();

    println!("{}", rng.gen::<u32>());
}
