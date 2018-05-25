#[cfg(test)]
#[macro_use]
extern crate meta;

extern crate rand;

use rand::{thread_rng, Rng};

fn bogo_sort<T: Ord>(v: &mut [T]) {
    let mut rng = thread_rng();
    while !is_sorted(v) {
        rng.shuffle(v);
    }
}

// helper function that checks for ascending order
fn is_sorted<T: Ord>(v: &[T]) -> bool {
    if v.len() > 1 {
        for i in 0..(v.len() - 1) {
            if v[i] > v[i + 1] {
                return false;
            }
        }
    }
    true
}

fn main() {
    let mut numbers = [4i32, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);
    bogo_sort(&mut numbers);
    println!("After: {:?}", numbers);
}

#[cfg(test)]
mod tests {
    // The sort is random, so it could take a very long time!
    test_sort!(super::bogo_sort, #[ignore]);
}
