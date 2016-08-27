type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
}

/// Iteration by value (simply empties the list as the caller now owns all values)
pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.head.take().map(|node| {
            let node = *node;
            self.0.head = node.next;
            node.elem
        })
    }
}

/// Iteration by immutable reference
pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

/// Iteration by mutable reference
pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}

/// Methods implemented for List<T>
impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

fn main() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    for item in list.iter() {
        println!("{}", item);
    }
}
