// Implements http://rosettacode.org/wiki/Luhn_test_of_credit_card_numbers
#![feature(core)]

use std::iter::Unfold;

#[derive(Copy)]
enum LuhnState { Even, Odd, }

type Digits = Unfold<u64, fn(&mut u64) -> Option<u64>>;

fn digits(n: u64) -> Digits {
    fn state(s: &mut u64) -> Option<u64> {
       match *s {
           0 => None,
           n => {
               let ret = n % 10;
               *s = n / 10;
               Some(ret)
           }
       }
    }
    Unfold::new(n, state as fn(&mut u64) -> Option<u64>)
}

fn luhn_test(n: u64) -> bool {
    let odd_even = [LuhnState::Odd, LuhnState::Even];
    let numbers = digits(n).zip(odd_even.iter().cycle().map(|&s| s));
    let sum =
        numbers.fold(0u64, |s, n| {
                     s +
                         match n {
                             (n, LuhnState::Odd) => n,
                             (n, LuhnState::Even) =>
                             digits(n * 2).fold(0, |s, n| s + n),
                         } });
    sum % 10 == 0
}

#[cfg(not(test))]
fn main() {
    let nos = [49927398716, 49927398717, 1234567812345678, 1234567812345670];
    for n in &nos {
        if luhn_test(*n) {
            println!("{} passes." , n);
        } else { println!("{} fails." , n); }
    }
}

#[test]
fn test_inputs() {
    assert!(luhn_test(49927398716));
    assert!(!luhn_test(49927398717));
    assert!(!luhn_test(1234567812345678));
    assert!(luhn_test(1234567812345670));
}
