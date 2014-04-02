// Implements http://rosettacode.org/wiki/Loops/For
extern crate collections;

use collections::hashmap::HashMap;

fn main() {
    let s = "hello, world!";
    for i in s.chars() {
        print!("{}", i);
    }
    println!("");

    let array = [1, 2, 3, 4, 5];
    for i in array.iter() {
        print!("{}", i);
    }
    println!("");

    let mut hashmap = HashMap::new();
    hashmap.insert("a", 1);
    hashmap.insert("b", 2);
    hashmap.insert("c", 3);
    for (c, i) in hashmap.iter() {
        println!("{}: '{}'", c, i)
    }
}
