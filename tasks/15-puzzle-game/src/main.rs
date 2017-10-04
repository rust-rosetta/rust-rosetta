extern crate rand;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::fmt;

#[derive(Copy, Clone)]
enum Cell {
    Card(usize),
    Empty,
}

#[derive(Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Action {
    Move(Direction),
    Quit,
}

struct P15 {
    board: Vec<Cell>,
}

impl P15 {
    // todo: make the board valid right from the start.
    fn new() -> P15 {
        let mut board = (1..16).map(Cell::Card).collect::<Vec<_>>();
        board.push(Cell::Empty);

        let mut rng = thread_rng();

        rng.shuffle(board.as_mut_slice());
        while !P15::is_valid(board.clone()) {
            rng.shuffle(board.as_mut_slice());
        }

        P15 { board: board }
    }

    fn is_valid(mut board: Vec<Cell>) -> bool {
        let mut permutations = 0;

        let pos = board.iter()
            .position(|&cell| match cell {
                Cell::Empty => true,
                _ => false,
            })
            .unwrap();

        if pos != 15 {
            board.swap(pos, 15);
            permutations += 1;
        }

        for i in 1..16 {
            let pos = board.iter()
                .position(|&cell| match cell {
                    Cell::Card(val) if val == i => true,
                    _ => false,
                })
                .unwrap();

            if pos + 1 != i {
                board.swap(pos, i - 1);
                permutations += 1;
            }
        }

        permutations % 2 == 0
    }

    fn get_empty_position(&self) -> usize {
        self.board
            .iter()
            .position(|&cell| match cell {
                Cell::Empty => true,
                _ => false,
            })
            .unwrap()
    }

    fn get_moves(&self) -> HashMap<Direction, Cell> {
        let mut moves = HashMap::with_capacity(4);
        let i = self.get_empty_position();

        if i > 3 {
            moves.insert(Direction::Up, self.board[i - 4]);
        }
        if i % 4 != 0 {
            moves.insert(Direction::Left, self.board[i - 1]);
        }
        if i < 12 {
            moves.insert(Direction::Down, self.board[i + 4]);
        }
        if i % 4 != 3 {
            moves.insert(Direction::Right, self.board[i + 1]);
        }
        moves
    }

    fn play(&mut self, direction: Direction) {
        use Direction::*;
        let i = self.get_empty_position();
        match direction {
            Up => self.board.swap(i, i - 4),
            Left => self.board.swap(i, i - 1),
            Right => self.board.swap(i, i + 1),
            Down => self.board.swap(i, i + 4),
        };
    }

    fn is_complete(&self) -> bool {
        for (i, cell) in self.board.iter().enumerate() {
            match cell {
                &Cell::Card(value) if value == i + 1 => (),
                &Cell::Empty if i == 15 => (),
                _ => return false,
            };
        }
        true
    }
}

impl fmt::Display for P15 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Cell::*;
        try!(write!(f, "+----+----+----+----+\n"));
        for (i, cell) in self.board.iter().enumerate() {
            match cell {
                &Card(value) => {
                    try!(write!(f, "| {:2} ", value));
                }
                &Empty => {
                    try!(write!(f, "|    "));
                }
            };

            if i % 4 == 3 {
                try!(write!(f, "|\n"));
                try!(write!(f, "+----+----+----+----+\n"));
            }
        }
        Ok(())
    }
}

fn main() {
    let mut p15 = P15::new();
    let mut turns = 0;
    loop {
        println!("{}", p15);
        match ask_action(&p15.get_moves()) {
            Action::Move(direction) => {
                p15.play(direction);
            }
            Action::Quit => {
                println!("Bye !");
                break;
            }
        }

        turns += 1;

        if p15.is_complete() {
            println!("Well done ! You win in {} turns", turns);
            break;
        }

    }
}

fn ask_action(moves: &HashMap<Direction, Cell>) -> Action {
    use std::io::{self, Write};
    use Action::*;
    use Direction::*;
    println!("Possible moves :");

    match moves.get(&Up) {
        Some(&Cell::Card(value)) => {
            println!("\tU) {}", value);
        }
        _ => (),
    }
    match moves.get(&Left) {
        Some(&Cell::Card(value)) => {
            println!("\tL) {}", value);
        }
        _ => (),
    }
    match moves.get(&Right) {
        Some(&Cell::Card(value)) => {
            println!("\tR) {}", value);
        }
        _ => (),
    }
    match moves.get(&Down) {
        Some(&Cell::Card(value)) => {
            println!("\tD) {}", value);
        }
        _ => (),
    }
    println!("\tQ) Quit");
    print!("Choose your move : ");
    io::stdout().flush().unwrap();

    let mut action = String::new();
    io::stdin().read_line(&mut action).ok().expect("read error");
    match action.to_uppercase().trim() {
        "U" if moves.contains_key(&Up) => Move(Up),
        "L" if moves.contains_key(&Left) => Move(Left),
        "R" if moves.contains_key(&Right) => Move(Right),
        "D" if moves.contains_key(&Down) => Move(Down),
        "Q" => Quit,
        _ => {
            println!("Unknown action : {}", action);
            ask_action(moves)
        }
    }
}

