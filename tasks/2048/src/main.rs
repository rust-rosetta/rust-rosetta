extern crate rand;

use std::io;
use std::ops::{Index, IndexMut};

use rand::prelude::*;

const GRID_DIMENSION: usize = 4;

/// A key press.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Key {
    Up,
    Left,
    Down,
    Right,
    Quit,
}

#[derive(Debug, PartialEq)]
enum State {
    Playing,
    Won,
    Lost,
}

/// Error returned when the grid is full.
#[derive(Debug)]
struct GridFullError;

/// Error returned when no tiles were actually moved.
#[derive(Debug)]
struct NoTilesMovedError;

/// An optionally filled tile in the grid.
type Tile = Option<u32>;

/// The set of game tiles.
#[derive(Debug, PartialEq, Eq)]
struct Grid([[Tile; GRID_DIMENSION]; GRID_DIMENSION]);

impl Index<(usize, usize)> for Grid {
    type Output = Tile;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.0[x][y]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.0[x][y]
    }
}

impl Grid {
    fn new() -> Self {
        Grid([[None; GRID_DIMENSION]; GRID_DIMENSION])
    }

    fn is_move_available(&self) -> bool {
        for x in 0..4 {
            for y in 0..4 {
                if self[(x, y)].is_none() {
                    return true;
                }
                if x < 3 && self[(x + 1, y)] == self[(x, y)] {
                    return true;
                };
                if x > 0 && self[(x - 1, y)] == self[(x, y)] {
                    return true;
                };
                if y < 3 && self[(x, y + 1)] == self[(x, y)] {
                    return true;
                };
                if y > 0 && self[(x, y - 1)] == self[(x, y)] {
                    return true;
                };
            }
        }
        false
    }

    fn spawn_tile(&mut self) -> Result<(), GridFullError> {
        if !self.is_move_available() {
            return Err(GridFullError);
        }

        let mut rng = SmallRng::from_rng(&mut thread_rng()).unwrap();

        loop {
            // `GRID_DIMENSION` is a power of two so modulo reduction is
            // unbiased. Because we only need 2 bits, we can just generate a
            // `u8` and split the bits. This is faster than generating two
            // `usize` values.
            let r = rng.gen::<u8>() as usize;
            let rand_tile = &mut self[(r % GRID_DIMENSION, r >> (8 - 2))];

            if rand_tile.is_none() {
                let tile = if rng.gen_bool(1.0 / 10.0) { 4 } else { 2 };
                *rand_tile = Some(tile);
                break;
            }
        }

        Ok(())
    }

    /// Given a slice of tiles, shifts and merges the tiles towards the left end of the vector.
    ///
    /// Returns a tuple containing the new vector and the change in score.
    fn merge(vector: &[Tile]) -> (Vec<Tile>, u32) {
        // Remove intermediate empty tiles.
        let mut shifted = vector.iter().filter_map(|&x| x).collect::<Vec<_>>();

        // Merge tiles that are next to each other, leaving an empty space.
        let mut score = 0;
        for i in 0..shifted.len() {
            if i + 1 < shifted.len() && shifted[i + 1] == shifted[i] {
                shifted[i] *= 2;
                shifted[i + 1] = 0;
                score += shifted[i];
            }
        }

        // Remove intermediate empty tiles
        let mut shifted = shifted
            .into_iter()
            .filter(|&x| x != 0)
            .map(Some)
            .collect::<Vec<_>>();

        // Fill remaining array.
        let len = shifted.len();
        shifted.extend(std::iter::repeat(None).take(GRID_DIMENSION - len));

        (shifted, score)
    }

    /// Shift tiles left.
    fn shift_left(&mut self) -> Result<u32, NoTilesMovedError> {
        let mut moved = false;
        let mut score = 0;
        for row in 0..GRID_DIMENSION {
            let tiles: Vec<_> = (0..GRID_DIMENSION).map(|col| self[(row, col)]).collect();

            let (merged_row, row_score) = Grid::merge(&tiles);
            score += row_score;
            if !moved && tiles != merged_row {
                moved = true;
            }

            for (col, tile) in merged_row.iter().enumerate() {
                self[(row, col)] = *tile;
            }
        }

        if moved {
            Ok(score)
        } else {
            Err(NoTilesMovedError)
        }
    }

    /// Shift tiles up.
    fn shift_up(&mut self) -> Result<u32, NoTilesMovedError> {
        let mut moved = false;
        let mut score = 0;
        for col in 0..GRID_DIMENSION {
            let mut tiles = vec![];

            for row in 0..GRID_DIMENSION {
                tiles.push(self[(row, col)]);
            }

            let (merged_col, col_score) = Grid::merge(&tiles);
            score += col_score;
            if !moved && tiles != merged_col {
                moved = true;
            }

            for (row, tile) in merged_col.iter().enumerate() {
                self[(row, col)] = *tile;
            }
        }

        if moved {
            Ok(score)
        } else {
            Err(NoTilesMovedError)
        }
    }

