#![feature(inclusive_range_syntax)]

const LIMIT: i32 = 12;

fn main() {
    for i in 1..=LIMIT {
        print!("{:3} ", i);
    }
    print!("\n");

    for _ in 0..LIMIT {
        print!("----");
    }
    print!("+\n");

    for i in 1..=LIMIT {
        for j in 1..=LIMIT {
            if j < i {
                print!("    ")
            } else {
                print!("{:3} ", j * i)
            }
        }
        println!("| {}", i);
    }
}
