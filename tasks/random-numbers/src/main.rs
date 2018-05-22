extern crate rand;

use rand::distributions::Normal;
use rand::Rng;

fn main() {
    let normal = Normal::new(1.0, 0.5);
    let mut rng = rand::thread_rng();

    let rands = (0..1000).map(|_| rng.sample(normal)).collect::<Vec<_>>();
    println!("{:?}", rands);
}
