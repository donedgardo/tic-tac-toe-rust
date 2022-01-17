use std::collections::HashMap;
use rand::prelude::SliceRandom;
use crate::{Game, PlayMarkers};

pub fn get_max_key<K, V>(a_hash_map: &HashMap<K, V>) -> Option<&K> where V: Ord,
{
    a_hash_map
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
}

pub fn get_best_move(game: &Game) -> u8 {
    let active_marker = game.get_active_marker();
    let best_move: u8;
    let mut moves = game.get_available_plays();
    let mut scores: HashMap<u8, i32> = HashMap::new();
    loop {
        if moves.len() == 9 {
            best_move = *moves.choose(&mut rand::thread_rng()).unwrap();
            break;
        } else if moves.is_empty() {
            let max_key = get_max_key(&scores);
            best_move = *max_key.unwrap();
            break;
        } else {
            let play = moves.pop().unwrap();
            let mut game_copy: Game = game.copy();
            game_copy.play(play);
            let score = minimax(&game_copy, active_marker, 0, false);
            scores.insert(play, score);
        };
    }
    best_move
}

fn minimax(game: &Game, player_mark: PlayMarkers, depth: i32, is_maximizing: bool) -> i32 {
    let mut score = score_game(game, player_mark);
    let new_depth = match is_maximizing {
        true => depth + 1,
        false => depth -1,
    };
    let moves = game.get_available_plays();
    loop {
        // game ended so score the game.
        if !score.is_none() { break; }

        let mut games = Vec::new();
        for m in &moves {
            let mut game_copy = game.copy();
            game_copy.play(*m);
            games.push(game_copy);
        }

        let mut scores = Vec::new();
        for g in games {
            scores.push(minimax(&g, player_mark, new_depth, !is_maximizing))
        }
        if is_maximizing {
            let max_value = scores.iter().max();
            score = match max_value {
                Some(max) => Some(max + depth),
                None => None,
            }
        } else {
            let min_value = scores.iter().min();
            score = match min_value {
                Some(min) => Some(min - depth),
                None => None,
            }
        }
    }
    score.unwrap()
}

fn score_game(game: &Game, marker: PlayMarkers) -> Option<i32> {
    let winner = game.get_winner();
    match winner {
        Some(winner_marker) => {
            if marker == winner_marker { Some(10) } else { Some(-10) }
        }
        None => {
            if game.is_over { Some(0) } else { None }
        }
    }
}

#[cfg(test)]
mod ai_game {
    use crate::Game;
    use crate::ai::get_best_move;

    #[test]
    fn gets_obvious_move() {
        let mut game = Game::new();
        for play in [0, 3, 1, 4] {
            game.play(play);
        }
        assert_eq!(get_best_move(&game), 2)
    }

    #[test]
    fn avoids_trap() {
        let mut game = Game::new();
        game.play(0);
        assert_eq!(get_best_move(&game), 4)
    }

    #[test]
    fn avoids_l_trap() {
        let mut game = Game::new();
        for play in [0, 4, 8] {
            game.play(play);
        }
        assert!(vec![1u8, 3u8, 5u8, 7u8].contains(&get_best_move(&game)))
    }
}
