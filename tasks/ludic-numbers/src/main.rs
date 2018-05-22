#![feature(iterator_step_by)] // for `step_by`

const ARRAY_MAX: usize = 25_000;
const LUDIC_MAX: usize = 2100;

/// Calculates and returns the first `LUDIC_MAX` Ludic numbers.
///
/// Needs a sufficiently large `ARRAY_MAX`.
fn ludic_numbers() -> Vec<usize> {
    // The first two Ludic numbers
    let mut numbers = vec![1, 2];
    // We start the array with an immediate first removal to reduce memory usage by
    // collecting only odd numbers.
    numbers.extend((3..ARRAY_MAX).step_by(2));

    // We keep the correct Ludic numbers in place, removing the incorrect ones.
    for ludic_idx in 2..LUDIC_MAX {
        let next_ludic = numbers[ludic_idx];

        // We remove incorrect numbers by counting the indices after the correct numbers.
        // We start from zero and keep until we reach the potentially incorrect numbers.
        // Then we keep only those not divisible by the `next_ludic`.
        let mut idx = 0;
        numbers.retain(|_| {
            let keep = idx <= ludic_idx || (idx - ludic_idx) % next_ludic != 0;
            idx += 1;
            keep
        });
    }

    numbers
}

fn main() {
    let ludic_numbers = ludic_numbers();

    print!("First 25: ");
    print_n_ludics(&ludic_numbers, 25);
    println!();
    print!("Number of Ludics below 1000: ");
    print_num_ludics_upto(&ludic_numbers, 1000);
    println!();
    print!("Ludics from 2000 to 2005: ");
    print_ludics_from_to(&ludic_numbers, 2000, 2005);
    println!();
    println!("Triplets below 250: ");
    print_triplets_until(&ludic_numbers, 250);
}

/// Prints the first `n` Ludic numbers.
fn print_n_ludics(x: &[usize], n: usize) {
    println!("{:?}", &x[..n]);
}

/// Calculates how many Ludic numbers are below `max_num`.
fn print_num_ludics_upto(x: &[usize], max_num: usize) {
    let num = x.iter().take_while(|&&i| i < max_num).count();
    println!("{}", num);
}

/// Prints Ludic numbers between two numbers.
fn print_ludics_from_to(x: &[usize], from: usize, to: usize) {
    println!("{:?}", &x[from - 1..to - 1]);
}

/// Calculates triplets until a certain Ludic number.
fn triplets_below(ludics: &[usize], limit: usize) -> Vec<(usize, usize, usize)> {
    ludics
        .iter()
        .enumerate()
        .take_while(|&(_, &num)| num < limit)
        .filter_map(|(idx, &number)| {
            let triplet_2 = number + 2;
            let triplet_3 = number + 6;

            // Search for the other two triplet numbers.  We know they are larger than
            // `number` so we can give the searches lower bounds of `idx + 1` and
            // `idx + 2`.  We also know that the `n + 2` number can only ever be two
            // numbers away from the previous and the `n + 6` number can only be four
            // away (because we removed some in between).  Short circuiting and doing
            // the check more likely to fail first are also useful.
            let is_triplet = ludics[idx + 1..idx + 3].binary_search(&triplet_2).is_ok()
                && ludics[idx + 2..idx + 5].binary_search(&triplet_3).is_ok();

            if is_triplet {
                Some((number, triplet_2, triplet_3))
            } else {
                None
            }
        })
        .collect()
}

/// Prints triplets until a certain Ludic number.
fn print_triplets_until(ludics: &[usize], limit: usize) {
    for (number, triplet_2, triplet_3) in triplets_below(ludics, limit) {
        println!("{} {} {}", number, triplet_2, triplet_3);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solutions() {
        let ludics = ludic_numbers();

        // First 25
        let first_25 = [
            1, 2, 3, 5, 7, 11, 13, 17, 23, 25, 29, 37, 41, 43, 47, 53, 61, 67, 71, 77, 83, 89, 91,
            97, 107,
        ];
        assert_eq!(ludics[..25], first_25);

        // Count below 1000
        assert_eq!(ludics.iter().take_while(|&&i| i < 1000).count(), 142);

        // 2000th to 2005th numbers
        let expected_range = [21475, 21481, 21487, 21493, 21503];
        assert_eq!(ludics[2000 - 1..2005 - 1], expected_range);

        // Triplets
        let expected_triplets = [
            (1, 3, 7),
            (5, 7, 11),
            (11, 13, 17),
            (23, 25, 29),
            (41, 43, 47),
            (173, 175, 179),
            (221, 223, 227),
            (233, 235, 239),
        ];
        let tested_triplets = triplets_below(&ludics, 250);
        assert_eq!(expected_triplets, tested_triplets.as_slice());
    }
}
