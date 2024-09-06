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

    println!("Args: n={:?}", args.board_size);


    let answer = run(args.board_size);
    println!("Solution:");
    println!("{:?}", answer);
}
