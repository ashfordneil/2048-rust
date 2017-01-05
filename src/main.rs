extern crate rand;
extern crate termion;
extern crate itertools;

use std::io::{stdin, stdout, Write};

use termion::{clear, cursor, terminal_size};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod board;
use board::{BOARD_WIDTH, CELL_WIDTH, Board};

fn main() {
    // initialise screen
    let (width, height) = terminal_size().unwrap();
    print!("{clear}{center}{down}{left}{hide}",
        clear = clear::All,
        center = cursor::Goto(width / 2, height / 2),
        down = cursor::Down(BOARD_WIDTH as u16 / 2),
        left = cursor::Left(BOARD_WIDTH * (CELL_WIDTH + 2) as u16 / 2),
        hide = cursor::Hide);

    // configure raw mode
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    // create the board
    let mut board = Board::new();
    write!(stdout, "{}", board).unwrap();
    stdout.flush().unwrap();

    // process input - main loop of the game
    for key in stdin.keys() {
        let moved = match key.unwrap() {
            Key::Left | Key::Char('a') => board.left(),
            Key::Right | Key::Char('d') => board.right(),
            Key::Up | Key::Char('w') => board.up(),
            Key::Down | Key::Char('s') => board.down(),
            // end game on anything that isn't an arrow key
            _ => break
        };

        // print the board
        write!(stdout, "{}", board).unwrap();
        stdout.flush().unwrap();

        // add new block, and check for gameover
        if moved {
            board.add();
            let can_move_left = board.to_owned().left();
            let can_move_up = board.to_owned().up();
            let has_empty_square = board.to_owned().add();
            if !(can_move_left || can_move_up || has_empty_square) {
                break;
            }
        }
    }

    // exit raw mode
    drop(stdout);

    // print the board
    print!("{board}{goto}", board=board, goto=cursor::Goto(0, height - 1));
    println!("Thanks for playing!");
}
