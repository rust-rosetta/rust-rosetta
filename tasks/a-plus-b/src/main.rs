use std::io;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input).unwrap();
    let words = input.split_whitespace()
        .take(2)
        .map(|i| i.parse::<i32>().ok())
        .collect::<Vec<Option<i32>>>();

    let err_msg = "Please enter 2 integers";
    let sum = if words.len() == 2 {
        words[0].expect(err_msg) + words[1].expect(err_msg)
    } else {
        panic!(err_msg);
    };

    println!("{}", sum);
}
