//! # Cartesian product of lists
//!
//! Solution for the "cartesian product of two or more lists" entry in rosetta code

use std::collections::VecDeque;

/// Computes the cartesian product of two or more lists of clonable elements.
///
/// Uses a queue to iterate over the elements (Breadth-First-Search).
///
fn cartesian_product<T: Clone>(lists: &[&[T]]) -> Vec<Vec<T>> {
    // If there are no lists, just returns an empty list.
    if lists.is_empty() {
        return vec![];
    }
    // Fills the queue with the first list.
    let mut queue = lists[0]
        .iter()
        .map(|value| vec![value.clone()])
        .collect::<VecDeque<Vec<T>>>();

    let mut result = vec![];

    while let Some(next) = queue.pop_front() {
        // find the index of the next list to multiply with.
        let next_list = next.len();

        if lists.len() == next_list {
            // If there are no more lists to multiply with, save the result.
            result.push(next)
        } else {
            // Else add the products to the queue.
            for value_to_add in lists[next_list] {
                let mut nouv = next.clone();
                nouv.push(value_to_add.clone());
                queue.push_back(nouv);
            }
        }
    }
    result
}

fn main() {
    println!("\n{:?}", cartesian_product(&[&[1, 2], &[3, 4]]));
    println!("\n{:?}", cartesian_product(&[&[3, 4], &[1, 2]]));
    /*
        This will print :

        [[1, 3], [1, 4], [2, 3], [2, 4]]

        [[3, 1], [3, 2], [4, 1], [4, 2]]
    */
}

#[cfg(test)]
mod tests {
    use super::cartesian_product;
    #[test]
    fn basic_test() {
        assert_eq!(
            cartesian_product(&[&[1, 2], &[3, 4]]),
            vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4]]
        );
    }

    #[test]
    fn empty_list() {
        assert_eq!(
            cartesian_product(&[&[1, 2], &[], &[3, 4]]),
            Vec::<Vec<i32>>::new()
        );
    }

    #[test]
    fn many_lists_different_length() {
        assert_eq!(
            cartesian_product(&[&[1, 2], &[4], &[3, 4, 7]]),
            vec![
                vec![1, 4, 3],
                vec![1, 4, 4],
                vec![1, 4, 7],
                vec![2, 4, 3],
                vec![2, 4, 4],
                vec![2, 4, 7],
            ]
        );
    }

    #[test]
    fn different_types() {
        assert_eq!(
            cartesian_product(&[&[true, false], &[true], &[false, true]]),
            vec![
                vec![true, true, false],
                vec![true, true, true],
                vec![false, true, false],
                vec![false, true, true],
            ]
        );
    }

    #[test]
    fn no_list() {
        assert_eq!(cartesian_product::<i32>(&[]), Vec::<Vec<i32>>::new());
    }
}
