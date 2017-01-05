use std::fmt;
use std::iter::{Iterator, Peekable};
use itertools::Itertools;
use rand::{thread_rng, ThreadRng, Rng};
use termion::cursor;

pub const BOARD_WIDTH: u16 = 4;
pub const CELL_WIDTH: u16 = 6;

/// A single square within the board.
#[derive(Debug, Copy, Clone, PartialEq)]
struct Digit(Option<u8>);

impl fmt::Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // pad to CELL_WIDTH characters wide
            Digit(Some(val)) => write!(f, "{: ^6}", 1 << val),
            Digit(None) => write!(f, "{: ^6}", ""),
        }
    }
}

/// Stores the board, and everything within it.
#[derive(Clone)]
pub struct Board {
    rng: ThreadRng,
    data: [[Digit; BOARD_WIDTH as usize]; BOARD_WIDTH as usize],
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", cursor::Up(BOARD_WIDTH)));
        for row in self.data.iter() {
            for element in row.iter() {
                try!(write!(f, "|{}|", element));
            }
            try!(write!(f,
                        "{left}{down}",
                        left = cursor::Left((CELL_WIDTH + 2) * BOARD_WIDTH),
                        down = cursor::Down(1)));
        }
        Ok(())
    }
}

impl Board {
    /// Creates a new board struct, with two randomly added pieces.
    pub fn new() -> Board {
        let mut output = Board {
            rng: thread_rng(),
            data: [[Digit(None); BOARD_WIDTH as usize]; BOARD_WIDTH as usize],
        };
        output.add();
        output.add();
        output
    }

    /// Adds a block to the board.
    pub fn add(&mut self) -> bool {
        if !self.contains_empty() {
            return false;
        }
        loop {
            let index = self.rng.next_u32() as usize;
            let (x, y) = (index % BOARD_WIDTH as usize,
                          (index / BOARD_WIDTH as usize) % BOARD_WIDTH as usize);
            if let Digit(None) = self.data[x][y] {
                self.data[x][y] =
                    Digit(Some((((index / BOARD_WIDTH as usize / BOARD_WIDTH as usize %
                                  2) as u8))));
                return true;
            }
        }
    }

    /// Determines if there is a square within the board that does not currently contain a tile.
    fn contains_empty(&self) -> bool {
        for row in self.data.iter() {
            for element in row.iter() {
                if let &Digit(None) = element {
                    return true;
                }
            }
        }
        false
    }

    /// A helper function, used in the collapse function. It takes the first digit from the
    /// iterator given to it, and checks if the next digit in the iterator has the same value. If
    /// it does, that digit is also consumed form the iterator, and this function returns a new
    /// digit one larger than these two. Otherwise, the original digit is returned.
    fn collapse_helper<'a, I>(iter: &mut Peekable<I>) -> Option<Digit>
        where I: Iterator<Item = &'a Digit> + 'a
    {
        let x = match iter.next() {
            Some(&Digit(Some(x))) => x,
            _ => return None,
        };
        let y = match iter.peek() {
            Some(&&Digit(Some(y))) => y,
            _ => return Some(Digit(Some(x))),
        };
        match x == y {
            true => {
                let _ = iter.next();
                Some(Digit(Some(x + 1)))
            }
            false => Some(Digit(Some(x))),
        }
    }

    /// Takes an iterator describing a row or column on the board, and collapses it -- filling
    /// empty spaces and combining adjacent equal components.
    fn collapse<'a, I>(row: I) -> Box<Iterator<Item = Digit> + 'a>
        where I: Iterator<Item = &'a Digit> + 'a
    {
        Box::new(row.filter(|element| {
                let &&Digit(data) = element;
                data.is_some()
            })
            .peekable()
            .batching(Board::collapse_helper))
    }

    /// Moves the board left. Returns true if the move was successful, and false if not.
    pub fn left(&mut self) -> bool {
        let mut output = false;
        for row in self.data.iter_mut() {
            let old_row = row.to_owned();
            let mut new_row = Board::collapse(old_row.iter());
            for i in 0..(BOARD_WIDTH as usize) {
                let new_digit = new_row.next().unwrap_or(Digit(None));
                if new_digit != row[i] {
                    row[i] = new_digit;
                    output = true;
                }
            }
        }
        output
    }

    /// Moves the board right. Returns true if the move was successful, and false if not.
    pub fn right(&mut self) -> bool {
        let mut output = false;
        for row in self.data.iter_mut() {
            let old_row = row.to_owned();
            let mut new_row = Board::collapse(old_row.iter().rev());
            for i in (0..(BOARD_WIDTH as usize)).rev() {
                let new_digit = new_row.next().unwrap_or(Digit(None));
                if new_digit != row[i] {
                    row[i] = new_digit;
                    output = true;
                }
            }
        }
        output
    }

    /// Moves the board up. Returns true if the move was successful, and false if not.
    pub fn up(&mut self) -> bool {
        let mut output = false;
        for column in 0..(BOARD_WIDTH as usize) {
            let old_column: Vec<_> =
                (0..(BOARD_WIDTH as usize)).map(|x| self.data[x][column]).collect();
            let mut new_column = Board::collapse(old_column.iter());
            for i in 0..(BOARD_WIDTH as usize) {
                let new_digit = new_column.next().unwrap_or(Digit(None));
                if new_digit != self.data[i][column] {
                    self.data[i][column] = new_digit;
                    output = true;
                }
            }
        }
        output
    }

    pub fn down(&mut self) -> bool {
        let mut output = false;
        for column in 0..(BOARD_WIDTH as usize) {
            let old_column: Vec<_> = (0..(BOARD_WIDTH as usize)).map(|x| self.data[x][column]).collect();
            let mut new_column = Board::collapse(old_column.iter().rev());
            for i in (0..(BOARD_WIDTH as usize)).rev() {
                let new_digit = new_column.next().unwrap_or(Digit(None));
                if new_digit != self.data[i][column] {
                    self.data[i][column] = new_digit;
                    output = true;
                }
            }
        }
        output
    }
}
