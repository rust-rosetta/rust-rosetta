pub fn sort_disjoint(values: &mut [i32], indices: &[usize]) {
    let mut sublist_indices = indices.to_owned();
    sublist_indices.sort();
    let mut sublist: Vec<i32> = sublist_indices.iter().map(|&i| values[i]).collect();
    sublist.sort();
    for i in 0..sublist.len() {
        values[sublist_indices[i]] = sublist[i];
    }
}

fn main() {
    let mut values = [7, 6, 5, 4, 3, 2, 1, 0];
    let indices = [6, 1, 7];
    sort_disjoint(&mut values, &indices);
    println!("{:?}", values);
}

#[cfg(test)]
mod tests {
    use super::sort_disjoint;
    #[test]
    fn test_example() {
        let mut values = [7, 6, 5, 4, 3, 2, 1, 0];
        let indices = [6, 1, 7];
        sort_disjoint(&mut values, &indices);
        assert_eq!(values, [7, 0, 5, 4, 3, 2, 1, 6]);
    }
    #[test]
    fn test_sort_one() {
        let mut values = [0];
        let indices = [0];
        sort_disjoint(&mut values, &indices);
        assert_eq!(values, [0]);
    }
    #[test]
    fn test_sort_all() {
        let mut values = [7, 6, 5, 4, 3, 2, 1, 0];
        let indices = [6, 1, 7, 2, 0, 4, 3, 5];
        sort_disjoint(&mut values, &indices);
        assert_eq!(values, [0, 1, 2, 3, 4, 5, 6, 7]);
    }
}
