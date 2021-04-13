use num::BigUint;
use num::CheckedSub;
use num_traits::{One, Zero};

fn isqrt(x: &BigUint) -> BigUint {
    let mut q: BigUint = One::one();
    while q <= *x {
        q <<= &2;
    }

    let mut z = x.clone();
    let mut r: BigUint = Zero::zero();

    while q > One::one() {
        q >>= &2;
        let t = z.checked_sub(&r).and_then(|diff| diff.checked_sub(&q));
        r >>= &1;

        match t {
            Some(t) => {
                z = t;
                r += &q;
            }
            None => (),
        }
    }

    r
}

fn with_thousand_separator(s: String) -> String {
    let digits: Vec<_> = s.chars().rev().collect();
    let chunks: Vec<_> = digits
        .chunks(3)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect();

    chunks.join(",").chars().rev().collect::<String>()
}

fn main() {
    println!("The integer square roots of integers from 0 to 65 are:");
    (0_u32..=65).for_each(|n| print!("{} ", isqrt(&n.into())));

    println!("\nThe integer square roots of odd powers of 7 from 7^1 up to 7^74 are:");
    (1_u32..75).step_by(2).for_each(|exp| {
        println!(
            "7^{:>2}={:>83} ISQRT: {:>42} ",
            exp,
            with_thousand_separator(BigUint::from(7_u8).pow(exp).to_string()),
            with_thousand_separator(isqrt(&BigUint::from(7_u8).pow(exp)).to_string())
        )
    });
}

#[cfg(test)]
mod tests {
    use num::Num;

    use super::*;

    #[test]
    fn test_isqrt() {
        assert_eq!(isqrt(&0_u32.into()), BigUint::from(0_u32));
        assert_eq!(isqrt(&1_u32.into()), BigUint::from(1_u32));
        assert_eq!(isqrt(&2_u32.into()), BigUint::from(1_u32));
        assert_eq!(isqrt(&4_u32.into()), BigUint::from(2_u32));
        assert_eq!(
            isqrt(&BigUint::from_str_radix("11398895185373143", 10).unwrap()),
            BigUint::from_str_radix("106765608", 10).unwrap()
        );
    }
}
