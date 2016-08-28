use std::collections::HashSet;

fn main() {
    // The first set contains integers from 0 to 7
    let set1 = (0i32..7).collect::<HashSet<i32>>();

    // The second set contains integers from 5 to 10
    let set2 = (5i32..10).collect();

    // A subset of set1
    let subset1 = (2i32..5).collect::<HashSet<i32>>();

    // Test if element is member of the set
    assert!(set1.contains(&1));

    // Test if subset1 is subset of set1
    assert!(subset1.is_subset(&set1));

    // Test if set1_copy is equal to set1
    let set1_copy = set1.clone();
    assert!(set1_copy == set1);

    println!("");
    println!("Print the union of set1 and set2");
    for num in set1.union(&set2) {
        println!("{}", num);
    }

    println!("");
    println!("Print the intersection of set1 and set2");
    for num in set1.intersection(&set2) {
        println!("{}", num);
    }

    println!("");
    println!("Print the difference between set1 and set2");
    for num in set1.difference(&set2) {
        println!("{}", num);
    }
}
