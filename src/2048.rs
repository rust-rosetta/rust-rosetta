// http://rosettacode.org/wiki/2048

//! Based on the C++ version: http://rosettacode.org/wiki/2048#C.2B.2B
//! Uses rustbox (termbox) to draw the board.

#[cfg(unix)]
mod mod2048 {
    extern crate rustbox;
    extern crate rand;

    use std::default::Default;
    use std::fmt;
    use self::rand::distributions::{IndependentSample, Range};
    use self::rustbox::{Color, RustBox};
    use self::rustbox::Key as RKey;

    #[derive(PartialEq, Clone)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    impl Direction {
        fn offset(self) -> i32 {
            match self {
                Direction::Up => -1,
                Direction::Down => 1,
                Direction::Left => -1,
                Direction::Right => 1,
            }
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Key {
        Right,
        Left,
        Up,
        Down,
        Char(char),
    }

    trait UI {
        fn wait_key(&self) -> Option<Key>;
        fn draw_grid(&self, grid: [[Tile; 4]; 4], rows: usize, cols: usize);
        fn present(&self);
        fn draw_lost(&self);
        fn draw_won(&self);
        fn draw_score(&self, text: String);
        fn draw_instructions(&self, text: String);
  }

    struct TermboxUI<'a> {
        rustbox: &'a RustBox,
    }

    impl<'a> UI for TermboxUI<'a> {
        fn wait_key(&self) -> Option<Key> {
            match self.rustbox.poll_event(false) {
                Ok(rustbox::Event::KeyEvent(key)) => {
                    match key {
                        RKey::Char('q') => Some(Key::Char('q')),
                        RKey::Up => Some(Key::Up),
                        RKey::Down => Some(Key::Down),
                        RKey::Left => Some(Key::Left),
                        RKey::Right => Some(Key::Right),
                        _ => None,
                    }
                }
                Err(e) => panic!("{}", e),
                _ => None,
            }
        }

        fn draw_grid(&self, grid: [[Tile; 4]; 4], rows: usize, cols: usize) {
            let x = 0;
            let y = 2;
            let width = 30;
            let height = 18;
            let cell_width = (width + 2 * rows) / cols;
            let cell_height = height / rows;

            for i in 0..rows {
                let x_coord = x + i * cell_width + i;

                for j in 0..cols {
                    let y_coord = y + j * cell_height + j;

                    let x_text_offset = (cell_width as f64 / 2 as f64).floor() as usize;
                    let y_text_offset = (cell_height as f64 / 2 as f64).floor() as usize;

                    let num: String = format!("{}", grid[i][j]);
                    let x_text_offset = x_text_offset - num.len() / 4;
                    self.draw_rectangle(x_coord,
                                        y_coord,
                                        cell_width,
                                        cell_height,
                                        Color::Black,
                                        Color::White,
                                        Color::Black);
                    if num != "0" {
                        self.rustbox.print(x_coord + x_text_offset,
                                           y_coord + y_text_offset,
                                           rustbox::RB_NORMAL,
                                           Color::White,
                                           Color::Black,
                                           &num);
                    }
                }
            }
        }

        fn present(&self) {
            self.rustbox.present();
        }

        fn draw_lost(&self) {
            self.draw_text(16, 12, "You lost!".to_string(), Color::Red, Color::Default);
        }

        fn draw_won(&self) {
            self.draw_text(16, 12, "You won!".to_string(), Color::Green, Color::Default);
        }

        fn draw_score(&self, text: String) {
            self.draw_text(16, 1, text, Color::White, Color::Default);
        }

        fn draw_instructions(&self, text: String) {
            self.draw_text(14, 22, text, Color::White, Color::Default);
        }
    }

