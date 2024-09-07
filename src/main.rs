use clap::Parser;

use nqueens::*;
use traversal::run;

#[derive(Parser)]
struct Cli {
    #[arg(short('n'), long, value_parser=clap::value_parser!(u8).range(1..16))]
    board_size: u8,
}

fn main() {
    let args = Cli::parse();

    let answer = run(args.board_size);
    println!("Got {} boards", answer.len());
    for (idx, board) in answer.iter().enumerate() {
        println!("Board: {}\n{}", idx + 1, board);
    }
}
