// rust
// http://rosettacode.org/wiki/foo
// Accepts two command line arguments

use std::os;
use std::num;

fn main () {
    let args = os::args();
    let args = args.as_slice();

    let x = from_str::<f32>(args[1].as_slice()).unwrap() ;
    let y = from_str::<f32>(args[2].as_slice()).unwrap() ;

    let result = agm(x,y);
    println!("The arithmetic-geometric mean is {}", result);
}

fn agm (x: f32, y: f32) -> (f32) {
    let e: f32 = 0.000001;
    let mut a = x;
    let mut g = y;
    let mut a1: f32;
    let mut g1: f32;

    loop {
            a1 = (a + g) / 2f32;
            g1 = (a * g).sqrt();
            a = a1;
            g = g1;
            if num::abs( a - g) < e {  return a; }
        }

}
