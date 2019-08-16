use num::integer::gcd;

fn main() {
    // Compute the totient of the first 25 natural integers
    println!("N\t phi(n)\t Prime");
    for n in 1..26 {
        let phi_n = phi(n);
        println!("{}\t {}\t {:?}", n, phi_n, phi_n == n - 1);
    }

    // Compute the number of prime numbers for various steps
    [1, 100, 1000, 10000, 100000]
        .windows(2)
        .scan(0, |acc, window| {
            let (lower, upper) = (window[0], window[1]);
            *acc += (lower..=upper).filter(is_prime).count();
            Some((upper, *acc))
        })
        .for_each(|x| println!("Until {}: {} prime numbers", x.0, x.1));
}

fn is_prime(n: &usize) -> bool {
    phi(*n) == *n - 1
}

fn phi(n: usize) -> usize {
    (1..=n).filter(|&x| gcd(n, x) == 1).count()
}

#[cfg(test)]
mod tests {
    use super::{is_prime, phi};

    #[test]
    fn test_totient_primes() {
        let results = vec![
            (1, 1, false),
            (2, 1, true),
            (3, 2, true),
            (4, 2, false),
            (5, 4, true),
            (6, 2, false),
            (7, 6, true),
            (8, 4, false),
            (9, 6, false),
            (10, 4, false),
            (11, 10, true),
            (12, 4, false),
            (13, 12, true),
            (14, 6, false),
            (15, 8, false),
            (16, 8, false),
            (17, 16, true),
            (18, 6, false),
            (19, 18, true),
            (20, 8, false),
            (21, 12, false),
            (22, 10, false),
            (23, 22, true),
            (24, 8, false),
            (25, 20, false),
        ];
        for n in 1..26 {
            let phi_n = phi(n);
            assert_eq!((n, phi_n, phi_n == n - 1), results[n - 1]);
        }
    }

    #[test]
    #[ignore]
    fn test_totient_prime_list() {
        let results: Vec<usize> = [1, 100, 1000, 10000, 100000]
            .windows(2)
            .scan(0, |acc, window| {
                let (lower, upper) = (window[0], window[1]);
                *acc += (lower..=upper).filter(is_prime).count();
                Some(*acc)
            })
            .collect();

        assert_eq!(results, vec![25, 168, 1229, 9592]);
    }
}
