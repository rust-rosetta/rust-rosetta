//! These are two naive solutions, one with lots of redundant calculations (memoizationless
//! recursion) and one with a few precomputed values. All digit square sums are no greater than 648
//! for numbers < 100,000,000.
//!
//! Both are slow algorithms, however, Rust is among faster languages, so this doesn't take minutes
//! or hours.

fn digit_square_sum(mut num: usize) -> usize {
    let mut sum = 0;
    while num != 0 {
        sum += (num % 10).pow(2);
        num /= 10;
    }
    sum
}

fn last_in_chain(num: usize) -> usize {
    match num {
        0 => 0,
        1 | 89 => num,
        _ => last_in_chain(digit_square_sum(num)),
    }
}

fn main() {
    let count = (1..100_000_000).filter(|&n| last_in_chain(n) == 89).count();
    println!("{}", count);

    let precomputed = (0..649).map(last_in_chain).collect::<Vec<_>>();
    let count = (1..100_000_000).filter(|&n| precomputed[digit_square_sum(n)] == 89).count();
    println!("{}", count);
}

/// Ignore these tests because they're pretty expensive on a non-release build.
#[cfg(test)]
mod tests {
    use super::{digit_square_sum, last_in_chain};

    #[test]
    #[ignore]
    fn naive() {
        let count = (1..100_000_000).filter(|&n| last_in_chain(n) == 89).count();
        assert_eq!(count, 85744333);
    }

    #[test]
    #[ignore]
    fn precomputation() {
        let precomputed = (0..649).map(last_in_chain).collect::<Vec<_>>();
        let count = (1..100_000_000).filter(|&n| precomputed[digit_square_sum(n)] == 89).count();
        assert_eq!(count, 85744333);
    }
}
