use std::collections::HashMap;

fn main() {
    // Rust's for-loop already is a foreach-loop.
    let collection = vec![1, 2, 3, 4, 5];
    for elem in collection {
        println!("{}", elem);
    }

    // Do note that Rust moves values by default and doesn't copy them. A vector would be unusable
    // after looping over it like above. To preserve it, borrow it or use an Iter, to mutate values
    // do a mutable borrow or create an IterMut. To get an immutable reference omit the mut-part.

    let mut collection = vec![1, 2, 3, 4, 5];
    for mut_ref in &mut collection {
        // alternatively:
        // for mut_ref in collection.iter_mut() {
        *mut_ref *= 2;
        println!("{}", *mut_ref);
    }

    // immutable borrow
    for immut_ref in &collection {
        // alternatively:
        // for immut_ref in collection.iter() {
        println!("{}", *immut_ref);
    }

    // Iterate through the characters of a string
    let s = "hello, world!";
    for i in s.chars() {
        print!("{}", i);
    }
    println!("");

    // Iterate through the elements of a slice
    let array = [1, 2, 3, 4, 5];
    for i in &array {
        print!("{}", i);
    }
    println!("");

    // Iterate through the elements of a hashmap
    let mut hashmap = HashMap::new();
    hashmap.insert("a", 1u8);
    hashmap.insert("b", 2);
    hashmap.insert("c", 3);
    for (c, i) in &hashmap {
        println!("{}: '{}'", c, i)
    }
}
