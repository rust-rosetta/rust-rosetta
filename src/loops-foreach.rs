// Implements http://rosettacode.org/wiki/Loops/For

use std::collections::HashMap;

fn main() {
    // Iterate through the characters of a string
    let s = "hello, world!";
    for i in s.chars() {
        print!("{}", i);
    }
    println!("");

    // Iterate through the elements of a slice
    let array = [1, 2, 3, 4, 5];
    for i in array.iter() {
        print!("{}", i);
    }
    println!("");

    // Iterate through the elements of a hasmap
    let mut hashmap = HashMap::new();
    hashmap.insert("a", 1u8);
    hashmap.insert("b", 2);
    hashmap.insert("c", 3);
    for (c, i) in hashmap.iter() {
        println!("{}: '{}'", c, i)
    }
}
