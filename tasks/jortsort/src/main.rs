use std::cmp::{Ord, Eq};

fn jortsort<T: Ord + Eq + Clone>(array: &[T]) -> bool {
    // sort the array
    let mut sorted_array = array.to_vec();
    sorted_array.sort();

    // compare to see if it was originally sorted
    for i in 0..array.len() {
        if array[i] != sorted_array[i] {
            return false;
        }
    }

    true
}

fn main() {
    let unsorted = [1, 3, 2];
    if jortsort(&unsorted) {
        println!("{:?} is sorted.", unsorted);
    } else {
        println!("{:?} is unsorted.", unsorted);
    }
}

#[cfg(test)]
mod tests {
    use super::jortsort;

    #[test]
    fn sorted() {
        let sorted = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert!(jortsort(&sorted))
    }

    #[test]
    fn unsorted() {
        let unsorted = [1, 3, 2];
        assert!(!jortsort(&unsorted))
    }
}
