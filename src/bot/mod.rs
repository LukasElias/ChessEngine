use {
    chess::{
        Game,
        Board,
        Color,
        ChessMove,
        Error,
    },
    std::time::Duration,
};

#[derive(Clone, Copy, Debug)]
pub struct GameOptions {
    time: Duration,
    engine_plays: Color,
}

impl Default for GameOptions {
    fn default() -> Self {
        Self {
            time: Duration::from_secs(5 * 60), // 5 Minutes
            engine_plays: Color::Black,
        }
    }
}

#[derive(Clone, Debug)]
pub struct EngineMatch {
    options: GameOptions,
    game: Game,
}

impl EngineMatch {
    pub fn new(options: GameOptions) -> Self {
        Self {
            options,
            game: Game::new(),
        }
    }

    pub fn user_move(chess_move: ChessMove) -> Result<(), Error> {
        Ok(())
    }
}
