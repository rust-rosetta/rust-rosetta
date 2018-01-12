extern crate linear_congruential_generator;

use linear_congruential_generator::*;

fn main() {
    println!("~~~ BSD ~~~");
    let mut bsd = BsdLcg::from_seed(0);
    for _ in 0..10 {
        println!("{}", bsd.next_u32());
    }

    println!("~~~ MS ~~~");
    let mut ms = MsLcg::from_seed(0);
    for _ in 0..10 {
        println!("{}", ms.next_u32());
    }

    // Because we have implemented the `rand::Rng` trait, we can generate a variety of other types.
    println!("~~~ Others ~~~");
    println!("{:?}", ms.gen::<[u32; 5]>());
    println!("{}", ms.gen::<bool>());
    println!("{}", ms.gen_ascii_chars().take(15).collect::<String>());
}
