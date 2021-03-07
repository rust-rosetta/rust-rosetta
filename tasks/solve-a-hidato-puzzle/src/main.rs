use std::cmp::{max, min};
use std::fmt;
use std::ops;

#[derive(Debug, Clone, PartialEq)]
struct Board {
    cells: Vec<Vec<Option<u32>>>,
}

impl Board {
    fn new(initial_board: Vec<Vec<u32>>) -> Self {
        let b = initial_board
            .iter()
            .map(|r| {
                r.iter()
                    .map(|c| if *c == u32::MAX { None } else { Some(*c) })
                    .collect()
            })
            .collect();

        Board { cells: b }
    }

    fn height(&self) -> usize {
        self.cells.len()
    }

    fn width(&self) -> usize {
        self.cells[0].len()
    }
}
impl ops::Index<(usize, usize)> for Board {
    type Output = Option<u32>;

    fn index(&self, (y, x): (usize, usize)) -> &Self::Output {
        &self.cells[y][x]
    }
}
impl ops::IndexMut<(usize, usize)> for Board {
    /// Returns a mutable reference to an cell for a given 'x' 'y' coordinates
    fn index_mut(&mut self, (y, x): (usize, usize)) -> &mut Option<u32> {
        &mut self.cells[y][x]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output: Vec<String> = self
            .cells
            .iter()
            .map(|r| {
                let mut row = String::default();

                r.iter().for_each(|c| match c {
                    None => row.push_str(format!("{:>2} ", " ").as_ref()),
                    Some(c) if c == &0 => row.push_str(format!("{:>2} ", ".").as_ref()),
                    Some(c) => row.push_str(format!("{:>2} ", c).as_ref()),
                });
                row
            })
            .collect();

        write!(f, "{}", output.join("\n"))
    }
}

/// Structure for holding puzzle related information.
#[derive(Clone, Debug)]
struct Puzzle {
    /// The state of the board.
    board: Board,

    /// All the numbers which were given at puzzle setup:
    /// the numbers which cannot be changed during solving the puzzle.
    fixed: Vec<u32>,

    /// Position of the first number (1).
    start: (usize, usize),
}

impl Puzzle {
    /// Creates a new puzzle
    /// * `initial_board` contains the  layout and the startin position.
    ///
    /// - Simple numbers in the `initial_board` are considered as "fixed",
    /// aka the solving does not change them
    ///
    /// - As the board can be non-rectangular, all cells which are invalid or cannot be used
    /// are marked with u32::MAX in the `initial_board`
    fn new(initial_board: Vec<Vec<u32>>) -> Self {
        let mut s: (usize, usize) = (0, 0);
        let mut f = initial_board
            .iter()
            .enumerate()
            .flat_map(|(y, r)| r.iter().enumerate().map(move |(x, c)| (y, x, *c)))
            .filter(|(_, _, c)| (1..u32::MAX).contains(c))
            .fold(Vec::new(), |mut fixed, (y, x, c)| {
                fixed.push(c);
                if c == 1 {
                    // store the position of the start
                    s = (y, x)
                };
                fixed
            });

        f.sort_unstable();

        Puzzle {
            board: Board::new(initial_board),
            fixed: f,
            start: s,
        }
    }

    pub fn print_board(&self) {
        println!("{}", self.board);
    }

    fn solver(&mut self, current: (usize, usize), n: &u32, mut next: usize) -> bool {
        // reached the last number, solving successful
        if n > self.fixed.last().unwrap() {
            return true;
        }

        // check for exit conditions
        match self.board[current] {
            // cell outside of the board
            None => return false,

            //cell is already has a number in it
            Some(c) if c != 0 && c != *n => return false,

            //cell is empty, but the to be placed number is already matching the next fixed number
            Some(c) if c == 0 && self.fixed[next] == *n => return false,

            // continue
            _ => (),
        }

        let mut backup: u32 = 0;
        if self.board[current] == Some(*n) {
            backup = *n;
            next += 1;
        }

        self.board[current] = Some(*n);

        for y in (max(current.0, 1) - 1)..=min(current.0 + 1, self.board.height() - 1) {
            for x in (max(current.1, 1) - 1)..=min(current.1 + 1, self.board.width() - 1) {
                if self.solver((y, x), &(n + 1), next) {
                    return true;
                }
            }
        }

        // unsuccessful branch, restore original value
        self.board[current] = Some(backup);
        false
    }

    pub fn solve(&mut self) {
        let start = self.start;
        self.solver(start, &1, 0);
    }
}

fn main() {
    let input = vec![
        vec![0, 33, 35, 0, 0, u32::MAX, u32::MAX, u32::MAX],
        vec![0, 0, 24, 22, 0, u32::MAX, u32::MAX, u32::MAX],
        vec![0, 0, 0, 21, 0, 0, u32::MAX, u32::MAX],
        vec![0, 26, 0, 13, 40, 11, u32::MAX, u32::MAX],
        vec![27, 0, 0, 0, 9, 0, 1, u32::MAX],
        vec![u32::MAX, u32::MAX, 0, 0, 18, 0, 0, u32::MAX],
        vec![u32::MAX, u32::MAX, u32::MAX, u32::MAX, 0, 7, 0, 0],
        vec![
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            5,
            0,
        ],
    ];

    let mut p = Puzzle::new(input);
    p.print_board();
    p.solve();
    println!("\nSolution:");
    p.print_board();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solver() {
        let input = vec![vec![0, 1], vec![4, 0]];
        let mut p = Puzzle::new(input);
        p.solve();
        assert_eq!(p.board, Board::new(vec![vec![2, 1], vec![4, 3]]));

        // test disabled cells
        let input = vec![vec![u32::MAX, 1, 0], vec![5, 0, 0]];
        let mut p = Puzzle::new(input);
        p.solve();
        assert_eq!(
            p.board,
            Board::new(vec![vec![u32::MAX, 1, 2], vec![5, 4, 3]])
        );
    }
}
