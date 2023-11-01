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
            Ok(n) => {
                println!("{n} bytes read");
                println!("{x}");
            }
            Err(error) => {
                println!("error: {error}");
                continue;
            }
        }
        //io::stdin().read_line(&mut x).unwrap_or({
        //    println!("aaaaa");
        //    continue;
        //});
        let x = x.chars().collect::<Vec<char>>();

        let get = |index: usize| -> Option<usize> { Some(x.get(index)?.to_digit(10)? as usize) };

        let x: ((usize, usize), (usize, usize)) = //Option<((usize, usize), (usize, usize))> =
            (|| {
                Some(((get(0)?, get(2)?), (get(4)?, get(6)?)))
            })()
            .unwrap_or({println!("abbbaaaa"); continue});
        println!("workyturkey: {:?}", x);
        board.move_piece(x.0, x.1);
    }
}
