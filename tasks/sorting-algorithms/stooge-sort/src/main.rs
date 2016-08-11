#[cfg(test)]
#[macro_use]
extern crate meta;

fn stoogesort<E>(a: &mut [E])
    where E: PartialOrd
{
    if a.is_empty() {
        return;
    }

    let len = a.len();

    if a.first().unwrap() > a.last().unwrap() {
        a.swap(0, len - 1);
    }
    if len - 1 > 1 {
        let t = len / 3;
        stoogesort(&mut a[..len - 1]);
        stoogesort(&mut a[t..]);
        stoogesort(&mut a[..len - 1]);
    }
}

fn main() {
    let mut numbers = vec![1_i32, 9, 4, 7, 6, 5, 3, 2, 8];
    println!("Before: {:?}", &numbers);
    stoogesort(&mut numbers);
    println!("After: {:?}", &numbers);
}

#[cfg(test)]
mod tests {
    test_sort!(super::stoogesort);
}
