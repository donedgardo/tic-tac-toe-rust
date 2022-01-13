use clap::Parser;
use std::io;
use crate::game::Game;
use crate::cli_game_manager::CLIGameManager;
use crate::play_markers::PlayMarkers;

mod game;
mod play_markers;
mod board;
mod winning_plays;
mod cli_game_manager;
mod ai;


#[derive(Parser, PartialEq)]
struct Cli {
    // Whether player goes first.
    #[clap(long, parse(try_from_str), default_value_t = true)]
    first: bool,
}

fn main() {
    let _args = Cli::parse();
    let mut cli_game = CLIGameManager::new();

    let stdio = io::stdin();
    let reader = stdio.lock();
    let writer = io::stdout();
    cli_game.start(reader, writer);
}
