#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

//use std::{
//    error::Error,
//    fmt::{Debug, Display},
//};

use std::{error::Error, fmt::Display};

use crate::board::Board;
//use crate::piece::{Piece, PieceTryFromError};

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
pub struct ChessPiece(pub ChessPieceTeam);

impl ChessPiece {
    pub const NONE: ChessPiece = ChessPiece(ChessPieceTeam::None);

    pub fn empty_board(width: usize, height: usize) -> Board {
        // Replace MyPiece with the actual piece.
        let board: Vec<Vec<Self>> = vec![vec![Self::NONE; width]; height]; // row oriented

        Board {
            board,
            width,
            height,
        }
    }

    pub fn default_board() -> Board {
        use ChessPiece as CP;
        use ChessPieceTeam::*;
        use ChessPieceType::*;
        const XX: CP = CP(None);
        const WK: CP = CP(White(King));
        const WQ: CP = CP(White(Queen));
        const WR: CP = CP(White(Rook));
        const WB: CP = CP(White(Bishop));
        const WN: CP = CP(White(Knight));
        const WP: CP = CP(White(Pawn));
        const BK: CP = CP(Black(King));
        const BQ: CP = CP(Black(Queen));
        const BR: CP = CP(Black(Rook));
        const BB: CP = CP(Black(Bishop));
        const BN: CP = CP(Black(Knight));
        const BP: CP = CP(Black(Pawn));

        /*
        Your board, for example:
        [
        [XX,XX,XX,XX],
        [XX,XX,XX,XX],
        [WP,WP,WP,WP],
        ]
        */
        let board = vec![
            vec![BR, BN, BB, BQ, BK, BB, BN, BR],
            vec![BP, BP, BP, BP, BP, BP, BP, BP],
            vec![XX, XX, XX, XX, XX, XX, XX, XX],
            vec![XX, XX, XX, XX, XX, XX, XX, XX],
            vec![XX, XX, XX, XX, XX, XX, XX, XX],
            vec![XX, XX, XX, XX, XX, XX, XX, XX],
            vec![WP, WP, WP, WP, WP, WP, WP, WP],
            vec![WR, WN, WB, WQ, WK, WB, WN, WR],
        ];
        // Put the amount of rows in height and the amount of columns in width

        Board {
            board,
            width: 4,
            height: 3,
        }
    }
}

impl TryFrom<char> for ChessPiece {
    type Error = PieceTryFromError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use ChessPieceTeam::*;
        use ChessPieceType::*;
        let piece_type = match value.to_ascii_lowercase() {
            '.' => return Ok(Self(None)),
            'k' => King,
            'q' => Queen,
            'r' => Rook,
            'b' => Bishop,
            'n' => Knight,
            'p' => Pawn,
            //' ' | '.' => PieceType::None,
            c => return Err(PieceTryFromError(c)),
        };
        let pteam = match value.is_ascii_uppercase() {
            true => White(piece_type),
            false => Black(piece_type),
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

#[derive(Debug)]
pub struct PieceTryFromError(pub char);

impl Error for PieceTryFromError {
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

impl Display for PieceTryFromError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "char '{}' is not a valid chess piece.", self.0)
    }
}