    /// Shift tiles down.
    fn shift_down(&mut self) -> Result<u32, NoTilesMovedError> {
        let mut moved = false;
        let mut score = 0;
        for col in 0..GRID_DIMENSION {
            let mut tiles = vec![];

            for row in (0..GRID_DIMENSION).rev() {
                tiles.push(self[(row, col)]);
            }

            let (merged_col, col_score) = Grid::merge(&tiles);
            score += col_score;
            if !moved && tiles != merged_col {
                moved = true;
            }

            for (row, tile) in merged_col.iter().rev().enumerate() {
                self[(row, col)] = *tile;
            }
        }

        if moved {
            Ok(score)
        } else {
            Err(NoTilesMovedError)
        }
    }

    /// Shift tiles down.
    fn shift_right(&mut self) -> Result<u32, NoTilesMovedError> {
        let mut moved = false;
        let mut score = 0;
        for row in 0..GRID_DIMENSION {
            let mut tiles = vec![];

            for col in (0..GRID_DIMENSION).rev() {
                tiles.push(self[(row, col)]);
            }

            let (merged_row, row_score) = Grid::merge(&tiles);
            score += row_score;
            if !moved && tiles != merged_row {
                moved = true;
            }

            for (col, tile) in merged_row.iter().rev().enumerate() {
                self[(row, col)] = *tile;
            }
        }

        if moved {
            Ok(score)
        } else {
            Err(NoTilesMovedError)
        }
    }
}

trait Ui {
    /// Wait for a key press, and report the key.
    fn wait_key(&mut self) -> Key;

    /// Draw the game.
    fn draw(&mut self, game: &Game) -> io::Result<()>;
}

struct Game {
    grid: Grid,
    state: State,
    score: u32,
}

impl Game {
    fn new() -> Self {
        let mut g = Game {
            grid: Grid::new(),
            state: State::Playing,
            score: 0,
        };

        for _ in 0..2 {
            g.grid.spawn_tile().unwrap();
        }

        g
    }

    fn step(&mut self, key: Key) {
        match key {
            key if self.state != State::Lost => {
                let move_result = match key {
                    Key::Up => self.grid.shift_up(),
                    Key::Down => self.grid.shift_down(),
                    Key::Left => self.grid.shift_left(),
                    Key::Right => self.grid.shift_right(),
                    _ => unreachable!(),
                };

                if let Ok(score) = move_result {
                    self.add_score(score);
                    self.grid.spawn_tile().unwrap();
                }

                if !self.grid.is_move_available() {
                    self.state = State::Lost;
                    return;
                }
            }
            _ => (),
        }
    }

    fn add_score(&mut self, score: u32) {
        self.score += score;
        if score == 2048 {
            self.state = State::Won;
        }
    }
}

#[cfg(unix)]
mod ui_imp {
    extern crate termion;

    use super::{Game, Key, State, Ui};

    use std::io::prelude::*;
    use std::io::{self, Stdin, Stdout};

    use self::termion::event::Key as TermKey;
    use self::termion::input::Keys;
    use self::termion::raw::{IntoRawMode, RawTerminal};
    use self::termion::{color, cursor, style};

    pub struct Termion {
        keys: Keys<Stdin>,
        stdout: RawTerminal<Stdout>,
    }

    impl Termion {
        pub fn new() -> Self {
            use std::io;
            use ui_imp::termion::input::TermRead;

            let mut stdout = io::stdout().into_raw_mode().unwrap();
            write!(stdout, "{}", termion::clear::All).unwrap();

            Termion {
                keys: io::stdin().keys(),
                stdout,
            }
        }
    }

    impl Ui for Termion {
        fn wait_key(&mut self) -> Key {
            while let Some(key) = self.keys.next() {
                let key = match key.unwrap() {
                    TermKey::Char('q') => Key::Quit,
                    TermKey::Up => Key::Up,
                    TermKey::Down => Key::Down,
                    TermKey::Left => Key::Left,
                    TermKey::Right => Key::Right,
                    _ => continue,
                };

                return key;
            }

            Key::Quit
        }

