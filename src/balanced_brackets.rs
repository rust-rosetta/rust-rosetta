// Implements http://rosettacode.org/wiki/Balanced_brackets

extern crate rand;

use rand::random;

fn check_balanced(bracket_str: &str) -> bool {
    let mut count: int = 0;
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
            _ => { fail!(); }
        }
    }
    return count == 0;
}

fn generate_brackets(mut num: uint) -> StrBuf {
    let mut brackets = StrBuf::new();
    while num > 0 {
        num -= 1;
        if random() {
            brackets.push_char('[')
        } else {
            brackets.push_char(']')
        }
    }
    brackets
}

fn main() {
    for i in range (0u, 10u)
    {
        let brackets = generate_brackets(i);
        let balanced = check_balanced(brackets.as_slice());

        println!("{:s}    {:b}", brackets, balanced)
    }
}
