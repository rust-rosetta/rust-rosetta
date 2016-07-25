extern crate prime_decomposition;

use prime_decomposition::factor;

fn main() {
    println!("Factors of 5: {:?}", factor(5));
    println!("Factors of 15: {:?}", factor(15));
    println!("Factors of 16: {:?}", factor(16));
    println!("Factors of 10287: {:?}", factor(10287));
}
