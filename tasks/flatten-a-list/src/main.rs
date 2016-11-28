#![feature(box_syntax)]

use Tree::{Node, Leaf};

#[derive(Debug)]
enum Tree<T> {
    Node(Vec<Tree<T>>),
    Leaf(T),
}

fn flatten<T>(tree: Tree<T>) -> Vec<T> {
    match tree {
        Leaf(val) => vec![val],
        Node(vec) => {
            vec.into_iter()
                .flat_map(|t| flatten(t).into_iter())
                .collect()
        }
    }
}

fn main() {
    // [[1], 2, [[3,4], 5], [[[]]], [[[6]]], 7, 8, []]
    let list: Tree<i32> = Node(vec![Node(vec![Leaf(1)]),
                                    Leaf(2),
                                    Node(vec![Node(vec![Leaf(3), Leaf(4)]), Leaf(5)]),
                                    Node(vec![Node(vec![Node(vec![])])]),
                                    Node(vec![Node(vec![Node(vec![Leaf(6)])])]),
                                    Leaf(7),
                                    Leaf(8),
                                    Node(vec![])]);

    println!("{:?}", list);

    let flattened = flatten(list);

    println!("{:?}", flattened);
}

#[test]
fn rosetta_flatten_test() {
    // [[1], 2, [[3,4], 5], [[[]]], [[[6]]], 7, 8, []]
    let list: Tree<i32> = Node(vec![Node(vec![Leaf(1)]),
                                    Leaf(2),
                                    Node(vec![Node(vec![Leaf(3), Leaf(4)]), Leaf(5)]),
                                    Node(vec![Node(vec![Node(vec![])])]),
                                    Node(vec![Node(vec![Node(vec![Leaf(6)])])]),
                                    Leaf(7),
                                    Leaf(8),
                                    Node(vec![])]);

    println!("{:?}", list);

    let flattened = flatten(list);

    assert!(flattened == vec![1, 2, 3, 4, 5, 6, 7, 8])
}
