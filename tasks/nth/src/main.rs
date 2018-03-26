fn nth(num: i32) -> String {
    format!(
        "{}{}",
        num,
        match num % 10 {
            1 if num % 100 != 11 => "st",
            2 if num % 100 != 12 => "nd",
            3 if num % 100 != 13 => "rd",
            _ => "th",
        }
    )
}

fn main() {
    let ranges = vec![(0, 26), (250, 266), (1000, 1026)];
    for &(s, e) in &ranges {
        println!("[{}, {}) :", s, e);
        for i in s..e {
            print!("{}, ", nth(i));
        }
        println!();
    }
}
