use std::char;

fn main() {
    // ascii char
    println!("{}", b'a');
    println!("{}", 97 as char);

    // unicode char
    println!("{}", 'π' as u32);
    println!("{}", char::from_u32(960).unwrap());
}
