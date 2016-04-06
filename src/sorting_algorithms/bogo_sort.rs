// http://rosettacode.org/wiki/Sorting_algorithms/Bogosort
extern crate rand;
use rand::{thread_rng, Rng};

fn bogo_sort<T: Ord>(mut v: &mut [T]) {
    let mut rng = thread_rng();
    while !is_sorted(v) {
        rng.shuffle(&mut v);
    }
}

// helper function that checks for ascending order
fn is_sorted<T: Ord>(v: &[T]) -> bool {
    if v.len() > 1 {
        for i in 0..(v.len() - 1) {
            if v[i] > v[i + 1] {
                return false;
            }
        }
    }
    true
}

fn main() {
    let mut numbers = [4i32, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);
    bogo_sort(&mut numbers);
    println!("After: {:?}", numbers);
}

#[cfg(test)]
mod tests {
    use super::bogo_sort;

    fn check_sort<T: Ord>(v: &[T]) {
        assert!(super::is_sorted(v));
    }

    #[test]
    fn test_rosetta_vector() {
        let numbers = &mut [4i32, 65, 2, -31, 0, 99, 2, 83, 782, 1];
        bogo_sort(numbers);
        check_sort(numbers);
    }

    #[test]
    fn test_empty_vector() {
        let mut numbers: Vec<i32> = Vec::new();
        bogo_sort(&mut numbers[..]);
        check_sort(&mut numbers[..]);
    }

    #[test]
    fn test_one_element_vector() {
        let numbers = &mut [0i32];
        bogo_sort(numbers);
        check_sort(numbers);
    }

    #[test]
    fn test_repeat_vector() {
        let numbers = &mut [1i32, 1, 1, 1, 1];
        bogo_sort(numbers);
        check_sort(numbers);
    }

    #[test]
    fn test_string_vector() {
        let strings = &mut ["beach", "hotel", "airplane", "car", "house", "art"];
        bogo_sort(strings);
        check_sort(strings);
    }
}
