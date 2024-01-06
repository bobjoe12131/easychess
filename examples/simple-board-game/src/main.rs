use clap::Parser;
use clap::ValueEnum;
use crossterm::{terminal::EnterAlternateScreen, ExecutableCommand};
use easychess::{board::Pos, chess::ChessPiece};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct CliPos(Pos, Pos);

impl FromStr for CliPos {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // "1,2 3,4"
        let error = Err("Parse error. Example: ".to_string());
        let split = s.split(' ');
        // ["1,2", "3,4"]
        if split.count() != 2 {
            return error;
        }
        let split = split.map(|pos| {
            let pos = pos.split(',');
            if pos.count() != 2 {
                return None;
            }
            pos.map(str::parse::<usize>)
                .collect::<Result<Vec<_>, _>>()
                .ok()
        });
        .collect::<>();
    }
}

fn main() {
    let mut board = ChessPiece::default_board();
    let mut out = std::io::stdout();
    out.execute(EnterAlternateScreen).unwrap();

    loop {
        println!("{board}");
        println!("Put in the coordinates of the piece and where you want it to go. e.g. '4,5 5,4'");
        let mut line = match read_line() {
            Some(l) => {
                println!("{l}");
            }
            None => {
                println!("Read error");
                continue;
            }
        };
        let x = line.chars().collect::<Vec<char>>();

        let get = |index: usize| -> Option<usize> { Some(x.get(index)?.to_digit(10)? as usize) };

        let x: ((usize, usize), (usize, usize)) = //Option<((usize, usize), (usize, usize))> =
            (|| {
                Some(((get(0)?, get(2)?), (get(4)?, get(6)?)))
            })()
            .unwrap_or({println!("abbbaaaa"); continue});
        println!("workyturkey: {:?}", x);
        board.move_piece(x.0, x.1);
    }
    out.execute(LeaveAlternateScreen).unwrap();
}

fn read_line() -> Option<String> {
    let mut x = String::new();
    match std::io::stdin().read_line(&mut x) {
        Ok(_) => Some(x),
        Err(_) => None,
    }
}

/*
macro_rules! parse_line {
    ($err_string:expr, $($t: ty),+) => ({
        let mut a_str = String::new();
        io::stdin().read_line(&mut a_str).expect("read error");
        let mut a_iter = a_str.split_whitespace();
        (
            $(
            a_iter.next().unwrap().parse::<$t>().expect($err_string),
            )+
        )
    })
}
*/
