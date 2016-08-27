fn main() {
    let mut a = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

    // Merge sort in place, allocates ~2*n memory
    a.sort();
    println!("{:?}", a);
}

#[test]
fn test_sort() {
    let mut a = vec![3, 1, 4, 1, 5, 9];
    a.sort();
    assert!(a == vec![1, 1, 3, 4, 5, 9]);
}
