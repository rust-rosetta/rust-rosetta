use itertools::Itertools;

const PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
const MAX_DIVISOR: usize = 64;

struct DivisorSeq {
    max_number_of_divisors: u64,
    index: u64,
}

impl DivisorSeq {
    fn new(max_number_of_divisors: u64) -> DivisorSeq {
        DivisorSeq {
            max_number_of_divisors,
            index: 1,
        }
    }
}

impl Iterator for DivisorSeq {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.max_number_of_divisors < self.index {
            return None;
        }
        #[allow(unused_mut)]
        let mut result: u64;
        let divisors_of_divisor = get_divisors(self.index);
        match divisors_of_divisor.len() {
            1 | 2 => {
                // when # divisors is a prime
                result = 2_u64.pow(self.index as u32 - 1);
                self.index += 1;
            }
            3 => {
                // when # divisors is a prime square
                result = 6_u64.pow(divisors_of_divisor[1] as u32 - 1);
                self.index += 1;
            }
            4 => {
                // when # divisors is a prime * non-prime
                result = 2_u64.pow(divisors_of_divisor[2] as u32 - 1)
                    * 3_u64.pow(divisors_of_divisor[1] as u32 - 1);
                self.index += 1;
            }
            8 if divisors_of_divisor
                .iter()
                .filter(|x| PRIMES.contains(x))
                .count()
                == 3 =>
            {
                // sphenic numbers, aka p*m*q, where, p, m and q are prime
                let first_primes = divisors_of_divisor
                    .iter()
                    .filter(|x| PRIMES.contains(x))
                    .collect::<Vec<_>>();
                result = 2_u64.pow(*first_primes[2] as u32 - 1)
                    * 3_u64.pow(*first_primes[1] as u32 - 1)
                    * 5_u64.pow(*first_primes[0] as u32 - 1);
                self.index += 1;
            }
            _ => {
                // brute force and slow: iterates over the numbers to find
                // one with the appropriate number of divisors
                let mut x: u64 = 1;
                loop {
                    if get_divisors(x).len() as u64 == self.index {
                        break;
                    }
                    x += 1;
                }

                result = x;
                self.index += 1;
            }
        }
        Some(result)
    }
}
/// Gets all divisors of a number
fn get_divisors(n: u64) -> Vec<u64> {
    let mut results = Vec::new();

    for i in 1..(n / 2 + 1) {
        if n % i == 0 {
            results.push(i);
        }
    }
    results.push(n);
    results
}

fn main() {
    // simple version using factorizing numbers
    // with rules applied from A005179 so speed up
    // but as such it is slow in some cases, e.g for 52
    let seq_iter = DivisorSeq::new(64);
    println!("Simple method with rules");
    println!("# divisors     Smallest number");
    for (i, x) in seq_iter.enumerate() {
        println!("{:>10}{:20}", i + 1, x);
    }

    // more advanced version using calculations based on number of
    // prime factors and their exponent

    // load initial result table with an initial value of 2**n for each item
    let mut min_numbers = vec![0_u64; MAX_DIVISOR];
    (0_usize..MAX_DIVISOR).for_each(|n| min_numbers[n] = 2_u64.pow(n as u32));

    let prime_list = (1..15).map(|i| PRIMES[0..=i].to_vec()).collect::<Vec<_>>();

    for pl in prime_list.iter() {
        // calculate the max exponent a prime can get in a given prime-list
        // to be able to produce the desired number of divisors
        let max_exponent = 1 + MAX_DIVISOR as u32 / 2_u32.pow(pl.len() as u32 - 1);

        // create a combination of exponents using cartesian product
        let exponents = (1_usize..=pl.len())
            .map(|_| 1_u32..=max_exponent)
            .multi_cartesian_product()
            .filter(|elt| {
                let mut prev = None::<&u32>;
                elt.iter().all(|x| match prev {
                    Some(n) if x > n => false,
                    _ => {
                        prev = Some(x);
                        true
                    }
                })
            });

        // iterate throught he exponent combinations
        for exp in exponents {
            // calculate the number of divisors using the formula
            // given primes of p, q, r
            // and exponents of a1, a2, a3
            // the # divisors is (a1+1)* (a2+1) *(a3+1)
            let num_of_divisors = exp.iter().map(|x| x + 1).product::<u32>() as usize;

            // and calculate the number with those primes and the given exponent set
            let num = pl.iter().zip(exp.iter()).fold(1_u64, |mut acc, (p, e)| {
                // check for overflow if numbers won't fit into u64
                acc = match acc.checked_mul(p.pow(*e)) {
                    Some(z) => z,
                    _ => 0,
                };
                acc
            });

            // finally, if the number is less than what we have so far in the result table
            // replace the result table with the smaller number
            if num > 0
                && min_numbers.len() >= num_of_divisors
                && min_numbers[num_of_divisors - 1] > num
            {
                min_numbers[num_of_divisors - 1] = num;
            }
        }
    }

    println!("Advanced method");
    println!("# divisors     Smallest number");
    for (i, x) in min_numbers.iter().enumerate() {
        println!("{:>10}{:20}", i + 1, x);
    }
}
