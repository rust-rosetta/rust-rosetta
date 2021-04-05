fn is_prime(number: u32) -> bool {
    #[allow(clippy::cast_precision_loss)]
    let limit = (number as f32).sqrt() as u32 + 1;

    // We test if the number is divisible by any number up to the limit
    !(number < 2 || (2..limit).any(|x| number % x == 0))
}

fn main() {
    println!(
        "Primes below 100:\n{:?}",
        (0_u32..100).fold(vec![], |mut acc, number| {
            if is_prime(number) {
                acc.push(number)
            };
            acc
        })
    );
}
