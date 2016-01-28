// http://rosettacode.org/wiki/Abundant,_deficient_and_perfect_number_classifications
use std::cmp::Ordering;
use std::iter::Map;
use std::ops::Range;

fn divisors(n: i32) -> Vec<i32> {
    let mut results = Vec::new();

    for i in 1..(n / 2 + 1) {
        if n % i == 0 {
            results.push(i);
        }
    }
    results
}

fn classify_numbers() -> Map<Range<i32>, fn(i32) -> Ordering> {
    fn sum_divisors_and_cmp(n: i32) -> Ordering {
        divisors(n).iter().fold(0, std::ops::Add::add).cmp(&n)
    }
    (1i32..20001).map(sum_divisors_and_cmp as fn(i32) -> Ordering)
}

fn main() {
    let mut deficient = 0;
    let mut perfect = 0;
    let mut abundant = 0;
    for item in classify_numbers() {
        match item {
            Ordering::Less => deficient += 1,
            Ordering::Equal => perfect += 1,
            Ordering::Greater => abundant += 1,
        }
    }
    println!("deficient: {}", deficient);
    println!("perfect: {}", perfect);
    println!("abundant: {}", abundant);
}

#[test]
fn test_divisors() {
    let n = 6;
    let divs = divisors(n);
    let expected_divs: Vec<i32> = vec![1, 2, 3];
    assert_eq!(divs, expected_divs);
}

#[test]
fn test_classify_numbers() {
    let mut deficient = 0;
    let mut perfect = 0;
    let mut abundant = 0;
    for item in classify_numbers() {
        match item {
            Ordering::Less => deficient += 1,
            Ordering::Equal => perfect += 1,
            Ordering::Greater => abundant += 1,
        }
    }

    // Numbers taken from the wiki page.
    assert_eq!(deficient, 15043);
    assert_eq!(perfect, 4);
    assert_eq!(abundant, 4953);
}
