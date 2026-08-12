mod bot;

use bot::{Engine, EngineError, UCI};

fn main() -> Result<(), EngineError> {
    let mut engine = Engine::default();

    engine.listen()
}
