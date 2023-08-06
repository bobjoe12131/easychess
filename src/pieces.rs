use std::fmt::Debug;

/// Error for when [Piece::try_from] gets an invalid [char].
pub struct TryFromError(char, u32, &'static str);

impl Debug for TryFromError {
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
    None,
}

#[derive(Clone, Copy)]
pub enum PieceTeam {
    White,
    Black,
    None,
}

/// Is placed on [crate::board::Board]
/// # TODO:
/// change to Piece(PieceTeam)
#[derive(Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub piece_team: PieceTeam,
}
impl Piece {
    /// Piece struct with None for both fields.
    ///
    /// ## Fields
    /// piece_type: [PieceType::None]
    ///
    /// piece_team: [PieceTeam::None]
    pub const NONE: Self = Self {
        piece_type: PieceType::None,
        piece_team: PieceTeam::None,
    };
}

impl TryFrom<char> for Piece {
    type Error = TryFromError;
    /// Matches a [char] that represents a chess piece to a [PieceType] variant.
    /// Valid chars are:
    /// ```
    /// 'k' => PieceType::King,
    /// 'q' => PieceType::Queen,
    /// 'r' => PieceType::Rook,
    /// 'b' => PieceType::Bishop,
    /// 'n' => PieceType::Knight,
    /// 'p' => PieceType::Pawn,
    /// ' ' | '.' => PieceType::None,
    /// ```
    /// Invalid chars return:
    /// ```
    /// Err(TryFromError(c, line!(), module_path!()))
    /// ```
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let ptype = match value.to_ascii_lowercase() {
            'k' => PieceType::King,
            'q' => PieceType::Queen,
            'r' => PieceType::Rook,
            'b' => PieceType::Bishop,
            'n' => PieceType::Knight,
            'p' => PieceType::Pawn,
            ' ' | '.' => PieceType::None,
            c => return Err(TryFromError(c, line!(), module_path!())),
        };
        let pteam = match value.is_ascii_uppercase() {
            true => PieceTeam::White,
            false => PieceTeam::Black,
        };
        Ok(Self {
            piece_type: ptype,
            piece_team: pteam,
        })
    }
}
impl Into<char> for Piece {
    /// Matches a [PieceType] variant to a [char] that represents a chess piece:
    /**
    ```
    PieceType::King => 'k',
    PieceType::Queen => 'q',
    PieceType::Rook => 'r',
    PieceType::Bishop => 'b',
    PieceType::Knight => 'n',
    PieceType::Pawn => 'p',
    PieceType::None => '.',
    ```
    */
    fn into(self) -> char {
        let matched_char = match self.piece_type {
            PieceType::King => 'k',
            PieceType::Queen => 'q',
            PieceType::Rook => 'r',
            PieceType::Bishop => 'b',
            PieceType::Knight => 'n',
            PieceType::Pawn => 'p',
            PieceType::None => '.',
        };
        match self.piece_team {
            PieceTeam::White => matched_char.to_ascii_uppercase(),
            _ => matched_char,
        }
    }
}
//TODO: impl display sometime
