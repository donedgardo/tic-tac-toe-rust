use std::io::{BufRead, Write};
use crate::Game;
use crate::play_markers::PlayMarkers;


struct CLIGameManager {
    game: Game,
    player_goes_first: bool,
}

impl CLIGameManager {
    pub fn new(first: bool) -> Self {
        Self {
            game: Game::new(),
            player_goes_first: first,
        }
    }

    pub fn start<R, W>(&mut self, mut reader: R, mut writer: W) where R: BufRead, W: Write {
        while !self.game.is_over {
            self.print(&mut writer);
            let space = self.input_play(&mut reader);
        }
    }

    pub fn input_play<R>(&mut self, mut reader: R) where R: BufRead {
        let mut s = String::new();
        let mut space = None;
        while space.is_none() {
            reader.read_line(&mut s).expect("Unable to read.");
            space = match s.parse::<u8>() {
                Ok(n) => Some(n),
                Err(_) => None
            };
        }
        self.game.play(space.unwrap());
    }

    pub fn print<W>(&self, mut writer: W) where W: Write {
        let mut output = self.format_board_display();
        let marker = if self.game.get_active_marker() == PlayMarkers::X { "X" } else { "O" };
        output += marker;
        output += "'s turn!\n";
        let available_plays = self.game.get_available_plays();
        let mut available_plays_formatted = Vec::new();
        for play in available_plays {
            available_plays_formatted.push(play.to_string())
        }
        output += "Available spaces in order from left to right and top to bottom: ";
        output += &*available_plays_formatted.join(", ");
        output += ".\n";
        output += "Enter number: \n";
        let _result = writeln!(writer, "{}", output);
    }

    pub fn format_board_display(&self) -> String {
        let mut board_display = String::new();
        for space in 1..10 {
            let marker = &self.game.board.get_space_marker(&(space - 1));
            let space_display = if marker.is_none() { "_" } else { "X" };
            board_display += space_display;
            if space % 3 != 0 {
                board_display += "|";
            } else {
                board_display += "\n";
            }
        }
        board_display
    }
}

#[cfg(test)]
mod cli_game {
    use crate::cli_game_manager::CLIGameManager;

    #[test]
    fn prints_welcome() {
        let cli = CLIGameManager::new(true);
        let mut result = Vec::new();
        cli.print(&mut result);
        assert_eq!(result, b"_|_|_\n_|_|_\n_|_|_\nX's turn!\nAvailable spaces in order from left to right and top to bottom: 0, 1, 2, 3, 4, 5, 6, 7, 8.\nEnter number: \n\n");
    }

    #[test]
    fn prints_after_play() {
        let mut cli = CLIGameManager::new(true);
        let input = b"0";
        cli.input_play(&input[..]);
        let mut result = Vec::new();
        cli.print(&mut result);
        assert_eq!(result, b"X|_|_\n_|_|_\n_|_|_\nO's turn!\nAvailable spaces in order from left to right and top to bottom: 1, 2, 3, 4, 5, 6, 7, 8.\nEnter number: \n\n");
    }
}
