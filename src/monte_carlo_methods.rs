// http://rosettacode.org/wiki/Monte_Carlo_methods

#![feature(core)]
extern crate rand;
use rand::Rng;
use std::iter::iterate;

#[inline]
fn inside_circle(p:(f64, f64)) -> i64 {
    if p.0*p.0 + p.1*p.1 <= 1f64 {
        1
    } else {
        0
    }
}

fn simulate(time: i64) -> f64 {
    let mut rng = rand::thread_rng();
    let mut cnt = 0;
    for _ in 0..time {
        cnt = cnt + inside_circle(rng.gen());
    }
    (cnt as f64) / (time as f64)
}

#[cfg(not(test))]
pub fn main() {
    for i in iterate(1000, |x| x*10).take(5) {
        println!("{:10}:{:.10}" , i, 4f64 * simulate(i));
    }
}
