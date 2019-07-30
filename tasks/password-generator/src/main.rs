// why use a library? Clap lets us quickly build a CLI and
// lets us focus the below code on password generation
// not the intricacies building a CLI in rust.
// Read more about clap here: https://clap.rs/
// declare our external dependency for parsing command line arguments
extern crate clap;
// bring the needed structures into scope so we
// invoke them later in the program
use clap::{App,Arg};

fn main() {
    // create our new CLI
    // clap provides powerful defaults so we don't have to
    // write all the logic here
    // For example, clap takes care of the help and version flags by default
    App::new("password-generator")
        .version("0.1")
        .about("generate a password according to the rosetta code rules: http://rosettacode.org/wiki/Password_generator")
        // configure our first required argument
        .arg(Arg::with_name("LENGTH")
             .help("password length")
             // make it SECURE by default
             // https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html
             .default_value("160")
             .required(true)
             .index(1)
            )
        // configure our second required argument
        .arg(Arg::with_name("COUNT")
             .help("how many passwords to generate")
             .default_value("1")
             .required(true)
             .index(2)
            )
        .get_matches();
}
