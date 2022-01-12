use crate::Game;

struct CLIGameManager {
    game: Game,
    player_goes_first: bool
}

impl CLIGameManager {
    pub fn new(first: bool) -> Self {
        Self {
            game: Game::new(),
            player_goes_first: first
        }
    }

    pub fn print(self, mut writer: impl std::io::Write ) {
        let mut output =  self.format_board_display();
        output += "X's turn!\n";
        let _result = writeln!(writer, "{}", output);
    }

    pub fn format_board_display(self) -> String {
        let mut board_display = String::new();
        for space in 1..10 {
            if space % 3 != 0 {
                board_display += "_|";
            } else {
                board_display += "_\n";
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
        assert_eq!(result, b"_|_|_\n_|_|_\n_|_|_\nX's turn!\n\n");
    }


}

