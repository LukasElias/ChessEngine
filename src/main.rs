mod bot;

use {
    bot::{
        Engine,
        UCI,
        EngineError,
    },
};

fn main() -> Result<(), EngineError> {
    let mut engine = Engine::default();

    engine.listen()
}
