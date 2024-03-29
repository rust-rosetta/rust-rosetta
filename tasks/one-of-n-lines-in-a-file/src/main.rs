extern crate rand;

use rand::{thread_rng, Rng};

fn one_of_n<R: Rng>(rng: &mut R, n: usize) -> usize {
    (1..n).fold(0, |keep, cand| {
        // Note that this will break if n is larger than u32::MAX
        if rng.gen_range(0..=cand) == 0 {
            cand
        } else {
            keep
        }
    })
}

fn main() {
    const LINES: usize = 10;

    let mut dist = [0; LINES];
    let mut rng = thread_rng();

    for _ in 0..1_000_000 {
        let num = one_of_n(&mut rng, LINES);
        dist[num] += 1;
    }

    println!("{:?}", dist);
}
