//! This shows the implementation of a singly-linked queue with dequeue and enqueue. There are two
//! peek implementations, one returns an immutable reference, the other returns a mutable one. This
//! implementation also shows iteration over the Queue by value (consumes queue), immutable
//! reference, and mutable reference.

use std::ptr;

pub struct Queue<T> {
    head: Link<T>,

    /// Raw, C-like pointer. Cannot be guaranteed safe
    tail: *mut Item<T>,
}

type Link<T> = Option<Box<Item<T>>>;

struct Item<T> {
    elem: T,
    next: Link<T>,
}

pub struct IntoIter<T>(Queue<T>);

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Item<T>>,
}

pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Item<T>>,
}


impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            head: None,
            tail: ptr::null_mut(),
        }
    }

    pub fn enqueue(&mut self, elem: T) {
        let mut new_tail = Box::new(Item {
            elem: elem,
            next: None,
        });

        let raw_tail: *mut _ = &mut *new_tail;

        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }

        self.tail = raw_tail;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;

            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }

            head.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|item| &item.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|item| &mut item.elem)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|item| &**item) }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_mut().map(|item| &mut **item) }
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_item) = cur_link {
            cur_link = boxed_item.next.take();
        }
    }
}

impl<T> IntoIterator for Queue<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.dequeue()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|item| {
            self.next = item.next.as_ref().map(|item| &**item);
            &item.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|item| {
            self.next = item.next.as_mut().map(|item| &mut **item);
            &mut item.elem
        })
    }
}

fn main() {
    // The standard library has a double-ended queue implementation
    // (VecDeque<T>) which will work here.
    use std::collections::VecDeque;

    let mut deque = VecDeque::new();
    deque.push_back("Element1");
    deque.push_back("Element2");
    deque.push_back("Element3");

    assert_eq!(Some(&"Element1"), deque.front());
    assert_eq!(Some("Element1"), deque.pop_front());
    assert_eq!(Some("Element2"), deque.pop_front());
    assert_eq!(Some("Element3"), deque.pop_front());
    assert_eq!(None, deque.pop_front());

    let mut queue = Queue::new();
    queue.enqueue("Element1");
    queue.enqueue("Element2");
    queue.enqueue("Element3");

    assert_eq!(Some(&"Element1"), queue.peek());
    assert_eq!(Some("Element1"), queue.dequeue());
    assert_eq!(Some("Element2"), queue.dequeue());
    assert_eq!(Some("Element3"), queue.dequeue());
    assert_eq!(None, queue.dequeue());
}

#[test]
fn test_queue() {
    let mut queue = Queue::new();
    queue.enqueue("Element1");
    queue.enqueue("Element2");
    queue.enqueue("Element3");

    assert_eq!(Some(&"Element1"), queue.peek());
    assert_eq!(Some("Element1"), queue.dequeue());
    assert_eq!(Some("Element2"), queue.dequeue());
    assert_eq!(Some("Element3"), queue.dequeue());
    assert_eq!(None, queue.dequeue());
}
