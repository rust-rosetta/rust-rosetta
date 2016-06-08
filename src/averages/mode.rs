// http://rosettacode.org/wiki/Averages/Mode

/// Returns the most common element in a Vec<T>, where T
/// must implement the Clone, Eq, and Hash traits, in a
/// Vec<T>.
///
/// ```
/// let v1: Vec<u32> = vec![1,3,6,6,6,6,7,7,12,12,17];
/// println!("{}", mode(&v1));
/// ```
fn mode<T: Clone + Eq + std::hash::Hash>(list: &Vec<T>) -> Vec<T> {
    use std::collections::HashMap;
    let mut hist: HashMap<T, u32> = HashMap::new();
    let mut modes: Vec<T> = Vec::new();
    let mut max: u32 = 0;
    for x in list {
        let counter = hist.entry((*x).clone()).or_insert(0);
        *counter += 1;
        if *counter == max {
            modes.push(x.clone());
        } else if *counter > max {
            max = *counter;
            modes.clear();
            modes.push(x.clone());
        }
    }
    modes
}

fn main() {
    let v1: Vec<i32> = vec![1, 2, 3, 1, 2, 4, 2, 6, 3, 3, 1, 3, 6];
    println!("{:?}", mode(&v1));
}

#[test]
fn simple_tests() {
    let v1: Vec<i8> = vec![1, 2, 3, 2, 1];
    let v2: Vec<u32> = vec![0xdeadbeef, 0xba5eba11, 0xdeadbeef];
    let v3: Vec<char> = vec!['E', 'n', 'e', 'y', 'i', '\u{e4}', 'n'];
    let v4: Vec<u16> = vec![1, 3, 6, 6, 7, 7, 12, 12, 17];
    assert_eq!(mode(&v1), vec![2, 1]);
    assert_eq!(mode(&v2), vec![0xdeadbeef]);
    assert_eq!(mode(&v3), vec!['n']);
    assert_eq!(mode(&v4), vec![6, 7, 12]);
}
