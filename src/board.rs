use std::fmt;

use crate::pieces::{Piece, TryFromError};

/// Row oriented.
/// Visually look like this:
/// - {
/// - {-----}
/// - {-----}
/// - {-----}
/// - }
/// Board coordinates go right and down.
pub struct Board {
    board: Vec<Vec<Piece>>,
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
    /// # TODO:
    /// Be able to place and move pieces after [Board] is made.
    pub fn new(board_width: i32, board_height: i32) -> Self {
        let thegrid: Vec<Vec<Piece>> =
            vec![vec![Piece::NONE; board_width as usize]; board_height as usize]; // row oriented

        Board { board: thegrid }
    }

    pub fn put_piece(self, piece: Piece, x_pos: usize, y_pos: usize) -> Self {
        let mut new_board = self;
        new_board.board[y_pos][x_pos] = piece;
        new_board
    }
}
impl fmt::Display for Board {
    /// Allow [Board] to be displayed to stdout with macros such as [`println!()`].
    /// Uses ANSI colors.
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
        Ok(Self { board: parseboard? })
    }
}
