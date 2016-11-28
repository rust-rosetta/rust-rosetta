use std::io;
use std::io::BufRead;

fn parse_entry(l: &str) -> (i32, String) {
    let params: Vec<&str> = l.split(' ').collect();

    let divisor = params[0].parse::<i32>().unwrap();
    let word = params[1].to_string();
    (divisor, word)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let l = lines.next().unwrap();
    let high = l.parse::<i32>().unwrap();

    let mut entries = Vec::new();
    for l in lines {
        if &l == "" {
            break;
        }
        let entry = parse_entry(&l);
        entries.push(entry);
    }

    for i in 1..(high + 1) {
        let mut line = String::new();
        for &(divisor, ref word) in &entries {
            if i % divisor == 0 {
                line = line + word;
            }
        }
        if line == "" {
            println!("{}", i);
        } else {
            println!("{}", line);
        }
    }
}
