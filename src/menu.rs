// Implements http://rosettacode.org/wiki/Menu

use std::io;

// Print the menu followed by the prompt
fn print_both(menu: &[&str], prompt: &str) {

    // Iterate through array and print index, period, and menu item
    for (i, item) in menu.iter().enumerate() {
        println!("{}. {}", i, item);
    }

    // Print the prompt
    println!("{}", prompt);
}


// Grab the next line of input
fn next_input() -> Option<uint> {

    let line = io::stdin().read_line().unwrap();
    let input: Option<uint> = line.trim().parse();
    return input;
}

fn select<'a>(menu: &'a [&str], prompt: &str) -> &'a str {
    
    // Check if menu is empty
    if menu.len() == 0 {
        return "";
    }

    // Loop until user inputs a valid menu index
    loop {

        print_both(menu, prompt);

        let input = next_input();

        let num = match input {
            Some(num) => num,
            None      => continue
        };

        if let 0...3 = num {
            return menu[num];
        }
    }
}

#[cfg(not(test))]
fn main() {

    let prompt = "Choose one.";
    let items = ["fee fie", "huff and puff", "mirror mirror", "tick tock"];
    let menu = items.slice(0,4);

    println!("{}", select(menu, prompt));
}


#[test]
fn test_empty_menu() {
    let prompt = "Choose one.";
    let items = ["fee fie", "huff and puff", "mirror mirror", "tick tock"];
    let menu = items.slice(0,0);
    assert_eq!(select(menu, prompt), "");
}

// Need to add more tests but having trouble simulating std input
