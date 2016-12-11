//! Doubly linked lists present a problem in Rust due to its ownership model. There cannot be two
//! mutable references to the same object, so what are we to do? Below are the relevant lines (with
//! added comments) from the `std` implementation ([Documentation][doc] [Source][src]).
//!
//! In order to circumvent the multiple mutable references, raw C-like pointers are used. Note that
//! these cannot be dereferenced with guaranteed safety and thus dereferencing is relegated to
//! `unsafe {}` blocks.
//!
//! [doc]: https://doc.rust-lang.org/std/collections/struct.LinkedList.html
//! [src]: https://github.com/rust-lang/rust/blob/master/src/libcollections/linked_list.rs

#![allow(dead_code)]

use std::ptr;

/// User-facing implementation
pub struct LinkedList<T> {
    length: usize,
    list_head: Link<T>,
    list_tail: Rawlink<Node<T>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            length: 0,
            list_head: None,
            list_tail: Rawlink { p: ptr::null_mut() },
        }
    }
}

/// Type definition
type Link<T> = Option<Box<Node<T>>>;

/// Pointer is wrapped in struct so that Option-like methods can be added to it later (wrappers
/// around NULL checks)
struct Rawlink<T> {
    /// Raw mutable pointer
    p: *mut T,
}

struct Node<T> {
    next: Link<T>,
    prev: Rawlink<Node<T>>,
    value: T,
}

fn main() {
    // Note: you can just import the standard definition.
    use std::collections;

    // Doubly linked list containing 32-bit integers
    let list1 = collections::LinkedList::<i32>::new();

    // Doubly linked list containing 32-bit integers
    let list2 = self::LinkedList::<i32>::new();

    drop(list1);
    drop(list2);
}
