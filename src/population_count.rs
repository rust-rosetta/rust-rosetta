// http://rosettacode.org/wiki/Population_count
#![allow(unstable)]
use std::iter::{count, Filter, Counter, Map};
use std::num::Int;

#[cfg(not(test))]
fn main() {
    fn print_30<T: Iterator<Item=usize>>(it: T) {
        for i in it.take(30) { print!("{} " , i); }
    }

    println!("Pow. of 3");
    print_30(pow_3());

    println!("\nEvil");
    print_30(evil());

    println!("\nOdious");
    print_30(odious());
}

type EvilOdiousIter = Filter<usize, Counter<usize>, fn(&usize) -> bool>;

fn even_ones(i: &usize) -> bool { i.count_ones() % 2 == 0 }

fn odious() -> EvilOdiousIter  {
    fn odds(n: &usize) -> bool { !even_ones(n) }
    count(0us, 1).filter(odds as fn(&usize) -> bool)
}

fn evil() -> EvilOdiousIter {
    count(0us, 1).filter(even_ones as fn(&usize) -> bool)
}

fn pow_3() -> Map<usize, usize, Counter<usize>, fn(usize) -> usize> {
    fn pw(n: usize) -> usize { 3u32.pow(n).count_ones() }

    count(0us, 1).map(pw as fn(usize) -> usize)
}

#[cfg(test)]
mod test {
    use super::{odious, evil, pow_3};
    #[test]
    fn test_odious() {
        let exp = vec![1us, 2, 4, 7, 8, 11, 13, 14, 16, 19, 21, 22,
                        25, 26, 28, 31, 32, 35, 37, 38, 41, 42, 44,
                        47, 49, 50, 52, 55, 56, 59];
        let act = odious().take(30).collect::<Vec<usize>>();
        assert_eq!(act, exp);
    }

    #[test]
    fn test_evil() {
        let exp = vec![0us, 3, 5, 6, 9, 10, 12, 15, 17, 18, 20, 23,
                        24, 27, 29, 30, 33, 34, 36, 39, 40, 43, 45, 46,
                        48, 51, 53, 54, 57, 58];
        let act = evil().take(30).collect::<Vec<usize>>();
        assert_eq!(act, exp);
    }

    #[test]
    fn test_pow_3() {
        let exp = vec![1us, 2, 2, 4, 3, 6, 6, 5, 6, 8, 9, 13, 10,
                        11, 14, 15, 11, 14, 14, 17, 17, 19, 16, 19,
                        14, 14, 18, 21, 18, 15];
        let act = pow_3().take(30).collect::<Vec<usize>>();
        assert_eq!(act, exp);
    }
}
