#![feature(iterator_step_by)]

fn is_prime(number: i32) -> bool {
    if number % 2 == 0 && number != 2 {
        return false;
    }

    let limit = (number as f32).sqrt() as i32 + 1;

    // We test if the number is divisible by any odd number up to the limit
    (3..limit).step_by(2).all(|x| number % x != 0)
}

fn main() {
    println!("{}", is_prime(15485863)); // The 1 000 000th prime.
    println!("{}", is_prime(62773913)); // The product of the 1000th and 1001st primes.
}

#[test]
fn test_one() {
    assert!(is_prime(1));
}

#[test]
fn test_two() {
    assert!(is_prime(2));
}

#[test]
fn test_many() {
    let primes = [3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
    assert!(primes.iter().all(|&x| is_prime(x)));
}
