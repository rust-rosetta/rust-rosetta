extern crate rand;

use rand::Rng;
use std::f64::consts::PI;

// `(f32, f32)` would be faster for some RNGs (including `rand::thread_rng` on 32-bit platforms
// and `rand::weak_rng` as of rand v0.4) as `next_u64` combines two `next_u32`s if not natively
// supported by the RNG.  It would less accurate however.
fn is_inside_circle((x, y): (f64, f64)) -> bool {
    x * x + y * y <= 1.0
}

fn simulate<R: Rng>(rng: &mut R, samples: usize) -> f64 {
    let mut count = 0;
    for _ in 0..samples {
        if is_inside_circle(rng.gen()) {
            count += 1;
        }
    }
    (count as f64) / (samples as f64)
}

fn main() {
    let mut rng = rand::weak_rng();

    println!("Real pi: {}", PI);

    for samples in (3..9).map(|e| 10_usize.pow(e)) {
        let estimate = 4.0 * simulate(&mut rng, samples);
        let deviation = 100.0 * (1.0 - estimate / PI).abs();
        println!("{:9}: {:<11} dev: {:.5}%", samples, estimate, deviation);
    }
}
