// Implements http://rosettacode.org/wiki/Arithmetic/Integer
// Implements http://rosettacode.org/wiki/A%2BB
use std::from_str::from_str;
#[cfg(not(test))]
use std::io::stdio::stdin;

#[cfg(not(test))]
use std::io::BufferedReader;

#[cfg(not(test))]
fn main() {
    let input = BufferedReader::new(stdin()).read_line().unwrap();
    let sum = extract_ints(input.as_slice());

    let (a, b) = match sum {
        Some(s) => s,
        None => fail!("Please enter 2 integers")
    };

    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * b = {}", a * b);
    println!("a / b = {}", a / b);
    println!("a % b = {}", a % b);
}

fn extract_ints(input: &str) -> Option<(int, int)> {
    let words = input.words().take(2)
                            .map(from_str::<int>)
                            .collect::<Vec<Option<int>>>();

    match words.as_slice() {
        [Some(x), Some(y)] => Some((x, y)),
        _ => None
    }
}

#[test]
fn test_extract() {
    assert_eq!(extract_ints("40 2").unwrap(), (40, 2));
}