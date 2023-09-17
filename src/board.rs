#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

use std::{
    error::Error,
    fmt::{self, Display},
};

use crate::pieces::{Piece, TryFromError};

/// Error for when a [Piece] is put or moved outside of the [Board] size.
///
/// # Fields
/// * piece_pos: (usize, usize) - (x,y) Where the [Piece] tried to be placed.
/// * board_size: (usize, usize) - (w,h) The size of the [Board].
#[derive(Debug)]
pub struct OutOfBoundsError {
    piece_pos: (usize, usize),
    board_size: (usize, usize),
}
impl OutOfBoundsError {
    pub fn new(piece_pos: (usize, usize), board_size: (usize, usize)) -> Self {
        OutOfBoundsError {
            piece_pos,
            board_size,
        }
    }
}
type OBE = OutOfBoundsError;
impl Error for OutOfBoundsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl Display for OutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Piece is out of bounds: A Piece tried to be at ({},{}) on a Board, but the Board's size is only ({},{}).",
            self.piece_pos.0, self.piece_pos.1, self.board_size.0, self.board_size.0,
        )
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
    /// * board_width: [usize] - Board width.
    /// * board_height: [usize] - Board height.
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

    pub fn get(&self, x_pos: usize, y_pos: usize) -> Option<Piece> {
        Some(self.board.get(y_pos)?.get(x_pos)?.clone())
    }
    pub fn get_mut(&mut self, x_pos: usize, y_pos: usize) -> Option<&mut Piece> {
        self.board.get_mut(y_pos)?.get_mut(x_pos)
    }

    /// Returns [self] with a [Piece] at board[y_pos][x_pos]
    ///
    /// # Arguments
    ///
    /// * piece: [Piece] -
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
        &mut self,
        piece: Piece,
        pos: (usize, usize),
    ) -> Result<&mut Self, OutOfBoundsError> {
        //let mut new_board = self;

        // match self.get_mut(x_pos, y_pos) {
        //     Some(mut square) => {
        //         square = &mut piece.clone();
        //         Ok(self)
        //     }
        //     None => Err(OutOfBoundsError::new(
        //         (x_pos, y_pos),
        //         (self.width, self.height),
        //     )),
        // }

        let width: usize = self.width;
        let height: usize = self.height;

        let mut square = self
            .get_mut(pos.0, pos.1)
            .ok_or(OutOfBoundsError::new((pos.0, pos.1), (width, height)))?;
        square = &mut piece.clone();
        Ok(self)
    }

    /// Moves the piece at the old position to the new position.
    ///
    /// The [Piece] at the new position set to the old position [Piece].
    /// The [Piece] at the old position set to [Piece(None)]
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
    /// # Errors
    ///
    /// If the old or new position is out of board bounds, it will return the following.
    /// The old position will be returned in the error first.
    /** ```no_run
    Err(OutOfBoundsError::new(
        (*_x_pos, *_y_pos),
        (self.width, self.height),
    ))
    ``` */
    pub fn move_piece(
        &mut self,
        old_pos: (usize, usize),
        new_pos: (usize, usize),
    ) -> Result<&mut Self, OutOfBoundsError> {
        // if let Some(mut old_square) = self.get_mut(old_pos.0, old_pos.1) {
        //     if let Some(mut new_square) = self.get_mut(new_pos.0, new_pos.1) {
        //         new_square = old_square;
        //         old_square = &mut Piece::NONE;
        //         Ok(self)
        //     } else {
        //         Err(OutOfBoundsError::new(
        //             (new_pos.0, new_pos.1),
        //             (self.width, self.height),
        //         ))
        //     }
        // } else {
        //     Err(OutOfBoundsError::new(
        //         (old_pos.0, old_pos.1),
        //         (self.width, self.height),
        //     ))
        // }

        let width: usize = self.width;
        let height: usize = self.height;

        // Get the piece at the old position.
        // Clone it because there can not be two mutable references.
        let mut old_square: Piece = self.get(old_pos.0, old_pos.1).ok_or(OutOfBoundsError::new(
            (old_pos.0, old_pos.1),
            (width, height),
        ))?;

        let mut new_square = self
            .get_mut(old_pos.0, old_pos.1)
            .ok_or(OutOfBoundsError::new(
                (old_pos.0, old_pos.1),
                (width, height),
            ))?;

        new_square = &mut old_square;
        Ok(self)
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
        let parseboard: Vec<Vec<Piece>> = value
            .lines()
            .map(|line| {
                line.chars()
                    .map(|character| Ok(Piece::try_from(character)?))
                    .collect::<Result<Vec<Piece>, Self::Error>>()
            })
            .collect::<Result<Vec<Vec<Piece>>, Self::Error>>()?;
        Ok(Self {
            board: parseboard.clone(),
            width: parseboard[0].len().clone(),
            height: parseboard.len().clone(),
        })
    }
}
