struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    fn my_map<U, F>(&self, f: &F) -> TreeNode<U>
        where F: Fn(&T) -> U
    {
        TreeNode {
            value: f(&self.value),
            left: match self.left {
                None => None,
                Some(ref n) => Some(Box::new(n.my_map(f))),
            },
            right: match self.right {
                None => None,
                Some(ref n) => Some(Box::new(n.my_map(f))),
            },
        }
    }
}

fn main() {
    let root = TreeNode {
        value: 3,
        left: Some(Box::new(TreeNode {
            value: 55,
            left: None,
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            value: 234,
            left: Some(Box::new(TreeNode {
                value: 0,
                left: None,
                right: None,
            })),
            right: None,
        })),
    };
    root.my_map(&|x| println!("{}", x));
    println!("---------------");
    let new_root = root.my_map(&|x| *x as f64 * 333.333f64);
    new_root.my_map(&|x| println!("{}", x));
}
