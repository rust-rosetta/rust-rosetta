extern crate num;

use num::FromPrimitive;
use num::bigint::BigInt;

use std::collections::HashSet;

/// Reverse a number then add it to the original.
fn rev_add(num: &BigInt) -> BigInt {
    let rev_string: String = num.to_string().chars().rev().collect();
    // should be safe, our string is guaranteed to be a number
    let rev_val: BigInt = rev_string.parse().unwrap();
    num + rev_val
}

/// Check if a number is a palindrome when written in base 10.
fn is_palindrome(num: &BigInt) -> bool {
    let num_string = num.to_string();
    let rev_string: String = num_string.chars().rev().collect();
    let comp_len = num_string.len() / 2;
    num_string[0..comp_len] == rev_string[0..comp_len]
}

/// Perform a lychrel test on a number, stopping after `max_tests`
/// Returns the sequence of numbers if this number is a lychrel, None otherwise.
fn test_lychrel(num: &BigInt, max_tests: usize) -> Option<Vec<BigInt>> {
    let mut sequence = Vec::<BigInt>::new();

    let is_lychrel = (0..max_tests)
        .scan(num.clone(), |current, _| {
            *current = rev_add(current);
            Some(current.clone())
        })
        .inspect(|current| sequence.push(current.clone()))
        .filter(|curent| is_palindrome(curent))
        .next()
        .is_none();

    if is_lychrel {
        Some(sequence)
    } else {
        None
    }
}

/// Determine if the sequence for a lychrel number is related to a previously seen sequence
fn is_related(seq: &[BigInt], lychrel_seq_numbers: &HashSet<BigInt>) -> bool {
    seq.iter().filter(|num| lychrel_seq_numbers.contains(num)).next().is_some()
}

/// Find the lychrel numbers up to `max_num` (inclusive).
/// Returns a tuple (lychrel numbers, related numbers, palindrome lychrel/related numbers)
fn find_lychrels(max_num: u64, max_tests: usize) -> (Vec<BigInt>, Vec<BigInt>, Vec<BigInt>) {
    // storage for various outputs
    let mut lychrels = Vec::<BigInt>::new();
    let mut relateds = Vec::<BigInt>::new();
    let mut palindrome_lychrels = Vec::<BigInt>::new();

    let mut lychrel_seq_numbers: HashSet<BigInt> = HashSet::new();

    for i in 1..(max_num + 1) {
        let num = FromPrimitive::from_u64(i).unwrap();
        let maybe_lychrel = test_lychrel(&num, max_tests);

        if let Some(lychrel_seq) = maybe_lychrel {
            // it's a lychrel - check if it's a related number
            let related = is_related(&lychrel_seq, &lychrel_seq_numbers);

            // update our sequences
            for seq_num in lychrel_seq.into_iter() {
                lychrel_seq_numbers.insert(seq_num);
            }

            if !related {
                // the number has a new lychrel sequence, store it
                lychrels.push(num.clone());
            } else {
                // just count it as a related number
                relateds.push(num.clone());
            }

            if is_palindrome(&num) {
                // doesn't matter if palindromes are related or not
                palindrome_lychrels.push(num.clone());
            }
        }
    }

    (lychrels, relateds, palindrome_lychrels)
}

fn print_nums(before: &str, numbers: &[BigInt]) {
    print!("{}", before);
    for (i, current) in numbers.iter().enumerate() {
        print!("{}", current);
        if i + 1 < numbers.len() {
            print!(", ");
        }
    }
    println!("");
}

fn main() {
    let max_num: u64 = 10_000;
    let max_tests: usize = 500;

    println!("Calculations using n = 1..{} and limiting each search to {} reverse-digits-and-adds",
             max_num,
             max_tests);

    let (lychrels, relateds, palindrome_lychrels) = find_lychrels(max_num, max_tests);

    println!("Number of Lychrel numbers: {}", lychrels.len());
    print_nums("Lychrel numbers: ", &lychrels);
    println!("Number of Lychrel related: {}", relateds.len());
    println!("Number of Lychrel palindromes: {}",
             palindrome_lychrels.len());
    print_nums("Lychrel palindromes: ", &palindrome_lychrels);
}

#[test]
#[ignore]
fn test_lychrel_numbers() {
    let (lychrels, relateds, palindrome_lychrels) = find_lychrels(10_000, 500);

    let expected_lychrels = [196, 879, 1997, 7059, 9999]
        .iter()
        .map(|&num| FromPrimitive::from_u64(num).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(lychrels, expected_lychrels);

    assert_eq!(relateds.len(), 244);

    let expected_palindromes = [4994, 8778, 9999]
        .iter()
        .map(|&num| FromPrimitive::from_u64(num).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(palindrome_lychrels, expected_palindromes);
}
