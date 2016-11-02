fn nth(num: i32) -> String {
    format!("{}{}",
            num,
            match (num % 10, num % 100) {
                (1, 11) => "th",
                (1, _) => "st",
                (2, 12) => "th",
                (2, _) => "nd",
                (3, 13) => "th",
                (3, _) => "rd",
                _ => "th",
            })
}

fn main() {
    let ranges = vec![(0, 26), (250, 266), (1000, 1026)];
    for &(s, e) in &ranges {
        println!("[{}, {}) :", s, e);
        for i in s..e {
            print!("{}, ", nth(i));
        }
        println!("");
    }
}
