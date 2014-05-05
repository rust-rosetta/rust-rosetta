// Implements http://rosettacode.org/wiki/Fibonacci_word
extern crate collections;

use entropy::shannon_entropy;
use std::iter::range_inclusive;
mod entropy;

// Returns "amount" fibonacci words as a vector of tuples
// The first value of the tuple is the length of the word
// and the second one its entropy
fn fib_words(amount: uint) -> Vec<(uint, f64)> {
    let mut data = Vec::with_capacity(amount);
    let mut previous = StrBuf::from_str("1");
    let mut next = StrBuf::from_str("0");

    // The first two words (we need to add them manually because
    // they are the base of the sequence)
    data.push((previous.len(), shannon_entropy(previous.as_slice())));
    data.push((next.len(), shannon_entropy(next.as_slice())));

    // The rest of the words
    for _ in range_inclusive(3, amount) {
        let temp = next.clone();
        next.push_str(previous.as_slice());
        previous = temp;
        data.push((next.len(), shannon_entropy(next.as_slice())));
    }

    data
}

#[cfg(not(test))]
fn main() {
    println!("Calculating... This may take a couple of minutes...\n");

    let words = fib_words(18);
    let mut i = 1;

    println!("{:>2}:{:>10} {}", "N", "length", "entropy");
    for &(length, entropy) in words.iter() {
        println!("{:>2i}:{:>10u} {:.15f}", i, length, entropy);
        i += 1;
    }
}
