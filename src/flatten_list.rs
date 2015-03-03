//http://rosettacode.org/wiki/Flatten_a_list

#![feature(box_syntax)]
use std::fmt;
use Tree::{Node, Leaf};

#[derive(Debug)]
enum Tree<T>{
    Node(Vec<Tree<T>>),
    Leaf(T),
}

/// fmt::Display is implemented here for pretty printing.
impl<T> fmt::Display for Tree<T>
    where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match *self {
            Node(ref vec) => write!(f, "{}", vec),
            Leaf(ref val) => write!(f, "{}", val)
        }
    }
}

impl<T> fmt::Display for Vec<Tree<T>>
    where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        try!(write!(f, "["));
        for x in self.iter(){
            if !first { try!(write!(f, ", ")); }
            try!(write!(f, "{}", *x));
            first = false;
        }
        write!(f, "]")
    }
}

fn flatten<T>(tree: Tree<T>) -> Vec<T>{
    match tree {
        Leaf(val) => vec![val],
        Node(vec) => vec.into_iter()
                        .flat_map(|t| flatten(t).into_iter())
                        .collect()
    }
}

#[cfg(not(test))]
fn main() {
    //[[1], 2, [[3,4], 5], [[[]]], [[[6]]], 7, 8, []]
    let list: Tree<i32> =
        Node(vec![Node(vec![Leaf(1)]),
            Leaf(2),
            Node(vec![Node(vec![Leaf(3), Leaf(4)]), Leaf(5)]),
            Node(vec![Node(vec![Node(vec![])])]),
            Node(vec![Node(vec![Node(vec![Leaf(6)])])]),
            Leaf(7),
            Leaf(8),
            Node(vec![])
            ]);

    println!("{}", list);

    let flattened = flatten(list);

    println!("{:?}", flattened);
}

#[test]
fn rosetta_flatten_test() {
    //[[1], 2, [[3,4], 5], [[[]]], [[[6]]], 7, 8, []]
    let list: Tree<i32> =
        Node(vec![Node(vec![Leaf(1)]),
            Leaf(2),
            Node(vec![Node(vec![Leaf(3), Leaf(4)]), Leaf(5)]),
            Node(vec![Node(vec![Node(vec![])])]),
            Node(vec![Node(vec![Node(vec![Leaf(6)])])]),
            Leaf(7),
            Leaf(8),
            Node(vec![])
            ]);

    println!("{}", list);

    let flattened = flatten(list);

    assert!(flattened == vec!(1,2,3,4,5,6,7,8))
}
