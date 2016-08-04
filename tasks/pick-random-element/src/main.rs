extern crate rand;

use rand::{thread_rng, Rng};

fn pick_random_e(a: &[u8]) -> u8 {
    // thread-local random number generator
    let mut rng = thread_rng();

    let ri = rng.gen_range(0, a.len());

    a[ri]
}

fn main() {
    let xs: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    println!("{}", pick_random_e(&xs));
}

#[test]
fn test_pick_random_e() {
    let xs: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let picked = pick_random_e(&xs);

    assert!(xs.contains(&picked));
}
