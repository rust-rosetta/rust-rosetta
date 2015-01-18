// http://rosettacode.org/wiki/Harshad_or_Niven_series
#![allow(unstable)]
use std::usize;
fn main() {
    let digit_sum = |&: i: usize| i.to_string().chars()
        .fold(0us, |d, c| d + c.to_digit(10).unwrap());
    let mut harshads = (1us..usize::MAX).filter(|&n| n % digit_sum(n) == 0);

    for _ in 0us..20 { print!("{} ", harshads.next().unwrap()) }
    println!("\n{}", harshads.skip_while(|&h| h <= 1000).next().unwrap());
}
