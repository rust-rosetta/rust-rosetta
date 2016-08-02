extern crate num;

use num::traits::{PrimInt, Zero};

fn horner<T: PrimInt + Zero>(cs: &[T], x: T) -> T {
    cs.iter().rev().fold(Zero::zero(), |acc: T, c| (acc * x) + (*c))
}

fn main() {
    println!("{}", horner(&[-19i32, 7, -4, 6], 3i32)); // 128
}

#[cfg(test)]
mod tests {
    use super::horner;

    #[test]
    fn test() {
        assert_eq!(horner(&[-19i32, 7, -4, 6], 3i32), 128);
        assert_eq!(horner(&[-1i32, 7, -4, 6], 0i32), -1);
        assert_eq!(horner(&[-0i32, 3], 100i32), 300);
        assert_eq!(horner(&[-20i32, 7, 1], 10i32), 150);
        assert_eq!(horner(&[-19i32, 7, -4, 0], 5i32), -84);
    }
}
