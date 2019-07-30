// why use a library? Clap lets us quickly build a CLI and
// lets us focus the below code on password generation
// not the intricacies building a CLI in rust.
// Read more about clap here: https://clap.rs/
// declare our external dependency for parsing command line arguments
extern crate clap;
// bring the App structure into scope so we can use it
use clap::App;

fn main() {
    App::new("password-generator")
        .version("0.1")
        .about("generate a password according to the rosetta code rules: http://rosettacode.org/wiki/Password_generator")
        .get_matches();
}
