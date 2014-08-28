// Implements http://rosettacode.org/wiki/Arrays

#[cfg(not(test))]
fn main() {}

#[test]
fn create_array() {
	let empty_vec: Vec<int> = vec![];
	assert!(empty_vec.len() == 0);

	let prepopulated_vec = vec![1i, 2, 3];
	assert!(prepopulated_vec.len() == 3);

	// Three string slices.
	let string_vec = vec!["Foo", "Bar", "Baz"];
	assert!(string_vec.len() == 3);
}

#[test]
fn add_to_array() {
	// Mutatable adding.
	let mut a_vec = vec![];
	a_vec.push(1i);
	assert_eq!(a_vec[0], 1);

	// Immutable adding.
	let b_vec = vec![2, 3, 4];
	let c_vec = a_vec.append(b_vec.as_slice());
	assert_eq!(c_vec.as_slice(), [1, 2, 3, 4].as_slice());
}

#[test]
fn retrieving_from_array() {
	// Indexing.
	let a_vec = vec![1i, 2, 3];
	assert!(a_vec[0] == 1i);

	// A full copy of the vector, but mutable.
	let mut mut_vec = a_vec.clone();
	assert_eq!(mut_vec.pop(), Some(3));
	assert_eq!(mut_vec.as_slice(), [1, 2].as_slice());
}
