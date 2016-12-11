use std::io::{self, BufRead};

fn main() {
    let reader = io::stdin();
    let lines = reader.lock().lines().take(2);
    let nums = lines.map(|string| string.unwrap().trim().parse().unwrap())
        .collect::<Vec<i32>>();
    let a = nums[0];
    let b = nums[1];
    if a < b {
        println!("{} is less than {}", a, b)
    } else if a == b {
        println!("{} equals {}", a, b)
    } else if a > b {
        println!("{} is greater than {}", a, b)
    };
}
