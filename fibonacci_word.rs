// Implements http://rosettacode.org/wiki/Fibonacci_word
extern crate collections;

use entropy::shannon_entropy;
mod entropy;

fn main() {
    println!("{:>2s}:{:>10s} {:s}", "N", "length", "entropy");

    let mut previous = ~"1";
    println!("{:>2i}:{:>10u} {:f}", 1, previous.len(), shannon_entropy(previous));

    let mut next = ~"0";
    println!("{:>2i}:{:>10u} {:f}", 2, next.len(), shannon_entropy(next));

    for i in range(3, 38) {
        let temp = next.clone();
        next.push_str(previous);
        previous = temp;
        println!("{:>2i}:{:>10u} {:.15f}", i, next.len(), shannon_entropy(next));
    }
}
