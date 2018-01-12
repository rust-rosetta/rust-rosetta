extern crate rand;

use std::collections::HashMap;
use std::fmt;

use rand::Rng;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Cell {
    Card(usize),
    Empty,
}

#[derive(Eq, PartialEq, Hash, Debug)]
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

type Board = [Cell; 16];
const EMPTY: Board = [Cell::Empty; 16];

struct P15 {
    board: Board,
}

impl P15 {
    fn new() -> Self {
        let mut board = EMPTY;
        for (i, cell) in board.iter_mut().enumerate().skip(1) {
            *cell = Cell::Card(i);
        }

        let mut rng = rand::thread_rng();

        rng.shuffle(&mut board);
        if !Self::is_valid(board) {
            // random swap
            let i = rng.gen_range(0, 16);
            let mut j = rng.gen_range(0, 16);
            while j == i {
                j = rng.gen_range(0, 16);
            }
            board.swap(i, j);
        }

        Self { board }
    }

    fn is_valid(mut board: Board) -> bool {
        // TODO: optimize
        let mut permutations = 0;

        let pos = board.iter().position(|&cell| cell == Cell::Empty).unwrap();

        if pos != 15 {
            board.swap(pos, 15);
            permutations += 1;
        }

        for i in 1..16 {
            let pos = board
                .iter()
                .position(|&cell| match cell {
                    Cell::Card(value) if value == i => true,
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
        self.board.iter().position(|&c| c == Cell::Empty).unwrap()
    }

    fn get_moves(&self) -> HashMap<Direction, Cell> {
        let mut moves = HashMap::new();
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

    fn play(&mut self, direction: &Direction) {
        let i = self.get_empty_position();
        // This is safe because `ask_action` only returns legal moves
        match *direction {
            Direction::Up => self.board.swap(i, i - 4),
            Direction::Left => self.board.swap(i, i - 1),
            Direction::Right => self.board.swap(i, i + 1),
            Direction::Down => self.board.swap(i, i + 4),
        };
    }

    fn is_complete(&self) -> bool {
        self.board.iter().enumerate().all(|(i, &cell)| match cell {
            Cell::Card(value) => value == i + 1,
            Cell::Empty => i == 15,
        })
    }
}

impl fmt::Display for P15 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "+----+----+----+----+\n"));
        for (i, &cell) in self.board.iter().enumerate() {
            match cell {
                Cell::Card(value) => try!(write!(f, "| {:2} ", value)),
                Cell::Empty => try!(write!(f, "|    ")),
            }

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

    for turns in 1.. {
        println!("{}", p15);
        match ask_action(&p15.get_moves()) {
            Action::Move(direction) => {
                p15.play(&direction);
            }
            Action::Quit => {
                println!("Bye !");
                break;
            }
        }

        if p15.is_complete() {
            println!("Well done ! You won in {} turns", turns);
            break;
        }
    }
}

fn ask_action(moves: &HashMap<Direction, Cell>) -> Action {
    use std::io::{self, Write};
    use Action::*;
    use Direction::*;

    println!("Possible moves:");

    if let Some(&Cell::Card(value)) = moves.get(&Up) {
        println!("\tU) {}", value);
    }
    if let Some(&Cell::Card(value)) = moves.get(&Left) {
        println!("\tL) {}", value);
    }
    if let Some(&Cell::Card(value)) = moves.get(&Right) {
        println!("\tR) {}", value);
    }
    if let Some(&Cell::Card(value)) = moves.get(&Down) {
        println!("\tD) {}", value);
    }
    println!("\tQ) Quit");
    print!("Choose your move : ");
    io::stdout().flush().unwrap();

    let mut action = String::new();
    io::stdin().read_line(&mut action).expect("read error");
    match action.to_uppercase().trim() {
        "U" if moves.contains_key(&Up) => Move(Up),
        "L" if moves.contains_key(&Left) => Move(Left),
        "R" if moves.contains_key(&Right) => Move(Right),
        "D" if moves.contains_key(&Down) => Move(Down),
        "Q" => Quit,
        _ => {
            println!("Unknown action: {}", action);
            ask_action(moves)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn board_from_ints(ints: &[usize; 16]) -> Board {
        let mut board = EMPTY;
        for (cell, &int) in board.iter_mut().zip(ints.iter()) {
            *cell = if int == 0 {
                Cell::Empty
            } else {
                Cell::Card(int)
            }
        }
        board
    }

    fn make_valid<R: Rng>(rng: &mut R, mut board: Board) -> Board {
        let i = rng.gen_range(0, 16);
        let mut j = rng.gen_range(0, 16);
        while j == i {
            j = rng.gen_range(0, 16);
        }
        board.swap(i, j);
        board
    }

    #[test]
    fn board_creation() {
        let p15 = P15::new();
        assert!(P15::is_valid(p15.board));
    }

    #[test]
    fn board_validity() {
        let mut rng = rand::weak_rng();

        fn assert_is_valid<R: Rng>(rng: &mut R, ints: &[usize; 16]) {
            let board = board_from_ints(ints);
            assert!(P15::is_valid(board));
        }

        fn assert_is_not_valid<R: Rng>(rng: &mut R, ints: &[usize; 16]) {
            let board = board_from_ints(ints);
            assert!(!P15::is_valid(board));
            assert!(P15::is_valid(make_valid(rng, board)));
        }

        assert_is_not_valid(&mut rng, &[2, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0]);
        assert_is_not_valid(&mut rng, &[13, 10, 11, 6, 5, 7, 4, 8, 1, 12, 14, 9, 3, 15, 2, 0]);
        assert_is_not_valid(&mut rng, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 15, 14, 0]);
        assert_is_not_valid(&mut rng, &[2, 1, 3, 4, 5, 8, 7, 6, 9, 10, 12, 11, 15, 13, 14, 0]);
        assert_is_not_valid(&mut rng, &[4, 15, 7, 9, 3, 12, 1, 8, 5, 0, 11, 2, 13, 6, 10, 14]);

        assert_is_valid(&mut rng, &[7, 1, 3, 14, 6, 0, 4, 8, 5, 9, 13, 2, 12, 15, 11, 10]);
        assert_is_valid(&mut rng, &[7, 9, 1, 14, 6, 0, 13, 10, 8, 3, 12, 4, 15, 5, 2, 11]);
        assert_is_valid(&mut rng, &[15, 8, 9, 14, 5, 10, 1, 0, 12, 4, 3, 13, 11, 2, 7, 6]);
        assert_is_valid(&mut rng, &[1, 12, 4, 8, 10, 7, 2, 11, 6, 3, 15, 14, 9, 13, 5, 0]);
    }

    #[test]
    fn directions() {
        fn get_puzzle_moves(ints: &[usize; 16]) -> HashMap<Direction, Cell> {
            let board = board_from_ints(&ints);
            let puzzle = P15 { board };
            puzzle.get_moves()
        }

        let mut test = HashMap::new();
        test.insert(Direction::Up, Cell::Card(12));
        test.insert(Direction::Left, Cell::Card(15));
        let moves = get_puzzle_moves(&[13, 6, 8, 3, 1, 5, 2, 4, 9, 7, 10, 12, 14, 11, 15, 0]);
        assert_eq!(moves, test);

        let mut test = HashMap::new();
        test.insert(Direction::Up, Cell::Card(12));
        test.insert(Direction::Left, Cell::Card(14));
        test.insert(Direction::Right, Cell::Card(8));
        test.insert(Direction::Down, Cell::Card(15));
        let moves = get_puzzle_moves(&[7, 12, 2, 1, 14, 0, 8, 13, 3, 15, 4, 6, 11, 5, 10, 9]);
        assert_eq!(moves, test);
    }

    #[test]
    fn victory() {
        let input = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
        let board = board_from_ints(&input);
        let puzzle = P15 { board };
        assert!(puzzle.is_complete());
    }
}
