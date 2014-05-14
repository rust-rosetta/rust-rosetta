// http://rosettacode.org/wiki/Greatest_element_of_a_list

#![cfg(not_tested)]
/*
	Generic function to find max value in list.
	return an Option that can contains a reference to the max of the list

	The type T requires to implement Ord to be compared.

	'a is used to define the lifetime of the returned reference.
	<'a, ...>: define a lifetime named 'a.
	(lst: &'a Vec<T>): 'a is equal to the lifetime of lst
	Option<&'a T>> the returned reference is defined to have the lifetime 'a, which is the same lifetime of the vector
	This means that the reference contained in the Option will be valid as long as the vector is valid.
	In fact, this mechanism exist to avoid having a refence to something that does not exist anymore. One of the Rust safety mechanism.

	For more information about lifetime, see http://static.rust-lang.org/doc/master/guide-lifetimes.html
*/
fn max_list<'a, T: Ord>(lst: &'a Vec<T>) -> Option<&'a T> {

	let mut max = None; //None is one of the Option values

	for i in lst.iter() {
		max = match max { //Max being Option(which is an enum), we deconstruct max to get its  possible values
			None => Some(i), //if no max. some wrap i in an Option.
			Some(ref m) if i > *m => Some(i), //m is a reference of the value contained in max
			_ => max //default value if i <= max
		}
	}
	max
}

fn main()
{
	//greater element of a list main
	let list1 = vec!(1,2,3,4,5,6,7,8,9);
	println!("Max of first list {}", max_list(&list1).unwrap());

	let list2 = vec!(123,3543,23,432,5,2,34,234,234,2,4,234,23,4,24,25,7,658,68);
	println!("Max of the second list {}", max_list(&list2).unwrap());

	let list3 = vec!('a','b','c','d','e');
	println!("Max of first list {}", max_list(&list3).unwrap());
}
