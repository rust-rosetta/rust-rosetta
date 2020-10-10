fn comb<T>(slice: &[T], k: usize) -> Vec<Vec<T>>
where
    T: Copy,
{
    // If k == 1, return a vector containing a vector for each element of the slice.
    if k == 1 {
        return slice.iter().map(|x| vec![*x]).collect::<Vec<Vec<T>>>();
    }
    // If k is exactly the slice length, return the slice inside a vector.
    if k == slice.len() {
        return vec![slice.to_vec()];
    }
    // Make a vector from the first element + all combinations of k - 1 elements of the rest of the slice.
    let mut result = comb(&slice[1..], k - 1)
        .into_iter()
        .map(|x| [&slice[..1], x.as_slice()].concat())
        .collect::<Vec<Vec<T>>>();
    // Extend this last vector with the all the combinations of k elements after from index 1 onward.
    result.extend(comb(&slice[1..], k));
    // Return final vector.
    return result;
}

fn main() {
    let vec1 = vec![1, 2, 3, 4, 5];
    println!("{:?}", comb(&vec1, 3));

    let vec2 = vec!["A", "B", "C", "D", "E"];
    println!("{:?}", comb(&vec2, 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_numbers_choose_three() {
        let computed = comb(&vec![1, 2, 3, 4, 5], 3);
        let expected = vec![
            vec![1, 2, 3],
            vec![1, 2, 4],
            vec![1, 2, 5],
            vec![1, 3, 4],
            vec![1, 3, 5],
            vec![1, 4, 5],
            vec![2, 3, 4],
            vec![2, 3, 5],
            vec![2, 4, 5],
            vec![3, 4, 5],
        ];

        assert_eq!(computed, expected);
    }

    #[test]
    fn four_letters_choose_two() {
        let computed = comb(&vec!["h", "e", "l", "l", "o"], 2);
        let expected = vec![
            vec!["h", "e"],
            vec!["h", "l"],
            vec!["h", "l"],
            vec!["h", "o"],
            vec!["e", "l"],
            vec!["e", "l"],
            vec!["e", "o"],
            vec!["l", "l"],
            vec!["l", "o"],
            vec!["l", "o"],
        ];

        assert_eq!(computed, expected);
    }
}
