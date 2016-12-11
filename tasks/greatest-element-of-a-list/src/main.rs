use std::fmt::Display;

fn main() {
    find_max("first", &[1i32, 2, 3, 4, 5, 6, 7, 8, 9]);
    find_max("second",
             &[123i32, 3543, 23, 432, 5, 2, 34, 234, 234, 2, 4, 234, 23, 4, 24, 25, 7, 658, 68]);
    find_max("third", &['a', 'b', 'c', 'd', 'e']);
    find_max("fourth",
             &["Bonjour", "Hola", "Hello", "Hallo", "Buongiorno"]);
}

fn find_max<T: Display + Ord>(count: &str, list: &[T]) {
    let max = list.iter().max().unwrap();
    println!("Max of the {} list: {}", count, max);
}
