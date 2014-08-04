// http://rosettacode.org/wiki/Harshad_or_Niven_series
// not_tested
use std::uint;
fn main() {
    let digit_sum = |i: uint| i.to_string().as_slice().chars()
        .fold(0u, |d, c| d + c.to_digit(10).unwrap());
    let mut harshads = range(1u, uint::MAX).filter(|&n| n % digit_sum(n) == 0);

    for i in range(0u, 20) { print!("{} ", harshads.next().unwrap()) }
    println!("\n{}", harshads.skip_while(|&h| h <= 1000).next().unwrap());
}
