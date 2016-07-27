extern crate aks_test_for_primes;

fn main() {
    for p in 0..8 {
        println!("{}: {:?}", p, aks_test_for_primes::coefficients(p));
    }

    for p in (1..51).filter(|&x| aks_test_for_primes::is_prime(x)) {
        print!("{} ", p);
    }
}
