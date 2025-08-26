use {
    chess::*,
    std::time::Duration,
};

#[derive(Clone, Copy, Debug)]
pub struct GameOptions {
    time: Option<Duration>,
    engine_plays: Color,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Game {
    board: Board,
    game_options: GameOptions,
}

impl Default for GameOptions {
    fn default() -> Self {
        Self {
            time: None,
            engine_plays: Color::Black,
        }
    }
}

impl Game {
    fn new(game_options: GameOptions) -> Self {
        Self {
            board: Board::default(),
            game_options,
        }
    }
}
