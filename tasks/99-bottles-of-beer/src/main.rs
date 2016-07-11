use std::string::String;

fn main() {
    for num_bottles in (1u32..100).rev() {
        println!("{}", bottles_line(num_bottles, true));
        println!("{}", bottles_line(num_bottles, false));
        println!("Take one down, pass it around...");
        println!("{}", bottles_line(num_bottles - 1, true));
        println!("-----------------------------------");
    }
}

fn bottles_line(num_bottles: u32, on_the_wall: bool) -> String {
    let tail = if on_the_wall {
        "of beer on the wall!\n"
    } else {
        "of beer\n"
    };

    match num_bottles {
        0 => format!("No bottles {}", tail),
        1 => format!("One bottle {}", tail),
        n => format!("{} bottles {}", n, tail),
    }
}

#[test]
fn gen_bottle_line() {
    let ln = bottles_line(42, false);
    let ln2 = bottles_line(42, true);

    assert_eq!(ln, "42 bottles of beer\n");
    assert_eq!(ln2, "42 bottles of beer on the wall!\n");
}
