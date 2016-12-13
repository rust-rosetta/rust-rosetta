use std::time::Duration;
use std::mem;
use std::thread;

#[derive(Copy, Clone)]
enum Cell {
    Empty(char),
    Alive,
    Dead,
}
impl Cell {
    fn from_char(c: char) -> Cell {
        match c {
            ' ' => Cell::Dead,
            '#' => Cell::Alive,
            c => Cell::Empty(c),
        }
    }
    fn to_char(&self) -> char {
        match *self {
            Cell::Dead => ' ',
            Cell::Alive => '#',
            Cell::Empty(c) => c,
        }
    }
}

#[cfg_attr(feature="clippy", allow(match_same_arms))]
fn next_world(input: &[Cell], output: &mut [Cell], w: usize, h: usize) {
    for i in 0..(w * h) {
        match input[i] {
            Cell::Empty(c) => output[i] = Cell::Empty(c),
            cell => {
                let live = vec![input.get(i - w - 1),
                                input.get(i - w),
                                input.get(i - w + 1),
                                input.get(i - 1),
                                input.get(i + 1),
                                input.get(i + w - 1),
                                input.get(i + w),
                                input.get(i + w + 1)]
                    .iter()
                    .fold(0, |sum, &o| {
                        if let Some(&Cell::Alive) = o {
                            sum + 1
                        } else {
                            sum
                        }
                    });
                output[i] = match (cell, live) {
                    (Cell::Alive, 0...1) => Cell::Dead,  // Lonely
                    (Cell::Alive, 4...8) => Cell::Dead,  // Overcrowded
                    (Cell::Alive, 2...3) => Cell::Alive, // Lives
                    (Cell::Dead, 3) => Cell::Alive, // It takes three to give birth!
                    _ => Cell::Dead,  // Barren
                }
            }
        }
    }
}

fn main() {
    let (w, h) = (100usize, 9usize);
    let mut world: Vec<Cell> = r"
+-------------------------------------------------------------------------------------------------+
|                                                                                                 |
|                                                                                        #  #     |
|                                                                                       #         |
|                                                                                       #   #     |
|                                                                                       ####      |
|                                                                                                 |
|                                                                                                 |
+-------------------------------------------------------------------------------------------------+
"
        .chars()
        .map(Cell::from_char)
        .collect();
    let mut next: Vec<Cell> = world.clone();

    loop {
        for cell in &world {
            print!("{}", cell.to_char());
        }
        print!("\n");
        next_world(&world, &mut next, w, h);
        mem::swap(&mut world, &mut next);

        // Use VT100 cursor control sequences to animate in-place.
        print!("\x1b[{}A", h + 1);
        print!("\x1b[{}D", w + 1);
        thread::sleep(Duration::from_millis(100));
    }
}

#[test]
fn test() {
    let (w, h) = (14usize, 7usize);
    let mut world: Vec<Cell> = r"
+-----------+
|           |
|     #     |
|     #     |
|     #     |
|           |
+-----------+
"
        .chars()
        .map(Cell::from_char)
        .collect();
    let mut next: Vec<Cell> = world.clone();

    next_world(&world, &mut next, w, h);
    mem::swap(&mut world, &mut next);

    let result: String = world.iter().map(|c| c.to_char()).collect();
    let correct = r"
+-----------+
|           |
|           |
|    ###    |
|           |
|           |
+-----------+
";
    assert_eq!(result, correct);
}
