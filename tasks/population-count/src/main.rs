use std::iter::{Filter, Map};
use std::ops::RangeFrom;

fn main() {
    fn print_30<T: Iterator<Item = u32>>(it: T) {
        for i in it.take(30) {
            print!("{} ", i);
        }
    }

    println!("Pow. of 3");
    print_30(pow_3());

    println!("\nEvil");
    print_30(evil());

    println!("\nOdious");
    print_30(odious());
}

type EvilOdiousIter = Filter<RangeFrom<u32>, fn(&u32) -> bool>;

fn even_ones(i: &u32) -> bool {
    i.count_ones() % 2 == 0
}

fn odious() -> EvilOdiousIter {
    fn odds(n: &u32) -> bool {
        !even_ones(n)
    }
    (0..).filter(odds as fn(&u32) -> bool)
}

fn evil() -> EvilOdiousIter {
    (0..).filter(even_ones as fn(&u32) -> bool)
}

fn pow_3() -> Map<RangeFrom<u32>, fn(u32) -> u32> {
    fn pw(n: u32) -> u32 {
        3u64.pow(n).count_ones()
    }

    (0..).map(pw as fn(u32) -> u32)
}

#[cfg(test)]
mod tests {
    use super::{odious, evil, pow_3};
    #[test]
    fn test_odious() {
        let exp = vec![1, 2, 4, 7, 8, 11, 13, 14, 16, 19, 21, 22, 25, 26, 28, 31, 32, 35, 37, 38,
                       41, 42, 44, 47, 49, 50, 52, 55, 56, 59];
        let act = odious().take(30).collect::<Vec<u32>>();
        assert_eq!(act, exp);
    }

    #[test]
    fn test_evil() {
        let exp = vec![0, 3, 5, 6, 9, 10, 12, 15, 17, 18, 20, 23, 24, 27, 29, 30, 33, 34, 36, 39,
                       40, 43, 45, 46, 48, 51, 53, 54, 57, 58];
        let act = evil().take(30).collect::<Vec<u32>>();
        assert_eq!(act, exp);
    }

    #[test]
    fn test_pow_3() {
        let exp = vec![1, 2, 2, 4, 3, 6, 6, 5, 6, 8, 9, 13, 10, 11, 14, 15, 11, 14, 14, 17, 17,
                       20, 19, 22, 16, 18, 24, 30, 25, 25];
        let act = pow_3().take(30).collect::<Vec<u32>>();
        assert_eq!(act, exp);
    }
}
