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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_prime_factor() {
        assert_eq!(extract_prime_factor(0), None);
        assert_eq!(extract_prime_factor(1), None);
        assert_eq!(extract_prime_factor(2), None);
        assert_eq!(extract_prime_factor(3), None);
        assert_eq!(extract_prime_factor(4), Some((2, 2)));
        assert_eq!(extract_prime_factor(5), None);
        assert_eq!(extract_prime_factor(6), Some((2, 3)));
        assert_eq!(extract_prime_factor(7), None);
        assert_eq!(extract_prime_factor(8), Some((2, 4)));
        assert_eq!(extract_prime_factor(9), Some((3, 3)));
        assert_eq!(extract_prime_factor(10), Some((2, 5)));
        assert_eq!(extract_prime_factor(11), None);
        assert_eq!(extract_prime_factor(12), Some((2, 6)));
    }

    #[test]
    fn test_factorize() {
        assert_eq!(factorize(0), vec![0]);
        assert_eq!(factorize(1), vec![1]);
        assert_eq!(factorize(2), vec![2]);
        assert_eq!(factorize(3), vec![3]);
        assert_eq!(factorize(4), vec![2, 2]);
        assert_eq!(factorize(5), vec![5]);
        assert_eq!(factorize(6), vec![2, 3]);
        assert_eq!(factorize(7), vec![7]);
        assert_eq!(factorize(8), vec![2, 2, 2]);
        assert_eq!(factorize(9), vec![3, 3]);
        assert_eq!(factorize(10), vec![2, 5]);
        assert_eq!(factorize(11), vec![11]);
        assert_eq!(factorize(12), vec![2, 2, 3]);
    }
}
