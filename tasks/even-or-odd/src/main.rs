#![allow(unused_variables)]

fn main() {
    // Checking the last significant digit:
    let is_odd = |x: i32| x & 1 == 1;
    let is_even = |x: i32| x & 1 == 0;

    // Using modular congruences:
    let is_odd = |x: i32| x % 2 != 0;
    let is_even = |x: i32| x % 2 == 0;
}
