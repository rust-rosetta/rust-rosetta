// http://rosettacode.org/wiki/Population_count
use std::num::Int;

fn is_evil(i: &uint) -> bool { i.count_ones() % 2 == 0 }
fn is_odious(i: &uint) -> bool { ! is_evil(i) }
fn pow_3_ones(i: uint) -> uint { 3u32.pow(i).count_ones() } 

#[cfg(not(test))]
fn main() {
    fn print_30<T: Iterator<uint>>(it: T) {
        for i in it.take(30) {
            print!("{} ", i);
        }
    }

    println!("Pow. of 3");
    print_30(count(0u, 1).map(pow_3_ones));

    println!("\nEvil");
    print_30(count(0u,1).filter(is_evil));

    println!("\nOdious");
    print_30(count(0u,1).filter(is_odious));
}

#[cfg(test)]
mod test {
    use super::{is_evil, is_odious, pow_3_ones};
    use std::iter::count;

    #[test]
    fn test_odious() {
        let exp = vec![1u, 2, 4, 7, 8, 11, 13, 14, 16, 19, 21, 22,
                       25, 26, 28, 31, 32, 35, 37, 38, 41, 42, 44,
                       47, 49, 50, 52, 55, 56, 59];

        let act = count(0u,1).filter(is_odious).take(30).collect::<Vec<uint>>();
        assert_eq!(act, exp);
    }

    #[test]
    fn test_evil() {
        let exp = vec![0u, 3, 5, 6, 9, 10, 12, 15, 17, 18, 20, 23,
                       24, 27, 29, 30, 33, 34, 36, 39, 40, 43, 45, 46,
                       48, 51, 53, 54, 57, 58];

        let act = count(0u,1).filter(is_evil).take(30).collect::<Vec<uint>>();
        assert_eq!(act, exp);
    }

    #[test]
    fn test_pow_3() {
        let exp = vec![1u, 2, 2, 4, 3, 6, 6, 5, 6, 8, 9, 13, 10,
                       11, 14, 15, 11, 14, 14, 17, 17, 19, 16, 19,
                       14, 14, 18, 21, 18, 15];

        let act = count(0u, 1).map(pow_3_ones).take(30).collect::<Vec<uint>>();
        assert_eq!(act, exp);
    }
}
