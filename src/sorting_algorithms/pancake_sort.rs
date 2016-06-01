// http://rosettacode.org/wiki/Sorting_algorithms/Pancake_sort

#[cfg_attr(feature="clippy", allow(needless_range_loop))]
fn pancake_sort<T: Ord>(mut v: &mut [T]) {
    let len = v.len();
    // trivial case -- no flips
    if len < 2 {
        return;
    }
    for i in (1..len + 1).rev() {
        // find index of maximum from 0 to i
        let mut max_index = 0;
        for j in 0..i {
            if v[max_index] < v[j] {
                max_index = j;
            }
        }
        // if max_index is not where it's supposed to be
        // do two flips to move it to i - 1
        if max_index != i - 1 {
            flip(&mut v, max_index);
            flip(&mut v, i - 1);
        }
    }
}

// function to flip a section of a mutable collection from 0..num
fn flip<E: PartialOrd>(v: &mut [E], num: usize) {
    for i in 0..(num + 1) / 2 {
        v.swap(i, num - i);
    }
}

fn main() {
    // Sort numbers
    let mut numbers = [4i32, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);
    pancake_sort(&mut numbers);
    println!("After: {:?}", numbers);
    // Sort strings
    let mut strings = ["beach", "hotel", "airplane", "car", "house", "art"];
    println!("Before: {:?}", strings);
    pancake_sort(&mut strings);
    println!("After: {:?}", strings);
}

#[cfg(test)]
mod tests {
    extern crate rust_rosetta;

    use std::fmt::Debug;

    fn check_sort<T>(v: &mut [T])
        where T: Ord + Clone + Debug
    {
        super::pancake_sort(v);
        rust_rosetta::check_sorted(v);
    }

    #[test]
    fn test_rosetta_vector() {
        let numbers = &mut [4i32, 65, 2, -31, 0, 99, 2, 83, 782, 1];
        check_sort(numbers);
    }

    #[test]
    fn test_empty_vector() {
        let mut numbers: Vec<i32> = Vec::new();
        check_sort(&mut numbers[..]);
    }

    #[test]
    fn test_one_element_vector() {
        let numbers = &mut [0i32];
        check_sort(numbers);
    }

    #[test]
    fn test_repeat_vector() {
        let numbers = &mut [1i32, 1, 1, 1, 1];
        check_sort(numbers);
    }

    #[test]
    fn test_string_vector() {
        let strings = &mut ["beach", "hotel", "airplane", "car", "house", "art"];
        check_sort(strings);
    }
}
