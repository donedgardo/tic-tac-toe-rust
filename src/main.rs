fn main() {
    println!("Hello, world!");
}


pub struct Game {
   is_over: bool
}

pub fn new_game() -> Game {
    Game {
        is_over: false
    }
}

#[cfg(test)]
mod test {
    use crate::{new_game};

    #[test]
    fn is_not_over() {
        let game = new_game();
        assert!(!game.is_over)
    }
}
