extern crate rand;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    loop {
        let num = rng.gen_range(0, 20);
        println!("{}", num);
        if num == 10 {
            break;
        }
        println!("{}", rng.gen_range(0, 20));
    }
}
