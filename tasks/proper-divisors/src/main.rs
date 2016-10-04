/// Populate input vector with prime numbers < maxvalue
fn add_more_prime_numbers(v: &mut Vec<usize>, maxvalue: usize) {
    let mut prime: usize = v[v.len() - 1];
    if prime <= 2 {
        prime = 1;
    } // start with odd number
    loop {
        prime += 2;
        if prime >= maxvalue {
            break;
        }
        let mut isprime = true;
        let ceiling = (prime as f64).sqrt() as usize;
        // check below sqrt(prime)
        for i in v.iter().skip(1).take_while(|&i| *i <= ceiling) {
            if prime % *i == 0 {
                isprime = false;
                break;
            }
        }
        if isprime {
            v.push(prime);
        }
    }
}

/// Get proper divisors
fn find_divisors(primes: &mut Vec<usize>, num: usize) -> Vec<usize> {
    assert!(num > 0);
    if num == 1 {
        return Vec::new();
    }
    let mut kprime_factors = vec![1];
    let ceiling = ((num as f64).sqrt() as usize) + 1;
    add_more_prime_numbers(primes, ceiling);
    // Filter all primes num % p == 0
    let prime_factors: Vec<usize> = primes.iter()
        .filter(|&p| num % *p == 0)
        .cloned()
        .collect();
    // Check all k*p p..ceiling
    // Following code is ineffective (due to duplicates) but simple
    for p in prime_factors {
        let mut kp = p;
        while kp < ceiling {
            if num % kp == 0 {
                kprime_factors.push(kp);
                kprime_factors.push(num / kp)
            }
            kp += p;
        }
    }
    kprime_factors.sort();
    kprime_factors.dedup();
    kprime_factors
}

fn main() {
    let mut primes: Vec<usize> = vec![2, 3]; // reusable prime number array

    // Show the proper divisors of the numbers 1 to 10 inclusive.
    for i in 1..11 {
        print!("{}: ", i);
        println!("{:?}", find_divisors(&mut primes, i));
    }

    // Find a number in the range 1 to 20,000
    // with the most proper divisors.
    let mut max_divs: (usize, Vec<usize>) = (0, Vec::new());
    for n in 1..20001 {
        let div_q = find_divisors(&mut primes, n).len();
        if div_q > max_divs.0 {
            max_divs.0 = div_q;
            max_divs.1.clear();
            max_divs.1.push(n);
        } else if div_q == max_divs.0 {
            max_divs.1.push(n);
        }
    }
    println!("Most divisors a number within 1 to 20000 has: {}",
             max_divs.0);
    print!("Numbers with {} divisors: ", max_divs.0);
    println!("{:?}", max_divs.1);
}

#[test]
fn test_divisors() {
    fn proper_divisors(num: usize) -> Vec<usize> {
        let mut primes: Vec<usize> = vec![2, 3];
        find_divisors(&mut primes, num)
    }

    assert!(proper_divisors(6) == vec![1, 2, 3]);
    assert!(proper_divisors(100) == vec![1, 2, 4, 5, 10, 20, 25, 50]);
}
