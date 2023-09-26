use crate::{board::Board, piece::Piece};

pub trait BoardDefaults<MyPiece: Piece<MyPiece>> {
    fn empty_board(width: usize, height: usize) -> Board<MyPiece> {
        let thegrid: Vec<Vec<MyPiece>> = vec![vec![MyPiece::NONE; width]; height]; // row oriented

        Board {
            board: thegrid,
            width,
            height,
        }
    }
}
