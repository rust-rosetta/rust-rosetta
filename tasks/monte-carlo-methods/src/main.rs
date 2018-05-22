extern crate rand;

use rand::prelude::*;
use std::f64::consts::PI;

// `(f32, f32)` may be faster for some RNGs (including `rand::XorShiftRng`),
// but less accurate.
fn is_inside_circle((x, y): (f64, f64)) -> bool {
    x * x + y * y <= 1.0
}

fn simulate<R: Rng>(rng: &mut R, samples: usize) -> f64 {
    let count = (0..samples).filter(|_| is_inside_circle(rng.gen())).count();
    // A branchless method might be faster
    /*for _ in 0..samples {
        count += is_inside_circle(rng.gen()) as usize;
    }*/
    (count as f64) / (samples as f64)
}

fn main() {
    let mut rng = SmallRng::from_entropy();

    println!("Real pi: {}", PI);

    for samples in (3..9).map(|e| 10_usize.pow(e)) {
        let estimate = 4.0 * simulate(&mut rng, samples);
        let deviation = 100.0 * (1.0 - estimate / PI).abs();
        println!("{:9}: {:<11} dev: {:.5}%", samples, estimate, deviation);
    }
}
