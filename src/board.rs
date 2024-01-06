#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
use crate::chess::{ChessPiece, PieceTryFromError};
use std::fmt;
#[derive(Clone, Copy, Debug)]
pub struct Pos {
    x: usize,
    y: usize,
}
impl Pos {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Option<Pos> {
        ((1..=width).contains(&x) && (1..=height).contains(&y)).then_some(Pos { x, y })
    }
    fn x(self) -> usize {
        self.x
    }
    fn y(self) -> usize {
        self.y
    }
}

// impl TryFrom<(u8, u8)> for Pos {
//     type Error = ();
//     fn try_from(value: (u8, u8)) -> Self {
//         Self::new(value.0, value.1)
//     }
// }

/// Row oriented.
/// Visually look like this:
/// - {
/// - {-----}
/// - {-----}
/// - {-----}
/// - }
///
/// Board coordinates go down and right.
#[derive(Clone)]
pub struct Board {
    pub board: Vec<Vec<ChessPiece>>,
    pub width: usize,
    pub height: usize,
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
        let board: Vec<Vec<ChessPiece>> = vec![vec![ChessPiece::NONE; width]; height]; // row oriented

        Board {
            board,
            width,
            height,
        }
    }

    pub fn get(self, pos: Pos) -> ChessPiece {
        self.board[pos.y() - 1][pos.x() - 1]
    }

    pub fn get_mut(&mut self, pos: Pos) -> &mut ChessPiece {
        &mut self.board[pos.y() - 1][pos.x() - 1]
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
    pub fn put_piece(&mut self, piece: &ChessPiece, pos: Pos) -> &mut Self {
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
        /*
        let width: usize = self.width;
        let height: usize = self.height;

        let mut square = self
            .get_mut(pos.0, pos.1)
            .ok_or(OutOfBoundsError::new((pos.0, pos.1), (width, height)))?;
        square = &mut piece.clone();
        */
        println!("{self}");
        let temp = self.get_mut(pos);
        let temp2 = piece;
        *temp = *temp2;
        println!("{self}");
        self
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
    pub fn move_piece(&mut self, old_pos: Pos, new_pos: Pos) -> &mut Self {
        {
            // set new square piece to old square piece
            let old_square = self.clone().get(old_pos);
            let new_square = self.get_mut(new_pos);
            *new_square = old_square;
        }
        let old_square = self.get_mut(old_pos);
        *old_square = ChessPiece::NONE;
        self
    }
    pub fn pos(&self, x: usize, y: usize) -> Option<Pos> {
        Pos::new(x, y, self.width, self.height)
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
//impl<MyPiece: Piece<MyPiece>> TryFrom<&str> for Board<MyPiece> {
impl TryFrom<&str> for Board {
    type Error = PieceTryFromError;
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
        let parseboard: Vec<Vec<ChessPiece>> = value
            .lines()
            .map(|line| {
                line.chars()
                    .map(|character| character.try_into())
                    .collect::<Result<Vec<ChessPiece>, Self::Error>>()
            })
            .collect::<Result<Vec<Vec<ChessPiece>>, Self::Error>>()?;
        Ok(Self {
            board: parseboard.clone(),
            width: parseboard[0].len().clone(),
            height: parseboard.len().clone(),
        })
    }
}
