#[cfg(not(test))]
fn main() {
	let q_array: Vec<uint> = hofstadter_q_wrapper(1000);
	for i in range(1u, 1+q_array.len()) {
		println!("H({}) = {}", i, q_array[i-1]);
	}
}

// Fill up the array A from indices 0 to maxval-1 (inclusive)
// where A[i] is equal to the Hofstadter Q sequence indexed at i+1
fn hofstadter_q_wrapper(maxval: uint) -> Vec<uint> {
	let mut memoize_vec: Vec<uint> = Vec::from_fn(maxval, |_| 0u);
	// Initialize the first two elements of the array
	*memoize_vec.get_mut(0) = 1;
	*memoize_vec.get_mut(1) = 1;
	// Fill up the array
	for n in range(1u, 1+maxval) {
		hofstadter_q(n, &mut memoize_vec);
	}
	// Return the array
	memoize_vec
}

// Returns the n-th element (counting starts from 1) of the Hofstadter Q sequence
fn hofstadter_q(n: uint, memoize_vec: &mut Vec<uint>) -> uint {
	if (*memoize_vec)[n-1] > 0u {
		(*memoize_vec)[n-1]
	} else {
		// Make the required recursive calls...
		let rec_call_1: uint = hofstadter_q(n - 1, memoize_vec);
		let rec_call_2: uint = hofstadter_q(n - 2, memoize_vec);
		let rec_call_3: uint = hofstadter_q(n - rec_call_1, memoize_vec);
		let rec_call_4: uint = hofstadter_q(n - rec_call_2, memoize_vec);
		// ...update the memoization vector...
		let new_val: uint = rec_call_3 + rec_call_4;
		*memoize_vec.get_mut(n-1) = new_val;
		// ...and return the result
		new_val
	}
}

#[test]
fn test_result() {
	let q_array: Vec<uint> = hofstadter_q_wrapper(1000);
	assert_eq!(q_array[0], 1);
	assert_eq!(q_array[1], 1);
	assert_eq!(q_array[2], 2);
	assert_eq!(q_array[3], 3);
	assert_eq!(q_array[4], 3);
	assert_eq!(q_array[5], 4);
	assert_eq!(q_array[6], 5);
	assert_eq!(q_array[7], 5);
	assert_eq!(q_array[8], 6);
	assert_eq!(q_array[9], 6);
	assert_eq!(q_array[999], 502);
}