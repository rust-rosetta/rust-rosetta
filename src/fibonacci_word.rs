// Implements http://rosettacode.org/wiki/Fibonacci_word
extern crate collections;

use entropy::shannon_entropy;
mod entropy;

fn main() {
    println!("{:>2s}:{:>10s} {:s}", "N", "length", "entropy");

    let mut previous = StrBuf::from_str("1");
    println!("{:>2i}:{:>10u} {:f}", 1, previous.len(),
             shannon_entropy(previous.as_slice()));

    let mut next = StrBuf::from_str("0");
    println!("{:>2i}:{:>10u} {:f}", 2, next.len(),
             shannon_entropy(next.as_slice()));

    for i in range(3, 38) {
        let temp = next.clone();
        next.push_str(previous.as_slice());
        previous = temp;
        println!("{:>2i}:{:>10u} {:.15f}", i, next.len(),
                 shannon_entropy(next.as_slice()));
    }
}
