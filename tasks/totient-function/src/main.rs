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
        .scan(0, |acc, tuple| {
            *acc += (tuple[0]..=tuple[1]).filter(is_prime).count();
            Some((tuple[1], *acc))
        })
        .for_each(|x| println!("Until {}: {} prime numbers", x.0, x.1));
}

fn is_prime(n: &usize) -> bool {
    phi(*n) == *n - 1
}

fn phi(n: usize) -> usize {
    (1..=n).filter(|&x| gcd(n, x) == 1).count()
}
