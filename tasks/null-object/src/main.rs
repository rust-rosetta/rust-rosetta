// If an option may return null - or nothing - in Rust, it's wrapped
// in an Optional which may return either the type of object specified
// in <> or None. We can check this using .is_some() and .is_none() on
// the Option.

fn check_number(num: &Option<u8>) {
    if num.is_none() {
        println!("Number is: None");
    } else {
        println!("Number is: {}", num.unwrap());
    }
}

fn main() {
    let mut possible_number: Option<u8> = None;
    check_number(&possible_number);

    possible_number = Some(31);
    check_number(&possible_number);
}
