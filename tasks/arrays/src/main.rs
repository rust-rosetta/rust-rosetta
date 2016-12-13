fn main() {}

#[test]
fn create_array() {
    let empty_vec: Vec<i32> = vec![];
    assert!(empty_vec.len() == 0);

    let prepopulated_vec = vec![1i32, 2, 3];
    assert!(prepopulated_vec.len() == 3);

    // Three string slices.
    let string_vec = vec!["Foo", "Bar", "Baz"];
    assert!(string_vec.len() == 3);
}

#[test]
fn add_to_array() {
    // Mutatable adding.
    let mut a_vec = vec![];
    a_vec.push(1i32);
    assert_eq!(a_vec[0], 1);

    // Immutable adding.
    let b_vec = vec![2, 3, 4];
    a_vec.extend(b_vec.into_iter());
    assert_eq!(a_vec, [1, 2, 3, 4]);
}

#[test]
fn retrieving_from_array() {
    // Indexing.
    let a_vec = vec![1i32, 2, 3];
    assert!(a_vec[0] == 1i32);

    // A full copy of the vector, but mutable.
    let mut mut_vec = a_vec.clone();
    assert_eq!(mut_vec.pop(), Some(3));
    assert_eq!(mut_vec, [1, 2]);
}
