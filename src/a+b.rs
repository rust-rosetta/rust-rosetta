// Implements http://rosettacode.org/wiki/A%2BB
use std::from_str::from_str;
#[cfg(not(test))]
use std::io::stdio::stdin;

#[cfg(not(test))]
use std::io::BufferedReader;

#[cfg(not(test))]
fn main() {
    let input = BufferedReader::new(stdin()).read_line().unwrap();
    let sum = sum_words(input.as_slice());

    let sum = match sum {
        Some(s) => s,
        None    => fail!("Please enter 2 integers")
    };

    println!("{:i}", sum);
}

fn sum_words(input: &str) -> Option<int> {
    let words = input.words().take(2)
                            .map(from_str::<int>)
                            .collect::<Vec<Option<int>>>();

    match words.as_slice() {
        [Some(x), Some(y)] => Some(x + y),
		_                  => None
    }
}

#[test]
fn test_sum() {
    assert_eq!(sum_words("40 2").unwrap(), 42);
}