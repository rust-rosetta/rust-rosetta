use std::num::Wrapping;

struct Ant {
    x: Wrapping<usize>,
    y: Wrapping<usize>,
    dir: Direction,
}

#[derive(Clone,Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

use Direction::*;

impl Ant {
    fn mv(&mut self, vec: &mut Vec<Vec<u8>>) {
        let pointer = &mut vec[self.y.0][self.x.0];
        // change direction
        match *pointer {
            0 => self.dir = self.dir.right(),
            1 => self.dir = self.dir.left(),
            _ => panic!("Unexpected colour in grid"),
        }
        // flip colour
        // if it's 1 it's black
        // if it's 0 it's white
        *pointer ^= 1;

        // move direction
        match self.dir {
            North => self.y -= Wrapping(1),
            South => self.y += Wrapping(1),
            East => self.x += Wrapping(1),
            West => self.x -= Wrapping(1),
        }

    }
}

impl Direction {
    fn right(self) -> Direction {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn left(self) -> Direction {
        // 3 rights equal a left
        self.right().right().right()
    }
}

fn main() {
    // create a 100x100 grid using vectors
    let mut grid: Vec<Vec<u8>> = vec![vec![0; 100]; 100];
    let mut ant = Ant {
        x: Wrapping(50),
        y: Wrapping(50),
        dir: Direction::North,
    };

    while ant.x < Wrapping(100) && ant.y < Wrapping(100) {
        ant.mv(&mut grid);
    }
    for each in &grid {
        // construct string
        // using iterator methods to quickly convert the vector
        // to a string
        let string = each.iter()
            .map(|&x| {
                if x == 0 {
                    " "
                } else {
                    "#"
                }
            })
            .fold(String::new(), |x, y| x + y);
        println!("{}", string);
    }
}
