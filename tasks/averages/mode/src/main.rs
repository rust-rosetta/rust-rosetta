use std::collections::HashMap;
use std::hash::Hash;

/// Returns the most common element a collection implementing
/// `IntoIterator<Item=T>`, where `T` must implement the `Eq` and `Hash` traits
///
/// ```
/// let v1 = vec![1,3,6,6,6,6,7,7,12,12,17];
/// println!("{}", mode(v1));
/// ```
fn mode<I>(items: I) -> Vec<I::Item>
    where I: IntoIterator,
          I::Item: Hash + Eq
{
    // NOTE: Usually, you wouldn't need to explicitly call `into_iter()` before
    // looping over a type implementing `IntoIterator`. However, we do it here
    // because we need to call `size_hint()` on the iterator so we can
    // preallocate the `HashMap`.
    let items = items.into_iter();
    let (lower_bound, _upper_bound) = items.size_hint();

    // Allocate a new HashMap with enough space to fit the lower bound of the
    // number of items in `items`.
    //
    // If the lower bound (`lower_bound`) is the same as the upper bound as
    // will be the case for most collections (e.g. `Vec<T>`, `[T]`,
    // `HashSet<T>`, etc.) it will be an overestimate on the number of unique
    // elements in the collection. This means that, in the common case, we'll
    // never have to grow the `HashMap`. While overestimating means we'll likely
    // use more memory than necessary, the allocation size will usually be
    // proportional to the size of the input collection (assuming it's not a
    // lazy collection). This `HashMap` is short lived anyways.
    let mut map = HashMap::with_capacity(lower_bound); // HashMap<I::Item, i32>

    // Count the number of occurrences of each item.
    for item in items {
        *map.entry(item).or_insert(0) += 1;
    }

    let max = map.values()         // Iterate over the counts by reference.
                 .cloned()         // Convert the `&i32`s to `i32`s.
                 .max()            // Find the maximum.
                 .unwrap_or(0);    // If there are no items, default 0.

    map.into_iter()                // Iterate by `(item, value)` pairs.
       .filter(|&(_, v)| v == max) // Find all modes (there may be multiple).
       .map(|(k, _)| k)            // Go from `(item, value)` pairs to `item`s.
       .collect()                  // Collect into a `Vec<I::Item>`.
}

fn main() {
    let items = vec![1, 2, 3, 1, 2, 4, 2, 6, 3, 3, 1, 3, 6];
    println!("{:?}", mode(&items));
}

#[test]
fn simple_tests() {
    let v1 = vec![1, 2, 3, 2, 1];
    let mut m1 = mode(v1);
    m1.sort();
    assert_eq!(m1, vec![1, 2]);

    let v2: &[u64] = &[0xdeadbeef, 0xba5eba11, 0xdeadbeef];
    let mut m2 = mode(v2.iter().cloned());
    m2.sort();
    assert_eq!(m2, vec![0xdeadbeef]);

    let v3 = "Eneyi\u{e4}n";
    let mut m3 = mode(v3.chars());
    m3.sort();
    assert_eq!(m3, vec!['n']);

    let v4 = vec![1, 3, 6, 6, 7, 7, 12, 12, 17];
    let mut m4 = mode(&v4);
    m4.sort();
    assert_eq!(m4, &[&6, &7, &12]);
}
