#![feature(range_contains)]

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

const NEW_TREE_PROB: f32 = 0.01;
const INITIAL_TREE_PROB: f32 = 0.5;
const FIRE_PROB: f32 = 0.001;

const FOREST_WIDTH: usize = 60;
const FOREST_HEIGHT: usize = 30;

const SLEEP_MILLIS: u64 = 25;

use std::fmt;
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::Stdout;
use std::process::Command;
use std::time::Duration;
use rand::Rng;
use ansi_term::Colour::*;

use Tile::{Empty, Tree, Burning, Heating};

fn main() {
    let sleep_duration = Duration::from_millis(SLEEP_MILLIS);
    let mut forest = [[Tile::Empty; FOREST_WIDTH]; FOREST_HEIGHT];

    prepopulate_forest(&mut forest);
    print_forest(forest, 0);

    std::thread::sleep(sleep_duration);

    for generation in 1.. {

        for row in forest.iter_mut() {
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
    for row in forest.iter_mut() {
        for tile in row.iter_mut() {
            *tile = if prob_check(INITIAL_TREE_PROB) {
                Tree
            } else {
                Empty
            };
        }
    }
}

fn update_tile(tile: &mut Tile) {
    *tile = match *tile {
        Empty => {
            if prob_check(NEW_TREE_PROB) == true {
                Tree
            } else {
                Empty
            }
        }
        Tree => {
            if prob_check(FIRE_PROB) == true {
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
    let neighbors = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    for &(xoff, yoff) in neighbors.iter() {
        let nx: i32 = (x as i32) + xoff;
        let ny: i32 = (y as i32) + yoff;
        if (0..FOREST_WIDTH as i32).contains(nx) && (0..FOREST_HEIGHT as i32).contains(ny) &&
           forest[ny as usize][nx as usize] == Tree {
            forest[ny as usize][nx as usize] = Heating
        }
    }
}

fn prob_check(chance: f32) -> bool {
    let roll = rand::thread_rng().gen::<f32>();
    if chance - roll > 0.0 { true } else { false }
}

fn print_forest(forest: [[Tile; FOREST_WIDTH]; FOREST_HEIGHT], generation: u32) {
    let mut writer = BufWriter::new(io::stdout());
    clear_screen(&mut writer);
    writeln!(writer, "Generation: {}", generation + 1).unwrap();
    for row in forest.iter() {
        for tree in row.iter() {
            write!(writer, "{}", tree).unwrap();
        }
        writer.write(b"\n").unwrap();
    }
}

fn clear_screen(writer: &mut BufWriter<Stdout>) {
    let output = Command::new("clear").output().unwrap();
    write!(writer, "{}", String::from_utf8_lossy(&output.stdout)).unwrap();
}
