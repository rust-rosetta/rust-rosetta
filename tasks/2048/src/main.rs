//! Uses rustbox (termbox) to draw the board.

#[cfg(unix)]
mod mod2048 {
    extern crate rustbox;
    extern crate rand;

    use std::default::Default;
    use self::rand::distributions::{IndependentSample, Range};
    use self::rustbox::{Color, RustBox};
    use self::rustbox::Key as RKey;

    struct Rectangle {
        x: usize,
        y: usize,
        w: usize,
        h: usize,
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
        fn draw_grid(&self, grid: [[usize; 4]; 4]);
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

        #[cfg_attr(feature = "clippy", allow(needless_range_loop))]
        fn draw_grid(&self, grid: [[usize; 4]; 4]) {
            let x = 0;
            let y = 2;
            let width = 36;
            let height = 18;
            let cell_width: usize = width / grid.len();
            let cell_height: usize = height / grid[0].len();
            for i in 0..grid.len() {
                let x_coord = x + i * cell_width + i;
                for j in 0..grid[0].len() {
                    let y_coord = y + j * cell_height + j;
                    let x_text_offset = cell_width / 2;
                    let y_text_offset = cell_height / 2;
                    let num: String = format!("{}", grid[i][j]);
                    let x_text_offset = x_text_offset - num.len() / 4;

                    let rect = Rectangle {
                        x: x_coord,
                        y: y_coord,
                        w: cell_width,
                        h: cell_height,
                    };
                    self.draw_rectangle(rect, Color::Black, Color::White, Color::Black);
                    if num != "0" {
                        self.rustbox.print(x_coord + x_text_offset,
                                           y_coord + y_text_offset,
                                           rustbox::RB_BOLD,
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

        fn draw_rectangle(&self, rect: Rectangle, fill: Color, fg: Color, bg: Color) {
            let Rectangle { x, y, w, h } = rect;

            self.fill_area(x, y, w, h, fill, fill);
            self.draw_horizontal_line(x, y, w, fg, bg); // top
            self.draw_horizontal_line(x, h + y, w, fg, bg); // bottom
            self.draw_vertical_line(x, y, h, fg, bg); // left
            self.draw_vertical_line(x + w, y, h, fg, bg); // right
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

    fn spawn_new(x: &mut usize) -> bool {
        if *x != 0 {
            return false;
        }

        let between = Range::new(0f64, 1.);
        let mut rng = rand::thread_rng();
        let a = between.ind_sample(&mut rng);
        *x = if a > 0.9 {
            4
        } else {
            2
        };
        true
    }

    #[derive(PartialEq, Debug)]
    enum State {
        Playing,
        Won,
        Lost,
    }

    struct Game<'a> {
        ui: &'a UI,
        grid: [[usize; 4]; 4],
        state: State,
        total_score: usize,
        moved: bool,
    }

    impl<'a> Game<'a> {
        fn new(ui: &'a UI) -> Game<'a> {
            let mut g = Game {
                ui: ui,
                grid: [[0; 4]; 4],
                state: State::Playing,
                total_score: 0,
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
                        Some(Key::Up) => self.move_up(),
                        Some(Key::Down) => self.move_down(),
                        Some(Key::Left) => self.move_left(),
                        Some(Key::Right) => self.move_right(),
                        _ => {}
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
            if self.can_move() {
                let mut cell1 = rand::random::<(usize, usize)>();
                while !spawn_new(&mut self.grid[cell1.0 % 4][cell1.1 % 4]) {
                    cell1 = rand::random::<(usize, usize)>();
                }
            }
        }

        fn can_move(&self) -> bool {
            for x in 0..4 {
                for y in 0..4 {
                    if self.grid[x][y] == 0 {
                        return true;
                    }
                    if x < 3 && self.grid[x + 1][y] == self.grid[x][y] {
                        return true;
                    };
                    if x > 0 && self.grid[x - 1][y] == self.grid[x][y] {
                        return true;
                    };
                    if y < 3 && self.grid[x][y + 1] == self.grid[x][y] {
                        return true;
                    };
                    if y > 0 && self.grid[x][y - 1] == self.grid[x][y] {
                        return true;
                    };
                }
            }
            false
        }

        fn add_score(&mut self, score: usize) {
            self.total_score += score;
            if score == 2048 {
                self.state = State::Won;
            }
        }

        fn draw(&self) {
            self.ui.draw_score(format!("Score: {}", self.total_score));
            self.ui.draw_grid(self.grid);
            self.ui.draw_instructions("←,↑,→,↓ or q".to_string());
            if self.state == State::Lost {
                self.ui.draw_lost();
            } else if self.state == State::Won {
                self.ui.draw_won();
            }
            self.ui.present();
        }

        fn combine_tile(&mut self, col: usize, row: usize, testcol: usize, testrow: usize) -> bool {
            if self.grid[testcol][testrow] == 0 {
                false
            } else if self.grid[col][row] == 0 {
                self.grid[col][row] += self.grid[testcol][testrow];
                self.grid[testcol][testrow] = 0;
                self.moved = true;
                false
            } else if self.grid[col][row] == self.grid[testcol][testrow] {
                self.grid[col][row] += self.grid[testcol][testrow];
                self.grid[testcol][testrow] = 0;
                let score = self.grid[col][row];
                self.add_score(score);
                self.moved = true;
                true
            } else {
                true
            }
        }

        fn move_up(&mut self) {
            for col in 0..4 {
                for row in 0..4 {
                    for testrow in (row + 1)..4 {
                        if self.combine_tile(col, row, col, testrow) {
                            break;
                        }
                    }
                }
            }
        }

        fn move_down(&mut self) {
            for col in 0..4 {
                for row in (0..4).rev() {
                    for testrow in (0..row).rev() {
                        if self.combine_tile(col, row, col, testrow) {
                            break;
                        }
                    }
                }
            }
        }

        fn move_left(&mut self) {
            for row in 0..4 {
                for col in 0..4 {
                    for testcol in (col + 1)..4 {
                        if self.combine_tile(col, row, testcol, row) {
                            break;
                        }
                    }
                }
            }
        }

        fn move_right(&mut self) {
            for row in 0..4 {
                for col in (0..4).rev() {
                    for testcol in (0..col).rev() {
                        if self.combine_tile(col, row, testcol, row) {
                            break;
                        }
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

#[cfg(not(unix))]
fn main() {
    println!("This solution is not supported on Windows.");
}
