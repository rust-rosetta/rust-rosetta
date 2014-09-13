// Implements http://rosettacode.org/wiki/24_game
// Uses RPN expression 
// Runs with rustc 0.12.0-pre-nightly (2e92c67dc 2014-08-28 23:56:20 +0000)

#[cfg(not(test))]
fn main() {
    use std::rand::{task_rng, Rng};
    use std::io;

    // generating 4 numbers
    let mut rng = task_rng();
    let choices: Vec<uint> = range(1u, 5).map(|_| rng.gen_range(1u, 10)).collect();
    println!("Make 24 with the following numbers");
    
    // start the game loop
    loop {
        print!("Your numbers: {}, {}, {}, {}\n", choices[0], choices[1], choices[2], choices[3]);
        let expr = io::stdin().read_line().ok().expect("Failed to read line!");
        if check_input(expr, &choices) { break; }
        print!("Try again? (y/n): ");
        let choice = io::stdin().read_line().ok().expect("Failed to read line!");
        if choice.as_slice().trim() != "y" { break; }
    }
}

fn check_input(expr: String, choices: &Vec<uint>) -> bool {
    let mut stack: Vec<uint> = Vec::new();
    for token in expr.as_slice().words() {
        if is_operator(token) {
            let (a, b) = (stack.pop(), stack.pop());
            match (a, b) {
                (Some(x), Some(y)) => stack.push(evaluate(y, x, token)),
                (_, _) => { 
                    println!("Not a valid RPN expression!"); 
                    return false ;
                }
            }
        } else {
            let v: Option<uint> = from_str(token);
            match v {
                Some(n) => {
                    if !is_valid_number(n, choices) { 
                        println!("Cannot use {}", n); 
                        return false; 
                    }
                    stack.push(n) 
                },
                None => { 
                    println!("Invalid input: {}", token); 
                    return false; 
                }
            }
        }
    }

    let ans = stack.pop();
    if stack.len() > 0 {
        println!("Not a valid RPN expression!");
        return false;
    }
    match ans {
        Some(x) => {
            if x == 24 { 
                println!("Good job!"); 
                return true;
            }
            println!("Wrong answer. Result: {}", x);
        }
        None => println!("Error encountered!"),
    }
    false
}

// since evaluate is wrapped in is_operator the last
// pattern has to be "/"
fn evaluate(a: uint, b: uint, op: &str) -> uint {
    match op {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
         _  => a / b, // has to be /
    }
}

fn is_operator(op: &str) -> bool {
    match op {
        "*" | "-" | "+" | "/" => true,
        _                     => false
    }
}

fn is_valid_number(n: uint, choices: &Vec<uint>) -> bool {
    for i in choices.iter() {
        if n == *i { return true } 
    }
    false
}

#[test]
fn test_check_input() {
    let v1: Vec<uint> = vec![4, 3, 6, 2];

    // correct result
    let result1 = check_input("4 3 * 6 2 * +".to_string(), &v1);
    assert!(result1);

    // incorrect result
    let result2 = check_input("4 3 + 2 6 + -".to_string(), &v1);
    assert!(!result2);
    
    // wrong numbers in input
    let result3 = check_input("4 5 + 6 2 * -".to_string(), &v1);
    assert!(!result3);

    // invalid chars in input
    let result4 = check_input("4 ) + _ 2 * -".to_string(), &v1);
    assert!(!result4);

    // invalid RPN expression
    let result5 = check_input("4 3 + 6 2 *".to_string(), &v1);
    assert!(!result5);
}
