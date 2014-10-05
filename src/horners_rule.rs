use std::num::zero;

fn horner<T:Num>(cs:&[T], x:T) -> T {
    cs.iter().rev().fold(zero::<T>(), |acc, c| (acc*x) + (*c))
}
 
fn main() {
    println!("{}", horner([-19i, 7, -4, 6], 3i)); // 128
}

#[cfg(test)]
mod test {
	use super::horner;
    #[test]
    fn test() {
        assert_eq!(horner([-19i, 7, -4, 6], 3i), 128);
        assert_eq!(horner([-1i, 7, -4, 6], 0i), -1);
        assert_eq!(horner([-0i, 3], 100i), 300);
        assert_eq!(horner([-20i, 7, 1], 10i), 150);
        assert_eq!(horner([-19i, 7, -4, 0], 5i), -84);
    }
}
