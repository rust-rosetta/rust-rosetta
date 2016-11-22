use std::u32;

fn main() {
    let digit_sum = |i: u32| {
        i.to_string()
            .chars()
            .fold(0u32, |d, c| d + c.to_digit(10).unwrap())
    };
    let mut harshads = (1u32..u32::MAX).filter(|&n| n % digit_sum(n) == 0);

    for _ in 0u32..20 {
        print!("{} ", harshads.next().unwrap())
    }
    println!("\n{}", harshads.skip_while(|&h| h <= 1000).next().unwrap());
}
