#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

use std::fmt;

use crate::pieces::{Piece, TryFromError};

/// Error for when a [Piece] is put or moved outside of the [Board] size.
///
/// # Fields
/// * piece_pos: (usize, usize) - (x,y) Where the [Piece] tried to be placed.
/// * board_size: (usize, usize) - (w,h) The size of the [Board].
struct OutOfBoundsError {
    piece_pos: (usize, usize),
    board_size: (usize, usize),
}
impl OutOfBoundsError {
    fn new(piece_pos: (usize, usize), board_size: (usize, usize)) -> Self {
        OutOfBoundsError {
            piece_pos,
            board_size,
        }
    }
}

/// Row oriented.
/// Visually look like this:
/// - {
/// - {-----}
/// - {-----}
/// - {-----}
/// - }
///
/// Board coordinates go right and down.
#[derive(Clone)]
pub struct Board {
    board: Vec<Vec<Piece>>,
    width: usize,
    height: usize,
}
impl Board {
    /// Returns an empty [Board]. Every element is [Piece::NONE]
    ///
    /// # Arguments
    ///
    /// * board_width: [i32] - Board width.
    /// * board_height: [i32] - Board height.
    ///
    /// # Examples
    ///
    /// ```
    /// use easychess::board::Board;
    ///
    /// let empty_board = Board::new(8,8);
    /// ```
    pub fn new(width: usize, height: usize) -> Self {
        let thegrid: Vec<Vec<Piece>> = vec![vec![Piece::NONE; width]; height]; // row oriented

        Board {
            board: thegrid,
            width,
            height,
        }
    }

    pub fn get(self, x_pos: usize, y_pos: usize) -> Option<&'static Piece> {
        self.board.get(y_pos).unwrap().get(x_pos)
    }

    pub fn get_mut(&mut self, x_pos: usize, y_pos: usize) -> Option<&mut Piece> {
        self.board.get_mut(y_pos)?.get_mut(x_pos)
    }

    /// Returns [self] with a [Piece] at board[y_pos][x_pos]
    ///
    /// # Arguments
    ///
    /// * board_width: [i32] - Board width.
    /// * board_height: [i32] - Board height.
    ///
    /// # Examples
    ///
    /// ```
    /// use easychess::board::Board;
    ///
    /// let empty_board = Board::new(8,8);
    /// ```
    pub fn put_piece(
        self,
        piece: Piece,
        x_pos: usize,
        y_pos: usize,
    ) -> Result<Self, OutOfBoundsError> {
        let mut new_board = self;

        match new_board.get_mut(x_pos, y_pos) {
            Some(mut square) => {
                square = &mut piece.clone();
                Ok(new_board)
            }
            None => Err(OutOfBoundsError::new(
                (x_pos, y_pos),
                (self.width, self.height),
            )),
        }
    }

    pub fn move_piece(
        self,
        old_x_pos: usize,
        old_y_pos: usize,
        new_x_pos: usize,
        new_y_pos: usize,
    ) -> Result<Self, OutOfBoundsError> {
        let mut new_board = self;

        match new_board.get_mut(new_x_pos, new_y_pos) {
            Some(mut square) => {
                let mut old_square = match new_board.get_mut(old_x_pos, old_y_pos) {
                    Some(my_square) => my_square,
                    None => {
                        return Err(OutOfBoundsError::new(
                            (old_x_pos, old_y_pos),
                            (self.width, self.height),
                        ))
                    }
                };
                square = &mut old_square.clone();
                old_square = &mut Piece::NONE;
                Ok(new_board)
            }
            None => Err(OutOfBoundsError::new(
                (new_x_pos, new_y_pos),
                (self.width, self.height),
            )),
        }
    }
}
impl fmt::Display for Board {
    /// Allow [Board] to be displayed to stdout with macros such as [`println!()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use easychess::board::Board;
    /// let my_board = Board::new(8,8);
    /// println!("{}", my_board);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let board = self.board.clone();
        let board = board
            .iter()
            .map(|v| {
                //.enumerate()
                //.map(|(n, v)| {
                v.iter()
                    .enumerate()
                    .map(|(m, e)| {
                        let piece_char: char = e.clone().into();
                        /*
                        let even_row = n % 2 == 0;
                        let even_square = match even_row {
                            true => m % 2 == 0,
                            false => !(m % 2 == 0),
                        };

                        let string = match even_square {
                            true => format!(
                                "{}{}{string}{}{}{}",
                                color::Fg(color::Black),
                                color::Bg(color::White),
                                " ",
                                color::Fg(color::Reset),
                                color::Bg(color::Reset),
                            ),
                            false => format!(
                                "{}{}{string}{}{}{}",
                                color::Fg(color::White),
                                color::Bg(color::Black),
                                " ",
                                color::Fg(color::Reset),
                                color::Bg(color::Reset),
                            ),
                        };
                        */
                        let string = match m + 1 == v.len() {
                            true => format!("{piece_char}\n"),
                            false => piece_char.to_string(),
                        };
                        string
                    })
                    .collect::<String>()
            })
            .collect::<String>();
        //board

        write!(f, "{board}")
    }
}
impl TryFrom<&str> for Board {
    type Error = TryFromError;
    /// Parses a [String] into a [Board].
    /// Matches each [char] in [str] with [Piece::try_from].
    /// ```no_run
    /// Piece {
    ///     piece_type: PieceType::None,
    ///     piece_team: PieceTeam::None,    
    /// }
    /// ```
    ///
    /// # Arguments
    ///
    /// * `value`: [`str`] - Board as a [str].
    ///
    /// # Examples
    ///
    /// ```
    /// use easychess::board::Board;
    ///
    /// let my_string =
    /// "rnbqkbnr
    /// pppppppp
    /// ........
    /// ........
    /// ........
    /// ........
    /// PPPPPPPP
    /// RNBQKBNR";
    /// ```
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parseboard: Result<Vec<Vec<Piece>>, TryFromError> = value
            .lines()
            .map(|line| {
                line.chars()
                    .map(|character| Ok(Piece::try_from(character)?))
                    .collect::<Result<Vec<Piece>, Self::Error>>()
            })
            .collect::<Result<Vec<Vec<Piece>>, Self::Error>>();
        Ok(Self {
            board: parseboard?,
            width: parseboard?[0].len(),
            height: parseboard?.len(),
        })
    }
}
