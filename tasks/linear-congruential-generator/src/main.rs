extern crate linear_congruential_generator;
extern crate rand;

use linear_congruential_generator::*;

use rand::distributions::Alphanumeric;
use rand::{FromEntropy, Rng};

fn main() {
    println!("~~~ BSD ~~~");
    let mut bsd = BsdLcg::from_seed_u32(0);
    for _ in 0..10 {
        println!("{}", bsd.gen::<u32>());
    }

    println!("~~~ MS ~~~");
    let mut ms = MsLcg::from_seed_u32(0);
    for _ in 0..10 {
        println!("{}", ms.gen::<u32>());
    }

    // Because we have implemented the `rand_core::RngCore` and `SeedableRng` traits,
    // we can generate a variety of other types.
    //
    // Note that the alphanumeric sampling and the boolean sampling are broken because
    // `MsLcg` does not generate a full 32 bits.
    let mut rng = MsLcg::from_entropy();
    println!("~~~ Others ~~~");
    println!("{:?}", rng.gen::<[f32; 5]>());
    println!("{}", rng.gen::<bool>());
    println!(
        "{}",
        rng.sample_iter(&Alphanumeric).take(15).collect::<String>()
    );
}