    impl<'a> TermboxUI<'a> {
        fn new(rustbox: &'a rustbox::RustBox) -> TermboxUI<'a> {
            TermboxUI { rustbox: rustbox }
        }

        fn fill_area(&self, x: usize, y: usize, w: usize, h: usize, fg: Color, bg: Color) {
            for row in 0..h {
                for column in 0..w {
                    self.rustbox.print_char(x + column, y + row, rustbox::RB_NORMAL, fg, bg, ' ');
                }
            }
        }

        fn draw_horizontal_line(&self, x: usize, y: usize, w: usize, fg: Color, bg: Color) {
            for i in 0..w + 1 {
                self.rustbox.print_char(x + i, y, rustbox::RB_NORMAL, fg, bg, '─');
            }
        }

        fn draw_vertical_line(&self, x: usize, y: usize, h: usize, fg: Color, bg: Color) {
            for i in 0..h + 1 {
                self.rustbox.print_char(x, y + i, rustbox::RB_NORMAL, fg, bg, '│');
            }
        }

        fn draw_rectangle(&self,
                          x: usize,
                          y: usize,
                          w: usize,
                          h: usize,
                          fill: Color,
                          fg: Color,
                          bg: Color) {
            self.fill_area(x, y, w, h, fill, fill);
            self.draw_horizontal_line(x, y, w, fg, bg);    // top
            self.draw_horizontal_line(x, h + y, w, fg, bg);  // bottom
            self.draw_vertical_line(x, y, h, fg, bg);      // left
            self.draw_vertical_line(x + w, y, h, fg, bg);    // right
            self.rustbox.print_char(x, y, rustbox::RB_NORMAL, fg, bg, '┌');
            self.rustbox.print_char(x + w, y, rustbox::RB_NORMAL, fg, bg, '┐');
            self.rustbox.print_char(x, y + h, rustbox::RB_NORMAL, fg, bg, '└');
            self.rustbox.print_char(x + w, y + h, rustbox::RB_NORMAL, fg, bg, '┘');
        }

        fn draw_text(&self,
                     x: usize,
                     y: usize,
                     line: String,
                     fg: Color,
                     bg: Color)
                     -> (usize, usize) {
            for (i, ch) in line.chars().enumerate() {
                self.rustbox.print_char(x + i, y, rustbox::RB_NORMAL, fg, bg, ch);
            }
            (x + line.len(), y)
        }
    }

    #[derive(Copy, Clone)]
    struct Tile {
        _value: usize,
        _blocked: bool,
    }

    impl Tile {
        fn new() -> Tile {
            Tile {
                _value: 0,
                _blocked: false,
            }
        }

        fn set(&mut self, val: usize) {
            self._value = val;
        }

        fn get(&self) -> usize {
            self._value
        }

        fn is_empty(&self) -> bool {
            self._value == 0
        }

        fn blocked(&mut self, b: bool) {
            self._blocked = b;
        }

        fn is_blocked(&self) -> bool {
            return self._blocked;
        }
    }

