fn is_prime(number: u32) -> bool {
    #[allow(clippy::cast_precision_loss)]
    let limit = (number as f32).sqrt() as u32 + 1;

    // We test if the number is divisible by any number up to the limit
    !(number < 2 || (2..limit).any(|x| number % x == 0))
}

fn main() {
    println!(
        "Primes below 100:\n{:?}",
        (0_u32..100).fold(vec![], |mut acc, number| {
            if is_prime(number) {
                acc.push(number)
            };
            acc
        })
    );
}

#[cfg(test)]
mod tests {
    use super::is_prime;

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(7919), true);
    }
}
