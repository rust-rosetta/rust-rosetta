extern crate regex;

use regex::Regex;

fn main() {
    let range = "-6,-3-1,3-5,7-11,14,15,17-20";
    println!("Expanded range: {:?}", expand_range(range));
}

/// Expand a string containing numbers and ranges, into a vector of numbers
fn expand_range(range: &str) -> Vec<isize> {
    let mut result = vec![];

    for item in range.split(',') {
        result.extend(expand_item(item).into_iter());
    }

    result
}

/// Expand a single element, which can be a number or a range.
fn expand_item(item: &str) -> Vec<isize> {
    // Handle the case of a single number
    for cap in Regex::new(r"^(-?\d+)$").unwrap().captures_iter(item) {
        return vec![cap.get(0).and_then(|s| s.as_str().parse().ok()).unwrap()];
    }

    // Handle the case of a range
    for cap in Regex::new(r"^(-?\d+)-(-?\d+)$").unwrap().captures_iter(item) {
        let left: isize = cap.get(1).and_then(|s| s.as_str().parse().ok()).unwrap();
        let right: isize = cap.get(2).and_then(|s| s.as_str().parse().ok()).unwrap();

        // Generate and collect a range between them
        return (left..right + 1).collect();
    }

    panic!("The item `{}` is not a number or a range!", item);
}

#[test]
fn test_basic() {
    let range = "1-5,6";
    assert!(expand_range(range) == vec![1, 2, 3, 4, 5, 6]);

    let range = "-6,-3-1,3-5,7-11,14,15,17-20";
    assert!(expand_range(range) ==
            vec![-6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20]);
}

#[test]
#[should_panic]
fn test_wrong() {
    let range = "one-five,six";
    assert!(expand_range(range) == vec![1, 2, 3, 4, 5, 6]);
}
