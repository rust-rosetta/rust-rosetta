#![feature(inclusive_range_syntax)]

fn main() {
    let sum: f64 = (1u64..=1000).fold(0., |sum, num| sum + 1. / (num * num) as f64);
    println!("{}", sum);
}
