// Implements http://rosettacode.org/wiki/24_game
// Uses RPN expression
#![allow(unused_features)] // feature(rand) is used only in main
#![feature(str_words)]
#![feature(old_io)]

extern crate rand;

#[cfg(not(test))]
fn main() {
    use rand::{thread_rng, Rng};
    use std::old_io;

    let mut rng = thread_rng();
    let mut reader = old_io::stdin();

    // generating 4 numbers
    let choices: Vec<u32> = (0u32..4).map(
		|_| rng.gen_range(1u32, 10)
    ).collect();
    println!("Make 24 with the following numbers");

    // start the game loop
    loop {
        print!("Your numbers: {}, {}, {}, {}\n", choices[0], choices[1], choices[2], choices[3]);
        let expr = reader.read_line().ok().expect("Failed to read line!");
        match check_input(&expr[..], &choices[..]) {
            Ok(()) => { println!("Good job!"); break; },
            Err(e) => println!("{}", e)
        }
        print!("Try again? (y/n): ");
        let choice = reader.read_line().ok().expect("Failed to read line!");
        if choice.trim() != "y" { break; }
    }
}

fn check_input(expr: &str, choices: &[u32]) -> Result<(), String> {
    let mut stack: Vec<u32> = Vec::new();
    for token in expr.words() {
        if is_operator(token) {
            let (a, b) = (stack.pop(), stack.pop());
            match (a, b) {
                (Some(x), Some(y)) => stack.push(evaluate(y, x, token)),
                (_, _) => return Err("Not a valid RPN expression!".to_string())
            }
        } else {
            match token.parse::<u32>() {
                Ok(n) => {
                    // check if the number is valid
                    if !choices.contains(&n) {
                        return Err(format!("Cannot use {}", n));
                    }
                    stack.push(n)
                },
                Err(_) => return Err(format!("Invalid input: {}", token))
            }
        }
    }

    let ans = stack.pop();
    if stack.len() > 0 {
        return Err("Not a valid RPN expression!".to_string());
    }
    match ans {
        Some(x) => {
            if x == 24 { return Ok(()); }
            return Err(format!("Wrong answer. Result: {}", x));
        }
        None => return Err("Error encountered!".to_string()),
    }
}

fn evaluate(a: u32, b: u32, op: &str) -> u32 {
    match op {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        _   => unreachable!()
    }
}

fn is_operator(op: &str) -> bool {
    ["*", "-", "+", "/"].contains(&op)
}

#[test]
fn test_check_input() {
    let v1 = [4u32, 3, 6, 2];

    // correct result
    assert_eq!(check_input("4 3 * 6 2 * +", &v1), Ok(()));

    // incorrect result
    assert_eq!(check_input("4 3 * 2 6 + -", &v1), Err("Wrong answer. Result: 4".to_string()));

    // wrong numbers in input
    assert_eq!(check_input("4 5 + 6 2 * -", &v1), Err("Cannot use 5".to_string()));

    // invalid chars in input
    assert_eq!(check_input("4 ) + _ 2 * -", &v1), Err("Invalid input: )".to_string()));

    // invalid RPN expression
    assert_eq!(check_input("4 3 + 6 2 *", &v1), Err("Not a valid RPN expression!".to_string()));
}
