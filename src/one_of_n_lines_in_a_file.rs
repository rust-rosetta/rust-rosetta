// http://rosettacode.org/wiki/One_of_n_lines_in_a_file

extern crate rand;

use rand::Rng;

fn one_of_n(rng: &mut rand::ThreadRng, n: usize) -> usize {
    (1..n).fold(0, |keep, cand| {
        match rng.next_f64() {
            y if y < (1.0 / (cand + 1) as f64) => cand,
            _ => keep,
        }
    })
}

fn main() {
    let mut dist = [0usize; 10];
    let mut rng = rand::thread_rng();

    for _ in 0..1_000_000 {
        dist[one_of_n(&mut rng, 10)] += 1;
    }

    println!("{:?}", dist);
}
