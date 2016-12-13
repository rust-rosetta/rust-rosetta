use std::io::stdin;

const MIN: isize = 1;
const MAX: isize = 100;

fn main() {
    loop {
        let mut min = MIN;
        let mut max = MAX;
        let mut num_guesses = 1;
        println!("Please think of a number between {} and {}", min, max);
        loop {
            let guess = (min + max) / 2;
            println!("Is it {}?", guess);
            println!("(type h if my guess is too high, l if too low, e if equal and q to quit)");

            let mut line = String::new();
            stdin().read_line(&mut line).unwrap();
            match Some(line.chars().next().unwrap().to_uppercase().next().unwrap()) {
                Some('H') => {
                    max = guess - 1;
                    num_guesses += 1;
                }
                Some('L') => {
                    min = guess + 1;
                    num_guesses += 1;
                }
                Some('E') => {
                    if num_guesses == 1 {
                        println!("\n*** That was easy! Got it in one guess! ***\n");
                    } else {
                        println!("\n*** I knew it! Got it in only {} guesses! ***\n",
                                 num_guesses);
                    }
                    break;
                }
                Some('Q') => return,
                _ => println!("Sorry, I didn't quite get that. Please try again."),
            }
        }
    }
}
