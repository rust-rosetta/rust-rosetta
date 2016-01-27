// http://rosettacode.org/wiki/Loops/Nested
extern crate rand;

use rand::Rng;

fn main() {
    let mut matrix = [[0u8; 10]; 10];
    let mut rng = rand::thread_rng();

    for row in matrix.iter_mut() {
        for item in row.iter_mut() {
            *item = rng.gen_range(0, 21);
        }
    }

    'outer: for row in matrix.iter() {
        for &item in row.iter() {
            print!("{:2} ", item);
            if item == 20 {
                break 'outer;
            }
        }
        println!("");
    }
}
