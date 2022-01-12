use std::collections::HashMap;
use crate::play_markers::PlayMarkers;

#[derive(Debug)]
pub struct Game {
    is_over: bool,
    board: HashMap<u8, PlayMarkers>,
    error: Option<String>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            is_over: false,
            board: HashMap::new(),
            error: None,
        }
    }
    pub fn play(&mut self, space: u8) {
        if self.board.contains_key(&space) {
            let error = format!("Can't play in position {}, as it has been already played.", space);
            self.error = Some(error);
        } else {
            self.play_board_space(space)
        }
    }

    fn play_board_space(&mut self, space: u8) {
        if self.board.len() % 2 == 0 {
            self.board.insert(space, PlayMarkers::X);
        } else {
            self.board.insert(space, PlayMarkers::O);
        }
    }
}

#[cfg(test)]
mod new_game {
    use crate::game::{Game};
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
        assert_eq!(game.board.get(&0), Some(&PlayMarkers::X));
    }

    #[test]
    fn o_plays_second() {
        let mut game = Game::new();
        game.play(0);
        game.play(1);
        assert_eq!(game.board.get(&1), Some(&PlayMarkers::O));
    }

    #[test]
    fn error_when_playing_in_taken_position_0() {
        let mut game = Game::new();
        game.play(0);
        game.play(0);
        let expected_error = "Can't play in position 0, as it has been already played.".to_string();
        assert_eq!(game.error, Some(expected_error))
    }
}
