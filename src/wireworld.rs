// Implements http://rosettacode.org/wiki/Wireworld

use std::mem;

// Use VT100 cursor control sequences to animate in-place
#[cfg(not(test))] const ANIMATE: bool = true;

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
    for i in 0..(w*h) {
        match input[i] {
            Cell::Empty(c)  => output[i] = Cell::Empty(c),
            Cell::Tail      => output[i] = Cell::Conductor,
            Cell::Head      => output[i] = Cell::Tail,
            Cell::Conductor => {
                let nc = vec![
                    input.get(i-w-1), input.get(i-w), input.get(i-w+1),
                    input.get(i-1),                   input.get(i+1),
                    input.get(i+w-1), input.get(i+w), input.get(i+w+1)
                ].iter().fold(0, |sum, &o| {
                    if let Some(&Cell::Head) = o { sum + 1 } else { sum }
                });
                output[i] = if nc == 1 || nc == 2 { Cell::Head } else { Cell::Conductor };
            }
        }
    }
}

#[cfg(not(test))]
fn main() {
    use std::thread::sleep_ms;
    let (w, h) = (14usize, 7usize);
    let mut world: Vec<Cell> = "
+-----------+
|tH.........|
|.   .      |
|   ...     |
|.   .      |
|Ht.. ......|
+-----------+
".chars().map(|c| Cell::from_char(c)).collect();
    let mut next: Vec<Cell> = world.clone();

    loop {
        for i in 0..(w*h) {
            print!("{}", world[i].to_char());
        }
        print!("\n");
        next_world(&world, &mut next, 14, 7);
        mem::swap(&mut world, &mut next);

        if ANIMATE {
            print!("\x1b[{}A", 8);
            print!("\x1b[{}D", 14);
            sleep_ms(100);
        }
    }
}

#[test]
fn test() {
    let (w, h) = (14usize, 7usize);
    let mut world: Vec<Cell> = "
+-----------+
|tH.........|
|.   .      |
|   ...     |
|.   .      |
|Ht.. ......|
+-----------+
".chars().map(|c| Cell::from_char(c)).collect();
    let mut next: Vec<Cell> = world.clone();

    for _ in 0..10 {
        next_world(&world, &mut next, w, h);
        mem::swap(&mut world, &mut next);
    }

    let result: String = world.iter().map(|c| c.to_char()).collect();
    let correct = "
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
