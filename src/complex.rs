// Implements http://rosettacode.org/wiki/Arithmetic/Complex
#![cfg(not_tested)]

extern crate num;

use num::complex::Complex;

fn main() {
    let a = Complex::new(-4.0, 5.0);
    let b = Complex::new(1.0, 1.0);

    println!("a = {}", a);
    println!("b = {}", b);
    println!("a + b = {}", a + b);
    println!("a * b = {}", a * b);
    println!("1 / a = {}", Complex::new(1.0, 0.0) / a);
    println!("-a = {}", -a);
    println!("conj a = {}", a.conj());
}
