//! Uses RPN expression

extern crate rand;

fn main() {
    use rand::Rng;
    use std::io::{self, Write};

    let mut rng = rand::thread_rng();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    // generating 4 numbers
    let choices: Vec<u32> = (0u32..4)
        .map(|_| rng.gen_range(1u32, 10))
        .collect();
    println!("Make 24 with the following numbers");

    // start the game loop
    let mut buffer = String::new();
    loop {
        println!("Your numbers: {}, {}, {}, {}",
                 choices[0],
                 choices[1],
                 choices[2],
                 choices[3]);
        buffer.clear();
        stdin.read_line(&mut buffer).expect("Failed to read line!");
        match check_input(&buffer[..], &choices[..]) {
            Ok(()) => {
                println!("Good job!");
                break;
            }
            Err(e) => println!("{}", e),
        }
        print!("Try again? (y/n): ");
        stdout.flush().unwrap();
        buffer.clear();
        stdin.read_line(&mut buffer).expect("Failed to read line!");
        if buffer.trim() != "y" {
            break;
        }
    }
}

fn check_input(expr: &str, choices: &[u32]) -> Result<(), String> {
    let mut stack: Vec<u32> = Vec::new();
    for token in expr.split_whitespace() {
        if is_operator(token) {
            let (a, b) = (stack.pop(), stack.pop());
            match (a, b) {
                (Some(x), Some(y)) => stack.push(evaluate(y, x, token)),
                (_, _) => return Err("Not a valid RPN expression!".to_string()),
            }
        } else {
            match token.parse::<u32>() {
                Ok(n) => {
                    // check if the number is valid
                    if !choices.contains(&n) {
                        return Err(format!("Cannot use {}", n));
                    }
                    stack.push(n)
                }
                Err(_) => return Err(format!("Invalid input: {}", token)),
            }
        }
    }

    let ans = stack.pop();
    if !stack.is_empty() {
        return Err("Not a valid RPN expression!".to_string());
    }
    match ans {
        Some(x) if x == 24 => Ok(()),
        Some(x) => Err(format!("Wrong answer. Result: {}", x)),
        None => Err("Error encountered!".to_string()),
    }
}

fn evaluate(a: u32, b: u32, op: &str) -> u32 {
    match op {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        _ => unreachable!(),
    }
}

fn is_operator(op: &str) -> bool {
    ["*", "-", "+", "/"].contains(&op)
}

#[cfg(tests)]
mod tests {
    const v1: [u32; 4] = [4u32, 3, 6, 2];

    #[test]
    fn correct_result() {
        assert_eq!(check_input("4 3 * 6 2 * +", &v1), Ok(()));
    }

    #[test]
    fn incorrect_result() {
        assert_eq!(check_input("4 3 * 2 6 + -", &v1),
                   Err("Wrong answer. Result: 4".to_string()));
    }

    #[test]
    fn wrong_numbers_in_input() {
        assert_eq!(check_input("4 5 + 6 2 * -", &v1),
                   Err("Cannot use 5".to_string()));
    }

    #[test]
    fn invalid_chars_in_input() {
        assert_eq!(check_input("4 ) + _ 2 * -", &v1),
                   Err("Invalid input: )".to_string()));
    }

    fn invalid_rpn_expression() {
        assert_eq!(check_input("4 3 + 6 2 *", &v1),
                   Err("Not a valid RPN expression!".to_string()));
    }
}
