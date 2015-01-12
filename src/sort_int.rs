// Implements http://rosettacode.org/wiki/Sort_an_integer_array
#![allow(unstable)]
#[cfg(not(test))]
fn main() {
    let mut a = vec!(9us, 8, 7, 6, 5, 4, 3, 2, 1, 0);

    // Merge sort in place, allocates ~2*n memory
    a.sort();
    println!("{:?}", a);
}

#[test]
fn test_sort() {
    let mut a = vec![3us, 1, 4, 1, 5, 9];
    a.sort();
    assert!(a == vec![1us, 1, 3, 4, 5, 9]);
}
