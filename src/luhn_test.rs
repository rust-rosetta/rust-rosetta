// Implements http://rosettacode.org/wiki/Luhn_test_of_credit_card_numbers

#[deriving(Copy)]
enum LuhnState {
    Even,
    Odd,
}

fn digits(n: u64) -> std::iter::Unfold<'static, u64, u64> {
    std::iter::Unfold::new(n, |state| {
        match *state {
            0 => None,
            n => {
                let ret = n % 10;
                *state = n / 10;
                Some(ret)
            },
        }
    })
}

fn luhn_test(n: u64) -> bool {
    let odd_even = [LuhnState::Odd, LuhnState::Even];
    let numbers = digits(n).zip(odd_even.iter().cycle().map(|&s|s));
    let sum = numbers.fold(0u64, |s,n| {
        s + match n {
            (n, LuhnState::Odd) => n,
            (n, LuhnState::Even) => digits(n*2).fold(0, |s,n|s+n),
        }
    });
    sum % 10 == 0
}

#[cfg(not(test))]
fn main() {
    let nos = [49927398716, 49927398717, 1234567812345678, 1234567812345670];
    for n in nos.iter() {
        if luhn_test(*n) {
            println!("{} passes.", n);
        } else {
            println!("{} fails.", n);
        }
    }
}

#[test]
fn test_inputs() {
    assert!(luhn_test(49927398716));
    assert!(!luhn_test(49927398717));
    assert!(!luhn_test(1234567812345678));
    assert!(luhn_test(1234567812345670));
}
