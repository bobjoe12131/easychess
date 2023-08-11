//! easychess is a library to easily create a chess game.
//! Work In Progress.
//!
//! This is my first rust project so please send pull requests and issues!
//!
//! ## 0.1.0
//! - [x] Write [board::Board]
//! - [x] Write [pieces::Piece]
//! ## 0.2.0
//! - [x] Complete 0.1.0 documentation
//! - [x] Change [pieces::Piece] structure to make pieces::PieceType::None uneeded
//! - [-] ~~Write [board::Board.to_string]~~
//! - [x] Change [board::Board] Display to be less fancy
//! - [x] Write [board::Board.put_piece()]
//! ## 0.3.0
//! - [ ] Write [board::Board.move_piece()]
//! - [ ] Write [board::Board.mut_put_piece()]
//! - [ ] Write [board::Board.mut_move_piece()]
//! - [ ] Make some examples in /examples
//! - [ ] Write game::Game

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
