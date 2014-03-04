//Implements http://rosettacode.org/wiki/Primality_by_Trial_Division
use std::iter::range_step;

fn is_prime(nb: int) -> bool {
    if nb%2 == 0 {
        return false;
    } else {
        for i in range_step(3,(nb as f32).sqrt() as int + 1, 2) {
            if nb%i == 0 {
                return false;
            }
        }
    }
    true
}

fn main() {
    println!("{:b}", is_prime(15485863)); // The 1 000 000th prime.
    println!("{:b}", is_prime(62773913)); // The product of the 1000th and 1001st primes.
}