// Implements http://rosettacode.org/wiki/Pascal%27s_triangle

fn pascaltriangle(rows: uint) {
    for row in range(0, rows) {
        let mut value = 1;

        for _space in range(0, (rows - row)) {
            print!(" ");
        }

        for col in range(0, row + 1) {
            print!("{} ", value);
            value = value * (row - col)/(col + 1)
        }

        println!("");
    }
}

fn main() {
    pascaltriangle(5);
}
