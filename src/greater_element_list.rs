// http://rosettacode.org/wiki/Greatest_element_of_a_list

fn max_list(list : &[i32]) -> i32
{
	assert!(list.len() > 0,"Empty list not allowed");
	let mut max = 0;
	for &value in list.iter() {
		//returns max of both value
		max = std::cmp::max(max,value);
	}
	return max;
}

//find greater element of a list generic
//the type T must implement TotalOrd to sort
//and Clone to clone the value from the list
fn max_list_gen<T : TotalOrd + Clone>(list : &[T]) -> T
{
	assert!(list.len() > 0, "Empty list not allowed");

	//need to clone the value, "list" being a immutable
	let mut max = list[0].clone();

	for n in range(1, list.len()) {
		let value = list[n].clone();
		//max of both value, require TotalOrd
		max = std::cmp::max(max,value);
	}
	return max;
}

fn main()
{
	//greater element of a list main
	println!("Max of first list {}", max_list([1,2,3,4,5,6,7]));
	println!("Max of the second list {}", max_list([123,3543,23,432,5,2,34,234,234,2,4,234,23,4,24,25,7,658,68]));
	println!("Max of list char {}", max_list_gen::<char>(['a','b','t','r','s',';','d','b']));
	println!("Max of list int {}", max_list_gen::<int>([123,3543,23,432,5,2,34,234,234,2,4,234,23,4,24,25,7,658,68]));
}
