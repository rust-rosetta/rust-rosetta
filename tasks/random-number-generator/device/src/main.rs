extern crate rand;

use rand::{EntropyRng, Rng};

fn main() {
    let mut rng = EntropyRng::new();

    let rand_num: u32 = rng.gen();
    println!("{}", rand_num);
}
