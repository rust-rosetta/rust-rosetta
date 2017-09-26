#![feature(iterator_step_by)]

use std::fmt;
use std::str::FromStr;

const BOARD_WIDTH: usize = 9;
const BOARD_HEIGHT: usize = 9;
const GROUP_WIDTH: usize = 3;
const GROUP_HEIGHT: usize = 3;
const MAX_NUMBER: usize = 9;

type BITS = u16;
const MASK_ALL: BITS = 0x1ff;
const INVALID_CELL: u32 = !0;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Sudoku {
    map: [[BITS; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Sudoku {
    fn new() -> Sudoku {
        Sudoku { map: [[MASK_ALL; BOARD_WIDTH]; BOARD_HEIGHT] }
    }

    fn get(&self, x: usize, y: usize) -> u32 {
        match self.map[y][x].count_ones() {
            0 => INVALID_CELL,
            1 => self.map[y][x].trailing_zeros() + 1,
            _ => 0,
        }
    }

    fn set(&mut self, x: usize, y: usize, n: u32) {
        self.map[y][x] = 1 << (n - 1);
    }
}

impl Default for Sudoku {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for Sudoku {
    type Err = ();

    fn from_str(s: &str) -> Result<Sudoku, ()> {
        let mut sudoku = Sudoku::new();

        for (y, line) in s.lines().filter(|l| !l.is_empty()).enumerate() {
            let line = line.trim_matches(|c: char| c.is_whitespace());
            for (x, c) in line.chars().enumerate() {
                if let Some(d) = c.to_digit(10) {
                    if d != 0 {
                        sudoku.set(x, y, d);
                    }
                } else {
                    return Err(());
                }
            }
        }

        Ok(sudoku)
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hbar = "+---+---+---+";

        for y in 0..BOARD_HEIGHT {
            if y % GROUP_HEIGHT == 0 {
                try!(writeln!(f, "{}", hbar));
            }

            for x in 0..BOARD_WIDTH {
                if x % GROUP_WIDTH == 0 {
                    try!(write!(f, "|"));
                }

                match self.get(x, y) {
                    INVALID_CELL => try!(write!(f, "!")),
                    0 => try!(write!(f, " ")),
                    d => try!(write!(f, "{}", d)),
                }
            }
            try!(writeln!(f, "|"));
        }
        try!(writeln!(f, "{}", hbar));

        Ok(())
    }
}

fn solve_sudoku(mut puzzle: Sudoku) -> Vec<Sudoku> {
    let idx_in_grp = [(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2), (2, 0), (2, 1), (2, 2)];

    loop {
        let bkup = puzzle;

        // If the number at cell (x, y) is uniquely determined, that number must
        // not have appeared at the cells in the same row/column/group.
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                if puzzle.map[y][x].count_ones() != 1 {
                    continue;
                }

                let (x0, y0) = ((x / GROUP_WIDTH) * GROUP_WIDTH, (y / GROUP_HEIGHT) * GROUP_HEIGHT);

                let row = (0..BOARD_WIDTH).map(|x| (x, y));
                let col = (0..BOARD_HEIGHT).map(|y| (x, y));
                let grp = idx_in_grp.iter().map(|&(dx, dy)| (x0 + dx, y0 + dy));

                let it = row.chain(col)
                    .chain(grp)
                    .filter(|&pos: &(usize, usize)| pos != (x, y));

                let mask = !puzzle.map[y][x] & MASK_ALL;
                for (x, y) in it {
                    puzzle.map[y][x] &= mask;
                }
            }
        }

        // If `n` appears only once at the cell in the row/column/group, the
        // number of the cell must be `n`.
        for n in 0..MAX_NUMBER {
            let bit = 1 << n;

            // Check each rows
            for y in 0..BOARD_HEIGHT {
                let next = {
                    let mut it = (0..BOARD_WIDTH).filter(|&x| puzzle.map[y][x] & bit != 0);
                    let next = it.next();
                    if next.is_none() || it.next().is_some() {
                        continue;
                    }
                    next
                };
                puzzle.map[y][next.unwrap()] = bit;
            }

            // Check each column
            for x in 0..BOARD_WIDTH {
                let next = {
                    let mut it = (0..BOARD_HEIGHT).filter(|&y| puzzle.map[y][x] & bit != 0);
                    let next = it.next();
                    if next.is_none() || it.next().is_some() {
                        continue;
                    }
                    next
                };
                puzzle.map[next.unwrap()][x] = bit;
            }

            // Check each group
            for y0 in (0..BOARD_HEIGHT).step_by(GROUP_WIDTH) {
                for x0 in (0..BOARD_WIDTH).step_by(GROUP_HEIGHT) {
                    let next = {
                        let mut it = idx_in_grp.iter()
                            .map(|&(dx, dy)| (x0 + dx, y0 + dy))
                            .filter(|&(x, y)| puzzle.map[y][x] & bit != 0);
                        let next = it.next();
                        if next.is_none() || it.next().is_some() {
                            continue;
                        }
                        next
                    };
                    let (x, y) = next.unwrap();
                    puzzle.map[y][x] = bit;
                }
            }
        }

        // Loop until no cell can be filled.
        if puzzle == bkup {
            break;
        }
    }

    let it = (0..BOARD_HEIGHT * BOARD_WIDTH)
        .map(|i| (i % BOARD_WIDTH, i / BOARD_WIDTH))
        .map(|(x, y)| (x, y, puzzle.map[y][x].count_ones() as BITS))
        .collect::<Vec<_>>();

    // If some cells have no possible number, there is no answer.
    if it.iter().any(|&(_x, _y, cnt)| cnt == 0) {
        return vec![];
    }

    // If all cells have exact one possible number, this is a answer.
    if it.iter().all(|&(_x, _y, cnt)| cnt == 1) {
        return vec![puzzle];
    }

    // Find the first undetermined cell.
    let (x, y, _cnt) = *it.iter()
        .filter(|&&(_x, _y, cnt)| cnt > 1)
        .min_by_key(|&&(_x, _y, cnt)| cnt)
        .unwrap();

    let mut answers = vec![];
    for n in 0..MAX_NUMBER {
        let bit = 1 << n;
        if puzzle.map[y][x] & bit == 0 {
            continue;
        }

        // Assuming the number at (x, y) is `n`, try to solve the problem again.
        // If some answers are found, append them to the `answers`.
        let mut p2 = puzzle;
        p2.map[y][x] = bit;
        answers.extend(solve_sudoku(p2).into_iter());
    }
    answers
}

const INPUT: &'static str = r"
    850002400
    720000009
    004000000
    000107002
    305000900
    040000000
    000080070
    017000000
    000036040
";

fn main() {
    let puzzle = INPUT.parse::<Sudoku>().unwrap();

    println!("{}", puzzle);

    for answer in &solve_sudoku(puzzle) {
        println!("{}", answer);
    }
}

#[cfg(test)]
const SOLUTION: &'static str = r"
    859612437
    723854169
    164379528
    986147352
    375268914
    241593786
    432981675
    617425893
    598736241
";

#[test]
fn solution() {
    let puzzle = INPUT.parse::<Sudoku>().unwrap();
    let answer = SOLUTION.parse::<Sudoku>().unwrap();
    let solution = solve_sudoku(puzzle);
    assert_eq!(solution, [answer]);
}
