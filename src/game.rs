use std::collections::HashMap;
use crate::board::Board;
use crate::play_markers::PlayMarkers;
use crate::winning_plays;

pub struct Game {
    board: Board,
    error: Option<String>,
    is_over: bool,
    winner: Option<PlayMarkers>,
    winning_plays: HashMap<u8, Vec<[u8; 3]>>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            error: None,
            is_over: false,
            winner: None,
            winning_plays: winning_plays::get_winning_plays(),
        }
    }

    pub fn play(&mut self, space: u8) {
        let active_marker = self.get_active_marker();
        if !self.is_valid_move(&space) {
            self.set_move_error(&space);
        } else {
            self.board.play(space, &active_marker);
        }
        if self.board.is_full() {
            self.is_over = true;
        }
        if self.is_winning_play(&space, &active_marker) {
            self.winner = Some(active_marker);
            self.is_over = true;
        }
    }

    fn is_winning_play(&self, space: &u8, marker: &PlayMarkers) -> bool {
        let mut is_position_winning_play = false;
        'win_loop: for winning_plays in self.winning_plays.get(&space) {
            for winning_spaces in winning_plays {
                if self.is_marker_in_all_positions(winning_spaces, marker) {
                    is_position_winning_play = true;
                    break 'win_loop;
                }
            }

        }
        is_position_winning_play
    }

    fn is_marker_in_all_positions(&self, winning_play: &[u8; 3], marker: &PlayMarkers) -> bool {
        let mut did_play_all_position = true;
        for space in winning_play {
            if self.board.get_space_marker(space) != Some(marker) {
                did_play_all_position = false;
                break;
            }
        }
        did_play_all_position
    }

    fn get_active_marker(&self) -> PlayMarkers {
        if self.board.spaces.len() % 2 == 0 {
            PlayMarkers::X
        } else {
            PlayMarkers::O
        }
    }
    fn is_valid_move(&self, space: &u8) -> bool {
        if !self.is_over && !self.board.is_space_played(space) { true } else { false }
    }

    fn set_move_error(&mut self, space: &u8) {
        if self.is_over {
            let error = "Can't play after game is over.".to_string();
            self.error = Some(error);
        } else if self.board.is_space_played(&space) {
            let error = format!("Can't play in position {}, as it has been already played.", space);
            self.error = Some(error);
        }
    }
}

#[cfg(test)]
mod new_game {
    use crate::game::Game;
    use crate::play_markers::PlayMarkers;

    const CATS_GAME: [u8; 9] = [0, 4, 2, 1, 7, 5, 3, 6, 8];

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
        for space in CATS_GAME {
            game.play(space);
        }
        assert!(game.is_over);
    }

    #[test]
    fn cats_game_has_no_winner() {
        let mut game = Game::new();
        for space in CATS_GAME {
            game.play(space);
        }
        assert_eq!(game.winner, None);
    }


    #[test]
    fn x_wins_any_position() {
        let games_where_x_wins: [[u8; 5]; 8] = [
            [0, 3, 1, 4, 2], [3, 1, 4, 2, 5],
            [6, 1, 7, 3, 8], [0, 1, 3, 2, 6],
            [1, 6, 4, 5, 7], [2, 1, 5, 6, 8],
            [0, 1, 4, 2, 8], [2, 3, 4, 5, 6],
        ];
        for game_play in games_where_x_wins {
            let mut game = Game::new();
            for space in game_play {
                game.play(space);
            }
            assert_eq!(game.winner, Some(PlayMarkers::X));
        }
    }

    #[test]
    fn o_wins_any_position() {
        let games_where_o_wins: [[u8; 6]; 8] = [
            [5, 0, 3, 1, 8, 2], [6, 3, 1, 4, 2, 5],
            [5, 6, 1, 7, 3, 8], [5, 0, 1, 3, 2, 6],
            [3, 1, 6, 4, 5, 7], [3, 2, 1, 5, 6, 8],
            [3, 0, 1, 4, 2, 8], [1, 2, 3, 4, 5, 6],
        ];
        for game_play in games_where_o_wins {
            let mut game = Game::new();
            for space in game_play {
                game.play(space);
            }
            assert_eq!(game.winner, Some(PlayMarkers::O));
        }
    }

    #[test]
    fn is_over_after_win() {
        let mut game = Game::new();
        for space in [0, 3, 1, 4, 2] {
            game.play(space);
        }
        assert_eq!(game.is_over, true);
    }

    #[test]
    fn error_when_playing_after_game_over() {
        let mut game = Game::new();
        for space in [0, 3, 1, 4, 2] {
            game.play(space);
        }
        game.play(5);
        let expected_error = "Can't play after game is over.".to_string();
        assert_eq!(game.error, Some(expected_error))
    }

    #[test]
    fn game_stays_the_same_after_playing_game_over() {
        let mut game = Game::new();
        for space in [0, 3, 1, 4, 2] {
            game.play(space);
        }
        game.play(5);
        assert_eq!(game.board.get_space_marker(&5), None)
    }
}
