use std::io::{BufRead, Write};
use crate::Game;
use crate::play_markers::PlayMarkers;


struct CLIGameManager {
    game: Game,
    error: Option<String>,
}

impl CLIGameManager {
    pub fn new() -> Self {
        Self {
            game: Game::new(),
            error: None,
        }
    }

    pub fn start<R, W>(&mut self, mut reader: R, mut writer: W) where R: BufRead, W: Write {
        while !self.game.is_over {
            self.print(&mut writer);
            self.input_play(&mut reader);
        }
    }

    pub fn input_play<R>(&mut self, mut reader: R) where R: BufRead {
        let mut s = String::new();
        let mut space = None;
        while space.is_none() {
            reader.read_line(&mut s).expect("Unable to read.");
            space = match s.parse::<u8>() {
                Ok(n) => Some(n),
                Err(_) => {
                    self.error = Some(format!("Error: {} is not valid.\n", s));
                    break;
                }
            };
        }
        if !space.is_none() {
            self.set_move_error(&space.unwrap());
            self.game.play(space.unwrap());
        }
    }

    pub fn print<W>(&self, mut writer: W) where W: Write {
        let mut output = self.format_board_display();
        output += &*self.active_turn_display();
        output += &*self.available_plays_display();
        if !self.error.is_none() {
            output += &self.error.as_ref().unwrap().as_str();
        }
        output += "Enter number: \n";
        println!("{}", output);
        let _result = writeln!(writer, "{}", output);
    }

    fn available_plays_display(&self) -> String {
        let mut display = String::new();
        let available_plays = self.game.get_available_plays();
        let mut available_plays_formatted = Vec::new();
        for play in available_plays {
            available_plays_formatted.push(play.to_string())
        }
        display += "Available spaces in order from left to right and top to bottom: ";
        display += &*available_plays_formatted.join(", ");
        display += ".\n";
        display
    }

    fn active_turn_display(&self) -> String {
        let mut turn_display = String::new();
        let marker = if self.game.get_active_marker() == PlayMarkers::X { "X" } else { "O" };
        turn_display += marker;
        turn_display += "'s turn!\n";
        turn_display
    }

    fn format_board_display(&self) -> String {
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

    fn set_move_error(&mut self, space: &u8) {
        let mut error = String::new();
        if &space > &&(8u8) {
            error = format!("Error: {} is not valid.\n", space);
        } else if self.game.board.is_space_played(&space) {
            error = format!("Error: Can't play in position {}, as it has been already played.\n", space);
        }
        self.error = Some(error);
    }
}

#[cfg(test)]
mod cli_game {
    use crate::cli_game_manager::CLIGameManager;

    #[test]
    fn prints_welcome() {
        let cli = CLIGameManager::new();
        let mut output = Vec::new();
        cli.print(&mut output);
        assert_eq!(output, b"_|_|_\n_|_|_\n_|_|_\nX's turn!\nAvailable spaces in order from left to right and top to bottom: 0, 1, 2, 3, 4, 5, 6, 7, 8.\nEnter number: \n\n");
    }

    #[test]
    fn prints_after_play() {
        let mut cli = CLIGameManager::new();
        let input = b"0";
        cli.input_play(&input[..]);
        let mut output = Vec::new();
        cli.print(&mut output);
        assert_eq!(output, b"X|_|_\n_|_|_\n_|_|_\nO's turn!\nAvailable spaces in order from left to right and top to bottom: 1, 2, 3, 4, 5, 6, 7, 8.\nEnter number: \n\n");
    }

    #[test]
    fn should_print_error_on_invalid_input() {
        let mut cli = CLIGameManager::new();
        let input = b"-1";
        cli.input_play(&input[..]);
        let mut output = Vec::new();
        cli.print(&mut output);
        assert_eq!(output, b"_|_|_\n_|_|_\n_|_|_\nX's turn!\nAvailable spaces in order from left to right and top to bottom: 0, 1, 2, 3, 4, 5, 6, 7, 8.\nError: -1 is not valid.\nEnter number: \n\n");
    }

    #[test]
    fn should_print_error_on_out_of_range_input() {
        let mut cli = CLIGameManager::new();
        let input = b"9";
        cli.input_play(&input[..]);
        let mut output = Vec::new();
        cli.print(&mut output);
        assert_eq!(output, b"_|_|_\n_|_|_\n_|_|_\nX's turn!\nAvailable spaces in order from left to right and top to bottom: 0, 1, 2, 3, 4, 5, 6, 7, 8.\nError: 9 is not valid.\nEnter number: \n\n");
    }

    #[test]
    fn should_print_error_on_playing_occupied_space() {
        let mut cli = CLIGameManager::new();
        let input = b"0";
        cli.input_play(&input[..]);
        let input = b"0";
        cli.input_play(&input[..]);
        let mut output = Vec::new();
        cli.print(&mut output);
        assert_eq!(output, b"X|_|_\n_|_|_\n_|_|_\nO's turn!\nAvailable spaces in order from left to right and top to bottom: 1, 2, 3, 4, 5, 6, 7, 8.\nError: Can't play in position 0, as it has been already played.\nEnter number: \n\n");
    }
}

