//! easychess is a library to create a chess game.
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
//! - [x] ~~Write [board::Board.to_string]~~
//! - [x] Change [board::Board] Display to be less fancy
//! - [x] Write [board::Board.put_piece()]
//! ## 0.3.0
//! - [x] Write [board::Board.get()]
//! - [x] Write [board::Board.get_mut()]
//! - [x] Write [board::Board.put_piece()]
//! - [x] Write [board::Board.move_piece()]
//! - [x] Write errors for the four features above
//! - [X] Impl [Error] for errors
//! - [ ] Write Piece as a trait
//! ### 0.4.0
//! - [ ] Make some examples in /examples
//! - [ ] Write game::Game
//! - [ ] Fix doc warnings
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
