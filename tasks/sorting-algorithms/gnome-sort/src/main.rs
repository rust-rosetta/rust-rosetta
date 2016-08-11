#[cfg(test)]
#[macro_use]
extern crate meta;

fn gnome_sort<T: PartialOrd>(v: &mut [T]) {
    let len = v.len();
    let mut i: usize = 1;
    let mut j: usize = 2;
    while i < len {
        if v[i - 1] <= v[i] {
            i = j;
            j += 1;
        } else {
            v.swap(i - 1, i);
            i -= 1;
            if i == 0 {
                i = j;
                j += 1;
            }
        }
    }
}

fn main() {
    let mut numbers = [4i32, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);
    gnome_sort(&mut numbers);
    println!("After: {:?}", numbers);
}

#[cfg(test)]
mod tests {
    test_sort!(super::gnome_sort);
}
