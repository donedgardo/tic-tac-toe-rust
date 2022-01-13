use clap::Parser;
use std::io;
use crate::game::Game;
use crate::cli_game_manager::{CLIGameManager, GameMode};
use crate::play_markers::PlayMarkers;

mod game;
mod play_markers;
mod board;
mod winning_plays;
mod cli_game_manager;
mod ai;


#[derive(Parser, PartialEq)]
struct Cli {
    // Whether to play against "ai-first", "ai-last" or "local"
    #[clap(arg_enum, default_value_t = GameMode::AiLast)]
    mode: GameMode,
}

fn main() {
    let args = Cli::parse();
    let mut cli_game = CLIGameManager::new(args.mode);
    let stdio = io::stdin();
    let reader = stdio.lock();
    let writer = io::stdout();
    cli_game.start(reader, writer);
}
