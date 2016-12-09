use std::env;

// HQ9+ requires that '+' increments an accumulator, but it's inaccessible (and thus, unused).
#[allow(unused_variables)]
fn execute(code: &str) {
    let mut accumulator = 0;

    for c in code.chars() {
        match c {
            'Q' => println!("{}", code),
            'H' => println!("Hello, World!"),
            '9' => {
                for n in (1..100).rev() {
                    println!("{} bottles of beer on the wall", n);
                    println!("{} bottles of beer", n);
                    println!("Take one down, pass it around");
                    if (n - 1) > 1 {
                        println!("{} bottles of beer on the wall\n", n - 1);
                    } else {
                        println!("1 bottle of beer on the wall\n");
                    }
                }
            }
            '+' => accumulator += 1,
            _ => panic!("Invalid character '{}' found in source.", c),
        }
    }
}

fn main() {
    execute(&env::args().nth(1).unwrap());
}
