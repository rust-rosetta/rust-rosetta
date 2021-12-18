//===With crate "primal"===
//primal implements the sieve of Eratosthenes with optimizations (10+ times faster for large limits)

fn sum_digits(u: usize) -> usize {
    std::iter::successors(Some(u), |&n| (n > 9).then(|| n / 10)).fold(0, |s, n| s + n % 10)
}

fn main() {
    let limit = 500;
    let sieve_primes = primal::Sieve::new(limit);
    let column_width = limit.to_string().len() + 1;
    let count = sieve_primes
        .primes_from(2)
        .filter(|&p| p < limit && sieve_primes.is_prime(sum_digits(p)))
        .zip(["\n"].iter().chain(&[""; 9]).cycle())
        .inspect(|(u, sn)| print!("{}{:2$}", sn, u, column_width))
        .count();
    println!("\n---\nFound {} additive primes less than {}", count, limit);
}
