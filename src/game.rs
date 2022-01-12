use std::collections::HashMap;
use crate::play_markers::PlayMarkers;

#[derive(Debug)]
pub struct Game {
    is_over: bool,
    board: HashMap<u8, PlayMarkers>,
}

impl Game {
    fn play(&mut self, space: u8) {
        if self.board.len() % 2 == 0 {
            self.board.insert(space, PlayMarkers::X);
        } else {
            self.board.insert(space, PlayMarkers::O);
        }
    }
}

pub fn new_game() -> Game {
    Game {
        is_over: false,
        board: HashMap::new(),
    }
}

#[cfg(test)]
mod new_game {
    use crate::game::new_game;
    use crate::play_markers::PlayMarkers;

    #[test]
    fn is_not_over() {
        let game = new_game();
        assert!(!game.is_over)
    }

    #[test]
    fn has_empty_board() {
        let game = new_game();
        let board = game.board;
        assert!(board.is_empty())
    }

    #[test]
    fn x_plays_first() {
        let mut game = new_game();
        game.play(0);
        assert_eq!(game.board.get(&0), Some(&PlayMarkers::X));
    }

    #[test]
    fn o_plays_second() {
        let mut game = new_game();
        game.play(0);
        game.play(1);
        assert_eq!(game.board.get(&1), Some(&PlayMarkers::O));
    }
}
