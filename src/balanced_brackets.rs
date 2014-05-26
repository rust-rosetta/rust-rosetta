// Implements http://rosettacode.org/wiki/Balanced_brackets

extern crate rand;

#[cfg(not(test))]
use rand::random;

// Returns true if the brackets are balanced
fn check_balanced(bracket_str: &str) -> bool {
    let mut count = 0;

    for bracket in bracket_str.chars() {
        match bracket {
            '[' => {
                count += 1;
            }
            ']' => {
                count -= 1;
                if count < 0 {
                    return false;
                }
            }
            _ => { fail!("Strings containing characters other than brackets are not allowed"); }
        }
    }

    count == 0
}

// Generates random brackets
#[cfg(not(test))]
fn generate_brackets(num: uint) -> String {
    let mut brackets = String::new();

    for _ in range(0, num) {
        if random() {
            brackets.push_char('[')
        } else {
            brackets.push_char(']')
        }
    }

    brackets
}

#[cfg(not(test))]
fn main() {
    for i in range (0u, 10u)
    {
        let brackets = generate_brackets(i);
        let balanced = check_balanced(brackets.as_slice());

        println!("{:s}    {:b}", brackets, balanced)
    }
}

#[test]
fn test_empty_string() {
    assert!(check_balanced(""));
}

#[test]
fn test_wrong_brackets() {
    assert!(!check_balanced("]["));
    assert!(!check_balanced("][]["));
    assert!(!check_balanced("[]][[]"));
}

#[test]
fn test_good_brackets() {
    assert!(check_balanced("[]"));
    assert!(check_balanced("[][]"));
    assert!(check_balanced("[[][]]"));
}
