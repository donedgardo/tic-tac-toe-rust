use crate::board::Board;

#[derive(Debug)]
pub struct Game {
    is_over: bool,
    board: Board,
    error: Option<String>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            is_over: false,
            board: Board::new(),
            error: None,
        }
    }

    pub fn play(&mut self, space: u8) {
        if self.board.is_space_played(&space) {
            let error = format!("Can't play in position {}, as it has been already played.", space);
            self.error = Some(error);
        } else {
            self.board.play(space)
        }
        if self.board.is_full() {
            self.is_over = true;
        }
    }
}

#[cfg(test)]
mod new_game {
    use crate::game::Game;
    use crate::play_markers::PlayMarkers;

    #[test]
    fn is_not_over() {
        let game = Game::new();
        assert!(!game.is_over)
    }

    #[test]
    fn has_empty_board() {
        let game = Game::new();
        let board = game.board;
        assert!(board.is_empty())
    }

    #[test]
    fn x_plays_first() {
        let mut game = Game::new();
        game.play(0);
        assert_eq!(game.board.get_space_marker(&0), Some(&PlayMarkers::X));
    }

    #[test]
    fn o_plays_second() {
        let mut game = Game::new();
        game.play(0);
        game.play(1);
        assert_eq!(game.board.get_space_marker(&1), Some(&PlayMarkers::O));
    }

    #[test]
    fn error_when_playing_in_taken_position() {
        let mut game = Game::new();
        game.play(0);
        game.play(0);
        let expected_error = "Can't play in position 0, as it has been already played.".to_string();
        assert_eq!(game.error, Some(expected_error))
    }

    #[test]
    fn board_unchanged_after_playing_in_taken_position() {
        let mut game = Game::new();
        game.play(0);
        game.play(0);
        assert_eq!(game.board.get_space_marker(&0), Some(&PlayMarkers::X))
    }

    #[test]
    fn cats_game_is_over() {
        let mut game = Game::new();
        const CATS_GAME: [u8; 9] = [0, 4, 2, 1, 7, 5, 3, 6, 8];
        for space in CATS_GAME  {
            game.play(space);
        }
        assert!(game.is_over);
    }
}
