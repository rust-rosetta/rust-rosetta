use rand::Rng;
use rand_distr::Normal;

fn main() {
    let normal = Normal::new(1.0, 0.5).unwrap();
    let mut rng = rand::thread_rng();

    let rands = (0..1000).map(|_| rng.sample(normal)).collect::<Vec<_>>();
    println!("{:?}", rands);
}
