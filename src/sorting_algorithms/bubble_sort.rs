// http://rosettacode.org/wiki/Sorting_algorithms/Bubble_sort

/// Progress through the slice and 'bubble' elements up until they are in order.
#[cfg_attr(feature = "clippy", allow(needless_range_loop))]
fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
    (1..v.len() + 1).rev().all(|length| {
        let mut changes = 0;

        for index in 0..length - 1 {
            if v[index] > v[index + 1] {
                changes += 1;
                v.swap(index, index + 1);
            }
        }

        // Continue to iterate if any 'bubble-ing' took place
        changes > 0
    });
}

fn main() {
    let mut numbers = [4i32, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    bubble_sort(&mut numbers);
}

#[cfg(test)]
mod tests {
    extern crate rust_rosetta;

    use std::fmt::Debug;

    fn check_sort<T>(v: &mut [T])
        where T: Ord + Clone + Debug
    {
        super::bubble_sort(v);

        rust_rosetta::check_sorted(v);
    }

    #[test]
    fn rosetta_vector() {
        let numbers = &mut [4i32, 65, 2, -31, 0, 99, 2, 83, 782, 1];
        check_sort(numbers);
    }

    #[test]
    fn empty_vector() {
        let mut numbers: &mut [i32] = &mut [];
        check_sort(numbers);
    }

    #[test]
    fn one_element_vector() {
        let numbers = &mut [0i32];
        check_sort(numbers);
    }

    #[test]
    fn repeat_vector() {
        let numbers = &mut [1i32, 1, 1, 1, 1];
        check_sort(numbers);
    }

    #[test]
    fn worst_case_vector() {
        let numbers = &mut [20i32, 10, 0, -1, -5];
        check_sort(numbers);
    }

    #[test]
    fn already_sorted_vector() {
        let numbers = &mut [-1i32, 0, 3, 6, 99];
        check_sort(numbers);
    }
}
