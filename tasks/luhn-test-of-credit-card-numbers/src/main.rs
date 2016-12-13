struct Digits {
    m: u64,
}

impl Iterator for Digits {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        match self.m {
            0 => None,
            n => {
                let ret = n % 10;
                self.m = n / 10;
                Some(ret)
            }
        }
    }
}

#[derive(Copy, Clone)]
enum LuhnState {
    Even,
    Odd,
}


fn digits(n: u64) -> Digits {
    Digits { m: n }
}

fn luhn_test(n: u64) -> bool {
    let odd_even = [LuhnState::Odd, LuhnState::Even];
    let numbers = digits(n).zip(odd_even.iter().cycle().cloned());
    let sum = numbers.fold(0u64, |s, n| {
        s +
        match n {
            (n, LuhnState::Odd) => n,
            (n, LuhnState::Even) => digits(n * 2).fold(0, |s, n| s + n),
        }
    });
    sum % 10 == 0
}

fn main() {
    let nos = [49927398716, 49927398717, 1234567812345678, 1234567812345670];
    for n in &nos {
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
