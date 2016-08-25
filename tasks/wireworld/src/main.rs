use std::mem;
use std::thread;
use std::time::Duration;

#[derive(Copy, Clone)]
enum Cell {
    Empty(char),
    Head,
    Tail,
    Conductor,
}

impl Cell {
    fn from_char(c: char) -> Cell {
        match c {
            '.' => Cell::Conductor,
            'H' => Cell::Head,
            't' => Cell::Tail,
            c => Cell::Empty(c),
        }
    }
    fn to_char(&self) -> char {
        match *self {
            Cell::Conductor => '.',
            Cell::Head => 'H',
            Cell::Tail => 't',
            Cell::Empty(c) => c,
        }
    }
}

fn next_world(input: &[Cell], output: &mut [Cell], w: usize, h: usize) {
    for i in 0..(w * h) {
        match input[i] {
            Cell::Empty(c) => output[i] = Cell::Empty(c),
            Cell::Tail => output[i] = Cell::Conductor,
            Cell::Head => output[i] = Cell::Tail,
            Cell::Conductor => {
                let nc = vec![input.get(i - w - 1),
                              input.get(i - w),
                              input.get(i - w + 1),
                              input.get(i - 1),
                              input.get(i + 1),
                              input.get(i + w - 1),
                              input.get(i + w),
                              input.get(i + w + 1)]
                    .iter()
                    .fold(0, |sum, &o| {
                        if let Some(&Cell::Head) = o {
                            sum + 1
                        } else {
                            sum
                        }
                    });
                output[i] = if nc == 1 || nc == 2 {
                    Cell::Head
                } else {
                    Cell::Conductor
                };
            }
        }
    }
}

fn main() {
    let (w, h) = (14usize, 7usize);
    let mut world: Vec<Cell> = r"
+-----------+
|tH.........|
|.   .      |
|   ...     |
|.   .      |
|Ht.. ......|
+-----------+
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

        // Use VT100 cursor control sequences to animate in-place
        print!("\x1b[{}A", 8);
        print!("\x1b[{}D", 14);
        thread::sleep(Duration::from_millis(100));
    }
}

#[test]
fn test() {
    let (w, h) = (14usize, 7usize);
    let mut world: Vec<Cell> = r"
+-----------+
|tH.........|
|.   .      |
|   ...     |
|.   .      |
|Ht.. ......|
+-----------+
"
        .chars()
        .map(Cell::from_char)
        .collect();
    let mut next: Vec<Cell> = world.clone();

    for _ in 0..10 {
        next_world(&world, &mut next, w, h);
        mem::swap(&mut world, &mut next);
    }

    let result: String = world.iter().map(|c| c.to_char()).collect();
    let correct = r"
+-----------+
|.tH.tH.tH.t|
|H   t      |
|   HHH     |
|H   .      |
|t.tH ......|
+-----------+
";
    assert_eq!(result, correct);
}
