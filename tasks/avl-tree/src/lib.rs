/// This implementation uses an addressable vector as the tree's store.
/// It is possible to construct a mutable tree using Rc<RefCell<>>,
/// but it adds some complexity.
///
/// "Pointers" to nodes are indices into the vector store, and have
/// trait Copy.
///
/// The index of a node in the vector store should not be confused with its key.

extern crate rand;
extern crate term_painter;

use rand::Rng;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, Result};
use term_painter::ToStyle;
use term_painter::Color::*;

pub type NodePtr = Option<usize>;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Side {
    Left,
    Right,
    Up,
    Root,
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

/// Handedness of balanced insert and delete operations differs only by values encapsulated here.
struct BalanceConstants {
    bal_incr: i8,
    this_side: Side,
    that_side: Side,
    key_order: Ordering, // Ins only
    // These are used in the +1/-1 & -1/+1 deletion cases
    gcm1_child_adj: i8, // Del only, balance adjustment to child for b = -1 grandchild
    gcm1_parent_adj: i8, // Del only, balance adjustment to parent for b = -1 grandchild
    gcp1_child_adj: i8, // Del only, balance adjustment to child for b = 1 grandchild
    gcp1_parent_adj: i8, // Del only, balance adjustment to parent for b = 1 grandchild
}

static BALANCE_CONSTANTS_A: BalanceConstants = BalanceConstants {
    bal_incr: -1,
    this_side: Side::Left,
    that_side: Side::Right,
    key_order: Ordering::Greater,
    gcm1_child_adj: 0,
    gcm1_parent_adj: 1,
    gcp1_child_adj: -1,
    gcp1_parent_adj: 0,
};

static BALANCE_CONSTANTS_B: BalanceConstants = BalanceConstants {
    bal_incr: 1,
    this_side: Side::Right,
    that_side: Side::Left,
    key_order: Ordering::Less,
    gcm1_child_adj: 1,
    gcm1_parent_adj: 0,
    gcp1_child_adj: 0,
    gcp1_parent_adj: -1,
};

#[derive(Debug, Clone, Copy)]
pub struct Node<K, V> {
    key: K,
    value: V,
    balance: i8,
    left: NodePtr,
    right: NodePtr,
    up: NodePtr,
}

#[derive(Debug)]
pub struct AVLTree<K, V> {
    root: NodePtr,
    store: Vec<Node<K, V>>,
}

impl<K: Ord + Copy + Debug + Display, V: Debug + Copy + Display> AVLTree<K, V> {
    pub fn get_node(&self, np: NodePtr) -> Node<K, V> {
        assert!(np.is_some());
        self.store[np.unwrap()]
    }

    pub fn get_balance(&self, np: NodePtr) -> i8 {
        assert!(np.is_some());
        self.store[np.unwrap()].balance
    }

    pub fn get_key(&self, np: NodePtr) -> K {
        assert!(np.is_some());
        self.store[np.unwrap()].key
    }

    pub fn get_value(&self, np: NodePtr) -> V {
        assert!(np.is_some());
        self.store[np.unwrap()].value
    }

    pub fn get_pointer(&self, np: NodePtr, side: Side) -> NodePtr {
        assert!(np.is_some());
        self.store[np.unwrap()].get_ptr(side)
    }

    pub fn set_balance(&mut self, np: NodePtr, bal: i8) {
        assert!(np.is_some());
        self.store[np.unwrap()].balance = bal;
    }

    pub fn set_key(&mut self, np: NodePtr, to: K) {
        assert!(np.is_some());
        self.store[np.unwrap()].key = to;
    }

    pub fn set_value(&mut self, np: NodePtr, to: V) {
        assert!(np.is_some());
        self.store[np.unwrap()].value = to;
    }

    pub fn set_pointer(&mut self, np: NodePtr, side: Side, to: NodePtr) {
        assert!(np.is_some());
        self.store[np.unwrap()].set_ptr(side, to);
    }

    pub fn increment_balance(&mut self, np: NodePtr, delta: i8) -> i8 {
        assert!(np.is_some());
        self.store[np.unwrap()].balance += delta;
        self.store[np.unwrap()].balance
    }

    pub fn new() -> Self {
        AVLTree {
            root: None,
            store: Vec::<Node<K, V>>::with_capacity(20000),
        }
    }

    /// Insert key-value
    pub fn insert(&mut self, k: K, v: V) -> Option<Node<K, V>> {
        let (n, _) = self.insert_node(Node::new(k, v));
        n
    }

    /// Insert Node struct
    pub fn insert_node(&mut self, mut n: Node<K, V>) -> (Option<Node<K, V>>, Side) {
        if self.root.is_none() {
            assert!(self.store.len() == 0);
            self.store.push(n);
            self.root = Some(0);
            return (Some(n), Side::Root);
        }

        let mut p = self.root;      // Possibly None
        let mut prev = p;
        let mut side = Side::Left;
        while p.is_some() {
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
                    println!("Key exists");
                    return (None, side);
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
        (Some(n), side)
    }

    /// Insert key-value and rebalance
    pub fn insert_bal(&mut self, k: K, v: V) -> Option<Node<K, V>> {
        self.insert_node_bal(Node::new(k, v))
    }

    /// Insert Node struct and rebalance
    pub fn insert_node_bal(&mut self, n: Node<K, V>) -> Option<Node<K, V>> {
        let (nins, side) = self.insert_node(n);
        if nins.is_none() || side == Side::Root {
            return nins;
        }

        let mut p = nins.unwrap().up;
        let mut is_left = false;
        if side == Side::Left {
            is_left = true;
        }

        while p.is_some() {

            let i_c = get_insertion_constants(is_left);

            let b = self.increment_balance(p, i_c.bal_incr);
            if b == 0 {
                break;  // No further adjustments necessary
            } else if b.abs() > 1 {
                let child_p = self.get_pointer(p, i_c.this_side);
                match self.get_balance(child_p) * b {
                    2 => {
                        // -2/-1 & +2/+1 patterns
                        self.single_rotation(i_c.this_side, p, child_p);
                        self.set_balance(p, 0);
                        self.set_balance(child_p, 0);
                        break;
                    }
                    -2 => {
                        // -2/+1 & +2/-1 patterns
                        let grand_p = self.get_pointer(child_p, i_c.that_side);
                        self.double_rotation(i_c.this_side, p, child_p, grand_p);
                        if self.get_pointer(child_p, i_c.this_side).is_none() {
                            // Degenerate case, no subtrees
                            self.set_balance(child_p, 0);
                            self.set_balance(p, 0);
                        } else if n.key.cmp(&self.get_key(grand_p)) == i_c.key_order {
                            self.set_balance(child_p, i_c.bal_incr);
                            self.set_balance(p, 0);
                        } else {
                            self.set_balance(child_p, 0);
                            self.set_balance(p, -1 * i_c.bal_incr);
                        }
                        self.set_balance(grand_p, 0);
                        break;
                    }
                    _ => unreachable!(),
                }
            }

            let child_p = p;
            p = self.get_pointer(p, Side::Up);
            if p.is_some() {
                let left_p = self.get_pointer(p, Side::Left);
                is_left = left_p.is_some() && left_p == child_p;
            }
        }

        nins
    }

    /// Remove the node at index from the store and patch the hole in the vector,
    /// modifying pointers in the moved node's parents and children.
    fn remove_carefully(&mut self, p: NodePtr) {
        assert!(p.is_some());
        let index = p.unwrap();
        let old_index = self.store.len() - 1;
        self.store.swap_remove(index);

        if index == old_index {
            // Nothing moved
            return;
        }

        // Element -1 has moved into the spot _index_. The in-pointers that need modifying
        // belong to that element's parent and children.

        // Fix child pointer in parent:
        let parent_p = self.get_pointer(p, Side::Up);
        if parent_p.is_some() {
            let l = self.get_pointer(parent_p, Side::Left);
            if l == Some(old_index) {
                self.set_pointer(parent_p, Side::Left, Some(index));
            } else {
                self.set_pointer(parent_p, Side::Right, Some(index));
            }
        }

        // Fix parent pointers in children:
        let l = self.get_pointer(p, Side::Left);
        let r = self.get_pointer(p, Side::Right);
        if l.is_some() {
            self.set_pointer(l, Side::Up, Some(index));
        }
        if r.is_some() {
            self.set_pointer(r, Side::Up, Some(index));
        }

        // Fix root if necessary
        if self.root == Some(old_index) {
            self.root = Some(index);
        }
    }

    /// Uses delete-by-copy procedure if node with key k has two children.
    /// Returns (parent, side) tuple.
    pub fn delete(&mut self, k: K) -> (NodePtr, Side) {
        let mut p = self.root;
        let mut prev = None;
        let mut res = None;
        let mut side = Side::Root;
        while p.is_some() {
            match k.cmp(&self.get_key(p)) {
                Ordering::Equal => {
                    break;
                }
                Ordering::Less => {
                    prev = p;
                    side = Side::Left;
                    p = self.get_pointer(p, side);
                }
                Ordering::Greater => {
                    prev = p;
                    side = Side::Right;
                    p = self.get_pointer(p, side);
                }
            }
        }

        if p.is_none() {
            println!("Key {:?} not found", k);
            return (res, side);
        }

        let n = self.get_node(p);
        // Is this a leaf?
        if n.is_leaf() {
            if n.key.cmp(&self.get_key(self.root)) == Ordering::Equal {
                self.root = None;
                assert_eq!(self.store.len(), 1);
            } else {
                self.set_pointer(prev, side, None);
            }
            self.remove_carefully(p);
            // The prev pointer is now stale
            if prev.is_some() && prev.unwrap() == self.store.len() {
                res = p;
            } else {
                res = prev;
            }

            // Is this a one-child node?
        } else if n.left.is_none() || n.right.is_none() {
            let ch;
            if n.left.is_some() {
                ch = n.left;
            } else {
                ch = n.right;
            }
            if n.key.cmp(&self.get_key(self.root)) == Ordering::Equal {
                self.set_pointer(ch, Side::Up, None);
                self.root = ch;
            } else {
                self.set_pointer(prev, side, ch);
                self.set_pointer(ch, Side::Up, prev);
            }
            self.remove_carefully(p);
            // The prev pointer is now stale
            if prev.is_some() && prev.unwrap() == self.store.len() {
                res = p;
            } else {
                res = prev;
            }

            // Complicated case:  two children, do delete-by-copy. Replace n with its first
            // predecessor (the mirror image using the first successor would work as well).
        } else {
            let mut tmp = n.left;
            let mut last = tmp;
            prev = self.get_pointer(tmp, Side::Up);
            while tmp.is_some() && self.get_pointer(last, Side::Right).is_some() {
                prev = self.get_pointer(tmp, Side::Up);
                last = tmp;
                tmp = self.get_pointer(tmp, Side::Right);
            }
            tmp = last;
            // Copy ...
            let the_key = self.get_key(tmp);
            let the_value = self.get_value(tmp);
            self.set_key(p, the_key);
            self.set_value(p, the_value);

            let left_ptr = self.get_pointer(tmp, Side::Left);
            if prev == p {
                self.set_pointer(p, Side::Left, left_ptr);
                if left_ptr.is_some() {
                    self.set_pointer(left_ptr, Side::Up, p);
                }
                side = Side::Left;
            } else {
                self.set_pointer(prev, Side::Right, left_ptr);
                if left_ptr.is_some() {
                    self.set_pointer(left_ptr, Side::Up, prev);
                }
                side = Side::Right;
            }

            self.remove_carefully(tmp);
            // The prev pointer is now stale
            if prev.unwrap() == self.store.len() {
                res = tmp;
            } else {
                res = prev;
            }
        }

        (res, side)
    }

    /// Rebalance on delete
    pub fn delete_bal(&mut self, k: K) -> Option<Node<K, V>> {
        // slug: (pointer to parent of deleted node, side of deleted node)
        let slug = self.delete(k);
        let (pdel, side) = slug;
        if pdel.is_none() {
            return None;
        };
        let ndel = self.get_node(pdel);

        let mut p = pdel;
        let mut is_left = false;
        if side == Side::Left {
            is_left = true;
        }

        // Rebalance and update balance factors. There are two different rotation sequences that
        // are the same within handedness,
        // and the +1/-1 / -1/+1 sequence has three possible balance adjustments
        // depending on the grandchild.
        while p.is_some() {
            let d_c = get_deletion_constants(is_left);

            let b = self.increment_balance(p, d_c.bal_incr);
            if b.abs() == 1 {
                break; // No further adjustments necessary
            } else if b.abs() > 1 {
                let child_p = self.get_pointer(p, d_c.this_side);
                match self.get_balance(child_p) * b {
                    2 => {
                        // +1/+1 & -1/-1 patterns
                        self.single_rotation(d_c.this_side, p, child_p);
                        self.set_balance(p, 0);
                        p = self.get_pointer(p, Side::Up);
                        self.set_balance(p, 0);
                    }
                    0 => {
                        // +1/0 & -1/0 patterns
                        self.single_rotation(d_c.this_side, p, child_p);
                        self.set_balance(p, d_c.bal_incr);
                        p = self.get_pointer(p, Side::Up);
                        self.set_balance(p, -1 * d_c.bal_incr);
                        break;  // No height change
                    }
                    -2 => {
                        // +1/-1/x & -1/+1/x patterns
                        let grand_p = self.get_pointer(child_p, d_c.that_side);
                        self.double_rotation(d_c.this_side, p, child_p, grand_p);
                        // p is now one child, grand_p is the other, child_p is their parent
                        match self.get_balance(grand_p) {
                            -1 => {
                                self.set_balance(p, d_c.gcm1_parent_adj);
                                self.set_balance(child_p, d_c.gcm1_child_adj);
                            }
                            0 => {
                                self.set_balance(p, 0);
                                self.set_balance(child_p, 0);
                            }
                            1 => {
                                self.set_balance(p, d_c.gcp1_parent_adj);
                                self.set_balance(child_p, d_c.gcp1_child_adj);
                            }
                            _ => unreachable!(),
                        }
                        self.set_balance(grand_p, 0);
                        p = self.get_pointer(p, Side::Up);
                    }
                    _ => unreachable!(),
                }
            }

            let child_p = p;
            p = self.get_pointer(p, Side::Up);
            if p.is_some() {
                let left_p = self.get_pointer(p, Side::Left);
                is_left = left_p.is_some() && left_p == child_p;
            }
        }

        Some(ndel)
    }

    /// Returns node value
    pub fn lookup(&self, k: K) -> Option<V> {
        if let Some(n) = self.search(k) {
            Some(n.value)
        } else {
            None
        }
    }

    /// Returns node (not pointer)
    pub fn search(&self, k: K) -> Option<Node<K, V>> {
        let mut p = self.root;
        let mut res = None;

        while p.is_some() {
            match k.cmp(&self.get_key(p)) {
                Ordering::Less => {
                    p = self.get_pointer(p, Side::Left);
                }
                Ordering::Greater => {
                    p = self.get_pointer(p, Side::Right);
                }
                Ordering::Equal => {
                    res = Some(self.get_node(p));
                    break;
                }
            }
        }
        res
    }

    /// Do an in-order traversal, where a "visit" prints the row with that node in it.
    fn display(&self, p: NodePtr, side: Side, e: &Vec<DisplayElement>, f: &mut Formatter) {
        if p.is_none() {
            return;
        }

        let mut elems = e.clone();
        let node = self.get_node(p);
        let mut tail = DisplayElement::SpaceSpace;
        if node.up != self.root {
            // Direction switching, need trunk element to be printed for lines before that node
            // is visited.
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
        } else
        // if side == Side::Left
        {
            elems[hindex] = DisplayElement::SpaceLeft;
            let parent_p = self.get_pointer(p, Side::Up);
            let gp_p = self.get_pointer(parent_p, Side::Up);
            if gp_p.is_some() && self.get_pointer(gp_p, Side::Right) == parent_p {
                // Direction switched, need trunk element starting with this node/line
                elems[hindex - 1] = DisplayElement::TrunkSpace;
            }
        }

        // Visit node => print accumulated elements. Each node gets a line.
        {
            for e in elems.clone() {
                let _ = write!(f, "{}", e.string());
            }
            let _ = write!(f,
                           "{key:>width$} ",
                           key = Green.bold().paint(node.key),
                           width = 2);
            let _ = write!(f,
                           "{value:>width$} ",
                           value = Blue.bold().paint(format!("{:.*}", 2, node.value)),
                           width = 4);
            let _ = write!(f,
                           "{bal:<-width$}\n",
                           bal = Red.bold().paint(node.balance),
                           width = 2);

            elems[hindex] = tail;
        }

        self.display(node.left, Side::Left, &elems, f);
    }

    pub fn gather_balances(&self) -> (Vec<K>, Vec<i8>) {
        let mut keys = Vec::<K>::new();
        let mut bals = Vec::<i8>::new();

        self.gather_balances_impl(self.root, &mut keys, &mut bals);
        (keys, bals)
    }

    fn gather_balances_impl(&self, p: NodePtr, k: &mut Vec<K>, b: &mut Vec<i8>) {
        if p.is_none() {
            return;
        }
        let r = self.get_pointer(p, Side::Right);
        self.gather_balances_impl(r, k, b);
        k.push(self.get_key(p));
        b.push(self.get_balance(p));
        let l = self.get_pointer(p, Side::Left);
        self.gather_balances_impl(l, k, b)
    }

    pub fn compute_balances(&mut self, p: NodePtr) -> i8 {
        self.compute_balances_impl(p, 0)
    }

    fn compute_balances_impl(&mut self, p: NodePtr, level: i8) -> i8 {
        if p.is_none() {
            return level - 1;
        }
        let r = self.get_pointer(p, Side::Right);
        let l = self.get_pointer(p, Side::Left);
        let rb = self.compute_balances_impl(r, level + 1);
        let lb = self.compute_balances_impl(l, level + 1);
        self.set_balance(p, rb - lb);
        std::cmp::max(rb, lb)
    }

    ///     P                Q
    ///   /   \     =>     /   \
    ///  h     Q          P     h'
    fn rotate_left(&mut self, p: NodePtr, q: NodePtr) {
        assert!(p.is_some());
        assert!(q.is_some());
        let p_parent = self.get_pointer(p, Side::Up);
        // Take care of parent pointers
        self.set_pointer(q, Side::Up, p_parent);
        self.set_pointer(p, Side::Up, q);
        let ql = self.get_pointer(q, Side::Left);
        if ql.is_some() {
            self.set_pointer(ql, Side::Up, p);
        }

        // Take care of child pointers
        self.set_pointer(q, Side::Left, p);
        self.set_pointer(p, Side::Right, ql);
        if p_parent.is_some() {
            if self.get_pointer(p_parent, Side::Right) == p {
                self.set_pointer(p_parent, Side::Right, q);
            } else {
                self.set_pointer(p_parent, Side::Left, q);
            }
        } else {
            self.root = q;
        }
    }

    ///     P                Q
    ///   /   \     =>     /   \
    ///  Q     h          h'    P
    fn rotate_right(&mut self, p: NodePtr, q: NodePtr) {
        assert!(p.is_some());
        assert!(q.is_some());
        let p_parent = self.get_pointer(p, Side::Up);
        // Take care of parent pointers
        self.set_pointer(q, Side::Up, p_parent);
        self.set_pointer(p, Side::Up, q);
        let qr = self.get_pointer(q, Side::Right);
        if qr.is_some() {
            self.set_pointer(qr, Side::Up, p);
        }

        // Take care of child pointers
        self.set_pointer(q, Side::Right, p);
        self.set_pointer(p, Side::Left, qr);
        if p_parent.is_some() {
            if self.get_pointer(p_parent, Side::Right) == p {
                self.set_pointer(p_parent, Side::Right, q);
            } else {
                self.set_pointer(p_parent, Side::Left, q);
            }
        } else {
            self.root = q;
        }
    }

    fn single_rotation(&mut self, side: Side, p: NodePtr, q: NodePtr) {
        if side == Side::Left {
            self.rotate_right(p, q);
        } else {
            self.rotate_left(p, q);
        }
    }

    fn double_rotation(&mut self, side: Side, p: NodePtr, child_p: NodePtr, grand_p: NodePtr) {
        if side == Side::Left {
            self.rotate_left(child_p, grand_p);
            self.rotate_right(p, grand_p);
        } else {
            self.rotate_right(child_p, grand_p);
            self.rotate_left(p, grand_p);
        }
    }
}

impl<K: Ord + Copy, V: Copy> Node<K, V> {
    pub fn new(k: K, v: V) -> Node<K, V> {
        Node {
            key: k,
            value: v,
            balance: 0,
            left: None,
            right: None,
            up: None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
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

impl<K: Ord + Copy + Debug + Display, V: Debug + Copy + Display> Display for AVLTree<K, V> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.root.is_none() {
            write!(f, "[empty]")
        } else {
            let mut v: Vec<DisplayElement> = Vec::new();
            self.display(self.root, Side::Up, &mut v, f);
            Ok(())
        }
    }
}

fn get_insertion_constants(is_left: bool) -> &'static BalanceConstants {
    if is_left {
        &BALANCE_CONSTANTS_A
    } else {
        &BALANCE_CONSTANTS_B
    }
}

fn get_deletion_constants(is_left: bool) -> &'static BalanceConstants {
    get_insertion_constants(!is_left)
}

pub fn random_bal_tree(n: u32) -> AVLTree<i32, f32> {
    let mut tree: AVLTree<i32, f32> = AVLTree::new();
    let mut rng = rand::thread_rng();
    tree.insert_bal(0, rng.gen_range(-1f32, 1f32));
    for _ in 0..n {
        tree.insert_bal(rng.gen_range(-(n as i32) / 2, (n as i32) / 2),
                        rng.gen_range(-1f32, 1f32));
    }
    tree
}

#[cfg(test)]
mod tests {
    use rand::{thread_rng, seq};

    use super::AVLTree;
    use random_bal_tree;

    #[test]
    fn test_insert() {
        let mut tree: AVLTree<i32, f32> = AVLTree::new();
        tree.insert(0, 0.0);
        tree.insert(8, 8.8);
        tree.insert(-8, -8.8);
        assert!(tree.insert(4, 4.4).is_some());
        tree.insert(12, 12.12);

        assert_eq!(tree.lookup(4), Some(4.4));
        assert_eq!(tree.lookup(5), None);
        assert_eq!(tree.lookup(-8), Some(-8.8));

        let s = &tree.store;
        assert_eq!(s[s[s[tree.root.unwrap()].right.unwrap()].right.unwrap()].value,
                   12.12);
        assert_eq!(s[s[s[tree.root.unwrap()].right.unwrap()].right.unwrap()].left,
                   None);
    }

    #[test]
    fn test_delete() {
        let mut tree: AVLTree<i32, f32> = AVLTree::new();
        tree.insert(0, 0.0);
        tree.insert(8, 8.8);
        tree.insert(-8, -8.8);
        assert!(tree.insert(4, 4.4).is_some());
        tree.insert(12, 12.12);

        // delete leaf
        tree.delete(12);
        assert_eq!(tree.lookup(12), None);
        let mut n = tree.search(8).unwrap();
        assert_eq!(n.right, None);

        // delete one-child node
        tree.delete(4);
        assert_eq!(tree.lookup(4), None);
        n = tree.search(0).unwrap();
        assert_eq!(tree.store[n.right.unwrap()].key, 8);

        // delete two-child node
        tree.insert(6, 6.6);
        tree.insert(10, 10.10);
        tree.insert(7, 7.7);
        tree.delete(8);
        n = tree.search(7).unwrap();
        assert_eq!(tree.store[n.left.unwrap()].key, 6);
        assert_eq!(tree.store[n.right.unwrap()].key, 10);
        assert_eq!(tree.store[n.up.unwrap()].key, 0);

        // delete two-child root
        tree.delete(0);
        assert_eq!(tree.store[tree.root.unwrap()].key, -8);

        // delete one-child root
        tree.delete(-8);
        assert_eq!(tree.store[tree.root.unwrap()].key, 7);

        // delete no-child root
        tree.delete(6);
        tree.delete(7);
        tree.delete(10);
        assert!(tree.root.is_none());
        assert_eq!(tree.store.len(), 0);
    }

    #[test]
    fn test_rotate_left() {
        let mut tree: AVLTree<i32, f32> = AVLTree::new();
        tree.insert(0, 0.0);
        tree.insert(8, 8.8);
        tree.insert(4, 4.4);
        tree.insert(-8, -8.8);

        let mut r = tree.root;
        let mut right = tree.store[r.unwrap()].right;
        tree.rotate_left(r, right);
        r = tree.root;
        right = tree.store[r.unwrap()].right;
        let left = tree.store[r.unwrap()].left;
        let left_left = tree.store[left.unwrap()].left;
        let left_right = tree.store[left.unwrap()].right;
        assert_eq!(right, None);
        assert_eq!(tree.store[left.unwrap()].key, 0);
        assert_eq!(tree.store[left_left.unwrap()].key, -8);
        assert_eq!(tree.store[left_right.unwrap()].key, 4);
        assert_eq!(tree.store[r.unwrap()].key, 8);
    }

    #[test]
    fn test_rotate_right() {
        let mut tree: AVLTree<i32, f32> = AVLTree::new();
        tree.insert(0, 0.0);
        tree.insert(8, 8.8);
        tree.insert(-8, -8.8);
        tree.insert(-4, 4.4);

        let mut r = tree.root;
        let mut left = tree.store[r.unwrap()].left;
        tree.rotate_right(r, left);
        r = tree.root;
        left = tree.store[r.unwrap()].left;
        let right = tree.store[r.unwrap()].right;
        let right_right = tree.store[right.unwrap()].right;
        let right_left = tree.store[right.unwrap()].left;
        assert_eq!(left, None);
        assert_eq!(tree.store[right.unwrap()].key, 0);
        assert_eq!(tree.store[right_right.unwrap()].key, 8);
        assert_eq!(tree.store[right_left.unwrap()].key, -4);
        assert_eq!(tree.store[r.unwrap()].key, -8);
    }

    #[test]
    // This tree tests all four insertion types
    fn test_balanced_inserts() {
        let mut tree: AVLTree<i32, f32> = AVLTree::new();
        tree.insert_bal(0, 0.0);
        tree.insert_bal(8, 8.8);
        tree.insert_bal(-8, -8.8);
        tree.insert_bal(12, 12.12);
        tree.insert_bal(16, 16.16);
        tree.insert_bal(11, 11.11);
        tree.insert_bal(4, 4.4);
        tree.insert_bal(-10, -8.8);
        tree.insert_bal(-12, -8.8);
        tree.insert_bal(-9, -8.8);

        let mut res = tree.gather_balances();
        let (_, bals) = res;
        assert!(bals.iter().max().unwrap() < &2);
        assert!(bals.iter().min().unwrap() > &-2);

        for _ in 0..10 {
            tree = random_bal_tree(1000);
            res = tree.gather_balances();
            let (_, bals) = res;
            assert!(bals.iter().max().unwrap() < &2);
            assert!(bals.iter().min().unwrap() > &-2);
        }
    }

    #[test]
    /// This sequence hits all five rotation possibilities on each side.
    fn test_balanced_deletes() {
        let mut tree: AVLTree<i32, f32> = AVLTree::new();
        tree.insert_bal(0, 0.0);
        tree.insert_bal(-32, 0.0);
        tree.insert_bal(32, 0.0);
        tree.insert_bal(-64, 0.0);
        tree.insert_bal(64, 0.0);
        tree.delete_bal(64);
        tree.delete_bal(32);
        tree.delete_bal(-32);
        tree.delete_bal(-64);
        tree.delete_bal(0);
        assert_eq!(tree.root, None);
        assert_eq!(tree.store.len(), 0);

        tree.insert_bal(0, 0.0);
        tree.insert_bal(-32, 0.0);
        tree.insert_bal(32, 0.0);
        tree.insert_bal(-64, 0.0);
        tree.insert_bal(64, 0.0);
        tree.insert_bal(-16, 0.0);
        tree.insert_bal(16, 0.0);
        tree.insert_bal(-8, 0.0);
        tree.insert_bal(8, 0.0);
        tree.insert_bal(-12, 0.0);
        tree.insert_bal(-7, 0.0);
        tree.insert_bal(-6, 0.0);
        tree.insert_bal(-11, 0.0);

        tree.delete_bal(-64);
        tree.delete_bal(-32);
        tree.delete_bal(-7);
        tree.delete_bal(-6);
        tree.delete_bal(-16);
        tree.delete_bal(-11);
        tree.delete_bal(-12);
        tree.delete_bal(8);
        tree.delete_bal(-8);
        tree.delete_bal(0);
        tree.insert_bal(24, 0.0);
        tree.insert_bal(8, 0.0);
        tree.insert_bal(4, 0.0);
        tree.insert_bal(128, 0.0);
        tree.insert_bal(48, 0.0);
        tree.delete_bal(32);
        tree.delete_bal(48);

        tree.insert_bal(-24, 0.0);
        tree.insert_bal(-8, 0.0);
        tree.insert_bal(-128, 0.0);
        tree.insert_bal(-48, 0.0);
        tree.insert_bal(-20, 0.0);
        tree.insert_bal(-30, 0.0);
        tree.insert_bal(-22, 0.0);
        tree.insert_bal(-21, 0.0);
        tree.delete_bal(24);
        tree.delete_bal(64);
        tree.delete_bal(-30);
        tree.delete_bal(-22);
        tree.delete_bal(-21);
        tree.delete_bal(-128);
        tree.delete_bal(128);
        tree.delete_bal(-8);
        tree.insert_bal(-96, 0.0);
        tree.insert_bal(-95, 0.0);
        tree.insert_bal(-10, 0.0);
        tree.insert_bal(6, 0.0);
        tree.delete_bal(-24);

        let mut res = tree.gather_balances();
        let (_, bals) = res;
        assert!(bals.iter().max().unwrap() < &2);
        assert!(bals.iter().min().unwrap() > &-2);

        let mut p = tree.root;
        while p.is_some() {
            let key = tree.store[p.unwrap()].key;
            tree.delete_bal(key);
            p = tree.root;
        }
        assert_eq!(tree.root, None);
        assert_eq!(tree.store.len(), 0);

        // */*/+1 patterns
        tree.insert(6, 0.0);
        tree.insert(-1, 0.0);
        tree.insert(9, 0.0);
        tree.insert(7, 0.0);
        tree.insert(3, 0.0);
        tree.insert(-9, 0.0);
        tree.insert(4, 0.0);
        p = tree.root;
        tree.compute_balances(p);
        tree.delete_bal(-9);
        res = tree.gather_balances();
        let (_, bals) = res;
        tree.compute_balances(p);
        res = tree.gather_balances();
        let (_, bals_after) = res;
        assert_eq!(bals, bals_after);

        tree.insert(6, 0.0);
        tree.insert(-1, 0.0);
        tree.insert(3, 0.0);
        tree.insert(9, 0.0);
        tree.insert(7, 0.0);
        tree.insert(11, 0.0);
        tree.insert(8, 0.0);
        p = tree.root;
        tree.compute_balances(p);
        tree.delete_bal(-1);
        res = tree.gather_balances();
        let (_, bals) = res;
        tree.compute_balances(p);
        res = tree.gather_balances();
        let (_, bals_after) = res;
        assert_eq!(bals, bals_after);

        let mut rng = thread_rng();
        for _ in 0..100 {
            tree = random_bal_tree(100);
            let sample = seq::sample_iter(&mut rng, -50..50, 80).unwrap();
            for i in sample {
                tree.delete_bal(i);
            }
        }

        res = tree.gather_balances();
        let (_, bals) = res;

        if bals.len() > 0 {
            assert!(bals.iter().max().unwrap() < &2);
            assert!(bals.iter().min().unwrap() > &-2);
        }

        return;
    }
}
