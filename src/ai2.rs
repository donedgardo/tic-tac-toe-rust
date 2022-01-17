use rand::seq::SliceRandom;
use crate::Game;

pub fn get_best_move_2(game: &Game) -> u8
{
    let moves = game.get_available_plays();
    *moves.choose(&mut rand::thread_rng()).unwrap()
}

#[cfg(test)]
mod ai_game_2 {
    use crate::ai2::get_best_move_2;
    use crate::Game;

    #[test]
    fn get_random_first_move() {
        let game = Game::new();
        let best_move = get_best_move_2(&game);
        let possible_moves: Vec<u8> = (0u8..9u8).collect();
        assert!(possible_moves.contains(&best_move));
    }

    #[test]
    fn get_only_move_available() {
        let mut game = Game::new();
        for play in [0, 4, 2, 1, 7, 5, 3, 6] {
            game.play(play);
        }
        let best_move = get_best_move_2(&game);
        assert_eq!(8u8, best_move);
    }

}
