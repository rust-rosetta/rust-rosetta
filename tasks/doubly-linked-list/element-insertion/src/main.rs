//! This expands upon the implementation defined on [Rosetta Code][element definition] and consists
//! of the relevant lines from the `LinkedList` implementation in the Rust standard library.
//!
//! [element definition]: http://rosettacode.org/wiki/Doubly-linked_list/Element_definition

#![allow(dead_code)]

use std::mem;
use std::ptr;

pub struct LinkedList<T> {
    length: usize,
    list_head: Link<T>,
    list_tail: Rawlink<Node<T>>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Rawlink<T> {
    p: *mut T,
}

struct Node<T> {
    next: Link<T>,
    prev: Rawlink<Node<T>>,
    value: T,
}

impl<T> Node<T> {
    fn new(v: T) -> Node<T> {
        Node {
            value: v,
            next: None,
            prev: Rawlink::none(),
        }
    }
}

impl<T> Rawlink<T> {
    fn none() -> Self {
        Rawlink { p: ptr::null_mut() }
    }

    fn some(n: &mut T) -> Rawlink<T> {
        Rawlink { p: n }
    }
}

impl<'a, T> From<&'a mut Link<T>> for Rawlink<Node<T>> {
    fn from(node: &'a mut Link<T>) -> Self {
        match node.as_mut() {
            None => Rawlink::none(),
            Some(ptr) => Rawlink::some(ptr),
        }
    }
}

fn link_no_prev<T>(mut next: Box<Node<T>>) -> Link<T> {
    next.prev = Rawlink::none();
    Some(next)
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            length: 0,
            list_head: None,
            list_tail: Rawlink { p: ptr::null_mut() },
        }
    }

    #[inline]
    fn push_front_node(&mut self, mut new_head: Box<Node<T>>) {
        match self.list_head {
            None => {
                self.list_head = link_no_prev(new_head);
                self.list_tail = Rawlink::from(&mut self.list_head);
            }
            Some(ref mut head) => {
                new_head.prev = Rawlink::none();
                head.prev = Rawlink::some(&mut *new_head);
                mem::swap(head, &mut new_head);
                head.next = Some(new_head);
            }
        }
        self.length += 1;
    }
    pub fn push_front(&mut self, elt: T) {
        self.push_front_node(Box::new(Node::new(elt)));
    }
}

fn main() {
    use std::collections;
    let mut list1 = collections::LinkedList::new();
    list1.push_front(8);

    let mut list2 = LinkedList::new();
    list2.push_front(8);
}
