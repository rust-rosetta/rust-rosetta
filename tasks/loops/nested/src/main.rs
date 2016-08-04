extern crate rand;

use rand::Rng;

fn main() {
    let mut matrix = [[0u8; 10]; 10];
    let mut rng = rand::thread_rng();

    for row in &mut matrix {
        for item in &mut *row {
            *item = rng.gen_range(0, 21);
        }
    }

    'outer: for row in &matrix {
        for &item in row {
            print!("{:2} ", item);
            if item == 20 {
                break 'outer;
            }
        }
        println!("");
    }
}
