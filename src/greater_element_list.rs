// http://rosettacode.org/wiki/Greatest_element_of_a_list
#![cfg(not_tested)]

fn main()
{
	/*
		iter(): http://static.rust-lang.org/doc/master/core/iter/index.html
		max_by(): http://static.rust-lang.org/doc/master/core/iter/trait.Iterator.html#method.max_by
		unwrap(): http://static.rust-lang.org/doc/master/core/option/type.Option.html#method.unwrap
	*/

	//greater element of a list main
	let list1 = vec!(1,2,3,4,5,6,7,8,9);
	println!("Max list {}", list1.iter().max_by(|x|{*x}).unwrap());

	let list2 = vec!(123,3543,23,432,5,2,34,234,234,2,4,234,23,4,24,25,7,658,68);
	println!("Max list {}", list2.iter().max_by(|x|{*x}).unwrap());

	let list3 = vec!('a','t','f','y','q','p','h');
	println!("Max list {}", list3.iter().max_by(|x|{*x}).unwrap());

	let list4 = vec!("Bonjour", "Hello", "Holla", "Hallo", "Bongiorno");
	println!("Max list {}", list4.iter().max_by(|x|{*x}).unwrap());
}
