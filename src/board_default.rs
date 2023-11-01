use crate::{board::Board, piece::Piece};

pub trait BoardDefaults<MyPiece> {
    fn empty_board(width: usize, height: usize) -> Board<MyPiece> {
        // Replace MyPiece with the actual piece.
        let board: Vec<Vec<MyPiece>> = vec![vec![MyPiece::NONE; width]; height]; // row oriented

        Board {
            board,
            width,
            height,
        }
    }
    fn default_board() -> Board<MyPiece> {
        /*
        Type aliases, for example:
        type MP = MyPiece;
        type MPT = MyPieceTeam
        type MPTy = MyPieceType
        type XX = MP(MPT::None);
        type WP = MP(MPT::White(MPTy::Pawn))
        */

        /*
        Your board, for example:
        [
        [XX,XX,XX,XX],
        [XX,XX,XX,XX],
        [WP,WP,WP,WP],
        ]
        */
        let board = todo!();
        // Put the amount of rows in height and the amount of columns in width
        /*
        Board {
            board,
            width: 4,
            height: 3,
        }
        */
    }
}
