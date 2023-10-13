/// Returns the value of the Möbius function at the input.
fn moebius(mut x: u64) -> i8 {
    let mut prime_count = 0;

    // If x is divisible by the given factor this macro counts the factor and divides it out.
    // It then returns zero if x is still divisible by the factor.
    macro_rules! divide_x_by {
        ($factor:expr) => {
            if x % $factor == 0 {
                x /= $factor;
                prime_count += 1;
                if x % $factor == 0 {
                    return 0;
                }
            }
        };
    }

    // Handle 2 and 3 separately, as they are not found by the wheel.
    divide_x_by!(2);
    divide_x_by!(3);

    // Then use the wheel to check the remaining factors <= √x.
    for i in (5..=isqrt(x)).step_by(6) {
        divide_x_by!(i);
        divide_x_by!(i + 2);
    }

    // If x is a prime it will never be divided by any factor <= its square root.
    // In that case we can check if x is still larger than one, and then count it.
    if x > 1 {
        prime_count += 1;
    }

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
        "Values of the Möbius function, μ(x), for x between 0 and {}:",
        COLS * ROWS
    );
    for i in 0..ROWS {
        for j in 0..=COLS {
            let x = COLS * i + j;
            let μ = moebius(x);
            if μ >= 0 {
                // Print an extra space if there's no minus sign in front of the output
                // in order to align the numbers in a nice grid.
                print!(" ");
            }
            print!("{μ}, ");
        }
        println!();
    }
    let x = u64::MAX;
    println!("\nμ({x}) = {}", moebius(x));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_möbius_function_for_small_inputs() {
        #[rustfmt::skip]
        const TEST_CASES: [i8; 51] = [0, 1, -1, -1, 0, -1, 1, -1, 0, 0, 1, -1, 0, -1, 1, 1, 0, -1, 0, -1, 0, 1, 1, -1, 0, 0, 1, 0, 0, -1, -1, -1, 0, 1, 1, 1, 0, -1, 1, 1, 0, -1, -1, -1, 0, 0, 1, -1, 0, 0, 0];
        for (n, ans) in TEST_CASES.into_iter().enumerate() {
            assert_eq!(moebius(n as u64), ans);
        }
    }

    #[test]
    fn verify_möbius_function_for_kilo_inputs() {
        #[rustfmt::skip]
        const KILO_TEST_CASES: [i8; 50] = [0, -1, -1, 1, 0, -1, 1, 1, 0, -1, -1, 1, 0, -1, 0, -1, 0, 0, 1, -1, 0, -1, -1, -1, 0, 0, 0, 1, 0, 0, -1, -1, 0, -1, -1, 0, 0, 1, -1, -1, 0, 1, 1, 1, 0, -1, 1, 1, 0, -1];
        for (n, ans) in KILO_TEST_CASES.into_iter().enumerate() {
            assert_eq!(moebius(n as u64 + 1000), ans);
        }
    }

    // This test passes, but it takes a long time to run (~40 seconds on my laptop)
    // #[test]
    // fn verify_möbius_function_for_enormous_inputs() {
    //     const ENORMOUS_TEST_CASES: [i8; 6] = [-1, 1, 0, -1, 0, -1];
    //     for (n, ans) in ENORMOUS_TEST_CASES.into_iter().enumerate() {
    //         assert_eq!(
    //             moebius(u64::MAX - 5 + n as u64),
    //             ans
    //         );
    //     }
    // }
}
