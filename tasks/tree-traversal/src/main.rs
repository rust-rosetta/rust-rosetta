#![feature(box_syntax, box_patterns)]

use std::collections::VecDeque;

#[derive(Debug)]
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[cfg_attr(feature = "clippy", allow(enum_variant_names))]
enum TraversalMethod {
    PreOrder,
    InOrder,
    PostOrder,
    LevelOrder,
}

impl<T> TreeNode<T> {
    pub fn new(arr: &[[i8; 3]]) -> TreeNode<i8> {

        let l = match arr[0][1] {
            -1 => None,
            i => Some(Box::new(TreeNode::<i8>::new(&arr[(i - arr[0][0]) as usize..]))),
        };
        let r = match arr[0][2] {
            -1 => None,
            i => Some(Box::new(TreeNode::<i8>::new(&arr[(i - arr[0][0]) as usize..]))),
        };

        TreeNode {
            value: arr[0][0],
            left: l,
            right: r,
        }
    }

    pub fn traverse(&self, tr: &TraversalMethod) -> Vec<&TreeNode<T>> {
        match *tr {
            TraversalMethod::PreOrder => self.iterative_preorder(),
            TraversalMethod::InOrder => self.iterative_inorder(),
            TraversalMethod::PostOrder => self.iterative_postorder(),
            TraversalMethod::LevelOrder => self.iterative_levelorder(),
        }
    }

    fn iterative_preorder(&self) -> Vec<&TreeNode<T>> {
        let mut stack: Vec<&TreeNode<T>> = Vec::new();
        let mut res: Vec<&TreeNode<T>> = Vec::new();

        stack.push(self);
        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            res.push(node);
            match node.right {
                None => {}
                Some(box ref n) => stack.push(n),
            }
            match node.left {
                None => {}
                Some(box ref n) => stack.push(n),
            }
        }
        res
    }

    // Leftmost to rightmost
    fn iterative_inorder(&self) -> Vec<&TreeNode<T>> {
        let mut stack: Vec<&TreeNode<T>> = Vec::new();
        let mut res: Vec<&TreeNode<T>> = Vec::new();
        let mut p = self;

        loop {
            // Stack parents and right children while left-descending
            loop {
                match p.right {
                    None => {}
                    Some(box ref n) => stack.push(n),
                }
                stack.push(p);
                match p.left {
                    None => break,
                    Some(box ref n) => p = n,
                }
            }
            // Visit the nodes with no right child
            p = stack.pop().unwrap();
            while !stack.is_empty() && p.right.is_none() {
                res.push(p);
                p = stack.pop().unwrap();
            }
            // First node that can potentially have a right child:
            res.push(p);
            if stack.is_empty() {
                break;
            } else {
                p = stack.pop().unwrap();
            }
        }
        res
    }

    // Left-to-right postorder is same sequence as right-to-left preorder, reversed
    fn iterative_postorder(&self) -> Vec<&TreeNode<T>> {
        let mut stack: Vec<&TreeNode<T>> = Vec::new();
        let mut res: Vec<&TreeNode<T>> = Vec::new();

        stack.push(self);
        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            res.push(node);
            match node.left {
                None => {}
                Some(box ref n) => stack.push(n),
            }
            match node.right {
                None => {}
                Some(box ref n) => stack.push(n),
            }
        }
        let rev_iter = res.iter().rev();
        let mut rev: Vec<&TreeNode<T>> = Vec::new();
        for elem in rev_iter {
            rev.push(elem);
        }
        rev
    }

    fn iterative_levelorder(&self) -> Vec<&TreeNode<T>> {
        let mut queue: VecDeque<&TreeNode<T>> = VecDeque::new();
        let mut res: Vec<&TreeNode<T>> = Vec::new();

        queue.push_back(self);
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            res.push(node);
            match node.left {
                None => {}
                Some(box ref n) => queue.push_back(n),
            }
            match node.right {
                None => {}
                Some(box ref n) => queue.push_back(n),
            }
        }
        res
    }
}

fn main() {
    // Array representation of task tree
    let arr_tree = [[1, 2, 3],
                    [2, 4, 5],
                    [3, 6, -1],
                    [4, 7, -1],
                    [5, -1, -1],
                    [6, 8, 9],
                    [7, -1, -1],
                    [8, -1, -1],
                    [9, -1, -1]];

    let root = TreeNode::<i8>::new(&arr_tree);

    let method_labels = [(TraversalMethod::PreOrder, "pre-order:"),
                         (TraversalMethod::InOrder, "in-order:"),
                         (TraversalMethod::PostOrder, "post-order:"),
                         (TraversalMethod::LevelOrder, "level-order:")];

    for method_label in &method_labels {
        print!("{}\t", method_label.1);
        for n in root.traverse(&method_label.0) {
            print!(" {}", n.value);
        }
        print!("\n");
    }
}

#[cfg(test)]
mod tests {
    use super::{TreeNode, TraversalMethod};

    macro_rules! trav_values {
        ($x:expr) => ($x.iter().map(|n| n.value).collect(););
    }

    #[test]
    fn test_traversals() {
        let arr_tree = [[1, 2, 3],
                        [2, 4, 5],
                        [3, 6, -1],
                        [4, 7, -1],
                        [5, -1, -1],
                        [6, 8, 9],
                        [7, -1, -1],
                        [8, -1, -1],
                        [9, -1, -1]];
        let root = TreeNode::<i8>::new(&arr_tree);

        let mut res: Vec<i8>;
        res = trav_values!(root.traverse(&TraversalMethod::PreOrder));
        assert_eq!(res.as_slice(), &[1, 2, 4, 7, 5, 3, 6, 8, 9]);
        res = trav_values!(root.traverse(&TraversalMethod::InOrder));
        assert_eq!(res.as_slice(), &[7, 4, 2, 5, 1, 8, 6, 9, 3]);
        res = trav_values!(root.traverse(&TraversalMethod::PostOrder));
        assert_eq!(res.as_slice(), &[7, 4, 5, 2, 8, 9, 6, 3, 1]);
        res = trav_values!(root.traverse(&TraversalMethod::LevelOrder));
        assert_eq!(res.as_slice(), &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
