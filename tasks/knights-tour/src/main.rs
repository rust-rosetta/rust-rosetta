use std::fmt;

const SIZE: usize = 8;
const MOVES: [(i32, i32); 8] = [(2, 1), (1, 2), (-1, 2), (-2, 1), (-2, -1), (-1, -2), (1, -2),
                                (2, -1)];

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn mov(&self, &(dx, dy): &(i32, i32)) -> Point {
        Point {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

struct Board {
    field: [[i32; SIZE]; SIZE],
}

impl Board {
    fn new() -> Board {
        Board { field: [[0; SIZE]; SIZE] }
    }

    fn available(&self, p: Point) -> bool {
        let valid = 0 <= p.x && p.x < SIZE as i32 && 0 <= p.y && p.y < SIZE as i32;

        valid && self.field[p.x as usize][p.y as usize] == 0
    }

    /// calculate the number of possible moves
    fn count_degree(&self, p: Point) -> i32 {
        let mut count = 0;
        for dir in &MOVES {
            let next = p.mov(dir);
            if self.available(next) {
                count += 1;
            }
        }
        count
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.field {
            for x in row.iter() {
                try!(write!(f, "{:3} ", x));
            }
            try!(write!(f, "\n"));
        }
        Ok(())
    }
}

fn knights_tour(x: i32, y: i32) -> Option<Board> {
    let mut board = Board::new();
    let mut p = Point { x: x, y: y };
    let mut step = 1;

    board.field[p.x as usize][p.y as usize] = step;
    step += 1;

    while step <= (SIZE * SIZE) as i32 {
        // choose next square by Warnsdorf's rule
        let mut candidates = vec![];

        for dir in &MOVES {
            let adj = p.mov(dir);
            if board.available(adj) {
                let degree = board.count_degree(adj);
                candidates.push((degree, adj));
            }
        }

        match candidates.iter().min() {
            Some(&(_, adj)) => p = adj,
            None => return None,
        };

        board.field[p.x as usize][p.y as usize] = step;
        step += 1;
    }

    Some(board)
}

fn main() {
    let (x, y) = (3, 1);

    println!("Board size: {}", SIZE);
    println!("Starting position: ({}, {})", x, y);

    match knights_tour(x, y) {
        Some(b) => print!("{}", b),
        None => println!("Fail!"),
    }

}

#[cfg(test)]
mod tests {
    use super::{SIZE, knights_tour};

    const ANSWER: [[i32; SIZE]; SIZE] = [[23, 20, 3, 32, 25, 10, 5, 8],
                                         [2, 33, 24, 21, 4, 7, 26, 11],
                                         [19, 22, 51, 34, 31, 28, 9, 6],
                                         [50, 1, 40, 29, 54, 35, 12, 27],
                                         [41, 18, 55, 52, 61, 30, 57, 36],
                                         [46, 49, 44, 39, 56, 53, 62, 13],
                                         [17, 42, 47, 60, 15, 64, 37, 58],
                                         [48, 45, 16, 43, 38, 59, 14, 63]];

    #[test]
    fn test() {
        let (x, y) = (3, 1);
        match knights_tour(x, y) {
            Some(b) => assert_eq!(b.field, ANSWER),
            None => panic!(),
        }
    }
}
