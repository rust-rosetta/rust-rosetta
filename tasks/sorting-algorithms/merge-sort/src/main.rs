//! This is an idiomatic-but-slow implementation. A more efficient implementation
//! would use `unsafe` to avoid allocating so many temporary vectors.

fn merge_sort<E: PartialOrd + Clone>(arr: &[E]) -> Vec<E> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }
    let midpoint = arr.len() / 2;
    let left = merge_sort(&arr[0..midpoint]);
    let right = merge_sort(&arr[midpoint..]);
    merge(&left[..], &right[..])
}

fn merge<E: PartialOrd + Clone>(left: &[E], right: &[E]) -> Vec<E> {
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

pub fn main() {
    let arr = [1i32, 9, 3, 2, 1003, 23, -123, 7];
    let sorted = merge_sort(&arr);
    println!("{:?}", sorted);
}

#[cfg(test)]
mod tests {
    use super::merge_sort;

    #[test]
    fn sorted() {
        let arr = [1i32, 2, 3, 4, 6, 8];
        assert_eq!(merge_sort(&arr), arr.to_vec());
    }

    #[test]
    fn reverse() {
        let arr = [8i32, 6, 4, 3, 2, 1];
        assert_eq!(merge_sort(&arr), vec![1i32, 2, 3, 4, 6, 8]);
    }

    #[test]
    fn random() {
        let arr = [12i32, 54, 2, 93, 13, 43, 15, 299, 234];
        assert_eq!(merge_sort(&arr),
                   vec![2i32, 12, 13, 15, 43, 54, 93, 234, 299]);
    }
}
