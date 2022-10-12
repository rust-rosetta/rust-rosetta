use num::rational::Ratio;
use num::BigInt;
use std::num::NonZeroU64;

fn main() {
    for n in 1..=20 {
        // harmonic_number takes the type `NonZeroU64`,
        // which is just a normal u64 which is guaranteed at compile time to never be 0.
        // We convert n into this type with `n.try_into().unwrap()`,
        // where the unwrap is okay because n is never 0.
        println!("Harmonic number {n} = {}", harmonic_number(n.try_into().unwrap()));
    }

    // the unwrap here is likewise okay because 100 is not 0.
    println!("Harmonic number 100 = {}", harmonic_number(100.try_into().unwrap()));

    // In order to avoid recomputing all the terms in the sum for every harmonic number
    // we save the value of the harmonic series between loop iterations
    // and just add 1/iter to it.

    let mut target = 1;
    let mut iter = 1;
    let mut harmonic_number: Ratio<BigInt> = Ratio::from_integer(1.into());

    while target <= 10 {
        if harmonic_number > Ratio::from_integer(target.into()) {
            println!("Position of first term > {target} is {iter}");
            target += 1;
        }

        // Compute the next term in the harmonic series
        iter += 1;
        harmonic_number += Ratio::from_integer(iter.into()).recip();
    }
}

fn harmonic_number(n: NonZeroU64) -> Ratio<BigInt> {
    // Convert each integer from 1 to n into an arbitrary precision rational number
    // and sum their reciprocals
    (1..=n.get()).map(|i| Ratio::from_integer(i.into()).recip()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twenty_first() {
        const CORRECT_FIRST_TWENTY: [[u64; 2]; 20] = [
            [1, 1],
            [3, 2],
            [11, 6],
            [25, 12],
            [137, 60],
            [49, 20],
            [363, 140],
            [761, 280],
            [7129, 2520],
            [7381, 2520],
            [83711, 27720],
            [86021, 27720],
            [1145993, 360360],
            [1171733, 360360],
            [1195757, 360360],
            [2436559, 720720],
            [42142223, 12252240],
            [14274301, 4084080],
            [275295799, 77597520],
            [55835135, 15519504],
        ];
        for (i, [numerator, denominator]) in CORRECT_FIRST_TWENTY.into_iter().enumerate() {
            assert_eq!(
                harmonic_number((i + 1).try_into().unwrap()),
                Ratio::new(numerator.into(), denominator.into())
            );
        }

        let correct_hundred = Ratio::new(
            BigInt::parse_bytes(b"14466636279520351160221518043104131447711", 10).unwrap(),
            BigInt::parse_bytes(b"2788815009188499086581352357412492142272", 10).unwrap(),
        );
        assert_eq!(harmonic_number(100), correct_hundred);
    }
}
