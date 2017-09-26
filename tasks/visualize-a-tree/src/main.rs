#[macro_use]
extern crate serde_derive;

extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate term_painter;

use std::cmp::Ordering;
use std::env;
use std::fmt::{Debug, Display, Formatter, Result};

use rand::Rng;
use term_painter::Color::*;
use term_painter::ToStyle;

type NodePtr = Option<usize>;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Side {
    Left,
    Right,
    Up,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum DisplayElement {
    TrunkSpace,
    SpaceLeft,
    SpaceRight,
    SpaceSpace,
    Root,
}

impl DisplayElement {
    fn string(&self) -> String {
        match *self {
            DisplayElement::TrunkSpace => "    │   ".to_string(),
            DisplayElement::SpaceRight => "    ┌───".to_string(),
            DisplayElement::SpaceLeft => "    └───".to_string(),
            DisplayElement::SpaceSpace => "        ".to_string(),
            DisplayElement::Root => "├──".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
struct Node<K, V> {
    key: K,
    value: V,
    left: NodePtr,
    right: NodePtr,
    up: NodePtr,
}

impl<K: Ord + Copy, V: Copy> Node<K, V> {
    pub fn new(k: K, v: V) -> Node<K, V> {
        Node {
            key: k,
            value: v,
            left: None,
            right: None,
            up: None,
        }
    }

    pub fn set_ptr(&mut self, side: Side, to: NodePtr) {
        let field = match side {
            Side::Up => &mut self.up,
            Side::Left => &mut self.left,
            _ => &mut self.right,
        };
        *field = to;
    }

    pub fn get_ptr(&self, side: Side) -> NodePtr {
        match side {
            Side::Up => self.up,
            Side::Left => self.left,
            _ => self.right,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Tree<K, V> {
    root: NodePtr,
    store: Vec<Node<K, V>>,
}

impl<K: Ord + Copy + Debug + Display, V: Debug + Copy + Display> Tree<K, V> {
    pub fn get_node(&self, np: NodePtr) -> Node<K, V> {
        assert!(np.is_some());
        self.store[np.unwrap()]
    }

    pub fn get_key(&self, np: NodePtr) -> K {
        assert!(np.is_some());
        self.store[np.unwrap()].key
    }

    pub fn _get_value(&self, np: NodePtr) -> V {
        assert!(np.is_some());
        self.store[np.unwrap()].value
    }

    pub fn get_pointer(&self, np: NodePtr, side: Side) -> NodePtr {
        assert!(np.is_some());
        self.store[np.unwrap()].get_ptr(side)
    }

    pub fn set_pointer(&mut self, np: NodePtr, side: Side, to: NodePtr) {
        assert!(np.is_some());
        self.store[np.unwrap()].set_ptr(side, to);
    }

    pub fn new() -> Self {
        Tree {
            root: None,
            store: Vec::<Node<K, V>>::with_capacity(128),
        }
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<Node<K, V>> {
        let mut n = Node::new(k, v);

        if self.root.is_none() {
            assert!(self.store.len() == 0);
            self.store.push(n);
            self.root = Some(0);
            return Some(n);
        }

        let mut p = self.root;      // Possibly None
        let mut prev = p;
        let mut side = Side::Left;
        while let Some(_) = p {
            prev = p;
            match n.key.cmp(&self.get_key(p)) {
                Ordering::Less => {
                    side = Side::Left;
                    p = self.get_pointer(p, side);
                }
                Ordering::Greater => {
                    side = Side::Right;
                    p = self.get_pointer(p, side);
                }
                Ordering::Equal => {
                    // Key exists
                    return None;
                }
            }
        }
        // Set child's pointer
        n.up = prev;
        // Stow the node
        self.store.push(n);
        // Set parent's pointer
        let ptr = Some(self.store.len() - 1);
        self.set_pointer(prev, side, ptr);
        Some(n)
    }

    // Prints the tree with root p.  The idea is to do an in-order traversal
    // (reverse in-order in this case, where right is on top), and print nodes as they
    // are visited, one per line. Each invocation of display() gets its own copy
    // of the display element vector e, which is grown with either whitespace or
    // a trunk element, then modified in its last and possibly second-to-last
    // characters in context.
    fn display(&self, p: NodePtr, side: Side, e: &[DisplayElement], f: &mut Formatter) {
        if p.is_none() {
            return;
        }

        let mut elems = e.to_vec();
        let node = self.get_node(p);
        let mut tail = DisplayElement::SpaceSpace;
        if node.up != self.root {
            // If the direction is switching, I need the trunk element to appear in the lines
            // printed before that node is visited.
            if side == Side::Left && node.right.is_some() {
                elems.push(DisplayElement::TrunkSpace);
            } else {
                elems.push(DisplayElement::SpaceSpace);
            }
        }
        let hindex = elems.len() - 1;
        self.display(node.right, Side::Right, &elems, f);

        if p == self.root {
            elems[hindex] = DisplayElement::Root;
            tail = DisplayElement::TrunkSpace;
        } else if side == Side::Right {
            // Right subtree finished
            elems[hindex] = DisplayElement::SpaceRight;
            // Prepare trunk element in case there is a left subtree
            tail = DisplayElement::TrunkSpace;
        } else if side == Side::Left {
            elems[hindex] = DisplayElement::SpaceLeft;
            let parent = self.get_node(node.up);
            if parent.up.is_some() && self.get_pointer(parent.up, Side::Right) == node.up {
                // Direction switched, need trunk element starting with this node/line
                elems[hindex - 1] = DisplayElement::TrunkSpace;
            }
        }

        // Visit node => print accumulated elements. Each node gets a line and each line gets a
        // node.
        for e in elems.clone() {
            let _ = write!(f, "{}", e.string());
        }
        let _ = write!(f,
                       "{key:>width$} ",
                       key = Green.bold().paint(node.key),
                       width = 2);
        let _ = write!(f,
                       "{value:>width$}\n",
                       value = Blue.bold().paint(format!("{:.*}", 2, node.value)),
                       width = 4);

        // Overwrite last element before continuing traversal
        elems[hindex] = tail;

        self.display(node.left, Side::Left, &elems, f);
    }
}

impl<K: Ord + Copy + Debug + Display, V: Debug + Copy + Display> Display for Tree<K, V> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.root.is_none() {
            write!(f, "[empty]")
        } else {
            let v: Vec<DisplayElement> = vec![];
            self.display(self.root, Side::Up, &v, f);
            Ok(())
        }
    }
}

fn random_tree(n: u32) -> Tree<i32, f32> {
    let mut tree: Tree<i32, f32> = Tree::new();
    let mut rng = rand::thread_rng();
    tree.insert(0, rng.gen_range(-1f32, 1f32));
    for _ in 0..n - 1 {
        tree.insert(rng.gen_range(-(n as i32) / 2, (n as i32) / 2),
                    rng.gen_range(-1f32, 1f32));
    }
    tree
}

/// Prints json representation of tree as well, useful for shorter code listings.
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut r_nodes: u32 = 20;

    match args.len() {
        1 => {}
        2 => {
            r_nodes = args[1].parse::<u32>().unwrap();
        }
        _ => {
            println!("visualize_a_tree <no. of nodes>");
            return;
        }
    }

    let tree = random_tree(r_nodes);
    let encoded = serde_json::to_string(&tree).unwrap();

    println!("{}", tree);
    println!("{}", encoded);

    println!("\nCanned tree, decoded:");
    _main_for_rosetta()
}

/// Decodes and prints a previously generated tree.  Presenting this with display(), the fmt()
/// overload, and the struct & enum definitions is sufficient for this demonstration.
fn _main_for_rosetta() {
    let encoded = r#"{"root":0,"store":[{"key":0,"value":0.45,"left":1,"right":3,
        "up":null},{"key":-8,"value":-0.94,"left":7,"right":2,"up":0}, {"key":-1,
        "value":0.15,"left":8,"right":null,"up":1},{"key":7, "value":-0.29,"left":4,
        "right":9,"up":0},{"key":5,"value":0.80,"left":5,"right":null,"up":3},
        {"key":4,"value":-0.85,"left":6,"right":null,"up":4},{"key":3,"value":-0.46,
        "left":null,"right":null,"up":5},{"key":-10,"value":-0.85,"left":null,
        "right":13,"up":1},{"key":-6,"value":-0.42,"left":null,"right":10,"up":2},
        {"key":9,"value":0.63,"left":12,"right":null,"up":3},{"key":-3,"value":-0.83,
        "left":null,"right":11,"up":8},{"key":-2,"value":0.75,"left":null,"right":null,
        "up":10},{"key":8,"value":-0.48,"left":null,"right":null,"up":9},{"key":-9,
        "value":0.53,"left":null,"right":null,"up":7}]}"#;
    let tree: Tree<i32, f32> = serde_json::from_str(encoded).unwrap();
    println!("{}", tree);
}

#[cfg(test)]
mod tests {
    use random_tree;
    use serde_json;

    use super::{Side, Tree};

    #[test]
    fn test_insert() {
        let mut tree: Tree<i32, f32> = Tree::new();
        tree.insert(0, 0.0);
        tree.insert(8, 8.8);
        tree.insert(-8, -8.8);
        assert!(tree.insert(4, 4.4).is_some());
        tree.insert(12, 12.12);

        assert_eq!(tree._get_value(tree.get_pointer(tree.root, Side::Left)),
                   -8.8);
        assert_eq!(tree._get_value(tree.get_pointer(tree.get_pointer(tree.root, Side::Right),
                                                    Side::Right)),
                   12.12);
        assert_eq!(tree.get_pointer(tree.get_pointer(tree.get_pointer(tree.root, Side::Right),
                                                     Side::Right),
                                    Side::Left),
                   None);

        tree = random_tree(100);
        assert!(tree.store.len() > 0);
    }

    #[test]
    fn test_decode() {
        let encoded = r#"{"root":0,"store":[{"key":0,"value":0.45,"left":1,"right":3,
            "up":null},{"key":-8,"value":-0.94,"left":7,"right":2,"up":0}, {"key":-1,
            "value":0.15,"left":8,"right":null,"up":1},{"key":7, "value":-0.29,"left":4,
            "right":9,"up":0},{"key":5,"value":0.80,"left":5,"right":null,"up":3},
            {"key":4,"value":-0.85,"left":6,"right":null,"up":4},{"key":3,"value":-0.46,
            "left":null,"right":null,"up":5},{"key":-10,"value":-0.85,"left":null,
            "right":13,"up":1},{"key":-6,"value":-0.42,"left":null,"right":10,"up":2},
            {"key":9,"value":0.63,"left":12,"right":null,"up":3},{"key":-3,"value":-0.83,
            "left":null,"right":11,"up":8},{"key":-2,"value":0.75,"left":null,"right":null,
            "up":10},{"key":8,"value":-0.48,"left":null,"right":null,"up":9},{"key":-9,
            "value":0.53,"left":null,"right":null,"up":7}]}"#;
        let tree: Tree<i32, f32> = serde_json::from_str(encoded).unwrap();
        assert_eq!(tree.root, Some(0));
        assert_eq!(tree.store[4].key, 5);
        assert_eq!(tree.store[4].value, 0.80);
        assert_eq!(tree.store[4].right, None);
        assert_eq!(tree.store[12].up, Some(9));
        assert_eq!(tree.store[9].left, Some(12));
    }
}
