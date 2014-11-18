// http://rosettacode.org/wiki/Closures/Value_capture

#![feature(overloaded_calls, unboxed_closures)]

use std::iter::count;
use std::num::Float;

fn main() {
    // An infinite iterator that generates closures
    let closures = count(0u, 1).map(|x| move |:| (x as f64).powi(2));

    // Take the first 9 closures from the iterator and call them
    for c in closures.take(9) {
        println!("{}", c())
    }
}

// FIXME: add a function that returns an iterator over unboxed closures
// (blocked by the lack of anonymous types)
