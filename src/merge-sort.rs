// Implements http://rosettacode.org/wiki/Sorting_algorithms/Merge_sort

// This is an idiomatic-but-slow implementation. A more efficient implementation
// would use `unsafe` to avoid allocating so many temporary vectors.

fn merge_sort<E: PartialOrd + Clone>(vec: Vec<E>) -> Vec<E> {
    if vec.len() <= 1 {
        return vec;
    }
    let midpoint = vec.len()/2;
    let left = merge_sort(vec.slice(0u, midpoint).to_vec());
    let right = merge_sort(vec.slice(midpoint, vec.len()).to_vec());
    merge(left, right)
}

fn merge<E: PartialOrd + Clone>(left: Vec<E>, right: Vec<E>) -> Vec<E> {
    let mut merged = Vec::with_capacity(left.len() + right.len());
    let mut i = 0;
    let mut j = 0;
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged.push(left[i].clone());
            i += 1;
        } else {
            merged.push(right[j].clone());
            j += 1;
        }
    }
    while i < left.len() {
        merged.push(left[i].clone());
        i += 1;
    }
    while j < right.len() {
        merged.push(right[j].clone());
        j += 1;
    }
    merged
}

#[cfg(not(test))]
pub fn main() {
    let vec = vec![1i, 9, 3, 2, 1003, 23, -123, 7];
    let sorted = merge_sort(vec);
    println!("{}", sorted.iter().map(|x| x.to_string()).collect::<Vec<String>>().connect(", "));
}

#[cfg(test)]
mod test {
    use super::merge_sort;

    #[test]
    fn sorted() {
        let vec = vec![1u, 2, 3, 4, 6, 8];
        assert_eq!(merge_sort(vec.clone()), vec);
    }

    #[test]
    fn reverse() {
        let vec = vec![8i, 6, 4, 3, 2, 1];
        assert_eq!(merge_sort(vec), vec![1i, 2, 3, 4, 6, 8]);
    }

    #[test]
    fn random() {
        let vec = vec![12u, 54, 2, 93, 13, 43, 15, 299, 234];
        assert_eq!(merge_sort(vec), vec![2u, 12, 13, 15, 43, 54, 93, 234, 299]);
    }
}
