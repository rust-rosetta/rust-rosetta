//! Examples of various Rust collections. Contains both original examples and those taken from the
//! standard API documentation.

use std::collections::{BinaryHeap, BTreeMap, HashMap, HashSet, LinkedList, VecDeque};

fn main() {
    // Stack-allocated collections
    // ---------------------------

    // Array
    //
    // Arrays ([T]) are stack allocated, fixed size collections of items of the same type.
    let a = [1u8, 2, 3, 4, 5]; // a is of type [u8; 5];
    let b = [0; 256]; // Equivalent to `let b = [0, 0, 0, 0, 0, 0... repeat 256 times]`
    assert_eq!(a.len(), 5);
    assert_eq!(b.len(), 256);

    // Slice
    //
    // Slices (&[T]) are dynamically sized views into contiguous sequences (arrays, vectors,
    // strings)
    let array = [1, 2, 3, 4, 5];
    let slice = &array[0..2];
    println!("{:?}", slice);    // Output: [1, 2]

    // String slice
    //
    // String slices are (str) are slices of Unicode characters. Plain strs are almost never seen
    // in Rust. Instead either heap-allocated Strings or borrowed string slices (&str which is
    // basically equivalent to a slice of bytes: &[u8]) are more often used. It should be noted
    // that strings are not indexable as they are UTF-8 (meaning that characters are not
    // necessarily of a fixed size) however iterators can be created over codepoints or graphemes.
    let string = "this is a string slice";
    println!("{}", string);

    // Heap-allocated collections
    // --------------------------

    // Vector
    //
    // Vectors (Vec<T>) are a growable list type. According to the Rust documentation, you want to
    // use a Vector if:
    // - You want to collect items up to be processed or sent elsewhere later, and don't care about
    //   any properties of the actual values being stored.
    // - You want a sequence of elements in a particular order, and will only be appending to (or
    //   near) the end.
    // - You want a stack.
    // - You want a resizable array.
    // - You want a heap-allocated array.
    let mut v1 = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);

    // Or (mostly) equivalently via a convenient macro in the standard library,
    let v2 = vec![1, 2, 3];
    assert_eq!(v1, v2);

    // String
    //
    // Strings are growable strings stored as a UTF-8 buffer which are just Vec<u8>s under the
    // hood. Like strs, they are not indexable (for the same reasons) but iterators can be created
    // over the graphemes, codepoints or bytes therein.
    let x = "abc";      // x is of type &str (a borrowed string slice)
    let s1 = String::from(x);
    assert_eq!(x, &s1);

    // or alternatively,
    let s2 = x.to_owned();
    assert_eq!(s1, s2);

    // VecDequeue
    //
    // A growable ring buffer. According to the Rust documentation you should use VecDequeue<T>
    // when:
    // - You want a Vec that supports efficient insertion at both ends of the sequence.
    // - You want a queue.
    // - You want a double-ended queue (deque).
    let mut deque = VecDeque::new();
    deque.push_back(3);
    deque.push_back(4);
    deque.push_back(5);
    assert_eq!(deque.get(1), Some(&4));

    // Linked List
    //
    // A doubly-linked list. According to the Rust documentation, you should use it when:
    // - You want a Vec or VecDeque of unknown size, and can't tolerate amortization.
    // - You want to efficiently split and append lists.
    // - You are absolutely certain you really, truly, want a doubly linked list.
    let mut a = LinkedList::new();
    let mut b = LinkedList::new();
    a.push_back(1);
    a.push_back(2);
    b.push_back(3);
    b.push_back(4);

    // A constant-time and -memory operation.
    a.append(&mut b);

    for e in &a {
        println!("{}", e);      // prints 1, then 2, then 3, then 4
    }

    // HashMap
    //
    // A hash map implementation which uses linear probing with Robin Hood bucket stealing.
    // According to the Rust documentation, you should use it when:
    // - You want to associate arbitrary keys with an arbitrary value.
    // - You want a cache.
    // - You want a map, with no extra functionality.
    let mut map = HashMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");
    for (key, value) in map {
        println!("key: {}, value: {}", key, value);
    }

    // BTreeMap
    //
    // A map based on a B-Tree. According to the Rust documentation, you should use it when:
    // - You're interested in what the smallest or largest key-value pair is.
    // - You want to find the largest or smallest key that is smaller or larger than something.
    // - You want to be able to get all of the entries in order on-demand.
    // - You want a sorted map.
    let mut map = BTreeMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");
    assert_eq!(map.get(&1), Some(&"a"));

    // HashSet/BTreeSet
    //
    // Set implementations that use an empty tuple () as the value of their respective maps (and
    // implement different methods). They should be used when:
    // - You just want to remember which keys you've seen.
    // - There is no meaningful value to associate with your keys.
    // - You just want a set.
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(2);
    assert_eq!(set.len(), 3);

    // BinaryHeap
    //
    // A priority queue implemented with a binary heap. You should use it when
    // - You want to store a bunch of elements, but only ever want to process the "biggest" or
    //   "most important" one at any given time.
    // - You want a priority queue.
    let mut heap = BinaryHeap::new();
    heap.push(1);
    heap.push(5);
    heap.push(2);
    assert_eq!(heap.peek(), Some(&5));
}
