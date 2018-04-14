extern crate rand;

use rand::{OsRng, Rng};

fn main() {
    // because `OsRng` opens files, it may fail
    let mut rng = match OsRng::new() {
        Ok(v) => v,
        Err(e) => panic!("Failed to obtain OS RNG: {}", e),
    };

    let rand_num: u32 = rng.gen();
    println!("{}", rand_num);
}
