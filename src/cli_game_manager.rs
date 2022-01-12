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
        writeln!(writer, "{}", "Hello World");
    }
}

#[cfg(test)]
mod cli_game {
    use crate::cli_game_manager::CLIGameManager;
    use crate::game::Game;

    #[test]
    fn prints_welcome() {
        let cli = CLIGameManager::new(true);
        let mut result = Vec::new();
        cli.print(&mut result);
        assert_eq!(result, b"Hello World\n");
    }


}

