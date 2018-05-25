extern crate rand;

use rand::prelude::*;

fn rand_n<R: Rng>(rng: &mut R, n: u32) -> usize {
    (rng.gen_range(0, n) == 0) as usize // maps `false` to 0 and `true` to 1
}

fn unbiased<R: Rng>(rng: &mut R, n: u32) -> usize {
    let mut bit = rand_n(rng, n);
    while bit == rand_n(rng, n) {
        bit = rand_n(rng, n);
    }
    bit
}

fn main() {
    const SAMPLES: usize = 100_000;
    let mut rng = SmallRng::from_entropy();

    println!(" Bias    rand_n  unbiased");
    for n in 3..=6 {
        let mut count_biased = 0;
        let mut count_unbiased = 0;
        for _ in 0..SAMPLES {
            count_biased += rand_n(&mut rng, n);
            count_unbiased += unbiased(&mut rng, n);
        }

        let b_percentage = 100.0 * count_biased as f64 / SAMPLES as f64;
        let ub_percentage = 100.0 * count_unbiased as f64 / SAMPLES as f64;
        println!(
            "bias {}:  {:0.2}%   {:0.2}%",
            n, b_percentage, ub_percentage
        );
    }
}

#[test]
fn test_unbiased() {
    let mut rng = SmallRng::from_entropy();
    const SAMPLES: usize = 10_000;

    for n in 3..=6 {
        let mut count = 0;
        for _ in 0..SAMPLES {
            count += unbiased(&mut rng, n);
        }

        let ratio = 1000 * count / SAMPLES;
        assert!(ratio > 450 && ratio < 550, "{}", ratio);
    }
}
