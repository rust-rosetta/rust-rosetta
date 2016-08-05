#[cfg(test)]
#[macro_use]
extern crate meta;

fn insertion_sort<T>(arr: &mut [T])
    where T: Ord
{
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn main() {
    let mut arr = vec![6, 8, 5, 9, 3, 2, 1, 4, 7];
    insertion_sort(&mut arr);
    println!("{:?}", arr);
}

#[cfg(test)]
mod tests {
    test_sort!(super::insertion_sort);
}
