// http://rosettacode.org/wiki/Sum_of_a_series

fn main() {
    let sum: f64 = (1u64 .. 1000 + 1).fold(0.,|sum, num| sum + 1./(num*num) as f64);
    println!("{}", sum);
}