    impl fmt::Display for Tile {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self._value)
        }
    }

    impl PartialEq for Tile {
        fn eq(&self, other: &Tile) -> bool {
            self._value == other._value
        }

        fn ne(&self, other: &Tile) -> bool {
            self._value != other._value
        }
    }

    #[derive(PartialEq, Debug)]
    enum State {
        Playing,
        Won,
        Lost,
    }

    struct Game<'a> {
        ui: &'a UI,
        grid: [[Tile; 4]; 4],
        state: State,
        score: usize,
        moved: bool,
    }

    impl<'a> Game<'a> {
        fn new(ui: &'a UI) -> Game<'a> {
            let mut g = Game {
                ui: ui,
                grid: [[Tile::new(); 4]; 4],
                state: State::Playing,
                score: 0,
                moved: false,
            };
            for _ in 0..2 {
                g.add_tile();
            }

            g
        }

        fn run(&mut self) {
            loop {
                self.draw();

                self.moved = false;

                let key = self.ui.wait_key();
                if key == Some(Key::Char('q')) {
                    break;
                }

                if self.state != State::Lost && self.state != State::Won {
                    match key {
                        Some(Key::Up) => {
                            self.move_up();
                        }
                        Some(Key::Down) => {
                            self.move_down();
                        }
                        Some(Key::Left) => {
                            self.move_left();
                        }
                        Some(Key::Right) => {
                            self.move_right();
                        }
                        _ => {}
                    }
                }

                for i in 0..4 {
                    for j in 0..4 {
                        self.grid[i][j].blocked(false);
                    }
                }

                if self.moved {
                    self.add_tile();
                } else if !self.can_move() {
                    self.state = State::Lost;
                }
            }
        }

        fn add_tile(&mut self) {
            let mut cantadd = true;
            'OUTER: for i in 0..4 {
                for j in 0..4 {
                    if self.grid[i][j].is_empty() {
                        cantadd = false;
                        break 'OUTER;
                    }
                }
            }

            let cantmove = !self.can_move();
            if cantadd || cantmove {
                return;
            }

            let between = Range::new(0f64, 1.);
            let mut rng = rand::thread_rng();
            let a = between.ind_sample(&mut rng);

            let mut cell1 = rand::random::<(usize, usize)>();
            while !self.grid[cell1.0 % 4][cell1.1 % 4].is_empty() {
                cell1 = rand::random::<(usize, usize)>();
            }
            self.grid[cell1.0 % 4][cell1.1 % 4].set(if a > 0.9 {
                4
            } else {
                2
            });
        }

        fn can_move(&self) -> bool {
            for i in 0..4 {
                for j in 0..4 {
                    if self.grid[i][j].is_empty() {
                        return true;
                    }

                    if self.test_add(i + 1, j, self.grid[i][j]) {
                        return true;
                    };
                    if i > 0 && self.test_add(i - 1, j, self.grid[i][j]) {
                        return true;
                    };
                    if self.test_add(i, j + 1, self.grid[i][j]) {
                        return true;
                    };
                    if j > 0 && self.test_add(i, j - 1, self.grid[i][j]) {
                        return true;
                    };
                }
            }

            return false;
        }

        fn test_add(&self, x: usize, y: usize, v: Tile) -> bool {
            if x > 3 || y > 3 {
                return false;
            }
            return self.grid[x][y] == v;
        }

        fn add_score(&mut self, score: usize) {
            self.score += score;

            if score == 2048 {
                self.state = State::Won;
            }
        }

        fn draw(&self) {
            self.ui.draw_score(format!("Score: {}", self.score));
            self.ui.draw_grid(self.grid, 4, 4);
            self.ui.draw_instructions("←,↑,→,↓ or q".to_string());

            if self.state == State::Lost {
                self.ui.draw_lost();
            } else if self.state == State::Won {
                self.ui.draw_won();
            }

            self.ui.present();
        }

        fn move_direction(&mut self, x: usize, y: usize, d: Direction) {
            let o = d.clone().offset();

            if d == Direction::Up || d == Direction::Down {
                if y as i32 + o < 0 || y as i32 + o > 3 {
                    return;
                }

                let yo: usize = (y as i32 + o) as usize;

                if !self.grid[x][yo].is_empty() && self.grid[x][yo] == self.grid[x][y] &&
                   !self.grid[x][y].is_blocked() &&
                   !self.grid[x][yo].is_blocked() {
                    self.grid[x][y].set(0);
                    let val = self.grid[x][yo].get();
                    self.grid[x][yo].set(val * 2);
                    self.add_score(val * 2);
                    self.grid[x][yo].blocked(true);
                    self.moved = true;
                } else if self.grid[x][yo].is_empty() && !self.grid[x][y].is_empty() {
                    let val = self.grid[x][y].get();
                    self.grid[x][yo].set(val);
                    self.grid[x][y].set(0);
                    self.moved = true;
                }

                self.move_direction(x, yo, d);
            } else if d == Direction::Left || d == Direction::Right {
                if x as i32 + o < 0 || x as i32 + o > 3 {
                    return;
                }

                let xo: usize = (x as i32 + o) as usize;

                if !self.grid[xo][y].is_empty() && self.grid[xo][y] == self.grid[x][y] &&
                   !self.grid[x][y].is_blocked() &&
                   !self.grid[xo][y].is_blocked() {
                    self.grid[x][y].set(0);
                    let val = self.grid[xo][y].get();
                    self.grid[xo][y].set(val * 2);
                    self.add_score(val * 2);
                    self.grid[xo][y].blocked(true);
                    self.moved = true;
                } else if self.grid[xo][y].is_empty() && !self.grid[x][y].is_empty() {
                    let val = self.grid[x][y].get();
                    self.grid[xo][y].set(val);
                    self.grid[x][y].set(0);
                    self.moved = true;
                }

                self.move_direction(xo, y, d);
            }
        }

        fn move_up(&mut self) {
            for i in 0..4 {
                for j in 1..4 {
                    if !self.grid[i][j].is_empty() {
                        self.move_direction(i, j, Direction::Up);
                    }
                }
            }
        }

        fn move_down(&mut self) {
            for i in 0..4 {
                for j in (0..3).rev() {
                    if !self.grid[i][j].is_empty() {
                        self.move_direction(i, j, Direction::Down);
                    }
                }
            }
        }

        fn move_left(&mut self) {
            for j in 0..4 {
                for i in 1..4 {
                    if !self.grid[i][j].is_empty() {
                        self.move_direction(i, j, Direction::Left);
                    }
                }
            }
        }

        fn move_right(&mut self) {
            for j in 0..4 {
                for i in (0..3).rev() {
                    if !self.grid[i][j].is_empty() {
                        self.move_direction(i, j, Direction::Right);
                    }
                }
            }
        }
    }

    pub fn main() {
        let rustbox = match RustBox::init(Default::default()) {
            Result::Ok(v) => v,
            Result::Err(e) => panic!("{}", e),
        };

        let ui = TermboxUI::new(&rustbox);
        let mut game = Game::new(&ui);
        game.run();
    }
}

#[cfg(unix)]
fn main() {
    mod2048::main();
}

// main to make the compiler happy when 2048 feature is disabled.
#[cfg(not(unix))]
fn main() {
    println!("This solution is not supported on Windows.");
}
