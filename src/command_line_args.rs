// Implements http://rosettacode.org/wiki/Command-line_arguments

use std::os;

fn main(){
    println!("{:?}", os::args());
}
