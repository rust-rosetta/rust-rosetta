use std::io;

fn main() {
	let mut max_lengths: Vec<uint> = Vec::new();
	let mut inputs: Vec<Vec<String>> = Vec::new();

	for line in io::stdin().lines() {
		let mut input = line.unwrap();
		input.pop_char();
		let split_input: Vec<String> = input.as_slice().split('$').map(|s| s.to_string()).collect();
		inputs.push(split_input.clone());
		let v: Vec<uint> = split_input.iter().map(|chunk| chunk.len() ).collect();

		for i in range(0u, v.len()) {
			if i < max_lengths.len() {
				*max_lengths.get_mut(i) = std::cmp::max(max_lengths[i], v[i]);
			} else {
				max_lengths.push(v[i]);
			}
		}
	}

	// left aligned
	for i in range(0u, inputs.len()) {
		for j in range(0u, inputs[i].len()) {
			print!("{0:<1$}", inputs[i][j], 1 + max_lengths[j]);
		}
		println!("");
	}
	println!("");
	// right aligned
	for i in range(0u, inputs.len()) {
		for j in range(0u, inputs[i].len()) {
			print!("{0:>1$}", inputs[i][j], 1 + max_lengths[j]);
		}
		println!("");
	}
	println!("");
	// center aligned
	for i in range(0u, inputs.len()) {
		for j in range(0u, inputs[i].len()) {
			let ref string: String = inputs[i][j];
			let spaces: uint = 1 + max_lengths[j] - string.len();
			for k in range(0u, spaces >> 2) {
				print!(" ");
			}
			print!("{}", string);
			for k in range(0u, spaces - (spaces >> 2)) {
				print!(" ");
			}
		}
		println!("");
	}
}