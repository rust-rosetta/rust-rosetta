// http://rosettacode.org/wiki/Closures/Value_capture
use std::iter::count;
use std::num::Float;

fn main() {
    // An infinite iterator that generates closures
    let closures = count(0us, 1).map(|x| move |:| (x as f64).powi(2));

    // Take the first 9 closures from the iterator and call them
    for c in closures.take(9) {
        println!("{:?}", c())
    }
}

// FIXME: add a function that returns an iterator over unboxed closures
// (blocked by the lack of anonymous types)
