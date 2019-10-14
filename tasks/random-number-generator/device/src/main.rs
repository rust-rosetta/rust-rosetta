extern crate rand;

use rand::prelude::*;
use rand::rngs::OsRng;

fn main() {
    let mut rng = OsRng;
    let rand_num: u32 = rng.gen();
    println!("{}", rand_num);
}
