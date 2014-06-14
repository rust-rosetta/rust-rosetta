// Implements http://rosettacode.org/wiki/99_Bottles_of_Beer
use std::string::String;

#[cfg(not(test))]
fn main() {
	for num_bottles in std::iter::range_step_inclusive(99, 1, -1) {
        let num_bottles=num_bottles as uint;

		println!("{}",bottles_line(num_bottles, true));
		println!("{}",bottles_line(num_bottles, false));
		println!("Take one down, pass it around...");
		println!("{}",bottles_line(num_bottles - 1, true));
		println!("-----------------------------------");
	}
}

fn bottles_line(num_bottles: uint, on_the_wall: bool) -> String {
    let mut ln = match num_bottles {
        0 => "No bottles".to_string(),
        1 => "One bottle".to_string(),
        n => format!("{} bottles", n)
    };

    ln.push_str(" of beer");

	if on_the_wall {
		ln.push_str(" on the wall!");
	}

	ln.push_str("\n");
    ln
}

#[test]
fn gen_bottle_line() {
    let ln=bottles_line(42, false);
    let ln2=bottles_line(42, true);

    assert_eq!(ln.as_slice(),"42 bottles of beer\n");
    assert_eq!(ln2.as_slice(),"42 bottles of beer on the wall!\n");
}
