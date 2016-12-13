use std::fmt::Display;

fn comb<T>(arr: &[T], n: u32)
    where T: Display
{
    let mut incl_arr = vec![false; arr.len()];
    comb_intern(arr, n as usize, &mut incl_arr, 0);
}

fn comb_intern<T>(arr: &[T], n: usize, incl_arr: &mut [bool], index: usize)
    where T: Display
{
    if arr.len() < n + index {
        return;
    }
    if n == 0 {
        let it = arr.iter().zip(incl_arr.iter()).filter_map(|(val, incl)| {
            if *incl {
                Some(val)
            } else {
                None
            }
        });
        for val in it {
            print!("{} ", *val);
        }
        print!("\n");
        return;
    }

    incl_arr[index] = true;
    comb_intern(arr, n - 1, incl_arr, index + 1);
    incl_arr[index] = false;

    comb_intern(arr, n, incl_arr, index + 1);
}

fn main() {
    let vec1 = vec![1, 2, 3, 4, 5];
    comb(&vec1, 3);

    let vec2 = vec!["A", "B", "C", "D", "E"];
    comb(&vec2, 3);
}
