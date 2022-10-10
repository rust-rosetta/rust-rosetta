use num::rational::Ratio;
use num::BigInt;

fn main() {
    for n in 1..=20 {
        println!("Harmonic number {n} = {}", harmonic_number(n));
    }

    println!("Harmonic number 100 = {}", harmonic_number(100));

    //In order to avoid recomputing all the terms in the sum for the n:th harmonic number
    //we save the value of the harmonic series between loop iterations
    //and just add 1/iter to it.

    let mut target = 1;
    let mut iter = 1;
    let mut h: Ratio<BigInt> = Ratio::from_integer(1.into());

    while target <= 10 {
        if h > Ratio::from_integer(target.into()) {
            println!("Position of first term > {target} is {iter}");
            target += 1;
        }

        //Compute the next term in the harmonic series
        iter += 1;
        h += Ratio::from_integer(iter.into()).recip();
    }
}

fn harmonic_number(n: u64) -> Ratio<BigInt> {
    //Convert each integer from 1 to n into an arbitrary precision rational number
    //and sum their reciprocals
    (1..=n).map(|i| Ratio::from_integer(i.into()).recip()).sum()
}
