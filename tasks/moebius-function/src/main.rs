use core::num::NonZeroU64;

/// Returns the value of the Möbius function at the input.
const fn moebius(x: NonZeroU64) -> i8 {
    let mut x = x.get();
    let mut prime_count = 0;

    // This macro lets us avoid some code repetition when we implement the wheel factorization.
    macro_rules! handle_factor {
        ($factor:expr) => {
            if x % $factor == 0 {
                // If the input is divisible by the factor we divide it out
                x /= $factor;
                // count it as a prime
                prime_count += 1;
                // and check if the input is still divisible by the factor
                if x % $factor == 0 {
                    // if it is we return 0
                    return 0;
                }
            }
        };
    }

    // Handle 2 and 3 separately
    handle_factor!(2);
    handle_factor!(3);

    // Then use a wheel to check the remaining factors <= √x
    let mut i = 5;
    let bound = isqrt(x);
    while i <= bound {
        handle_factor!(i);
        handle_factor!(i + 2);
        i += 6;
    }

    // If x is a prime it will never be divided by any factor <= its square root.
    // In that case we can check if x is still larger than one, and then count it.
    if x > 1 {
        prime_count += 1;
    }

    // Return 1 or -1 depending on whether `x` has an even or odd number of prime factors.
    if prime_count % 2 == 0 {
        1
    } else {
        -1
    }
}

/// Computes the integer square root of `n` through a binary search.
/// 
/// This is the integer `i` such that `i^2 <= n < (i + 1)^2`.
const fn isqrt(n: u64) -> u64 {
    if n == u64::MAX {
        return 4_294_967_296;
    }

    let mut left = 0;
    let mut right = n + 1;

    while left != right - 1 {
        let mid = left + (right - left) / 2;
        if mid as u128 * mid as u128 <= n as u128 {
            left = mid;
        } else {
            right = mid;
        }
    }

    left
}

fn main() {
    const ROWS: u64 = 10;
    const COLS: u64 = 20;
    println!(
        "Values of the Möbius function, μ(x), for x between 1 and {}:",
        ROWS * COLS
    );
    for i in 0..=ROWS {
        for j in 0..=COLS {
            let x = NonZeroU64::new(i + j + 1).unwrap();
            let μ = moebius(x);
            if μ >= 0 {
                print!(" ");
            }
            print!("{μ}, ");
        }
        println!();
    }
    let x = NonZeroU64::new(u64::MAX).unwrap();
    println!("\nμ({x}) = {}", moebius(x));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_möbius_function_for_small_inputs() {
        #[rustfmt::skip]
        const TEST_CASES: [i8; 50] = [1, -1, -1, 0, -1, 1, -1, 0, 0, 1, -1, 0, -1, 1, 1, 0, -1, 0, -1, 0, 1, 1, -1, 0, 0, 1, 0, 0, -1, -1, -1, 0, 1, 1, 1, 0, -1, 1, 1, 0, -1, -1, -1, 0, 0, 1, -1, 0, 0, 0];
        for (n, ans) in TEST_CASES.into_iter().enumerate() {
            assert_eq!(moebius(NonZeroU64::new(n as u64 + 1).unwrap()), ans);
        }
    }

    #[test]
    fn verify_möbius_function_for_kilo_inputs() {
        #[rustfmt::skip]
        const KILO_TEST_CASES: [i8; 50] = [0, -1, -1, 1, 0, -1, 1, 1, 0, -1, -1, 1, 0, -1, 0, -1, 0, 0, 1, -1, 0, -1, -1, -1, 0, 0, 0, 1, 0, 0, -1, -1, 0, -1, -1, 0, 0, 1, -1, -1, 0, 1, 1, 1, 0, -1, 1, 1, 0, -1];
        for (n, ans) in KILO_TEST_CASES.into_iter().enumerate() {
            assert_eq!(moebius(NonZeroU64::new(n as u64 + 1000).unwrap()), ans);
        }
    }

    // This test passes, but it takes a long time to run (~40 seconds on my laptop)
    // #[test]
    // fn verify_möbius_function_for_enormous_inputs() {
    //     const ENORMOUS_TEST_CASES: [i8; 6] = [-1, 1, 0, -1, 0, -1];
    //     for (n, ans) in ENORMOUS_TEST_CASES.into_iter().enumerate() {
    //         assert_eq!(
    //             moebius(NonZeroU64::new(u64::MAX - 5 + n as u64).unwrap()),
    //             ans
    //         );
    //     }
    // }
}
