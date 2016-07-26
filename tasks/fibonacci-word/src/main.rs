extern crate entropy;

use entropy::shannon_entropy;

/// Returns "amount" fibonacci words as a vector of tuples
/// The first value of the tuple is the length of the word
/// and the second one its entropy
fn fib_words(amount: usize) -> Vec<(usize, f64)> {
    let mut data = Vec::with_capacity(amount);
    let mut previous = String::from("1");
    let mut next = String::from("0");

    // The first two words (we need to add them manually because
    // they are the base of the sequence)
    data.push((previous.len(), shannon_entropy(&previous[..])));
    data.push((next.len(), shannon_entropy(&next[..])));

    // The rest of the words
    for _ in 3..(amount + 1) {
        let temp = next.clone();
        next.push_str(&previous[..]);
        previous = temp;
        data.push((next.len(), shannon_entropy(&next[..])));
    }

    data
}

fn main() {
    println!("Calculating... This may take a couple of minutes...\n");

    let words = fib_words(18);
    let mut i = 1;

    println!("{:>2}:{:>10} {}", "N", "length", "entropy");
    for &(length, entropy) in &words {
        println!("{:>2}:{:>10} {:.15}", i, length, entropy);
        i += 1;
    }
}

#[test]
fn test_fibonacii_words() {
    let expected = vec![(1, 0.000000000000000f64),
                        (1, 0.000000000000000),
                        (2, 1.000000000000000),
                        (3, 0.918295834054490),
                        (5, 0.970950594454669),
                        (8, 0.954434002924965),
                        (13, 0.961236604722876),
                        (21, 0.958711882977132),
                        (34, 0.959686893774217),
                        (55, 0.959316032054378),
                        (89, 0.959457915838670),
                        (144, 0.959403754221023),
                        (233, 0.959424446955987),
                        (377, 0.959416543740441),
                        (610, 0.959419562603144),
                        (987, 0.959418409515224),
                        (1597, 0.959418849957810),
                        (2584, 0.959418681724032)];

    let epsilon = 0.0000000001f64;
    let output = fib_words(18);

    for ((output_length, output_entropy), (expected_length, expected_entropy)) in output.into_iter()
        .zip(expected.into_iter()) {
        assert!(output_length == expected_length);
        assert!((output_entropy - expected_entropy).abs() < epsilon);
    }
}
