extern crate num;

use num::complex::Complex;

fn main() {
    let a = Complex::new(-4.0f32, 5.0);
    let b = Complex::new(1.0f32, 1.0);

    println!("a = {}", a);
    println!("b = {}", b);
    println!("a + b = {}", a + b);
    println!("a * b = {}", a * b);
    println!("1 / a = {}", Complex::new(1.0f32, 0.0) / a);
    println!("-a = {}", -a);
    println!("conj a = {}", a.conj());
}