        fn draw(&mut self, game: &Game) -> io::Result<()> {
            write!(self.stdout, "{}{}", cursor::Hide, termion::clear::All)?;
            write!(self.stdout, "{}Score: {}", cursor::Goto(16, 1), game.score)?;

            const CELL_WIDTH: usize = 10;
            const CELL_HEIGHT: usize = 5;
            const GRID_X_OFFSET: usize = 0;
            const GRID_Y_OFFSET: usize = 2;

            for row in 0..4 {
                for col in 0..4 {
                    let tile = game.grid[(row, col)];
                    let x = 1 + GRID_X_OFFSET + col * CELL_WIDTH;
                    let y = GRID_Y_OFFSET + row * CELL_HEIGHT;

                    write!(
                        self.stdout,
                        "{}┌────────┐",
                        cursor::Goto(x as u16, y as u16)
                    )?;

                    for i in 1..=3 {
                        let y = y + i;
                        write!(
                            self.stdout,
                            "{}│        │",
                            cursor::Goto(x as u16, y as u16)
                        )?;
                    }

                    if let Some(value) = tile {
                        let text_x = x + CELL_WIDTH / 2 - (1 + value.to_string().len() / 3);
                        let text_y = y + CELL_HEIGHT / 2;

                        write!(
                            self.stdout,
                            "{}{}{}{}{}{}",
                            cursor::Goto(text_x as u16, text_y as u16),
                            style::Bold,
                            color::Fg(color::LightWhite),
                            value,
                            color::Fg(color::Reset),
                            style::Reset
                        )?;
                    }

                    write!(
                        self.stdout,
                        "{}└────────┘",
                        cursor::Goto(x as u16, y as u16 + 4)
                    )?;
                }
            }

            match game.state {
                State::Won => write!(self.stdout, "{}You won!", cursor::Goto(16, 12))?,
                State::Lost => write!(self.stdout, "{}You lost!", cursor::Goto(16, 12))?,
                _ => (),
            }

            write!(self.stdout, "{}←,↑,→,↓ or q", cursor::Goto(14, 22))?;

            self.stdout.flush()?;
            Ok(())
        }
    }
}

#[cfg(unix)]
fn main() {
    use ui_imp::Termion;

    let mut game = Game::new();
    let mut ui = Termion::new();

    loop {
        ui.draw(&game).unwrap();
        let key = ui.wait_key();
        if let Key::Quit = key {
            break;
        }
        game.step(key);
    }
}

#[cfg(not(unix))]
fn main() {
    println!("This solution is not supported on Windows.");
}

#[cfg(test)]
mod tests {
    use super::{Grid, GRID_DIMENSION};

    #[test]
    fn merge() {
        let test_cases = [
            ([None, None, None, None], [None, None, None, None]),
            ([None, None, None, Some(2)], [Some(2), None, None, None]),
            ([None, None, Some(2), Some(2)], [Some(4), None, None, None]),
            ([None, Some(2), None, Some(2)], [Some(4), None, None, None]),
            ([Some(2), None, None, Some(2)], [Some(4), None, None, None]),
            ([Some(2), None, Some(2), None], [Some(4), None, None, None]),
            (
                [Some(2), Some(2), Some(2), None],
                [Some(4), Some(2), None, None],
            ),
            (
                [Some(2), None, Some(2), Some(2)],
                [Some(4), Some(2), None, None],
            ),
            (
                [Some(2), Some(2), None, Some(2)],
                [Some(4), Some(2), None, None],
            ),
            (
                [Some(2), Some(2), Some(2), Some(2)],
                [Some(4), Some(4), None, None],
            ),
            (
                [Some(4), Some(4), Some(2), Some(2)],
                [Some(8), Some(4), None, None],
            ),
            (
                [Some(2), Some(2), Some(4), Some(4)],
                [Some(4), Some(8), None, None],
            ),
            (
                [Some(8), None, Some(2), Some(2)],
                [Some(8), Some(4), None, None],
            ),
            (
                [Some(4), None, Some(2), Some(2)],
                [Some(4), Some(4), None, None],
            ),
        ];

        for &(candidate, expected) in &test_cases {
            let (merged, _) = Grid::merge(&candidate);
            assert_eq!(&merged, &expected);
        }
    }

    #[test]
    fn directions() {
        let mut grid = Grid::new();
        grid[(0, 0)] = Some(2);

        grid.shift_right().unwrap();
        assert_eq!(grid[(0, GRID_DIMENSION - 1)], Some(2));
        println!("{:?}", grid);

        grid.shift_down().unwrap();
        assert_eq!(grid[(GRID_DIMENSION - 1, GRID_DIMENSION - 1)], Some(2));
        println!("{:?}", grid);

        grid.shift_left().unwrap();
        println!("{:?}", grid);
        assert_eq!(grid[(GRID_DIMENSION - 1, 0)], Some(2));

        grid.shift_up().unwrap();
        assert_eq!(grid[(0, 0)], Some(2));
    }

    #[test]
    fn non_greedy_movement() {
        let mut grid = Grid::new();
        for i in 0..4 {
            grid[(0, i)] = Some(2);
        }

        grid.shift_right().unwrap();

        let mut expected_grid = Grid::new();
        expected_grid[(0, 2)] = Some(4);
        expected_grid[(0, 3)] = Some(4);
        assert_eq!(grid, expected_grid);
    }

    #[test]
    fn move_direction_priority() {
        let mut grid = Grid::new();
        for i in 1..4 {
            grid[(0, i)] = Some(2);
        }

        grid.shift_right().unwrap();

        let mut expected_grid = Grid::new();
        expected_grid[(0, 2)] = Some(2);
        expected_grid[(0, 3)] = Some(4);
        assert_eq!(grid, expected_grid);
    }
}
