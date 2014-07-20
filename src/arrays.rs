// Implements http://rosettacode.org/wiki/Arrays

#[cfg(not(test))]
fn main() {}

#[test]
fn create_array() {
	let empty_vec: Vec<int> = vec![];
	assert!(empty_vec.len() == 0);

	let prepopulated_vec = vec![1i, 2i, 3i];
	assert!(prepopulated_vec.len() == 3);

	// Three strings.
	let string_vec = vec!(
		"Foo".to_string(),
		"Bar".to_string(),
		"Baz".to_string()
	);
	assert!(string_vec.len() == 3);
}

#[test]
fn add_to_array() {
	// Mutatable adding.
	let mut a_vec: Vec<int> = vec![];
	a_vec.push(1i);
	assert!(a_vec == vec![1i]);

	// Immutable adding.
	let b_vec = vec![2i, 3i, 4i];
	let c_vec = a_vec.append(b_vec.as_slice());
	assert!(c_vec == vec![1i, 2i, 3i, 4i]);
}

#[test]
fn retrieving_from_array() {
	// Indexing.
	let a_vec = vec![1i, 2i, 3i];
	assert!(a_vec[0] == 1i);

	// A full copy of the vector, but mutable.
	let mut mut_vec = a_vec.clone();
	assert!(mut_vec.pop() == Some(3i));
	assert!(mut_vec == vec![1i, 2i]);
}
