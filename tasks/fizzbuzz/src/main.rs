// Using conditional expressions,
// checking 15 divisibility avoided so negligibly faster
fn fb_one() {
    let mut f: bool;
    for x in 1..101 {
        f = false;
        if x % 3 == 0 {
            print!("Fizz");
            f = true;
        }
        if x % 5 == 0 {
            print!("Buzz");
            f = true;
        }
        if f {
            println!("");
        } else {
            println!("{}", x)
        }
    }
}

// Rust tuples come in handy, as does matching
fn fb_two() {
    for i in 1..101 {
        match (i % 3 == 0, i % 5 == 0) {
            (true, true) => println!("FizzBuzz"),
            (true, false) => println!("Fizz"),
            (false, true) => println!("Buzz"),
            (false, false) => println!("{}", i),
        }
    }
}

fn main() {
    fb_one();
    fb_two();
}
