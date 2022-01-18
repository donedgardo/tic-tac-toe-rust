use rand::seq::SliceRandom;
use crate::{Game, PlayMarkers};


pub fn get_best_move_2(game: &Game) -> u8
{
    let moves = game.get_available_plays();
    let mut best_move: u8 = *moves.choose(&mut rand::thread_rng()).unwrap();
    if should_min_max(moves.len()) {
        for board_space in moves {
            let mut game_copy = game.copy();
            game_copy.play(board_space);
            match game_copy.get_winner() {
                Some(_winner) => {
                    best_move = board_space;
                    break;
                }
                _ => {}
            }
        }
    }
    best_move
}

fn mini_max(game: &Game, marker: PlayMarkers) -> i32 {
    let mut score = get_game_score(marker, game);
    let is_maximizing = game.get_active_marker() == marker;
    loop {
        if !score.is_none() { break; }
        let games = get_possible_games(game);
        let scores = score_games(marker, games);
        let scores_iter = scores.iter();
        if  is_maximizing {
            score = match scores_iter.max() {
                Some(value) => Some(*value),
                None => None
            };
        } else {
            score = match scores_iter.min() {
                Some(value) => Some(*value),
                None => None
            };
        }
    }
    score.unwrap()
}

fn score_games(marker: PlayMarkers, games: Vec<Game>) -> Vec<i32> {
    let mut scores = Vec::new();
    for g in games {
        scores.push(mini_max(&g, marker));
    }
    scores
}

fn get_possible_games(game: &Game) -> Vec<Game> {
    let moves = game.get_available_plays();
    let mut games = Vec::new();
    for m in &moves {
        let mut game_copy = game.copy();
        game_copy.play(*m);
        games.push(game_copy);
    }
    games
}

fn get_game_score(marker: PlayMarkers, game: &Game) -> Option<i32> {
    let score = match game.get_winner() {
        Some(winner) => {
            if winner == marker { Some(10) } else { Some(-10) }
        }
        None => {
            if game.is_over { Some(0) } else { None }
        }
    };
    score
}

fn should_min_max(available_move_count: usize) -> bool {
    available_move_count < 9 && available_move_count > 1
}

#[cfg(test)]
mod ai_game_2 {
    use crate::ai2::{mini_max};
    use crate::{Game, PlayMarkers};

    #[test]
    fn mini_max_scores_winning_game() {
        let mut game = Game::new();
        for play in [0, 4, 6, 2, 3] {
            game.play(play);
        }
        assert_eq!(10, mini_max(&game, PlayMarkers::X))
    }

    #[test]
    fn mini_max_scores_loosing_game() {
        let mut game = Game::new();
        for play in [0, 4, 6, 2, 3] {
            game.play(play);
        }
        assert_eq!(-10, mini_max(&game, PlayMarkers::O))
    }

    #[test]
    fn mini_max_scores_next_play_winning_game() {
        let mut game = Game::new();
        for play in [0, 4, 2, 5] {
            game.play(play);
        }
        assert_eq!(10, mini_max(&game, PlayMarkers::X))
    }

    #[test]
    fn mini_max_scores_next_play_loosing_game() {
        let mut game = Game::new();
        for play in [0, 4, 2, 3] {
            game.play(play);
        }
        assert_eq!(-10, mini_max(&game, PlayMarkers::O))
    }
}
