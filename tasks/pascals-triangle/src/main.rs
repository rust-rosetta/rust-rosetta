fn pascaltriangle(rows: usize) -> Vec<Vec<usize>> {
    let mut all_rows = Vec::with_capacity(rows);

    for row in 0..rows {
        let mut row_vals = Vec::with_capacity(row + 1);
        let mut value = 1;

        for col in 0..row + 1 {
            row_vals.push(value);
            value = value * (row - col) / (col + 1)
        }

        all_rows.push(row_vals);
    }

    all_rows
}

fn printpascal(rows: &[Vec<usize>]) {
    let mut i = 0;
    for row in rows.iter() {
        for _ in 0..(rows.len() - i) {
            print!(" ");
        }

        for col in row.iter() {
            print!("{} ", col);
        }

        println!("");

        i += 1;
    }
}

fn main() {
    printpascal(&pascaltriangle(5));
}

#[test]
fn test_triangle() {
    assert_eq!(pascaltriangle(5),
               vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1], vec![1, 4, 6, 4, 1]]);
}
