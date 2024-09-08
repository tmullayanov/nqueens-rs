use clap::Parser;

use nqueens::*;
use traversal::run;

#[derive(Parser)]
struct Cli {
    #[arg(short('n'), long, value_parser=clap::value_parser!(u8).range(1..15))]
    board_size: u8,
    #[arg(long("first-only"), default_value = "false")]
    first_only: bool,
}

fn main() {
    let args = Cli::parse();

    let answer = run(args.board_size);
    println!("Got {} boards", answer.len());

    if !args.first_only {
        for (idx, board) in answer.iter().enumerate() {
            println!("Board: {}\n{}", idx + 1, board);
        }
    } else if !answer.is_empty() {
        println!("Board: 1\n{}", answer[0]);
    }
}
