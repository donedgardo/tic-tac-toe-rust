fn main() {
    println!("Hello, world!");
}


const X: u8 = 1;

#[derive(Debug)]
pub struct Game {
   is_over: bool,
   board: [u8; 9]
}

impl Game {
    fn play(&mut self, space: usize) {
        self.board[space] = X;
    }
}

pub fn new_game() -> Game {
    Game {
        is_over: false,
        board: [0, 0, 0, 0, 0, 0, 0, 0, 0]
    }
}

#[cfg(test)]
mod new_game {
    use crate::{new_game, X};

    #[test]
    fn is_not_over() {
        let game = new_game();
        assert!(!game.is_over)
    }

    #[test]
    fn has_empty_board() {
        let game = new_game();
        let board = game.board;
        for space in board {
            assert_eq!(space, 0)
        }
    }

    #[test]
    fn x_plays_first() {
        let mut game = new_game();
        game.play(0);
        assert_eq!(game.board[0], X)
    }

}
