use clap::Parser;
use crate::game::Game;

mod game;
mod play_markers;
mod board;
mod winning_plays;
mod cli_game_manager;

#[derive(Parser, PartialEq)]
struct Cli {
    // Whether player goes first.
    #[clap(long, parse(try_from_str), default_value_t = true)]
    first: bool,
}

fn main() {
    let args = Cli::parse();
    println!("First: {}", args.first);
    println!("Hello, world!");
}
