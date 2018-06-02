extern crate ansi_term;
extern crate rand;

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Empty,
    Tree,
    Burning,
    Heating,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match *self {
            Empty => Black.paint(" "),
            Tree => Green.bold().paint("T"),
            Burning => Red.bold().paint("B"),
            Heating => Yellow.bold().paint("T"),
        };
        write!(f, "{}", output)
    }
}

const NEW_TREE_PROB: f64 = 0.01;
const INITIAL_TREE_PROB: f64 = 0.5;
const FIRE_PROB: f64 = 0.001;

const FOREST_WIDTH: usize = 60;
const FOREST_HEIGHT: usize = 30;

const SLEEP_MILLIS: u64 = 25;

use std::fmt;
use std::io::prelude::*;
use std::io::{stdout, BufWriter, StdoutLock};
use std::process::Command;
use std::time::Duration;

use ansi_term::Colour::*;
use rand::{thread_rng, Rng};

use Tile::{Burning, Empty, Heating, Tree};

fn main() {
    let sleep_duration = Duration::from_millis(SLEEP_MILLIS);
    let mut forest = [[Tile::Empty; FOREST_WIDTH]; FOREST_HEIGHT];

    prepopulate_forest(&mut forest);
    print_forest(forest, 0);

    std::thread::sleep(sleep_duration);

    for generation in 1.. {
        for row in &mut forest {
            for tile in row.iter_mut() {
                update_tile(tile);
            }
        }

        for y in 0..FOREST_HEIGHT {
            for x in 0..FOREST_WIDTH {
                if forest[y][x] == Burning {
                    heat_neighbors(&mut forest, y, x);
                }
            }
        }

        print_forest(forest, generation);

        std::thread::sleep(sleep_duration);
    }
}

fn prepopulate_forest(forest: &mut [[Tile; FOREST_WIDTH]; FOREST_HEIGHT]) {
    let mut rng = thread_rng();

    for row in forest.iter_mut() {
        for tile in row.iter_mut() {
            *tile = if rng.gen_bool(INITIAL_TREE_PROB) {
                Tree
            } else {
                Empty
            };
        }
    }
}

fn update_tile(tile: &mut Tile) {
    let mut rng = thread_rng();
    *tile = match *tile {
        Empty => {
            if rng.gen_bool(NEW_TREE_PROB) {
                Tree
            } else {
                Empty
            }
        }
        Tree => {
            if rng.gen_bool(FIRE_PROB) {
                Burning
            } else {
                Tree
            }
        }
        Burning => Empty,
        Heating => Burning,
    }
}

fn heat_neighbors(forest: &mut [[Tile; FOREST_WIDTH]; FOREST_HEIGHT], y: usize, x: usize) {
    let neighbors = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (nx, ny) in neighbors
        .iter()
        .map(|&(x_off, y_off)| (x as i8 + x_off, (y as i8 + y_off)))
        .filter(|&(nx, ny)| nx > 0 && ny > 0)
        .map(|(nx, ny)| (nx as usize, (ny as usize)))
    {
        if let Some(tile) = forest.get_mut(ny).and_then(|r| r.get_mut(nx)) {
            if *tile == Tree {
                *tile = Heating;
            }
        }
    }
}

fn print_forest(forest: [[Tile; FOREST_WIDTH]; FOREST_HEIGHT], generation: usize) {
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    clear_screen(&mut writer);
    writeln!(writer, "Generation: {}", generation + 1).unwrap();
    for row in &forest {
        for tree in row.iter() {
            write!(writer, "{}", tree).unwrap();
        }
        writeln!(writer).unwrap();
    }
}

fn clear_screen(writer: &mut BufWriter<StdoutLock>) {
    let output = Command::new("clear").output().unwrap();
    write!(writer, "{}", String::from_utf8_lossy(&output.stdout)).unwrap();
}
