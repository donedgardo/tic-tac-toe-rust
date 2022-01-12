use std::collections::HashMap;

pub fn get_winning_plays() -> HashMap<u8, Vec<[u8; 3]>> {
    const TOP_HORIZONTAL: [u8; 3] = [0, 1, 2];
    const MIDDLE_HORIZONTAL: [u8; 3] = [3, 4, 5];
    const BOTTOM_HORIZONTAL: [u8; 3] = [6, 7, 8];
    const LEFT_VERTICAL: [u8; 3] = [0, 3, 6];
    const MIDDLE_VERTICAL: [u8; 3] = [1, 4, 7];
    const RIGHT_VERTICAL: [u8; 3] = [2, 5, 8];
    const DIAGONAL_DOWN: [u8; 3] = [0, 4, 8];
    const DIAGONAL_UP: [u8; 3] = [2, 4, 6];
    let mut winning_plays = HashMap::new();
    winning_plays.insert(0, vec![TOP_HORIZONTAL, DIAGONAL_DOWN, LEFT_VERTICAL]);
    winning_plays.insert(1, vec![TOP_HORIZONTAL, MIDDLE_VERTICAL]);
    winning_plays.insert(2, vec![TOP_HORIZONTAL, RIGHT_VERTICAL, DIAGONAL_UP]);
    winning_plays.insert(3, vec![LEFT_VERTICAL, MIDDLE_HORIZONTAL]);
    winning_plays.insert(4, vec![MIDDLE_VERTICAL, MIDDLE_HORIZONTAL, DIAGONAL_UP, DIAGONAL_DOWN]);
    winning_plays.insert(5, vec![MIDDLE_HORIZONTAL, RIGHT_VERTICAL]);
    winning_plays.insert(6, vec![BOTTOM_HORIZONTAL, LEFT_VERTICAL, DIAGONAL_UP]);
    winning_plays.insert(7, vec![BOTTOM_HORIZONTAL, MIDDLE_VERTICAL]);
    winning_plays.insert(8, vec![BOTTOM_HORIZONTAL, RIGHT_VERTICAL, DIAGONAL_DOWN]);
    winning_plays
}
