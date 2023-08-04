//! easychess is a library to easily create a chess game.
//! Work In Progress, right now it can only parse a string into a [Board] and display it with ANSI colors.
//! ## 0.1.0
//! - [x] Write [board::Board]
//! - [x] Write [pieces::Piece]
//! ## 0.2.0
//! - [ ] Complete documentation
//! - [ ] Change [pieces::Piece] into Piece(PieceTeam) and [pieces::PieceTeam] into enum PieceTeam {White(PieceType), Black(PieceType), None}
//! - [ ] Write [board::Board.display]
//! - [ ] Write [board::Board.put]
//!
//#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

pub mod board;
pub mod pieces;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let chesboardstr = "rnbqkbnr
pppppppp
........
........
........
........
PPPPPPPP
RNBQKBNR";
        board::Board::try_from(chesboardstr).unwrap();
    }
}
