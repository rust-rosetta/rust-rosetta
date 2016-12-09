use std::io::{self, Write};
use std::fmt::Display;
use std::{env, process};

fn main() {
    let shift = env::args()
        .nth(1)
        .unwrap_or_else(|| exit_err("No shift provided", 2))
        .parse::<u8>()
        .unwrap_or_else(|e| exit_err(e, 3));

    let plain = get_input().unwrap_or_else(|e| exit_err(&e, e.raw_os_error().unwrap_or(-1)));

    let cipher = cipher(&plain, shift);

    println!("Cipher text: {}", cipher.trim());
}

fn cipher(input: &str, shift: u8) -> String {
    input.chars()
        .map(|c| {
            let case = if c.is_uppercase() {
                b'A'
            } else {
                b'a'
            };

            if c.is_alphabetic() {
                (((c as u8 - case + shift) % 26) + case) as char
            } else {
                c
            }

        })
        .collect()
}

fn get_input() -> io::Result<String> {
    print!("Plain text:  ");
    try!(io::stdout().flush());

    let mut buf = String::new();
    try!(io::stdin().read_line(&mut buf));
    Ok(buf)
}

fn exit_err<T: Display>(msg: T, code: i32) -> ! {
    writeln!(&mut io::stderr(), "ERROR: {}", msg).unwrap();
    process::exit(code);
}

#[cfg(test)]
mod tests {
    #[test]
    fn encode() {
        let original = "The five boxing wizards jump quickly";
        let encoded = "Wkh ilyh eralqj zlcdugv mxps txlfnob";
        assert_eq!(super::cipher(original, 3), encoded);
    }
}
