// http://rosettacode.org/wiki/Greatest_element_of_a_list

// not_tested
fn main()
{
	//greater element of a list main
	let list1 = vec!(1,2,3,4,5,6,7,8,9);
	println!("Max of first list {}", list1.iter().max_by(|x| *x ).unwrap());

	let list2 = vec!(123,3543,23,432,5,2,34,234,234,2,4,234,23,4,24,25,7,658,68);
	println!("Max of the second list {}", list2.iter().max_by(|x| *x ).unwrap());

	let list3 = vec!('a','b','c','d','e');
	println!("Max of Third list {}", list3.iter().max_by(|x| *x ).unwrap());

	let list4 = vec!("Bonjour","Hola","Hello","Hallo","Bongiorno");
	println!("Max of Forth list {}", list4.iter().max_by(|x| *x ).unwrap());
}
