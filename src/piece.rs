use std::{char, error::Error, fmt::Display};

pub trait Piece<MyPiece>:
    TryFrom<char, Error = PieceTryFromError> + Into<char> + Clone + Copy
{
    const NONE: MyPiece;
    // fn get(self) -> MyPiece;
    // fn get_mut(&mut self) -> &mut MyPiece;
    // fn is_none(&self) -> bool;
}

/// Error for when [Piece::try_from] gets an invalid [char].
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
        write!(
            f,
            "No char to Piece match: A char, '{}', tried to be matched to a Piece, but there is no pattern.",
            self.0
        )
    }
}
