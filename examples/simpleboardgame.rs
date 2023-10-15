use std::io;

use easychess_lib::{
    self,
    board::Board,
    board_default::BoardDefaults,
    chess::{ChessPiece, ChessPieceTeam, ChessPieceType},
};

//fn get(index: usize) {}

fn main() {
    let mut board = ChessPiece::default_board();
    loop {
        println!("{board}");
        println!("Put in the coordinates of the piece and where you want it to go. e.g. '4,5 5,4'");
        let mut x = String::new();
        match io::stdin().read_line(&mut x) {
            Err(_) => todo!(),
            _ => {}
        }
        let x = x.chars().collect::<Vec<char>>();

        let get = |index: usize| -> Option<usize> { Some(x.get(index)?.to_digit(10)? as usize) };

        let x: Option<((usize, usize), (usize, usize))> =
            (|| -> Option<((usize, usize), (usize, usize))> {
                Some(((get(0)?, get(2)?), (get(4)?, get(6)?)))
            })();
    }
}
