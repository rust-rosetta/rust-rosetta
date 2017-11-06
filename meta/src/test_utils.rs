//! Utilities to assist in testing tasks in this repository.

use std::fmt::Debug;

/// Generates a comprehensive test suite for a sorting algorithm.
///
/// This macro can be used to test any sort function that sorts a mutable slice of objects that
/// implement the `Ord` or `PartialOrd` traits.
///
/// # Example
///
/// ```
/// # #[allow(dead_code)]
/// fn sort<E>(elements: &mut [E]) where E: Ord {
///     elements.sort();
/// }
///
/// #[cfg(test)]
/// mod tests {
///     test_sort!(super::sort);
/// }
/// ```
#[macro_export]
macro_rules! test_sort {
    ( $function:path $(, #[$attr:meta])* ) => {
        use $crate::rand::Rng;

        macro_rules! test_case {
           ($name:ident => $values:expr) => {
                $(#[$attr])*
                #[test]
                fn $name() {
                    let mut values = $values;
                    $function(&mut values);
                    $crate::test_utils::check_sorted(&values);
                }
            }
        }

        test_case!(already_sorted => [-1i32, 0, 3, 6, 99]);
        test_case!(array_of_strings => ["beach", "hotel", "airplane", "car", "house", "art"]);
        test_case!(empty_vector => Vec::<i32>::new());
        test_case!(one_element_vector => vec![0_i32]);
        test_case!(random_numbers => {
                       let mut rng = $crate::rand::thread_rng();
                       rng.gen_iter::<i32>().take(10).collect::<Vec<i32>>()
                   });
        test_case!(reverse_sorted_array => [20_i32, 10, 0, -1, -5]);
        test_case!(unsorted_array => [4_i32, 65, 2, -31, 0, 99, 2, 83, 782, 1]);
        test_case!(unsorted_array_positive => [12_i32, 54, 2, 93, 13, 43, 15, 299, 234]);
        test_case!(unsorted_vector_positive => vec![1_i32, 9, 4, 7, 6, 5, 3, 2, 8]);
        test_case!(vector_with_repeated_elements => vec![1_i32, 1, 1, 1, 1]);
    }
}

/// Check if a slice is sorted properly.
pub fn check_sorted<E>(candidate: &[E])
where
    E: Ord + Clone + Debug,
{
    let sorted = {
        let mut copy = candidate.iter().cloned().collect::<Vec<_>>();
        copy.sort();
        copy
    };

    assert_eq!(sorted.as_slice(), candidate);
}

#[cfg(test)]
mod tests {

    #[test]
    fn check_sorted() {
        let sorted = vec![1, 2, 3, 4, 5];

        super::check_sorted(&sorted);
    }

    #[test]
    #[should_panic]
    fn check_unsorted() {
        let unsorted = vec![1, 3, 2];

        super::check_sorted(&unsorted);
    }
}
