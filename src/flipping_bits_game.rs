// http://rosettacode.org/wiki/Flipping_bits_game

// For random generation
extern crate rand;

// For fmt::Display
use std::fmt;
// For I/O (stdin, stdout, etc)
use std::io::prelude::*;

// A simple struct for our board
struct Board {
    cells: Vec<bool>,
    size: usize,
}

// Functions for the Board struct
impl Board {
    // Generate a new, empty board
    fn new(size: usize) -> Board {
        // Ensure we make a board with a non-zero size
        if size == 0 {
            // Don't make a board of size 0
            Board {
                cells: vec![false; 1],
                size: 1,
            }
        } else {
            Board {
                cells: vec![false; size*size],
                size: size,
            }
        }
    }

    // Flip the specified row
    fn fliprow(&mut self, row: usize) -> bool {
        // Check constraints
        if row > self.size {
            return false;
        }
        // Starting position in the vector
        let start = row * self.size;
        // Loop through the vector row
        for i in start..start + self.size {
            self.cells[i] = !self.cells[i];
        }
        true
    }

    // Flip the specified column
    fn flipcol(&mut self, col: usize) -> bool {
        // Check constraints
        if col > self.size {
            return false;
        }
        // Loop through the vector column
        for i in 0..self.size {
            self.cells[col + i * self.size] = !self.cells[col + i * self.size];
        }
        true
    }

    // Generate a random board
    fn random(size: usize) -> Board {
        // Make a vector of the board size
        let mut r: Vec<bool> = vec![false; size*size];
        // Loop through all the cells
        for i in 0..size * size {
            // Give it a random state
            r[i] = rand::random::<bool>();
        }
        // Return the random board
        Board {
            cells: r.clone(),
            size: size,
        }
    }

    // Check if a board is equal to another
    fn eq(&self, rhs: &Board) -> bool {
        // Has to be a board of the same size
        if self.size != rhs.size {
            return false;
        }
        // Loop through the board
        for i in 0..self.size * self.size {
            // If cells do not match, not equal
            if self.cells[i] != rhs.cells[i] {
                return false;
            }
        }
        // If we get here, boards are equal
        true
    }
}

// Implement the Display format, used with `print!("{}", &board);`
impl fmt::Display for Board {
    // Example output:
    //   0 1 2
    // 0 0 1 0
    // 1 1 0 0
    // 2 0 1 1
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Get the string width of the size of the board
        let width = (self.size - 1).to_string().len();
        // Write the initial spaces (upper left)
        try!(write!(f, "{space: >0$}", width, space = " "));
        // Write the column numbers
        for i in 0..self.size {
            try!(write!(f, " {offset:>0$}", width, offset = i));
        }
        // Newline for rows
        try!(write!(f, "\n"));
        // Loop through the rows
        for row in 0..self.size {
            // Write the row number
            try!(write!(f, "{row:>0$}", width, row = row));
            // Loop through the columns
            for col in 0..self.size {
                // Get the value of the cell as 1 or 0
                let p = if self.cells[row * self.size + col] {
                    1
                } else {
                    0
                };
                // Write the column value
                try!(write!(f, " {col:>0$}", width, col = p));
            }
            // Newline for next row
            try!(write!(f, "\n"));
        }
        // Return Formatter result
        write!(f, "")
    }
}

fn main() {
    // The board size
    let size: usize = 3;
    // The target board
    let target: Board = Board::random(size);
    // The user board
    let mut board: Board = Board::new(size);
    // How many moves taken
    let mut moves: u32 = 0;
    // Loop until win or quit
    'mainloop: loop {
        // User input
        let mut input: String = String::new();
        // Write the boards
        println!("Target:\n{}\nBoard:\n{}", &target, &board);
        // User input loop
        'userinput: loop {
            // Prompt
            print!("\nFlip? [q|[r|c]#] ");
            // Flush stdout to write the previous print, if we can't then exit
            match std::io::stdout().flush() {
                Ok(_) => {}
                Err(e) => {
                    println!("Error: cannot flush stdout: {}", e);
                    break 'mainloop;
                }
            };
            // Read user input
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => {
                    // Get the first character
                    let rc: char = match input.chars().nth(0) {
                        Some(c) => c,
                        None => {
                            println!("Error: No input");
                            continue 'userinput;
                        }
                    };
                    // Make sure input is r, c, or q
                    if rc != 'r' && rc != 'c' && rc != 'q' {
                        println!("Error: '{}': Must use 'r'ow or 'c'olumn or 'q'uit", rc);
                        continue 'userinput;
                    }
                    // If input is q, exit game
                    if rc == 'q' {
                        println!("Thanks for playing!");
                        break 'mainloop;
                    }
                    // If input is r or c, get the number after
                    let n: usize = match input[1..].to_string().trim().parse() {
                        Ok(x) => {
                            // If we're within bounds, return the parsed number
                            if x < size {
                                x
                            } else {
                                println!("Error: Must specify a row or column within size({})",
                                         size);
                                continue 'userinput;
                            }
                        }
                        Err(_) => {
                            println!("Error: '{}': Unable to parse row or column number",
                                     input[1..].to_string());
                            continue 'userinput;
                        }
                    };
                    // Flip the row or column specified
                    match rc {
                        'r' => board.fliprow(n),
                        'c' => board.flipcol(n),
                        _ => {
                            // We want to panic here because should NEVER
                            // have anything other than 'r' or 'c' here
                            panic!("How did you end up here?");
                        }
                    };
                    // Increment moves
                    moves += 1;
                    println!("Moves taken: {}", moves);
                    break 'userinput;
                }
                Err(e) => {
                    println!("Error reading input: {}", e);
                    break 'mainloop;
                }
            }
        } // 'userinput
        if board.eq(&target) {
            println!("You win!");
            break;
        }
    } // 'mainloop
}
