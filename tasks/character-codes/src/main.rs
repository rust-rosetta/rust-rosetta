use std::char::from_u32;

fn main() {
    // ascii char
    println!("{}", 'a' as u8);
    println!("{}", 97 as char);

    // unicode char
    println!("{}", 'π' as u32);
    println!("{}", from_u32(960).unwrap());
}
