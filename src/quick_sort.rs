//Implements http://rosettacode.org/wiki/Sorting_algorithms/Quicksort
// Used by the tests
#![allow(unused_features)]

extern crate rand;
#[cfg(test)]
use rand::{thread_rng, Rng};

// We use in place quick sort
// For details see http://en.wikipedia.org/wiki/Quicksort#In-place_version
fn quick_sort<T: Ord>(v: &mut[T]) {
    let len = v.len();
    if len < 2 {
        return;
    }

    let pivot_index = partition(v);

    // Sort the left side
    quick_sort(&mut v[0..pivot_index]);

    // Sort the right side
    quick_sort(& mut v[pivot_index + 1..len]);
}

// Reorders the slice with values lower than the pivot at the left side,
// and values bigger than it at the right side.
// Also returns the store index.
fn partition<T: Ord>(v: &mut [T]) -> usize {
    let len = v.len();
    let pivot_index = len / 2;

    v.swap(pivot_index, len - 1);

    let mut store_index = 0;
    for i in (0..len - 1) {
        if v[i] <= v[len - 1] {
            v.swap(i, store_index);
            store_index += 1;
        }
    }

    v.swap(store_index, len - 1);
    store_index
}

#[cfg(not(test))]
fn main() {
    // Sort numbers
    let mut numbers = [4i32, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);

    quick_sort(&mut numbers);
    println!("After: {:?}", numbers);

    // Sort strings
    let mut strings = ["beach", "hotel", "airplane", "car", "house", "art"];
    println!("Before: {:?}", strings);

    quick_sort(&mut strings);
    println!("After: {:?}", strings);
}

#[cfg(test)]
fn check_sort<T: Ord>(v: &[T]) {
    if v.len() > 1 {
        for i in (0..v.len()-1) {
            assert!(v[i] <= v[i+1]);
        }
    }
}

#[test]
fn test_rosetta_vector() {
    let numbers = &mut [4i32, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    quick_sort(numbers);
    check_sort(numbers);
}

#[test]
fn test_empty_vector() {
    let mut numbers: Vec<i32> = Vec::new();
    quick_sort(&mut numbers[..]);
    check_sort(&mut numbers[..]);
}

#[test]
fn test_one_element_vector() {
    let numbers = &mut [0i32];
    quick_sort(numbers);
    check_sort(numbers);
}

#[test]
fn test_repeat_vector() {
    let numbers = &mut [1i32, 1, 1, 1, 1];
    quick_sort(numbers);
    check_sort(numbers);
}

#[test]
fn test_worst_case_vector() {
    let numbers = &mut [20i32, 10, 0, -1, -5];
    quick_sort(numbers);
    check_sort(numbers);
}

#[test]
fn test_already_sorted_vector() {
    let numbers = &mut [-1i32, 0, 3, 6, 99];
    quick_sort(numbers);
    check_sort(numbers);
}

#[test]
fn test_random_numbers() {
    let mut rng = thread_rng();
    let mut numbers : Vec<i32> = rng.gen_iter::<i32>().take(500).collect();
    quick_sort(&mut numbers[..]);
    check_sort(&mut numbers[..]);
}
