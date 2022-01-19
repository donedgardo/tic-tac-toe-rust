use std::collections::HashMap;
use rand::seq::SliceRandom;
use crate::{Game, PlayMarkers};

pub fn get_best_move(game: &Game) -> u8
{
    let mut moves = game.get_available_plays();
    let mut best_move: u8 = *moves.choose(&mut rand::thread_rng()).unwrap();
    let active_marker = game.get_active_marker();
    let mut move_scores: HashMap<u8, i32> = HashMap::new();
    if should_min_max(moves.len()) {
        loop {
            if moves.is_empty() {
                best_move = *get_max_key(&move_scores).unwrap();
                break;
            } else {
                let play = moves.pop().unwrap();
                let mut game_copy: Game = game.copy();
                game_copy.play(play);
                let score = mini_max(&game_copy, active_marker, 0);
                move_scores.insert(play, score);
            }
        }
    }
    best_move
}

fn mini_max(game: &Game, marker: PlayMarkers, depth: i32) -> i32 {
    let mut score = get_game_score(marker, game);
    let is_maximizing = game.get_active_marker() == marker;
    loop {
        if !score.is_none() { break; }
        let games = get_possible_games(game);
        let scores = score_games(marker, games, depth + 1);
        let scores_iter = scores.iter();
        if  is_maximizing {
            score = match scores_iter.max() {
                Some(value) => Some(value - depth),
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

fn score_games(marker: PlayMarkers, games: Vec<Game>, depth: i32) -> Vec<i32> {
    let mut scores = Vec::new();
    for g in games {
        scores.push(mini_max(&g, marker, depth));
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

pub fn get_max_key<K, V>(a_hash_map: &HashMap<K, V>) -> Option<&K> where V: Ord,
{
    a_hash_map
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
}


#[cfg(test)]
mod ai_game_2 {
    use crate::ai2::{mini_max, get_best_move};
    use crate::{Game, PlayMarkers};

    #[test]
    fn mini_max_scores_winning_game() {
        let mut game = Game::new();
        for play in [0, 4, 6, 2, 3] {
            game.play(play);
        }
        assert_eq!(10, mini_max(&game, PlayMarkers::X, 0))
    }

    #[test]
    fn mini_max_scores_loosing_game() {
        let mut game = Game::new();
        for play in [0, 4, 6, 2, 3] {
            game.play(play);
        }
        assert_eq!(-10, mini_max(&game, PlayMarkers::O, 0))
    }

    #[test]
    fn mini_max_scores_next_play_winning_game() {
        let mut game = Game::new();
        for play in [0, 4, 2, 5] {
            game.play(play);
        }
        assert_eq!(10, mini_max(&game, PlayMarkers::X, 0))
    }

    #[test]
    fn mini_max_scores_next_play_loosing_game() {
        let mut game = Game::new();
        for play in [0, 4, 2, 3] {
            game.play(play);
        }
        assert_eq!(-11, mini_max(&game, PlayMarkers::O, 0))
    }

    #[test]
    fn get_best_gets_offensive_move() {
        let mut game = Game::new();
        for play in [0, 4, 2, 3] {
            game.play(play);
        }
        assert_eq!(1, get_best_move(&game))
    }

    #[test]
    fn get_best_gets_defensive_move() {
        let mut game = Game::new();
        for play in [0] {
            game.play(play);
        }
        assert_eq!(4, get_best_move(&game))
    }

    #[test]
    fn get_best_prevents_winner() {
        let mut game = Game::new();
        for play in [0, 8, 1] {
            game.play(play);
        }
        assert_eq!(2, get_best_move(&game))
    }

    #[test]
    fn get_best_move_traps_player() {
        let mut game = Game::new();
        for play in [0, 1] {
            game.play(play);
        }
        println!("{}", get_best_move(&game));
        assert!(vec![3u8, 4u8, 6u8].contains(&get_best_move(&game)))
    }

}
