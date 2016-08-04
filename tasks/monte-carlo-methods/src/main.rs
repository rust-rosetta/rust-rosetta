extern crate rand;

use rand::Rng;

#[inline]
fn inside_circle(p: (f64, f64)) -> i64 {
    if p.0 * p.0 + p.1 * p.1 <= 1f64 {
        1
    } else {
        0
    }
}

fn simulate(time: i64) -> f64 {
    let mut rng = rand::thread_rng();
    let mut cnt = 0;
    for _ in 0..time {
        cnt += inside_circle(rng.gen());
    }
    (cnt as f64) / (time as f64)
}

pub fn main() {
    for i in (3..9).map(|a| 10i64.pow(a)) {
        println!("{:10}:{:.10}", i, 4f64 * simulate(i));
    }
}
