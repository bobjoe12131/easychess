#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

use std::{
    error::Error,
    fmt::{Debug, Display},
};

/// Error for when [Piece::try_from] gets an invalid [char].
#[derive(Debug)]
pub struct TryFromError(char, u32, &'static str);

impl Error for TryFromError {
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

impl Display for TryFromError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "invalid char '{}' at line {} in file {}",
            self.0, self.1, self.2
        )
    }
}

#[derive(Clone, Copy)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy)]
pub enum PieceTeam {
    White(PieceType),
    Black(PieceType),
    None,
}

/// Is placed on [crate::board::Board]
#[derive(Clone, Copy)]
pub struct Piece(pub PieceTeam);
impl Piece {
    /// Piece struct with [PieceTeam::None]
    pub const NONE: Self = Self(PieceTeam::None);
}

impl TryFrom<char> for Piece {
    type Error = TryFromError;
    /// Matches a [char] that represents a chess piece to a [Piece].
    /// Valid chars are:
    /// ```no_run
    /// '.' => return PieceTeam::None
    /// 'k' => PieceType::King,
    /// 'q' => PieceType::Queen,
    /// 'r' => PieceType::Rook,
    /// 'b' => PieceType::Bishop,
    /// 'n' => PieceType::Knight,
    /// 'p' => PieceType::Pawn,
    /// ```
    /// Matches uppercase and lowercase to [PieceTeam]:
    /// ```no_run
    /// true => PieceTeam::White(ptype),
    /// false => PieceTeam::Black(ptype),
    /// ```
    ///
    /// # Errors
    /// Invalid chars return:
    /// ```
    /// Err(TryFromError(c, line!(), module_path!()))
    /// ```
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let ptype = match value.to_ascii_lowercase() {
            '.' => return Ok(Self(PieceTeam::None)),
            'k' => PieceType::King,
            'q' => PieceType::Queen,
            'r' => PieceType::Rook,
            'b' => PieceType::Bishop,
            'n' => PieceType::Knight,
            'p' => PieceType::Pawn,
            //' ' | '.' => PieceType::None,
            c => return Err(TryFromError(c, line!(), module_path!())),
        };
        let pteam = match value.is_ascii_uppercase() {
            true => PieceTeam::White(ptype),
            false => PieceTeam::Black(ptype),
        };
        Ok(Self(pteam))
    }
}
impl Into<char> for Piece {
    /// Matches a [PieceType] variant to a [char] that represents a chess piece:
    /**
    ```
    PieceType::None => return '.',
    PieceType::King => 'k',
    PieceType::Queen => 'q',
    PieceType::Rook => 'r',
    PieceType::Bishop => 'b',
    PieceType::Knight => 'n',
    PieceType::Pawn => 'p',
    ```
    */
    ///
    fn into(self) -> char {
        // let matched_char = match self.piece_type {
        // PieceType::King => 'k',
        // PieceType::Queen => 'q',
        // PieceType::Rook => 'r',
        // PieceType::Bishop => 'b',
        // PieceType::Knight => 'n',
        // PieceType::Pawn => 'p',
        // PieceType::None => '.',
        // };
        // match self.piece_team {
        //     PieceTeam::White => matched_char.to_ascii_uppercase(),
        //     _ => matched_char,
        // }
        let matched_char = match self.0 {
            PieceTeam::None => return '.',
            PieceTeam::White(ptype) | PieceTeam::Black(ptype) => match ptype {
                PieceType::King => 'k',
                PieceType::Queen => 'q',
                PieceType::Rook => 'r',
                PieceType::Bishop => 'b',
                PieceType::Knight => 'n',
                PieceType::Pawn => 'p',
            },
        };
        match self.0 {
            PieceTeam::White(_) => matched_char.to_ascii_uppercase(),
            _ => matched_char,
        }
    }
}
//TODO: impl display sometime
