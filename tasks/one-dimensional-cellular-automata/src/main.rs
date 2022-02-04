const MAX_GENERATION: usize = 10;
const CELLS_LENGTH: usize = 20;

fn get_new_state(windowed: &[bool]) -> bool {
    match windowed {
        [false, true, true] | [true, true, false] => true,
        _ => false,
    }
}

fn next_gen(cell: &mut [bool]) {
    let mut v = Vec::with_capacity(cell.len());
    v.push(cell[0]);
    for i in cell.windows(3) {
        v.push(get_new_state(i));
    }
    v.push(cell[cell.len() - 1]);
    cell.copy_from_slice(&v);
}

fn print_cell(cell: &[bool]) {
    for v in cell {
        print!("{} ", if *v { '#' } else { ' ' });
    }
    println!();
}

fn main() {
    let mut cell: [bool; CELLS_LENGTH] = rand::random();

    for i in 1..=MAX_GENERATION {
        print!("Gen {:2}: ", i);
        print_cell(&cell);
        next_gen(&mut cell);
    }
}
