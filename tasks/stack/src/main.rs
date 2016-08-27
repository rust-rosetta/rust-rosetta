#[derive(Debug)]
struct Stack<T> {
    /// We use a vector because of simplicity
    vec: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack { vec: Vec::new() }
    }

    /// Adds an element at the top of the stack
    fn push(&mut self, elem: T) {
        self.vec.push(elem);
    }

    /// Removes and returns the element at the top of the stack
    fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }

    /// Returns a reference of the element at the top of the stack
    fn peek(&self) -> Option<&T> {
        self.vec.last()
    }

    /// Returns true if the stack is empty
    fn empty(&self) -> bool {
        self.vec.is_empty()
    }
}

fn main() {
    let mut stack = Stack::new();

    // Fill the stack
    stack.push(5i32);
    stack.push(8);
    stack.push(9);

    // Show the element at the top
    println!("{}", stack.peek().unwrap());
    // Show the element we popped
    println!("{}", stack.pop().unwrap());
    if stack.empty() {
        println!("The stack is empty.")
    } else {
        println!("The stack is not empty.")
    }
}

#[test]
fn test_basic() {
    let mut stack = Stack::new();

    // The stack is empty
    assert!(stack.empty());

    // Fill the stack
    stack.push(5i32);
    stack.push(8);
    stack.push(9);

    // The stack is not empty
    assert!(!stack.empty());

    // The element at the top is 9
    assert!(stack.peek().unwrap() == &9);

    // Remove one element
    stack.pop();

    // The element at the top is now 8
    assert!(stack.peek().unwrap() == &8);
}
