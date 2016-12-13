use std::iter::Map;
use std::ops::RangeFrom;

#[allow(dead_code)]
fn simple_unboxed() {
    // simple usage when unboxed closures will do
    let mut v = Vec::new();
    for i in 0..10 {
        v.push(move || i * i);
    }
    println!("7th val: {}", v[7]());
}

// given a number x, return the (boxed) closure that
// computes x squared
fn closure_gen<'a>(x: u32) -> Box<Fn() -> f64 + 'a> {
    Box::new(move || (x as f64).powi(2))
}

// type alias for the closure iterator
type ClosureIter<'a> = Map<RangeFrom<u32>, fn(u32) -> Box<Fn() -> f64 + 'a>>;

// return an iterator that on every iteration returns
// a closure computing the index of the iteration squared
fn closures_iterator<'a>() -> ClosureIter<'a> {
    let cl_gen: fn(u32) -> Box<Fn() -> f64 + 'a> = closure_gen;
    (0..).map(cl_gen)
}

fn main() {
    // Take the first 9 closures from the iterator and call them
    for c in closures_iterator().take(9) {
        println!("{}", c())
    }
}

#[cfg(test)]
mod tests {
    use super::{closure_gen, closures_iterator};

    use std::f64;

    #[test]
    fn closure_generator() {
        let five_squarer = closure_gen(5);
        assert!((five_squarer() - 25f64).abs() < f64::EPSILON);
    }

    #[test]
    fn closure_iterator() {
        for (idx, f) in closures_iterator().take(9).enumerate() {
            assert!((f() - (idx as f64).powi(2)).abs() < f64::EPSILON);
        }
    }
}
