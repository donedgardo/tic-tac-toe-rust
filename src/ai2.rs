use rand::seq::SliceRandom;
use crate::Game;


pub fn get_best_move_2(game: &Game) -> u8
{
    let moves = game.get_available_plays();
    let mut best_move: u8 = *moves.choose(&mut rand::thread_rng()).unwrap();
    if should_min_max(moves.len()) {
        for board_space in moves {
            let mut game_copy = game.copy();
            game_copy.play(board_space);
            if game_copy.is_over {
                best_move = board_space;
                break;
            }
        }
    }
    best_move
}

fn should_min_max(available_move_count: usize) -> bool {
    available_move_count < 9 && available_move_count > 1
}

#[cfg(test)]
mod ai_game_2 {
    use crate::ai2::get_best_move_2;
    use crate::Game;

    #[test]
    fn plays_random_first_move() {
        let game = Game::new();
        let best_move = get_best_move_2(&game);
        let possible_moves: Vec<u8> = (0u8..9u8).collect();
        assert!(possible_moves.contains(&best_move));
    }

    #[test]
    fn plays_only_move_available() {
        let mut game = Game::new();
        for play in [0, 4, 2, 1, 7, 5, 3, 6] {
            game.play(play);
        }
        let best_move = get_best_move_2(&game);
        assert_eq!(8u8, best_move);
    }

    #[test]
    fn plays_obvious_winning_move() {
        let mut game = Game::new();
        for play in [0, 4, 6, 7] {
            game.play(play);
        }
        let best_move = get_best_move_2(&game);
        assert_eq!(3u8, best_move);
    }
}
