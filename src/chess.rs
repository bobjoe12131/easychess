#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

//use std::{
//    error::Error,
//    fmt::{Debug, Display},
//};

use crate::board::Board;
use crate::board_default::BoardDefaults;
use crate::piece::{Piece, PieceTryFromError};

#[derive(Debug, Clone, Copy)]
pub enum ChessPieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}
#[derive(Debug, Clone, Copy)]
pub enum ChessPieceTeam {
    White(ChessPieceType),
    Black(ChessPieceType),
    None,
}

#[derive(Debug, Clone, Copy)]
pub struct ChessPiece(ChessPieceTeam);

impl Piece<ChessPiece> for ChessPiece {
    const NONE: ChessPiece = ChessPiece(ChessPieceTeam::None);
}

impl TryFrom<char> for ChessPiece {
    type Error = PieceTryFromError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let piece_type = match value.to_ascii_lowercase() {
            '.' => return Ok(Self(ChessPieceTeam::None)),
            'k' => ChessPieceType::King,
            'q' => ChessPieceType::Queen,
            'r' => ChessPieceType::Rook,
            'b' => ChessPieceType::Bishop,
            'n' => ChessPieceType::Knight,
            'p' => ChessPieceType::Pawn,
            //' ' | '.' => PieceType::None,
            c => return Err(PieceTryFromError(c)),
        };
        let pteam = match value.is_ascii_uppercase() {
            true => ChessPieceTeam::White(piece_type),
            false => ChessPieceTeam::Black(piece_type),
        };
        Ok(Self(pteam))
    }
}

impl Into<char> for ChessPiece {
    fn into(self) -> char {
        let matched_char = match self.0 {
            ChessPieceTeam::None => return '.',
            ChessPieceTeam::White(ptype) | ChessPieceTeam::Black(ptype) => match ptype {
                ChessPieceType::King => 'k',
                ChessPieceType::Queen => 'q',
                ChessPieceType::Rook => 'r',
                ChessPieceType::Bishop => 'b',
                ChessPieceType::Knight => 'n',
                ChessPieceType::Pawn => 'p',
            },
        };
        match self.0 {
            ChessPieceTeam::White(_) => matched_char.to_ascii_uppercase(),
            _ => matched_char,
        }
    }
}

impl BoardDefaults<ChessPiece> for ChessPiece {
    fn empty_board(width: usize, height: usize) -> Board<ChessPiece> {
        let thegrid: Vec<Vec<ChessPiece>> = vec![vec![ChessPiece::NONE; width]; height]; // row oriented

        Board {
            board: thegrid,
            width,
            height,
        }
    }
}

// /// Error for when [Piece::try_from] gets an invalid [char].
// #[derive(Debug)]
// pub struct TryFromError(char);

// impl Error for TryFromError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         None
//     }

//     fn description(&self) -> &str {
//         "description() is deprecated; use Display"
//     }

//     fn cause(&self) -> Option<&dyn Error> {
//         self.source()
//     }
// }

// impl Display for TryFromError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "Invalid char to Piece match: A char, '{}', tried to be matched to a Piece, but there is no pattern.",
//             self.0
//         )
//     }
// }

// #[derive(Clone, Copy)]
// pub enum PieceType {
//     King,
//     Queen,
//     Rook,
//     Bishop,
//     Knight,
//     Pawn,
// }

// #[derive(Clone, Copy)]
// pub enum PieceTeam {
//     White(PieceType),
//     Black(PieceType),
//     None,
// }

// /// Is placed on [crate::board::Board]
// #[derive(Clone, Copy)]
// pub struct Piece(pub PieceTeam);
// impl Piece {
//     /// Piece struct with [PieceTeam::None]
//     pub const NONE: Self = Self(PieceTeam::None);
// }

// impl TryFrom<char> for Piece {
//     type Error = TryFromError;
//     /// Matches a [char] that represents a chess piece to a [Piece].
//     /// Valid chars are:
//     /// ```no_run
//     /// '.' => return PieceTeam::None
//     /// 'k' => PieceType::King,
//     /// 'q' => PieceType::Queen,
//     /// 'r' => PieceType::Rook,
//     /// 'b' => PieceType::Bishop,
//     /// 'n' => PieceType::Knight,
//     /// 'p' => PieceType::Pawn,
//     /// ```
//     /// Matches uppercase and lowercase to [PieceTeam]:
//     /// ```no_run
//     /// true => PieceTeam::White(ptype),
//     /// false => PieceTeam::Black(ptype),
//     /// ```
//     ///
//     /// # Errors
//     ///
//     /// Invalid chars return:
//     /// ```no_run
//     /// Err(TryFromError(c, line!(), module_path!()))
//     /// ```
//     fn try_from(value: char) -> Result<Self, Self::Error> {
//         let ptype = match value.to_ascii_lowercase() {
//             '.' => return Ok(Self(PieceTeam::None)),
//             'k' => PieceType::King,
//             'q' => PieceType::Queen,
//             'r' => PieceType::Rook,
//             'b' => PieceType::Bishop,
//             'n' => PieceType::Knight,
//             'p' => PieceType::Pawn,
//             //' ' | '.' => PieceType::None,
//             c => return Err(TryFromError(c)),
//         };
//         let pteam = match value.is_ascii_uppercase() {
//             true => PieceTeam::White(ptype),
//             false => PieceTeam::Black(ptype),
//         };
//         Ok(Self(pteam))
//     }
// }
// impl Into<char> for Piece {
//     /// Matches a [PieceType] variant to a [char] that represents a chess piece:
//     /**
//     ```
//     PieceType::None => return '.',
//     PieceType::King => 'k',
//     PieceType::Queen => 'q',
//     PieceType::Rook => 'r',
//     PieceType::Bishop => 'b',
//     PieceType::Knight => 'n',
//     PieceType::Pawn => 'p',
//     ```
//     */
//     ///
//     fn into(self) -> char {
//         // let matched_char = match self.piece_type {
//         // PieceType::King => 'k',
//         // PieceType::Queen => 'q',
//         // PieceType::Rook => 'r',
//         // PieceType::Bishop => 'b',
//         // PieceType::Knight => 'n',
//         // PieceType::Pawn => 'p',
//         // PieceType::None => '.',
//         // };
//         // match self.piece_team {
//         //     PieceTeam::White => matched_char.to_ascii_uppercase(),
//         //     _ => matched_char,
//         // }
//         let matched_char = match self.0 {
//             PieceTeam::None => return '.',
//             PieceTeam::White(ptype) | PieceTeam::Black(ptype) => match ptype {
//                 PieceType::King => 'k',
//                 PieceType::Queen => 'q',
//                 PieceType::Rook => 'r',
//                 PieceType::Bishop => 'b',
//                 PieceType::Knight => 'n',
//                 PieceType::Pawn => 'p',
//             },
//         };
//         match self.0 {
//             PieceTeam::White(_) => matched_char.to_ascii_uppercase(),
//             _ => matched_char,
//         }
//     }
// }
// //TODO: impl display sometime
