// http://rosettacode.org/wiki/Sorting_algorithms/Cocktail_sort

// Progress through the slice and 'bubble' elements up and down until they are in order.
#[cfg_attr(feature = "clippy", allow(needless_range_loop))]
fn cocktail_sort<T: PartialOrd>(v: &mut [T]) {
    (1..v.len() + 1).rev().all(|length| {
        let mut swapped: bool = false;
        // bubble up
        for index in 0..length - 1 {
            if v[index] > v[index + 1] {
                swapped = true;
                v.swap(index, index + 1);
            }
        }
        // break if no swap occured before bubbling down
        if !swapped {
            return false;
        }
        // bubble down
        for index in (0..length - 1).rev() {
            if v[index] > v[index + 1] {
                swapped = true;
                v.swap(index, index + 1);
            }
        }
        // Continue to iterate if any swapping took place
        swapped
    });
}

fn main() {
    let mut numbers = [4i32, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);
    cocktail_sort(&mut numbers);
    println!("After: {:?}", numbers);
}

#[cfg(test)]
mod tests {
    extern crate rust_rosetta;

    use std::fmt::Debug;

    fn check_sort<T>(v: &mut [T])
        where T: Ord + Clone + Debug
    {
        super::cocktail_sort(v);
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
    fn turtles_vector() {
        let numbers = &mut [20i32, 10, 0, -1, -5];
        check_sort(numbers);
    }

    #[test]
    fn already_sorted_vector() {
        let numbers = &mut [-1i32, 0, 3, 6, 99];
        check_sort(numbers);
    }
}
