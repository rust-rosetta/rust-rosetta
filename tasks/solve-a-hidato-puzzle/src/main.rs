use std::cmp::{max, min};

#[derive(Clone, Debug)]
struct Puzzle {
    board: Vec<Vec<i32>>,
    fixed: Vec<i32>,
    start: (usize, usize),
}

impl Puzzle {
    fn new(initial_board: Vec<Vec<i32>>) -> Self {
        let mut s: (usize, usize) = (0, 0);
        let mut f = initial_board
            .iter()
            .enumerate()
            .flat_map(|(y, r)| r.iter().enumerate().map(move |(x, c)| (y, x, *c)))
            .filter(|(_, _, c)| c > &0)
            .fold(Vec::new(), |mut fixed, (y, x, c)| {
                fixed.push(c);
                if c == 1 {
                    s = (y, x)
                };
                fixed
            });

        f.sort_unstable();

        Puzzle {
            board: initial_board,
            fixed: f,
            start: s,
        }
    }

    pub fn print_board(&self) {
        self.board.iter().for_each(|r| {
            println!("{}", {
                let mut row = String::default();

                r.iter().for_each(|c| match c {
                    c if c < &0 => row.push_str(format!("{:>2} ", " ").as_ref()),
                    c if c == &0 => row.push_str(format!("{:>2} ", ".").as_ref()),
                    _ => row.push_str(format!("{:>2} ", c).as_ref()),
                });
                row
            })
        });
    }

    fn solver(&mut self, current: &(usize, usize), n: &i32, mut next: i32) -> bool {
        if n > self.fixed.last().unwrap() {
            return true;
        }

        if self.board[current.0][current.1] != 0 && &self.board[current.0][current.1] != n {
            return false;
        }

        if self.board[current.0][current.1] == 0 && self.fixed[next as usize] == *n {
            return false;
        }

        let mut backup: i32 = 0;
        if self.board[current.0][current.1] == *n {
            backup = *n;
            next += 1;
        }

        self.board[current.0][current.1] = *n;

        for y in (max(current.0, 1) - 1)..=min(current.0 + 1, self.board.len() - 1) {
            for x in (max(current.1, 1) - 1)..=min(current.1 + 1, self.board[0].len() - 1) {
                if self.solver(&(y, x), &(n + 1), next) {
                    return true;
                }
            }
        }

        self.board[current.0][current.1] = backup;
        false
    }

    pub fn solve(&mut self) {
        let start = &self.start.clone();
        self.solver(start, &1, 0);
    }
}

fn main() {
    let input = vec![
        vec![0, 33, 35, 0, 0, -1, -1, -1],
        vec![0, 0, 24, 22, 0, -1, -1, -1],
        vec![0, 0, 0, 21, 0, 0, -1, -1],
        vec![0, 26, 0, 13, 40, 11, -1, -1],
        vec![27, 0, 0, 0, 9, 0, 1, -1],
        vec![-1, -1, 0, 0, 18, 0, 0, -1],
        vec![-1, -1, -1, -1, 0, 7, 0, 0],
        vec![-1, -1, -1, -1, -1, -1, 5, 0],
    ];

    let mut p = Puzzle::new(input);
    p.print_board();
    p.solve();
    println!("\nSolution:");
    p.print_board();
}
