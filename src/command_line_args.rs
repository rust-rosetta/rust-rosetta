// Implements http://rosettacode.org/wiki/Command-line_arguments

use std::env;

fn main(){
    for arg in env::args() {
        println!("{}", arg);
    }
}
