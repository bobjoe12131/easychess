use std::io;

use easychess_lib::{
    self,
    board::Board,
    board_default::BoardDefaults,
    chess::{ChessPiece, ChessPieceTeam, ChessPieceType},
};
fn main() {
    let mut board = ChessPiece::default_board();
    loop {
        println!("{board}");
        println!("Put in the coordinates of the piece and where you want it to go. e.g. '4,5 5,4'");
        let mut x: String;
        io::stdin().read_line(&mut x).or_else(continue);
        let y = x.chars().collect::<Vec<char>>();
    }
}
