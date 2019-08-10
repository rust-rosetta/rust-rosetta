use rand::distributions::Alphanumeric;
use rand::prelude::IteratorRandom;
use rand::{thread_rng, Rng};
use std::iter;
use structopt::StructOpt;

// the core logic that creates our password
fn generate_password(length: u8) -> String {
    // cache thread_rng for better performance
    let mut rng = thread_rng();
    let mut base_password: Vec<char> = iter::repeat(())
        // the Alphanumeric struct provides 3/4
        // of the characters for passwords
        // so we can sample from it
        .map(|()| rng.sample(Alphanumeric))
        .take(length as usize)
        .collect();
    // create an iterator of required other characters
    const OTHER_VALUES: &str = "!\"#$%&'()*+,-./:;<=>?@[]^_{|}~";
    // create a random count of how many other characters to add
    let mut to_add = rng.gen_range(1, 10);
    loop {
        let special = OTHER_VALUES.chars().choose(&mut rng).unwrap();
        to_add -= 1;
        base_password[to_add] = special;
        if to_add == 0 {
            break;
        }
    }
    base_password.iter().collect()
}

#[derive(StructOpt, Debug)]
#[structopt(name = "password-generator", about = "A simple password generator.")]
struct Opt {
    // make it SECURE by default
    // https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html
    /// The password length
    #[structopt(short = "l", long = "length", default_value = "160")]
    length: u8,
    /// How many passwords to generate
    #[structopt(short = "c", long = "count", default_value = "1")]
    count: u8,
}

fn main() {
    // instantiate the options and use them as
    // arguments to our password generator
    let opt = Opt::from_args();
    for index in 0..opt.count {
        let password = generate_password(opt.length);
        // do not print a newline after the last password
        match index + 1 == opt.count {
            true => print!("{}", password),
            _ => println!("{}", password)
        };
    }
}

// keep it within this file for convenience
#[cfg(test)]
mod tests {
    use super::generate_password;

    // test our core password logic according to the rules described in the wiki
    #[test]
    fn generate_password_customizes_length() {
        let a_password = generate_password(50);
        assert_eq!(a_password.len(), 50);
    }

    #[test]
    fn generate_password_has_numerals() {
        // TODO how can I pass in char::is_ascii_digit instead of defining a closure?
        assert!(generate_password(50).chars().any(|c| c.is_ascii_digit()));
    }

    #[test]
    fn generate_password_has_upper_and_lowercase_characters() {
        let password = generate_password(50);
        // the following line only prints when this test fails
        // you can see it printed by running `cargo test -- --nocapture`
        println!("{}", password);
        assert!(password.chars().any(|c| c.is_ascii_lowercase()));
        assert!(generate_password(50)
            .chars()
            .any(|c| c.is_ascii_uppercase()));
    }

    #[test]
    fn generate_password_has_other_characters() {
        let password = generate_password(10);
        println!("{}", password);
        // TODO the below assertion is quite verbose
        // make it more idiomatic and terse
        assert!(
            password
                .chars()
                .filter(|&c| {
                    c == '!'
                        || c == '"'
                        || c == '#'
                        || c == '$'
                        || c == '%'
                        || c == '&'
                        || c == '\''
                        || c == '('
                        || c == ')'
                        || c == '*'
                        || c == '+'
                        || c == ','
                        || c == '-'
                        || c == '_'
                        || c == '.'
                        || c == '/'
                        || c == ':'
                        || c == ';'
                        || c == '<'
                        || c == '>'
                        || c == '='
                        || c == '?'
                        || c == '@'
                        || c == '['
                        || c == ']'
                        || c == '^'
                        || c == '{'
                        || c == '}'
                        || c == '|'
                        || c == '~'
                })
                .count()
                >= 1
        );
    }

}
