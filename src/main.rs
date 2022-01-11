use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

const X: u8 = 1;
const O: u8 = 2;

#[derive(Debug)]
pub struct Game {
    is_over: bool,
    board: HashMap<u8, u8>,
}

impl Game {
    fn play(&mut self, space: u8) {
        if self.board.len() % 2 == 0 {
            self.board.insert(space, X);
        } else {
            self.board.insert(space, O);
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
    use crate::{new_game, X, O};
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
        assert_eq!(game.board.get(&0), Some(&X));
    }

    #[test]
    fn o_plays_second() {
        let mut game = new_game();
        game.play(0);
        game.play(1);
        assert_eq!(game.board.get(&1), Some(&O));
    }
}
