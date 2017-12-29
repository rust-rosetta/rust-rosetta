use std::io;

/// Print the menu followed by the prompt
fn print_both(menu: &[&str], prompt: &str) {
    // Iterate through array and print index, period, and menu item
    for (i, item) in menu.iter().enumerate() {
        println!("{}. {}", i, item);
    }

    // Print the prompt
    println!("{}", prompt);
}

/// Grab the next line of input
fn next_input() -> Option<usize> {
    let mut in_s = String::new();
    if io::stdin().read_line(&mut in_s).is_ok() {
        in_s.trim().parse().ok()
    } else {
        None
    }
}

fn select<'a>(menu: &'a [&str], prompt: &str) -> &'a str {
    if menu.is_empty() {
        return "";
    }

    // Loop until user inputs a valid menu index
    loop {
        print_both(menu, prompt);

        let input = next_input();

        let num = match input {
            Some(num) => num,
            None => continue,
        };

        if num < menu.len() {
            return menu[num];
        }
    }
}

fn main() {
    let prompt = "Choose one.";
    let menu = &["fee fie", "huff and puff", "mirror mirror", "tick tock"];
    println!("{}", select(menu, prompt));
}

#[test]
fn test_empty_menu() {
    let prompt = "Choose one.";
    let menu = &[];
    assert_eq!(select(menu, prompt), "");
}
