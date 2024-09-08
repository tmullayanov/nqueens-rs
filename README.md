# N-Queens Solver in Rust

A command-line utility to solve the N-Queens problem, implemented in Rust. The N-Queens problem is a classical problem in computer science that involves placing N queens on an N×N chessboard such that no two queens threaten each other.
Table of Contents

    Installation
    Usage
    Examples
    How it Works
    Contributing
    License

## Installation

### Prerequisites

Ensure you have Rust installed on your machine.

Clone this repository and build it manually:

```bash
git clone https://github.com/tmullayanov/nqueens.git
cd nqueens
cargo build --release
```

Alternatively, you can install to your system locally (as a cargo binary):
```bash
git clone https://github.com/tmullayanov/nqueens.git
cd nqueens
cargo install --path .
```

After that, if your `$HOME/.cargo/bin` is in `$PATH`, `nqueens` will be available everywhere in your command line.


## Usage

Once installed, run the utility from the command line.

```bash
nqueens --board-size <N>
```

Example:

```bash

nqueens --board-size 8
```

This command will solve the 8-Queens problem and display the solutions.
Command-line options

    -n, --board-size <N>: Specify the size of the board. Must be in range 1..<16.
    --help: prints help message and quits. 

Examples

Solve for 4 queens:

```bash
nqueens -n 4
```

## How it Works

This utility uses a backtracking algorithm to solve the N-Queens problem. It explores all possible configurations of queens on the board and backtracks whenever it finds a conflict (i.e., two queens threaten each other). This method efficiently finds one or all solutions to the problem.

Backtracking algorithm is generalized and might be reused for other tasks as long as the programmer implements trait `TraversalNode<T>` and provides their own representation of the solution.
(For example, support for N-Rooks problem might be added easily)
This trait consists of methods that should provide a way to:
- generate next intermediate solution.
- detect whether we've reached ultimate solution
- provide answer if we've reached ultimate solution.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the LICENSE file for details.