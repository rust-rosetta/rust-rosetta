use std::collections::LinkedList;

/// Count the number of Pythagorean triples whose sum are below the specified limit (inclusive).
/// Does a BFS over the tree of primitive Pythagorean triples (see [0]), and uses the fact that
/// each child has a bigger sum than its parent.
/// [0]: http://en.wikipedia.org/wiki/Tree_of_Pythagorean_triples
fn count_pythagorean_triples(below: u64) -> (u64, u64) {
    let mut tot_cnt = 0;
    let mut prim_cnt = 0;
    let mut queue = LinkedList::new();

    // Initiate the BFS with the root of the tree: (3, 4, 5)
    queue.push_back((3i64, 4i64, 5i64));

    while let Some((a, b, c)) = queue.pop_back() {
        // We found a new primitive Pythagorean triplet: (a, b, c).
        // (k*a, k*b, k*c) is a (not necessarily primitive) Pythagorean triplet for any positive
        // integer k.
        // We're interested in those with k*a + k*b + k*c <= below, and the number of them are
        // exactly below / (a + b + c)
        let cur = below / (a + b + c) as u64;
        if cur > 0 {
            tot_cnt += cur;
            prim_cnt += 1;

            // Explore the children of the current node
            queue.push_back((a - 2 * b + 2 * c, 2 * a - b + 2 * c, 2 * a - 2 * b + 3 * c));
            queue.push_back((a + 2 * b + 2 * c, 2 * a + b + 2 * c, 2 * a + 2 * b + 3 * c));
            queue.push_back((-a + 2 * b + 2 * c, -2 * a + b + 2 * c, -2 * a + 2 * b + 3 * c));
        }
    }

    (tot_cnt, prim_cnt)
}

fn main() {
    for n in 1..9 {
        let (tot, prim) = count_pythagorean_triples(10u64.pow(n));
        println!("Up to 10^{}: {:>10} triples {:>10} primitives",
                 n,
                 tot,
                 prim);
    }
}

#[test]
fn test_count_pythagorean_triples() {
    assert_eq!(count_pythagorean_triples(10u64.pow(6)), (808950, 70229));
}
