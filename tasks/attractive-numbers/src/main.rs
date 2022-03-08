use primal::StreamingSieve;

const MAX: u64 = 120;

/// Returns an Option with a tuple => Ok((smaller prime factor, num divided by that prime factor))
/// If num is a prime number itself, returns None
fn extract_prime_factor(num: u64) -> Option<(u64, u64)> {
    let mut i = 1;

    if num < 2 || primal::is_prime(num) {
        None
    } else {
        loop {
            let prime = StreamingSieve::nth_prime(i) as u64;

            if num % prime == 0 {
                return Some((prime, num / prime));
            } else {
                i += 1;
            }
        }
    }
}

/// Returns a vector containing all the prime factors of num
fn factorize(num: u64) -> Vec<u64> {
    let mut factorized = Vec::new();
    let mut rest = num;

    while let Some((prime, factorizable_rest)) = extract_prime_factor(rest) {
        factorized.push(prime);
        rest = factorizable_rest;
    }

    factorized.push(rest);
    factorized
}

fn main() {
    let mut output: Vec<u64> = Vec::new();
    for num in 4..=MAX {
        if primal::is_prime(factorize(num).len() as u64) {
            output.push(num);
        }
    }

    println!(
        "The attractive numbers up to and including {} are\n{:?}",
        MAX, output
    )
}
