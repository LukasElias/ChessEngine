use {
    std::time::Duration,
    chess::{
        Board,
        Color,
    },
};

pub struct GameOptions {
    time: Duration,
    plays: Color,
}

pub fn start_game(game_options: GameOptions) {
    let board = Board::default();
}
