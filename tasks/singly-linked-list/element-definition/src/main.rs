#![allow(dead_code)]

//! Rust's `Option<T>` type makes the definition of a singly-linked list trivial. The use of
//! `Box<T>` (an owned pointer) is necessary because it has a known size, thus making sure the
//! struct that contains it can have a finite size.
//!
//! ```{rust}
//! struct Node<T> {
//!     elem: T,
//!     next: Option<Box<Node<T>>>,
//! }
//! ```
//!
//! However, the above example would not be suitable for a library because, first and foremost, it
//! is private by default but simply making it public would not allow for any encapsulation.

/// Type alias
type Link<T> = Option<Box<Node<T>>>;

/// User-facing interface for list
pub struct List<T> {
    head: Link<T>,
}

/// Private implementation of Node
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    /// List constructor
    #[inline]
    pub fn new() -> Self {
        List { head: None }
    }

    // Add other methods here...
}

fn main() {
    let _ = List::<i32>::new();
}
