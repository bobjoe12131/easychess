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
        println!("")
    }
}
