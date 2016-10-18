//! Given a set, generate its power set, which is the set of all subsets of that set.

use std::vec::Vec;
use std::slice::Iter;

// If set == {}
//   return {{}}
// else if set == {a} U rest
//   return power_set(rest) U ({a} U each set in power_set(rest))
fn power_set<T>(items: &mut Iter<T>) -> Vec<Vec<T>>
    where T: Clone
{
    let mut power = Vec::new();
    match items.next() {
        None => power.push(Vec::new()),
        Some(item) => {
            for mut set in power_set(items).into_iter() {
                power.push(set.clone());
                set.push(item.clone());
                power.push(set);
            }
        }
    }
    power
}

#[test]
fn test() {
    let set = Vec::<i32>::new();
    let power = power_set(&mut set.iter());
    assert!(power == vec![vec![]]);

    let mut set = Vec::<i32>::new();
    set.push(1);
    set.push(2);
    set.push(3);
    let power = power_set(&mut set.iter());
    assert!(power ==
            vec![vec![],
                 vec![1],
                 vec![2],
                 vec![2, 1],
                 vec![3],
                 vec![3, 1],
                 vec![3, 2],
                 vec![3, 2, 1]]);
}

fn main() {
    let mut set = Vec::<i32>::new();
    set.push(1);
    set.push(2);
    set.push(3);
    set.push(4);
    let power = power_set(&mut set.iter());
    println!("Set      : {:?}", set);
    println!("Power Set: {:?}", power);
}
