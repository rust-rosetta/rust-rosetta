// Implements http://rosettacode.org/wiki/A%2BB
// not_tested

use std::io::stdio;

fn main() {
    let input = stdio::stdin().read_line().unwrap();
    let words = input.as_slice().words().take(2)
                            .map(from_str::<int>)
                            .collect::<Vec<Option<int>>>();

    let sum = match words.as_slice() {
        [Some(x), Some(y)] => x + y,
            _ => fail!("Please enter 2 integers")
    };

    println!("{:i}", sum);
}
