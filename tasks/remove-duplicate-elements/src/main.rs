use std::vec::Vec;
use std::collections::HashSet;
use std::hash::Hash;
use std::cmp::Eq;

fn main() {
    let mut sample_elements = vec![0u8, 0, 1, 1, 2, 3, 2];
    println!("Before removal of duplicates : {:?}", sample_elements);
    remove_duplicate_elements(&mut sample_elements);
    println!("After removal of duplicates : {:?}", sample_elements);
}

fn remove_duplicate_elements<T: Hash + Eq>(elements: &mut Vec<T>) {
    let set: HashSet<_> = elements.drain(..).collect();
    elements.extend(set.into_iter());
}

#[test]
fn test_remove_duplicate_elements() {
    let mut sample_elements = vec![0u8, 0, 1, 1, 2, 3, 2];
    remove_duplicate_elements(&mut sample_elements);
    sample_elements.sort();
    assert_eq!(sample_elements, vec![0, 1, 2, 3])
}
