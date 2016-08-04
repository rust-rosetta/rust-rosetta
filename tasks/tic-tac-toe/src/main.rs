extern crate rand;

use GameState::{PlayerWin, ComputerWin, Draw, Playing};

#[derive(PartialEq, Debug)]
enum GameState {
    PlayerWin,
    ComputerWin,
    Draw,
    Playing,
}

fn main() {
    let mut board: [[char; 3]; 3] = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];

    draw_board(board);
    loop {
        player_turn(&mut board);
        if check_win(board) != Playing {
            break;
        }
        computer_turn(&mut board);
        if check_win(board) != Playing {
            break;
        }
        draw_board(board);
    }

    draw_board(board);
    let announcement = match check_win(board) {
        PlayerWin => "The Player has won!",
        ComputerWin => "The Computer has won!",
        Draw => "Draw!",
        Playing => unreachable!(),
    };
    println!("{}", announcement);
}

fn check_win(board: [[char; 3]; 3]) -> GameState {
    // check for win
    for (i, row) in board.iter().enumerate() {
        if row[0] == row[1] && row[0] == row[2] {
            return which_win(row[0]);
        } else if board[0][i] == board[1][i] && board[0][i] == board[2][i] {
            return which_win(board[0][i]);
        }
    }
    if board[0][0] == board[1][1] && board[0][0] == board[2][2] {
        return which_win(board[0][0]);
    } else if board[0][2] == board[1][1] && board[0][2] == board[2][0] {
        return which_win(board[0][2]);
    }

    // check if it's not a draw
    for row in &board {
        for c in row {
            if *c != 'X' && *c != 'O' {
                return Playing;
            }
        }
    }
    Draw
}

fn which_win(s: char) -> GameState {
    match s {
        'X' => PlayerWin,
        'O' => ComputerWin,
        _ => unreachable!(),
    }
}

fn player_turn(board: &mut [[char; 3]; 3]) {
    use std::io;

    println!("Player, enter your field of choice!: ");
    let mut ln = String::new();
    let _ = io::stdin().read_line(&mut ln);
    let choice = ln.trim().parse::<usize>().unwrap();
    let row = (choice - 1) / 3;
    let col = (choice - 1) % 3;

    if board[row][col] == 'X' || board[row][col] == 'O' {
        println!("Someone already took this field!");
        player_turn(board);
    } else {
        board[row][col] = 'X';
    }
}

fn computer_turn(board: &mut [[char; 3]; 3]) {
    use rand::{thread_rng, Rng};

    // Computer just does a random possible move
    let mut rng = thread_rng();

    let mut possible_choices: Vec<char> = vec![];
    for row in board.iter() {
        for space in row {
            if *space != 'X' && *space != 'O' {
                possible_choices.push(*space);
            }
        }
    }

    let choice_index: usize = rng.gen_range(0, possible_choices.len());
    let choice = possible_choices[choice_index] as usize - 48;
    println!("Computer chose: {}", choice);
    let row = (choice - 1) / 3;
    let col = (choice - 1) % 3;
    board[row][col] = 'O';
}

fn draw_board(board: [[char; 3]; 3]) {
    for row in &board {
        println!("{} {} {}", row[0], row[1], row[2]);
    }
}

#[test]
fn test_which_win() {
    assert_eq!(which_win('X'), PlayerWin);
    assert_eq!(which_win('O'), ComputerWin);
}

#[test]
fn test_check_win() {
    assert_eq!(check_win([['X', 'X', 'X'], ['4', 'O', 'O'], ['7', '8', '9']]),
               PlayerWin);
    assert_eq!(check_win([['O', 'X', '3'], ['X', 'O', '6'], ['X', '8', 'O']]),
               ComputerWin);
    assert_eq!(check_win([['O', 'X', 'X'], ['X', 'O', 'O'], ['O', 'X', 'X']]),
               Draw);
}
