#[cfg(feature = "ncurses")]
extern crate ncurses;

#[cfg(feature = "ncurses")]
fn main() {
    ncurses::initscr();
    loop {
        ncurses::printw("Yes or no? ");
        ncurses::refresh();

        match ncurses::getch() as u8 as char {
            'Y' | 'y' => {
                ncurses::printw("You said yes!");
            }
            'N' | 'n' => {
                ncurses::printw("You said no!");
            }
            _ => {
                ncurses::printw("Try again!\n");
                continue;
            }
        }

        break;
    }

    ncurses::refresh();
    ncurses::endwin();
}

#[cfg(not(feature = "ncurses"))]
fn main() {}
